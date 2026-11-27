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
    C0(u64, Bytes, List<Bytes>, Bytes, Bytes, List<Adt16>),
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
    F240(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F65(k0, k1) => f65(*k0, k1.clone(), p0),
            Fn0::F240(k0) => f240(k0.clone(), p0),
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
    F242,
    F325,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F109 => f109(p0, p1),
            Fn2::F242 => f242(p0, p1),
            Fn2::F325 => f325(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F166(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt17)) -> R<(u64, Adt17)> {
        match self {
            Fn3::F166(k0, k1, k2) => f166(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F178(Bytes),
    F189(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F178(k0) => f178(k0.clone(), p0),
            Fn4::F189(k0, k1) => Ok(f189(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F180(Bytes),
    F181(Bytes),
    F215(Bytes),
    F216(Bytes),
    F244(Bytes, u64, Bytes, u64),
    F303(Bytes),
    F336(Bytes),
    F338(Bytes),
    F350(Bytes, u64),
    F389(Bytes),
    F419(Bytes, Bytes),
    F422(Bytes),
    F424(Bytes),
    F450(Bytes, bool),
    F455(u64, Bytes),
    F559(u64, Bytes, u64),
    F568(Bytes),
    F569(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F180(k0) => f180(k0.clone(), p0),
            Fn5::F181(k0) => f181(k0.clone(), p0),
            Fn5::F215(k0) => f215(k0.clone(), p0),
            Fn5::F216(k0) => f216(k0.clone(), p0),
            Fn5::F244(k0, k1, k2, k3) => f244(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn5::F303(k0) => f303(k0.clone(), p0),
            Fn5::F336(k0) => f336(k0.clone(), p0),
            Fn5::F338(k0) => f338(k0.clone(), p0),
            Fn5::F350(k0, k1) => f350(k0.clone(), *k1, p0),
            Fn5::F389(k0) => f389(k0.clone(), p0),
            Fn5::F419(k0, k1) => f419(k0.clone(), k1.clone(), p0),
            Fn5::F422(k0) => f422(k0.clone(), p0),
            Fn5::F424(k0) => f424(k0.clone(), p0),
            Fn5::F450(k0, k1) => f450(k0.clone(), *k1, p0),
            Fn5::F455(k0, k1) => f455(*k0, k1.clone(), p0),
            Fn5::F559(k0, k1, k2) => f559(*k0, k1.clone(), *k2, p0),
            Fn5::F568(k0) => f568(k0.clone(), p0),
            Fn5::F569(k0) => f569(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F219(Bytes),
    F430(Bytes),
    F446(bool, Bytes),
    F518(Bytes),
    F533(Bytes),
    F558(u64, Bytes),
    F561(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn6::F219(k0) => f219(k0.clone(), p0),
            Fn6::F430(k0) => f430(k0.clone(), p0),
            Fn6::F446(k0, k1) => f446(*k0, k1.clone(), p0),
            Fn6::F518(k0) => f518(k0.clone(), p0),
            Fn6::F533(k0) => f533(k0.clone(), p0),
            Fn6::F558(k0, k1) => f558(*k0, k1.clone(), p0),
            Fn6::F561(k0, k1) => f561(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F305,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt22) -> R<Adt22> {
        match self {
            Fn7::F305 => f305(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F319,
    F605,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F319 => f319(p0, p1),
            Fn8::F605 => f605(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F322,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn9::F322 => f322(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F410,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn10::F410 => f410(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F413,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn11::F413 => f413(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F415,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn12::F415 => f415(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F418,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn13::F418 => f418(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F484(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn14::F484(k0) => f484(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F493,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F493 => f493(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F502,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F502 => f502(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F504,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F504 => f504(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F506,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn18::F506 => f506(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F509,
}

impl Fn19 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn19::F509 => f509(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F530(u64, Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn20::F530(k0, k1) => f530(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F555,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F555 => f555(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F584,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn22::F584 => f584(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F594,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn23::F594 => f594(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F600,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn24::F600 => f600(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F603,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F603 => f603(p0),
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
                    }, List::cons(Adt16::C0(f138(), Bytes::lit(&[]), f139(false), List::<Bytes>::nil()), List::cons(Adt16::C0(f140(), Bytes::lit(&[]), f139(true), List::<Bytes>::nil()), f141(f142(f143(v6.clone(), v6.clone())?, f144(v6.clone())), List::<Bytes>::nil()))))?;
                    let v8: List<Bytes> = f145({
                        let a525 = {
                            let a524 = v7.clone();
                            length_list(a524)
                        };
                        let a526 = 1u64;
                        nat_add(a525, a526)?
                    }, f146(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f147(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f148({
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
                    }, f7(List::cons(f7(f149(v3, v7, v6)), List::cons(f7(f150({
                        let (_, h532) = v10;
                        h532
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f151(f152({
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
            let m536 = f153(v1)?;
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
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f154(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m538 - 1;
                    let m541 = {
                        let a539 = v0.clone();
                        let a540 = v1;
                        index_bytes(a539, a540)
                    };
                    match m541 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f154(v0.clone(), v5, v1, v6)))
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
                                    }, f154(v0.clone(), v5, v1, v6)))))))
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
    let v2: u64 = f155(v0.clone(), v1, {
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
        f156(v0.clone(), v1, {
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
    let v4: u64 = f157(v0.clone(), v3)?;
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
        f157(v0.clone(), v7)?
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
    let v12: bool = f105(v10, f158({
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
        f157(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a656 = v14;
        let a657 = v13;
        nat_sub(a656, a657)
    });
    let v16: u64 = f159(v15.clone())?;
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
                f158(if v10 {
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
                f160(v17)?
            }
        };
        match m677 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m682 = f161(f46(v0.clone(), v3, v5), f46(v0, v7, {
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
                            let m688 = f160({
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
                                    if f162(v21, v20) {
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
    let m700 = f163(v0.clone(), v3);
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
    let a740 = f164(match v0.clone() {
        Adt17::C0(h720, _, _, _, _, _, _, _) => {
            h720
        }
    });
    let a741 = {
        let a738 = f164(match v0.clone() {
            Adt17::C0(_, h721, _, _, _, _, _, _) => {
                h721
            }
        });
        let a739 = {
            let a736 = f164(match v0.clone() {
                Adt17::C0(_, _, h722, _, _, _, _, _) => {
                    h722
                }
            });
            let a737 = {
                let a734 = f164(match v0.clone() {
                    Adt17::C0(_, _, _, h723, _, _, _, _) => {
                        h723
                    }
                });
                let a735 = {
                    let a732 = f164(match v0.clone() {
                        Adt17::C0(_, _, _, _, h724, _, _, _) => {
                            h724
                        }
                    });
                    let a733 = {
                        let a730 = f164(match v0.clone() {
                            Adt17::C0(_, _, _, _, _, h725, _, _) => {
                                h725
                            }
                        });
                        let a731 = {
                            let a728 = f164(match v0.clone() {
                                Adt17::C0(_, _, _, _, _, _, h726, _) => {
                                    h726
                                }
                            });
                            let a729 = f164(match v0 {
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
    let (_, h750) = f165(Fn3::F166(v0, v1, v2), {
        let a751 = v2;
        let a752 = 64u64;
        let a753 = 0u64;
        nat_quot(a751, a752, a753)
    }, (0u64, f167()))?;
    Ok(h750)
}

pub fn f113(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m754 = f168(f169(v0.clone(), v1.clone()), v1.clone());
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
    f170(v0.clone(), 0u64, {
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
    let a762 = f171(v0)?;
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
                List::cons(v3, f172(v5, v1, v2))
            } else {
                List::cons(v4, f121(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f122(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f151(f173(f174(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f123(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f119(Adt2::C7(std::rc::Rc::new(f84(f175(f116(v1, f73(f51(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
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
                        let m767 = f176(v1.clone(), f8(Str::lit("\u{a}")))?;
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
        let (h786, _) = f177(Fn4::F178(v0.clone()), {
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
    let (h790, _) = f179(Fn5::F180(v0.clone()), {
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
    let (h794, _) = f179(Fn5::F181(v0.clone()), {
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
                        if f182(v0.clone(), v1, Str::lit("...")) {
                            f183(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m805 = f184(v0.clone(), {
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
        let a811 = f185(v0.clone(), v1)?;
        let a812 = 37u64;
        if nat_eq(a811, a812) {
            Ok(Err::<List<Adt2>, Bytes>(f129(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m819 = if if f182(v0.clone(), v1, Str::lit("---")) {
                f183(v0.clone(), v1)?
            } else {
                false
            } {
                f186(v0.clone(), {
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
                    f186(v0.clone(), v1, 0u64, {
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
                                let m827 = f131(v0, v11, f187({
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
        Ok(f188(v0, 0u64))
    } else {
        let (_, h830) = {
            let (h831, _) = f177(Fn4::F189(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h831
        };
        Ok(h830)
    }
}

pub fn f133() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f134(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m832 = f190(v0)?;
    match m832 {
        Ok(v2) => {
            f191({
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
                    let v3: List<Adt16> = f192(v1.clone(), v1.clone());
                    let a841 = f193(v3.clone())?;
                    let a842 = f193(v1.clone())?;
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

pub fn f141(v0: List<Adt16>, v1: List<Bytes>) -> List<Adt16> {
    let m846 = v0;
    match m846.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            if f114(f194(v2.clone()), v1.clone()) {
                f141(v3, v1)
            } else {
                List::cons(v2.clone(), f141(v3, List::cons(f194(v2), v1)))
            }
        }
    }
}

pub fn f142(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m847 = v0;
    match m847.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f142(v3, v1))
        }
    }
}

pub fn f143(v0: List<Adt13>, v1: List<Adt13>) -> R<List<Adt16>> {
    let m848 = v0;
    match m848.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            if f195(f196(v2.clone())) {
                let v6: Adt16 = {
                    let m849 = f197(v1.clone(), v2.clone())?;
                    match m849 {
                        Ok(v5) => {
                            Adt16::C0(f198(v2), Bytes::lit(&[]), {
                                let (h850, _) = v5;
                                h850
                            }, {
                                let (_, h851) = v5;
                                h851
                            })
                        }
                        Err(v4) => {
                            Adt16::C0(f198(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
                        }
                    }
                };
                Ok(List::cons(v6, f143(v3, v1)?))
            } else {
                f143(v3, v1)
            }
        }
    }
}

pub fn f144(v0: List<Adt13>) -> List<Adt16> {
    let m852 = v0;
    match m852.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v1, v2)) => {
            f142(f199(f200(v1.clone())), f142(f201(f202(v1)), f144(v2)))
        }
    }
}

pub fn f145(p857: u64, p858: List<Bytes>, p859: List<Bytes>, v3: List<Adt16>) -> List<Bytes> {
    let m856 = (p857, (p858, p859));
    {
        let mut m856 = m856;
        loop {
            let step = {
                let (v0, (v1, v2)) = m856;
                let m853 = v0;
                if m853 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m853 - 1;
                    let m854 = v1;
                    match m854.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f114(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m855 = f203(v3.clone(), v5.clone());
                                match m855 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f32(f204(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f205(f206(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m856 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: List<Adt13>) -> List<Bytes> {
    let m860 = v0;
    match m860.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f196(v1.clone());
            let a861 = v3;
            let a862 = 0u64;
            if if nat_eq(a861, a862) {
                true
            } else {
                let a863 = v3;
                let a864 = 2u64;
                if nat_eq(a863, a864) {
                    true
                } else {
                    let a865 = v3;
                    let a866 = 3u64;
                    if nat_eq(a865, a866) {
                        true
                    } else {
                        let a867 = v3;
                        let a868 = 4u64;
                        if nat_eq(a867, a868) {
                            true
                        } else {
                            let a869 = v3;
                            let a870 = 5u64;
                            if nat_eq(a869, a870) {
                                true
                            } else {
                                let a871 = v3;
                                let a872 = 7u64;
                                if nat_eq(a871, a872) {
                                    true
                                } else {
                                    let a873 = v3;
                                    let a874 = 8u64;
                                    nat_eq(a873, a874)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f198(v1), f146(v2))
            } else {
                f146(v2)
            }
        }
    }
}

pub fn f147(v0: List<Adt16>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m875 = v0;
    match m875.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f114(f194(v2.clone()), v1.clone()) {
                f32(f204(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f194(v2.clone()), (f206(v2.clone()), f207(v2))), f147(v3, v1))
            } else {
                f147(v3, v1)
            }
        }
    }
}

pub fn f148(p884: u64, p885: List<(Bytes, (List<Bytes>, Bytes))>, p886: List<Bytes>, p887: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m883 = (p884, (p885, (p886, p887)));
    {
        let mut m883 = m883;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m883;
                let m876 = v0;
                if m876 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m876 - 1;
                    let m877 = f208(v1.clone(), v2.clone());
                    match m877 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h878) = v5.clone();
                                h878
                            }, (List::cons({
                                let (h879, _) = {
                                    let (h880, _) = v5.clone();
                                    h880
                                };
                                h879
                            }, v2), List::cons({
                                let (_, h881) = {
                                    let (h882, _) = v5;
                                    h882
                                };
                                h881
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m883 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: List<Adt13>, v1: List<Adt16>, v2: List<Adt13>) -> List<Bytes> {
    let m888 = v0;
    match m888.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f196(v3.clone());
            let a889 = v5;
            let a890 = 5u64;
            if if nat_eq(a889, a890) {
                true
            } else {
                let a891 = v5;
                let a892 = 7u64;
                nat_eq(a891, a892)
            } {
                let m895 = f209(v2.clone(), {
                    let a893 = v5;
                    let a894 = 5u64;
                    if nat_eq(a893, a894) {
                        0u64
                    } else {
                        2u64
                    }
                }, f198(v3.clone()));
                match m895 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f210(v5), List::cons(f198(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f198(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f149(v4, v1, v2))
                    }
                    Some(_) => {
                        f149(v4, v1, v2)
                    }
                }
            } else {
                let a896 = v5;
                let a897 = 8u64;
                if nat_eq(a896, a897) {
                    let m898 = f209(v2.clone(), 0u64, f198(v3.clone()));
                    match m898 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f198(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f211(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f198(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f149(v4, v1, v2))
                        }
                        Some(_) => {
                            f149(v4, v1, v2)
                        }
                    }
                } else {
                    if f195(v5) {
                        let m899 = f203(v1.clone(), f198(v3.clone()));
                        match m899 {
                            None => {
                                f149(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f32(f204(v8.clone()), Bytes::lit(&[])) {
                                    f149(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f198(v3), List::cons(f8(Str::lit(": ")), List::cons(f204(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f149(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f149(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f150(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m900 = v0;
    match m900.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h901, _) = v1;
                h901
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f150(v2))
        }
    }
}

pub fn f151(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m902 = v0;
    match m902.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m903 = v3.clone();
            match m903.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a906 = v2;
                    let a907 = {
                        let a904 = v1.clone();
                        let a905 = f151(v3, v1);
                        append_bytes(a904, a905)
                    };
                    append_bytes(a906, a907)
                }
            }
        }
    }
}

pub fn f152(p910: List<Bytes>, p911: List<Bytes>) -> List<Bytes> {
    let m909 = (p910, p911);
    {
        let mut m909 = m909;
        loop {
            let step = {
                let (v0, v1) = m909;
                let m908 = v0;
                match m908.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m912 = f212(v0.clone());
    match m912 {
        None => {
            let m913 = f213(v0.clone())?;
            match m913 {
                None => {
                    Ok(Ok::<Bytes, Bytes>(f171(v0)?))
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

pub fn f154(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a916 = v3;
    let a917 = f46(v0, v1, {
        let a914 = v2;
        let a915 = v1;
        nat_sub(a914, a915)
    });
    append_bytes(a916, a917)
}

pub fn f155(v0: Bytes, p930: u64, p931: u64) -> R<u64> {
    let m929 = (p930, p931);
    {
        let mut m929 = m929;
        loop {
            let step = {
                let (v1, v2) = m929;
                let m918 = v2;
                if m918 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m918 - 1;
                    let v4: u64 = f103(v0.clone(), v1)?;
                    let a919 = v4;
                    let a920 = 34u64;
                    if nat_eq(a919, a920) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a921 = v4;
                        let a922 = 92u64;
                        if nat_eq(a921, a922) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a923 = v4;
                            let a924 = 32u64;
                            if nat_lt(a923, a924) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a925 = v4;
                                let a926 = 256u64;
                                if nat_eq(a925, a926) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a927 = v1;
                                        let a928 = 1u64;
                                        nat_add(a927, a928)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Bytes, p957: u64, p958: u64, p959: Bytes) -> R<Option<(u64, Bytes)>> {
    let m956 = (p957, (p958, p959));
    {
        let mut m956 = m956;
        loop {
            let step = {
                let (v1, (v2, v3)) = m956;
                let m932 = v2;
                if m932 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m932 - 1;
                    let v5: u64 = f155(v0.clone(), v1, {
                        let a936 = {
                            let a934 = {
                                let a933 = v0.clone();
                                length_bytes(a933)
                            };
                            let a935 = v1;
                            nat_sub(a934, a935)
                        };
                        let a937 = 1u64;
                        nat_add(a936, a937)?
                    })?;
                    let v6: Bytes = {
                        let a940 = v3;
                        let a941 = f46(v0.clone(), v1, {
                            let a938 = v5;
                            let a939 = v1;
                            nat_sub(a938, a939)
                        });
                        append_bytes(a940, a941)
                    };
                    let v7: u64 = f103(v0.clone(), v5)?;
                    let a942 = v7;
                    let a943 = 34u64;
                    if nat_eq(a942, a943) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a944 = v5;
                            let a945 = 1u64;
                            nat_add(a944, a945)?
                        }, v6))))
                    } else {
                        let a946 = v7;
                        let a947 = 92u64;
                        if nat_eq(a946, a947) {
                            let m948 = f214(v0.clone(), v5, v6)?;
                            match m948 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a951 = {
                                        let (_, h949) = {
                                            let (_, h950) = v8.clone();
                                            h950
                                        };
                                        h949
                                    };
                                    let a952 = 0u64;
                                    if nat_eq(a951, a952) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h953, _) = v8;
                                            h953
                                        }, (v4, {
                                            let (h954, _) = {
                                                let (_, h955) = v8;
                                                h955
                                            };
                                            h954
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
                Ok(next) => { m956 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: Bytes, v1: u64) -> R<u64> {
    let (h960, _) = f179(Fn5::F215(v0.clone()), {
        let a962 = {
            let a961 = v0;
            length_bytes(a961)
        };
        let a963 = 1u64;
        nat_add(a962, a963)?
    }, v1)?;
    Ok(h960)
}

pub fn f158(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f159(v0: Bytes) -> R<u64> {
    let (h964, _) = f179(Fn5::F216(v0.clone()), {
        let a966 = {
            let a965 = v0;
            length_bytes(a965)
        };
        let a967 = 1u64;
        nat_add(a966, a967)?
    }, 0u64)?;
    Ok(h964)
}

pub fn f160(v0: Bytes) -> R<Option<u64>> {
    if f217(v0.clone())? {
        Ok(Some({
            let (_, h968) = {
                let (h969, _) = f218(Fn6::F219(v0.clone()), {
                    let a970 = v0;
                    length_bytes(a970)
                }, (0u64, 0u64))?;
                h969
            };
            h968
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f161(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a972 = {
            let a971 = v1.clone();
            length_bytes(a971)
        };
        let a973 = f220(v1)?;
        nat_sub(a972, a973)
    });
    let v5: Bytes = {
        let a974 = v0;
        let a975 = v4.clone();
        append_bytes(a974, a975)
    };
    let v6: u64 = f159(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a977 = {
            let a976 = v5;
            length_bytes(a976)
        };
        let a978 = v6;
        nat_sub(a977, a978)
    });
    let v8: u64 = {
        let a979 = v4;
        length_bytes(a979)
    };
    let v9: u64 = {
        let a980 = v7.clone();
        length_bytes(a980)
    };
    let a981 = v9;
    let a982 = 0u64;
    if nat_eq(a981, a982) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f158(v2, {
            let a983 = v3;
            let a984 = v8;
            nat_lt(a983, a984)
        }) {
            let v10: u64 = if v2 {
                let a985 = v3;
                let a986 = v8;
                nat_add(a985, a986)?
            } else {
                let a987 = v8;
                let a988 = v3;
                nat_sub(a987, a988)
            };
            let v11: u64 = f220(v7.clone())?;
            let a989 = v10;
            let a990 = v11;
            if nat_le(a989, a990) {
                Ok(Some((f46(v7, 0u64, {
                    let a991 = v9;
                    let a992 = v10;
                    nat_sub(a991, a992)
                }), 0u64)))
            } else {
                let a995 = {
                    let a993 = v10;
                    let a994 = v11;
                    nat_sub(a993, a994)
                };
                let a996 = 19u64;
                if nat_le(a995, a996) {
                    Ok(Some((f46(v7, 0u64, {
                        let a997 = v9;
                        let a998 = v11;
                        nat_sub(a997, a998)
                    }), {
                        let a999 = v10;
                        let a1000 = v11;
                        nat_sub(a999, a1000)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1001 = v3;
                let a1002 = v8;
                nat_sub(a1001, a1002)
            };
            let a1005 = {
                let a1003 = v9;
                let a1004 = v12;
                nat_add(a1003, a1004)?
            };
            let a1006 = 19u64;
            if nat_le(a1005, a1006) {
                Ok(Some(({
                    let a1007 = v7;
                    let a1008 = f221(v12);
                    append_bytes(a1007, a1008)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f162(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1009 = 0u64;
        let a1010 = v1;
        if nat_lt(a1009, a1010) {
            let a1011 = v1;
            let a1012 = 9223372036854775808u64;
            nat_le(a1011, a1012)
        } else {
            false
        }
    } else {
        let a1013 = v1;
        let a1014 = 9223372036854775807u64;
        nat_le(a1013, a1014)
    }
}

pub fn f163(v0: Bytes, v1: u64) -> Option<u8> {
    let a1015 = v0;
    let a1016 = v1;
    index_bytes(a1015, a1016)
}

pub fn f164(v0: u32) -> Bytes {
    let a1045 = f222({
        let a1017 = f223(v0, 28u32);
        let a1018 = 15u32;
        fixed_u32::bit_and(a1017, a1018)
    });
    let a1046 = {
        let a1043 = f222({
            let a1019 = f223(v0, 24u32);
            let a1020 = 15u32;
            fixed_u32::bit_and(a1019, a1020)
        });
        let a1044 = {
            let a1041 = f222({
                let a1021 = f223(v0, 20u32);
                let a1022 = 15u32;
                fixed_u32::bit_and(a1021, a1022)
            });
            let a1042 = {
                let a1039 = f222({
                    let a1023 = f223(v0, 16u32);
                    let a1024 = 15u32;
                    fixed_u32::bit_and(a1023, a1024)
                });
                let a1040 = {
                    let a1037 = f222({
                        let a1025 = f223(v0, 12u32);
                        let a1026 = 15u32;
                        fixed_u32::bit_and(a1025, a1026)
                    });
                    let a1038 = {
                        let a1035 = f222({
                            let a1027 = f223(v0, 8u32);
                            let a1028 = 15u32;
                            fixed_u32::bit_and(a1027, a1028)
                        });
                        let a1036 = {
                            let a1033 = f222({
                                let a1029 = f223(v0, 4u32);
                                let a1030 = 15u32;
                                fixed_u32::bit_and(a1029, a1030)
                            });
                            let a1034 = f222({
                                let a1031 = v0;
                                let a1032 = 15u32;
                                fixed_u32::bit_and(a1031, a1032)
                            });
                            append_bytes(a1033, a1034)
                        };
                        append_bytes(a1035, a1036)
                    };
                    append_bytes(a1037, a1038)
                };
                append_bytes(a1039, a1040)
            };
            append_bytes(a1041, a1042)
        };
        append_bytes(a1043, a1044)
    };
    append_bytes(a1045, a1046)
}

pub fn f165(v0: Fn3, p1050: u64, p1051: (u64, Adt17)) -> R<(u64, Adt17)> {
    let m1049 = (p1050, p1051);
    {
        let mut m1049 = m1049;
        loop {
            let step = {
                let (v1, v2) = m1049;
                let m1047 = v1;
                if m1047 == 0 {
                    Err::<(u64, (u64, Adt17)), R<(u64, Adt17)>>(Ok(v2))
                } else {
                    let v3 = m1047 - 1;
                    Ok::<(u64, (u64, Adt17)), R<(u64, Adt17)>>((v3, {
                        let c1048: Fn3 = v0.clone();
                        c1048.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1049 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt17)) -> R<(u64, Adt17)> {
    Ok(({
        let a1053 = {
            let (h1052, _) = v3.clone();
            h1052
        };
        let a1054 = 64u64;
        nat_add(a1053, a1054)?
    }, f224({
        let (_, h1055) = v3;
        h1055
    }, f225(v0, v1, v2, {
        let (h1056, _) = v3;
        h1056
    })?)?))
}

pub fn f167() -> Adt17 {
    Adt17::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f168(p1061: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m1060 = p1061;
    {
        let mut m1060 = m1060;
        loop {
            let step = {
                let v0 = m1060;
                let m1057 = v0;
                match m1057.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1058, _) = v2;
                            h1058
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h1059) = v2;
                                h1059
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m1065 = {
        let a1063 = match v0 {
            Adt1::C0(_, _, _, h1062) => {
                h1062
            }
        };
        let a1064 = f40(v1);
        index_list(a1063, a1064)
    };
    match m1065 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f170(v0: Bytes, p1086: u64, p1087: u64, p1088: bool, p1089: Bytes) -> R<Bytes> {
    let m1085 = (p1086, (p1087, (p1088, p1089)));
    {
        let mut m1085 = m1085;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1085;
                let m1066 = v2;
                if m1066 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1066 - 1;
                    let m1069 = {
                        let a1067 = v0.clone();
                        let a1068 = v1;
                        index_bytes(a1067, a1068)
                    };
                    match m1069 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f60(v6)?;
                            if f226(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1070 = v1;
                                    let a1071 = 1u64;
                                    nat_add(a1070, a1071)?
                                }, (v5, (true, v4))))
                            } else {
                                if f105(f227(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1072 = v1;
                                        let a1073 = 1u64;
                                        nat_add(a1072, a1073)?
                                    }, (v5, (false, {
                                        let a1077 = v4;
                                        let a1078 = f46({
                                            let a1074 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1074)
                                        }, {
                                            let a1075 = v7;
                                            let a1076 = 97u64;
                                            nat_sub(a1075, a1076)
                                        }, 1u64);
                                        append_bytes(a1077, a1078)
                                    }))))
                                } else {
                                    if f158(f227(v7), f158(f228(v7), f229(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1079 = v1;
                                            let a1080 = 1u64;
                                            nat_add(a1079, a1080)?
                                        }, (v5, (false, {
                                            let a1081 = v4;
                                            let a1082 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1081, a1082)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1083 = v1;
                                            let a1084 = 1u64;
                                            nat_add(a1083, a1084)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1085 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: Adt2) -> R<Bytes> {
    let m1090 = v0;
    match m1090 {
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
            f230(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f231(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f232(v9, v10)
        }
        Adt2::C6(v11) => {
            f233(v11)
        }
        Adt2::C7(r1091) => {
            let v12 = (*r1091).clone();
            Ok(f234(f235(v12)?))
        }
        Adt2::C8(r1092) => {
            let v13 = (*r1092).clone();
            f236(f237(v13)?)
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

pub fn f172(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1093 = v0;
    match m1093.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f32(f115(v3.clone(), v1.clone()), v2.clone()) {
                f172(v4, v1, v2)
            } else {
                List::cons(v3, f172(v4, v1, v2))
            }
        }
    }
}

pub fn f173(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1094 = v0;
    match m1094.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f53(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f151(f238(f175(List::cons(v1, f239(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f173(v3, v1)?))
            }
        }
    }
}

pub fn f174(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1095 = v0.clone();
        length_bytes(a1095)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1096, _) = f64(Fn0::F240(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1096
    };
    let v3: u64 = {
        let (h1097, _) = {
            let (_, h1098) = v2.clone();
            h1098
        };
        h1097
    };
    let v4: List<Bytes> = {
        let (_, h1099) = {
            let (_, h1100) = v2;
            h1100
        };
        h1099
    };
    let a1101 = v3;
    let a1102 = v1;
    if nat_lt(a1101, a1102) {
        Ok(f66(List::cons(f46(v0, v3, {
            let a1103 = v1;
            let a1104 = v3;
            nat_sub(a1103, a1104)
        }), v4)))
    } else {
        Ok(f66(v4))
    }
}

pub fn f175(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f241(f108(Fn2::F242, List::<Bytes>::nil(), f243(v0)?)))
}

pub fn f176(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1105 = v0.clone();
        length_bytes(a1105)
    };
    let v3: u64 = {
        let a1106 = v1.clone();
        length_bytes(a1106)
    };
    let v4: u64 = {
        let (h1107, _) = f179(Fn5::F244(v1.clone(), v3, v0.clone(), v2), {
            let a1108 = v2;
            let a1109 = 1u64;
            nat_add(a1108, a1109)?
        }, 0u64)?;
        h1107
    };
    if f125(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f177(v0: Fn4, p1114: u64, p1115: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1113 = (p1114, p1115);
    {
        let mut m1113 = m1113;
        loop {
            let step = {
                let (v1, v2) = m1113;
                let m1110 = v1;
                if m1110 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1110 - 1;
                    let m1112 = {
                        let c1111: Fn4 = v0.clone();
                        c1111.apply(v2.clone())?
                    };
                    match m1112 {
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
                Ok(next) => { m1113 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f245(v0, v1)
}

pub fn f179(v0: Fn5, p1120: u64, p1121: u64) -> R<(u64, bool)> {
    let m1119 = (p1120, p1121);
    {
        let mut m1119 = m1119;
        loop {
            let step = {
                let (v1, v2) = m1119;
                let m1116 = v1;
                if m1116 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1116 - 1;
                    let m1118 = {
                        let c1117: Fn5 = v0.clone();
                        c1117.apply(v2)?
                    };
                    match m1118 {
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
                Ok(next) => { m1119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f246(v0, v1)
}

pub fn f181(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f247(v0, v1)
}

pub fn f182(v0: Bytes, v1: u64, v2: Str) -> bool {
    f125(v0, {
        let a1122 = v2;
        utf8_encode(a1122)
    }, v1)
}

pub fn f183(v0: Bytes, v1: u64) -> R<bool> {
    if if if f182(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f182(v0.clone(), v1, Str::lit("..."))
    } {
        if f248(f185(v0.clone(), {
            let a1123 = v1;
            let a1124 = 3u64;
            nat_add(a1123, a1124)?
        })?) {
            true
        } else {
            f249(f185(v0.clone(), {
                let a1125 = v1;
                let a1126 = 3u64;
                nat_add(a1125, a1126)?
            })?)
        }
    } else {
        false
    } {
        let a1127 = f250(v0, v1)?;
        let a1128 = 0u64;
        Ok(nat_eq(a1127, a1128))
    } else {
        Ok(false)
    }
}

pub fn f184(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f251(v0.clone(), v1)?;
    let v3: u64 = f185(v0.clone(), v2)?;
    if if f249(v3) {
        true
    } else {
        let a1129 = v3;
        let a1130 = 35u64;
        if nat_eq(a1129, a1130) {
            let a1131 = v1;
            let a1132 = v2;
            nat_lt(a1131, a1132)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f252(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f129(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f185(v0: Bytes, v1: u64) -> R<u64> {
    f103(v0, v1)
}

pub fn f186(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>, v8: Adt18) -> R<Adt19> {
    let m1133 = v3;
    if m1133 == 0 {
        Ok(Adt19::C1(f129(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v9 = m1133 - 1;
        let a1134 = v4;
        let a1135 = 0u64;
        if nat_eq(a1134, a1135) {
            let v10: u64 = f253(v0.clone(), v1)?;
            let v11: u64 = {
                let a1136 = v10;
                let a1137 = v1;
                nat_sub(a1136, a1137)
            };
            let a1138 = f185(v0.clone(), v10)?;
            let a1139 = 9u64;
            if nat_eq(a1138, a1139) {
                let v12: u64 = f251(v0.clone(), v10)?;
                if if f254(v0.clone(), v12)? {
                    true
                } else {
                    f255(v0.clone(), v12)?
                } {
                    Ok(Adt19::C1(f129(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v12)?))
                } else {
                    let m1140 = f256(v0.clone(), v12)?;
                    match m1140 {
                        None => {
                            f257(v0, v12, v2, v8)
                        }
                        Some(v13) => {
                            Ok(Adt19::C1(v13))
                        }
                    }
                }
            } else {
                let a1141 = f185(v0.clone(), v10)?;
                let a1142 = 33u64;
                if if nat_eq(a1141, a1142) {
                    f187(f255(v0.clone(), v10)?)
                } else {
                    false
                } {
                    if f258(v8) {
                        Ok(Adt19::C1(f129(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v10)?))
                    } else {
                        let v14: Adt18 = f259(v0.clone(), v10)?;
                        let m1143 = f260(v14.clone());
                        match m1143 {
                            None => {
                                f186(v0.clone(), f261(v0.clone(), v10)?, v2, v9, 1u64, false, {
                                    let a1145 = {
                                        let a1144 = v0;
                                        length_bytes(a1144)
                                    };
                                    let a1146 = 1u64;
                                    nat_add(a1145, a1146)?
                                }, List::<Bytes>::nil(), v14)
                            }
                            Some(v15) => {
                                Ok(Adt19::C1(v15))
                            }
                        }
                    }
                } else {
                    if f254(v0.clone(), v10)? {
                        f262(v8, true, v10, f186(v0, v10, v2, v9, 2u64, false, v11, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                    } else {
                        let m1149 = {
                            let a1147 = f185(v0.clone(), v10)?;
                            let a1148 = 33u64;
                            if nat_eq(a1147, a1148) {
                                None::<Bytes>
                            } else {
                                f256(v0.clone(), v10)?
                            }
                        };
                        match m1149 {
                            None => {
                                if f255(v0.clone(), v10)? {
                                    f262(v8, false, v10, f186(v0, v10, v2, v9, 3u64, false, v11, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                                } else {
                                    f257(v0, v10, v2, v8)
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
            let a1150 = v4;
            let a1151 = 1u64;
            if nat_eq(a1150, a1151) {
                let v17: u64 = f251(v0.clone(), v1)?;
                let v18: u64 = f185(v0.clone(), v17)?;
                if if f249(v18) {
                    true
                } else {
                    let a1152 = v18;
                    let a1153 = 35u64;
                    if nat_eq(a1152, a1153) {
                        let a1154 = v1;
                        let a1155 = v17;
                        nat_lt(a1154, a1155)
                    } else {
                        false
                    }
                } {
                    let v19: u64 = f128(v0.clone(), f252(v0.clone(), v17)?)?;
                    let a1157 = {
                        let a1156 = v0.clone();
                        length_bytes(a1156)
                    };
                    let a1158 = v19;
                    if if nat_le(a1157, a1158) {
                        true
                    } else {
                        f183(v0.clone(), v19)?
                    } {
                        f263(v8, v17, v19)
                    } else {
                        let v20: u64 = {
                            let a1159 = f253(v0.clone(), v19)?;
                            let a1160 = v19;
                            nat_sub(a1159, a1160)
                        };
                        let a1161 = v2;
                        let a1162 = v20;
                        if nat_le(a1161, a1162) {
                            f186(v0, v19, v2, v9, 0u64, false, 0u64, List::<Bytes>::nil(), v8)
                        } else {
                            if if if f187(v5) {
                                let a1163 = v20;
                                let a1164 = v6;
                                nat_eq(a1163, a1164)
                            } else {
                                false
                            } {
                                f254(v0.clone(), {
                                    let a1165 = v19;
                                    let a1166 = v20;
                                    nat_add(a1165, a1166)?
                                })?
                            } else {
                                false
                            } {
                                f262(v8, true, v17, f186(v0, {
                                    let a1167 = v19;
                                    let a1168 = v20;
                                    nat_add(a1167, a1168)?
                                }, v2, v9, 2u64, false, v20, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                            } else {
                                f263(v8, v17, v19)
                            }
                        }
                    }
                } else {
                    let a1169 = v18;
                    let a1170 = 33u64;
                    if if nat_eq(a1169, a1170) {
                        f258(v8.clone())
                    } else {
                        false
                    } {
                        Ok(Adt19::C1(f129(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v17)?))
                    } else {
                        let a1171 = v18;
                        let a1172 = 33u64;
                        if if nat_eq(a1171, a1172) {
                            f187(if v5 {
                                f255(v0.clone(), v17)?
                            } else {
                                false
                            })
                        } else {
                            false
                        } {
                            let v21: Adt18 = f259(v0.clone(), v17)?;
                            let m1173 = f260(v21.clone());
                            match m1173 {
                                None => {
                                    f186(v0.clone(), f261(v0, v17)?, v2, v9, 1u64, v5, v6, List::<Bytes>::nil(), v21)
                                }
                                Some(v22) => {
                                    Ok(Adt19::C1(v22))
                                }
                            }
                        } else {
                            if if v5 {
                                f254(v0.clone(), v17)?
                            } else {
                                false
                            } {
                                let a1176 = f264(f265(v0.clone(), v1, v17), 9u64)?;
                                let a1177 = {
                                    let a1174 = v17;
                                    let a1175 = v1;
                                    nat_sub(a1174, a1175)
                                };
                                if nat_lt(a1176, a1177) {
                                    Ok(Adt19::C1(f129(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v17)?))
                                } else {
                                    f262(v8, true, v17, f186(v0.clone(), v17, v2, v9, 2u64, false, f250(v0, v17)?, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                                }
                            } else {
                                let m1180 = {
                                    let a1178 = v18;
                                    let a1179 = 33u64;
                                    if nat_eq(a1178, a1179) {
                                        None::<Bytes>
                                    } else {
                                        f256(v0.clone(), v17)?
                                    }
                                };
                                match m1180 {
                                    None => {
                                        if if v5 {
                                            f255(v0.clone(), v17)?
                                        } else {
                                            false
                                        } {
                                            f262(v8, false, v17, f186(v0.clone(), v17, v2, v9, 3u64, false, f250(v0, v17)?, List::<Bytes>::nil(), Adt18::C0(0u64))?)
                                        } else {
                                            f257(v0, v17, v2, v8)
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
                let a1181 = v4;
                let a1182 = 2u64;
                if nat_eq(a1181, a1182) {
                    let m1187 = f186(v0.clone(), {
                        let a1183 = v1;
                        let a1184 = 1u64;
                        nat_add(a1183, a1184)?
                    }, {
                        let a1185 = v6;
                        let a1186 = 1u64;
                        nat_add(a1185, a1186)?
                    }, v9, 1u64, true, v6, List::<Bytes>::nil(), Adt18::C0(0u64))?;
                    match m1187 {
                        Adt19::C0(v25, v26) => {
                            let v27: u64 = f128(v0.clone(), v26)?;
                            let v28: u64 = {
                                let a1188 = f253(v0.clone(), v27)?;
                                let a1189 = v27;
                                nat_sub(a1188, a1189)
                            };
                            let a1191 = {
                                let a1190 = v0.clone();
                                length_bytes(a1190)
                            };
                            let a1192 = v27;
                            if nat_le(a1191, a1192) {
                                Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C9)), v27))
                            } else {
                                let a1193 = v28;
                                let a1194 = v6;
                                if if nat_eq(a1193, a1194) {
                                    f254(v0.clone(), {
                                        let a1195 = v27;
                                        let a1196 = v28;
                                        nat_add(a1195, a1196)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1199 = f186(v0, {
                                        let a1197 = v27;
                                        let a1198 = v28;
                                        nat_add(a1197, a1198)?
                                    }, v2, v9, 2u64, false, v6, List::<Bytes>::nil(), Adt18::C0(0u64))?;
                                    match m1199 {
                                        Adt19::C0(v30, v31) => {
                                            Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v25), std::rc::Rc::new(v30)), v31))
                                        }
                                        Adt19::C1(v29) => {
                                            Ok(Adt19::C1(v29))
                                        }
                                    }
                                } else {
                                    let a1200 = v6;
                                    let a1201 = v28;
                                    if nat_lt(a1200, a1201) {
                                        Ok(Adt19::C1(f129(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1202 = v27;
                                            let a1203 = v28;
                                            nat_add(a1202, a1203)?
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
                    let m1204 = f266(v0.clone(), v1)?;
                    match m1204 {
                        Ok(v33) => {
                            let m1205 = f256(v0.clone(), v33)?;
                            match m1205 {
                                None => {
                                    let m1206 = f267(v0.clone(), v33)?;
                                    match m1206 {
                                        None => {
                                            Ok(Adt19::C1(f129(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v33)?))
                                        }
                                        Some(v35) => {
                                            let v36: Bytes = f268(v0.clone(), v33)?;
                                            if f269(v36.clone(), v7.clone()) {
                                                Ok(Adt19::C1(f129(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                            } else {
                                                let m1211 = f186(v0.clone(), {
                                                    let a1207 = v35;
                                                    let a1208 = 1u64;
                                                    nat_add(a1207, a1208)?
                                                }, {
                                                    let a1209 = v6;
                                                    let a1210 = 1u64;
                                                    nat_add(a1209, a1210)?
                                                }, v9, 1u64, false, v6, List::<Bytes>::nil(), Adt18::C0(0u64))?;
                                                match m1211 {
                                                    Adt19::C0(v38, v39) => {
                                                        let v40: u64 = f128(v0.clone(), v39)?;
                                                        let v41: u64 = {
                                                            let a1212 = f253(v0.clone(), v40)?;
                                                            let a1213 = v40;
                                                            nat_sub(a1212, a1213)
                                                        };
                                                        let a1215 = {
                                                            let a1214 = v0.clone();
                                                            length_bytes(a1214)
                                                        };
                                                        let a1216 = v40;
                                                        if nat_le(a1215, a1216) {
                                                            Ok(Adt19::C0(Adt2::C11(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C9)), v40))
                                                        } else {
                                                            let a1217 = v41;
                                                            let a1218 = v6;
                                                            if if if nat_eq(a1217, a1218) {
                                                                f187(f254(v0.clone(), {
                                                                    let a1219 = v40;
                                                                    let a1220 = v41;
                                                                    nat_add(a1219, a1220)?
                                                                })?)
                                                            } else {
                                                                false
                                                            } {
                                                                f187(f183(v0.clone(), v40)?)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1223 = f186(v0, {
                                                                    let a1221 = v40;
                                                                    let a1222 = v41;
                                                                    nat_add(a1221, a1222)?
                                                                }, v2, v9, 3u64, false, v6, List::cons(v36.clone(), v7), Adt18::C0(0u64))?;
                                                                match m1223 {
                                                                    Adt19::C0(v43, v44) => {
                                                                        Ok(Adt19::C0(Adt2::C11(v36, std::rc::Rc::new(v38), std::rc::Rc::new(v43)), v44))
                                                                    }
                                                                    Adt19::C1(v42) => {
                                                                        Ok(Adt19::C1(v42))
                                                                    }
                                                                }
                                                            } else {
                                                                let a1224 = v6;
                                                                let a1225 = v41;
                                                                if nat_lt(a1224, a1225) {
                                                                    Ok(Adt19::C1(f129(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                        let a1226 = v40;
                                                                        let a1227 = v41;
                                                                        nat_add(a1226, a1227)?
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

pub fn f187(v0: bool) -> bool {
    !v0
}

pub fn f188(v0: Bytes, v1: u64) -> Bytes {
    let m1231 = {
        let a1228 = v0;
        let a1229 = v1;
        let a1230 = 1u64;
        slice_bytes(a1228, a1229, a1230)
    };
    match m1231 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f189(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f270(v1, v0, v2)
}

pub fn f190(v0: Bytes) -> R<Result<List<Adt20>, Bytes>> {
    f271({
        let a1233 = {
            let a1232 = v0.clone();
            length_bytes(a1232)
        };
        let a1234 = 1u64;
        nat_add(a1233, a1234)?
    }, v0, 0u64, List::<Adt20>::nil())
}

pub fn f191(p1242: u64, p1243: List<Adt20>, p1244: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1241 = (p1242, (p1243, p1244));
    {
        let mut m1241 = m1241;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1241;
                let m1235 = v0;
                if m1235 == 0 {
                    Err::<(u64, (List<Adt20>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1235 - 1;
                    let m1236 = v1.clone();
                    match m1236.uncons() {
                        None => {
                            Err::<(u64, (List<Adt20>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f272(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1237 = f273(v1)?;
                            match m1237 {
                                Ok(v7) => {
                                    let m1238 = f274(v3, v7)?;
                                    match m1238 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt20>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1239) = v9;
                                                h1239
                                            }, List::cons({
                                                let (h1240, _) = v9;
                                                h1240
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
                Ok(next) => { m1241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m1245 = v0;
    match m1245.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f32(f204(v2.clone()), Bytes::lit(&[])) {
                f275(f206(v2.clone()), v1.clone())
            } else {
                f204(v2.clone())
            };
            List::cons(Adt16::C0(f194(v2.clone()), v4, f207(v2.clone()), f206(v2)), f192(v3, v1))
        }
    }
}

pub fn f193(v0: List<Adt16>) -> R<u64> {
    let m1246 = v0;
    match m1246.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f32(f204(v1), Bytes::lit(&[])) {
                f193(v2)
            } else {
                let a1247 = f193(v2)?;
                let a1248 = 1u64;
                nat_add(a1247, a1248)
            }
        }
    }
}

pub fn f194(v0: Adt16) -> Bytes {
    let m1249 = v0;
    match m1249 {
        Adt16::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f195(v0: u64) -> bool {
    let a1250 = v0;
    let a1251 = 0u64;
    if nat_eq(a1250, a1251) {
        true
    } else {
        let a1252 = v0;
        let a1253 = 2u64;
        if nat_eq(a1252, a1253) {
            true
        } else {
            let a1254 = v0;
            let a1255 = 3u64;
            if nat_eq(a1254, a1255) {
                true
            } else {
                let a1256 = v0;
                let a1257 = 4u64;
                if nat_eq(a1256, a1257) {
                    true
                } else {
                    let a1258 = v0;
                    let a1259 = 9u64;
                    nat_eq(a1258, a1259)
                }
            }
        }
    }
}

pub fn f196(v0: Adt13) -> u64 {
    let m1260 = v0;
    match m1260 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f197(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f196(v1.clone());
    let v3: Bytes = f276(f198(v1.clone()))?;
    let a1261 = v2;
    let a1262 = 0u64;
    if if nat_eq(a1261, a1262) {
        true
    } else {
        let a1263 = v2;
        let a1264 = 2u64;
        nat_eq(a1263, a1264)
    } {
        let m1265 = f277(v0.clone(), v0.clone(), f198(v1.clone()));
        match m1265 {
            None => {
                let m1269 = f278({
                    let a1267 = {
                        let a1266 = v0.clone();
                        length_list(a1266)
                    };
                    let a1268 = 1u64;
                    nat_add(a1267, a1268)?
                }, v0, v1)?;
                match m1269 {
                    Ok(v5) => {
                        let a1271 = {
                            let a1270 = v5.clone();
                            length_list(a1270)
                        };
                        let a1272 = 0u64;
                        if nat_eq(a1271, a1272) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f279(v3), List::<Bytes>::nil())))
                        } else {
                            let m1273 = f280(f281(f282(v5.clone()))?, v5)?;
                            match m1273 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f283(v3, {
                                        let (h1274, _) = v7;
                                        h1274
                                    }), {
                                        let (_, h1275) = v7;
                                        h1275
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
        let a1276 = v2;
        let a1277 = 3u64;
        if nat_eq(a1276, a1277) {
            let m1278 = f284(f285(v1.clone()));
            match m1278 {
                None => {
                    let m1279 = f286(f281(f285(v1.clone()))?, f287(f285(v1)), List::<Bytes>::nil())?;
                    match m1279 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f288(v3, v10), List::<Bytes>::nil())))
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
            let a1280 = v2;
            let a1281 = 4u64;
            if nat_eq(a1280, a1281) {
                let v12: Adt15 = f202(v1);
                if !f32(f289(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f289(v12)))
                } else {
                    let a1282 = f290(v12.clone());
                    let a1283 = 1u64;
                    if nat_eq(a1282, a1283) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f288(v3, List::cons(f291(v12.clone()), List::<Bytes>::nil())), f292(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f293(v3, f291(v12.clone())), f292(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f285(v1))))
            }
        }
    }
}

pub fn f198(v0: Adt13) -> Bytes {
    let m1284 = v0;
    match m1284 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f199(v0: List<Adt14>) -> List<Adt16> {
    let m1285 = v0;
    match m1285.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v1, v2)) => {
            f142(f201(f294(v1)), f199(v2))
        }
    }
}

pub fn f200(v0: Adt13) -> List<Adt14> {
    let m1286 = v0;
    match m1286 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f201(v0: Adt15) -> List<Adt16> {
    let m1287 = v0;
    match m1287 {
        Adt15::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f202(v0: Adt13) -> Adt15 {
    let m1288 = v0;
    match m1288 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f203(p1291: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    let m1290 = p1291;
    {
        let mut m1290 = m1290;
        loop {
            let step = {
                let v0 = m1290;
                let m1289 = v0;
                match m1289.uncons() {
                    None => {
                        Err::<List<Adt16>, Option<Adt16>>(None::<Adt16>)
                    }
                    Some((v2, v3)) => {
                        if f32(f194(v2.clone()), v1.clone()) {
                            Err::<List<Adt16>, Option<Adt16>>(Some(v2))
                        } else {
                            Ok::<List<Adt16>, Option<Adt16>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1290 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Adt16) -> Bytes {
    let m1292 = v0;
    match m1292 {
        Adt16::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f205(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1293 = v0;
    match m1293.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f205(v3, v1))
        }
    }
}

pub fn f206(v0: Adt16) -> List<Bytes> {
    let m1294 = v0;
    match m1294 {
        Adt16::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f207(v0: Adt16) -> Bytes {
    let m1295 = v0;
    match m1295 {
        Adt16::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f208(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1296 = v0;
    match m1296.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1299 = f295({
                let (h1297, _) = {
                    let (_, h1298) = v2.clone();
                    h1298
                };
                h1297
            }, v1.clone());
            match m1299 {
                None => {
                    Some((({
                        let (h1300, _) = v2;
                        h1300
                    }, {
                        let (_, h1301) = {
                            let (_, h1302) = v2;
                            h1302
                        };
                        h1301
                    }), v3))
                }
                Some(_) => {
                    let m1303 = f208(v3, v1);
                    match m1303 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1304, _) = v5;
                                h1304
                            }, List::cons(v2, {
                                let (_, h1305) = v5;
                                h1305
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f209(p1310: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1309 = p1310;
    {
        let mut m1309 = m1309;
        loop {
            let step = {
                let v0 = m1309;
                let m1306 = v0;
                match m1306.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1307 = f196(v3.clone());
                        let a1308 = v1;
                        if if nat_eq(a1307, a1308) {
                            f32(f198(v3.clone()), v2.clone())
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
                Ok(next) => { m1309 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f210(v0: u64) -> Bytes {
    let a1311 = v0;
    let a1312 = 5u64;
    if nat_eq(a1311, a1312) {
        f8(Str::lit("partial interface "))
    } else {
        let a1313 = v0;
        let a1314 = 7u64;
        if nat_eq(a1313, a1314) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f211(v0: Adt13) -> Bytes {
    let m1315 = v0;
    match m1315 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f212(v0: Adt2) -> Option<Bytes> {
    let m1316 = v0;
    match m1316 {
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
        Adt2::C7(r1319) => {
            let v3 = (*r1319).clone();
            f212(v3)
        }
        Adt2::C8(r1320) => {
            let v4 = (*r1320).clone();
            f212(v4)
        }
        Adt2::C9 => {
            None::<Bytes>
        }
        Adt2::C10(r1321, r1322) => {
            let v5 = (*r1321).clone();
            let v6 = (*r1322).clone();
            let m1317 = f212(v5);
            match m1317 {
                None => {
                    f212(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C11(_, r1323, r1324) => {
            let v9 = (*r1323).clone();
            let v10 = (*r1324).clone();
            let m1318 = f212(v9);
            match m1318 {
                None => {
                    f212(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f213(v0: Adt2) -> R<Option<Bytes>> {
    let m1325 = v0;
    match m1325 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, r1328) => {
            let v2 = (*r1328).clone();
            f213(v2)
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
            if f296(v18, v19)? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f232(v18, v19)?))
            }
        }
        Adt2::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(r1329) => {
            let v3 = (*r1329).clone();
            f213(v3)
        }
        Adt2::C8(r1330) => {
            let v4 = (*r1330).clone();
            f213(v4)
        }
        Adt2::C9 => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(r1331, r1332) => {
            let v5 = (*r1331).clone();
            let v6 = (*r1332).clone();
            let m1326 = f213(v5)?;
            match m1326 {
                None => {
                    f213(v6)
                }
                Some(v7) => {
                    Ok(Some(v7))
                }
            }
        }
        Adt2::C11(_, r1333, r1334) => {
            let v9 = (*r1333).clone();
            let v10 = (*r1334).clone();
            let m1327 = f213(v9)?;
            match m1327 {
                None => {
                    f213(v10)
                }
                Some(v11) => {
                    Ok(Some(v11))
                }
            }
        }
    }
}

pub fn f214(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f103(v0.clone(), {
        let a1335 = v1;
        let a1336 = 1u64;
        nat_add(a1335, a1336)?
    })?;
    let v4: Bytes = {
        let a1337 = v3;
        let a1338 = 117u64;
        if nat_eq(a1337, a1338) {
            f297(v0, v1)?
        } else {
            f298(v3)
        }
    };
    let a1339 = v4.clone();
    let a1340 = Bytes::lit(&[]);
    if equal(a1339, a1340) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1341 = v3;
            let a1342 = 117u64;
            if nat_eq(a1341, a1342) {
                let a1343 = v1;
                let a1344 = 6u64;
                nat_add(a1343, a1344)?
            } else {
                let a1345 = v1;
                let a1346 = 2u64;
                nat_add(a1345, a1346)?
            }
        }, ({
            let a1347 = v2;
            let a1348 = v4;
            append_bytes(a1347, a1348)
        }, 0u64))))
    }
}

pub fn f215(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f299(v0, v1)
}

pub fn f216(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f300(v0, v1)
}

pub fn f217(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1349 = v0.clone();
        length_bytes(a1349)
    };
    let a1350 = 0u64;
    let a1351 = v1;
    if nat_lt(a1350, a1351) {
        let a1352 = f301(v0.clone())?;
        let a1353 = v1;
        if nat_eq(a1352, a1353) {
            let a1354 = 1u64;
            let a1355 = v1;
            if nat_lt(a1354, a1355) {
                let m1358 = {
                    let a1356 = v0;
                    let a1357 = 0u64;
                    index_bytes(a1356, a1357)
                };
                match m1358 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1359 = v2;
                        let a1360 = 48u8;
                        Ok(!equal(a1359, a1360))
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

pub fn f218(v0: Fn6, p1365: u64, p1366: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1364 = (p1365, p1366);
    {
        let mut m1364 = m1364;
        loop {
            let step = {
                let (v1, v2) = m1364;
                let m1361 = v1;
                if m1361 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1361 - 1;
                    let m1363 = {
                        let c1362: Fn6 = v0.clone();
                        c1362.apply(v2)?
                    };
                    match m1363 {
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
                Ok(next) => { m1364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f302(v0, v1)
}

pub fn f220(v0: Bytes) -> R<u64> {
    let (h1367, _) = f179(Fn5::F303(v0.clone()), {
        let a1369 = {
            let a1368 = v0;
            length_bytes(a1368)
        };
        let a1370 = 1u64;
        nat_add(a1369, a1370)?
    }, 0u64)?;
    Ok(h1367)
}

pub fn f221(v0: u64) -> Bytes {
    let m1371 = v0;
    if m1371 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1371 - 1;
        let a1372 = f8(Str::lit("0"));
        let a1373 = f221(v1);
        append_bytes(a1372, a1373)
    }
}

pub fn f222(v0: u32) -> Bytes {
    let a1376 = {
        let a1374 = v0;
        let a1375 = 8u32;
        fixed_u32::bit_and(a1374, a1375)
    };
    let a1377 = 0u32;
    if equal(a1376, a1377) {
        let a1380 = {
            let a1378 = v0;
            let a1379 = 4u32;
            fixed_u32::bit_and(a1378, a1379)
        };
        let a1381 = 0u32;
        if equal(a1380, a1381) {
            let a1384 = {
                let a1382 = v0;
                let a1383 = 2u32;
                fixed_u32::bit_and(a1382, a1383)
            };
            let a1385 = 0u32;
            if equal(a1384, a1385) {
                let a1388 = {
                    let a1386 = v0;
                    let a1387 = 1u32;
                    fixed_u32::bit_and(a1386, a1387)
                };
                let a1389 = 0u32;
                if equal(a1388, a1389) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1392 = {
                    let a1390 = v0;
                    let a1391 = 1u32;
                    fixed_u32::bit_and(a1390, a1391)
                };
                let a1393 = 0u32;
                if equal(a1392, a1393) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1396 = {
                let a1394 = v0;
                let a1395 = 2u32;
                fixed_u32::bit_and(a1394, a1395)
            };
            let a1397 = 0u32;
            if equal(a1396, a1397) {
                let a1400 = {
                    let a1398 = v0;
                    let a1399 = 1u32;
                    fixed_u32::bit_and(a1398, a1399)
                };
                let a1401 = 0u32;
                if equal(a1400, a1401) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1404 = {
                    let a1402 = v0;
                    let a1403 = 1u32;
                    fixed_u32::bit_and(a1402, a1403)
                };
                let a1405 = 0u32;
                if equal(a1404, a1405) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1408 = {
            let a1406 = v0;
            let a1407 = 4u32;
            fixed_u32::bit_and(a1406, a1407)
        };
        let a1409 = 0u32;
        if equal(a1408, a1409) {
            let a1412 = {
                let a1410 = v0;
                let a1411 = 2u32;
                fixed_u32::bit_and(a1410, a1411)
            };
            let a1413 = 0u32;
            if equal(a1412, a1413) {
                let a1416 = {
                    let a1414 = v0;
                    let a1415 = 1u32;
                    fixed_u32::bit_and(a1414, a1415)
                };
                let a1417 = 0u32;
                if equal(a1416, a1417) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1420 = {
                    let a1418 = v0;
                    let a1419 = 1u32;
                    fixed_u32::bit_and(a1418, a1419)
                };
                let a1421 = 0u32;
                if equal(a1420, a1421) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1424 = {
                let a1422 = v0;
                let a1423 = 2u32;
                fixed_u32::bit_and(a1422, a1423)
            };
            let a1425 = 0u32;
            if equal(a1424, a1425) {
                let a1428 = {
                    let a1426 = v0;
                    let a1427 = 1u32;
                    fixed_u32::bit_and(a1426, a1427)
                };
                let a1429 = 0u32;
                if equal(a1428, a1429) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1432 = {
                    let a1430 = v0;
                    let a1431 = 1u32;
                    fixed_u32::bit_and(a1430, a1431)
                };
                let a1433 = 0u32;
                if equal(a1432, a1433) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f223(v0: u32, v1: u32) -> u32 {
    let m1436 = {
        let a1434 = v0;
        let a1435 = v1;
        fixed_u32::shift_right(a1434, a1435)
    };
    match m1436 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f224(v0: Adt17, v1: Adt21) -> R<Adt17> {
    let v2: Adt22 = f304(Fn7::F305, 64u64, Adt22::C0(match v0.clone() {
        Adt17::C0(h1437, _, _, _, _, _, _, _) => {
            h1437
        }
    }, match v0.clone() {
        Adt17::C0(_, h1438, _, _, _, _, _, _) => {
            h1438
        }
    }, match v0.clone() {
        Adt17::C0(_, _, h1439, _, _, _, _, _) => {
            h1439
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h1440, _, _, _, _) => {
            h1440
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, h1441, _, _, _) => {
            h1441
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, h1442, _, _) => {
            h1442
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, h1443, _) => {
            h1443
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, _, h1444) => {
            h1444
        }
    }, v1, 0u64))?;
    Ok(Adt17::C0(f306(match v0.clone() {
        Adt17::C0(h1445, _, _, _, _, _, _, _) => {
            h1445
        }
    }, match v2.clone() {
        Adt22::C0(h1446, _, _, _, _, _, _, _, _, _) => {
            h1446
        }
    }), f306(match v0.clone() {
        Adt17::C0(_, h1447, _, _, _, _, _, _) => {
            h1447
        }
    }, match v2.clone() {
        Adt22::C0(_, h1448, _, _, _, _, _, _, _, _) => {
            h1448
        }
    }), f306(match v0.clone() {
        Adt17::C0(_, _, h1449, _, _, _, _, _) => {
            h1449
        }
    }, match v2.clone() {
        Adt22::C0(_, _, h1450, _, _, _, _, _, _, _) => {
            h1450
        }
    }), f306(match v0.clone() {
        Adt17::C0(_, _, _, h1451, _, _, _, _) => {
            h1451
        }
    }, match v2.clone() {
        Adt22::C0(_, _, _, h1452, _, _, _, _, _, _) => {
            h1452
        }
    }), f306(match v0.clone() {
        Adt17::C0(_, _, _, _, h1453, _, _, _) => {
            h1453
        }
    }, match v2.clone() {
        Adt22::C0(_, _, _, _, h1454, _, _, _, _, _) => {
            h1454
        }
    }), f306(match v0.clone() {
        Adt17::C0(_, _, _, _, _, h1455, _, _) => {
            h1455
        }
    }, match v2.clone() {
        Adt22::C0(_, _, _, _, _, h1456, _, _, _, _) => {
            h1456
        }
    }), f306(match v0.clone() {
        Adt17::C0(_, _, _, _, _, _, h1457, _) => {
            h1457
        }
    }, match v2.clone() {
        Adt22::C0(_, _, _, _, _, _, h1458, _, _, _) => {
            h1458
        }
    }), f306(match v0 {
        Adt17::C0(_, _, _, _, _, _, _, h1459) => {
            h1459
        }
    }, match v2 {
        Adt22::C0(_, _, _, _, _, _, _, h1460, _, _) => {
            h1460
        }
    })))
}

pub fn f225(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt21> {
    Ok(Adt21::C0(f307(v0.clone(), v1, v2, v3)?, f307(v0.clone(), v1, v2, {
        let a1461 = v3;
        let a1462 = 4u64;
        nat_add(a1461, a1462)?
    })?, f307(v0.clone(), v1, v2, {
        let a1463 = v3;
        let a1464 = 8u64;
        nat_add(a1463, a1464)?
    })?, f307(v0.clone(), v1, v2, {
        let a1465 = v3;
        let a1466 = 12u64;
        nat_add(a1465, a1466)?
    })?, f307(v0.clone(), v1, v2, {
        let a1467 = v3;
        let a1468 = 16u64;
        nat_add(a1467, a1468)?
    })?, f307(v0.clone(), v1, v2, {
        let a1469 = v3;
        let a1470 = 20u64;
        nat_add(a1469, a1470)?
    })?, f307(v0.clone(), v1, v2, {
        let a1471 = v3;
        let a1472 = 24u64;
        nat_add(a1471, a1472)?
    })?, f307(v0.clone(), v1, v2, {
        let a1473 = v3;
        let a1474 = 28u64;
        nat_add(a1473, a1474)?
    })?, f307(v0.clone(), v1, v2, {
        let a1475 = v3;
        let a1476 = 32u64;
        nat_add(a1475, a1476)?
    })?, f307(v0.clone(), v1, v2, {
        let a1477 = v3;
        let a1478 = 36u64;
        nat_add(a1477, a1478)?
    })?, f307(v0.clone(), v1, v2, {
        let a1479 = v3;
        let a1480 = 40u64;
        nat_add(a1479, a1480)?
    })?, f307(v0.clone(), v1, v2, {
        let a1481 = v3;
        let a1482 = 44u64;
        nat_add(a1481, a1482)?
    })?, f307(v0.clone(), v1, v2, {
        let a1483 = v3;
        let a1484 = 48u64;
        nat_add(a1483, a1484)?
    })?, f307(v0.clone(), v1, v2, {
        let a1485 = v3;
        let a1486 = 52u64;
        nat_add(a1485, a1486)?
    })?, f307(v0.clone(), v1, v2, {
        let a1487 = v3;
        let a1488 = 56u64;
        nat_add(a1487, a1488)?
    })?, f307(v0, v1, v2, {
        let a1489 = v3;
        let a1490 = 60u64;
        nat_add(a1489, a1490)?
    })?))
}

pub fn f226(v0: u64) -> bool {
    f158({
        let a1491 = v0;
        let a1492 = 95u64;
        nat_eq(a1491, a1492)
    }, f158({
        let a1493 = v0;
        let a1494 = 45u64;
        nat_eq(a1493, a1494)
    }, f158({
        let a1495 = v0;
        let a1496 = 32u64;
        nat_eq(a1495, a1496)
    }, f158({
        let a1497 = v0;
        let a1498 = 46u64;
        nat_eq(a1497, a1498)
    }, {
        let a1499 = v0;
        let a1500 = 47u64;
        nat_eq(a1499, a1500)
    }))))
}

pub fn f227(v0: u64) -> bool {
    f105({
        let a1501 = 97u64;
        let a1502 = v0;
        nat_le(a1501, a1502)
    }, {
        let a1503 = v0;
        let a1504 = 122u64;
        nat_le(a1503, a1504)
    })
}

pub fn f228(v0: u64) -> bool {
    f105({
        let a1505 = 65u64;
        let a1506 = v0;
        nat_le(a1505, a1506)
    }, {
        let a1507 = v0;
        let a1508 = 90u64;
        nat_le(a1507, a1508)
    })
}

pub fn f229(v0: u64) -> bool {
    f105({
        let a1509 = 48u64;
        let a1510 = v0;
        nat_le(a1509, a1510)
    }, {
        let a1511 = v0;
        let a1512 = 57u64;
        nat_le(a1511, a1512)
    })
}

pub fn f230(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1513 = 0u64;
        let a1514 = v1;
        nat_lt(a1513, a1514)
    } else {
        false
    } {
        let a1515 = Bytes::lit(&[0x2d]);
        let a1516 = f89(v1)?;
        Ok(append_bytes(a1515, a1516))
    } else {
        f89(v1)
    }
}

pub fn f231(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f89(v1)?;
    let v4: u64 = {
        let a1517 = v3.clone();
        length_bytes(a1517)
    };
    let v5: Bytes = {
        let a1518 = v2;
        let a1519 = v4;
        if nat_lt(a1518, a1519) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1520 = v4;
                let a1521 = v2;
                nat_sub(a1520, a1521)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1522 = v4;
                let a1523 = v2;
                nat_sub(a1522, a1523)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f221({
                let a1524 = v2;
                let a1525 = v4;
                nat_sub(a1524, a1525)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1526 = f8(Str::lit("-"));
        let a1527 = v5;
        Ok(append_bytes(a1526, a1527))
    } else {
        Ok(v5)
    }
}

pub fn f232(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f308(v0)?;
    let v3: bool = {
        let a1528 = f309(v0)?;
        let a1529 = v1;
        nat_le(a1528, a1529)
    };
    let v4: u64 = {
        let a1533 = {
            let a1530 = v1;
            let a1531 = v2;
            let a1532 = 0u64;
            nat_quot(a1530, a1531, a1532)
        };
        let a1534 = f310(f311(v0))?;
        let a1535 = 0u64;
        nat_rem(a1533, a1534, a1535)
    };
    let v5: u64 = {
        let a1536 = v1;
        let a1537 = v2;
        let a1538 = 0u64;
        nat_rem(a1536, a1537, a1538)
    };
    let a1539 = v4;
    let a1540 = f312(v0)?;
    if nat_eq(a1539, a1540) {
        let a1541 = v5;
        let a1542 = 0u64;
        if nat_eq(a1541, a1542) {
            Ok(f313(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f313(v3, {
                let a1546 = {
                    let a1543 = v2;
                    let a1544 = 2u64;
                    let a1545 = 0u64;
                    nat_quot(a1543, a1544, a1545)
                };
                let a1547 = v5;
                if nat_le(a1546, a1547) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f105({
            let a1548 = v4;
            let a1549 = 0u64;
            nat_eq(a1548, a1549)
        }, {
            let a1550 = v5;
            let a1551 = 0u64;
            nat_eq(a1550, a1551)
        }) {
            Ok(f313(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1552 = v4;
                let a1553 = 0u64;
                if nat_eq(a1552, a1553) {
                    v5
                } else {
                    let a1554 = v2;
                    let a1555 = v5;
                    nat_add(a1554, a1555)?
                }
            };
            let v7: u64 = {
                let a1556 = v4;
                let a1557 = 0u64;
                if nat_eq(a1556, a1557) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1564 = {
                    let a1560 = {
                        let a1558 = v7;
                        let a1559 = 2000u64;
                        nat_add(a1558, a1559)?
                    };
                    let a1561 = f314(v0);
                    nat_sub(a1560, a1561)
                };
                let a1565 = {
                    let a1562 = f315(v0);
                    let a1563 = 1u64;
                    nat_sub(a1562, a1563)
                };
                nat_sub(a1564, a1565)
            };
            Ok(f313(v3, {
                let a1566 = 2000u64;
                let a1567 = v8;
                if nat_le(a1566, a1567) {
                    f316(v6, true, {
                        let a1568 = v8;
                        let a1569 = 2000u64;
                        nat_sub(a1568, a1569)
                    })?
                } else {
                    f316(v6, false, {
                        let a1570 = 2000u64;
                        let a1571 = v8;
                        nat_sub(a1570, a1571)
                    })?
                }
            }))
        }
    }
}

pub fn f233(v0: Bytes) -> R<Bytes> {
    let a1576 = f155(v0.clone(), 0u64, {
        let a1573 = {
            let a1572 = v0.clone();
            length_bytes(a1572)
        };
        let a1574 = 1u64;
        nat_add(a1573, a1574)?
    })?;
    let a1577 = {
        let a1575 = v0.clone();
        length_bytes(a1575)
    };
    if nat_eq(a1576, a1577) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1583 = {
            let a1581 = Bytes::lit(&[0x22]);
            let a1582 = f317(v0.clone(), 0u64, {
                let a1579 = {
                    let a1578 = v0;
                    length_bytes(a1578)
                };
                let a1580 = 1u64;
                nat_add(a1579, a1580)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1581, a1582)
        };
        let a1584 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1583, a1584))
    }
}

pub fn f234(v0: List<Bytes>) -> Bytes {
    let a1587 = {
        let a1585 = Bytes::lit(&[0x5b]);
        let a1586 = f47(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1585, a1586)
    };
    let a1588 = Bytes::lit(&[0x5d]);
    append_bytes(a1587, a1588)
}

pub fn f235(v0: Adt2) -> R<List<Bytes>> {
    let m1589 = v0;
    match m1589 {
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
        Adt2::C10(r1590, r1591) => {
            let v14 = (*r1590).clone();
            let v15 = (*r1591).clone();
            Ok(List::cons(f171(v14)?, f235(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f236(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f318(Fn8::F319, List::<(Bytes, Bytes)>::nil(), f320(v0)?);
    let v2: List<Bytes> = f321(Fn9::F322, List::<Bytes>::nil(), v1)?;
    let a1594 = {
        let a1592 = Bytes::lit(&[0x7b]);
        let a1593 = f47(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1592, a1593)
    };
    let a1595 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1594, a1595))
}

pub fn f237(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1596 = v0;
    match m1596 {
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
        Adt2::C11(v16, r1597, r1598) => {
            let v17 = (*r1597).clone();
            let v18 = (*r1598).clone();
            Ok(List::cons((v16, f171(v17)?), f237(v18)?))
        }
    }
}

pub fn f238(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1599 = v0;
    match m1599.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f323(v1)?, f238(v2)?))
        }
    }
}

pub fn f239(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1600 = v0;
    match m1600.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f239(v3, false))
            } else {
                f239(v3, true)
            }
        }
    }
}

pub fn f240(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f324(v0, v1)
}

pub fn f241(v0: List<Bytes>) -> List<Bytes> {
    f108(Fn2::F325, List::<Bytes>::nil(), v0)
}

pub fn f242(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f326(v0, v1)
}

pub fn f243(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f327(f328(f329(v0))?))
}

pub fn f244(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f125(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1603 = {
            let a1601 = v4;
            let a1602 = v1;
            nat_add(a1601, a1602)?
        };
        let a1604 = v3;
        if nat_lt(a1603, a1604) {
            Ok(Some({
                let a1605 = v4;
                let a1606 = 1u64;
                nat_add(a1605, a1606)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f245(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1607, _) = v1;
        h1607
    };
    let a1609 = {
        let a1608 = v0.clone();
        length_bytes(a1608)
    };
    let a1610 = v2;
    if nat_le(a1609, a1610) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1611 = f330(v0.clone(), v2)?;
        let a1612 = 13u64;
        if nat_eq(a1611, a1612) {
            Ok(Some(({
                let a1615 = f330(v0, {
                    let a1613 = v2;
                    let a1614 = 1u64;
                    nat_add(a1613, a1614)?
                })?;
                let a1616 = 10u64;
                if nat_eq(a1615, a1616) {
                    let a1617 = v2;
                    let a1618 = 2u64;
                    nat_add(a1617, a1618)?
                } else {
                    let a1619 = v2;
                    let a1620 = 1u64;
                    nat_add(a1619, a1620)?
                }
            }, {
                let a1622 = {
                    let (_, h1621) = v1;
                    h1621
                };
                let a1623 = f8(Str::lit("\u{a}"));
                append_bytes(a1622, a1623)
            })))
        } else {
            let v4: u64 = {
                let m1624 = f331(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1624 {
                    None => {
                        let a1625 = v0.clone();
                        length_bytes(a1625)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1629 = {
                    let (_, h1626) = v1;
                    h1626
                };
                let a1630 = f46(v0, v2, {
                    let a1627 = v4;
                    let a1628 = v2;
                    nat_sub(a1627, a1628)
                });
                append_bytes(a1629, a1630)
            })))
        }
    }
}

pub fn f246(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1631 = f332(v0, v1)?;
    match m1631 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f333({
                let (h1632, _) = v2;
                h1632
            }) {
                Ok(Some({
                    let a1634 = v1;
                    let a1635 = {
                        let (_, h1633) = v2;
                        h1633
                    };
                    nat_add(a1634, a1635)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f247(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1637 = v1;
    let a1638 = {
        let a1636 = v0.clone();
        length_bytes(a1636)
    };
    if if nat_lt(a1637, a1638) {
        f334(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f252(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f248(v0: u64) -> bool {
    let a1639 = v0;
    let a1640 = 32u64;
    if nat_eq(a1639, a1640) {
        true
    } else {
        let a1641 = v0;
        let a1642 = 9u64;
        nat_eq(a1641, a1642)
    }
}

pub fn f249(v0: u64) -> bool {
    let a1643 = v0;
    let a1644 = 10u64;
    if nat_eq(a1643, a1644) {
        true
    } else {
        let a1645 = v0;
        let a1646 = 256u64;
        nat_eq(a1645, a1646)
    }
}

pub fn f250(v0: Bytes, v1: u64) -> R<u64> {
    let a1647 = v1;
    let a1648 = f335(v0, v1)?;
    Ok(nat_sub(a1647, a1648))
}

pub fn f251(v0: Bytes, v1: u64) -> R<u64> {
    let (h1649, _) = f179(Fn5::F336(v0.clone()), {
        let a1651 = {
            let a1650 = v0;
            length_bytes(a1650)
        };
        let a1652 = 1u64;
        nat_add(a1651, a1652)?
    }, v1)?;
    Ok(h1649)
}

pub fn f252(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f337(v0.clone(), v1)?;
    let a1654 = v2;
    let a1655 = {
        let a1653 = v0;
        length_bytes(a1653)
    };
    if nat_lt(a1654, a1655) {
        let a1656 = v2;
        let a1657 = 1u64;
        nat_add(a1656, a1657)
    } else {
        Ok(v2)
    }
}

pub fn f253(v0: Bytes, v1: u64) -> R<u64> {
    let (h1658, _) = f179(Fn5::F338(v0.clone()), {
        let a1660 = {
            let a1659 = v0;
            length_bytes(a1659)
        };
        let a1661 = 1u64;
        nat_add(a1660, a1661)?
    }, v1)?;
    Ok(h1658)
}

pub fn f254(v0: Bytes, v1: u64) -> R<bool> {
    let a1662 = f185(v0.clone(), v1)?;
    let a1663 = 45u64;
    if nat_eq(a1662, a1663) {
        if f248(f185(v0.clone(), {
            let a1664 = v1;
            let a1665 = 1u64;
            nat_add(a1664, a1665)?
        })?) {
            Ok(true)
        } else {
            Ok(f249(f185(v0, {
                let a1666 = v1;
                let a1667 = 1u64;
                nat_add(a1666, a1667)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f255(v0: Bytes, v1: u64) -> R<bool> {
    let m1668 = f267(v0, v1)?;
    match m1668 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f256(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f185(v0.clone(), v1)?;
    let v3: bool = f187(f339(f185(v0, {
        let a1669 = v1;
        let a1670 = 1u64;
        nat_add(a1669, a1670)?
    })?, true));
    let a1671 = v2;
    let a1672 = 58u64;
    if if nat_eq(a1671, a1672) {
        v3
    } else {
        false
    } {
        Ok(Some(f129(Str::lit("a null key is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1673 = v2;
        let a1674 = 38u64;
        if nat_eq(a1673, a1674) {
            Ok(Some(f129(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1675 = v2;
            let a1676 = 42u64;
            if nat_eq(a1675, a1676) {
                Ok(Some(f129(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1677 = v2;
                let a1678 = 33u64;
                if nat_eq(a1677, a1678) {
                    Ok(Some(f129(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1679 = v2;
                    let a1680 = 63u64;
                    if if nat_eq(a1679, a1680) {
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

pub fn f257(v0: Bytes, v1: u64, v2: u64, v3: Adt18) -> R<Adt19> {
    let v4: u64 = f185(v0.clone(), v1)?;
    let a1681 = v4;
    let a1682 = 124u64;
    if if nat_eq(a1681, a1682) {
        true
    } else {
        let a1683 = v4;
        let a1684 = 62u64;
        nat_eq(a1683, a1684)
    } {
        let m1685 = f340(v0, v1, v2)?;
        match m1685 {
            Adt19::C0(v6, v7) => {
                let m1686 = f72(Some(v6.clone()));
                match m1686 {
                    None => {
                        Ok(Adt19::C0(v6, v7))
                    }
                    Some(v8) => {
                        f341(v3, v8, true, v1, v7)
                    }
                }
            }
            Adt19::C1(v5) => {
                Ok(Adt19::C1(v5))
            }
        }
    } else {
        let a1687 = v4;
        let a1688 = 34u64;
        if if nat_eq(a1687, a1688) {
            true
        } else {
            let a1689 = v4;
            let a1690 = 39u64;
            nat_eq(a1689, a1690)
        } {
            let m1691 = f342(v0.clone(), v1, v2, false)?;
            match m1691 {
                Adt23::C0(v10, v11) => {
                    let m1692 = f184(v0, v11)?;
                    match m1692 {
                        Ok(v13) => {
                            f341(v3, v10, true, v1, v13)
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
            let a1693 = v4;
            let a1694 = 91u64;
            if if nat_eq(a1693, a1694) {
                true
            } else {
                let a1695 = v4;
                let a1696 = 123u64;
                nat_eq(a1695, a1696)
            } {
                let m1700 = f343(v0.clone(), v1, v2, {
                    let a1698 = {
                        let a1697 = v0.clone();
                        length_bytes(a1697)
                    };
                    let a1699 = 2u64;
                    nat_add(a1698, a1699)?
                }, 0u64, List::<Bytes>::nil(), v3)?;
                match m1700 {
                    Adt19::C0(v15, v16) => {
                        let a1701 = f185(v0.clone(), f251(v0.clone(), v16)?)?;
                        let a1702 = 58u64;
                        if nat_eq(a1701, a1702) {
                            Ok(Adt19::C1(f129(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1703 = f184(v0, v16)?;
                            match m1703 {
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
                let m1704 = f256(v0.clone(), v1)?;
                match m1704 {
                    None => {
                        if f187(f344(v0.clone(), v1, false)?) {
                            Ok(Adt19::C1(f129(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v20: u64 = f345(v0.clone(), v1, false)?;
                            let m1708 = f346(v0.clone(), v20, v2, {
                                let a1706 = {
                                    let a1705 = v0.clone();
                                    length_bytes(a1705)
                                };
                                let a1707 = 2u64;
                                nat_add(a1706, a1707)?
                            }, f265(v0, v1, v20))?;
                            match m1708 {
                                Adt23::C0(v22, v23) => {
                                    f341(v3, v22, false, v1, v23)
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

pub fn f258(v0: Adt18) -> bool {
    let m1709 = v0;
    match m1709 {
        Adt18::C0(v2) => {
            let a1710 = 0u64;
            let a1711 = v2;
            nat_lt(a1710, a1711)
        }
        Adt18::C1(_) => {
            true
        }
        Adt18::C2(_) => {
            true
        }
    }
}

pub fn f259(v0: Bytes, v1: u64) -> R<Adt18> {
    let v2: u64 = f347(v0.clone(), v1)?;
    let a1714 = f185(v0.clone(), {
        let a1712 = v1;
        let a1713 = 1u64;
        nat_add(a1712, a1713)?
    })?;
    let a1715 = 60u64;
    if nat_eq(a1714, a1715) {
        let a1716 = f185(v0.clone(), v2)?;
        let a1717 = 62u64;
        if nat_eq(a1716, a1717) {
            let v3: Bytes = f265(v0, {
                let a1718 = v1;
                let a1719 = 2u64;
                nat_add(a1718, a1719)?
            }, v2);
            if f182(v3.clone(), 0u64, Str::lit("tag:yaml.org,2002:")) {
                f348(f265(v3.clone(), 18u64, {
                    let a1720 = v3;
                    length_bytes(a1720)
                }), v1)
            } else {
                Ok(Adt18::C1(v3))
            }
        } else {
            Ok(Adt18::C2(f129(Str::lit("a verbatim tag must end with > (YAML 1.2.2 [98])"), v1)?))
        }
    } else {
        let a1723 = v2;
        let a1724 = {
            let a1721 = v1;
            let a1722 = 1u64;
            nat_add(a1721, a1722)?
        };
        if nat_eq(a1723, a1724) {
            Ok(Adt18::C0(8u64))
        } else {
            let a1727 = f185(v0.clone(), {
                let a1725 = v1;
                let a1726 = 1u64;
                nat_add(a1725, a1726)?
            })?;
            let a1728 = 33u64;
            if nat_eq(a1727, a1728) {
                f348(f265(v0, {
                    let a1729 = v1;
                    let a1730 = 2u64;
                    nat_add(a1729, a1730)?
                }, v2), v1)
            } else {
                let a1737 = f264(f265(v0.clone(), {
                    let a1731 = v1;
                    let a1732 = 1u64;
                    nat_add(a1731, a1732)?
                }, v2), 33u64)?;
                let a1738 = {
                    let a1735 = v2;
                    let a1736 = {
                        let a1733 = v1;
                        let a1734 = 1u64;
                        nat_add(a1733, a1734)?
                    };
                    nat_sub(a1735, a1736)
                };
                if nat_lt(a1737, a1738) {
                    Ok(Adt18::C2(f129(Str::lit("a named tag handle is not read: it needs a TAG directive (YAML 1.2.2 [92], [82])"), v1)?))
                } else {
                    Ok(Adt18::C1(f265(v0, v1, v2)))
                }
            }
        }
    }
}

pub fn f260(v0: Adt18) -> Option<Bytes> {
    let m1739 = v0;
    match m1739 {
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

pub fn f261(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f347(v0.clone(), v1)?;
    let a1742 = f185(v0.clone(), {
        let a1740 = v1;
        let a1741 = 1u64;
        nat_add(a1740, a1741)?
    })?;
    let a1743 = 60u64;
    if if nat_eq(a1742, a1743) {
        let a1744 = f185(v0, v2)?;
        let a1745 = 62u64;
        nat_eq(a1744, a1745)
    } else {
        false
    } {
        let a1746 = v2;
        let a1747 = 1u64;
        nat_add(a1746, a1747)
    } else {
        Ok(v2)
    }
}

pub fn f262(v0: Adt18, v1: bool, v2: u64, v3: Adt19) -> R<Adt19> {
    let m1748 = v3;
    match m1748 {
        Adt19::C0(v5, v6) => {
            let v7: Adt2 = if v1 {
                Adt2::C7(std::rc::Rc::new(v5))
            } else {
                Adt2::C8(std::rc::Rc::new(v5))
            };
            let m1749 = v0;
            match m1749 {
                Adt18::C0(v10) => {
                    let a1750 = v10;
                    let a1751 = 0u64;
                    if if if nat_eq(a1750, a1751) {
                        true
                    } else {
                        let a1752 = v10;
                        let a1753 = 8u64;
                        nat_eq(a1752, a1753)
                    } {
                        true
                    } else {
                        let a1754 = v10;
                        let a1755 = if v1 {
                            6u64
                        } else {
                            7u64
                        };
                        nat_eq(a1754, a1755)
                    } {
                        Ok(Adt19::C0(v7, v6))
                    } else {
                        Ok(Adt19::C1(f349(v2)?))
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

pub fn f263(v0: Adt18, v1: u64, v2: u64) -> R<Adt19> {
    let m1756 = v0;
    match m1756 {
        Adt18::C0(v5) => {
            let a1757 = v5;
            let a1758 = 1u64;
            if nat_eq(a1757, a1758) {
                Ok(Adt19::C0(Adt2::C6(Bytes::lit(&[])), v2))
            } else {
                let a1759 = v5;
                let a1760 = 8u64;
                if nat_eq(a1759, a1760) {
                    Ok(Adt19::C1(f129(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                } else {
                    let a1761 = v5;
                    let a1762 = 0u64;
                    if if nat_eq(a1761, a1762) {
                        true
                    } else {
                        let a1763 = v5;
                        let a1764 = 5u64;
                        nat_eq(a1763, a1764)
                    } {
                        Ok(Adt19::C0(Adt2::C0, v2))
                    } else {
                        Ok(Adt19::C1(f349(v1)?))
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

pub fn f264(v0: Bytes, v1: u64) -> R<u64> {
    let (h1765, _) = f179(Fn5::F350(v0.clone(), v1), {
        let a1767 = {
            let a1766 = v0;
            length_bytes(a1766)
        };
        let a1768 = 1u64;
        nat_add(a1767, a1768)?
    }, 0u64)?;
    Ok(h1765)
}

pub fn f265(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f46(v0, v1, {
        let a1769 = v2;
        let a1770 = v1;
        nat_sub(a1769, a1770)
    })
}

pub fn f266(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let a1771 = f185(v0.clone(), v1)?;
    let a1772 = 33u64;
    if nat_eq(a1771, a1772) {
        let m1773 = f259(v0.clone(), v1)?;
        match m1773 {
            Adt18::C0(v4) => {
                let a1774 = v4;
                let a1775 = 1u64;
                if if nat_eq(a1774, a1775) {
                    true
                } else {
                    let a1776 = v4;
                    let a1777 = 8u64;
                    nat_eq(a1776, a1777)
                } {
                    Ok(Ok::<u64, Bytes>(f251(v0.clone(), f261(v0, v1)?)?))
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

pub fn f267(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a1778 = f185(v0.clone(), v1)?;
        let a1779 = 33u64;
        if nat_eq(a1778, a1779) {
            f251(v0.clone(), f261(v0.clone(), v1)?)?
        } else {
            v1
        }
    };
    let v3: u64 = f185(v0.clone(), v2)?;
    let a1780 = v3;
    let a1781 = 34u64;
    if if nat_eq(a1780, a1781) {
        true
    } else {
        let a1782 = v3;
        let a1783 = 39u64;
        nat_eq(a1782, a1783)
    } {
        let m1784 = f342(v0.clone(), v2, 0u64, true)?;
        match m1784 {
            Adt23::C0(_, v6) => {
                let v7: u64 = f251(v0.clone(), v6)?;
                if f351(v0, v7)? {
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
        if f344(v0.clone(), v2, false)? {
            let v8: u64 = f251(v0.clone(), f345(v0.clone(), v2, false)?)?;
            if f351(v0, v8)? {
                Ok(Some(v8))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f268(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f185(v0.clone(), v1)?;
    let a1785 = v2;
    let a1786 = 34u64;
    if if nat_eq(a1785, a1786) {
        true
    } else {
        let a1787 = v2;
        let a1788 = 39u64;
        nat_eq(a1787, a1788)
    } {
        let m1789 = f342(v0, v1, 0u64, true)?;
        match m1789 {
            Adt23::C0(v4, _) => {
                Ok(v4)
            }
            Adt23::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f265(v0.clone(), v1, f345(v0, v1, false)?))
    }
}

pub fn f269(v0: Bytes, p1792: List<Bytes>) -> bool {
    let m1791 = p1792;
    {
        let mut m1791 = m1791;
        loop {
            let step = {
                let v1 = m1791;
                let m1790 = v1;
                match m1790.uncons() {
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
                Ok(next) => { m1791 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1793, _) = v2;
        h1793
    };
    let a1794 = v3;
    let a1795 = 0u64;
    if nat_eq(a1794, a1795) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1796 = v3;
            let a1797 = v1;
            let a1798 = 0u64;
            nat_quot(a1796, a1797, a1798)
        }, {
            let a1803 = f188(v0, {
                let a1799 = v3;
                let a1800 = v1;
                let a1801 = 0u64;
                nat_rem(a1799, a1800, a1801)
            });
            let a1804 = {
                let (_, h1802) = v2;
                h1802
            };
            append_bytes(a1803, a1804)
        }))
    }
}

pub fn f271(p1859: u64, v1: Bytes, p1860: u64, p1861: List<Adt20>) -> R<Result<List<Adt20>, Bytes>> {
    let m1858 = (p1859, (p1860, p1861));
    {
        let mut m1858 = m1858;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1858;
                let m1805 = v0;
                if m1805 == 0 {
                    Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Err::<List<Adt20>, Bytes>(f352(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1805 - 1;
                    let v5: u64 = f330(v1.clone(), v2)?;
                    let a1806 = v5;
                    let a1807 = 256u64;
                    if nat_eq(a1806, a1807) {
                        Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Ok::<List<Adt20>, Bytes>(f353(v3, List::<Adt20>::nil()))))
                    } else {
                        if f354(v5)? {
                            Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                let a1808 = v2;
                                let a1809 = 1u64;
                                nat_add(a1808, a1809)?
                            }, v3)))
                        } else {
                            if f182(v1.clone(), v2, Str::lit("//")) {
                                let m1810 = f331(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1810 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Ok::<List<Adt20>, Bytes>(f353(v3, List::<Adt20>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                            let a1811 = v6;
                                            let a1812 = 1u64;
                                            nat_add(a1811, a1812)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f182(v1.clone(), v2, Str::lit("/*")) {
                                    let m1816 = f331(v1.clone(), {
                                        let a1813 = Str::lit("*/");
                                        utf8_encode(a1813)
                                    }, {
                                        let a1814 = v2;
                                        let a1815 = 2u64;
                                        nat_add(a1814, a1815)?
                                    })?;
                                    match m1816 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Err::<List<Adt20>, Bytes>(f352(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                                let a1817 = v7;
                                                let a1818 = 2u64;
                                                nat_add(a1817, a1818)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1819 = v5;
                                    let a1820 = 34u64;
                                    if nat_eq(a1819, a1820) {
                                        let m1823 = f331(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1821 = v2;
                                            let a1822 = 1u64;
                                            nat_add(a1821, a1822)?
                                        })?;
                                        match m1823 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>(Ok(Err::<List<Adt20>, Bytes>(f352(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                                    let a1824 = v8;
                                                    let a1825 = 1u64;
                                                    nat_add(a1824, a1825)?
                                                }, List::cons(Adt20::C0(3u64, f46(v1.clone(), {
                                                    let a1826 = v2;
                                                    let a1827 = 1u64;
                                                    nat_add(a1826, a1827)?
                                                }, {
                                                    let a1830 = v8;
                                                    let a1831 = {
                                                        let a1828 = v2;
                                                        let a1829 = 1u64;
                                                        nat_add(a1828, a1829)?
                                                    };
                                                    nat_sub(a1830, a1831)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1832 = v5;
                                            let a1833 = 95u64;
                                            if if nat_eq(a1832, a1833) {
                                                true
                                            } else {
                                                let a1834 = v5;
                                                let a1835 = 45u64;
                                                nat_eq(a1834, a1835)
                                            } {
                                                let a1836 = v2;
                                                let a1837 = 1u64;
                                                nat_add(a1836, a1837)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f355(f330(v1.clone(), v9)?) {
                                            let v10: u64 = f356(v1.clone(), 3u64, {
                                                let a1838 = v9;
                                                let a1839 = 1u64;
                                                nat_add(a1838, a1839)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, (v10, List::cons(Adt20::C0(0u64, f46(v1.clone(), v2, {
                                                let a1840 = v10;
                                                let a1841 = v2;
                                                nat_sub(a1840, a1841)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1842 = v5;
                                                let a1843 = 45u64;
                                                if nat_eq(a1842, a1843) {
                                                    let a1844 = v2;
                                                    let a1845 = 1u64;
                                                    nat_add(a1844, a1845)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f357(v1.clone(), v11)?;
                                            let v13: u64 = f358(v1.clone(), v11)?;
                                            let a1846 = v12;
                                            let a1847 = v13;
                                            if nat_lt(a1846, a1847) {
                                                Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, (v13, List::cons(Adt20::C0(2u64, f46(v1.clone(), v2, {
                                                    let a1848 = v13;
                                                    let a1849 = v2;
                                                    nat_sub(a1848, a1849)
                                                })), v3))))
                                            } else {
                                                let a1850 = v11;
                                                let a1851 = v12;
                                                if nat_lt(a1850, a1851) {
                                                    Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, (v12, List::cons(Adt20::C0(1u64, f46(v1.clone(), v2, {
                                                        let a1852 = v12;
                                                        let a1853 = v2;
                                                        nat_sub(a1852, a1853)
                                                    })), v3))))
                                                } else {
                                                    if f182(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                                            let a1854 = v2;
                                                            let a1855 = 3u64;
                                                            nat_add(a1854, a1855)?
                                                        }, List::cons(Adt20::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt20>)), R<Result<List<Adt20>, Bytes>>>((v4, ({
                                                            let a1856 = v2;
                                                            let a1857 = 1u64;
                                                            nat_add(a1856, a1857)?
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
                Ok(next) => { m1858 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f272(p1864: List<Adt13>, p1865: List<Adt13>) -> List<Adt13> {
    let m1863 = (p1864, p1865);
    {
        let mut m1863 = m1863;
        loop {
            let step = {
                let (v0, v1) = m1863;
                let m1862 = v0;
                match m1862.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
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

pub fn f273(v0: List<Adt20>) -> R<Result<List<Adt20>, Bytes>> {
    if f359(v0.clone(), Str::lit("[")) {
        Ok(f360({
            let a1867 = {
                let a1866 = v0.clone();
                length_list(a1866)
            };
            let a1868 = 1u64;
            nat_add(a1867, a1868)?
        }, f361(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt20>, Bytes>(v0))
    }
}

pub fn f274(v0: u64, v1: List<Adt20>) -> R<Result<(Adt13, List<Adt20>), Bytes>> {
    if f359(v1.clone(), Str::lit("interface")) {
        if f359(f361(v1.clone()), Str::lit("mixin")) {
            f362(v0, 1u64, f361(f361(v1)), false)
        } else {
            f362(v0, 0u64, f361(v1), true)
        }
    } else {
        if f359(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt20> = f361(v1);
            if f359(v2.clone(), Str::lit("interface")) {
                if f359(f361(v2.clone()), Str::lit("mixin")) {
                    f362(v0, 6u64, f361(f361(v2)), false)
                } else {
                    f362(v0, 5u64, f361(v2), false)
                }
            } else {
                if f359(v2.clone(), Str::lit("dictionary")) {
                    f362(v0, 7u64, f361(v2), false)
                } else {
                    if f359(v2.clone(), Str::lit("namespace")) {
                        f363(f361(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt13, List<Adt20>), Bytes>(f364(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f359(v1.clone(), Str::lit("dictionary")) {
                f362(v0, 2u64, f361(v1), true)
            } else {
                if f359(v1.clone(), Str::lit("enum")) {
                    let m1869 = f365(f361(v1));
                    match m1869 {
                        Ok(v4) => {
                            let m1870 = f366(f367(v4.clone()), Str::lit("{"));
                            match m1870 {
                                Ok(v6) => {
                                    let m1871 = f368(v0, v6, List::<Bytes>::nil());
                                    match m1871 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(3u64, f369(v4), Bytes::lit(&[]), List::<Adt14>::nil(), {
                                                let (h1872, _) = v8;
                                                h1872
                                            }, f370(Bytes::lit(&[]))), {
                                                let (_, h1873) = v8;
                                                h1873
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
                    if f359(v1.clone(), Str::lit("typedef")) {
                        let m1874 = f371(v0, f361(v1), 0u64)?;
                        match m1874 {
                            Ok(v10) => {
                                let m1875 = f365(f372(v10.clone()));
                                match m1875 {
                                    Ok(v12) => {
                                        let m1876 = f366(f367(v12.clone()), Str::lit(";"));
                                        match m1876 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(4u64, f369(v12), Bytes::lit(&[]), List::<Adt14>::nil(), List::<Bytes>::nil(), f373(v10)), v14)))
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
                        if f359(v1.clone(), Str::lit("callback")) {
                            if f359(f361(v1.clone()), Str::lit("interface")) {
                                f363(f361(f361(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f363(f361(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f359(v1.clone(), Str::lit("namespace")) {
                                f363(f361(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1877 = f365(v1);
                                match m1877 {
                                    Ok(v16) => {
                                        let m1878 = f366(f367(v16.clone()), Str::lit("includes"));
                                        match m1878 {
                                            Ok(v18) => {
                                                let m1879 = f365(v18);
                                                match m1879 {
                                                    Ok(v20) => {
                                                        let m1880 = f366(f367(v20.clone()), Str::lit(";"));
                                                        match m1880 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(8u64, f369(v16), f369(v20), List::<Adt14>::nil(), List::<Bytes>::nil(), f370(Bytes::lit(&[]))), v22)))
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

pub fn f275(p1884: List<Bytes>, v1: List<Adt16>) -> Bytes {
    let m1883 = p1884;
    {
        let mut m1883 = m1883;
        loop {
            let step = {
                let v0 = m1883;
                let m1881 = v0;
                match m1881.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1882 = f203(v1.clone(), v2.clone());
                        match m1882 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f32(f204(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f204(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1883 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f117(v0)?;
    if f114(v1.clone(), f374()) {
        f375(v1)
    } else {
        Ok(v1)
    }
}

pub fn f277(p1890: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m1889 = p1890;
    {
        let mut m1889 = m1889;
        loop {
            let step = {
                let v0 = m1889;
                let m1885 = v0;
                match m1885.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1886 = f196(v3.clone());
                        let a1887 = 8u64;
                        if if nat_eq(a1886, a1887) {
                            f32(f198(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1888 = f209(v1.clone(), 1u64, f211(v3.clone()));
                            match m1888 {
                                None => {
                                    Err::<List<Adt13>, Option<Bytes>>(Some(f211(v3)))
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
                Ok(next) => { m1889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<Adt14>, Bytes>> {
    let m1891 = v0;
    if m1891 == 0 {
        Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f198(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1891 - 1;
        let v4: List<Adt14> = {
            let a1892 = f196(v2.clone());
            let a1893 = 2u64;
            if nat_eq(a1892, a1893) {
                f376(f377(f200(v2.clone()), f378(v1.clone(), 7u64, f198(v2.clone()))))?
            } else {
                f377(f200(v2.clone()), f377(f378(v1.clone(), 5u64, f198(v2.clone())), f379(v1.clone(), v1.clone(), f198(v2.clone()))))
            }
        };
        if f32(f211(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt14>, Bytes>(v4))
        } else {
            let m1894 = f209(v1.clone(), f196(v2.clone()), f211(v2.clone()));
            match m1894 {
                None => {
                    Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f211(v2.clone()), List::cons({
                        let a1895 = f196(v2);
                        let a1896 = 2u64;
                        if nat_eq(a1895, a1896) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1897 = f278(v3, v1, v5)?;
                    match m1897 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f377(v7, v4)))
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

pub fn f279(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f280(v0: bool, v1: List<Adt14>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1898 = v1;
    match m1898.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt15 = f294(v2.clone());
            if !f32(f289(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f380(v2), List::cons(f8(Str::lit(": ")), List::cons(f289(v4), List::<Bytes>::nil())))))))
            } else {
                let m1899 = f381(v0, f380(v2.clone()))?;
                match m1899 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f380(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f114(v5.clone(), f382(v0, f282(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m1900 = f280(v0, v3)?;
                            match m1900 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a1901 = f383(v2);
                                        let a1902 = 1u64;
                                        if nat_eq(a1901, a1902) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f291(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f291(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h1903, _) = v7;
                                        h1903
                                    }), f205(f292(v4), {
                                        let (_, h1904) = v7;
                                        h1904
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

pub fn f281(p1908: List<Bytes>) -> R<bool> {
    let m1907 = p1908;
    {
        let mut m1907 = m1907;
        loop {
            let step = {
                let v0 = m1907;
                let m1905 = v0;
                match m1905.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1906 = f384(v1)?;
                        match m1906 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f114(v3, f382(false, v2.clone())?) {
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
                Ok(next) => { m1907 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(v0: List<Adt14>) -> List<Bytes> {
    let m1909 = v0;
    match m1909.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f380(v1), f282(v2))
        }
    }
}

pub fn f283(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f151(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f284(p1912: List<Bytes>) -> Option<Bytes> {
    let m1911 = p1912;
    {
        let mut m1911 = m1911;
        loop {
            let step = {
                let v0 = m1911;
                let m1910 = v0;
                match m1910.uncons() {
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
                Ok(next) => { m1911 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: Adt13) -> List<Bytes> {
    let m1913 = v0;
    match m1913 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f286(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1914 = v1;
    match m1914.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1915 = f385(v0, v3.clone())?;
            match m1915 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f386(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f114(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1916 = f286(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1916 {
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

pub fn f287(v0: List<Bytes>) -> List<Adt2> {
    let m1917 = v0;
    match m1917.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C6(v1), f287(v2))
        }
    }
}

pub fn f288(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f151(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f289(v0: Adt15) -> Bytes {
    let m1918 = v0;
    match m1918 {
        Adt15::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f290(v0: Adt15) -> u64 {
    let m1919 = v0;
    match m1919 {
        Adt15::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f291(v0: Adt15) -> Bytes {
    let m1920 = v0;
    match m1920 {
        Adt15::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f292(v0: Adt15) -> List<Bytes> {
    let m1921 = v0;
    match m1921 {
        Adt15::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f293(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f294(v0: Adt14) -> Adt15 {
    let m1922 = v0;
    match m1922 {
        Adt14::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f295(p1925: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1924 = p1925;
    {
        let mut m1924 = m1924;
        loop {
            let step = {
                let v0 = m1924;
                let m1923 = v0;
                match m1923.uncons() {
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
                Ok(next) => { m1924 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: bool, v1: u64) -> R<bool> {
    let a1932 = {
        let a1929 = {
            let a1926 = v1;
            let a1927 = f308(v0)?;
            let a1928 = 0u64;
            nat_quot(a1926, a1927, a1928)
        };
        let a1930 = f310(f311(v0))?;
        let a1931 = 0u64;
        nat_rem(a1929, a1930, a1931)
    };
    let a1933 = f312(v0)?;
    Ok(!nat_eq(a1932, a1933))
}

pub fn f297(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1936 = f46(v0.clone(), {
        let a1934 = v1;
        let a1935 = 2u64;
        nat_add(a1934, a1935)?
    }, 2u64);
    let a1937 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1936, a1937) {
        let m1940 = f387(f46(v0, {
            let a1938 = v1;
            let a1939 = 4u64;
            nat_add(a1938, a1939)?
        }, 2u64))?;
        match m1940 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f103(v2.clone(), 0u64)?;
                let a1941 = v3;
                let a1942 = 32u64;
                if nat_lt(a1941, a1942) {
                    if f388(v3) {
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

pub fn f298(v0: u64) -> Bytes {
    let a1943 = v0;
    let a1944 = 34u64;
    if nat_eq(a1943, a1944) {
        Bytes::lit(&[0x22])
    } else {
        let a1945 = v0;
        let a1946 = 92u64;
        if nat_eq(a1945, a1946) {
            Bytes::lit(&[0x5c])
        } else {
            let a1947 = v0;
            let a1948 = 98u64;
            if nat_eq(a1947, a1948) {
                Bytes::lit(&[0x08])
            } else {
                let a1949 = v0;
                let a1950 = 102u64;
                if nat_eq(a1949, a1950) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1951 = v0;
                    let a1952 = 110u64;
                    if nat_eq(a1951, a1952) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1953 = v0;
                        let a1954 = 114u64;
                        if nat_eq(a1953, a1954) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1955 = v0;
                            let a1956 = 116u64;
                            if nat_eq(a1955, a1956) {
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

pub fn f299(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f103(v0, v1)?;
    let a1957 = 47u64;
    let a1958 = v2;
    if if nat_lt(a1957, a1958) {
        let a1959 = v2;
        let a1960 = 58u64;
        nat_lt(a1959, a1960)
    } else {
        false
    } {
        Ok(Some({
            let a1961 = v1;
            let a1962 = 1u64;
            nat_add(a1961, a1962)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f300(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1963 = f103(v0, v1)?;
    let a1964 = 48u64;
    if nat_eq(a1963, a1964) {
        Ok(Some({
            let a1965 = v1;
            let a1966 = 1u64;
            nat_add(a1965, a1966)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f301(v0: Bytes) -> R<u64> {
    let (h1967, _) = f179(Fn5::F389(v0.clone()), {
        let a1968 = v0;
        length_bytes(a1968)
    }, 0u64)?;
    Ok(h1967)
}

pub fn f302(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1972 = {
        let a1970 = v0;
        let a1971 = {
            let (h1969, _) = v1;
            h1969
        };
        index_bytes(a1970, a1971)
    };
    match m1972 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1974 = {
                    let (h1973, _) = v1;
                    h1973
                };
                let a1975 = 1u64;
                nat_add(a1974, a1975)?
            }, {
                let a1979 = {
                    let a1977 = {
                        let (_, h1976) = v1;
                        h1976
                    };
                    let a1978 = 10u64;
                    nat_mul(a1977, a1978)?
                };
                let a1980 = f390(v2)?;
                nat_add(a1979, a1980)?
            })))
        }
    }
}

pub fn f303(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f391(v0, v1)
}

pub fn f304(v0: Fn7, p1984: u64, p1985: Adt22) -> R<Adt22> {
    let m1983 = (p1984, p1985);
    {
        let mut m1983 = m1983;
        loop {
            let step = {
                let (v1, v2) = m1983;
                let m1981 = v1;
                if m1981 == 0 {
                    Err::<(u64, Adt22), R<Adt22>>(Ok(v2))
                } else {
                    let v3 = m1981 - 1;
                    Ok::<(u64, Adt22), R<Adt22>>((v3, {
                        let c1982: Fn7 = v0.clone();
                        c1982.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f305(v0: Adt22) -> R<Adt22> {
    f392(v0)
}

pub fn f306(v0: u32, v1: u32) -> u32 {
    let m1988 = {
        let a1986 = f393(v0);
        let a1987 = f393(v1);
        fixed_u64::checked_add(a1986, a1987)
    };
    match m1988 {
        None => {
            0u32
        }
        Some(v2) => {
            f394(v2)
        }
    }
}

pub fn f307(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1991 = {
        let a1989 = v3;
        let a1990 = 8u64;
        nat_add(a1989, a1990)?
    };
    let a1992 = v2;
    if nat_eq(a1991, a1992) {
        Ok(f395({
            let a1995 = {
                let a1993 = v1;
                let a1994 = 8u64;
                nat_mul(a1993, a1994)?
            };
            let a1996 = 4294967296u64;
            let a1997 = 0u64;
            nat_quot(a1995, a1996, a1997)
        }))
    } else {
        let a2000 = {
            let a1998 = v3;
            let a1999 = 4u64;
            nat_add(a1998, a1999)?
        };
        let a2001 = v2;
        if nat_eq(a2000, a2001) {
            Ok(f395({
                let a2004 = {
                    let a2002 = v1;
                    let a2003 = 8u64;
                    nat_mul(a2002, a2003)?
                };
                let a2005 = 4294967296u64;
                let a2006 = 0u64;
                nat_rem(a2004, a2005, a2006)
            }))
        } else {
            Ok(f396(f397(v0.clone(), v1, v3), f397(v0.clone(), v1, {
                let a2007 = v3;
                let a2008 = 1u64;
                nat_add(a2007, a2008)?
            }), f397(v0.clone(), v1, {
                let a2009 = v3;
                let a2010 = 2u64;
                nat_add(a2009, a2010)?
            }), f397(v0, v1, {
                let a2011 = v3;
                let a2012 = 3u64;
                nat_add(a2011, a2012)?
            })))
        }
    }
}

pub fn f308(v0: bool) -> R<u64> {
    f310({
        let a2013 = f315(v0);
        let a2014 = 1u64;
        nat_sub(a2013, a2014)
    })
}

pub fn f309(v0: bool) -> R<u64> {
    f310(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f310(v0: u64) -> R<u64> {
    let m2015 = v0;
    if m2015 == 0 {
        Ok(1u64)
    } else {
        let v1 = m2015 - 1;
        let a2016 = 2u64;
        let a2017 = f310(v1)?;
        nat_mul(a2016, a2017)
    }
}

pub fn f311(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f312(v0: bool) -> R<u64> {
    let a2018 = f310(f311(v0))?;
    let a2019 = 1u64;
    Ok(nat_sub(a2018, a2019))
}

pub fn f313(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a2020 = Bytes::lit(&[0x2d]);
        let a2021 = v1;
        append_bytes(a2020, a2021)
    } else {
        v1
    }
}

pub fn f314(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f315(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f316(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f398(f399(v2, f400(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f398(f399(v2, f400(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a2022 = v3.clone();
            length_bytes(a2022)
        };
        let v5: Bytes = {
            let a2023 = v2;
            let a2024 = v4;
            if nat_lt(a2023, a2024) {
                f46(v3.clone(), 0u64, {
                    let a2025 = v4;
                    let a2026 = v2;
                    nat_sub(a2025, a2026)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f401({
            let a2027 = v2;
            let a2028 = v4;
            if nat_lt(a2027, a2028) {
                f46(v3, {
                    let a2029 = v4;
                    let a2030 = v2;
                    nat_sub(a2029, a2030)
                }, v2)
            } else {
                let a2033 = f402({
                    let a2031 = v2;
                    let a2032 = v4;
                    nat_sub(a2031, a2032)
                });
                let a2034 = v3;
                append_bytes(a2033, a2034)
            }
        }, {
            let a2035 = v2;
            let a2036 = 1u64;
            nat_add(a2035, a2036)?
        })?;
        let a2038 = {
            let a2037 = v6.clone();
            length_bytes(a2037)
        };
        let a2039 = 0u64;
        if nat_eq(a2038, a2039) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f317(v0: Bytes, p2058: u64, p2059: u64, p2060: Bytes) -> R<Bytes> {
    let m2057 = (p2058, (p2059, p2060));
    {
        let mut m2057 = m2057;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2057;
                let m2040 = v2;
                if m2040 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2040 - 1;
                    let v5: u64 = f155(v0.clone(), v1, {
                        let a2044 = {
                            let a2042 = {
                                let a2041 = v0.clone();
                                length_bytes(a2041)
                            };
                            let a2043 = v1;
                            nat_sub(a2042, a2043)
                        };
                        let a2045 = 1u64;
                        nat_add(a2044, a2045)?
                    })?;
                    let v6: Bytes = {
                        let a2048 = v3;
                        let a2049 = f46(v0.clone(), v1, {
                            let a2046 = v5;
                            let a2047 = v1;
                            nat_sub(a2046, a2047)
                        });
                        append_bytes(a2048, a2049)
                    };
                    let m2052 = {
                        let a2050 = v0.clone();
                        let a2051 = v5;
                        index_bytes(a2050, a2051)
                    };
                    match m2052 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2053 = v5;
                                let a2054 = 1u64;
                                nat_add(a2053, a2054)?
                            }, (v4, {
                                let a2055 = v6;
                                let a2056 = f403(v0.clone(), v5, v7)?;
                                append_bytes(a2055, a2056)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2057 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(v0: Fn8, p2064: List<(Bytes, Bytes)>, p2065: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2063 = (p2064, p2065);
    {
        let mut m2063 = m2063;
        loop {
            let step = {
                let (v1, v2) = m2063;
                let m2061 = v2;
                match m2061.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2062: Fn8 = v0.clone();
                            c2062.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f404(v0, v1)
}

pub fn f320(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f405(v0)
}

pub fn f321(v0: Fn9, p2069: List<Bytes>, p2070: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2068 = (p2069, p2070);
    {
        let mut m2068 = m2068;
        loop {
            let step = {
                let (v1, v2) = m2068;
                let m2066 = v2;
                match m2066.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2067: Fn9 = v0.clone();
                            c2067.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f406(v1)?, v0))
}

pub fn f323(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f407(v0.clone(), 0u64, {
        let a2071 = v0;
        length_bytes(a2071)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f324(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2072, _) = v1.clone();
        h2072
    };
    let v3: u64 = {
        let (h2073, _) = {
            let (_, h2074) = v1.clone();
            h2074
        };
        h2073
    };
    let v4: List<Bytes> = {
        let (_, h2075) = {
            let (_, h2076) = v1;
            h2076
        };
        h2075
    };
    let m2077 = f163(v0.clone(), v2);
    match m2077 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2078 = v5;
            let a2079 = 10u8;
            if equal(a2078, a2079) {
                Ok(Some(({
                    let a2080 = v2;
                    let a2081 = 1u64;
                    nat_add(a2080, a2081)?
                }, ({
                    let a2082 = v2;
                    let a2083 = 1u64;
                    nat_add(a2082, a2083)?
                }, List::cons(f408(f46(v0, v3, {
                    let a2084 = v2;
                    let a2085 = v3;
                    nat_sub(a2084, a2085)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2086 = v2;
                    let a2087 = 1u64;
                    nat_add(a2086, a2087)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f325(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f326(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m2088 = v0.clone();
    match m2088.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a2089 = v2;
            let a2090 = v1.clone();
            if equal(a2089, a2090) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f327(v0: List<(Bytes, ())>) -> List<Bytes> {
    f241(f409(Fn10::F410, List::<Bytes>::nil(), v0))
}

pub fn f328(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f411(f412(Fn11::F413, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m2095 = {
        let (h2091, _) = f414(Fn12::F415, {
            let a2093 = {
                let a2092 = v0;
                length_list(a2092)
            };
            let a2094 = 1u64;
            nat_add(a2093, a2094)?
        }, v1)?;
        h2091
    };
    match m2095.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f329(v0: List<Bytes>) -> List<(Bytes, ())> {
    f416(f417(Fn13::F418, List::<(Bytes, ())>::nil(), v0))
}

pub fn f330(v0: Bytes, v1: u64) -> R<u64> {
    f103(v0, v1)
}

pub fn f331(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h2096, _) = f179(Fn5::F419(v1.clone(), v0.clone()), {
            let a2098 = {
                let a2097 = v0.clone();
                length_bytes(a2097)
            };
            let a2099 = 1u64;
            nat_add(a2098, a2099)?
        }, v2)?;
        h2096
    };
    if f125(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f332(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2102 = {
        let a2100 = v0.clone();
        let a2101 = v1;
        index_bytes(a2100, a2101)
    };
    match m2102 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f60(v2)?;
            let a2103 = v3;
            let a2104 = 128u64;
            if nat_lt(a2103, a2104) {
                Ok(Some((v3, 1u64)))
            } else {
                if f105({
                    let a2105 = 194u64;
                    let a2106 = v3;
                    nat_le(a2105, a2106)
                }, {
                    let a2107 = v3;
                    let a2108 = 223u64;
                    nat_le(a2107, a2108)
                }) {
                    let m2111 = f420(v0, {
                        let a2109 = v1;
                        let a2110 = 1u64;
                        nat_add(a2109, a2110)?
                    })?;
                    match m2111 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f421({
                                let a2116 = {
                                    let a2114 = {
                                        let a2112 = v3;
                                        let a2113 = 192u64;
                                        nat_sub(a2112, a2113)
                                    };
                                    let a2115 = 64u64;
                                    nat_mul(a2114, a2115)?
                                };
                                let a2117 = v4;
                                nat_add(a2116, a2117)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f105({
                        let a2118 = 224u64;
                        let a2119 = v3;
                        nat_le(a2118, a2119)
                    }, {
                        let a2120 = v3;
                        let a2121 = 239u64;
                        nat_le(a2120, a2121)
                    }) {
                        let m2124 = f420(v0.clone(), {
                            let a2122 = v1;
                            let a2123 = 1u64;
                            nat_add(a2122, a2123)?
                        })?;
                        match m2124 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2127 = f420(v0, {
                                    let a2125 = v1;
                                    let a2126 = 2u64;
                                    nat_add(a2125, a2126)?
                                })?;
                                match m2127 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f421({
                                            let a2136 = {
                                                let a2130 = {
                                                    let a2128 = v3;
                                                    let a2129 = 224u64;
                                                    nat_sub(a2128, a2129)
                                                };
                                                let a2131 = 4096u64;
                                                nat_mul(a2130, a2131)?
                                            };
                                            let a2137 = {
                                                let a2134 = {
                                                    let a2132 = v5;
                                                    let a2133 = 64u64;
                                                    nat_mul(a2132, a2133)?
                                                };
                                                let a2135 = v6;
                                                nat_add(a2134, a2135)?
                                            };
                                            nat_add(a2136, a2137)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f105({
                            let a2138 = 240u64;
                            let a2139 = v3;
                            nat_le(a2138, a2139)
                        }, {
                            let a2140 = v3;
                            let a2141 = 244u64;
                            nat_le(a2140, a2141)
                        }) {
                            let m2144 = f420(v0.clone(), {
                                let a2142 = v1;
                                let a2143 = 1u64;
                                nat_add(a2142, a2143)?
                            })?;
                            match m2144 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2147 = f420(v0.clone(), {
                                        let a2145 = v1;
                                        let a2146 = 2u64;
                                        nat_add(a2145, a2146)?
                                    })?;
                                    match m2147 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2150 = f420(v0, {
                                                let a2148 = v1;
                                                let a2149 = 3u64;
                                                nat_add(a2148, a2149)?
                                            })?;
                                            match m2150 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f421({
                                                        let a2163 = {
                                                            let a2153 = {
                                                                let a2151 = v3;
                                                                let a2152 = 240u64;
                                                                nat_sub(a2151, a2152)
                                                            };
                                                            let a2154 = 262144u64;
                                                            nat_mul(a2153, a2154)?
                                                        };
                                                        let a2164 = {
                                                            let a2161 = {
                                                                let a2155 = v7;
                                                                let a2156 = 4096u64;
                                                                nat_mul(a2155, a2156)?
                                                            };
                                                            let a2162 = {
                                                                let a2159 = {
                                                                    let a2157 = v8;
                                                                    let a2158 = 64u64;
                                                                    nat_mul(a2157, a2158)?
                                                                };
                                                                let a2160 = v9;
                                                                nat_add(a2159, a2160)?
                                                            };
                                                            nat_add(a2161, a2162)?
                                                        };
                                                        nat_add(a2163, a2164)?
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

pub fn f333(v0: u64) -> bool {
    let a2165 = v0;
    let a2166 = 9u64;
    if if if nat_eq(a2165, a2166) {
        true
    } else {
        let a2167 = v0;
        let a2168 = 10u64;
        nat_eq(a2167, a2168)
    } {
        true
    } else {
        let a2169 = v0;
        let a2170 = 13u64;
        if nat_eq(a2169, a2170) {
            true
        } else {
            let a2171 = 32u64;
            let a2172 = v0;
            if nat_le(a2171, a2172) {
                let a2173 = v0;
                let a2174 = 126u64;
                nat_le(a2173, a2174)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2175 = v0;
        let a2176 = 133u64;
        if if nat_eq(a2175, a2176) {
            true
        } else {
            let a2177 = 160u64;
            let a2178 = v0;
            if nat_le(a2177, a2178) {
                let a2179 = v0;
                let a2180 = 55295u64;
                nat_le(a2179, a2180)
            } else {
                false
            }
        } {
            true
        } else {
            let a2181 = 57344u64;
            let a2182 = v0;
            if if nat_le(a2181, a2182) {
                let a2183 = v0;
                let a2184 = 65533u64;
                nat_le(a2183, a2184)
            } else {
                false
            } {
                true
            } else {
                let a2185 = 65536u64;
                let a2186 = v0;
                if nat_le(a2185, a2186) {
                    let a2187 = v0;
                    let a2188 = 1114111u64;
                    nat_le(a2187, a2188)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f334(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f251(v0.clone(), v1)?;
    if f249(f185(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2189 = f185(v0, v2)?;
        let a2190 = 35u64;
        Ok(nat_eq(a2189, a2190))
    }
}

pub fn f335(v0: Bytes, v1: u64) -> R<u64> {
    let (h2191, _) = f179(Fn5::F422(v0), {
        let a2192 = v1;
        let a2193 = 1u64;
        nat_add(a2192, a2193)?
    }, v1)?;
    Ok(h2191)
}

pub fn f336(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f423(v0, v1)
}

pub fn f337(v0: Bytes, v1: u64) -> R<u64> {
    let (h2194, _) = f179(Fn5::F424(v0.clone()), {
        let a2196 = {
            let a2195 = v0;
            length_bytes(a2195)
        };
        let a2197 = 1u64;
        nat_add(a2196, a2197)?
    }, v1)?;
    Ok(h2194)
}

pub fn f338(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f425(v0, v1)
}

pub fn f339(v0: u64, v1: bool) -> bool {
    f187(if if f248(v0) {
        true
    } else {
        f249(v0)
    } {
        true
    } else {
        if v1 {
            f426(v0)
        } else {
            false
        }
    })
}

pub fn f340(v0: Bytes, v1: u64, v2: u64) -> R<Adt19> {
    let v3: bool = {
        let a2198 = f185(v0.clone(), v1)?;
        let a2199 = 62u64;
        nat_eq(a2198, a2199)
    };
    let v4: u64 = f185(v0.clone(), {
        let a2200 = v1;
        let a2201 = 1u64;
        nat_add(a2200, a2201)?
    })?;
    let v5: u64 = f185(v0.clone(), {
        let a2202 = v1;
        let a2203 = 2u64;
        nat_add(a2202, a2203)?
    })?;
    let v6: u64 = f427(v4);
    let v7: u64 = f428(v4);
    let v8: bool = {
        let a2204 = 0u64;
        let a2205 = v6;
        if nat_lt(a2204, a2205) {
            true
        } else {
            let a2206 = 0u64;
            let a2207 = v7;
            nat_lt(a2206, a2207)
        }
    };
    let v9: u64 = {
        let a2208 = 0u64;
        let a2209 = v7;
        if nat_lt(a2208, a2209) {
            f427(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2210 = 0u64;
        let a2211 = v6;
        if nat_lt(a2210, a2211) {
            f428(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2212 = v6;
        let a2213 = v9;
        nat_add(a2212, a2213)?
    };
    let v12: u64 = {
        let a2214 = v7;
        let a2215 = v10;
        nat_add(a2214, a2215)?
    };
    let v13: u64 = {
        let a2224 = {
            let a2216 = v1;
            let a2217 = 1u64;
            nat_add(a2216, a2217)?
        };
        let a2225 = {
            let a2222 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2223 = {
                let a2218 = 0u64;
                let a2219 = v9;
                if if nat_lt(a2218, a2219) {
                    true
                } else {
                    let a2220 = 0u64;
                    let a2221 = v10;
                    nat_lt(a2220, a2221)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2222, a2223)?
        };
        nat_add(a2224, a2225)?
    };
    let m2226 = f184(v0.clone(), v13)?;
    match m2226 {
        Ok(v15) => {
            let a2227 = 0u64;
            let a2228 = v12;
            if nat_lt(a2227, a2228) {
                f429(v0, v15, {
                    let a2231 = {
                        let a2229 = v2;
                        let a2230 = v12;
                        nat_add(a2229, a2230)?
                    };
                    let a2232 = 1u64;
                    nat_sub(a2231, a2232)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2233, _) = f218(Fn6::F430(v0.clone()), {
                        let a2235 = {
                            let a2234 = v0.clone();
                            length_bytes(a2234)
                        };
                        let a2236 = 1u64;
                        nat_add(a2235, a2236)?
                    }, (v15, 0u64))?;
                    h2233
                };
                let v17: u64 = f253(v0.clone(), {
                    let (h2237, _) = v16;
                    h2237
                })?;
                let a2238 = f185(v0.clone(), v17)?;
                let a2239 = 256u64;
                if nat_eq(a2238, a2239) {
                    Ok(Adt19::C0(Adt2::C6({
                        let a2240 = v11;
                        let a2241 = 2u64;
                        if nat_eq(a2240, a2241) {
                            f431(f432(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2242 = v0;
                        length_bytes(a2242)
                    }))
                } else {
                    let v18: u64 = f433({
                        let a2244 = v17;
                        let a2245 = {
                            let (h2243, _) = v16;
                            h2243
                        };
                        nat_sub(a2244, a2245)
                    }, v2);
                    let a2247 = v18;
                    let a2248 = {
                        let (_, h2246) = v16;
                        h2246
                    };
                    if nat_lt(a2247, a2248) {
                        Ok(Adt19::C1(f129(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f429(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt19::C1(v14))
        }
    }
}

pub fn f341(v0: Adt18, v1: Bytes, v2: bool, v3: u64, v4: u64) -> R<Adt19> {
    let m2249 = v0;
    match m2249 {
        Adt18::C0(v7) => {
            let a2250 = v7;
            let a2251 = 1u64;
            if if if nat_eq(a2250, a2251) {
                true
            } else {
                let a2252 = v7;
                let a2253 = 8u64;
                nat_eq(a2252, a2253)
            } {
                true
            } else {
                let a2254 = v7;
                let a2255 = 0u64;
                if nat_eq(a2254, a2255) {
                    v2
                } else {
                    false
                }
            } {
                Ok(Adt19::C0(Adt2::C6(v1), v4))
            } else {
                let a2256 = v7;
                let a2257 = 0u64;
                if nat_eq(a2256, a2257) {
                    f434(v1, v3, v4)
                } else {
                    let a2258 = v7;
                    let a2259 = 5u64;
                    if nat_eq(a2258, a2259) {
                        if f435(v1) {
                            Ok(Adt19::C0(Adt2::C0, v4))
                        } else {
                            Ok(Adt19::C1(f349(v3)?))
                        }
                    } else {
                        let a2260 = v7;
                        let a2261 = 2u64;
                        if if nat_eq(a2260, a2261) {
                            f436(v1.clone())?
                        } else {
                            let a2262 = v7;
                            let a2263 = 3u64;
                            if nat_eq(a2262, a2263) {
                                let m2264 = f437(v1.clone())?;
                                match m2264 {
                                    None => {
                                        f438(v1.clone())
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            } else {
                                let a2265 = v7;
                                let a2266 = 4u64;
                                if nat_eq(a2265, a2266) {
                                    if f439(v1.clone()) {
                                        true
                                    } else {
                                        f440(v1.clone())
                                    }
                                } else {
                                    false
                                }
                            }
                        } {
                            f434(v1, v3, v4)
                        } else {
                            Ok(Adt19::C1(f349(v3)?))
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

pub fn f342(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt23> {
    let a2267 = f185(v0.clone(), v1)?;
    let a2268 = 34u64;
    if nat_eq(a2267, a2268) {
        f441(v0.clone(), {
            let a2269 = v1;
            let a2270 = 1u64;
            nat_add(a2269, a2270)?
        }, v2, v3, {
            let a2272 = {
                let a2271 = v0;
                length_bytes(a2271)
            };
            let a2273 = 2u64;
            nat_add(a2272, a2273)?
        }, Bytes::lit(&[]))
    } else {
        f442(v0.clone(), {
            let a2274 = v1;
            let a2275 = 1u64;
            nat_add(a2274, a2275)?
        }, v2, v3, {
            let a2277 = {
                let a2276 = v0;
                length_bytes(a2276)
            };
            let a2278 = 2u64;
            nat_add(a2277, a2278)?
        }, Bytes::lit(&[]))
    }
}

pub fn f343(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>, v6: Adt18) -> R<Adt19> {
    let m2279 = v3;
    if m2279 == 0 {
        Ok(Adt19::C1(f129(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v7 = m2279 - 1;
        let a2280 = v4;
        let a2281 = 1u64;
        if nat_eq(a2280, a2281) {
            let m2282 = f343(v0.clone(), v1, v2, v7, 0u64, List::<Bytes>::nil(), Adt18::C0(0u64))?;
            match m2282 {
                Adt19::C0(v9, v10) => {
                    let m2283 = f443(v0.clone(), v10, v2)?;
                    match m2283 {
                        Ok(v12) => {
                            let v13: u64 = f185(v0.clone(), v12)?;
                            let a2284 = v13;
                            let a2285 = 58u64;
                            if nat_eq(a2284, a2285) {
                                Ok(Adt19::C1(f129(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v12)?))
                            } else {
                                let a2286 = v13;
                                let a2287 = 93u64;
                                if nat_eq(a2286, a2287) {
                                    Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C9)), {
                                        let a2288 = v12;
                                        let a2289 = 1u64;
                                        nat_add(a2288, a2289)?
                                    }))
                                } else {
                                    let a2290 = v13;
                                    let a2291 = 44u64;
                                    if nat_eq(a2290, a2291) {
                                        let m2294 = f443(v0.clone(), {
                                            let a2292 = v12;
                                            let a2293 = 1u64;
                                            nat_add(a2292, a2293)?
                                        }, v2)?;
                                        match m2294 {
                                            Ok(v15) => {
                                                let a2295 = f185(v0.clone(), v15)?;
                                                let a2296 = 93u64;
                                                if nat_eq(a2295, a2296) {
                                                    Ok(Adt19::C0(Adt2::C10(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C9)), {
                                                        let a2297 = v15;
                                                        let a2298 = 1u64;
                                                        nat_add(a2297, a2298)?
                                                    }))
                                                } else {
                                                    let m2299 = f343(v0, v15, v2, v7, 1u64, List::<Bytes>::nil(), Adt18::C0(0u64))?;
                                                    match m2299 {
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
            let a2300 = v4;
            let a2301 = 2u64;
            if nat_eq(a2300, a2301) {
                let m2302 = f444(v0.clone(), v1, v2)?;
                match m2302 {
                    Adt23::C0(v20, v21) => {
                        let m2303 = f443(v0.clone(), v21, v2)?;
                        match m2303 {
                            Ok(v23) => {
                                let m2317 = {
                                    let a2304 = f185(v0.clone(), v23)?;
                                    let a2305 = 44u64;
                                    if if nat_eq(a2304, a2305) {
                                        true
                                    } else {
                                        let a2306 = f185(v0.clone(), v23)?;
                                        let a2307 = 125u64;
                                        nat_eq(a2306, a2307)
                                    } {
                                        Adt19::C0(Adt2::C0, v23)
                                    } else {
                                        if f187({
                                            let a2308 = f185(v0.clone(), v23)?;
                                            let a2309 = 58u64;
                                            nat_eq(a2308, a2309)
                                        }) {
                                            Adt19::C1(f129(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v23)?)
                                        } else {
                                            let m2312 = f443(v0.clone(), {
                                                let a2310 = v23;
                                                let a2311 = 1u64;
                                                nat_add(a2310, a2311)?
                                            }, v2)?;
                                            match m2312 {
                                                Ok(v25) => {
                                                    let a2313 = f185(v0.clone(), v25)?;
                                                    let a2314 = 44u64;
                                                    if if nat_eq(a2313, a2314) {
                                                        true
                                                    } else {
                                                        let a2315 = f185(v0.clone(), v25)?;
                                                        let a2316 = 125u64;
                                                        nat_eq(a2315, a2316)
                                                    } {
                                                        Adt19::C0(Adt2::C0, v25)
                                                    } else {
                                                        f343(v0.clone(), v25, v2, v7, 0u64, List::<Bytes>::nil(), Adt18::C0(0u64))?
                                                    }
                                                }
                                                Err(v24) => {
                                                    Adt19::C1(v24)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m2317 {
                                    Adt19::C0(v27, v28) => {
                                        if f269(v20.clone(), v5.clone()) {
                                            Ok(Adt19::C1(f129(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m2318 = f443(v0.clone(), v28, v2)?;
                                            match m2318 {
                                                Ok(v30) => {
                                                    let v31: u64 = f185(v0.clone(), v30)?;
                                                    let a2319 = v31;
                                                    let a2320 = 125u64;
                                                    if nat_eq(a2319, a2320) {
                                                        Ok(Adt19::C0(Adt2::C11(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C9)), {
                                                            let a2321 = v30;
                                                            let a2322 = 1u64;
                                                            nat_add(a2321, a2322)?
                                                        }))
                                                    } else {
                                                        let a2323 = v31;
                                                        let a2324 = 44u64;
                                                        if nat_eq(a2323, a2324) {
                                                            let m2327 = f443(v0.clone(), {
                                                                let a2325 = v30;
                                                                let a2326 = 1u64;
                                                                nat_add(a2325, a2326)?
                                                            }, v2)?;
                                                            match m2327 {
                                                                Ok(v33) => {
                                                                    let a2328 = f185(v0.clone(), v33)?;
                                                                    let a2329 = 125u64;
                                                                    if nat_eq(a2328, a2329) {
                                                                        Ok(Adt19::C0(Adt2::C11(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C9)), {
                                                                            let a2330 = v33;
                                                                            let a2331 = 1u64;
                                                                            nat_add(a2330, a2331)?
                                                                        }))
                                                                    } else {
                                                                        let m2332 = f343(v0, v33, v2, v7, 2u64, List::cons(v20.clone(), v5), Adt18::C0(0u64))?;
                                                                        match m2332 {
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
                let v37: u64 = f185(v0.clone(), v1)?;
                let a2333 = v37;
                let a2334 = 33u64;
                if nat_eq(a2333, a2334) {
                    if f258(v6) {
                        Ok(Adt19::C1(f129(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v1)?))
                    } else {
                        let v38: Adt18 = f259(v0.clone(), v1)?;
                        let m2335 = f260(v38.clone());
                        match m2335 {
                            None => {
                                let m2336 = f443(v0.clone(), f261(v0.clone(), v1)?, v2)?;
                                match m2336 {
                                    Ok(v41) => {
                                        let a2337 = f264(f8(Str::lit(",]}:")), f185(v0.clone(), v41)?)?;
                                        let a2338 = 4u64;
                                        if nat_lt(a2337, a2338) {
                                            f263(v38, v41, v41)
                                        } else {
                                            f343(v0, v41, v2, v7, 0u64, List::<Bytes>::nil(), v38)
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
                    let a2339 = v37;
                    let a2340 = 91u64;
                    if if nat_eq(a2339, a2340) {
                        true
                    } else {
                        let a2341 = v37;
                        let a2342 = 123u64;
                        nat_eq(a2341, a2342)
                    } {
                        let m2345 = f443(v0.clone(), {
                            let a2343 = v1;
                            let a2344 = 1u64;
                            nat_add(a2343, a2344)?
                        }, v2)?;
                        match m2345 {
                            Ok(v43) => {
                                f262(v6, {
                                    let a2346 = v37;
                                    let a2347 = 91u64;
                                    nat_eq(a2346, a2347)
                                }, v1, {
                                    let a2350 = f185(v0.clone(), v43)?;
                                    let a2351 = {
                                        let a2348 = v37;
                                        let a2349 = 2u64;
                                        nat_add(a2348, a2349)?
                                    };
                                    if nat_eq(a2350, a2351) {
                                        Adt19::C0(Adt2::C9, {
                                            let a2352 = v43;
                                            let a2353 = 1u64;
                                            nat_add(a2352, a2353)?
                                        })
                                    } else {
                                        f343(v0, v43, v2, v7, {
                                            let a2354 = v37;
                                            let a2355 = 91u64;
                                            if nat_eq(a2354, a2355) {
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
                        let a2356 = v37;
                        let a2357 = 34u64;
                        if if nat_eq(a2356, a2357) {
                            true
                        } else {
                            let a2358 = v37;
                            let a2359 = 39u64;
                            nat_eq(a2358, a2359)
                        } {
                            let m2360 = f342(v0, v1, v2, false)?;
                            match m2360 {
                                Adt23::C0(v45, v46) => {
                                    f341(v6, v45, true, v1, v46)
                                }
                                Adt23::C1(v44) => {
                                    Ok(Adt19::C1(v44))
                                }
                            }
                        } else {
                            let m2361 = f256(v0.clone(), v1)?;
                            match m2361 {
                                None => {
                                    if f187(f344(v0.clone(), v1, true)?) {
                                        Ok(Adt19::C1(f129(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                    } else {
                                        let v48: u64 = f345(v0.clone(), v1, true)?;
                                        let v49: u64 = f251(v0.clone(), v48)?;
                                        let m2364 = {
                                            let a2362 = f185(v0.clone(), v49)?;
                                            let a2363 = 10u64;
                                            if nat_eq(a2362, a2363) {
                                                f443(v0.clone(), v49, v2)?
                                            } else {
                                                Ok::<u64, Bytes>(v49)
                                            }
                                        };
                                        match m2364 {
                                            Ok(v51) => {
                                                let a2365 = f185(v0.clone(), v49)?;
                                                let a2366 = 10u64;
                                                if if if nat_eq(a2365, a2366) {
                                                    f187(f426(f185(v0.clone(), v51)?))
                                                } else {
                                                    false
                                                } {
                                                    f187({
                                                        let a2367 = f185(v0.clone(), v51)?;
                                                        let a2368 = 58u64;
                                                        nat_eq(a2367, a2368)
                                                    })
                                                } else {
                                                    false
                                                } {
                                                    Ok(Adt19::C1(f129(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v49)?))
                                                } else {
                                                    f341(v6, f265(v0, v1, v48), false, v1, v48)
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

pub fn f344(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f185(v0.clone(), v1)?;
    if if f248(v3) {
        true
    } else {
        f249(v3)
    } {
        Ok(false)
    } else {
        let a2369 = v3;
        let a2370 = 45u64;
        if if if nat_eq(a2369, a2370) {
            true
        } else {
            let a2371 = v3;
            let a2372 = 63u64;
            nat_eq(a2371, a2372)
        } {
            true
        } else {
            let a2373 = v3;
            let a2374 = 58u64;
            nat_eq(a2373, a2374)
        } {
            Ok(f339(f185(v0, {
                let a2375 = v1;
                let a2376 = 1u64;
                nat_add(a2375, a2376)?
            })?, v2))
        } else {
            Ok(f187(f445(v3)?))
        }
    }
}

pub fn f345(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2377) = {
        let (h2378, _) = f218(Fn6::F446(v2, v0.clone()), {
            let a2380 = {
                let a2379 = v0;
                length_bytes(a2379)
            };
            let a2381 = 1u64;
            nat_add(a2380, a2381)?
        }, (v1, v1))?;
        h2378
    };
    Ok(h2377)
}

pub fn f346(v0: Bytes, p2406: u64, v2: u64, p2407: u64, p2408: Bytes) -> R<Adt23> {
    let m2405 = (p2406, (p2407, p2408));
    {
        let mut m2405 = m2405;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2405;
                let m2382 = v3;
                if m2382 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2382 - 1;
                    let v6: u64 = f251(v0.clone(), v1)?;
                    let v7: u64 = f185(v0.clone(), v6)?;
                    let a2383 = v7;
                    let a2384 = 58u64;
                    if nat_eq(a2383, a2384) {
                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f447(v6)?)))
                    } else {
                        let a2385 = v7;
                        let a2386 = 35u64;
                        if if nat_eq(a2385, a2386) {
                            true
                        } else {
                            let a2387 = v7;
                            let a2388 = 256u64;
                            nat_eq(a2387, a2388)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C0(v4, f252(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f448(v0.clone(), {
                                let a2389 = v6;
                                let a2390 = 1u64;
                                nat_add(a2389, a2390)?
                            })?;
                            let v9: u64 = {
                                let (h2391, _) = v8;
                                h2391
                            };
                            let v10: u64 = f253(v0.clone(), v9)?;
                            let v11: u64 = f251(v0.clone(), v10)?;
                            let v12: u64 = f185(v0.clone(), v11)?;
                            let a2392 = v12;
                            let a2393 = 256u64;
                            if if if if nat_eq(a2392, a2393) {
                                true
                            } else {
                                let a2396 = {
                                    let a2394 = v10;
                                    let a2395 = v9;
                                    nat_sub(a2394, a2395)
                                };
                                let a2397 = v2;
                                nat_lt(a2396, a2397)
                            } {
                                true
                            } else {
                                let a2398 = v12;
                                let a2399 = 35u64;
                                nat_eq(a2398, a2399)
                            } {
                                true
                            } else {
                                f183(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C0(v4, {
                                    let a2400 = v6;
                                    let a2401 = 1u64;
                                    nat_add(a2400, a2401)?
                                })))
                            } else {
                                let v13: u64 = f345(v0.clone(), v11, false)?;
                                let a2402 = v13;
                                let a2403 = v11;
                                if nat_eq(a2402, a2403) {
                                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f447(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt23>>((v13, (v5, f7(List::cons(v4, List::cons(f449({
                                        let (_, h2404) = v8;
                                        h2404
                                    }), List::cons(f265(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2405 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: Bytes, v1: u64) -> R<u64> {
    let v2: bool = {
        let a2411 = f185(v0.clone(), {
            let a2409 = v1;
            let a2410 = 1u64;
            nat_add(a2409, a2410)?
        })?;
        let a2412 = 60u64;
        nat_eq(a2411, a2412)
    };
    let (h2413, _) = f179(Fn5::F450(v0.clone(), v2), {
        let a2415 = {
            let a2414 = v0;
            length_bytes(a2414)
        };
        let a2416 = 1u64;
        nat_add(a2415, a2416)?
    }, if v2 {
        let a2417 = v1;
        let a2418 = 2u64;
        nat_add(a2417, a2418)?
    } else {
        let a2419 = v1;
        let a2420 = 1u64;
        nat_add(a2419, a2420)?
    })?;
    Ok(h2413)
}

pub fn f348(v0: Bytes, v1: u64) -> R<Adt18> {
    let v2: u64 = f451(v0, List::cons(f8(Str::lit("str")), List::cons(f8(Str::lit("int")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("bool")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("seq")), List::cons(f8(Str::lit("map")), List::<Bytes>::nil()))))))), 1u64)?;
    let a2421 = v2;
    let a2422 = 0u64;
    if nat_eq(a2421, a2422) {
        Ok(Adt18::C2(f129(Str::lit("a tag outside the YAML 1.2.2 core schema is not read (YAML 1.2.2 section 10.3)"), v1)?))
    } else {
        Ok(Adt18::C0(v2))
    }
}

pub fn f349(v0: u64) -> R<Bytes> {
    f129(Str::lit("a node's content does not match its tag (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f350(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f452(v0, v1, v2)
}

pub fn f351(v0: Bytes, v1: u64) -> R<bool> {
    let a2423 = f185(v0.clone(), v1)?;
    let a2424 = 58u64;
    if nat_eq(a2423, a2424) {
        if f248(f185(v0.clone(), {
            let a2425 = v1;
            let a2426 = 1u64;
            nat_add(a2425, a2426)?
        })?) {
            Ok(true)
        } else {
            Ok(f249(f185(v0, {
                let a2427 = v1;
                let a2428 = 1u64;
                nat_add(a2427, a2428)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f352(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2429 = v0;
        utf8_encode(a2429)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f89(v1)?, List::<Bytes>::nil())))))
}

pub fn f353(p2432: List<Adt20>, p2433: List<Adt20>) -> List<Adt20> {
    let m2431 = (p2432, p2433);
    {
        let mut m2431 = m2431;
        loop {
            let step = {
                let (v0, v1) = m2431;
                let m2430 = v0;
                match m2430.uncons() {
                    None => {
                        Err::<(List<Adt20>, List<Adt20>), List<Adt20>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt20>, List<Adt20>), List<Adt20>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: u64) -> R<bool> {
    f453(v0, f454())
}

pub fn f355(v0: u64) -> bool {
    let a2434 = 65u64;
    let a2435 = v0;
    if if nat_le(a2434, a2435) {
        let a2436 = v0;
        let a2437 = 90u64;
        nat_le(a2436, a2437)
    } else {
        false
    } {
        true
    } else {
        let a2438 = 97u64;
        let a2439 = v0;
        if nat_le(a2438, a2439) {
            let a2440 = v0;
            let a2441 = 122u64;
            nat_le(a2440, a2441)
        } else {
            false
        }
    }
}

pub fn f356(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2442, _) = f179(Fn5::F455(v1, v0.clone()), {
        let a2444 = {
            let a2443 = v0;
            length_bytes(a2443)
        };
        let a2445 = 1u64;
        nat_add(a2444, a2445)?
    }, v2)?;
    Ok(h2442)
}

pub fn f357(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f330(v0.clone(), v1)?;
    let a2446 = v2;
    let a2447 = 48u64;
    if nat_eq(a2446, a2447) {
        let v3: u64 = f330(v0.clone(), {
            let a2448 = v1;
            let a2449 = 1u64;
            nat_add(a2448, a2449)?
        })?;
        let a2450 = v3;
        let a2451 = 88u64;
        if if if nat_eq(a2450, a2451) {
            true
        } else {
            let a2452 = v3;
            let a2453 = 120u64;
            nat_eq(a2452, a2453)
        } {
            f456(f330(v0.clone(), {
                let a2454 = v1;
                let a2455 = 2u64;
                nat_add(a2454, a2455)?
            })?)
        } else {
            false
        } {
            f356(v0, 1u64, {
                let a2456 = v1;
                let a2457 = 2u64;
                nat_add(a2456, a2457)?
            })
        } else {
            f356(v0, 2u64, {
                let a2458 = v1;
                let a2459 = 1u64;
                nat_add(a2458, a2459)?
            })
        }
    } else {
        if f457(v2) {
            f356(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f358(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f356(v0.clone(), 0u64, v1)?;
    let a2460 = f330(v0.clone(), v2)?;
    let a2461 = 46u64;
    if nat_eq(a2460, a2461) {
        let v3: u64 = f356(v0.clone(), 0u64, {
            let a2462 = v2;
            let a2463 = 1u64;
            nat_add(a2462, a2463)?
        })?;
        let a2464 = v1;
        let a2465 = v2;
        if if nat_lt(a2464, a2465) {
            true
        } else {
            let a2468 = {
                let a2466 = v2;
                let a2467 = 1u64;
                nat_add(a2466, a2467)?
            };
            let a2469 = v3;
            nat_lt(a2468, a2469)
        } {
            f458(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2470 = v1;
        let a2471 = v2;
        if nat_lt(a2470, a2471) {
            let v4: u64 = f458(v0, v2)?;
            let a2472 = v2;
            let a2473 = v4;
            if nat_lt(a2472, a2473) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f359(v0: List<Adt20>, v1: Str) -> bool {
    let m2474 = v0;
    match m2474.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f459(v2, {
                let a2475 = v1;
                utf8_encode(a2475)
            })
        }
    }
}

pub fn f360(p2488: u64, p2489: List<Adt20>, p2490: List<Bytes>, v3: bool) -> Result<List<Adt20>, Bytes> {
    let m2487 = (p2488, (p2489, p2490));
    {
        let mut m2487 = m2487;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2487;
                let m2476 = v0;
                if m2476 == 0 {
                    Err::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>(Err::<List<Adt20>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2476 - 1;
                    let m2477 = v1;
                    match m2477.uncons() {
                        None => {
                            Err::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>(Err::<List<Adt20>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2478 = f460(v5.clone());
                            let a2479 = 3u64;
                            if nat_eq(a2478, a2479) {
                                Ok::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f461(v5);
                                let v8: Bytes = f462(v7.clone());
                                if !f32(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f463(v7.clone()) {
                                        let m2480 = v2;
                                        match m2480.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<List<Adt20>, Bytes>>(Err::<List<Adt20>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f32(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2482 = {
                                                            let a2481 = v10.clone();
                                                            length_list(a2481)
                                                        };
                                                        let a2483 = 0u64;
                                                        nat_eq(a2482, a2483)
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
                                                let a2485 = {
                                                    let a2484 = v2.clone();
                                                    length_list(a2484)
                                                };
                                                let a2486 = 0u64;
                                                nat_eq(a2485, a2486)
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
                Ok(next) => { m2487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: List<Adt20>) -> List<Adt20> {
    let m2491 = v0;
    match m2491.uncons() {
        None => {
            List::<Adt20>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f362(v0: u64, v1: u64, v2: List<Adt20>, v3: bool) -> R<Result<(Adt13, List<Adt20>), Bytes>> {
    let m2492 = f365(v2);
    match m2492 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f359(f367(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m2493 = if v6 {
                f365(f361(f367(v5.clone())))
            } else {
                Ok::<Adt24, Bytes>(Adt24::C0(Bytes::lit(&[]), f367(v5.clone())))
            };
            match m2493 {
                Ok(v8) => {
                    let m2494 = f366(f367(v8.clone()), Str::lit("{"));
                    match m2494 {
                        Ok(v10) => {
                            let m2499 = {
                                let a2495 = v1;
                                let a2496 = 2u64;
                                if if nat_eq(a2495, a2496) {
                                    true
                                } else {
                                    let a2497 = v1;
                                    let a2498 = 7u64;
                                    nat_eq(a2497, a2498)
                                } {
                                    f464(v0, v10, List::<Adt14>::nil())?
                                } else {
                                    f465(v0, v10, List::<Adt14>::nil())?
                                }
                            };
                            match m2499 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(v1, f369(v5), f369(v8), {
                                        let (h2500, _) = v12;
                                        h2500
                                    }, List::<Bytes>::nil(), f370(Bytes::lit(&[]))), {
                                        let (_, h2501) = v12;
                                        h2501
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

pub fn f363(v0: List<Adt20>, v1: Bytes) -> R<Result<(Adt13, List<Adt20>), Bytes>> {
    let m2502 = f365(v0);
    match m2502 {
        Ok(v3) => {
            let m2503 = f466(f367(v3.clone()))?;
            match m2503 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<Adt20>), Bytes>((Adt13::C0(9u64, f369(v3), Bytes::lit(&[]), List::<Adt14>::nil(), List::cons(v1, List::<Bytes>::nil()), f370(Bytes::lit(&[]))), v5)))
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

pub fn f364(v0: Str, v1: List<Adt20>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2504 = v0;
        utf8_encode(a2504)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f467(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f365(v0: List<Adt20>) -> Result<Adt24, Bytes> {
    let m2505 = v0.clone();
    match m2505.uncons() {
        None => {
            Err::<Adt24, Bytes>(f364(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2506 = f460(v1.clone());
            let a2507 = 0u64;
            if nat_eq(a2506, a2507) {
                Ok::<Adt24, Bytes>(Adt24::C0(f468(f461(v1)), v2))
            } else {
                Err::<Adt24, Bytes>(f364(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f366(v0: List<Adt20>, v1: Str) -> Result<List<Adt20>, Bytes> {
    if f359(v0.clone(), v1.clone()) {
        Ok::<List<Adt20>, Bytes>(f361(v0))
    } else {
        Err::<List<Adt20>, Bytes>(f364(v1, v0))
    }
}

pub fn f367(v0: Adt24) -> List<Adt20> {
    let m2508 = v0;
    match m2508 {
        Adt24::C0(_, v2) => {
            v2
        }
    }
}

pub fn f368(p2516: u64, p2517: List<Adt20>, p2518: List<Bytes>) -> Result<(List<Bytes>, List<Adt20>), Bytes> {
    let m2515 = (p2516, (p2517, p2518));
    {
        let mut m2515 = m2515;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2515;
                let m2509 = v0;
                if m2509 == 0 {
                    Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2509 - 1;
                    let m2510 = v1.clone();
                    match m2510.uncons() {
                        None => {
                            Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(f364(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2511 = f460(v4.clone());
                            let a2512 = 3u64;
                            if !nat_eq(a2511, a2512) {
                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(f364(Str::lit("a string"), v1)))
                            } else {
                                if f359(v5.clone(), Str::lit(",")) {
                                    if f359(f361(v5.clone()), Str::lit("}")) {
                                        let m2513 = f366(f361(f361(v5)), Str::lit(";"));
                                        match m2513 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Ok::<(List<Bytes>, List<Adt20>), Bytes>((f66(List::cons(f461(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>((v3, (f361(v5), List::cons(f461(v4), v2))))
                                    }
                                } else {
                                    if f359(v5.clone(), Str::lit("}")) {
                                        let m2514 = f366(f361(v5), Str::lit(";"));
                                        match m2514 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Ok::<(List<Bytes>, List<Adt20>), Bytes>((f66(List::cons(f461(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt20>, List<Bytes>)), Result<(List<Bytes>, List<Adt20>), Bytes>>(Err::<(List<Bytes>, List<Adt20>), Bytes>(f364(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: Adt24) -> Bytes {
    let m2519 = v0;
    match m2519 {
        Adt24::C0(v1, _) => {
            v1
        }
    }
}

pub fn f370(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]), List::<Adt16>::nil())
}

pub fn f371(v0: u64, v1: List<Adt20>, v2: u64) -> R<Result<Adt25, Bytes>> {
    let m2520 = v0;
    if m2520 == 0 {
        Ok(Err::<Adt25, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2520 - 1;
        let a2521 = v2;
        let a2522 = 0u64;
        if nat_eq(a2521, a2522) {
            let m2523 = f273(v1)?;
            match m2523 {
                Ok(v5) => {
                    if f359(v5.clone(), Str::lit("(")) {
                        let m2524 = f371(v3, f361(v5), 1u64)?;
                        match m2524 {
                            Ok(v7) => {
                                if f359(f372(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt25, Bytes>(Adt25::C0(f469(f470(Str::lit("Option"), f373(v7.clone()))?, f7(List::cons(f471(f373(v7.clone())), List::cons(f8(Str::lit("OrNull")), List::<Bytes>::nil())))), f361(f372(v7)))))
                                } else {
                                    Ok(Ok::<Adt25, Bytes>(Adt25::C0(f373(v7.clone()), f372(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt25, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2525 = v5.clone();
                        match m2525.uncons() {
                            None => {
                                Ok(Err::<Adt25, Bytes>(f364(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f461(v8.clone());
                                let a2526 = f460(v8);
                                let a2527 = 0u64;
                                if !nat_eq(a2526, a2527) {
                                    Ok(Err::<Adt25, Bytes>(f364(Str::lit("a type"), v5)))
                                } else {
                                    if f32(v10.clone(), f8(Str::lit("sequence"))) {
                                        let m2528 = f366(v9, Str::lit("<"));
                                        match m2528 {
                                            Ok(v12) => {
                                                let m2529 = f371(v3, v12, 0u64)?;
                                                match m2529 {
                                                    Ok(v14) => {
                                                        let m2530 = f366(f372(v14.clone()), Str::lit(">"));
                                                        match m2530 {
                                                            Ok(v16) => {
                                                                f472(f469(f470(Str::lit("List"), f373(v14.clone()))?, f7(List::cons(f471(f373(v14)), List::cons(f8(Str::lit("Sequence")), List::<Bytes>::nil())))), v16)
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
                                        if f32(v10.clone(), f8(Str::lit("FrozenArray"))) {
                                            let m2531 = f366(v9, Str::lit("<"));
                                            match m2531 {
                                                Ok(v18) => {
                                                    let m2532 = f371(v3, v18, 0u64)?;
                                                    match m2532 {
                                                        Ok(v20) => {
                                                            let m2533 = f366(f372(v20.clone()), Str::lit(">"));
                                                            match m2533 {
                                                                Ok(v22) => {
                                                                    f472(f469(f470(Str::lit("List"), f373(v20.clone()))?, f7(List::cons(f471(f373(v20)), List::cons(f8(Str::lit("Array")), List::<Bytes>::nil())))), v22)
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
                                            if f32(v10.clone(), f8(Str::lit("ObservableArray"))) {
                                                let m2534 = f366(v9, Str::lit("<"));
                                                match m2534 {
                                                    Ok(v24) => {
                                                        let m2535 = f371(v3, v24, 0u64)?;
                                                        match m2535 {
                                                            Ok(v26) => {
                                                                let m2536 = f366(f372(v26.clone()), Str::lit(">"));
                                                                match m2536 {
                                                                    Ok(v28) => {
                                                                        f472(f469(f470(Str::lit("List"), f373(v26.clone()))?, f7(List::cons(f471(f373(v26)), List::cons(f8(Str::lit("ObservableArray")), List::<Bytes>::nil())))), v28)
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
                                                if f32(v10.clone(), f8(Str::lit("async_sequence"))) {
                                                    let m2537 = f366(v9, Str::lit("<"));
                                                    match m2537 {
                                                        Ok(v30) => {
                                                            let m2538 = f371(v3, v30, 0u64)?;
                                                            match m2538 {
                                                                Ok(v32) => {
                                                                    let m2539 = f366(f372(v32), Str::lit(">"));
                                                                    match m2539 {
                                                                        Ok(v34) => {
                                                                            f472(f473(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v34)
                                                                        }
                                                                        Err(v33) => {
                                                                            Ok(Err::<Adt25, Bytes>(v33))
                                                                        }
                                                                    }
                                                                }
                                                                Err(v31) => {
                                                                    Ok(Err::<Adt25, Bytes>(v31))
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt25, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v10.clone(), f8(Str::lit("Promise"))) {
                                                        let m2540 = f366(v9, Str::lit("<"));
                                                        match m2540 {
                                                            Ok(v36) => {
                                                                let m2541 = f371(v3, v36, 0u64)?;
                                                                match m2541 {
                                                                    Ok(v38) => {
                                                                        let m2542 = f366(f372(v38), Str::lit(">"));
                                                                        match m2542 {
                                                                            Ok(v40) => {
                                                                                f472(f473(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v40)
                                                                            }
                                                                            Err(v39) => {
                                                                                Ok(Err::<Adt25, Bytes>(v39))
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
                                                    } else {
                                                        if f32(v10.clone(), f8(Str::lit("record"))) {
                                                            let m2543 = f366(v9, Str::lit("<"));
                                                            match m2543 {
                                                                Ok(v42) => {
                                                                    let m2544 = v42.clone();
                                                                    match m2544.uncons() {
                                                                        None => {
                                                                            Ok(Err::<Adt25, Bytes>(f364(Str::lit("a string type"), v42)))
                                                                        }
                                                                        Some((v43, v44)) => {
                                                                            if if f32(f461(v43.clone()), f8(Str::lit("DOMString"))) {
                                                                                true
                                                                            } else {
                                                                                if f32(f461(v43.clone()), f8(Str::lit("USVString"))) {
                                                                                    true
                                                                                } else {
                                                                                    f32(f461(v43.clone()), f8(Str::lit("ByteString")))
                                                                                }
                                                                            } {
                                                                                let m2545 = f366(v44, Str::lit(","));
                                                                                match m2545 {
                                                                                    Ok(v46) => {
                                                                                        let m2546 = f371(v3, v46, 0u64)?;
                                                                                        match m2546 {
                                                                                            Ok(v48) => {
                                                                                                let m2547 = f366(f372(v48.clone()), Str::lit(">"));
                                                                                                match m2547 {
                                                                                                    Ok(v50) => {
                                                                                                        let m2548 = f474(f461(v43));
                                                                                                        match m2548 {
                                                                                                            None => {
                                                                                                                Ok(Err::<Adt25, Bytes>(f364(Str::lit("a string type"), v42)))
                                                                                                            }
                                                                                                            Some(v51) => {
                                                                                                                let v52: Adt15 = f475(f373(v48.clone()))?;
                                                                                                                f472(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f291(v51.clone()), List::cons(f8(Str::lit(" * ")), List::cons(f291(v52.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f292(v52.clone()), f289(v52.clone()), f7(List::cons(f471(v51), List::cons(f471(f373(v48)), List::cons(f8(Str::lit("Record")), List::<Bytes>::nil())))), f201(v52)), v50)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    Err(v49) => {
                                                                                                        Ok(Err::<Adt25, Bytes>(v49))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v47) => {
                                                                                                Ok(Err::<Adt25, Bytes>(v47))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v45) => {
                                                                                        Ok(Err::<Adt25, Bytes>(v45))
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                Ok(Err::<Adt25, Bytes>(f364(Str::lit("DOMString, USVString or ByteString"), v42)))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(v41) => {
                                                                    Ok(Err::<Adt25, Bytes>(v41))
                                                                }
                                                            }
                                                        } else {
                                                            if f32(v10.clone(), f8(Str::lit("unsigned"))) {
                                                                if f359(v9.clone(), Str::lit("short")) {
                                                                    f472(f469(f370(f8(Str::lit("Nat"))), f8(Str::lit("UnsignedShort"))), f361(v9))
                                                                } else {
                                                                    if f359(v9.clone(), Str::lit("long")) {
                                                                        let v53: List<Adt20> = f361(v9);
                                                                        if f359(v53.clone(), Str::lit("long")) {
                                                                            f472(f469(f370(f8(Str::lit("Nat"))), f8(Str::lit("UnsignedLongLong"))), f361(v53))
                                                                        } else {
                                                                            f472(f469(f370(f8(Str::lit("Nat"))), f8(Str::lit("UnsignedLong"))), v53)
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt25, Bytes>(f364(Str::lit("short or long"), v9)))
                                                                    }
                                                                }
                                                            } else {
                                                                if f32(v10.clone(), f8(Str::lit("short"))) {
                                                                    f472(f469(f370(f8(Str::lit("Int"))), f8(Str::lit("Short"))), v9)
                                                                } else {
                                                                    if f32(v10.clone(), f8(Str::lit("long"))) {
                                                                        if f359(v9.clone(), Str::lit("long")) {
                                                                            f472(f469(f370(f8(Str::lit("Int"))), f8(Str::lit("LongLong"))), f361(v9))
                                                                        } else {
                                                                            f472(f469(f370(f8(Str::lit("Int"))), f8(Str::lit("Long"))), v9)
                                                                        }
                                                                    } else {
                                                                        if f32(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                            if f359(v9.clone(), Str::lit("float")) {
                                                                                f472(f469(f476(false), f8(Str::lit("UnrestrictedFloat"))), f361(v9))
                                                                            } else {
                                                                                if f359(v9.clone(), Str::lit("double")) {
                                                                                    f472(f469(f476(true), f8(Str::lit("UnrestrictedDouble"))), f361(v9))
                                                                                } else {
                                                                                    Ok(Err::<Adt25, Bytes>(f364(Str::lit("float or double"), v9)))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            let m2549 = f474(v10.clone());
                                                                            match m2549 {
                                                                                None => {
                                                                                    let v54: Bytes = f468(v10);
                                                                                    f472(Adt15::C0(0u64, f276(v54.clone())?, List::cons(v54.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v54, List::<Adt16>::nil()), v9)
                                                                                }
                                                                                Some(v55) => {
                                                                                    f472(v55, v9)
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
                Err(v4) => {
                    Ok(Err::<Adt25, Bytes>(v4))
                }
            }
        } else {
            let m2550 = f371(v3, v1, 0u64)?;
            match m2550 {
                Ok(v57) => {
                    let v58: Adt15 = f373(v57.clone());
                    let v61: Adt15 = {
                        let a2551 = f290(v58.clone());
                        let a2552 = 0u64;
                        if if nat_eq(a2551, a2552) {
                            if f32(f471(v58.clone()), f8(Str::lit("Undefined"))) {
                                f32(f289(v58.clone()), f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m2553 = f381(false, f471(v58.clone()))?;
                            match m2553 {
                                None => {
                                    Adt15::C0(1u64, Bytes::lit(&[]), List::<Bytes>::nil(), f8(Str::lit("the union member undefined has no constructor name")), f471(v58), List::<Adt16>::nil())
                                }
                                Some(v59) => {
                                    Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v59, List::cons(f8(Str::lit(", fields: []}")), List::<Bytes>::nil())))), List::<Bytes>::nil(), Bytes::lit(&[]), f471(v58), List::<Adt16>::nil())
                                }
                            }
                        } else {
                            let a2554 = f290(v58.clone());
                            let a2555 = 0u64;
                            if nat_eq(a2554, a2555) {
                                if f32(f289(v58.clone()), Bytes::lit(&[])) {
                                    if f32(f471(v58.clone()), Bytes::lit(&[])) {
                                        Adt15::C0(1u64, Bytes::lit(&[]), f292(v58.clone()), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), f471(v58), List::<Adt16>::nil())
                                    } else {
                                        let m2556 = f381(false, f471(v58.clone()))?;
                                        match m2556 {
                                            None => {
                                                Adt15::C0(1u64, Bytes::lit(&[]), f292(v58.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f471(v58.clone()), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), f471(v58), List::<Adt16>::nil())
                                            }
                                            Some(v60) => {
                                                Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v60, List::cons(f8(Str::lit(", fields: [")), List::cons(f291(v58.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f292(v58.clone()), Bytes::lit(&[]), f471(v58.clone()), f201(v58))
                                            }
                                        }
                                    }
                                } else {
                                    Adt15::C0(1u64, Bytes::lit(&[]), f292(v58.clone()), f289(v58.clone()), f471(v58), List::<Adt16>::nil())
                                }
                            } else {
                                v58
                            }
                        }
                    };
                    if f359(f372(v57.clone()), Str::lit("or")) {
                        let m2557 = f371(v3, f361(f372(v57)), 2u64)?;
                        match m2557 {
                            Ok(v63) => {
                                Ok(Ok::<Adt25, Bytes>(Adt25::C0(Adt15::C0(1u64, if f32(f291(v61.clone()), Bytes::lit(&[])) {
                                    f291(f373(v63.clone()))
                                } else {
                                    if f32(f291(f373(v63.clone())), Bytes::lit(&[])) {
                                        f291(v61.clone())
                                    } else {
                                        f7(List::cons(f291(v61.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f291(f373(v63.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f205(f292(v61.clone()), f292(f373(v63.clone()))), if f32(f289(v61.clone()), Bytes::lit(&[])) {
                                    f289(f373(v63.clone()))
                                } else {
                                    f289(v61.clone())
                                }, f7(List::cons(f471(v61.clone()), List::cons(f8(Str::lit("Or")), List::cons(f471(f373(v63.clone())), List::<Bytes>::nil())))), f142(f201(v61), f201(f373(v63.clone())))), f372(v63))))
                            }
                            Err(v62) => {
                                Ok(Err::<Adt25, Bytes>(v62))
                            }
                        }
                    } else {
                        let a2558 = v2;
                        let a2559 = 2u64;
                        if if nat_eq(a2558, a2559) {
                            f359(f372(v57.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt25, Bytes>(Adt25::C0(v61, f361(f372(v57)))))
                        } else {
                            Ok(Err::<Adt25, Bytes>({
                                let a2560 = v2;
                                let a2561 = 1u64;
                                if nat_eq(a2560, a2561) {
                                    f364(Str::lit("or"), f372(v57))
                                } else {
                                    f364(Str::lit("or or )"), f372(v57))
                                }
                            }))
                        }
                    }
                }
                Err(v56) => {
                    Ok(Err::<Adt25, Bytes>(v56))
                }
            }
        }
    }
}

pub fn f372(v0: Adt25) -> List<Adt20> {
    let m2562 = v0;
    match m2562 {
        Adt25::C0(_, v2) => {
            v2
        }
    }
}

pub fn f373(v0: Adt25) -> Adt15 {
    let m2563 = v0;
    match m2563 {
        Adt25::C0(v1, _) => {
            v1
        }
    }
}

pub fn f374() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f375(v0: Bytes) -> R<Bytes> {
    let m2566 = {
        let a2564 = v0.clone();
        let a2565 = 0u64;
        index_bytes(a2564, a2565)
    };
    match m2566 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2572 = {
                let a2567 = f8(Str::lit("u"));
                let a2568 = f477(8u64, f60(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2567, a2568)
            };
            let a2573 = f46(v0.clone(), 1u64, {
                let a2570 = {
                    let a2569 = v0;
                    length_bytes(a2569)
                };
                let a2571 = 1u64;
                nat_sub(a2570, a2571)
            });
            Ok(append_bytes(a2572, a2573))
        }
    }
}

pub fn f376(v0: List<Adt14>) -> R<List<Adt14>> {
    let m2574 = v0;
    match m2574.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            f478(v1, f376(v2)?)
        }
    }
}

pub fn f377(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m2575 = v0;
    match m2575.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f377(v3, v1))
        }
    }
}

pub fn f378(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<Adt14> {
    let m2576 = v0;
    match m2576.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2577 = f196(v3.clone());
            let a2578 = v1;
            if if nat_eq(a2577, a2578) {
                f32(f198(v3.clone()), v2.clone())
            } else {
                false
            } {
                f377(f200(v3), f378(v4, v1, v2))
            } else {
                f378(v4, v1, v2)
            }
        }
    }
}

pub fn f379(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<Adt14> {
    let m2579 = v0;
    match m2579.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2580 = f196(v3.clone());
            let a2581 = 8u64;
            if if nat_eq(a2580, a2581) {
                f32(f198(v3.clone()), v2.clone())
            } else {
                false
            } {
                f377(f479(v1.clone(), f211(v3)), f379(v4, v1, v2))
            } else {
                f379(v4, v1, v2)
            }
        }
    }
}

pub fn f380(v0: Adt14) -> Bytes {
    let m2582 = v0;
    match m2582 {
        Adt14::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f381(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f480(v1)
    } else {
        f384(v1)
    }
}

pub fn f382(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2583 = v1;
    match m2583.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2584 = f381(v0, v2)?;
            match m2584 {
                None => {
                    f382(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f382(v0, v3)?))
                }
            }
        }
    }
}

pub fn f383(v0: Adt14) -> u64 {
    let m2585 = v0;
    match m2585 {
        Adt14::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f384(v0: Bytes) -> R<Option<Bytes>> {
    let m2586 = f481(v0)?;
    match m2586 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f482(v1)?))
        }
    }
}

pub fn f385(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2587 = v1;
    match m2587 {
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
                let a2598 = if f105(v9, {
                    let a2596 = 0u64;
                    let a2597 = v10;
                    nat_lt(a2596, a2597)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2599 = f230(false, v10)?;
                append_bytes(a2598, a2599)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m2590 = f480({
                let a2588 = f8(Str::lit("n"));
                let a2589 = f231(false, v4, v5)?;
                append_bytes(a2588, a2589)
            })?;
            match m2590 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2594 = f8(Str::lit("nMinus"));
                        let a2595 = f46(v6.clone(), 1u64, {
                            let a2592 = {
                                let a2591 = v6;
                                length_bytes(a2591)
                            };
                            let a2593 = 1u64;
                            nat_sub(a2592, a2593)
                        });
                        append_bytes(a2594, a2595)
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
            f381(v0, v2)
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

pub fn f386(v0: Adt2) -> R<Bytes> {
    f171(v0)
}

pub fn f387(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2600 = v0.clone();
        length_bytes(a2600)
    };
    let a2604 = {
        let a2601 = v1;
        let a2602 = 2u64;
        let a2603 = 0u64;
        nat_rem(a2601, a2602, a2603)
    };
    let a2605 = 0u64;
    if nat_eq(a2604, a2605) {
        let (_, h2606) = {
            let (h2607, _) = f483(Fn14::F484(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2607
        };
        Ok(h2606)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f388(v0: u64) -> bool {
    let a2608 = v0;
    let a2609 = 8u64;
    if nat_eq(a2608, a2609) {
        true
    } else {
        let a2610 = v0;
        let a2611 = 9u64;
        if nat_eq(a2610, a2611) {
            true
        } else {
            let a2612 = v0;
            let a2613 = 10u64;
            if nat_eq(a2612, a2613) {
                true
            } else {
                let a2614 = v0;
                let a2615 = 12u64;
                if nat_eq(a2614, a2615) {
                    true
                } else {
                    let a2616 = v0;
                    let a2617 = 13u64;
                    nat_eq(a2616, a2617)
                }
            }
        }
    }
}

pub fn f389(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2620 = {
        let a2618 = v0;
        let a2619 = v1;
        index_bytes(a2618, a2619)
    };
    match m2620 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f485(v2)? {
                Ok(Some({
                    let a2621 = v1;
                    let a2622 = 1u64;
                    nat_add(a2621, a2622)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f390(v0: u8) -> R<u64> {
    let a2669 = {
        let v1: u8 = v0;
        let a2667 = {
            let a2643 = {
                let a2631 = {
                    let a2625 = {
                        let a2623 = v1;
                        let a2624 = 128u8;
                        fixed_u8::bit_and(a2623, a2624)
                    };
                    let a2626 = 128u8;
                    if equal(a2625, a2626) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2632 = {
                    let a2629 = {
                        let a2627 = v1;
                        let a2628 = 64u8;
                        fixed_u8::bit_and(a2627, a2628)
                    };
                    let a2630 = 64u8;
                    if equal(a2629, a2630) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2631, a2632)?
            };
            let a2644 = {
                let a2641 = {
                    let a2635 = {
                        let a2633 = v1;
                        let a2634 = 32u8;
                        fixed_u8::bit_and(a2633, a2634)
                    };
                    let a2636 = 32u8;
                    if equal(a2635, a2636) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2642 = {
                    let a2639 = {
                        let a2637 = v1;
                        let a2638 = 16u8;
                        fixed_u8::bit_and(a2637, a2638)
                    };
                    let a2640 = 16u8;
                    if equal(a2639, a2640) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2641, a2642)?
            };
            nat_add(a2643, a2644)?
        };
        let a2668 = {
            let a2665 = {
                let a2653 = {
                    let a2647 = {
                        let a2645 = v1;
                        let a2646 = 8u8;
                        fixed_u8::bit_and(a2645, a2646)
                    };
                    let a2648 = 8u8;
                    if equal(a2647, a2648) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2654 = {
                    let a2651 = {
                        let a2649 = v1;
                        let a2650 = 4u8;
                        fixed_u8::bit_and(a2649, a2650)
                    };
                    let a2652 = 4u8;
                    if equal(a2651, a2652) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2653, a2654)?
            };
            let a2666 = {
                let a2663 = {
                    let a2657 = {
                        let a2655 = v1;
                        let a2656 = 2u8;
                        fixed_u8::bit_and(a2655, a2656)
                    };
                    let a2658 = 2u8;
                    if equal(a2657, a2658) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2664 = {
                    let a2661 = {
                        let a2659 = v1;
                        let a2660 = 1u8;
                        fixed_u8::bit_and(a2659, a2660)
                    };
                    let a2662 = 1u8;
                    if equal(a2661, a2662) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2663, a2664)?
            };
            nat_add(a2665, a2666)?
        };
        nat_add(a2667, a2668)?
    };
    let a2670 = 48u64;
    Ok(nat_sub(a2669, a2670))
}

pub fn f391(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2671 = v0.clone();
        length_bytes(a2671)
    };
    let a2672 = v1;
    let a2673 = v2;
    if if nat_lt(a2672, a2673) {
        let a2678 = f103(v0, {
            let a2676 = v2;
            let a2677 = {
                let a2674 = v1;
                let a2675 = 1u64;
                nat_add(a2674, a2675)?
            };
            nat_sub(a2676, a2677)
        })?;
        let a2679 = 48u64;
        nat_eq(a2678, a2679)
    } else {
        false
    } {
        Ok(Some({
            let a2680 = v1;
            let a2681 = 1u64;
            nat_add(a2680, a2681)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f392(v0: Adt22) -> R<Adt22> {
    let v1: u32 = f486(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2682) => {
            h2682
        }
    });
    let v2: u32 = f306(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2683, _, _) => {
            h2683
        }
    }, f306(f487(match v0.clone() {
        Adt22::C0(_, _, _, _, h2684, _, _, _, _, _) => {
            h2684
        }
    }), f306(f488(match v0.clone() {
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
    }), f306(v1, {
        let m2690 = match v0.clone() {
            Adt22::C0(_, _, _, _, _, _, _, _, h2689, _) => {
                h2689
            }
        };
        match m2690 {
            Adt21::C0(h2688, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2688
            }
        }
    }))));
    let v3: u32 = f306(f489(match v0.clone() {
        Adt22::C0(h2691, _, _, _, _, _, _, _, _, _) => {
            h2691
        }
    }), f490(match v0.clone() {
        Adt22::C0(h2692, _, _, _, _, _, _, _, _, _) => {
            h2692
        }
    }, match v0.clone() {
        Adt22::C0(_, h2693, _, _, _, _, _, _, _, _) => {
            h2693
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2694, _, _, _, _, _, _, _) => {
            h2694
        }
    }));
    Ok(Adt22::C0(f306(v2, v3), match v0.clone() {
        Adt22::C0(h2695, _, _, _, _, _, _, _, _, _) => {
            h2695
        }
    }, match v0.clone() {
        Adt22::C0(_, h2696, _, _, _, _, _, _, _, _) => {
            h2696
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2697, _, _, _, _, _, _, _) => {
            h2697
        }
    }, f306(match v0.clone() {
        Adt22::C0(_, _, _, h2698, _, _, _, _, _, _) => {
            h2698
        }
    }, v2), match v0.clone() {
        Adt22::C0(_, _, _, _, h2699, _, _, _, _, _) => {
            h2699
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2700, _, _, _, _) => {
            h2700
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2701, _, _, _) => {
            h2701
        }
    }, f491(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2702, _) => {
            h2702
        }
    }), {
        let a2704 = match v0 {
            Adt22::C0(_, _, _, _, _, _, _, _, _, h2703) => {
                h2703
            }
        };
        let a2705 = 1u64;
        nat_add(a2704, a2705)?
    }))
}

pub fn f393(v0: u32) -> u64 {
    let m2707 = {
        let a2706 = v0;
        fixed_u64::convert(i128::from(a2706))
    };
    match m2707 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f394(v0: u64) -> u32 {
    let m2711 = {
        let a2710 = {
            let a2708 = v0;
            let a2709 = 4294967295u64;
            fixed_u64::bit_and(a2708, a2709)
        };
        fixed_u32::convert(i128::from(a2710))
    };
    match m2711 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f395(v0: u64) -> u32 {
    let (_, h2712) = {
        let (_, h2713) = f492(Fn15::F493, 32u64, (v0, (1u32, 0u32)));
        h2713
    };
    h2712
}

pub fn f396(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2718 = f494(v0, 24u32);
    let a2719 = {
        let a2716 = f494(v1, 16u32);
        let a2717 = {
            let a2714 = f494(v2, 8u32);
            let a2715 = v3;
            fixed_u32::bit_or(a2714, a2715)
        };
        fixed_u32::bit_or(a2716, a2717)
    };
    fixed_u32::bit_or(a2718, a2719)
}

pub fn f397(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2720 = v2;
    let a2721 = v1;
    if nat_lt(a2720, a2721) {
        let m2724 = {
            let a2722 = v0;
            let a2723 = v2;
            index_bytes(a2722, a2723)
        };
        match m2724 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2726 = {
                    let a2725 = v3;
                    fixed_u32::convert(i128::from(a2725))
                };
                match m2726 {
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
        let a2727 = v2;
        let a2728 = v1;
        if nat_eq(a2727, a2728) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f398(v0: List<u64>) -> R<Bytes> {
    let m2729 = f495(v0, List::<u64>::nil());
    match m2729.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2730 = f89(v1)?;
            let a2731 = f496(v2)?;
            Ok(append_bytes(a2730, a2731))
        }
    }
}

pub fn f399(p2734: u64, p2735: List<u64>, v2: u64) -> R<List<u64>> {
    let m2733 = (p2734, p2735);
    {
        let mut m2733 = m2733;
        loop {
            let step = {
                let (v0, v1) = m2733;
                let m2732 = v0;
                if m2732 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2732 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f497(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: u64, v1: u64) -> List<u64> {
    let m2736 = v0;
    if m2736 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2736 - 1;
        let a2737 = v1;
        let a2738 = 0u64;
        if nat_eq(a2737, a2738) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2739 = v1;
                let a2740 = f498();
                let a2741 = 0u64;
                nat_rem(a2739, a2740, a2741)
            }, f400(v2, {
                let a2742 = v1;
                let a2743 = f498();
                let a2744 = 0u64;
                nat_quot(a2742, a2743, a2744)
            }))
        }
    }
}

pub fn f401(p2755: Bytes, p2756: u64) -> R<Bytes> {
    let m2754 = (p2755, p2756);
    {
        let mut m2754 = m2754;
        loop {
            let step = {
                let (v0, v1) = m2754;
                let m2745 = v1;
                if m2745 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2745 - 1;
                    let a2749 = f103(v0.clone(), {
                        let a2747 = {
                            let a2746 = v0.clone();
                            length_bytes(a2746)
                        };
                        let a2748 = 1u64;
                        nat_sub(a2747, a2748)
                    })?;
                    let a2750 = 48u64;
                    if nat_eq(a2749, a2750) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2752 = {
                                let a2751 = v0;
                                length_bytes(a2751)
                            };
                            let a2753 = 1u64;
                            nat_sub(a2752, a2753)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2754 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: u64) -> Bytes {
    let m2757 = v0;
    if m2757 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2757 - 1;
        let a2758 = Bytes::lit(&[0x30]);
        let a2759 = f402(v1);
        append_bytes(a2758, a2759)
    }
}

pub fn f403(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2804 = {
            let a2780 = {
                let a2768 = {
                    let a2762 = {
                        let a2760 = v3;
                        let a2761 = 128u8;
                        fixed_u8::bit_and(a2760, a2761)
                    };
                    let a2763 = 128u8;
                    if equal(a2762, a2763) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2769 = {
                    let a2766 = {
                        let a2764 = v3;
                        let a2765 = 64u8;
                        fixed_u8::bit_and(a2764, a2765)
                    };
                    let a2767 = 64u8;
                    if equal(a2766, a2767) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2768, a2769)?
            };
            let a2781 = {
                let a2778 = {
                    let a2772 = {
                        let a2770 = v3;
                        let a2771 = 32u8;
                        fixed_u8::bit_and(a2770, a2771)
                    };
                    let a2773 = 32u8;
                    if equal(a2772, a2773) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2779 = {
                    let a2776 = {
                        let a2774 = v3;
                        let a2775 = 16u8;
                        fixed_u8::bit_and(a2774, a2775)
                    };
                    let a2777 = 16u8;
                    if equal(a2776, a2777) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2778, a2779)?
            };
            nat_add(a2780, a2781)?
        };
        let a2805 = {
            let a2802 = {
                let a2790 = {
                    let a2784 = {
                        let a2782 = v3;
                        let a2783 = 8u8;
                        fixed_u8::bit_and(a2782, a2783)
                    };
                    let a2785 = 8u8;
                    if equal(a2784, a2785) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2791 = {
                    let a2788 = {
                        let a2786 = v3;
                        let a2787 = 4u8;
                        fixed_u8::bit_and(a2786, a2787)
                    };
                    let a2789 = 4u8;
                    if equal(a2788, a2789) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2790, a2791)?
            };
            let a2803 = {
                let a2800 = {
                    let a2794 = {
                        let a2792 = v3;
                        let a2793 = 2u8;
                        fixed_u8::bit_and(a2792, a2793)
                    };
                    let a2795 = 2u8;
                    if equal(a2794, a2795) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2801 = {
                    let a2798 = {
                        let a2796 = v3;
                        let a2797 = 1u8;
                        fixed_u8::bit_and(a2796, a2797)
                    };
                    let a2799 = 1u8;
                    if equal(a2798, a2799) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2800, a2801)?
            };
            nat_add(a2802, a2803)?
        };
        nat_add(a2804, a2805)?
    };
    let a2806 = v4;
    let a2807 = 34u64;
    if nat_eq(a2806, a2807) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2808 = v4;
        let a2809 = 92u64;
        if nat_eq(a2808, a2809) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2810 = v4;
            let a2811 = 8u64;
            if nat_eq(a2810, a2811) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2812 = v4;
                let a2813 = 12u64;
                if nat_eq(a2812, a2813) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2814 = v4;
                    let a2815 = 10u64;
                    if nat_eq(a2814, a2815) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2816 = v4;
                        let a2817 = 13u64;
                        if nat_eq(a2816, a2817) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2818 = v4;
                            let a2819 = 9u64;
                            if nat_eq(a2818, a2819) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2820 = v4;
                                let a2821 = 32u64;
                                if nat_lt(a2820, a2821) {
                                    let a2822 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2823 = f499(v2)?;
                                    Ok(append_bytes(a2822, a2823))
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

pub fn f404(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2824 = v0.clone();
    match m2824.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2827 = {
                let (h2825, _) = v2;
                h2825
            };
            let a2828 = {
                let (h2826, _) = v1.clone();
                h2826
            };
            if equal(a2827, a2828) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f405(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f500(f501(Fn16::F502, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2833 = {
        let (h2829, _) = f503(Fn17::F504, {
            let a2831 = {
                let a2830 = v0;
                length_list(a2830)
            };
            let a2832 = 1u64;
            nat_add(a2831, a2832)?
        }, v1)?;
        h2829
    };
    match m2833.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f406(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2838 = {
        let a2835 = f233({
            let (h2834, _) = v0;
            h2834
        })?;
        let a2836 = Bytes::lit(&[0x3a]);
        append_bytes(a2835, a2836)
    };
    let a2839 = {
        let (_, h2837) = v0;
        h2837
    };
    Ok(append_bytes(a2838, a2839))
}

pub fn f407(v0: Bytes, p2865: u64, p2866: u64, p2867: Bytes) -> R<Bytes> {
    let m2864 = (p2865, (p2866, p2867));
    {
        let mut m2864 = m2864;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2864;
                let m2840 = v2;
                if m2840 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2840 - 1;
                    let m2843 = {
                        let a2841 = v0.clone();
                        let a2842 = v1;
                        index_bytes(a2841, a2842)
                    };
                    match m2843 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f60(v5)?;
                            if f158({
                                let a2844 = v6;
                                let a2845 = 34u64;
                                nat_eq(a2844, a2845)
                            }, {
                                let a2846 = v6;
                                let a2847 = 92u64;
                                nat_eq(a2846, a2847)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a2848 = v1;
                                    let a2849 = 1u64;
                                    nat_add(a2848, a2849)?
                                }, (v4, {
                                    let a2852 = v3;
                                    let a2853 = {
                                        let a2850 = f8(Str::lit("\\"));
                                        let a2851 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a2850, a2851)
                                    };
                                    append_bytes(a2852, a2853)
                                })))
                            } else {
                                let a2854 = v6;
                                let a2855 = 10u64;
                                if nat_eq(a2854, a2855) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a2856 = v1;
                                        let a2857 = 1u64;
                                        nat_add(a2856, a2857)?
                                    }, (v4, {
                                        let a2858 = v3;
                                        let a2859 = f8(Str::lit("\\n"));
                                        append_bytes(a2858, a2859)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a2860 = v1;
                                        let a2861 = 1u64;
                                        nat_add(a2860, a2861)?
                                    }, (v4, {
                                        let a2862 = v3;
                                        let a2863 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a2862, a2863)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2864 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a2869 = {
                let a2868 = v0;
                length_bytes(a2868)
            };
            let a2870 = 1u64;
            nat_sub(a2869, a2870)
        })
    } else {
        v0
    }
}

pub fn f409(v0: Fn10, p2874: List<Bytes>, p2875: List<(Bytes, ())>) -> List<Bytes> {
    let m2873 = (p2874, p2875);
    {
        let mut m2873 = m2873;
        loop {
            let step = {
                let (v1, v2) = m2873;
                let m2871 = v2;
                match m2871.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c2872: Fn10 = v0.clone();
                            c2872.apply(v1, v3)
                        }, v4))
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

pub fn f410(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h2876, _) = v1;
        h2876
    }, v0)
}

pub fn f411(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f505(Fn18::F506, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f412(v0: Fn11, p2880: List<List<(Bytes, ())>>, p2881: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m2879 = (p2880, p2881);
    {
        let mut m2879 = m2879;
        loop {
            let step = {
                let (v1, v2) = m2879;
                let m2877 = v2;
                match m2877.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c2878: Fn11 = v0.clone();
                            c2878.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2879 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f414(v0: Fn12, p2886: u64, p2887: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m2885 = (p2886, p2887);
    {
        let mut m2885 = m2885;
        loop {
            let step = {
                let (v1, v2) = m2885;
                let m2882 = v1;
                if m2882 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2882 - 1;
                    let m2884 = {
                        let c2883: Fn12 = v0.clone();
                        c2883.apply(v2.clone())?
                    };
                    match m2884 {
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
                Ok(next) => { m2885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f415(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f507(v0)
}

pub fn f416(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f508(Fn19::F509, List::<(Bytes, ())>::nil(), v0)
}

pub fn f417(v0: Fn13, p2891: List<(Bytes, ())>, p2892: List<Bytes>) -> List<(Bytes, ())> {
    let m2890 = (p2891, p2892);
    {
        let mut m2890 = m2890;
        loop {
            let step = {
                let (v1, v2) = m2890;
                let m2888 = v2;
                match m2888.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c2889: Fn13 = v0.clone();
                            c2889.apply(v1, v3)
                        }, v4))
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

pub fn f418(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f419(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f510(v1, v0, v2)
}

pub fn f420(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2895 = {
        let a2893 = v0;
        let a2894 = v1;
        index_bytes(a2893, a2894)
    };
    match m2895 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f60(v2)?;
            if f105({
                let a2896 = 128u64;
                let a2897 = v3;
                nat_le(a2896, a2897)
            }, {
                let a2898 = v3;
                let a2899 = 191u64;
                nat_le(a2898, a2899)
            }) {
                Ok(Some({
                    let a2900 = v3;
                    let a2901 = 128u64;
                    nat_sub(a2900, a2901)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f421(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f105({
        let a2902 = v1;
        let a2903 = v0;
        nat_le(a2902, a2903)
    }, f105({
        let a2904 = v0;
        let a2905 = v2;
        nat_le(a2904, a2905)
    }, f158({
        let a2906 = v0;
        let a2907 = 55296u64;
        nat_lt(a2906, a2907)
    }, {
        let a2908 = 57343u64;
        let a2909 = v0;
        nat_lt(a2908, a2909)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f422(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f511(v0, v1)
}

pub fn f423(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f248(f185(v0, v1)?) {
        Ok(Some({
            let a2910 = v1;
            let a2911 = 1u64;
            nat_add(a2910, a2911)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f424(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f512(v0, v1)
}

pub fn f425(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2912 = f185(v0, v1)?;
    let a2913 = 32u64;
    if nat_eq(a2912, a2913) {
        Ok(Some({
            let a2914 = v1;
            let a2915 = 1u64;
            nat_add(a2914, a2915)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f426(v0: u64) -> bool {
    let a2916 = v0;
    let a2917 = 44u64;
    if if if nat_eq(a2916, a2917) {
        true
    } else {
        let a2918 = v0;
        let a2919 = 91u64;
        nat_eq(a2918, a2919)
    } {
        true
    } else {
        let a2920 = v0;
        let a2921 = 93u64;
        if nat_eq(a2920, a2921) {
            true
        } else {
            let a2922 = v0;
            let a2923 = 123u64;
            nat_eq(a2922, a2923)
        }
    } {
        true
    } else {
        let a2924 = v0;
        let a2925 = 125u64;
        nat_eq(a2924, a2925)
    }
}

pub fn f427(v0: u64) -> u64 {
    let a2926 = v0;
    let a2927 = 45u64;
    if nat_eq(a2926, a2927) {
        1u64
    } else {
        let a2928 = v0;
        let a2929 = 43u64;
        if nat_eq(a2928, a2929) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f428(v0: u64) -> u64 {
    let a2930 = 49u64;
    let a2931 = v0;
    if if nat_le(a2930, a2931) {
        let a2932 = v0;
        let a2933 = 57u64;
        nat_le(a2932, a2933)
    } else {
        false
    } {
        let a2934 = v0;
        let a2935 = 48u64;
        nat_sub(a2934, a2935)
    } else {
        0u64
    }
}

pub fn f429(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt19> {
    let m2936 = f513(v0.clone(), v1, v2)?;
    match m2936 {
        Adt26::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f514(v0.clone(), v8, v2)?;
            if f515(v0.clone(), v8, v2)? {
                let m2940 = f516(v0.clone(), v8, v2, v3, {
                    let a2938 = {
                        let a2937 = v0;
                        length_bytes(a2937)
                    };
                    let a2939 = 2u64;
                    nat_add(a2938, a2939)?
                }, Bytes::lit(&[]), v6)?;
                match m2940 {
                    Adt27::C0(v11, v12, v13) => {
                        Ok(Adt19::C0(Adt2::C6(f7(List::cons(v11, List::cons({
                            let a2941 = v4;
                            let a2942 = 1u64;
                            if nat_eq(a2941, a2942) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2943 = v4;
                            let a2944 = 2u64;
                            if nat_eq(a2943, a2944) {
                                f431(v12)
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
                    let a2945 = v4;
                    let a2946 = 2u64;
                    if nat_eq(a2945, a2946) {
                        f431(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2947 = f185(v0, v9)?;
                    let a2948 = 256u64;
                    if nat_eq(a2947, a2948) {
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

pub fn f430(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f517(v0, v1)
}

pub fn f431(v0: u64) -> Bytes {
    let m2949 = v0;
    if m2949 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2949 - 1;
        let a2950 = Bytes::lit(&[0x0a]);
        let a2951 = f431(v1);
        append_bytes(a2950, a2951)
    }
}

pub fn f432(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2952) = {
        let (h2953, _) = f218(Fn6::F518(v0.clone()), {
            let a2955 = {
                let a2954 = v0;
                length_bytes(a2954)
            };
            let a2956 = 1u64;
            nat_add(a2955, a2956)?
        }, (v1, 0u64))?;
        h2953
    };
    Ok(h2952)
}

pub fn f433(v0: u64, v1: u64) -> u64 {
    let a2957 = v0;
    let a2958 = v1;
    if nat_lt(a2957, a2958) {
        v1
    } else {
        v0
    }
}

pub fn f434(v0: Bytes, v1: u64, v2: u64) -> R<Adt19> {
    if f435(v0.clone()) {
        Ok(Adt19::C0(Adt2::C0, v2))
    } else {
        if f439(v0.clone()) {
            Ok(Adt19::C0(Adt2::C2(true), v2))
        } else {
            if f440(v0.clone()) {
                Ok(Adt19::C0(Adt2::C2(false), v2))
            } else {
                if f438(v0.clone()) {
                    Ok(Adt19::C1(f129(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f182(v0.clone(), 0u64, Str::lit("0o")) {
                        let a2960 = 2u64;
                        let a2961 = {
                            let a2959 = v0.clone();
                            length_bytes(a2959)
                        };
                        nat_lt(a2960, a2961)
                    } else {
                        false
                    } {
                        f519(v0, 8u64, v1, v2)
                    } else {
                        if if f182(v0.clone(), 0u64, Str::lit("0x")) {
                            let a2963 = 2u64;
                            let a2964 = {
                                let a2962 = v0.clone();
                                length_bytes(a2962)
                            };
                            nat_lt(a2963, a2964)
                        } else {
                            false
                        } {
                            f519(v0, 16u64, v1, v2)
                        } else {
                            let m2965 = f437(v0.clone())?;
                            match m2965 {
                                None => {
                                    Ok(Adt19::C0(Adt2::C6(v0), v2))
                                }
                                Some(v3) => {
                                    let m2966 = f106(v3.clone(), 0u64)?;
                                    match m2966 {
                                        None => {
                                            Ok(Adt19::C1(f520(v1)?))
                                        }
                                        Some(v4) => {
                                            let a2969 = {
                                                let (h2967, _) = v4;
                                                h2967
                                            };
                                            let a2970 = {
                                                let a2968 = v3;
                                                length_bytes(a2968)
                                            };
                                            if nat_eq(a2969, a2970) {
                                                Ok(Adt19::C0({
                                                    let (_, h2971) = v4;
                                                    h2971
                                                }, v2))
                                            } else {
                                                Ok(Adt19::C1(f520(v1)?))
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

pub fn f435(v0: Bytes) -> bool {
    let a2973 = {
        let a2972 = v0.clone();
        length_bytes(a2972)
    };
    let a2974 = 0u64;
    if nat_eq(a2973, a2974) {
        true
    } else {
        f269(v0, List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    }
}

pub fn f436(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2975 = v0.clone();
        length_bytes(a2975)
    };
    let v2: u64 = f185(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2976 = v2;
        let a2977 = 43u64;
        if if nat_eq(a2976, a2977) {
            true
        } else {
            let a2978 = v2;
            let a2979 = 45u64;
            nat_eq(a2978, a2979)
        } {
            1u64
        } else {
            0u64
        }
    };
    let a2980 = v3;
    let a2981 = v1;
    if if nat_lt(a2980, a2981) {
        let a2982 = f157(v0.clone(), v3)?;
        let a2983 = v1;
        nat_eq(a2982, a2983)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2984 = 2u64;
        let a2985 = v1;
        if nat_lt(a2984, a2985) {
            if if f182(v0.clone(), 0u64, Str::lit("0o")) {
                let a2987 = {
                    let (h2986, _) = f521(v0.clone(), 8u64, 2u64)?;
                    h2986
                };
                let a2988 = v1;
                nat_eq(a2987, a2988)
            } else {
                false
            } {
                Ok(true)
            } else {
                if f182(v0.clone(), 0u64, Str::lit("0x")) {
                    let a2990 = {
                        let (h2989, _) = f521(v0, 16u64, 2u64)?;
                        h2989
                    };
                    let a2991 = v1;
                    Ok(nat_eq(a2990, a2991))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f437(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2992 = v0.clone();
        length_bytes(a2992)
    };
    let v2: u64 = f185(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2993 = v2;
        let a2994 = 43u64;
        if if nat_eq(a2993, a2994) {
            true
        } else {
            let a2995 = v2;
            let a2996 = 45u64;
            nat_eq(a2995, a2996)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f157(v0.clone(), v3)?;
    let v5: bool = {
        let a2997 = f185(v0.clone(), v4)?;
        let a2998 = 46u64;
        nat_eq(a2997, a2998)
    };
    let v6: u64 = if v5 {
        let a2999 = v4;
        let a3000 = 1u64;
        nat_add(a2999, a3000)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f157(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f185(v0.clone(), v7)?;
    let v9: bool = {
        let a3001 = v8;
        let a3002 = 101u64;
        if nat_eq(a3001, a3002) {
            true
        } else {
            let a3003 = v8;
            let a3004 = 69u64;
            nat_eq(a3003, a3004)
        }
    };
    let v10: u64 = f185(v0.clone(), {
        let a3005 = v7;
        let a3006 = 1u64;
        nat_add(a3005, a3006)?
    })?;
    let v11: bool = if v9 {
        let a3007 = v10;
        let a3008 = 43u64;
        if nat_eq(a3007, a3008) {
            true
        } else {
            let a3009 = v10;
            let a3010 = 45u64;
            nat_eq(a3009, a3010)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a3011 = v7;
            let a3012 = 2u64;
            nat_add(a3011, a3012)?
        } else {
            let a3013 = v7;
            let a3014 = 1u64;
            nat_add(a3013, a3014)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f157(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f265(v0.clone(), v3, v4);
    let v15: u64 = f159(v14.clone())?;
    let a3015 = v13;
    let a3016 = v1;
    if if if nat_eq(a3015, a3016) {
        let a3017 = v3;
        let a3018 = v4;
        if nat_lt(a3017, a3018) {
            true
        } else {
            if v5 {
                let a3019 = v6;
                let a3020 = v7;
                nat_lt(a3019, a3020)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f187(v9) {
            true
        } else {
            let a3021 = v12;
            let a3022 = v13;
            nat_lt(a3021, a3022)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a3023 = v2;
            let a3024 = 45u64;
            if nat_eq(a3023, a3024) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a3026 = v15;
            let a3027 = {
                let a3025 = v14.clone();
                length_bytes(a3025)
            };
            if nat_eq(a3026, a3027) {
                Bytes::lit(&[0x30])
            } else {
                f265(v14.clone(), v15, {
                    let a3028 = v14;
                    length_bytes(a3028)
                })
            }
        }, List::cons({
            let a3029 = v6;
            let a3030 = v7;
            if nat_lt(a3029, a3030) {
                let a3031 = Bytes::lit(&[0x2e]);
                let a3032 = f265(v0.clone(), v6, v7);
                append_bytes(a3031, a3032)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a3035 = if if v11 {
                let a3033 = v10;
                let a3034 = 45u64;
                nat_eq(a3033, a3034)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a3036 = f265(v0, v12, v13);
            append_bytes(a3035, a3036)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f438(v0: Bytes) -> bool {
    f269(v0, List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil())))))))))))))
}

pub fn f439(v0: Bytes) -> bool {
    f269(v0, List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil()))))
}

pub fn f440(v0: Bytes) -> bool {
    f269(v0, List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil()))))
}

pub fn f441(v0: Bytes, p3095: u64, v2: u64, v3: bool, p3096: u64, p3097: Bytes) -> R<Adt23> {
    let m3094 = (p3095, (p3096, p3097));
    {
        let mut m3094 = m3094;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3094;
                let m3037 = v4;
                if m3037 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m3037 - 1;
                    let v7: u64 = f185(v0.clone(), v1)?;
                    let a3038 = v7;
                    let a3039 = 34u64;
                    if nat_eq(a3038, a3039) {
                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C0(v5, {
                            let a3040 = v1;
                            let a3041 = 1u64;
                            nat_add(a3040, a3041)?
                        })))
                    } else {
                        let a3042 = v7;
                        let a3043 = 256u64;
                        if nat_eq(a3042, a3043) {
                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a3044 = v7;
                            let a3045 = 92u64;
                            if nat_eq(a3044, a3045) {
                                let v8: u64 = f185(v0.clone(), {
                                    let a3046 = v1;
                                    let a3047 = 1u64;
                                    nat_add(a3046, a3047)?
                                })?;
                                let m3048 = f522(v8)?;
                                match m3048 {
                                    None => {
                                        let a3053 = v8;
                                        let a3054 = 10u64;
                                        if nat_eq(a3053, a3054) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f524(v1)?)))
                                            } else {
                                                let m3057 = f525(v0.clone(), {
                                                    let a3055 = v1;
                                                    let a3056 = 1u64;
                                                    nat_add(a3055, a3056)?
                                                }, v2, v6, 0u64)?;
                                                match m3057 {
                                                    Adt26::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt23>>((v12, (v6, {
                                                            let a3058 = v5;
                                                            let a3059 = f431(v11);
                                                            append_bytes(a3058, a3059)
                                                        })))
                                                    }
                                                    Adt26::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a3060 = v8;
                                                let a3061 = 120u64;
                                                if nat_eq(a3060, a3061) {
                                                    2u64
                                                } else {
                                                    let a3062 = v8;
                                                    let a3063 = 117u64;
                                                    if nat_eq(a3062, a3063) {
                                                        4u64
                                                    } else {
                                                        let a3064 = v8;
                                                        let a3065 = 85u64;
                                                        if nat_eq(a3064, a3065) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a3066 = v13;
                                            let a3067 = 0u64;
                                            if nat_eq(a3066, a3067) {
                                                Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m3070 = f526(v0.clone(), {
                                                    let a3068 = v1;
                                                    let a3069 = 2u64;
                                                    nat_add(a3068, a3069)?
                                                }, v13, 0u64)?;
                                                match m3070 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a3071 = 1114111u64;
                                                        let a3072 = v14;
                                                        if if nat_lt(a3071, a3072) {
                                                            true
                                                        } else {
                                                            let a3073 = 55296u64;
                                                            let a3074 = v14;
                                                            if nat_le(a3073, a3074) {
                                                                let a3075 = v14;
                                                                let a3076 = 57343u64;
                                                                nat_le(a3075, a3076)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt23>>(({
                                                                let a3079 = {
                                                                    let a3077 = v1;
                                                                    let a3078 = 2u64;
                                                                    nat_add(a3077, a3078)?
                                                                };
                                                                let a3080 = v13;
                                                                nat_add(a3079, a3080)?
                                                            }, (v6, {
                                                                let a3081 = v5;
                                                                let a3082 = f523(v14)?;
                                                                append_bytes(a3081, a3082)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt23>>(({
                                            let a3049 = v1;
                                            let a3050 = 2u64;
                                            nat_add(a3049, a3050)?
                                        }, (v6, {
                                            let a3051 = v5;
                                            let a3052 = f523(v9)?;
                                            append_bytes(a3051, a3052)
                                        })))
                                    }
                                }
                            } else {
                                if if f248(v7) {
                                    true
                                } else {
                                    let a3083 = v7;
                                    let a3084 = 10u64;
                                    nat_eq(a3083, a3084)
                                } {
                                    let v15: u64 = f251(v0.clone(), v1)?;
                                    let a3085 = f185(v0.clone(), v15)?;
                                    let a3086 = 10u64;
                                    if nat_eq(a3085, a3086) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f524(v1)?)))
                                        } else {
                                            let m3087 = f525(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m3087 {
                                                Adt26::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt23>>((v18, (v6, {
                                                        let a3088 = v5;
                                                        let a3089 = f449(v17);
                                                        append_bytes(a3088, a3089)
                                                    })))
                                                }
                                                Adt26::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt23>>((v15, (v6, {
                                            let a3090 = v5;
                                            let a3091 = f265(v0.clone(), v1, v15);
                                            append_bytes(a3090, a3091)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f527(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt23>>((v19, (v6, {
                                        let a3092 = v5;
                                        let a3093 = f265(v0.clone(), v1, v19);
                                        append_bytes(a3092, a3093)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3094 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: Bytes, p3125: u64, v2: u64, v3: bool, p3126: u64, p3127: Bytes) -> R<Adt23> {
    let m3124 = (p3125, (p3126, p3127));
    {
        let mut m3124 = m3124;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3124;
                let m3098 = v4;
                if m3098 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m3098 - 1;
                    let v7: u64 = f185(v0.clone(), v1)?;
                    let a3099 = v7;
                    let a3100 = 39u64;
                    if nat_eq(a3099, a3100) {
                        let a3103 = f185(v0.clone(), {
                            let a3101 = v1;
                            let a3102 = 1u64;
                            nat_add(a3101, a3102)?
                        })?;
                        let a3104 = 39u64;
                        if nat_eq(a3103, a3104) {
                            Ok::<(u64, (u64, Bytes)), R<Adt23>>(({
                                let a3105 = v1;
                                let a3106 = 2u64;
                                nat_add(a3105, a3106)?
                            }, (v6, {
                                let a3107 = v5;
                                let a3108 = Bytes::lit(&[0x27]);
                                append_bytes(a3107, a3108)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C0(v5, {
                                let a3109 = v1;
                                let a3110 = 1u64;
                                nat_add(a3109, a3110)?
                            })))
                        }
                    } else {
                        let a3111 = v7;
                        let a3112 = 256u64;
                        if nat_eq(a3111, a3112) {
                            Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f129(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f248(v7) {
                                true
                            } else {
                                let a3113 = v7;
                                let a3114 = 10u64;
                                nat_eq(a3113, a3114)
                            } {
                                let v8: u64 = f251(v0.clone(), v1)?;
                                let a3115 = f185(v0.clone(), v8)?;
                                let a3116 = 10u64;
                                if nat_eq(a3115, a3116) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(f524(v1)?)))
                                    } else {
                                        let m3117 = f525(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m3117 {
                                            Adt26::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt23>>((v11, (v6, {
                                                    let a3118 = v5;
                                                    let a3119 = f449(v10);
                                                    append_bytes(a3118, a3119)
                                                })))
                                            }
                                            Adt26::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt23>>(Ok(Adt23::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt23>>((v8, (v6, {
                                        let a3120 = v5;
                                        let a3121 = f265(v0.clone(), v1, v8);
                                        append_bytes(a3120, a3121)
                                    })))
                                }
                            } else {
                                let v12: u64 = f528(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt23>>((v12, (v6, {
                                    let a3122 = v5;
                                    let a3123 = f265(v0.clone(), v1, v12);
                                    append_bytes(a3122, a3123)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3124 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h3128, _) = f529(Fn20::F530(v2, v0.clone()), {
            let a3130 = {
                let a3129 = v0.clone();
                length_bytes(a3129)
            };
            let a3131 = 1u64;
            nat_add(a3130, a3131)?
        }, (v1, false))?;
        h3128
    };
    let (_, h3132) = v3;
    if h3132 {
        if f183(v0, {
            let (h3133, _) = v3;
            h3133
        })? {
            Ok(Err::<u64, Bytes>(f129(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h3134, _) = v3;
                h3134
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f129(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h3135, _) = v3;
                h3135
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h3136, _) = v3;
            h3136
        }))
    }
}

pub fn f444(v0: Bytes, v1: u64, v2: u64) -> R<Adt23> {
    let a3137 = f185(v0.clone(), v1)?;
    let a3138 = 33u64;
    if nat_eq(a3137, a3138) {
        let m3139 = f266(v0.clone(), v1)?;
        match m3139 {
            Ok(v4) => {
                let m3140 = f443(v0.clone(), v4, v2)?;
                match m3140 {
                    Ok(v6) => {
                        let a3141 = f185(v0.clone(), v6)?;
                        let a3142 = 58u64;
                        if nat_eq(a3141, a3142) {
                            let a3145 = f261(v0, v1)?;
                            let a3146 = {
                                let a3143 = v1;
                                let a3144 = 1u64;
                                nat_add(a3143, a3144)?
                            };
                            if nat_eq(a3145, a3146) {
                                Ok(Adt23::C1(f129(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                            } else {
                                Ok(Adt23::C0(Bytes::lit(&[]), v6))
                            }
                        } else {
                            f531(v0, v6, v2)
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
        f531(v0, v1, v2)
    }
}

pub fn f445(v0: u64) -> R<bool> {
    let a3147 = f264(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a3148 = 18u64;
    if nat_lt(a3147, a3148) {
        Ok(true)
    } else {
        let a3149 = v0;
        let a3150 = 37u64;
        Ok(nat_eq(a3149, a3150))
    }
}

pub fn f446(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f532(v1, v0, v2)
}

pub fn f447(v0: u64) -> R<Bytes> {
    f129(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f448(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h3151, _) = f218(Fn6::F533(v0.clone()), {
        let a3153 = {
            let a3152 = v0;
            length_bytes(a3152)
        };
        let a3154 = 1u64;
        nat_add(a3153, a3154)?
    }, (v1, 0u64))?;
    Ok(h3151)
}

pub fn f449(v0: u64) -> Bytes {
    let a3155 = v0;
    let a3156 = 0u64;
    if nat_eq(a3155, a3156) {
        Bytes::lit(&[0x20])
    } else {
        f431(v0)
    }
}

pub fn f450(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    f534(v0, v1, v2)
}

pub fn f451(v0: Bytes, p3161: List<Bytes>, p3162: u64) -> R<u64> {
    let m3160 = (p3161, p3162);
    {
        let mut m3160 = m3160;
        loop {
            let step = {
                let (v1, v2) = m3160;
                let m3157 = v1;
                match m3157.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<u64>>(Ok(0u64))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(List<Bytes>, u64), R<u64>>((v4, {
                                let a3158 = v2;
                                let a3159 = 1u64;
                                nat_add(a3158, a3159)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3160 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3164 = v2;
    let a3165 = {
        let a3163 = v0.clone();
        length_bytes(a3163)
    };
    if if nat_lt(a3164, a3165) {
        f187({
            let a3166 = f185(v0, v2)?;
            let a3167 = v1;
            nat_eq(a3166, a3167)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3168 = v2;
            let a3169 = 1u64;
            nat_add(a3168, a3169)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f453(v0: u64, v1: Bytes) -> R<bool> {
    f535(v0, v1.clone(), 0u64, {
        let a3171 = {
            let a3170 = v1;
            length_bytes(a3170)
        };
        let a3172 = 3u64;
        let a3173 = 0u64;
        nat_quot(a3171, a3172, a3173)
    }, true)
}

pub fn f454() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f455(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f536(v1, v0, v2)
}

pub fn f456(v0: u64) -> bool {
    if f457(v0) {
        true
    } else {
        let a3174 = 65u64;
        let a3175 = v0;
        if if nat_le(a3174, a3175) {
            let a3176 = v0;
            let a3177 = 70u64;
            nat_le(a3176, a3177)
        } else {
            false
        } {
            true
        } else {
            let a3178 = 97u64;
            let a3179 = v0;
            if nat_le(a3178, a3179) {
                let a3180 = v0;
                let a3181 = 102u64;
                nat_le(a3180, a3181)
            } else {
                false
            }
        }
    }
}

pub fn f457(v0: u64) -> bool {
    let a3182 = 48u64;
    let a3183 = v0;
    if nat_le(a3182, a3183) {
        let a3184 = v0;
        let a3185 = 57u64;
        nat_le(a3184, a3185)
    } else {
        false
    }
}

pub fn f458(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f330(v0.clone(), v1)?;
    let a3186 = v2;
    let a3187 = 69u64;
    if if nat_eq(a3186, a3187) {
        true
    } else {
        let a3188 = v2;
        let a3189 = 101u64;
        nat_eq(a3188, a3189)
    } {
        let v3: u64 = f330(v0.clone(), {
            let a3190 = v1;
            let a3191 = 1u64;
            nat_add(a3190, a3191)?
        })?;
        let v4: u64 = {
            let a3192 = v3;
            let a3193 = 43u64;
            if if nat_eq(a3192, a3193) {
                true
            } else {
                let a3194 = v3;
                let a3195 = 45u64;
                nat_eq(a3194, a3195)
            } {
                let a3196 = v1;
                let a3197 = 2u64;
                nat_add(a3196, a3197)?
            } else {
                let a3198 = v1;
                let a3199 = 1u64;
                nat_add(a3198, a3199)?
            }
        };
        let v5: u64 = f356(v0, 0u64, v4)?;
        let a3200 = v4;
        let a3201 = v5;
        if nat_lt(a3200, a3201) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f459(v0: Adt20, v1: Bytes) -> bool {
    let a3202 = f460(v0.clone());
    let a3203 = 3u64;
    if !nat_eq(a3202, a3203) {
        f32(f461(v0), v1)
    } else {
        false
    }
}

pub fn f460(v0: Adt20) -> u64 {
    let m3204 = v0;
    match m3204 {
        Adt20::C0(v1, _) => {
            v1
        }
    }
}

pub fn f461(v0: Adt20) -> Bytes {
    let m3205 = v0;
    match m3205 {
        Adt20::C0(_, v2) => {
            v2
        }
    }
}

pub fn f462(v0: Bytes) -> Bytes {
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

pub fn f463(v0: Bytes) -> bool {
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

pub fn f464(p3214: u64, p3215: List<Adt20>, p3216: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt20>), Bytes>> {
    let m3213 = (p3214, (p3215, p3216));
    {
        let mut m3213 = m3213;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3213;
                let m3206 = v0;
                if m3206 == 0 {
                    Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m3206 - 1;
                    if f359(v1.clone(), Str::lit("}")) {
                        let m3207 = f366(f361(v1), Str::lit(";"));
                        match m3207 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt20>), Bytes>((f537(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3208 = f273(v1)?;
                        match m3208 {
                            Ok(v7) => {
                                let v8: bool = f359(v7.clone(), Str::lit("required"));
                                let m3209 = f371(v3, if v8 {
                                    f361(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m3209 {
                                    Ok(v10) => {
                                        let m3210 = f365(f372(v10.clone()));
                                        match m3210 {
                                            Ok(v12) => {
                                                if if f359(f367(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m3211 = f466(f361(f367(v12.clone())))?;
                                                    match m3211 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>((v3, (v14, List::cons(Adt14::C0(f369(v12), f475(f373(v10))?, 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m3212 = f366(f367(v12.clone()), Str::lit(";"));
                                                    match m3212 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>((v3, (v16, List::cons(Adt14::C0(f369(v12), f475(f373(v10))?, if v8 {
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
                Ok(next) => { m3213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(p3225: u64, p3226: List<Adt20>, p3227: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt20>), Bytes>> {
    let m3224 = (p3225, (p3226, p3227));
    {
        let mut m3224 = m3224;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3224;
                let m3217 = v0;
                if m3217 == 0 {
                    Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m3217 - 1;
                    if f359(v1.clone(), Str::lit("}")) {
                        let m3218 = f366(f361(v1), Str::lit(";"));
                        match m3218 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt20>), Bytes>((f537(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt20>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3219 = f273(v1)?;
                        match m3219 {
                            Ok(v7) => {
                                let v8: List<Adt20> = if if f359(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f359(v7.clone(), Str::lit("inherit"))
                                } {
                                    f361(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt20> = if f359(v8.clone(), Str::lit("readonly")) {
                                    f361(v8)
                                } else {
                                    v8
                                };
                                if f359(v9.clone(), Str::lit("attribute")) {
                                    let m3220 = f371(v3, f361(v9), 0u64)?;
                                    match m3220 {
                                        Ok(v11) => {
                                            let m3221 = f365(f372(v11.clone()));
                                            match m3221 {
                                                Ok(v13) => {
                                                    let m3222 = f366(f367(v13.clone()), Str::lit(";"));
                                                    match m3222 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt20>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt20>), Bytes>>>((v3, (v15, List::cons(Adt14::C0(f369(v13), f475(f373(v11))?, 0u64), v2))))
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
                                    let m3223 = f466(v7)?;
                                    match m3223 {
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
                Ok(next) => { m3224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: List<Adt20>) -> R<Result<List<Adt20>, Bytes>> {
    Ok(f360({
        let a3229 = {
            let a3228 = v0.clone();
            length_list(a3228)
        };
        let a3230 = 1u64;
        nat_add(a3229, a3230)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f467(v0: List<Adt20>) -> Bytes {
    let m3231 = v0;
    match m3231.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f461(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f468(v0: Bytes) -> Bytes {
    if f53(v0.clone(), f8(Str::lit("_"))) {
        f46(v0.clone(), 1u64, {
            let a3233 = {
                let a3232 = v0;
                length_bytes(a3232)
            };
            let a3234 = 1u64;
            nat_sub(a3233, a3234)
        })
    } else {
        v0
    }
}

pub fn f469(v0: Adt15, v1: Bytes) -> Adt15 {
    Adt15::C0(f290(v0.clone()), f291(v0.clone()), f292(v0.clone()), f289(v0.clone()), v1, f201(v0))
}

pub fn f470(v0: Str, v1: Adt15) -> R<Adt15> {
    let v2: Adt15 = f475(v1)?;
    Ok(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a3235 = v0;
        utf8_encode(a3235)
    }, List::cons(f8(Str::lit(" ")), List::cons(f291(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f292(v2.clone()), f289(v2.clone()), Bytes::lit(&[]), f201(v2)))
}

pub fn f471(v0: Adt15) -> Bytes {
    let m3236 = v0;
    match m3236 {
        Adt15::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f472(v0: Adt15, v1: List<Adt20>) -> R<Result<Adt25, Bytes>> {
    if f359(v1.clone(), Str::lit("?")) {
        Ok(Ok::<Adt25, Bytes>(Adt25::C0(f469(f470(Str::lit("Option"), v0.clone())?, f7(List::cons(f471(v0), List::cons(f8(Str::lit("OrNull")), List::<Bytes>::nil())))), f361(v1))))
    } else {
        Ok(Ok::<Adt25, Bytes>(Adt25::C0(v0, v1)))
    }
}

pub fn f473(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]), List::<Adt16>::nil())
}

pub fn f474(v0: Bytes) -> Option<Adt15> {
    f538(v0, f539())
}

pub fn f475(v0: Adt15) -> R<Adt15> {
    let a3237 = f290(v0.clone());
    let a3238 = 0u64;
    if nat_eq(a3237, a3238) {
        Ok(v0)
    } else {
        if !f32(f289(v0.clone()), Bytes::lit(&[])) {
            Ok(Adt15::C0(0u64, Bytes::lit(&[]), f292(v0.clone()), f289(v0), Bytes::lit(&[]), List::<Adt16>::nil()))
        } else {
            let v1: Bytes = f276(f471(v0.clone()))?;
            Ok(Adt15::C0(0u64, v1.clone(), List::cons(v1.clone(), f292(v0.clone())), Bytes::lit(&[]), f471(v0.clone()), List::cons(Adt16::C0(v1.clone(), Bytes::lit(&[]), f288(v1, List::cons(f291(v0.clone()), List::<Bytes>::nil())), f292(v0.clone())), f201(v0))))
        }
    }
}

pub fn f476(v0: bool) -> Adt15 {
    let v1: Bytes = if v0 {
        f140()
    } else {
        f138()
    };
    Adt15::C0(0u64, v1.clone(), List::cons(v1.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v1, List::<Adt16>::nil())
}

pub fn f477(p3255: u64, p3256: u64, p3257: u64, p3258: Bytes) -> Bytes {
    let m3254 = (p3255, (p3256, (p3257, p3258)));
    {
        let mut m3254 = m3254;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3254;
                let m3239 = v0;
                if m3239 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3239 - 1;
                    if f105({
                        let a3240 = v1;
                        let a3241 = 0u64;
                        nat_eq(a3240, a3241)
                    }, {
                        let a3242 = v2;
                        let a3243 = 0u64;
                        nat_eq(a3242, a3243)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3244 = v1;
                            let a3245 = 16u64;
                            let a3246 = 0u64;
                            nat_quot(a3244, a3245, a3246)
                        }, ({
                            let a3247 = v2;
                            let a3248 = 1u64;
                            nat_sub(a3247, a3248)
                        }, {
                            let a3252 = f540({
                                let a3249 = v1;
                                let a3250 = 16u64;
                                let a3251 = 0u64;
                                nat_rem(a3249, a3250, a3251)
                            });
                            let a3253 = v3;
                            append_bytes(a3252, a3253)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3254 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(v0: Adt14, v1: List<Adt14>) -> R<List<Adt14>> {
    let m3259 = v1;
    match m3259.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            if f541({
                let a3261 = {
                    let a3260 = f380(v0.clone());
                    length_bytes(a3260)
                };
                let a3262 = 1u64;
                nat_add(a3261, a3262)?
            }, f380(v0.clone()), f380(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f478(v0, v3)?))
            }
        }
    }
}

pub fn f479(v0: List<Adt13>, v1: Bytes) -> List<Adt14> {
    let m3263 = f209(v0.clone(), 1u64, v1.clone());
    match m3263 {
        None => {
            f378(v0, 6u64, v1)
        }
        Some(v2) => {
            f377(f200(v2), f378(v0, 6u64, v1))
        }
    }
}

pub fn f480(v0: Bytes) -> R<Option<Bytes>> {
    let m3265 = f542({
        let a3264 = v0.clone();
        length_bytes(a3264)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3265 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3267 = {
                let a3266 = v1.clone();
                length_bytes(a3266)
            };
            let a3268 = 0u64;
            if nat_eq(a3267, a3268) {
                let a3270 = {
                    let a3269 = v0;
                    length_bytes(a3269)
                };
                let a3271 = 0u64;
                if nat_eq(a3270, a3271) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f482(v1)?))
            }
        }
    }
}

pub fn f481(v0: Bytes) -> R<Option<Bytes>> {
    if f543({
        let a3272 = v0.clone();
        length_bytes(a3272)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f544(v0)?;
        if f545(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3275 = {
                let a3273 = v1.clone();
                let a3274 = 0u64;
                index_bytes(a3273, a3274)
            };
            match m3275 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f229(f60(v2)?) {
                        Ok(Some({
                            let a3276 = f8(Str::lit("n"));
                            let a3277 = v1;
                            append_bytes(a3276, a3277)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f546({
            let a3278 = v0.clone();
            length_bytes(a3278)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f482(v0: Bytes) -> R<Bytes> {
    if f114(v0.clone(), f547()) {
        f375(v0)
    } else {
        Ok(v0)
    }
}

pub fn f483(v0: Fn14, p3283: u64, p3284: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3282 = (p3283, p3284);
    {
        let mut m3282 = m3282;
        loop {
            let step = {
                let (v1, v2) = m3282;
                let m3279 = v1;
                if m3279 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3279 - 1;
                    let m3281 = {
                        let c3280: Fn14 = v0.clone();
                        c3280.apply(v2.clone())?
                    };
                    match m3281 {
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
                Ok(next) => { m3282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f548(v0, v1)
}

pub fn f485(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3329 = {
            let a3305 = {
                let a3293 = {
                    let a3287 = {
                        let a3285 = v1;
                        let a3286 = 128u8;
                        fixed_u8::bit_and(a3285, a3286)
                    };
                    let a3288 = 128u8;
                    if equal(a3287, a3288) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3294 = {
                    let a3291 = {
                        let a3289 = v1;
                        let a3290 = 64u8;
                        fixed_u8::bit_and(a3289, a3290)
                    };
                    let a3292 = 64u8;
                    if equal(a3291, a3292) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3293, a3294)?
            };
            let a3306 = {
                let a3303 = {
                    let a3297 = {
                        let a3295 = v1;
                        let a3296 = 32u8;
                        fixed_u8::bit_and(a3295, a3296)
                    };
                    let a3298 = 32u8;
                    if equal(a3297, a3298) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3304 = {
                    let a3301 = {
                        let a3299 = v1;
                        let a3300 = 16u8;
                        fixed_u8::bit_and(a3299, a3300)
                    };
                    let a3302 = 16u8;
                    if equal(a3301, a3302) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3303, a3304)?
            };
            nat_add(a3305, a3306)?
        };
        let a3330 = {
            let a3327 = {
                let a3315 = {
                    let a3309 = {
                        let a3307 = v1;
                        let a3308 = 8u8;
                        fixed_u8::bit_and(a3307, a3308)
                    };
                    let a3310 = 8u8;
                    if equal(a3309, a3310) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3316 = {
                    let a3313 = {
                        let a3311 = v1;
                        let a3312 = 4u8;
                        fixed_u8::bit_and(a3311, a3312)
                    };
                    let a3314 = 4u8;
                    if equal(a3313, a3314) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3315, a3316)?
            };
            let a3328 = {
                let a3325 = {
                    let a3319 = {
                        let a3317 = v1;
                        let a3318 = 2u8;
                        fixed_u8::bit_and(a3317, a3318)
                    };
                    let a3320 = 2u8;
                    if equal(a3319, a3320) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3326 = {
                    let a3323 = {
                        let a3321 = v1;
                        let a3322 = 1u8;
                        fixed_u8::bit_and(a3321, a3322)
                    };
                    let a3324 = 1u8;
                    if equal(a3323, a3324) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3325, a3326)?
            };
            nat_add(a3327, a3328)?
        };
        nat_add(a3329, a3330)?
    };
    let a3331 = 48u64;
    let a3332 = v2;
    if nat_le(a3331, a3332) {
        let a3333 = v2;
        let a3334 = 57u64;
        Ok(nat_le(a3333, a3334))
    } else {
        Ok(false)
    }
}

pub fn f486(v0: u64) -> u32 {
    let a3335 = v0;
    let a3336 = 32u64;
    if nat_lt(a3335, a3336) {
        let a3337 = v0;
        let a3338 = 16u64;
        if nat_lt(a3337, a3338) {
            let a3339 = v0;
            let a3340 = 8u64;
            if nat_lt(a3339, a3340) {
                let a3341 = v0;
                let a3342 = 4u64;
                if nat_lt(a3341, a3342) {
                    let a3343 = v0;
                    let a3344 = 2u64;
                    if nat_lt(a3343, a3344) {
                        let a3345 = v0;
                        let a3346 = 1u64;
                        if nat_lt(a3345, a3346) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3347 = v0;
                        let a3348 = 3u64;
                        if nat_lt(a3347, a3348) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3349 = v0;
                    let a3350 = 6u64;
                    if nat_lt(a3349, a3350) {
                        let a3351 = v0;
                        let a3352 = 5u64;
                        if nat_lt(a3351, a3352) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3353 = v0;
                        let a3354 = 7u64;
                        if nat_lt(a3353, a3354) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3355 = v0;
                let a3356 = 12u64;
                if nat_lt(a3355, a3356) {
                    let a3357 = v0;
                    let a3358 = 10u64;
                    if nat_lt(a3357, a3358) {
                        let a3359 = v0;
                        let a3360 = 9u64;
                        if nat_lt(a3359, a3360) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3361 = v0;
                        let a3362 = 11u64;
                        if nat_lt(a3361, a3362) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3363 = v0;
                    let a3364 = 14u64;
                    if nat_lt(a3363, a3364) {
                        let a3365 = v0;
                        let a3366 = 13u64;
                        if nat_lt(a3365, a3366) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3367 = v0;
                        let a3368 = 15u64;
                        if nat_lt(a3367, a3368) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3369 = v0;
            let a3370 = 24u64;
            if nat_lt(a3369, a3370) {
                let a3371 = v0;
                let a3372 = 20u64;
                if nat_lt(a3371, a3372) {
                    let a3373 = v0;
                    let a3374 = 18u64;
                    if nat_lt(a3373, a3374) {
                        let a3375 = v0;
                        let a3376 = 17u64;
                        if nat_lt(a3375, a3376) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3377 = v0;
                        let a3378 = 19u64;
                        if nat_lt(a3377, a3378) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3379 = v0;
                    let a3380 = 22u64;
                    if nat_lt(a3379, a3380) {
                        let a3381 = v0;
                        let a3382 = 21u64;
                        if nat_lt(a3381, a3382) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3383 = v0;
                        let a3384 = 23u64;
                        if nat_lt(a3383, a3384) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3385 = v0;
                let a3386 = 28u64;
                if nat_lt(a3385, a3386) {
                    let a3387 = v0;
                    let a3388 = 26u64;
                    if nat_lt(a3387, a3388) {
                        let a3389 = v0;
                        let a3390 = 25u64;
                        if nat_lt(a3389, a3390) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3391 = v0;
                        let a3392 = 27u64;
                        if nat_lt(a3391, a3392) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3393 = v0;
                    let a3394 = 30u64;
                    if nat_lt(a3393, a3394) {
                        let a3395 = v0;
                        let a3396 = 29u64;
                        if nat_lt(a3395, a3396) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3397 = v0;
                        let a3398 = 31u64;
                        if nat_lt(a3397, a3398) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3399 = v0;
        let a3400 = 48u64;
        if nat_lt(a3399, a3400) {
            let a3401 = v0;
            let a3402 = 40u64;
            if nat_lt(a3401, a3402) {
                let a3403 = v0;
                let a3404 = 36u64;
                if nat_lt(a3403, a3404) {
                    let a3405 = v0;
                    let a3406 = 34u64;
                    if nat_lt(a3405, a3406) {
                        let a3407 = v0;
                        let a3408 = 33u64;
                        if nat_lt(a3407, a3408) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3409 = v0;
                        let a3410 = 35u64;
                        if nat_lt(a3409, a3410) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3411 = v0;
                    let a3412 = 38u64;
                    if nat_lt(a3411, a3412) {
                        let a3413 = v0;
                        let a3414 = 37u64;
                        if nat_lt(a3413, a3414) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3415 = v0;
                        let a3416 = 39u64;
                        if nat_lt(a3415, a3416) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3417 = v0;
                let a3418 = 44u64;
                if nat_lt(a3417, a3418) {
                    let a3419 = v0;
                    let a3420 = 42u64;
                    if nat_lt(a3419, a3420) {
                        let a3421 = v0;
                        let a3422 = 41u64;
                        if nat_lt(a3421, a3422) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3423 = v0;
                        let a3424 = 43u64;
                        if nat_lt(a3423, a3424) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3425 = v0;
                    let a3426 = 46u64;
                    if nat_lt(a3425, a3426) {
                        let a3427 = v0;
                        let a3428 = 45u64;
                        if nat_lt(a3427, a3428) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3429 = v0;
                        let a3430 = 47u64;
                        if nat_lt(a3429, a3430) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3431 = v0;
            let a3432 = 56u64;
            if nat_lt(a3431, a3432) {
                let a3433 = v0;
                let a3434 = 52u64;
                if nat_lt(a3433, a3434) {
                    let a3435 = v0;
                    let a3436 = 50u64;
                    if nat_lt(a3435, a3436) {
                        let a3437 = v0;
                        let a3438 = 49u64;
                        if nat_lt(a3437, a3438) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3439 = v0;
                        let a3440 = 51u64;
                        if nat_lt(a3439, a3440) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3441 = v0;
                    let a3442 = 54u64;
                    if nat_lt(a3441, a3442) {
                        let a3443 = v0;
                        let a3444 = 53u64;
                        if nat_lt(a3443, a3444) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3445 = v0;
                        let a3446 = 55u64;
                        if nat_lt(a3445, a3446) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3447 = v0;
                let a3448 = 60u64;
                if nat_lt(a3447, a3448) {
                    let a3449 = v0;
                    let a3450 = 58u64;
                    if nat_lt(a3449, a3450) {
                        let a3451 = v0;
                        let a3452 = 57u64;
                        if nat_lt(a3451, a3452) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3453 = v0;
                        let a3454 = 59u64;
                        if nat_lt(a3453, a3454) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3455 = v0;
                    let a3456 = 62u64;
                    if nat_lt(a3455, a3456) {
                        let a3457 = v0;
                        let a3458 = 61u64;
                        if nat_lt(a3457, a3458) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3459 = v0;
                        let a3460 = 63u64;
                        if nat_lt(a3459, a3460) {
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

pub fn f487(v0: u32) -> u32 {
    let a3463 = {
        let a3461 = f549(v0, 6u32, 26u32);
        let a3462 = f549(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3461, a3462)
    };
    let a3464 = f549(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3463, a3464)
}

pub fn f488(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3470 = {
        let a3465 = v0;
        let a3466 = v1;
        fixed_u32::bit_and(a3465, a3466)
    };
    let a3471 = {
        let a3468 = {
            let a3467 = v0;
            fixed_u32::bit_not(a3467)
        };
        let a3469 = v2;
        fixed_u32::bit_and(a3468, a3469)
    };
    fixed_u32::bit_xor(a3470, a3471)
}

pub fn f489(v0: u32) -> u32 {
    let a3474 = {
        let a3472 = f549(v0, 2u32, 30u32);
        let a3473 = f549(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3472, a3473)
    };
    let a3475 = f549(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3474, a3475)
}

pub fn f490(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3484 = {
        let a3480 = {
            let a3476 = v0;
            let a3477 = v1;
            fixed_u32::bit_and(a3476, a3477)
        };
        let a3481 = {
            let a3478 = v0;
            let a3479 = v2;
            fixed_u32::bit_and(a3478, a3479)
        };
        fixed_u32::bit_xor(a3480, a3481)
    };
    let a3485 = {
        let a3482 = v1;
        let a3483 = v2;
        fixed_u32::bit_and(a3482, a3483)
    };
    fixed_u32::bit_xor(a3484, a3485)
}

pub fn f491(v0: Adt21) -> Adt21 {
    Adt21::C0(match v0.clone() {
        Adt21::C0(_, h3486, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3486
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h3487, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3487
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, h3488, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3488
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, h3489, _, _, _, _, _, _, _, _, _, _, _) => {
            h3489
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, h3490, _, _, _, _, _, _, _, _, _, _) => {
            h3490
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h3491, _, _, _, _, _, _, _, _, _) => {
            h3491
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, h3492, _, _, _, _, _, _, _, _) => {
            h3492
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, h3493, _, _, _, _, _, _, _) => {
            h3493
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, h3494, _, _, _, _, _, _) => {
            h3494
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, h3495, _, _, _, _, _) => {
            h3495
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, h3496, _, _, _, _) => {
            h3496
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3497, _, _, _) => {
            h3497
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3498, _, _) => {
            h3498
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3499, _) => {
            h3499
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3500) => {
            h3500
        }
    }, f306(f550(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3501, _) => {
            h3501
        }
    }), f306(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, _, _, h3502, _, _, _, _, _, _) => {
            h3502
        }
    }, f306(f551(match v0.clone() {
        Adt21::C0(_, h3503, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3503
        }
    }), match v0 {
        Adt21::C0(h3504, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3504
        }
    }))))
}

pub fn f492(v0: Fn15, p3508: u64, p3509: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3507 = (p3508, p3509);
    {
        let mut m3507 = m3507;
        loop {
            let step = {
                let (v1, v2) = m3507;
                let m3505 = v1;
                if m3505 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3505 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3506: Fn15 = v0.clone();
                        c3506.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3510, _) = v0;
        h3510
    };
    let v2: u32 = {
        let (h3511, _) = {
            let (_, h3512) = v0;
            h3512
        };
        h3511
    };
    let v3: u32 = {
        let (_, h3513) = {
            let (_, h3514) = v0;
            h3514
        };
        h3513
    };
    ({
        let a3515 = v1;
        let a3516 = 2u64;
        let a3517 = 0u64;
        nat_quot(a3515, a3516, a3517)
    }, (f494(v2, 1u32), {
        let a3521 = {
            let a3518 = v1;
            let a3519 = 2u64;
            let a3520 = 0u64;
            nat_rem(a3518, a3519, a3520)
        };
        let a3522 = 1u64;
        if nat_eq(a3521, a3522) {
            let a3523 = v3;
            let a3524 = v2;
            fixed_u32::bit_or(a3523, a3524)
        } else {
            v3
        }
    }))
}

pub fn f494(v0: u32, v1: u32) -> u32 {
    let m3527 = {
        let a3525 = v0;
        let a3526 = v1;
        fixed_u32::shift_left(a3525, a3526)
    };
    match m3527 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f495(p3530: List<u64>, p3531: List<u64>) -> List<u64> {
    let m3529 = (p3530, p3531);
    {
        let mut m3529 = m3529;
        loop {
            let step = {
                let (v0, v1) = m3529;
                let m3528 = v0;
                match m3528.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f496(v0: List<u64>) -> R<Bytes> {
    let m3532 = v0;
    match m3532.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a3533 = f552(v1)?;
            let a3534 = f496(v2)?;
            Ok(append_bytes(a3533, a3534))
        }
    }
}

pub fn f497(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m3535 = v0;
    match m3535.uncons() {
        None => {
            let a3536 = v2;
            let a3537 = 0u64;
            if nat_eq(a3536, a3537) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a3540 = {
                    let a3538 = v3;
                    let a3539 = v1;
                    nat_mul(a3538, a3539)?
                };
                let a3541 = v2;
                nat_add(a3540, a3541)?
            };
            Ok(List::cons({
                let a3542 = v5;
                let a3543 = f498();
                let a3544 = 0u64;
                nat_rem(a3542, a3543, a3544)
            }, f497(v4, v1, {
                let a3545 = v5;
                let a3546 = f498();
                let a3547 = 0u64;
                nat_quot(a3545, a3546, a3547)
            })?))
        }
    }
}

pub fn f498() -> u64 {
    1000000000u64
}

pub fn f499(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3592 = {
            let a3568 = {
                let a3556 = {
                    let a3550 = {
                        let a3548 = v1;
                        let a3549 = 128u8;
                        fixed_u8::bit_and(a3548, a3549)
                    };
                    let a3551 = 128u8;
                    if equal(a3550, a3551) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3557 = {
                    let a3554 = {
                        let a3552 = v1;
                        let a3553 = 64u8;
                        fixed_u8::bit_and(a3552, a3553)
                    };
                    let a3555 = 64u8;
                    if equal(a3554, a3555) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3556, a3557)?
            };
            let a3569 = {
                let a3566 = {
                    let a3560 = {
                        let a3558 = v1;
                        let a3559 = 32u8;
                        fixed_u8::bit_and(a3558, a3559)
                    };
                    let a3561 = 32u8;
                    if equal(a3560, a3561) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3567 = {
                    let a3564 = {
                        let a3562 = v1;
                        let a3563 = 16u8;
                        fixed_u8::bit_and(a3562, a3563)
                    };
                    let a3565 = 16u8;
                    if equal(a3564, a3565) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3566, a3567)?
            };
            nat_add(a3568, a3569)?
        };
        let a3593 = {
            let a3590 = {
                let a3578 = {
                    let a3572 = {
                        let a3570 = v1;
                        let a3571 = 8u8;
                        fixed_u8::bit_and(a3570, a3571)
                    };
                    let a3573 = 8u8;
                    if equal(a3572, a3573) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3579 = {
                    let a3576 = {
                        let a3574 = v1;
                        let a3575 = 4u8;
                        fixed_u8::bit_and(a3574, a3575)
                    };
                    let a3577 = 4u8;
                    if equal(a3576, a3577) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3578, a3579)?
            };
            let a3591 = {
                let a3588 = {
                    let a3582 = {
                        let a3580 = v1;
                        let a3581 = 2u8;
                        fixed_u8::bit_and(a3580, a3581)
                    };
                    let a3583 = 2u8;
                    if equal(a3582, a3583) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3589 = {
                    let a3586 = {
                        let a3584 = v1;
                        let a3585 = 1u8;
                        fixed_u8::bit_and(a3584, a3585)
                    };
                    let a3587 = 1u8;
                    if equal(a3586, a3587) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3588, a3589)?
            };
            nat_add(a3590, a3591)?
        };
        nat_add(a3592, a3593)?
    };
    let a3600 = f188(f553(), {
        let a3594 = v2;
        let a3595 = 16u64;
        let a3596 = 0u64;
        nat_quot(a3594, a3595, a3596)
    });
    let a3601 = f188(f553(), {
        let a3597 = v2;
        let a3598 = 16u64;
        let a3599 = 0u64;
        nat_rem(a3597, a3598, a3599)
    });
    Ok(append_bytes(a3600, a3601))
}

pub fn f500(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f554(Fn21::F555, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f501(v0: Fn16, p3605: List<List<(Bytes, Bytes)>>, p3606: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3604 = (p3605, p3606);
    {
        let mut m3604 = m3604;
        loop {
            let step = {
                let (v1, v2) = m3604;
                let m3602 = v2;
                match m3602.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3603: Fn16 = v0.clone();
                            c3603.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3604 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f503(v0: Fn17, p3611: u64, p3612: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3610 = (p3611, p3612);
    {
        let mut m3610 = m3610;
        loop {
            let step = {
                let (v1, v2) = m3610;
                let m3607 = v1;
                if m3607 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3607 - 1;
                    let m3609 = {
                        let c3608: Fn17 = v0.clone();
                        c3608.apply(v2.clone())?
                    };
                    match m3609 {
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
                Ok(next) => { m3610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f556(v0)
}

pub fn f505(v0: Fn18, p3616: List<List<(Bytes, ())>>, p3617: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m3615 = (p3616, p3617);
    {
        let mut m3615 = m3615;
        loop {
            let step = {
                let (v1, v2) = m3615;
                let m3613 = v2;
                match m3613.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c3614: Fn18 = v0.clone();
                            c3614.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3615 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f507(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a3619 = {
        let a3618 = v0.clone();
        length_list(a3618)
    };
    let a3620 = 1u64;
    if nat_le(a3619, a3620) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f557(v0)?))
    }
}

pub fn f508(v0: Fn19, p3624: List<(Bytes, ())>, p3625: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m3623 = (p3624, p3625);
    {
        let mut m3623 = m3623;
        loop {
            let step = {
                let (v1, v2) = m3623;
                let m3621 = v2;
                match m3621.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c3622: Fn19 = v0.clone();
                            c3622.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3623 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f510(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f125(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a3627 = v2;
        let a3628 = {
            let a3626 = v0;
            length_bytes(a3626)
        };
        if nat_lt(a3627, a3628) {
            Ok(Some({
                let a3629 = v2;
                let a3630 = 1u64;
                nat_add(a3629, a3630)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f511(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3631 = 0u64;
    let a3632 = v1;
    if if nat_lt(a3631, a3632) {
        f187({
            let a3635 = f185(v0, {
                let a3633 = v1;
                let a3634 = 1u64;
                nat_sub(a3633, a3634)
            })?;
            let a3636 = 10u64;
            nat_eq(a3635, a3636)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3637 = v1;
            let a3638 = 1u64;
            nat_sub(a3637, a3638)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f512(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f249(f185(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3639 = v1;
            let a3640 = 1u64;
            nat_add(a3639, a3640)?
        }))
    }
}

pub fn f513(v0: Bytes, v1: u64, v2: u64) -> R<Adt26> {
    let v3: (u64, u64) = {
        let (h3641, _) = f218(Fn6::F558(v2, v0.clone()), {
            let a3643 = {
                let a3642 = v0.clone();
                length_bytes(a3642)
            };
            let a3644 = 1u64;
            nat_add(a3643, a3644)?
        }, (v1, 0u64))?;
        h3641
    };
    let v4: u64 = f514(v0.clone(), {
        let (h3645, _) = v3;
        h3645
    }, v2)?;
    let a3646 = f185(v0, v4)?;
    let a3647 = 9u64;
    if if nat_eq(a3646, a3647) {
        let a3651 = {
            let a3649 = v4;
            let a3650 = {
                let (h3648, _) = v3;
                h3648
            };
            nat_sub(a3649, a3650)
        };
        let a3652 = v2;
        nat_lt(a3651, a3652)
    } else {
        false
    } {
        Ok(Adt26::C1(f129(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt26::C0({
            let (_, h3653) = v3;
            h3653
        }, {
            let (h3654, _) = v3;
            h3654
        }))
    }
}

pub fn f514(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3655, _) = f179(Fn5::F559(v2, v0.clone(), v1), {
        let a3657 = {
            let a3656 = v0;
            length_bytes(a3656)
        };
        let a3658 = 1u64;
        nat_add(a3657, a3658)?
    }, v1)?;
    Ok(h3655)
}

pub fn f515(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f514(v0.clone(), v1, v2)?;
    let a3661 = {
        let a3659 = v3;
        let a3660 = v1;
        nat_sub(a3659, a3660)
    };
    let a3662 = v2;
    if nat_eq(a3661, a3662) {
        Ok(f187({
            let a3663 = f185(v0, v3)?;
            let a3664 = 256u64;
            nat_eq(a3663, a3664)
        }))
    } else {
        Ok(false)
    }
}

pub fn f516(v0: Bytes, p3674: u64, v2: u64, v3: bool, p3675: u64, p3676: Bytes, p3677: u64) -> R<Adt27> {
    let m3673 = (p3674, (p3675, (p3676, p3677)));
    {
        let mut m3673 = m3673;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3673;
                let m3665 = v4;
                if m3665 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt27>>(Ok(Adt27::C1(f129(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3665 - 1;
                    let v8: u64 = f514(v0.clone(), v1, v2)?;
                    let v9: bool = f187(f248(f185(v0.clone(), v8)?));
                    let v10: u64 = f337(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f431(v6), List::cons(f265(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3666 = f513(v0.clone(), f252(v0.clone(), v10)?, v2)?;
                    match m3666 {
                        Adt26::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f514(v0.clone(), v15, v2)?;
                            if f515(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt27>>((v15, (v7, ({
                                    let a3669 = v11;
                                    let a3670 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f187(f248(f185(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a3667 = v13;
                                        let a3668 = 0u64;
                                        if nat_eq(a3667, a3668) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3669, a3670)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt27>>(Ok(Adt27::C0(v11, v13, {
                                    let a3671 = f185(v0.clone(), v16)?;
                                    let a3672 = 256u64;
                                    if nat_eq(a3671, a3672) {
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
                Ok(next) => { m3673 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f253(v0.clone(), {
        let (h3678, _) = v1;
        h3678
    })?;
    let a3679 = f185(v0, v2)?;
    let a3680 = 10u64;
    if nat_eq(a3679, a3680) {
        Ok(Some(({
            let a3681 = v2;
            let a3682 = 1u64;
            nat_add(a3681, a3682)?
        }, f433({
            let (_, h3683) = v1;
            h3683
        }, {
            let a3685 = v2;
            let a3686 = {
                let (h3684, _) = v1;
                h3684
            };
            nat_sub(a3685, a3686)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f518(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f560(v0, v1)
}

pub fn f519(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt19> {
    let v4: (u64, u64) = f521(v0.clone(), v1, 2u64)?;
    let a3689 = {
        let (h3687, _) = v4;
        h3687
    };
    let a3690 = {
        let a3688 = v0.clone();
        length_bytes(a3688)
    };
    if nat_eq(a3689, a3690) {
        if f162(false, {
            let (_, h3691) = v4;
            h3691
        }) {
            Ok(Adt19::C0(Adt2::C3(false, {
                let (_, h3692) = v4;
                h3692
            }), v3))
        } else {
            Ok(Adt19::C1(f520(v2)?))
        }
    } else {
        Ok(Adt19::C0(Adt2::C6(v0), v3))
    }
}

pub fn f520(v0: u64) -> R<Bytes> {
    f129(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f521(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3693, _) = f218(Fn6::F561(v1, v0.clone()), {
        let a3695 = {
            let a3694 = v0;
            length_bytes(a3694)
        };
        let a3696 = 1u64;
        nat_add(a3695, a3696)?
    }, (v2, 0u64))?;
    Ok(h3693)
}

pub fn f522(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f264(f8(Str::lit("abtnvfr")), v0)?;
    let a3697 = v1;
    let a3698 = 7u64;
    if nat_lt(a3697, a3698) {
        Ok(Some({
            let a3699 = v1;
            let a3700 = 7u64;
            nat_add(a3699, a3700)?
        }))
    } else {
        let a3701 = v0;
        let a3702 = 48u64;
        if nat_eq(a3701, a3702) {
            Ok(Some(0u64))
        } else {
            let a3703 = v0;
            let a3704 = 9u64;
            if nat_eq(a3703, a3704) {
                Ok(Some(9u64))
            } else {
                let a3705 = v0;
                let a3706 = 101u64;
                if nat_eq(a3705, a3706) {
                    Ok(Some(27u64))
                } else {
                    let a3707 = v0;
                    let a3708 = 78u64;
                    if nat_eq(a3707, a3708) {
                        Ok(Some(133u64))
                    } else {
                        let a3709 = v0;
                        let a3710 = 95u64;
                        if nat_eq(a3709, a3710) {
                            Ok(Some(160u64))
                        } else {
                            let a3711 = v0;
                            let a3712 = 76u64;
                            if nat_eq(a3711, a3712) {
                                Ok(Some(8232u64))
                            } else {
                                let a3713 = v0;
                                let a3714 = 80u64;
                                if nat_eq(a3713, a3714) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a3715 = f264(f8(Str::lit(" \"/\\")), v0)?;
                                    let a3716 = 4u64;
                                    if nat_lt(a3715, a3716) {
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

pub fn f523(v0: u64) -> R<Bytes> {
    let a3717 = v0;
    let a3718 = 128u64;
    if nat_lt(a3717, a3718) {
        Ok(f562(f563(), v0))
    } else {
        let a3719 = v0;
        let a3720 = 2048u64;
        if nat_lt(a3719, a3720) {
            let a3728 = f562(f564(), {
                let a3726 = {
                    let a3724 = {
                        let a3721 = v0;
                        let a3722 = 64u64;
                        let a3723 = 0u64;
                        nat_quot(a3721, a3722, a3723)
                    };
                    let a3725 = 2u64;
                    nat_sub(a3724, a3725)
                };
                let a3727 = 2u64;
                nat_mul(a3726, a3727)?
            });
            let a3729 = f565(v0)?;
            Ok(append_bytes(a3728, a3729))
        } else {
            let a3730 = v0;
            let a3731 = 65536u64;
            if nat_lt(a3730, a3731) {
                let a3742 = f562(f566(), {
                    let a3735 = {
                        let a3732 = v0;
                        let a3733 = 4096u64;
                        let a3734 = 0u64;
                        nat_quot(a3732, a3733, a3734)
                    };
                    let a3736 = 3u64;
                    nat_mul(a3735, a3736)?
                });
                let a3743 = {
                    let a3740 = f565({
                        let a3737 = v0;
                        let a3738 = 64u64;
                        let a3739 = 0u64;
                        nat_quot(a3737, a3738, a3739)
                    })?;
                    let a3741 = f565(v0)?;
                    append_bytes(a3740, a3741)
                };
                Ok(append_bytes(a3742, a3743))
            } else {
                let a3757 = f562(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a3744 = v0;
                    let a3745 = 262144u64;
                    let a3746 = 0u64;
                    nat_quot(a3744, a3745, a3746)
                });
                let a3758 = {
                    let a3755 = f565({
                        let a3747 = v0;
                        let a3748 = 4096u64;
                        let a3749 = 0u64;
                        nat_quot(a3747, a3748, a3749)
                    })?;
                    let a3756 = {
                        let a3753 = f565({
                            let a3750 = v0;
                            let a3751 = 64u64;
                            let a3752 = 0u64;
                            nat_quot(a3750, a3751, a3752)
                        })?;
                        let a3754 = f565(v0)?;
                        append_bytes(a3753, a3754)
                    };
                    append_bytes(a3755, a3756)
                };
                Ok(append_bytes(a3757, a3758))
            }
        }
    }
}

pub fn f524(v0: u64) -> R<Bytes> {
    f129(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f525(v0: Bytes, p3773: u64, v2: u64, p3774: u64, p3775: u64) -> R<Adt26> {
    let m3772 = (p3773, (p3774, p3775));
    {
        let mut m3772 = m3772;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3772;
                let m3759 = v3;
                if m3759 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt26>>(Ok(Adt26::C1(f129(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3759 - 1;
                    let v6: u64 = {
                        let a3760 = v1;
                        let a3761 = 1u64;
                        nat_add(a3760, a3761)?
                    };
                    let v7: u64 = f253(v0.clone(), v6)?;
                    let v8: u64 = f251(v0.clone(), v7)?;
                    let v9: u64 = f185(v0.clone(), v8)?;
                    if f183(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt26>>(Ok(Adt26::C1(f129(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3762 = v9;
                        let a3763 = 10u64;
                        if nat_eq(a3762, a3763) {
                            Ok::<(u64, (u64, u64)), R<Adt26>>((v8, (v5, {
                                let a3764 = v4;
                                let a3765 = 1u64;
                                nat_add(a3764, a3765)?
                            })))
                        } else {
                            let a3766 = v9;
                            let a3767 = 256u64;
                            if nat_eq(a3766, a3767) {
                                Err::<(u64, (u64, u64)), R<Adt26>>(Ok(Adt26::C1(f129(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3770 = {
                                    let a3768 = v7;
                                    let a3769 = v6;
                                    nat_sub(a3768, a3769)
                                };
                                let a3771 = v2;
                                if nat_lt(a3770, a3771) {
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
                Ok(next) => { m3772 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(v0: Bytes, p3785: u64, p3786: u64, p3787: u64) -> R<Option<u64>> {
    let m3784 = (p3785, (p3786, p3787));
    {
        let mut m3784 = m3784;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3784;
                let m3776 = v2;
                if m3776 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3776 - 1;
                    let m3777 = f567(f185(v0.clone(), v1)?);
                    match m3777 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3778 = v1;
                                let a3779 = 1u64;
                                nat_add(a3778, a3779)?
                            }, (v4, {
                                let a3782 = {
                                    let a3780 = v3;
                                    let a3781 = 16u64;
                                    nat_mul(a3780, a3781)?
                                };
                                let a3783 = v5;
                                nat_add(a3782, a3783)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(v0: Bytes, v1: u64) -> R<u64> {
    let (h3788, _) = f179(Fn5::F568(v0.clone()), {
        let a3790 = {
            let a3789 = v0;
            length_bytes(a3789)
        };
        let a3791 = 1u64;
        nat_add(a3790, a3791)?
    }, v1)?;
    Ok(h3788)
}

pub fn f528(v0: Bytes, v1: u64) -> R<u64> {
    let (h3792, _) = f179(Fn5::F569(v0.clone()), {
        let a3794 = {
            let a3793 = v0;
            length_bytes(a3793)
        };
        let a3795 = 1u64;
        nat_add(a3794, a3795)?
    }, v1)?;
    Ok(h3792)
}

pub fn f529(v0: Fn20, p3800: u64, p3801: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3799 = (p3800, p3801);
    {
        let mut m3799 = m3799;
        loop {
            let step = {
                let (v1, v2) = m3799;
                let m3796 = v1;
                if m3796 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3796 - 1;
                    let m3798 = {
                        let c3797: Fn20 = v0.clone();
                        c3797.apply(v2)?
                    };
                    match m3798 {
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
                Ok(next) => { m3799 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f530(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f570(v1, v0, v2)
}

pub fn f531(v0: Bytes, v1: u64, v2: u64) -> R<Adt23> {
    let v3: u64 = f185(v0.clone(), v1)?;
    let a3802 = v3;
    let a3803 = 34u64;
    if if nat_eq(a3802, a3803) {
        true
    } else {
        let a3804 = v3;
        let a3805 = 39u64;
        nat_eq(a3804, a3805)
    } {
        f342(v0, v1, v2, false)
    } else {
        let m3806 = f256(v0.clone(), v1)?;
        match m3806 {
            None => {
                let a3807 = v3;
                let a3808 = 91u64;
                if if nat_eq(a3807, a3808) {
                    true
                } else {
                    let a3809 = v3;
                    let a3810 = 123u64;
                    nat_eq(a3809, a3810)
                } {
                    Ok(Adt23::C1(f129(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f344(v0.clone(), v1, true)? {
                        let v5: u64 = f345(v0.clone(), v1, true)?;
                        let v6: u64 = f251(v0.clone(), v5)?;
                        let a3811 = f185(v0.clone(), v6)?;
                        let a3812 = 10u64;
                        if nat_eq(a3811, a3812) {
                            let m3813 = f443(v0.clone(), v6, v2)?;
                            match m3813 {
                                Ok(v8) => {
                                    let v9: u64 = f185(v0.clone(), v8)?;
                                    let a3814 = v9;
                                    let a3815 = 58u64;
                                    if if if nat_eq(a3814, a3815) {
                                        true
                                    } else {
                                        let a3816 = v9;
                                        let a3817 = 44u64;
                                        nat_eq(a3816, a3817)
                                    } {
                                        true
                                    } else {
                                        let a3818 = v9;
                                        let a3819 = 125u64;
                                        nat_eq(a3818, a3819)
                                    } {
                                        Ok(Adt23::C0(f265(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt23::C1(f129(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt23::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt23::C0(f265(v0, v1, v5), v5))
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

pub fn f532(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3820, _) = v2;
        h3820
    };
    let v4: u64 = f185(v0.clone(), v3)?;
    if f249(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f248(v4) {
            Ok(Some(({
                let a3821 = v3;
                let a3822 = 1u64;
                nat_add(a3821, a3822)?
            }, {
                let (_, h3823) = v2;
                h3823
            })))
        } else {
            let a3824 = v4;
            let a3825 = 58u64;
            if if nat_eq(a3824, a3825) {
                f187(f339(f185(v0.clone(), {
                    let a3826 = v3;
                    let a3827 = 1u64;
                    nat_add(a3826, a3827)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3828 = v4;
                let a3829 = 35u64;
                if if nat_eq(a3828, a3829) {
                    f248(f185(v0, {
                        let a3830 = v3;
                        let a3831 = 1u64;
                        nat_sub(a3830, a3831)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f426(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3832 = v3;
                            let a3833 = 1u64;
                            nat_add(a3832, a3833)?
                        }, {
                            let a3834 = v3;
                            let a3835 = 1u64;
                            nat_add(a3834, a3835)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f533(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f571(v0, v1)
}

pub fn f534(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    let v3: u64 = f185(v0, v2)?;
    if if if f248(v3) {
        true
    } else {
        f249(v3)
    } {
        true
    } else {
        if v1 {
            let a3836 = v3;
            let a3837 = 62u64;
            nat_eq(a3836, a3837)
        } else {
            f426(v3)
        }
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3838 = v2;
            let a3839 = 1u64;
            nat_add(a3838, a3839)?
        }))
    }
}

pub fn f535(v0: u64, v1: Bytes, p3850: u64, p3851: u64, p3852: bool) -> R<bool> {
    let m3849 = (p3850, (p3851, p3852));
    {
        let mut m3849 = m3849;
        loop {
            let step = {
                let (v2, (v3, v4)) = m3849;
                let m3840 = v3;
                if m3840 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m3840 - 1;
                    let v6: u64 = f572(v1.clone(), v2)?;
                    if v4 {
                        let a3841 = v0;
                        let a3842 = v6;
                        if nat_lt(a3841, a3842) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3843 = v2;
                                let a3844 = 1u64;
                                nat_add(a3843, a3844)?
                            }, (v5, false)))
                        }
                    } else {
                        let a3845 = v0;
                        let a3846 = v6;
                        if nat_le(a3845, a3846) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3847 = v2;
                                let a3848 = 1u64;
                                nat_add(a3847, a3848)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f536(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f573(v1, f330(v0, v2)?) {
        Ok(Some({
            let a3853 = v2;
            let a3854 = 1u64;
            nat_add(a3853, a3854)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f537(p3857: List<Adt14>, p3858: List<Adt14>) -> List<Adt14> {
    let m3856 = (p3857, p3858);
    {
        let mut m3856 = m3856;
        loop {
            let step = {
                let (v0, v1) = m3856;
                let m3855 = v0;
                match m3855.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3856 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: Bytes, p3863: List<(Bytes, Adt15)>) -> Option<Adt15> {
    let m3862 = p3863;
    {
        let mut m3862 = m3862;
        loop {
            let step = {
                let v1 = m3862;
                let m3859 = v1;
                match m3859.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt15)>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h3860, _) = v2;
                            h3860
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt15)>, Option<Adt15>>(Some({
                                let (_, h3861) = v2;
                                h3861
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt15)>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3862 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539() -> List<(Bytes, Adt15)> {
    List::cons((f8(Str::lit("boolean")), f469(f370(f8(Str::lit("Bool"))), f8(Str::lit("Boolean")))), List::cons((f8(Str::lit("byte")), f469(f370(f8(Str::lit("Int"))), f8(Str::lit("Byte")))), List::cons((f8(Str::lit("octet")), f469(f370(f8(Str::lit("Nat"))), f8(Str::lit("Octet")))), List::cons((f8(Str::lit("bigint")), f469(f370(f8(Str::lit("Int"))), f8(Str::lit("BigInt")))), List::cons((f8(Str::lit("float")), f469(f370(f574()), f8(Str::lit("Float")))), List::cons((f8(Str::lit("double")), f469(f370(f574()), f8(Str::lit("Double")))), List::cons((f8(Str::lit("DOMString")), f469(f370(f8(Str::lit("(List Nat)"))), f8(Str::lit("String")))), List::cons((f8(Str::lit("USVString")), f469(f370(f8(Str::lit("Bytes"))), f8(Str::lit("USVString")))), List::cons((f8(Str::lit("ByteString")), f469(f370(f8(Str::lit("Bytes"))), f8(Str::lit("ByteString")))), List::cons((f8(Str::lit("ArrayBuffer")), f469(f370(f8(Str::lit("Bytes"))), f8(Str::lit("ArrayBuffer")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f469(f370(f8(Str::lit("Bytes"))), f8(Str::lit("SharedArrayBuffer")))), List::cons((f8(Str::lit("DataView")), f469(f370(f8(Str::lit("Bytes"))), f8(Str::lit("DataView")))), List::cons((f8(Str::lit("Int8Array")), f469(f370(f8(Str::lit("(List Int)"))), f8(Str::lit("Int8Array")))), List::cons((f8(Str::lit("Int16Array")), f469(f370(f8(Str::lit("(List Int)"))), f8(Str::lit("Int16Array")))), List::cons((f8(Str::lit("Int32Array")), f469(f370(f8(Str::lit("(List Int)"))), f8(Str::lit("Int32Array")))), List::cons((f8(Str::lit("BigInt64Array")), f469(f370(f8(Str::lit("(List Int)"))), f8(Str::lit("BigInt64Array")))), List::cons((f8(Str::lit("Uint8Array")), f469(f370(f8(Str::lit("(List Nat)"))), f8(Str::lit("Uint8Array")))), List::cons((f8(Str::lit("Uint16Array")), f469(f370(f8(Str::lit("(List Nat)"))), f8(Str::lit("Uint16Array")))), List::cons((f8(Str::lit("Uint32Array")), f469(f370(f8(Str::lit("(List Nat)"))), f8(Str::lit("Uint32Array")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f469(f370(f8(Str::lit("(List Nat)"))), f8(Str::lit("Uint8ClampedArray")))), List::cons((f8(Str::lit("BigUint64Array")), f469(f370(f8(Str::lit("(List Nat)"))), f8(Str::lit("BigUint64Array")))), List::cons((f8(Str::lit("Float16Array")), f469(f473(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)"))), f8(Str::lit("Float16Array")))), List::cons((f8(Str::lit("Float32Array")), f469(f575(false), f8(Str::lit("Float32Array")))), List::cons((f8(Str::lit("Float64Array")), f469(f575(true), f8(Str::lit("Float64Array")))), List::cons((f8(Str::lit("any")), f469(f473(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)"))), f8(Str::lit("Any")))), List::cons((f8(Str::lit("object")), f469(f473(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)"))), f8(Str::lit("Object")))), List::cons((f8(Str::lit("symbol")), f469(f473(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)"))), f8(Str::lit("Symbol")))), List::cons((f8(Str::lit("undefined")), f469(f473(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)"))), f8(Str::lit("Undefined")))), List::<(Bytes, Adt15)>::nil()))))))))))))))))))))))))))))
}

pub fn f540(v0: u64) -> Bytes {
    f46({
        let a3864 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3864)
    }, v0, 1u64)
}

pub fn f541(p3875: u64, v1: Bytes, v2: Bytes, p3876: u64) -> R<bool> {
    let m3874 = (p3875, p3876);
    {
        let mut m3874 = m3874;
        loop {
            let step = {
                let (v0, v3) = m3874;
                let m3865 = v0;
                if m3865 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m3865 - 1;
                    let v5: u64 = f576(v1.clone(), v3)?;
                    let v6: u64 = f576(v2.clone(), v3)?;
                    let a3866 = v5;
                    let a3867 = v6;
                    if nat_lt(a3866, a3867) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a3868 = v6;
                        let a3869 = v5;
                        if nat_lt(a3868, a3869) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a3870 = v5;
                            let a3871 = 0u64;
                            if nat_eq(a3870, a3871) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3872 = v3;
                                    let a3873 = 1u64;
                                    nat_add(a3872, a3873)?
                                }))
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

pub fn f542(p3889: u64, v1: Bytes, p3890: u64, p3891: Bytes) -> R<Option<Bytes>> {
    let m3888 = (p3889, (p3890, p3891));
    {
        let mut m3888 = m3888;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3888;
                let m3877 = v0;
                if m3877 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3877 - 1;
                    let a3879 = {
                        let a3878 = v1.clone();
                        length_bytes(a3878)
                    };
                    let a3880 = v2;
                    if nat_le(a3879, a3880) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3881 = f332(v1.clone(), v2)?;
                        match m3881 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3883 = v2;
                                    let a3884 = {
                                        let (_, h3882) = v5;
                                        h3882
                                    };
                                    nat_add(a3883, a3884)?
                                }, {
                                    let a3886 = v3;
                                    let a3887 = f577(v1.clone(), v2, {
                                        let (h3885, _) = v5;
                                        h3885
                                    });
                                    append_bytes(a3886, a3887)
                                })))
                            }
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

pub fn f543(p3901: u64, v1: Bytes, p3902: u64) -> R<bool> {
    let m3900 = (p3901, p3902);
    {
        let mut m3900 = m3900;
        loop {
            let step = {
                let (v0, v2) = m3900;
                let m3892 = v0;
                if m3892 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3892 - 1;
                    let m3895 = {
                        let a3893 = v1.clone();
                        let a3894 = v2;
                        index_bytes(a3893, a3894)
                    };
                    match m3895 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3896 = f60(v4)?;
                            let a3897 = 128u64;
                            if nat_lt(a3896, a3897) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3898 = v2;
                                    let a3899 = 1u64;
                                    nat_add(a3898, a3899)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f544(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f170(v0.clone(), 0u64, {
        let a3903 = v0;
        length_bytes(a3903)
    }, false, Bytes::lit(&[]))?;
    let m3906 = {
        let a3904 = v1.clone();
        let a3905 = 0u64;
        index_bytes(a3904, a3905)
    };
    match m3906 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f60(v2)?;
            if f228(v3) {
                let a3913 = f46({
                    let a3907 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3907)
                }, {
                    let a3908 = v3;
                    let a3909 = 65u64;
                    nat_sub(a3908, a3909)
                }, 1u64);
                let a3914 = f46(v1.clone(), 1u64, {
                    let a3911 = {
                        let a3910 = v1;
                        length_bytes(a3910)
                    };
                    let a3912 = 1u64;
                    nat_sub(a3911, a3912)
                });
                Ok(append_bytes(a3913, a3914))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f545(v0: Bytes) -> R<bool> {
    let m3917 = {
        let a3915 = v0;
        let a3916 = 0u64;
        index_bytes(a3915, a3916)
    };
    match m3917 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f158(f227(f60(v1)?), f228(f60(v1)?)))
        }
    }
}

pub fn f546(p3930: u64, v1: Bytes, p3931: u64, p3932: Bytes) -> R<Option<Bytes>> {
    let m3929 = (p3930, (p3931, p3932));
    {
        let mut m3929 = m3929;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3929;
                let m3918 = v0;
                if m3918 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3918 - 1;
                    let a3920 = {
                        let a3919 = v1.clone();
                        length_bytes(a3919)
                    };
                    let a3921 = v2;
                    if nat_le(a3920, a3921) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3922 = f332(v1.clone(), v2)?;
                        match m3922 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3924 = v2;
                                    let a3925 = {
                                        let (_, h3923) = v5;
                                        h3923
                                    };
                                    nat_add(a3924, a3925)?
                                }, {
                                    let a3927 = v3;
                                    let a3928 = f578({
                                        let (h3926, _) = v5;
                                        h3926
                                    });
                                    append_bytes(a3927, a3928)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("false")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f548(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3933, _) = v1;
        h3933
    };
    let m3935 = {
        let (_, h3934) = v1;
        h3934
    };
    match m3935 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3938 = {
                let a3936 = v0.clone();
                let a3937 = v2;
                index_bytes(a3936, a3937)
            };
            match m3938 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3943 = {
                        let a3941 = v0;
                        let a3942 = {
                            let a3939 = v2;
                            let a3940 = 1u64;
                            nat_add(a3939, a3940)?
                        };
                        index_bytes(a3941, a3942)
                    };
                    match m3943 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f579(v4)? {
                                f579(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3944 = v2;
                                    let a3945 = 2u64;
                                    nat_add(a3944, a3945)?
                                }, Some({
                                    let a3950 = v3;
                                    let a3951 = f188(f580(), {
                                        let a3948 = {
                                            let a3946 = f581(v4)?;
                                            let a3947 = 16u64;
                                            nat_mul(a3946, a3947)?
                                        };
                                        let a3949 = f581(v5)?;
                                        nat_add(a3948, a3949)?
                                    });
                                    append_bytes(a3950, a3951)
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

pub fn f549(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3952 = f223(v0, v1);
    let a3953 = f494(v0, v2);
    fixed_u32::bit_or(a3952, a3953)
}

pub fn f550(v0: u32) -> u32 {
    let a3956 = {
        let a3954 = f549(v0, 17u32, 15u32);
        let a3955 = f549(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3954, a3955)
    };
    let a3957 = f223(v0, 10u32);
    fixed_u32::bit_xor(a3956, a3957)
}

pub fn f551(v0: u32) -> u32 {
    let a3960 = {
        let a3958 = f549(v0, 7u32, 25u32);
        let a3959 = f549(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3958, a3959)
    };
    let a3961 = f223(v0, 3u32);
    fixed_u32::bit_xor(a3960, a3961)
}

pub fn f552(v0: u64) -> R<Bytes> {
    let v1: Bytes = f89(v0)?;
    let a3965 = f402({
        let a3963 = 9u64;
        let a3964 = {
            let a3962 = v1.clone();
            length_bytes(a3962)
        };
        nat_sub(a3963, a3964)
    });
    let a3966 = v1;
    Ok(append_bytes(a3965, a3966))
}

pub fn f553() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f554(v0: Fn21, p3970: List<List<(Bytes, Bytes)>>, p3971: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3969 = (p3970, p3971);
    {
        let mut m3969 = m3969;
        loop {
            let step = {
                let (v1, v2) = m3969;
                let m3967 = v2;
                match m3967.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3968: Fn21 = v0.clone();
                            c3968.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3969 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f556(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3973 = {
        let a3972 = v0.clone();
        length_list(a3972)
    };
    let a3974 = 1u64;
    if nat_le(a3973, a3974) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f582(v0)?))
    }
}

pub fn f557(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f411({
        let (_, h3975) = {
            let (h3976, _) = f583(Fn22::F584, {
                let a3978 = {
                    let a3977 = v0.clone();
                    length_list(a3977)
                };
                let a3979 = 1u64;
                nat_add(a3978, a3979)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h3976
        };
        h3975
    }))
}

pub fn f558(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f585(v1, v0, v2)
}

pub fn f559(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f586(v1, v2, v0, v3)
}

pub fn f560(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3982 = {
        let (h3980, _) = v1;
        h3980
    };
    let a3983 = {
        let a3981 = v0.clone();
        length_bytes(a3981)
    };
    if nat_lt(a3982, a3983) {
        Ok(Some((f252(v0, {
            let (h3984, _) = v1;
            h3984
        })?, {
            let a3986 = {
                let (_, h3985) = v1;
                h3985
            };
            let a3987 = 1u64;
            nat_add(a3986, a3987)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f561(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f587(v1, v0, v2)
}

pub fn f562(v0: Bytes, v1: u64) -> Bytes {
    f46(v0, v1, 1u64)
}

pub fn f563() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f564() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f565(v0: u64) -> R<Bytes> {
    Ok(f562(f588(), {
        let a3993 = {
            let a3991 = {
                let a3988 = v0;
                let a3989 = 64u64;
                let a3990 = 0u64;
                nat_rem(a3988, a3989, a3990)
            };
            let a3992 = 2u64;
            nat_mul(a3991, a3992)?
        };
        let a3994 = 1u64;
        nat_add(a3993, a3994)?
    }))
}

pub fn f566() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f567(v0: u64) -> Option<u64> {
    let a3995 = 48u64;
    let a3996 = v0;
    if if nat_le(a3995, a3996) {
        let a3997 = v0;
        let a3998 = 57u64;
        nat_le(a3997, a3998)
    } else {
        false
    } {
        Some({
            let a3999 = v0;
            let a4000 = 48u64;
            nat_sub(a3999, a4000)
        })
    } else {
        let a4001 = 97u64;
        let a4002 = v0;
        if if nat_le(a4001, a4002) {
            let a4003 = v0;
            let a4004 = 102u64;
            nat_le(a4003, a4004)
        } else {
            false
        } {
            Some({
                let a4005 = v0;
                let a4006 = 87u64;
                nat_sub(a4005, a4006)
            })
        } else {
            let a4007 = 65u64;
            let a4008 = v0;
            if if nat_le(a4007, a4008) {
                let a4009 = v0;
                let a4010 = 70u64;
                nat_le(a4009, a4010)
            } else {
                false
            } {
                Some({
                    let a4011 = v0;
                    let a4012 = 55u64;
                    nat_sub(a4011, a4012)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f568(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f589(v0, v1)
}

pub fn f569(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f590(v0, v1)
}

pub fn f570(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h4013, _) = v2;
        h4013
    };
    let v4: u64 = f185(v0.clone(), v3)?;
    let (_, h4014) = v2;
    if h4014 {
        Ok(None::<(u64, bool)>)
    } else {
        if f248(v4) {
            Ok(Some(({
                let a4015 = v3;
                let a4016 = 1u64;
                nat_add(a4015, a4016)?
            }, false)))
        } else {
            let a4017 = v4;
            let a4018 = 35u64;
            if if nat_eq(a4017, a4018) {
                let a4019 = v3;
                let a4020 = 0u64;
                if if nat_eq(a4019, a4020) {
                    true
                } else {
                    f248(f185(v0.clone(), {
                        let a4021 = v3;
                        let a4022 = 1u64;
                        nat_sub(a4021, a4022)
                    })?)
                } {
                    true
                } else {
                    let a4025 = f185(v0.clone(), {
                        let a4023 = v3;
                        let a4024 = 1u64;
                        nat_sub(a4023, a4024)
                    })?;
                    let a4026 = 10u64;
                    nat_eq(a4025, a4026)
                }
            } else {
                false
            } {
                Ok(Some((f337(v0, v3)?, false)))
            } else {
                let a4027 = v4;
                let a4028 = 10u64;
                if nat_eq(a4027, a4028) {
                    let v5: u64 = {
                        let a4029 = v3;
                        let a4030 = 1u64;
                        nat_add(a4029, a4030)?
                    };
                    let v6: u64 = f253(v0.clone(), v5)?;
                    let v7: u64 = f251(v0.clone(), v6)?;
                    if if f183(v0.clone(), v5)? {
                        true
                    } else {
                        let a4033 = {
                            let a4031 = v6;
                            let a4032 = v5;
                            nat_sub(a4031, a4032)
                        };
                        let a4034 = v1;
                        if if nat_lt(a4033, a4034) {
                            f187(f249(f185(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f187({
                                let a4035 = f185(v0, v7)?;
                                let a4036 = 35u64;
                                nat_eq(a4035, a4036)
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

pub fn f571(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f251(v0.clone(), {
        let (h4037, _) = v1;
        h4037
    })?;
    let a4038 = f185(v0, v2)?;
    let a4039 = 10u64;
    if nat_eq(a4038, a4039) {
        Ok(Some(({
            let a4040 = v2;
            let a4041 = 1u64;
            nat_add(a4040, a4041)?
        }, {
            let a4043 = {
                let (_, h4042) = v1;
                h4042
            };
            let a4044 = 1u64;
            nat_add(a4043, a4044)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f572(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a4045 = v1;
        let a4046 = 3u64;
        nat_mul(a4045, a4046)?
    };
    let a4057 = {
        let a4053 = {
            let a4047 = f103(v0.clone(), v2)?;
            let a4048 = 16384u64;
            nat_mul(a4047, a4048)?
        };
        let a4054 = {
            let a4051 = f103(v0.clone(), {
                let a4049 = v2;
                let a4050 = 1u64;
                nat_add(a4049, a4050)?
            })?;
            let a4052 = 128u64;
            nat_mul(a4051, a4052)?
        };
        nat_add(a4053, a4054)?
    };
    let a4058 = f103(v0, {
        let a4055 = v2;
        let a4056 = 2u64;
        nat_add(a4055, a4056)?
    })?;
    nat_add(a4057, a4058)
}

pub fn f573(v0: u64, v1: u64) -> bool {
    let a4059 = v0;
    let a4060 = 0u64;
    if nat_eq(a4059, a4060) {
        f457(v1)
    } else {
        let a4061 = v0;
        let a4062 = 1u64;
        if nat_eq(a4061, a4062) {
            f456(v1)
        } else {
            let a4063 = v0;
            let a4064 = 2u64;
            if nat_eq(a4063, a4064) {
                f591(v1)
            } else {
                f592(v1)
            }
        }
    }
}

pub fn f574() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f575(v0: bool) -> Adt15 {
    let v1: Bytes = if v0 {
        f140()
    } else {
        f138()
    };
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List ")), List::cons(v1.clone(), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), List::cons(v1, List::<Bytes>::nil()), Bytes::lit(&[]), Bytes::lit(&[]), List::<Adt16>::nil())
}

pub fn f576(v0: Bytes, v1: u64) -> R<u64> {
    let a4066 = v1;
    let a4067 = {
        let a4065 = v0.clone();
        length_bytes(a4065)
    };
    if nat_lt(a4066, a4067) {
        let a4068 = f330(v0, v1)?;
        let a4069 = 1u64;
        nat_add(a4068, a4069)
    } else {
        Ok(0u64)
    }
}

pub fn f577(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f158(f105(f158(f227(v2), f228(v2)), f105({
        let a4070 = v2;
        let a4071 = 117u64;
        !nat_eq(a4070, a4071)
    }, {
        let a4072 = v2;
        let a4073 = 85u64;
        !nat_eq(a4072, a4073)
    })), f105(f229(v2), {
        let a4074 = 0u64;
        let a4075 = v1;
        nat_lt(a4074, a4075)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a4076 = v2;
        let a4077 = 65536u64;
        if nat_lt(a4076, a4077) {
            let a4078 = f8(Str::lit("u"));
            let a4079 = f477(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4078, a4079)
        } else {
            let a4080 = f8(Str::lit("U"));
            let a4081 = f477(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4080, a4081)
        }
    }
}

pub fn f578(v0: u64) -> Bytes {
    let a4082 = f8(Str::lit("u"));
    let a4083 = f477(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4082, a4083)
}

pub fn f579(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4128 = {
            let a4104 = {
                let a4092 = {
                    let a4086 = {
                        let a4084 = v1;
                        let a4085 = 128u8;
                        fixed_u8::bit_and(a4084, a4085)
                    };
                    let a4087 = 128u8;
                    if equal(a4086, a4087) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4093 = {
                    let a4090 = {
                        let a4088 = v1;
                        let a4089 = 64u8;
                        fixed_u8::bit_and(a4088, a4089)
                    };
                    let a4091 = 64u8;
                    if equal(a4090, a4091) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4092, a4093)?
            };
            let a4105 = {
                let a4102 = {
                    let a4096 = {
                        let a4094 = v1;
                        let a4095 = 32u8;
                        fixed_u8::bit_and(a4094, a4095)
                    };
                    let a4097 = 32u8;
                    if equal(a4096, a4097) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4103 = {
                    let a4100 = {
                        let a4098 = v1;
                        let a4099 = 16u8;
                        fixed_u8::bit_and(a4098, a4099)
                    };
                    let a4101 = 16u8;
                    if equal(a4100, a4101) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4102, a4103)?
            };
            nat_add(a4104, a4105)?
        };
        let a4129 = {
            let a4126 = {
                let a4114 = {
                    let a4108 = {
                        let a4106 = v1;
                        let a4107 = 8u8;
                        fixed_u8::bit_and(a4106, a4107)
                    };
                    let a4109 = 8u8;
                    if equal(a4108, a4109) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4115 = {
                    let a4112 = {
                        let a4110 = v1;
                        let a4111 = 4u8;
                        fixed_u8::bit_and(a4110, a4111)
                    };
                    let a4113 = 4u8;
                    if equal(a4112, a4113) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4114, a4115)?
            };
            let a4127 = {
                let a4124 = {
                    let a4118 = {
                        let a4116 = v1;
                        let a4117 = 2u8;
                        fixed_u8::bit_and(a4116, a4117)
                    };
                    let a4119 = 2u8;
                    if equal(a4118, a4119) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4125 = {
                    let a4122 = {
                        let a4120 = v1;
                        let a4121 = 1u8;
                        fixed_u8::bit_and(a4120, a4121)
                    };
                    let a4123 = 1u8;
                    if equal(a4122, a4123) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4124, a4125)?
            };
            nat_add(a4126, a4127)?
        };
        nat_add(a4128, a4129)?
    };
    let a4130 = 48u64;
    let a4131 = v2;
    if if nat_le(a4130, a4131) {
        let a4132 = v2;
        let a4133 = 57u64;
        nat_le(a4132, a4133)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4134 = 97u64;
        let a4135 = v2;
        if nat_le(a4134, a4135) {
            let a4136 = v2;
            let a4137 = 102u64;
            Ok(nat_le(a4136, a4137))
        } else {
            Ok(false)
        }
    }
}

pub fn f580() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f581(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4182 = {
            let a4158 = {
                let a4146 = {
                    let a4140 = {
                        let a4138 = v1;
                        let a4139 = 128u8;
                        fixed_u8::bit_and(a4138, a4139)
                    };
                    let a4141 = 128u8;
                    if equal(a4140, a4141) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4147 = {
                    let a4144 = {
                        let a4142 = v1;
                        let a4143 = 64u8;
                        fixed_u8::bit_and(a4142, a4143)
                    };
                    let a4145 = 64u8;
                    if equal(a4144, a4145) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4146, a4147)?
            };
            let a4159 = {
                let a4156 = {
                    let a4150 = {
                        let a4148 = v1;
                        let a4149 = 32u8;
                        fixed_u8::bit_and(a4148, a4149)
                    };
                    let a4151 = 32u8;
                    if equal(a4150, a4151) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4157 = {
                    let a4154 = {
                        let a4152 = v1;
                        let a4153 = 16u8;
                        fixed_u8::bit_and(a4152, a4153)
                    };
                    let a4155 = 16u8;
                    if equal(a4154, a4155) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4156, a4157)?
            };
            nat_add(a4158, a4159)?
        };
        let a4183 = {
            let a4180 = {
                let a4168 = {
                    let a4162 = {
                        let a4160 = v1;
                        let a4161 = 8u8;
                        fixed_u8::bit_and(a4160, a4161)
                    };
                    let a4163 = 8u8;
                    if equal(a4162, a4163) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4169 = {
                    let a4166 = {
                        let a4164 = v1;
                        let a4165 = 4u8;
                        fixed_u8::bit_and(a4164, a4165)
                    };
                    let a4167 = 4u8;
                    if equal(a4166, a4167) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4168, a4169)?
            };
            let a4181 = {
                let a4178 = {
                    let a4172 = {
                        let a4170 = v1;
                        let a4171 = 2u8;
                        fixed_u8::bit_and(a4170, a4171)
                    };
                    let a4173 = 2u8;
                    if equal(a4172, a4173) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4179 = {
                    let a4176 = {
                        let a4174 = v1;
                        let a4175 = 1u8;
                        fixed_u8::bit_and(a4174, a4175)
                    };
                    let a4177 = 1u8;
                    if equal(a4176, a4177) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4178, a4179)?
            };
            nat_add(a4180, a4181)?
        };
        nat_add(a4182, a4183)?
    };
    let a4184 = v2;
    let a4185 = 57u64;
    if nat_le(a4184, a4185) {
        let a4186 = v2;
        let a4187 = 48u64;
        Ok(nat_sub(a4186, a4187))
    } else {
        let a4188 = v2;
        let a4189 = 87u64;
        Ok(nat_sub(a4188, a4189))
    }
}

pub fn f582(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f500({
        let (_, h4190) = {
            let (h4191, _) = f593(Fn23::F594, {
                let a4193 = {
                    let a4192 = v0.clone();
                    length_list(a4192)
                };
                let a4194 = 1u64;
                nat_add(a4193, a4194)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4191
        };
        h4190
    }))
}

pub fn f583(v0: Fn22, p4199: u64, p4200: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m4198 = (p4199, p4200);
    {
        let mut m4198 = m4198;
        loop {
            let step = {
                let (v1, v2) = m4198;
                let m4195 = v1;
                if m4195 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4195 - 1;
                    let m4197 = {
                        let c4196: Fn22 = v0.clone();
                        c4196.apply(v2.clone())?
                    };
                    match m4197 {
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
                Ok(next) => { m4198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f595(v0)
}

pub fn f585(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f514(v0.clone(), {
        let (h4201, _) = v2;
        h4201
    }, v1)?;
    let a4202 = f185(v0, v3)?;
    let a4203 = 10u64;
    if nat_eq(a4202, a4203) {
        Ok(Some(({
            let a4204 = v3;
            let a4205 = 1u64;
            nat_add(a4204, a4205)?
        }, {
            let a4207 = {
                let (_, h4206) = v2;
                h4206
            };
            let a4208 = 1u64;
            nat_add(a4207, a4208)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f586(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a4209 = f185(v0, v3)?;
    let a4210 = 32u64;
    if if nat_eq(a4209, a4210) {
        let a4213 = {
            let a4211 = v3;
            let a4212 = v1;
            nat_sub(a4211, a4212)
        };
        let a4214 = v2;
        nat_lt(a4213, a4214)
    } else {
        false
    } {
        Ok(Some({
            let a4215 = v3;
            let a4216 = 1u64;
            nat_add(a4215, a4216)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f587(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4218 = f567(f185(v0, {
        let (h4217, _) = v2;
        h4217
    })?);
    match m4218 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a4219 = v3;
            let a4220 = v1;
            if nat_lt(a4219, a4220) {
                Ok(Some(({
                    let a4222 = {
                        let (h4221, _) = v2;
                        h4221
                    };
                    let a4223 = 1u64;
                    nat_add(a4222, a4223)?
                }, {
                    let a4227 = {
                        let a4225 = {
                            let (_, h4224) = v2;
                            h4224
                        };
                        let a4226 = v1;
                        nat_mul(a4225, a4226)?
                    };
                    let a4228 = v3;
                    nat_add(a4227, a4228)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f588() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f589(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f185(v0, v1)?;
    let a4229 = v2;
    let a4230 = 34u64;
    if if if if nat_eq(a4229, a4230) {
        true
    } else {
        let a4231 = v2;
        let a4232 = 92u64;
        nat_eq(a4231, a4232)
    } {
        true
    } else {
        f248(v2)
    } {
        true
    } else {
        f249(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4233 = v1;
            let a4234 = 1u64;
            nat_add(a4233, a4234)?
        }))
    }
}

pub fn f590(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f185(v0, v1)?;
    let a4235 = v2;
    let a4236 = 39u64;
    if if if nat_eq(a4235, a4236) {
        true
    } else {
        f248(v2)
    } {
        true
    } else {
        f249(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4237 = v1;
            let a4238 = 1u64;
            nat_add(a4237, a4238)?
        }))
    }
}

pub fn f591(v0: u64) -> bool {
    let a4239 = 48u64;
    let a4240 = v0;
    if nat_le(a4239, a4240) {
        let a4241 = v0;
        let a4242 = 55u64;
        nat_le(a4241, a4242)
    } else {
        false
    }
}

pub fn f592(v0: u64) -> bool {
    if f355(v0) {
        true
    } else {
        if f457(v0) {
            true
        } else {
            let a4243 = v0;
            let a4244 = 95u64;
            if nat_eq(a4243, a4244) {
                true
            } else {
                let a4245 = v0;
                let a4246 = 45u64;
                nat_eq(a4245, a4246)
            }
        }
    }
}

pub fn f593(v0: Fn23, p4251: u64, p4252: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4250 = (p4251, p4252);
    {
        let mut m4250 = m4250;
        loop {
            let step = {
                let (v1, v2) = m4250;
                let m4247 = v1;
                if m4247 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4247 - 1;
                    let m4249 = {
                        let c4248: Fn23 = v0.clone();
                        c4248.apply(v2.clone())?
                    };
                    match m4249 {
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
                Ok(next) => { m4250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f594(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f596(v0)
}

pub fn f595(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h4253, _) = v0;
        h4253
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h4254) = v0;
        h4254
    };
    let m4255 = v1;
    match m4255.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m4256 = v4;
            match m4256.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f597(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f596(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4257, _) = v0;
        h4257
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4258) = v0;
        h4258
    };
    let m4259 = v1;
    match m4259.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4260 = v4;
            match m4260.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f598(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f597(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a4265 = {
            let a4263 = {
                let a4261 = v0.clone();
                length_list(a4261)
            };
            let a4264 = {
                let a4262 = v1.clone();
                length_list(a4262)
            };
            nat_add(a4263, a4264)?
        };
        let a4266 = 1u64;
        nat_add(a4265, a4266)?
    };
    Ok(f416({
        let (_, h4267) = {
            let (_, h4268) = {
                let (h4269, _) = f599(Fn24::F600, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h4269
            };
            h4268
        };
        h4267
    }))
}

pub fn f598(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4274 = {
            let a4272 = {
                let a4270 = v0.clone();
                length_list(a4270)
            };
            let a4273 = {
                let a4271 = v1.clone();
                length_list(a4271)
            };
            nat_add(a4272, a4273)?
        };
        let a4275 = 1u64;
        nat_add(a4274, a4275)?
    };
    Ok(f601({
        let (_, h4276) = {
            let (_, h4277) = {
                let (h4278, _) = f602(Fn25::F603, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4278
            };
            h4277
        };
        h4276
    }))
}

pub fn f599(v0: Fn24, p4283: u64, p4284: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4282 = (p4283, p4284);
    {
        let mut m4282 = m4282;
        loop {
            let step = {
                let (v1, v2) = m4282;
                let m4279 = v1;
                if m4279 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4279 - 1;
                    let m4281 = {
                        let c4280: Fn24 = v0.clone();
                        c4280.apply(v2.clone())
                    };
                    match m4281 {
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
                Ok(next) => { m4282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f604(v0)
}

pub fn f601(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f318(Fn8::F605, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f602(v0: Fn25, p4289: u64, p4290: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4288 = (p4289, p4290);
    {
        let mut m4288 = m4288;
        loop {
            let step = {
                let (v1, v2) = m4288;
                let m4285 = v1;
                if m4285 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4285 - 1;
                    let m4287 = {
                        let c4286: Fn25 = v0.clone();
                        c4286.apply(v2.clone())
                    };
                    match m4287 {
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
                Ok(next) => { m4288 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f603(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f606(v0)
}

pub fn f604(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4291, _) = v0.clone();
        h4291
    };
    let v2: List<(Bytes, ())> = {
        let (h4292, _) = {
            let (_, h4293) = v0.clone();
            h4293
        };
        h4292
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4294) = {
            let (_, h4295) = v0;
            h4295
        };
        h4294
    };
    let m4296 = v1.clone();
    match m4296.uncons() {
        None => {
            let m4297 = v2;
            match m4297.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4298 = v2.clone();
            match m4298.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f607({
                        let (h4299, _) = v8.clone();
                        h4299
                    }, {
                        let (h4300, _) = v6.clone();
                        h4300
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

pub fn f605(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f606(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4301, _) = v0.clone();
        h4301
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4302, _) = {
            let (_, h4303) = v0.clone();
            h4303
        };
        h4302
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4304) = {
            let (_, h4305) = v0;
            h4305
        };
        h4304
    };
    let m4306 = v1.clone();
    match m4306.uncons() {
        None => {
            let m4307 = v2;
            match m4307.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4308 = v2.clone();
            match m4308.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f607({
                        let (h4309, _) = v8.clone();
                        h4309
                    }, {
                        let (h4310, _) = v6.clone();
                        h4310
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

pub fn f607(v0: Bytes, v1: Bytes) -> bool {
    let a4315 = {
        let a4311 = v0;
        let a4312 = v1;
        compare_bytes(a4311, a4312)
    };
    let a4316 = {
        let a4313 = Bytes::lit(&[]);
        let a4314 = Bytes::lit(&[0x01]);
        compare_bytes(a4313, a4314)
    };
    equal(a4315, a4316)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
