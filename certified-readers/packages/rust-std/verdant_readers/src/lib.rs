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
    C0(u64, Bytes, Bytes, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<Bytes>, (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes, std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C1(Bytes, Bytes, std::rc::Rc<Adt13>),
    C2(Bytes),
    C3(std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C4,
}

#[derive(Clone)]
pub enum Adt14 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt14>),
    C5(std::rc::Rc<Adt14>),
    C6,
    C7(std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
    C8(Bytes, std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, Adt13, Adt13, List<(Bytes, Bytes)>),
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
    F122(u64, Bytes),
    F123(Bytes),
    F325(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F122(k0, k1) => Ok(f122(*k0, k1.clone(), p0)),
            Fn3::F123(k0) => f123(k0.clone(), p0),
            Fn3::F325(k0) => f325(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F125(Bytes),
    F151(Bytes),
    F152(Bytes),
    F206(Bytes, Bytes),
    F207(Bytes),
    F208(Bytes, u64),
    F223(Bytes),
    F224(Bytes),
    F285(bool, u64, Bytes),
    F298(Bytes),
    F324(Bytes),
    F329(u64, Bytes),
    F415(u64, Bytes, u64),
    F418(Bytes),
    F419(Bytes),
    F423(Bytes),
    F424(Bytes),
    F448(Bytes),
    F454(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F125(k0) => f125(k0.clone(), p0),
            Fn4::F151(k0) => f151(k0.clone(), p0),
            Fn4::F152(k0) => f152(k0.clone(), p0),
            Fn4::F206(k0, k1) => f206(k0.clone(), k1.clone(), p0),
            Fn4::F207(k0) => f207(k0.clone(), p0),
            Fn4::F208(k0, k1) => f208(k0.clone(), *k1, p0),
            Fn4::F223(k0) => f223(k0.clone(), p0),
            Fn4::F224(k0) => f224(k0.clone(), p0),
            Fn4::F285(k0, k1, k2) => f285(*k0, *k1, k2.clone(), p0),
            Fn4::F298(k0) => f298(k0.clone(), p0),
            Fn4::F324(k0) => f324(k0.clone(), p0),
            Fn4::F329(k0, k1) => f329(*k0, k1.clone(), p0),
            Fn4::F415(k0, k1, k2) => f415(*k0, k1.clone(), *k2, p0),
            Fn4::F418(k0) => f418(k0.clone(), p0),
            Fn4::F419(k0) => f419(k0.clone(), p0),
            Fn4::F423(k0) => f423(k0.clone(), p0),
            Fn4::F424(k0) => f424(k0.clone(), p0),
            Fn4::F448(k0) => f448(k0.clone(), p0),
            Fn4::F454(k0) => f454(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F240,
    F466,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F240 => f240(p0, p1),
            Fn5::F466 => f466(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F243,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F243 => f243(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F306(Bytes),
    F315(bool, Bytes),
    F360(bool, Bytes),
    F376(Bytes),
    F386(Bytes),
    F414(u64, Bytes),
    F422(u64, Bytes),
    F426(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F306(k0) => f306(k0.clone(), p0),
            Fn7::F315(k0, k1) => f315(*k0, k1.clone(), p0),
            Fn7::F360(k0, k1) => f360(*k0, k1.clone(), p0),
            Fn7::F376(k0) => f376(k0.clone(), p0),
            Fn7::F386(k0) => f386(k0.clone(), p0),
            Fn7::F414(k0, k1) => f414(*k0, k1.clone(), p0),
            Fn7::F422(k0, k1) => f422(*k0, k1.clone(), p0),
            Fn7::F426(k0) => f426(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F384(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F384(k0, k1) => f384(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F397,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F397 => f397(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F399,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F399 => f399(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F430,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F430 => f430(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F459,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F459 => f459(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F465,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F465 => f465(p0),
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
                    let v7: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> = f65({
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

pub fn f52(v0: Bytes) -> R<Result<Adt13, Bytes>> {
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
                                                    Ok(Ok::<Adt13, Bytes>({
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

pub fn f53(v0: Adt13) -> Bytes {
    let m319 = v0;
    match m319 {
        Adt13::C0(v1, _, _) => {
            v1
        }
        Adt13::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt13::C2(_) => {
            Bytes::lit(&[])
        }
        Adt13::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt13::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f54(v0: Adt13) -> R<u64> {
    let m320 = v0;
    match m320 {
        Adt13::C0(_, _, r325) => {
            let v3 = (*r325).clone();
            let a321 = 1u64;
            let a322 = f54(v3)?;
            nat_add(a321, a322)
        }
        Adt13::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt13::C2(_) => {
            Ok(0u64)
        }
        Adt13::C3(r326, r327) => {
            let v8 = (*r326).clone();
            let v9 = (*r327).clone();
            let a323 = f54(v8)?;
            let a324 = f54(v9)?;
            nat_add(a323, a324)
        }
        Adt13::C4 => {
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
                Adt13::C0(v3, r341, r342) => {
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
                Adt13::C1(_, _, _) => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C2(_) => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C3(_, _) => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C4 => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f87(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f56(v0: Bytes) -> R<Result<Adt14, Bytes>> {
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
        Ok(Err::<Adt14, Bytes>(f81(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
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
            Ok(Err::<Adt14, Bytes>(f81(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
        } else {
            let a353 = f100(v2.clone(), v6)?;
            let a354 = 37u64;
            if nat_eq(a353, a354) {
                Ok(Err::<Adt14, Bytes>(f81(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v6)?))
            } else {
                let a355 = v3;
                let a356 = v6;
                if nat_le(a355, a356) {
                    Ok(Err::<Adt14, Bytes>(f81(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
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
                                    Ok(Ok::<Adt14, Bytes>({
                                        let (h362, _) = v8;
                                        h362
                                    }))
                                }
                                Err(v9) => {
                                    Ok(Err::<Adt14, Bytes>(v9))
                                }
                            }
                        }
                        Err(v7) => {
                            Ok(Err::<Adt14, Bytes>(v7))
                        }
                    }
                }
            }
        }
    }
}

pub fn f57(v0: Adt14) -> R<Bytes> {
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

pub fn f65(p401: u64, p402: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m400 = (p401, p402);
    {
        let mut m400 = m400;
        loop {
            let step = {
                let (v0, v1) = m400;
                let m397 = v0;
                if m397 == 0 {
                    Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                } else {
                    let v2 = m397 - 1;
                    let v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> = f107(v1.clone(), v1.clone());
                    let a398 = f108(v3.clone())?;
                    let a399 = f108(v1.clone())?;
                    if nat_eq(a398, a399) {
                        Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>((v2, v3))
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

pub fn f66(v0: List<Adt12>, v1: List<Adt12>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m403 = v0;
    match m403.uncons() {
        None => {
            Ok(List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil())
        }
        Some((v2, v3)) => {
            if f109(f110(v2.clone())) {
                let v6: (Bytes, (Bytes, (Bytes, List<Bytes>))) = {
                    let m404 = f111(v1.clone(), v2.clone())?;
                    match m404 {
                        Ok(v5) => {
                            (f112(v2), (Bytes::lit(&[]), ({
                                let (h405, _) = v5;
                                h405
                            }, {
                                let (_, h406) = v5;
                                h406
                            })))
                        }
                        Err(v4) => {
                            (f112(v2), (v4, (Bytes::lit(&[]), List::<Bytes>::nil())))
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

pub fn f67(p416: u64, p417: List<Bytes>, p418: List<Bytes>, v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<Bytes> {
    let m415 = (p416, (p417, p418));
    {
        let mut m415 = m415;
        loop {
            let step = {
                let (v0, (v1, v2)) = m415;
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
                                        if f35({
                                            let (h410, _) = {
                                                let (_, h411) = v7.clone();
                                                h411
                                            };
                                            h410
                                        }, Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f115({
                                                let (_, h412) = {
                                                    let (_, h413) = {
                                                        let (_, h414) = v7;
                                                        h414
                                                    };
                                                    h413
                                                };
                                                h412
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
                Ok(next) => { m415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt12>) -> List<Bytes> {
    let m419 = v0;
    match m419.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f110(v1.clone());
            let a420 = v3;
            let a421 = 0u64;
            if if nat_eq(a420, a421) {
                true
            } else {
                let a422 = v3;
                let a423 = 2u64;
                if nat_eq(a422, a423) {
                    true
                } else {
                    let a424 = v3;
                    let a425 = 3u64;
                    if nat_eq(a424, a425) {
                        true
                    } else {
                        let a426 = v3;
                        let a427 = 4u64;
                        if nat_eq(a426, a427) {
                            true
                        } else {
                            let a428 = v3;
                            let a429 = 5u64;
                            if nat_eq(a428, a429) {
                                true
                            } else {
                                let a430 = v3;
                                let a431 = 7u64;
                                if nat_eq(a430, a431) {
                                    true
                                } else {
                                    let a432 = v3;
                                    let a433 = 8u64;
                                    nat_eq(a432, a433)
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

pub fn f69(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m434 = v0;
    match m434.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f113({
                let (h435, _) = v2.clone();
                h435
            }, v1.clone()) {
                f35({
                    let (h436, _) = {
                        let (_, h437) = v2.clone();
                        h437
                    };
                    h436
                }, Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons(({
                    let (h438, _) = v2.clone();
                    h438
                }, ({
                    let (_, h439) = {
                        let (_, h440) = {
                            let (_, h441) = v2.clone();
                            h441
                        };
                        h440
                    };
                    h439
                }, {
                    let (h442, _) = {
                        let (_, h443) = {
                            let (_, h444) = v2;
                            h444
                        };
                        h443
                    };
                    h442
                })), f69(v3, v1))
            } else {
                f69(v3, v1)
            }
        }
    }
}

pub fn f70(p453: u64, p454: List<(Bytes, (List<Bytes>, Bytes))>, p455: List<Bytes>, p456: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m452 = (p453, (p454, (p455, p456)));
    {
        let mut m452 = m452;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m452;
                let m445 = v0;
                if m445 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m445 - 1;
                    let m446 = f116(v1.clone(), v2.clone());
                    match m446 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h447) = v5.clone();
                                h447
                            }, (List::cons({
                                let (h448, _) = {
                                    let (h449, _) = v5.clone();
                                    h449
                                };
                                h448
                            }, v2), List::cons({
                                let (_, h450) = {
                                    let (h451, _) = v5;
                                    h451
                                };
                                h450
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<Adt12>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v2: List<Adt12>) -> List<Bytes> {
    let m457 = v0;
    match m457.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f110(v3.clone());
            let a458 = v5;
            let a459 = 5u64;
            if if nat_eq(a458, a459) {
                true
            } else {
                let a460 = v5;
                let a461 = 7u64;
                nat_eq(a460, a461)
            } {
                let m464 = f117(v2.clone(), {
                    let a462 = v5;
                    let a463 = 5u64;
                    if nat_eq(a462, a463) {
                        0u64
                    } else {
                        2u64
                    }
                }, f112(v3.clone()));
                match m464 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f118(v5), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f112(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f71(v4, v1, v2))
                    }
                    Some(_) => {
                        f71(v4, v1, v2)
                    }
                }
            } else {
                let a465 = v5;
                let a466 = 8u64;
                if nat_eq(a465, a466) {
                    let m467 = f117(v2.clone(), 0u64, f112(v3.clone()));
                    match m467 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f119(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f112(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f71(v4, v1, v2))
                        }
                        Some(_) => {
                            f71(v4, v1, v2)
                        }
                    }
                } else {
                    if f109(v5) {
                        let m468 = f114(v1.clone(), f112(v3.clone()));
                        match m468 {
                            None => {
                                f71(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f35({
                                    let (h469, _) = {
                                        let (_, h470) = v8.clone();
                                        h470
                                    };
                                    h469
                                }, Bytes::lit(&[])) {
                                    f71(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f112(v3), List::cons(f8(Str::lit(": ")), List::cons({
                                        let (h471, _) = {
                                            let (_, h472) = v8;
                                            h472
                                        };
                                        h471
                                    }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f71(v4, v1, v2))
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
    let m473 = v0;
    match m473.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h474, _) = v1;
                h474
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f72(v2))
        }
    }
}

pub fn f73(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m475 = v0;
    match m475.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m476 = v3.clone();
            match m476.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a479 = v2;
                    let a480 = {
                        let a477 = v1.clone();
                        let a478 = f73(v3, v1);
                        append_bytes(a477, a478)
                    };
                    append_bytes(a479, a480)
                }
            }
        }
    }
}

pub fn f74(p483: List<Bytes>, p484: List<Bytes>) -> List<Bytes> {
    let m482 = (p483, p484);
    {
        let mut m482 = m482;
        loop {
            let step = {
                let (v0, v1) = m482;
                let m481 = v0;
                match m481.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a485 = v2;
    let a486 = 0u64;
    if nat_eq(a485, a486) {
        Ok(f120(v0, 0u64))
    } else {
        let (_, h487) = {
            let (h488, _) = f121(Fn3::F122(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h488
        };
        Ok(h487)
    }
}

pub fn f76() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f77(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m493 = {
        let a490 = v0;
        let a491 = v2;
        let a492 = {
            let a489 = v1.clone();
            length_bytes(a489)
        };
        slice_bytes(a490, a491, a492)
    };
    match m493 {
        None => {
            false
        }
        Some(v3) => {
            let a494 = v3;
            let a495 = v1;
            equal(a494, a495)
        }
    }
}

pub fn f78(v0: Bytes) -> R<Bytes> {
    let (_, h496) = {
        let (h497, _) = f121(Fn3::F123(v0.clone()), {
            let a499 = {
                let a498 = v0;
                length_bytes(a498)
            };
            let a500 = 1u64;
            nat_add(a499, a500)?
        }, (0u64, Bytes::lit(&[])))?;
        h497
    };
    Ok(h496)
}

pub fn f79(v0: Bytes) -> R<u64> {
    let (h501, _) = f124(Fn4::F125(v0.clone()), {
        let a503 = {
            let a502 = v0;
            length_bytes(a502)
        };
        let a504 = 1u64;
        nat_add(a503, a504)?
    }, 0u64)?;
    Ok(h501)
}

pub fn f80(v0: Bytes) -> Result<Adt13, Bytes> {
    Err::<Adt13, Bytes>(v0)
}

pub fn f81(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a505 = v0;
        utf8_encode(a505)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f82(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f84(v0.clone(), 0u64, Str::lit("<?xml")) {
        f126(f85(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m507 = f127(v0.clone(), {
            let a506 = Str::lit("?>");
            utf8_encode(a506)
        }, 5u64)?;
        match m507 {
            None => {
                Ok(f128(f81(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m509 = f127(f32(v0.clone(), 0u64, v1), {
                    let a508 = Str::lit("encoding");
                    utf8_encode(a508)
                }, 5u64)?;
                match m509 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a510 = v1;
                            let a511 = 2u64;
                            nat_add(a510, a511)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f129(v0.clone(), {
                            let a512 = v2;
                            let a513 = 8u64;
                            nat_add(a512, a513)?
                        })?;
                        let v4: u64 = f129(v0.clone(), {
                            let a514 = v3;
                            let a515 = 1u64;
                            nat_add(a514, a515)?
                        })?;
                        let _: u64 = f85(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m518 = f127(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a516 = v4;
                                let a517 = 1u64;
                                nat_add(a516, a517)?
                            })?;
                            match m518 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f130(f32(v0, {
                            let a519 = v4;
                            let a520 = 1u64;
                            nat_add(a519, a520)?
                        }, {
                            let a523 = v7;
                            let a524 = {
                                let a521 = v4;
                                let a522 = 1u64;
                                nat_add(a521, a522)?
                            };
                            nat_sub(a523, a524)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a525 = v1;
                                let a526 = 2u64;
                                nat_add(a525, a526)?
                            }))
                        } else {
                            Ok(f128(f81(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f83(v0: Bytes, p531: u64, p532: u64) -> R<Result<u64, Bytes>> {
    let m530 = (p531, p532);
    {
        let mut m530 = m530;
        loop {
            let step = {
                let (v1, v2) = m530;
                let m527 = v2;
                if m527 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f128(f81(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m527 - 1;
                    let v4: u64 = f129(v0.clone(), v1)?;
                    if f84(v0.clone(), v4, Str::lit("<!--")) {
                        let m528 = f131(v0.clone(), v4)?;
                        match m528 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f128(v5)))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v4, Str::lit("<?")) {
                            let m529 = f132(v0.clone(), v4)?;
                            match m529 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f128(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: u64, v2: Str) -> bool {
    f77(v0, {
        let a533 = v2;
        utf8_encode(a533)
    }, v1)
}

pub fn f85(v0: Bytes, v1: u64) -> R<u64> {
    f133(v0, v1)
}

pub fn f86(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt13, u64), Bytes>> {
    let m534 = v2;
    if m534 == 0 {
        Ok(f134(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m534 - 1;
        let a535 = v3;
        let a536 = 0u64;
        if nat_eq(a535, a536) {
            let v5: u64 = f135(v0.clone(), {
                let a537 = v1;
                let a538 = 1u64;
                nat_add(a537, a538)?
            })?;
            let a541 = v5;
            let a542 = {
                let a539 = v1;
                let a540 = 1u64;
                nat_add(a539, a540)?
            };
            if nat_eq(a541, a542) {
                Ok(f134(f81(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a543 = v1;
                    let a544 = 1u64;
                    nat_add(a543, a544)?
                }, {
                    let a547 = v5;
                    let a548 = {
                        let a545 = v1;
                        let a546 = 1u64;
                        nat_add(a545, a546)?
                    };
                    nat_sub(a547, a548)
                });
                let m549 = f136(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m549 {
                    Ok(v8) => {
                        if f84(v0.clone(), {
                            let (_, h550) = v8.clone();
                            h550
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
                                let (h551, _) = v8;
                                h551
                            }), std::rc::Rc::new(Adt13::C4)), {
                                let a553 = {
                                    let (_, h552) = v8;
                                    h552
                                };
                                let a554 = 2u64;
                                nat_add(a553, a554)?
                            })))
                        } else {
                            let m558 = f86(v0.clone(), {
                                let a556 = {
                                    let (_, h555) = v8;
                                    h555
                                };
                                let a557 = 1u64;
                                nat_add(a556, a557)?
                            }, v4, 1u64)?;
                            match m558 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a560 = {
                                            let (_, h559) = v10.clone();
                                            h559
                                        };
                                        let a561 = 2u64;
                                        nat_add(a560, a561)?
                                    };
                                    let v12: u64 = f135(v0.clone(), v11)?;
                                    if f35(f32(v0.clone(), v11, {
                                        let a562 = v12;
                                        let a563 = v11;
                                        nat_sub(a562, a563)
                                    }), v6.clone()) {
                                        let v13: u64 = f129(v0.clone(), v12)?;
                                        let a564 = f85(v0, v13)?;
                                        let a565 = 62u64;
                                        if nat_eq(a564, a565) {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
                                                let (h566, _) = v8;
                                                h566
                                            }), std::rc::Rc::new({
                                                let (h567, _) = v10;
                                                h567
                                            })), {
                                                let a568 = v13;
                                                let a569 = 1u64;
                                                nat_add(a568, a569)?
                                            })))
                                        } else {
                                            Ok(f134(f81(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f134(f81(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h570) = v10;
                                            h570
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f134(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f134(v7))
                    }
                }
            }
        } else {
            let a571 = f85(v0.clone(), v1)?;
            let a572 = 256u64;
            if nat_eq(a571, a572) {
                Ok(f134(f81(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f84(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v1)))
                } else {
                    if f84(v0.clone(), v1, Str::lit("<!--")) {
                        let m573 = f131(v0.clone(), v1)?;
                        match m573 {
                            Ok(v15) => {
                                f86(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f134(v14))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m577 = f127(v0.clone(), {
                                let a574 = Str::lit("]]>");
                                utf8_encode(a574)
                            }, {
                                let a575 = v1;
                                let a576 = 9u64;
                                nat_add(a575, a576)?
                            })?;
                            match m577 {
                                None => {
                                    Ok(f134(f81(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m580 = f86(v0.clone(), {
                                        let a578 = v16;
                                        let a579 = 3u64;
                                        nat_add(a578, a579)?
                                    }, v4, 1u64)?;
                                    match m580 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2(f32(v0, {
                                                let a581 = v1;
                                                let a582 = 9u64;
                                                nat_add(a581, a582)?
                                            }, {
                                                let a585 = v16;
                                                let a586 = {
                                                    let a583 = v1;
                                                    let a584 = 9u64;
                                                    nat_add(a583, a584)?
                                                };
                                                nat_sub(a585, a586)
                                            }))), std::rc::Rc::new({
                                                let (h587, _) = v18;
                                                h587
                                            })), {
                                                let (_, h588) = v18;
                                                h588
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f134(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f84(v0.clone(), v1, Str::lit("<?")) {
                                let m589 = f132(v0.clone(), v1)?;
                                match m589 {
                                    Ok(v20) => {
                                        f86(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f134(v19))
                                    }
                                }
                            } else {
                                if f84(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f134(f81(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a590 = f85(v0.clone(), v1)?;
                                    let a591 = 60u64;
                                    if nat_eq(a590, a591) {
                                        let m592 = f86(v0.clone(), v1, v4, 0u64)?;
                                        match m592 {
                                            Ok(v22) => {
                                                let m594 = f86(v0, {
                                                    let (_, h593) = v22;
                                                    h593
                                                }, v4, 1u64)?;
                                                match m594 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new({
                                                            let (h595, _) = v22;
                                                            h595
                                                        }), std::rc::Rc::new({
                                                            let (h596, _) = v24;
                                                            h596
                                                        })), {
                                                            let (_, h597) = v24;
                                                            h597
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f134(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f134(v21))
                                            }
                                        }
                                    } else {
                                        let m598 = f137(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m598 {
                                            Ok(v26) => {
                                                let m600 = f86(v0, {
                                                    let (_, h599) = v26;
                                                    h599
                                                }, v4, 1u64)?;
                                                match m600 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2({
                                                            let (h601, _) = v26;
                                                            h601
                                                        })), std::rc::Rc::new({
                                                            let (h602, _) = v28;
                                                            h602
                                                        })), {
                                                            let (_, h603) = v28;
                                                            h603
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f134(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f134(v25))
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

pub fn f88(v0: Adt13, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m604 = v0;
    match m604 {
        Adt13::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt13::C1(v2, v3, r609) => {
            let v4 = (*r609).clone();
            let m605 = f88(v4, v1);
            match m605 {
                Ok(v6) => {
                    if f35(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f35(v3.clone(), f139()) {
                            true
                        } else {
                            f35(v3.clone(), f140())
                        } {
                            f138(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f141(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a607 = {
                                    let a606 = v2;
                                    length_bytes(a606)
                                };
                                let a608 = 6u64;
                                nat_sub(a607, a608)
                            });
                            if f35(v7.clone(), f8(Str::lit("xmlns"))) {
                                f138(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f35(v7.clone(), f8(Str::lit("xml"))) {
                                    !f35(v3.clone(), f139())
                                } else {
                                    false
                                } {
                                    f138(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f35(v3.clone(), f139()) {
                                        true
                                    } else {
                                        f35(v3.clone(), f140())
                                    } {
                                        !f35(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f138(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f35(v3.clone(), Bytes::lit(&[])) {
                                            f138(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f138(v5)
                }
            }
        }
        Adt13::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt13::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt13::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f89(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f142(v1.clone())?;
    if f35(v2.clone(), Bytes::lit(&[])) {
        let m610 = f143(v0, Bytes::lit(&[]));
        match m610 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f35(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f139(), f144(v1)?)))
        } else {
            let m611 = f143(v0, v2.clone());
            match m611 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f144(v1)?)))
                }
            }
        }
    }
}

pub fn f90(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f35({
        let (h612, _) = v0;
        h612
    }, f145()) {
        f35({
            let (_, h613) = v0;
            h613
        }, {
            let a614 = v1;
            utf8_encode(a614)
        })
    } else {
        false
    }
}

pub fn f91(p618: Adt13, v1: Bytes) -> Option<Bytes> {
    let m617 = p618;
    {
        let mut m617 = m617;
        loop {
            let step = {
                let v0 = m617;
                let m615 = v0;
                match m615 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt13::C1(v2, v3, r616) => {
                        let v4 = (*r616).clone();
                        if f35(v2, v1.clone()) {
                            Err::<Adt13, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Bytes>>(v4)
                        }
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt13::C3(_, _) => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt13::C4 => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
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

pub fn f92(v0: Adt13, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m619 = v0;
    match m619 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C3(r630, r631) => {
            let v3 = (*r630).clone();
            let v4 = (*r631).clone();
            let m620 = f92(v4, v1.clone(), v2.clone())?;
            match m620 {
                Ok(v6) => {
                    let m621 = v3;
                    match m621 {
                        Adt13::C0(v7, r628, r629) => {
                            let v8 = (*r628).clone();
                            let v9 = (*r629).clone();
                            let m622 = f88(v8.clone(), v1);
                            match m622 {
                                Ok(v11) => {
                                    let m623 = f89(v11.clone(), v7)?;
                                    match m623 {
                                        Ok(v13) => {
                                            if f90(v13, Str::lit("simpleType")) {
                                                let m624 = f91(v8, f8(Str::lit("name")));
                                                match m624 {
                                                    None => {
                                                        Ok(f146(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m625 = f147(v14.clone(), v9, v11, v2)?;
                                                        match m625 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f148(v14)?, (match v16.clone() {
                                                                    Adt15::C0(_, h626) => {
                                                                        h626
                                                                    }
                                                                }, match v16 {
                                                                    Adt15::C0(h627, _) => {
                                                                        h627
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f146(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f146(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f146(v10))
                                }
                            }
                        }
                        Adt13::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt13::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt13::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt13::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(f146(v5))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f93(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m632 = v0;
    match m632.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h633, _) = v1;
                h633
            }, f93(v2))
        }
    }
}

pub fn f94(p636: List<Bytes>) -> Option<Bytes> {
    let m635 = p636;
    {
        let mut m635 = m635;
        loop {
            let step = {
                let v0 = m635;
                let m634 = v0;
                match m634.uncons() {
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
                Ok(next) => { m635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(p642: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m641 = p642;
    {
        let mut m641 = m641;
        loop {
            let step = {
                let v0 = m641;
                let m637 = v0;
                match m637.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m640 = f149({
                            let (h638, _) = {
                                let (_, h639) = v2;
                                h639
                            };
                            h638
                        }, v1.clone());
                        match m640 {
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
                Ok(next) => { m641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(p653: u64, p654: List<(Bytes, (List<Bytes>, Bytes))>, p655: List<Bytes>, p656: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m652 = (p653, (p654, (p655, p656)));
    {
        let mut m652 = m652;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m652;
                let m643 = v0;
                if m643 == 0 {
                    let m644 = v1;
                    match m644.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f150(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m643 - 1;
                    let m645 = v1.clone();
                    match m645.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m646 = f116(v1, v2.clone());
                            match m646 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f150(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h647) = v9.clone();
                                        h647
                                    }, (List::cons({
                                        let (h648, _) = {
                                            let (h649, _) = v9.clone();
                                            h649
                                        };
                                        h648
                                    }, v2), List::cons({
                                        let (_, h650) = {
                                            let (h651, _) = v9;
                                            h651
                                        };
                                        h650
                                    }, v3)))))
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

pub fn f97(v0: Bytes) -> R<u64> {
    let (h657, _) = f124(Fn4::F151(v0.clone()), {
        let a659 = {
            let a658 = v0;
            length_bytes(a658)
        };
        let a660 = 1u64;
        nat_add(a659, a660)?
    }, 0u64)?;
    Ok(h657)
}

pub fn f98(v0: Bytes, v1: u64) -> R<u64> {
    let (h661, _) = f124(Fn4::F152(v0.clone()), {
        let a663 = {
            let a662 = v0;
            length_bytes(a662)
        };
        let a664 = 1u64;
        nat_add(a663, a664)?
    }, v1)?;
    Ok(h661)
}

pub fn f99(v0: Bytes, v1: u64) -> R<bool> {
    if if if f84(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f84(v0.clone(), v1, Str::lit("..."))
    } {
        if f153(f100(v0.clone(), {
            let a665 = v1;
            let a666 = 3u64;
            nat_add(a665, a666)?
        })?) {
            true
        } else {
            f154(f100(v0.clone(), {
                let a667 = v1;
                let a668 = 3u64;
                nat_add(a667, a668)?
            })?)
        }
    } else {
        false
    } {
        let a669 = f155(v0, v1)?;
        let a670 = 0u64;
        Ok(nat_eq(a669, a670))
    } else {
        Ok(false)
    }
}

pub fn f100(v0: Bytes, v1: u64) -> R<u64> {
    f133(v0, v1)
}

pub fn f101(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Result<(Adt14, u64), Bytes>> {
    let m671 = v3;
    if m671 == 0 {
        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m671 - 1;
        let a672 = v4;
        let a673 = 0u64;
        if nat_eq(a672, a673) {
            let v9: u64 = f156(v0.clone(), v1)?;
            let v10: u64 = {
                let a674 = v9;
                let a675 = v1;
                nat_sub(a674, a675)
            };
            let a676 = f100(v0.clone(), v9)?;
            let a677 = 9u64;
            if nat_eq(a676, a677) {
                let v11: u64 = f157(v0.clone(), v9)?;
                if if f158(v0.clone(), v11)? {
                    true
                } else {
                    f159(v0.clone(), v11)?
                } {
                    Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m678 = f160(v0.clone(), v11)?;
                    match m678 {
                        None => {
                            f161(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v12))
                        }
                    }
                }
            } else {
                if f158(v0.clone(), v9)? {
                    Ok(f162(true, f101(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m679 = f160(v0.clone(), v9)?;
                    match m679 {
                        None => {
                            if f159(v0.clone(), v9)? {
                                Ok(f162(false, f101(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f161(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v13))
                        }
                    }
                }
            }
        } else {
            let a680 = v4;
            let a681 = 1u64;
            if nat_eq(a680, a681) {
                let v14: u64 = f157(v0.clone(), v1)?;
                let v15: u64 = f100(v0.clone(), v14)?;
                if if f154(v15) {
                    true
                } else {
                    let a682 = v15;
                    let a683 = 35u64;
                    if nat_eq(a682, a683) {
                        let a684 = v1;
                        let a685 = v14;
                        nat_lt(a684, a685)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f98(v0.clone(), f163(v0.clone(), v14)?)?;
                    let a687 = {
                        let a686 = v0.clone();
                        length_bytes(a686)
                    };
                    let a688 = v16;
                    if if nat_le(a687, a688) {
                        true
                    } else {
                        f99(v0.clone(), v16)?
                    } {
                        Ok(Err::<(Adt14, u64), Bytes>(f164(v14)?))
                    } else {
                        let v17: u64 = {
                            let a689 = f156(v0.clone(), v16)?;
                            let a690 = v16;
                            nat_sub(a689, a690)
                        };
                        let a691 = v2;
                        let a692 = v17;
                        if nat_le(a691, a692) {
                            f101(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f165(v5) {
                                let a693 = v17;
                                let a694 = v6;
                                nat_eq(a693, a694)
                            } else {
                                false
                            } {
                                f158(v0.clone(), {
                                    let a695 = v16;
                                    let a696 = v17;
                                    nat_add(a695, a696)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f162(true, f101(v0, {
                                    let a697 = v16;
                                    let a698 = v17;
                                    nat_add(a697, a698)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Err::<(Adt14, u64), Bytes>(f164(v14)?))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f158(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        Ok(f162(true, f101(v0.clone(), v14, v2, v8, 2u64, false, f155(v0, v14)?, List::<Bytes>::nil())?))
                    } else {
                        let m699 = f160(v0.clone(), v14)?;
                        match m699 {
                            None => {
                                if if v5 {
                                    f159(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f162(false, f101(v0.clone(), v14, v2, v8, 3u64, false, f155(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f161(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Err::<(Adt14, u64), Bytes>(v18))
                            }
                        }
                    }
                }
            } else {
                let a700 = v4;
                let a701 = 2u64;
                if nat_eq(a700, a701) {
                    let m706 = f101(v0.clone(), {
                        let a702 = v1;
                        let a703 = 1u64;
                        nat_add(a702, a703)?
                    }, {
                        let a704 = v6;
                        let a705 = 1u64;
                        nat_add(a704, a705)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m706 {
                        Ok(v20) => {
                            let v21: u64 = f98(v0.clone(), {
                                let (_, h707) = v20;
                                h707
                            })?;
                            let v22: u64 = {
                                let a708 = f156(v0.clone(), v21)?;
                                let a709 = v21;
                                nat_sub(a708, a709)
                            };
                            let a711 = {
                                let a710 = v0.clone();
                                length_bytes(a710)
                            };
                            let a712 = v21;
                            if nat_le(a711, a712) {
                                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                    let (h713, _) = v20;
                                    h713
                                }), std::rc::Rc::new(Adt14::C6)), v21)))
                            } else {
                                let a714 = v22;
                                let a715 = v6;
                                if if nat_eq(a714, a715) {
                                    f158(v0.clone(), {
                                        let a716 = v21;
                                        let a717 = v22;
                                        nat_add(a716, a717)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m720 = f101(v0, {
                                        let a718 = v21;
                                        let a719 = v22;
                                        nat_add(a718, a719)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m720 {
                                        Ok(v24) => {
                                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                                let (h721, _) = v20;
                                                h721
                                            }), std::rc::Rc::new({
                                                let (h722, _) = v24;
                                                h722
                                            })), {
                                                let (_, h723) = v24;
                                                h723
                                            })))
                                        }
                                        Err(v23) => {
                                            Ok(Err::<(Adt14, u64), Bytes>(v23))
                                        }
                                    }
                                } else {
                                    let a724 = v6;
                                    let a725 = v22;
                                    if nat_lt(a724, a725) {
                                        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a726 = v21;
                                            let a727 = v22;
                                            nat_add(a726, a727)?
                                        })?))
                                    } else {
                                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                            let (h728, _) = v20;
                                            h728
                                        }), std::rc::Rc::new(Adt14::C6)), v21)))
                                    }
                                }
                            }
                        }
                        Err(v19) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v19))
                        }
                    }
                } else {
                    let m729 = f160(v0.clone(), v1)?;
                    match m729 {
                        None => {
                            let m730 = f166(v0.clone(), v1)?;
                            match m730 {
                                None => {
                                    Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v26) => {
                                    let v27: Bytes = f167(v0.clone(), v1)?;
                                    if f168(v27.clone(), v7.clone()) {
                                        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m735 = f101(v0.clone(), {
                                            let a731 = v26;
                                            let a732 = 1u64;
                                            nat_add(a731, a732)?
                                        }, {
                                            let a733 = v6;
                                            let a734 = 1u64;
                                            nat_add(a733, a734)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m735 {
                                            Ok(v29) => {
                                                let v30: u64 = f98(v0.clone(), {
                                                    let (_, h736) = v29;
                                                    h736
                                                })?;
                                                let v31: u64 = {
                                                    let a737 = f156(v0.clone(), v30)?;
                                                    let a738 = v30;
                                                    nat_sub(a737, a738)
                                                };
                                                let a740 = {
                                                    let a739 = v0.clone();
                                                    length_bytes(a739)
                                                };
                                                let a741 = v30;
                                                if nat_le(a740, a741) {
                                                    Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8(v27, std::rc::Rc::new({
                                                        let (h742, _) = v29;
                                                        h742
                                                    }), std::rc::Rc::new(Adt14::C6)), v30)))
                                                } else {
                                                    let a743 = v31;
                                                    let a744 = v6;
                                                    if if if nat_eq(a743, a744) {
                                                        f165(f158(v0.clone(), {
                                                            let a745 = v30;
                                                            let a746 = v31;
                                                            nat_add(a745, a746)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f165(f99(v0.clone(), v30)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m749 = f101(v0, {
                                                            let a747 = v30;
                                                            let a748 = v31;
                                                            nat_add(a747, a748)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v27.clone(), v7))?;
                                                        match m749 {
                                                            Ok(v33) => {
                                                                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8(v27, std::rc::Rc::new({
                                                                    let (h750, _) = v29;
                                                                    h750
                                                                }), std::rc::Rc::new({
                                                                    let (h751, _) = v33;
                                                                    h751
                                                                })), {
                                                                    let (_, h752) = v33;
                                                                    h752
                                                                })))
                                                            }
                                                            Err(v32) => {
                                                                Ok(Err::<(Adt14, u64), Bytes>(v32))
                                                            }
                                                        }
                                                    } else {
                                                        let a753 = v6;
                                                        let a754 = v31;
                                                        if nat_lt(a753, a754) {
                                                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a755 = v30;
                                                                let a756 = v31;
                                                                nat_add(a755, a756)?
                                                            })?))
                                                        } else {
                                                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8(v27, std::rc::Rc::new({
                                                                let (h757, _) = v29;
                                                                h757
                                                            }), std::rc::Rc::new(Adt14::C6)), v30)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v28) => {
                                                Ok(Err::<(Adt14, u64), Bytes>(v28))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v25) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v25))
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
        let a758 = v0.clone();
        length_bytes(a758)
    };
    let a759 = v2;
    let a760 = v3;
    if if nat_lt(a759, a760) {
        if f84(v0.clone(), v2, Str::lit("...")) {
            f99(v0.clone(), v2)?
        } else {
            false
        }
    } else {
        false
    } {
        let m763 = f169(v0.clone(), {
            let a761 = v2;
            let a762 = 3u64;
            nat_add(a761, a762)?
        })?;
        match m763 {
            Ok(v5) => {
                let v6: u64 = f98(v0, v5)?;
                let a764 = v6;
                let a765 = v3;
                if nat_lt(a764, a765) {
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
        let a766 = v2;
        let a767 = v3;
        if if nat_lt(a766, a767) {
            f99(v0, v2)?
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v2)?))
        } else {
            let a768 = v2;
            let a769 = v3;
            if nat_lt(a768, a769) {
                Ok(Err::<u64, Bytes>(f81(Str::lit("only comments may follow the root node (YAML 1.2.2 [207])"), v2)?))
            } else {
                Ok(Ok::<u64, Bytes>(v2))
            }
        }
    }
}

pub fn f103(v0: Adt14) -> R<Bytes> {
    let m770 = v0;
    match m770 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f170(v2, v3)
        }
        Adt14::C2(v4, v5, v6) => {
            f171(v4, v5, v6)
        }
        Adt14::C3(v7) => {
            f172(v7)
        }
        Adt14::C4(r771) => {
            let v8 = (*r771).clone();
            Ok(f173(f174(v8)?))
        }
        Adt14::C5(r772) => {
            let v9 = (*r772).clone();
            f175(f176(v9)?)
        }
        Adt14::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt14::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt14::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f104(v0: Bytes, v1: u64) -> Option<u8> {
    let a773 = v0;
    let a774 = v1;
    index_bytes(a773, a774)
}

pub fn f105(v0: Bytes) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    f177({
        let a776 = {
            let a775 = v0.clone();
            length_bytes(a775)
        };
        let a777 = 1u64;
        nat_add(a776, a777)?
    }, v0, 0u64, List::<(u64, Bytes)>::nil())
}

pub fn f106(p785: u64, p786: List<(u64, Bytes)>, p787: List<Adt12>) -> R<Result<List<Adt12>, Bytes>> {
    let m784 = (p785, (p786, p787));
    {
        let mut m784 = m784;
        loop {
            let step = {
                let (v0, (v1, v2)) = m784;
                let m778 = v0;
                if m778 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m778 - 1;
                    let m779 = v1.clone();
                    match m779.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Ok::<List<Adt12>, Bytes>(f178(v2, List::<Adt12>::nil()))))
                        }
                        Some((_, _)) => {
                            let m780 = f179(v1)?;
                            match m780 {
                                Ok(v7) => {
                                    let m781 = f180(v3, v7)?;
                                    match m781 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>((v3, ({
                                                let (_, h782) = v9;
                                                h782
                                            }, List::cons({
                                                let (h783, _) = v9;
                                                h783
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m788 = v0;
    match m788.uncons() {
        None => {
            List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f35({
                let (h789, _) = {
                    let (_, h790) = v2.clone();
                    h790
                };
                h789
            }, Bytes::lit(&[])) {
                f181({
                    let (_, h791) = {
                        let (_, h792) = {
                            let (_, h793) = v2.clone();
                            h793
                        };
                        h792
                    };
                    h791
                }, v1.clone())
            } else {
                let (h794, _) = {
                    let (_, h795) = v2.clone();
                    h795
                };
                h794
            };
            List::cons(({
                let (h796, _) = v2;
                h796
            }, (v4, {
                let (_, h797) = {
                    let (_, h798) = v2;
                    h798
                };
                h797
            })), f107(v3, v1))
        }
    }
}

pub fn f108(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<u64> {
    let m799 = v0;
    match m799.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f35({
                let (h800, _) = {
                    let (_, h801) = v1;
                    h801
                };
                h800
            }, Bytes::lit(&[])) {
                f108(v2)
            } else {
                let a802 = f108(v2)?;
                let a803 = 1u64;
                nat_add(a802, a803)
            }
        }
    }
}

pub fn f109(v0: u64) -> bool {
    let a804 = v0;
    let a805 = 0u64;
    if nat_eq(a804, a805) {
        true
    } else {
        let a806 = v0;
        let a807 = 2u64;
        if nat_eq(a806, a807) {
            true
        } else {
            let a808 = v0;
            let a809 = 3u64;
            if nat_eq(a808, a809) {
                true
            } else {
                let a810 = v0;
                let a811 = 4u64;
                if nat_eq(a810, a811) {
                    true
                } else {
                    let a812 = v0;
                    let a813 = 9u64;
                    nat_eq(a812, a813)
                }
            }
        }
    }
}

pub fn f110(v0: Adt12) -> u64 {
    let m814 = v0;
    match m814 {
        Adt12::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f111(v0: List<Adt12>, v1: Adt12) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f110(v1.clone());
    let v3: Bytes = f148(f112(v1.clone()))?;
    if if f182(v3.clone()) {
        f109(v2)
    } else {
        false
    } {
        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f183(v3)))
    } else {
        let a815 = v2;
        let a816 = 0u64;
        if if nat_eq(a815, a816) {
            true
        } else {
            let a817 = v2;
            let a818 = 2u64;
            nat_eq(a817, a818)
        } {
            let m819 = f184(v0.clone(), v0.clone(), f112(v1.clone()));
            match m819 {
                None => {
                    let m823 = f185({
                        let a821 = {
                            let a820 = v0.clone();
                            length_list(a820)
                        };
                        let a822 = 1u64;
                        nat_add(a821, a822)?
                    }, v0, v1)?;
                    match m823 {
                        Ok(v5) => {
                            let a825 = {
                                let a824 = v5.clone();
                                length_list(a824)
                            };
                            let a826 = 0u64;
                            if nat_eq(a825, a826) {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
                            } else {
                                let m827 = f186(f187(f188(v5.clone()))?, v5)?;
                                match m827 {
                                    Ok(v7) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f189(v3, {
                                            let (h828, _) = v7;
                                            h828
                                        }), {
                                            let (_, h829) = v7;
                                            h829
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
            let a830 = v2;
            let a831 = 3u64;
            if nat_eq(a830, a831) {
                let m832 = f94(f190(v1.clone()));
                match m832 {
                    None => {
                        let m833 = f191(f187(f190(v1.clone()))?, f190(v1.clone()))?;
                        match m833 {
                            None => {
                                let m834 = f192(f187(f190(v1.clone()))?, f193(f190(v1)), List::<Bytes>::nil())?;
                                match m834 {
                                    Ok(v10) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f194(v3, v10), List::<Bytes>::nil())))
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v9))
                                    }
                                }
                            }
                            Some(v11) => {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f183(v11)))
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the enumeration value \"")), List::cons(v12, List::cons(f8(Str::lit("\" appears twice (Web IDL 2.9: the values must be distinct)")), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                let a835 = v2;
                let a836 = 4u64;
                if nat_eq(a835, a836) {
                    let v13: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f195(v1);
                    if !f35(f196(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f196(v13)))
                    } else {
                        let a837 = f197(v13.clone());
                        let a838 = 1u64;
                        if nat_eq(a837, a838) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f194(v3, List::cons(f198(v13.clone()), List::<Bytes>::nil())), f199(v13))))
                        } else {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f200(v3, f198(v13.clone())), f199(v13))))
                        }
                    }
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f190(v1))))
                }
            }
        }
    }
}

pub fn f112(v0: Adt12) -> Bytes {
    let m839 = v0;
    match m839 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f113(v0: Bytes, p842: List<Bytes>) -> bool {
    let m841 = p842;
    {
        let mut m841 = m841;
        loop {
            let step = {
                let v1 = m841;
                let m840 = v1;
                match m840.uncons() {
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
                Ok(next) => { m841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(p846: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: Bytes) -> Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m845 = p846;
    {
        let mut m845 = m845;
        loop {
            let step = {
                let v0 = m845;
                let m843 = v0;
                match m843.uncons() {
                    None => {
                        Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(None::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h844, _) = v2.clone();
                            h844
                        }, v1.clone()) {
                            Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(Some(v2))
                        } else {
                            Ok::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m847 = v0;
    match m847.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f115(v3, v1))
        }
    }
}

pub fn f116(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m848 = v0;
    match m848.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m851 = f149({
                let (h849, _) = {
                    let (_, h850) = v2.clone();
                    h850
                };
                h849
            }, v1.clone());
            match m851 {
                None => {
                    Some((({
                        let (h852, _) = v2;
                        h852
                    }, {
                        let (_, h853) = {
                            let (_, h854) = v2;
                            h854
                        };
                        h853
                    }), v3))
                }
                Some(_) => {
                    let m855 = f116(v3, v1);
                    match m855 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h856, _) = v5;
                                h856
                            }, List::cons(v2, {
                                let (_, h857) = v5;
                                h857
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f117(p862: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m861 = p862;
    {
        let mut m861 = m861;
        loop {
            let step = {
                let v0 = m861;
                let m858 = v0;
                match m858.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a859 = f110(v3.clone());
                        let a860 = v1;
                        if if nat_eq(a859, a860) {
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
                Ok(next) => { m861 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: u64) -> Bytes {
    let a863 = v0;
    let a864 = 5u64;
    if nat_eq(a863, a864) {
        f8(Str::lit("partial interface "))
    } else {
        let a865 = v0;
        let a866 = 7u64;
        if nat_eq(a865, a866) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f119(v0: Adt12) -> Bytes {
    let m867 = v0;
    match m867 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f120(v0: Bytes, v1: u64) -> Bytes {
    let m871 = {
        let a868 = v0;
        let a869 = v1;
        let a870 = 1u64;
        slice_bytes(a868, a869, a870)
    };
    match m871 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f121(v0: Fn3, p876: u64, p877: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m875 = (p876, p877);
    {
        let mut m875 = m875;
        loop {
            let step = {
                let (v1, v2) = m875;
                let m872 = v1;
                if m872 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m872 - 1;
                    let m874 = {
                        let c873: Fn3 = v0.clone();
                        c873.apply(v2.clone())?
                    };
                    match m874 {
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
                Ok(next) => { m875 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f201(v1, v0, v2)
}

pub fn f123(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f202(v0, v1)
}

pub fn f124(v0: Fn4, p882: u64, p883: u64) -> R<(u64, bool)> {
    let m881 = (p882, p883);
    {
        let mut m881 = m881;
        loop {
            let step = {
                let (v1, v2) = m881;
                let m878 = v1;
                if m878 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m878 - 1;
                    let m880 = {
                        let c879: Fn4 = v0.clone();
                        c879.apply(v2)?
                    };
                    match m880 {
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
                Ok(next) => { m881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f203(v0, v1)
}

pub fn f126(v0: u64) -> R<bool> {
    f204(v0, f205())
}

pub fn f127(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h884, _) = f124(Fn4::F206(v1.clone(), v0.clone()), {
            let a886 = {
                let a885 = v0.clone();
                length_bytes(a885)
            };
            let a887 = 1u64;
            nat_add(a886, a887)?
        }, v2)?;
        h884
    };
    if f77(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f128(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f129(v0: Bytes, v1: u64) -> R<u64> {
    let (h888, _) = f124(Fn4::F207(v0.clone()), {
        let a890 = {
            let a889 = v0;
            length_bytes(a889)
        };
        let a891 = 1u64;
        nat_add(a890, a891)?
    }, v1)?;
    Ok(h888)
}

pub fn f130(v0: Bytes) -> bool {
    f168(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f131(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m895 = f127(v0.clone(), {
        let a892 = Str::lit("--");
        utf8_encode(a892)
    }, {
        let a893 = v1;
        let a894 = 4u64;
        nat_add(a893, a894)?
    })?;
    match m895 {
        None => {
            Ok(f128(f81(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a898 = f85(v0, {
                let a896 = v2;
                let a897 = 2u64;
                nat_add(a896, a897)?
            })?;
            let a899 = 62u64;
            if nat_eq(a898, a899) {
                Ok(Ok::<u64, Bytes>({
                    let a900 = v2;
                    let a901 = 3u64;
                    nat_add(a900, a901)?
                }))
            } else {
                Ok(f128(f81(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f132(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f135(v0.clone(), {
        let a902 = v1;
        let a903 = 2u64;
        nat_add(a902, a903)?
    })?;
    let a906 = v2;
    let a907 = {
        let a904 = v1;
        let a905 = 2u64;
        nat_add(a904, a905)?
    };
    if nat_eq(a906, a907) {
        Ok(f128(f81(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a910 = v2;
        let a911 = {
            let a908 = v1;
            let a909 = 5u64;
            nat_add(a908, a909)?
        };
        if if nat_eq(a910, a911) {
            f168(f32(v0.clone(), {
                let a912 = v1;
                let a913 = 2u64;
                nat_add(a912, a913)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f128(f81(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m915 = f127(v0, {
                let a914 = Str::lit("?>");
                utf8_encode(a914)
            }, v2)?;
            match m915 {
                None => {
                    Ok(f128(f81(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a916 = v3;
                        let a917 = 2u64;
                        nat_add(a916, a917)?
                    }))
                }
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: u64) -> R<u64> {
    let m920 = {
        let a918 = v0;
        let a919 = v1;
        index_bytes(a918, a919)
    };
    match m920 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a965 = {
                let a941 = {
                    let a929 = {
                        let a923 = {
                            let a921 = v3;
                            let a922 = 128u8;
                            fixed_u8::bit_and(a921, a922)
                        };
                        let a924 = 128u8;
                        if equal(a923, a924) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a930 = {
                        let a927 = {
                            let a925 = v3;
                            let a926 = 64u8;
                            fixed_u8::bit_and(a925, a926)
                        };
                        let a928 = 64u8;
                        if equal(a927, a928) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a929, a930)?
                };
                let a942 = {
                    let a939 = {
                        let a933 = {
                            let a931 = v3;
                            let a932 = 32u8;
                            fixed_u8::bit_and(a931, a932)
                        };
                        let a934 = 32u8;
                        if equal(a933, a934) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a940 = {
                        let a937 = {
                            let a935 = v3;
                            let a936 = 16u8;
                            fixed_u8::bit_and(a935, a936)
                        };
                        let a938 = 16u8;
                        if equal(a937, a938) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a939, a940)?
                };
                nat_add(a941, a942)?
            };
            let a966 = {
                let a963 = {
                    let a951 = {
                        let a945 = {
                            let a943 = v3;
                            let a944 = 8u8;
                            fixed_u8::bit_and(a943, a944)
                        };
                        let a946 = 8u8;
                        if equal(a945, a946) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a952 = {
                        let a949 = {
                            let a947 = v3;
                            let a948 = 4u8;
                            fixed_u8::bit_and(a947, a948)
                        };
                        let a950 = 4u8;
                        if equal(a949, a950) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a951, a952)?
                };
                let a964 = {
                    let a961 = {
                        let a955 = {
                            let a953 = v3;
                            let a954 = 2u8;
                            fixed_u8::bit_and(a953, a954)
                        };
                        let a956 = 2u8;
                        if equal(a955, a956) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a962 = {
                        let a959 = {
                            let a957 = v3;
                            let a958 = 1u8;
                            fixed_u8::bit_and(a957, a958)
                        };
                        let a960 = 1u8;
                        if equal(a959, a960) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a961, a962)?
                };
                nat_add(a963, a964)?
            };
            nat_add(a965, a966)
        }
    }
}

pub fn f134(v0: Bytes) -> Result<(Adt13, u64), Bytes> {
    Err::<(Adt13, u64), Bytes>(v0)
}

pub fn f135(v0: Bytes, v1: u64) -> R<u64> {
    let (h967, _) = f124(Fn4::F208(v0.clone(), v1), {
        let a969 = {
            let a968 = v0;
            length_bytes(a968)
        };
        let a970 = 1u64;
        nat_add(a969, a970)?
    }, v1)?;
    Ok(h967)
}

pub fn f136(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt13, u64), Bytes>> {
    let m971 = v2;
    if m971 == 0 {
        Ok(f134(f81(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m971 - 1;
        let v5: u64 = f129(v0.clone(), v1)?;
        let a972 = f85(v0.clone(), v5)?;
        let a973 = 62u64;
        if if nat_eq(a972, a973) {
            true
        } else {
            f84(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v5)))
        } else {
            let a974 = v5;
            let a975 = v1;
            if nat_eq(a974, a975) {
                Ok(f134(f81(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f135(v0.clone(), v5)?;
                let a976 = v6;
                let a977 = v5;
                if nat_eq(a976, a977) {
                    Ok(f134(f81(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a978 = v6;
                        let a979 = v5;
                        nat_sub(a978, a979)
                    });
                    if f168(v7.clone(), v3.clone()) {
                        Ok(f134(f81(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f129(v0.clone(), v6)?;
                        let a980 = f85(v0.clone(), v8)?;
                        let a981 = 61u64;
                        if nat_eq(a980, a981) {
                            let v9: u64 = f129(v0.clone(), {
                                let a982 = v8;
                                let a983 = 1u64;
                                nat_add(a982, a983)?
                            })?;
                            let v10: u64 = f85(v0.clone(), v9)?;
                            let a984 = v10;
                            let a985 = 34u64;
                            if if nat_eq(a984, a985) {
                                true
                            } else {
                                let a986 = v10;
                                let a987 = 39u64;
                                nat_eq(a986, a987)
                            } {
                                let m990 = f137(v0.clone(), {
                                    let a988 = v9;
                                    let a989 = 1u64;
                                    nat_add(a988, a989)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m990 {
                                    Ok(v12) => {
                                        let m992 = f136(v0, {
                                            let (_, h991) = v12;
                                            h991
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m992 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C1(v7, {
                                                    let (h993, _) = v12;
                                                    h993
                                                }, std::rc::Rc::new({
                                                    let (h994, _) = v14;
                                                    h994
                                                })), {
                                                    let (_, h995) = v14;
                                                    h995
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f134(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f134(v11))
                                    }
                                }
                            } else {
                                Ok(f134(f81(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f134(f81(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f137(v0: Bytes, p1035: u64, p1036: u64, p1037: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1034 = (p1035, (p1036, p1037));
    {
        let mut m1034 = m1034;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1034;
                let m996 = v2;
                if m996 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m996 - 1;
                    let v7: u64 = f85(v0.clone(), v1)?;
                    if if v4 {
                        let a997 = v7;
                        let a998 = v5;
                        nat_eq(a997, a998)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a999 = v1;
                            let a1000 = 1u64;
                            nat_add(a999, a1000)?
                        }))))
                    } else {
                        let a1001 = v7;
                        let a1002 = 256u64;
                        if nat_eq(a1001, a1002) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1003 = v7;
                            let a1004 = 60u64;
                            if nat_eq(a1003, a1004) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1005 = v7;
                                let a1006 = 38u64;
                                if nat_eq(a1005, a1006) {
                                    let m1007 = f210(v0.clone(), v1)?;
                                    match m1007 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1008) = v9;
                                                h1008
                                            }, (v6, {
                                                let a1010 = v3;
                                                let a1011 = {
                                                    let (h1009, _) = v9;
                                                    h1009
                                                };
                                                append_bytes(a1010, a1011)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1012 = v7;
                                        let a1013 = 9u64;
                                        if if nat_eq(a1012, a1013) {
                                            true
                                        } else {
                                            let a1014 = v7;
                                            let a1015 = 10u64;
                                            nat_eq(a1014, a1015)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1016 = v1;
                                                let a1017 = 1u64;
                                                nat_add(a1016, a1017)?
                                            }, (v6, {
                                                let a1018 = v3;
                                                let a1019 = f8(Str::lit(" "));
                                                append_bytes(a1018, a1019)
                                            })))
                                        } else {
                                            let v10: u64 = f211(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1022 = v3;
                                                let a1023 = f32(v0.clone(), v1, {
                                                    let a1020 = v10;
                                                    let a1021 = v1;
                                                    nat_sub(a1020, a1021)
                                                });
                                                append_bytes(a1022, a1023)
                                            })))
                                        }
                                    } else {
                                        let a1024 = v7;
                                        let a1025 = 93u64;
                                        if nat_eq(a1024, a1025) {
                                            if f84(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1026 = v1;
                                                    let a1027 = 1u64;
                                                    nat_add(a1026, a1027)?
                                                }, (v6, {
                                                    let a1028 = v3;
                                                    let a1029 = f8(Str::lit("]"));
                                                    append_bytes(a1028, a1029)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f211(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1032 = v3;
                                                let a1033 = f32(v0.clone(), v1, {
                                                    let a1030 = v11;
                                                    let a1031 = v1;
                                                    nat_sub(a1030, a1031)
                                                });
                                                append_bytes(a1032, a1033)
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
                Ok(next) => { m1034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f139() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f140() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f141(v0: Bytes, v1: Bytes) -> bool {
    let m1042 = {
        let a1039 = v0;
        let a1040 = 0u64;
        let a1041 = {
            let a1038 = v1.clone();
            length_bytes(a1038)
        };
        slice_bytes(a1039, a1040, a1041)
    };
    match m1042 {
        None => {
            false
        }
        Some(v2) => {
            let a1043 = v2;
            let a1044 = v1;
            equal(a1043, a1044)
        }
    }
}

pub fn f142(v0: Bytes) -> R<Bytes> {
    let m1045 = f127(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1045 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f143(p1050: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1049 = p1050;
    {
        let mut m1049 = m1049;
        loop {
            let step = {
                let v0 = m1049;
                let m1046 = v0;
                match m1046.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1047, _) = v2;
                            h1047
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1048) = v2;
                                h1048
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1049 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: Bytes) -> R<Bytes> {
    let m1051 = f127(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1051 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1052 = v1;
                let a1053 = 1u64;
                nat_add(a1052, a1053)?
            }, {
                let a1057 = {
                    let a1054 = v0;
                    length_bytes(a1054)
                };
                let a1058 = {
                    let a1055 = v1;
                    let a1056 = 1u64;
                    nat_add(a1055, a1056)?
                };
                nat_sub(a1057, a1058)
            }))
        }
    }
}

pub fn f145() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f146(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f147(v0: Bytes, v1: Adt13, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt15, Bytes>> {
    let v4: Bytes = f148(v0.clone())?;
    let m1059 = f212(v1, v2)?;
    match m1059 {
        Ok(v6) => {
            let m1060 = v6;
            match m1060 {
                None => {
                    Ok(f213(f214(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt16::C0(h1061, _, _, _) => {
                            h1061
                        }
                    };
                    let v9: Adt13 = match v7.clone() {
                        Adt16::C0(_, h1062, _, _) => {
                            h1062
                        }
                    };
                    let v10: Adt13 = match v7.clone() {
                        Adt16::C0(_, _, h1063, _) => {
                            h1063
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt16::C0(_, _, _, h1064) => {
                            h1064
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f213(f214(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m1065 = f91(v9, f8(Str::lit("itemType")));
                            match m1065 {
                                None => {
                                    Ok(f213(f214(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1066 = f215(v11, v3, v12)?;
                                    match m1066 {
                                        Ok(v14) => {
                                            if f141(match v14.clone() {
                                                Adt15::C0(h1067, _) => {
                                                    h1067
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f213(f214(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt15, Bytes>(Adt15::C0(f200(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt15::C0(h1068, _) => {
                                                        h1068
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt15::C0(_, h1069) => {
                                                        h1069
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f213(f214(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1070 = f91(v9, f8(Str::lit("base")));
                            match m1070 {
                                None => {
                                    Ok(f213(f214(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1071 = f215(v11.clone(), v3, v15)?;
                                    match m1071 {
                                        Ok(v17) => {
                                            let m1072 = f216(v10, v11)?;
                                            match m1072 {
                                                Ok(v19) => {
                                                    let v20: List<Adt14> = f217(v19, List::<Bytes>::nil())?;
                                                    let m1073 = v20.clone();
                                                    match m1073.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt15, Bytes>(Adt15::C0(f200(v4, match v17.clone() {
                                                                Adt15::C0(h1074, _) => {
                                                                    h1074
                                                                }
                                                            }), match v17 {
                                                                Adt15::C0(_, h1075) => {
                                                                    h1075
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1076 = f192(f187(f218(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1076 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt15, Bytes>(Adt15::C0(f194(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f213(f214(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f213(f214(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f213(f214(v16, v0)))
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
            Ok(f213(f214(v5, v0)))
        }
    }
}

pub fn f148(v0: Bytes) -> R<Bytes> {
    f219(v0.clone(), 0u64, {
        let a1077 = v0;
        length_bytes(a1077)
    }, true, Bytes::lit(&[]))
}

pub fn f149(p1080: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1079 = p1080;
    {
        let mut m1079 = m1079;
        loop {
            let step = {
                let v0 = m1079;
                let m1078 = v0;
                match m1078.uncons() {
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
                Ok(next) => { m1079 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f151(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f220(v0, v1)
}

pub fn f152(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f221(v0, v1)
}

pub fn f153(v0: u64) -> bool {
    let a1081 = v0;
    let a1082 = 32u64;
    if nat_eq(a1081, a1082) {
        true
    } else {
        let a1083 = v0;
        let a1084 = 9u64;
        nat_eq(a1083, a1084)
    }
}

pub fn f154(v0: u64) -> bool {
    let a1085 = v0;
    let a1086 = 10u64;
    if nat_eq(a1085, a1086) {
        true
    } else {
        let a1087 = v0;
        let a1088 = 256u64;
        nat_eq(a1087, a1088)
    }
}

pub fn f155(v0: Bytes, v1: u64) -> R<u64> {
    let a1089 = v1;
    let a1090 = f222(v0, v1)?;
    Ok(nat_sub(a1089, a1090))
}

pub fn f156(v0: Bytes, v1: u64) -> R<u64> {
    let (h1091, _) = f124(Fn4::F223(v0.clone()), {
        let a1093 = {
            let a1092 = v0;
            length_bytes(a1092)
        };
        let a1094 = 1u64;
        nat_add(a1093, a1094)?
    }, v1)?;
    Ok(h1091)
}

pub fn f157(v0: Bytes, v1: u64) -> R<u64> {
    let (h1095, _) = f124(Fn4::F224(v0.clone()), {
        let a1097 = {
            let a1096 = v0;
            length_bytes(a1096)
        };
        let a1098 = 1u64;
        nat_add(a1097, a1098)?
    }, v1)?;
    Ok(h1095)
}

pub fn f158(v0: Bytes, v1: u64) -> R<bool> {
    let a1099 = f100(v0.clone(), v1)?;
    let a1100 = 45u64;
    if nat_eq(a1099, a1100) {
        if f153(f100(v0.clone(), {
            let a1101 = v1;
            let a1102 = 1u64;
            nat_add(a1101, a1102)?
        })?) {
            Ok(true)
        } else {
            Ok(f154(f100(v0, {
                let a1103 = v1;
                let a1104 = 1u64;
                nat_add(a1103, a1104)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f159(v0: Bytes, v1: u64) -> R<bool> {
    let m1105 = f166(v0, v1)?;
    match m1105 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f160(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f100(v0.clone(), v1)?;
    let v3: bool = f165(f225(f100(v0, {
        let a1106 = v1;
        let a1107 = 1u64;
        nat_add(a1106, a1107)?
    })?, true));
    let a1108 = v2;
    let a1109 = 58u64;
    if if nat_eq(a1108, a1109) {
        v3
    } else {
        false
    } {
        Ok(Some(f81(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1110 = v2;
        let a1111 = 38u64;
        if nat_eq(a1110, a1111) {
            Ok(Some(f81(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1112 = v2;
            let a1113 = 42u64;
            if nat_eq(a1112, a1113) {
                Ok(Some(f81(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1114 = v2;
                let a1115 = 33u64;
                if nat_eq(a1114, a1115) {
                    Ok(Some(f81(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1116 = v2;
                    let a1117 = 63u64;
                    if if nat_eq(a1116, a1117) {
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

pub fn f161(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt14, u64), Bytes>> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a1118 = v3;
    let a1119 = 124u64;
    if if nat_eq(a1118, a1119) {
        true
    } else {
        let a1120 = v3;
        let a1121 = 62u64;
        nat_eq(a1120, a1121)
    } {
        f226(v0, v1, v2)
    } else {
        let a1122 = v3;
        let a1123 = 34u64;
        if if nat_eq(a1122, a1123) {
            true
        } else {
            let a1124 = v3;
            let a1125 = 39u64;
            nat_eq(a1124, a1125)
        } {
            let m1126 = f227(v0.clone(), v1, v2, false)?;
            match m1126 {
                Ok(v5) => {
                    let m1128 = f169(v0, {
                        let (_, h1127) = v5;
                        h1127
                    })?;
                    match m1128 {
                        Ok(v7) => {
                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3({
                                let (h1129, _) = v5;
                                h1129
                            }), v7)))
                        }
                        Err(v6) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<(Adt14, u64), Bytes>(v4))
                }
            }
        } else {
            let a1130 = v3;
            let a1131 = 91u64;
            if if nat_eq(a1130, a1131) {
                true
            } else {
                let a1132 = v3;
                let a1133 = 123u64;
                nat_eq(a1132, a1133)
            } {
                let m1137 = f228(v0.clone(), v1, v2, {
                    let a1135 = {
                        let a1134 = v0.clone();
                        length_bytes(a1134)
                    };
                    let a1136 = 2u64;
                    nat_add(a1135, a1136)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1137 {
                    Ok(v9) => {
                        let a1139 = f100(v0.clone(), f157(v0.clone(), {
                            let (_, h1138) = v9.clone();
                            h1138
                        })?)?;
                        let a1140 = 58u64;
                        if nat_eq(a1139, a1140) {
                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1142 = f169(v0, {
                                let (_, h1141) = v9;
                                h1141
                            })?;
                            match m1142 {
                                Ok(v11) => {
                                    Ok(Ok::<(Adt14, u64), Bytes>(({
                                        let (h1143, _) = v9;
                                        h1143
                                    }, v11)))
                                }
                                Err(v10) => {
                                    Ok(Err::<(Adt14, u64), Bytes>(v10))
                                }
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(Err::<(Adt14, u64), Bytes>(v8))
                    }
                }
            } else {
                let m1144 = f160(v0.clone(), v1)?;
                match m1144 {
                    None => {
                        if f165(f229(v0.clone(), v1, false)?) {
                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v13: u64 = f230(v0.clone(), v1, false)?;
                            let m1148 = f231(v0.clone(), v13, v2, {
                                let a1146 = {
                                    let a1145 = v0.clone();
                                    length_bytes(a1145)
                                };
                                let a1147 = 2u64;
                                nat_add(a1146, a1147)?
                            }, f232(v0, v1, v13))?;
                            match m1148 {
                                Ok(v15) => {
                                    let m1150 = f233({
                                        let (h1149, _) = v15;
                                        h1149
                                    }, v1)?;
                                    match m1150 {
                                        Ok(v17) => {
                                            Ok(Ok::<(Adt14, u64), Bytes>((v17, {
                                                let (_, h1151) = v15;
                                                h1151
                                            })))
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Adt14, u64), Bytes>(v16))
                                        }
                                    }
                                }
                                Err(v14) => {
                                    Ok(Err::<(Adt14, u64), Bytes>(v14))
                                }
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Adt14, u64), Bytes>(v12))
                    }
                }
            }
        }
    }
}

pub fn f162(v0: bool, v1: Result<(Adt14, u64), Bytes>) -> Result<(Adt14, u64), Bytes> {
    let m1152 = v1;
    match m1152 {
        Ok(v3) => {
            Ok::<(Adt14, u64), Bytes>((if v0 {
                Adt14::C4(std::rc::Rc::new({
                    let (h1153, _) = v3;
                    h1153
                }))
            } else {
                Adt14::C5(std::rc::Rc::new({
                    let (h1154, _) = v3;
                    h1154
                }))
            }, {
                let (_, h1155) = v3;
                h1155
            }))
        }
        Err(v2) => {
            Err::<(Adt14, u64), Bytes>(v2)
        }
    }
}

pub fn f163(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f234(v0.clone(), v1)?;
    let a1157 = v2;
    let a1158 = {
        let a1156 = v0;
        length_bytes(a1156)
    };
    if nat_lt(a1157, a1158) {
        let a1159 = v2;
        let a1160 = 1u64;
        nat_add(a1159, a1160)
    } else {
        Ok(v2)
    }
}

pub fn f164(v0: u64) -> R<Bytes> {
    f81(Str::lit("a null node is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f165(v0: bool) -> bool {
    !v0
}

pub fn f166(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0.clone(), v1)?;
    let a1161 = v2;
    let a1162 = 34u64;
    if if nat_eq(a1161, a1162) {
        true
    } else {
        let a1163 = v2;
        let a1164 = 39u64;
        nat_eq(a1163, a1164)
    } {
        let m1165 = f227(v0.clone(), v1, 0u64, true)?;
        match m1165 {
            Ok(v4) => {
                let v5: u64 = f157(v0.clone(), {
                    let (_, h1166) = v4;
                    h1166
                })?;
                if f235(v0, v5)? {
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
        if f229(v0.clone(), v1, false)? {
            let v6: u64 = f157(v0.clone(), f230(v0.clone(), v1, false)?)?;
            if f235(v0, v6)? {
                Ok(Some(v6))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f167(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f100(v0.clone(), v1)?;
    let a1167 = v2;
    let a1168 = 34u64;
    if if nat_eq(a1167, a1168) {
        true
    } else {
        let a1169 = v2;
        let a1170 = 39u64;
        nat_eq(a1169, a1170)
    } {
        let m1171 = f227(v0, v1, 0u64, true)?;
        match m1171 {
            Ok(v4) => {
                let (h1172, _) = v4;
                Ok(h1172)
            }
            Err(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f232(v0.clone(), v1, f230(v0, v1, false)?))
    }
}

pub fn f168(v0: Bytes, p1175: List<Bytes>) -> bool {
    let m1174 = p1175;
    {
        let mut m1174 = m1174;
        loop {
            let step = {
                let v1 = m1174;
                let m1173 = v1;
                match m1173.uncons() {
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
                Ok(next) => { m1174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f157(v0.clone(), v1)?;
    let v3: u64 = f100(v0.clone(), v2)?;
    if if f154(v3) {
        true
    } else {
        let a1176 = v3;
        let a1177 = 35u64;
        if nat_eq(a1176, a1177) {
            let a1178 = v1;
            let a1179 = v2;
            nat_lt(a1178, a1179)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f163(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f81(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f170(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1180 = 0u64;
        let a1181 = v1;
        nat_lt(a1180, a1181)
    } else {
        false
    } {
        let a1182 = Bytes::lit(&[0x2d]);
        let a1183 = f51(v1)?;
        Ok(append_bytes(a1182, a1183))
    } else {
        f51(v1)
    }
}

pub fn f171(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f51(v1)?;
    let v4: u64 = {
        let a1184 = v3.clone();
        length_bytes(a1184)
    };
    let v5: Bytes = {
        let a1185 = v2;
        let a1186 = v4;
        if nat_lt(a1185, a1186) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1187 = v4;
                let a1188 = v2;
                nat_sub(a1187, a1188)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1189 = v4;
                let a1190 = v2;
                nat_sub(a1189, a1190)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f236({
                let a1191 = v2;
                let a1192 = v4;
                nat_sub(a1191, a1192)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1193 = f8(Str::lit("-"));
        let a1194 = v5;
        Ok(append_bytes(a1193, a1194))
    } else {
        Ok(v5)
    }
}

pub fn f172(v0: Bytes) -> R<Bytes> {
    let a1199 = f237(v0.clone(), 0u64, {
        let a1196 = {
            let a1195 = v0.clone();
            length_bytes(a1195)
        };
        let a1197 = 1u64;
        nat_add(a1196, a1197)?
    })?;
    let a1200 = {
        let a1198 = v0.clone();
        length_bytes(a1198)
    };
    if nat_eq(a1199, a1200) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f238(v0)
    }
}

pub fn f173(v0: List<Bytes>) -> Bytes {
    let a1203 = {
        let a1201 = Bytes::lit(&[0x5b]);
        let a1202 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1201, a1202)
    };
    let a1204 = Bytes::lit(&[0x5d]);
    append_bytes(a1203, a1204)
}

pub fn f174(v0: Adt14) -> R<List<Bytes>> {
    let m1205 = v0;
    match m1205 {
        Adt14::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C7(r1206, r1207) => {
            let v10 = (*r1206).clone();
            let v11 = (*r1207).clone();
            Ok(List::cons(f103(v10)?, f174(v11)?))
        }
        Adt14::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f175(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f239(Fn5::F240, List::<(Bytes, Bytes)>::nil(), f241(v0)?);
    let v2: List<Bytes> = f242(Fn6::F243, List::<Bytes>::nil(), v1)?;
    let a1210 = {
        let a1208 = Bytes::lit(&[0x7b]);
        let a1209 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1208, a1209)
    };
    let a1211 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1210, a1211))
}

pub fn f176(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1212 = v0;
    match m1212 {
        Adt14::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C8(v12, r1213, r1214) => {
            let v13 = (*r1213).clone();
            let v14 = (*r1214).clone();
            Ok(List::cons((v12, f103(v13)?), f176(v14)?))
        }
    }
}

pub fn f177(p1269: u64, v1: Bytes, p1270: u64, p1271: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    let m1268 = (p1269, (p1270, p1271));
    {
        let mut m1268 = m1268;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1268;
                let m1215 = v0;
                if m1215 == 0 {
                    Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f244(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1215 - 1;
                    let v5: u64 = f85(v1.clone(), v2)?;
                    let a1216 = v5;
                    let a1217 = 256u64;
                    if nat_eq(a1216, a1217) {
                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f245(v3, List::<(u64, Bytes)>::nil()))))
                    } else {
                        if f126(v5)? {
                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                let a1218 = v2;
                                let a1219 = 1u64;
                                nat_add(a1218, a1219)?
                            }, v3)))
                        } else {
                            if f84(v1.clone(), v2, Str::lit("//")) {
                                let m1220 = f127(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1220 {
                                    None => {
                                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f245(v3, List::<(u64, Bytes)>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                            let a1221 = v6;
                                            let a1222 = 1u64;
                                            nat_add(a1221, a1222)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f84(v1.clone(), v2, Str::lit("/*")) {
                                    let m1226 = f127(v1.clone(), {
                                        let a1223 = Str::lit("*/");
                                        utf8_encode(a1223)
                                    }, {
                                        let a1224 = v2;
                                        let a1225 = 2u64;
                                        nat_add(a1224, a1225)?
                                    })?;
                                    match m1226 {
                                        None => {
                                            Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f244(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                let a1227 = v7;
                                                let a1228 = 2u64;
                                                nat_add(a1227, a1228)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1229 = v5;
                                    let a1230 = 34u64;
                                    if nat_eq(a1229, a1230) {
                                        let m1233 = f127(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1231 = v2;
                                            let a1232 = 1u64;
                                            nat_add(a1231, a1232)?
                                        })?;
                                        match m1233 {
                                            None => {
                                                Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f244(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                    let a1234 = v8;
                                                    let a1235 = 1u64;
                                                    nat_add(a1234, a1235)?
                                                }, List::cons((3u64, f32(v1.clone(), {
                                                    let a1236 = v2;
                                                    let a1237 = 1u64;
                                                    nat_add(a1236, a1237)?
                                                }, {
                                                    let a1240 = v8;
                                                    let a1241 = {
                                                        let a1238 = v2;
                                                        let a1239 = 1u64;
                                                        nat_add(a1238, a1239)?
                                                    };
                                                    nat_sub(a1240, a1241)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1242 = v5;
                                            let a1243 = 95u64;
                                            if if nat_eq(a1242, a1243) {
                                                true
                                            } else {
                                                let a1244 = v5;
                                                let a1245 = 45u64;
                                                nat_eq(a1244, a1245)
                                            } {
                                                let a1246 = v2;
                                                let a1247 = 1u64;
                                                nat_add(a1246, a1247)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f246(f85(v1.clone(), v9)?) {
                                            let v10: u64 = f247(v1.clone(), 3u64, {
                                                let a1248 = v9;
                                                let a1249 = 1u64;
                                                nat_add(a1248, a1249)?
                                            })?;
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v10, List::cons((0u64, f32(v1.clone(), v2, {
                                                let a1250 = v10;
                                                let a1251 = v2;
                                                nat_sub(a1250, a1251)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1252 = v5;
                                                let a1253 = 45u64;
                                                if nat_eq(a1252, a1253) {
                                                    let a1254 = v2;
                                                    let a1255 = 1u64;
                                                    nat_add(a1254, a1255)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f248(v1.clone(), v11)?;
                                            let v13: u64 = f249(v1.clone(), v11)?;
                                            let a1256 = v12;
                                            let a1257 = v13;
                                            if nat_lt(a1256, a1257) {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v13, List::cons((2u64, f32(v1.clone(), v2, {
                                                    let a1258 = v13;
                                                    let a1259 = v2;
                                                    nat_sub(a1258, a1259)
                                                })), v3))))
                                            } else {
                                                let a1260 = v11;
                                                let a1261 = v12;
                                                if nat_lt(a1260, a1261) {
                                                    Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v12, List::cons((1u64, f32(v1.clone(), v2, {
                                                        let a1262 = v12;
                                                        let a1263 = v2;
                                                        nat_sub(a1262, a1263)
                                                    })), v3))))
                                                } else {
                                                    if f84(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1264 = v2;
                                                            let a1265 = 3u64;
                                                            nat_add(a1264, a1265)?
                                                        }, List::cons((4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1266 = v2;
                                                            let a1267 = 1u64;
                                                            nat_add(a1266, a1267)?
                                                        }, List::cons((4u64, f32(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(p1274: List<Adt12>, p1275: List<Adt12>) -> List<Adt12> {
    let m1273 = (p1274, p1275);
    {
        let mut m1273 = m1273;
        loop {
            let step = {
                let (v0, v1) = m1273;
                let m1272 = v0;
                match m1272.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    if f250(v0.clone(), Str::lit("[")) {
        Ok(f251({
            let a1277 = {
                let a1276 = v0.clone();
                length_list(a1276)
            };
            let a1278 = 1u64;
            nat_add(a1277, a1278)?
        }, f252(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<(u64, Bytes)>, Bytes>(v0))
    }
}

pub fn f180(v0: u64, v1: List<(u64, Bytes)>) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    if f250(v1.clone(), Str::lit("interface")) {
        if f250(f252(v1.clone()), Str::lit("mixin")) {
            f253(v0, 1u64, f252(f252(v1)), false)
        } else {
            f253(v0, 0u64, f252(v1), true)
        }
    } else {
        if f250(v1.clone(), Str::lit("partial")) {
            let v2: List<(u64, Bytes)> = f252(v1);
            if f250(v2.clone(), Str::lit("interface")) {
                if f250(f252(v2.clone()), Str::lit("mixin")) {
                    f253(v0, 6u64, f252(f252(v2)), false)
                } else {
                    f253(v0, 5u64, f252(v2), false)
                }
            } else {
                if f250(v2.clone(), Str::lit("dictionary")) {
                    f253(v0, 7u64, f252(v2), false)
                } else {
                    if f250(v2.clone(), Str::lit("namespace")) {
                        f254(f252(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(f255(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f250(v1.clone(), Str::lit("dictionary")) {
                f253(v0, 2u64, f252(v1), true)
            } else {
                if f250(v1.clone(), Str::lit("enum")) {
                    let m1279 = f256(f252(v1));
                    match m1279 {
                        Ok(v4) => {
                            let m1281 = f257({
                                let (_, h1280) = v4;
                                h1280
                            }, Str::lit("{"));
                            match m1281 {
                                Ok(v6) => {
                                    let m1282 = f258(v0, v6, List::<Bytes>::nil());
                                    match m1282 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(3u64, {
                                                let (h1283, _) = v4;
                                                h1283
                                            }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), {
                                                let (h1284, _) = v8;
                                                h1284
                                            }, f259(Bytes::lit(&[]))), {
                                                let (_, h1285) = v8;
                                                h1285
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v3))
                        }
                    }
                } else {
                    if f250(v1.clone(), Str::lit("typedef")) {
                        let m1286 = f260(v0, f252(v1), 0u64)?;
                        match m1286 {
                            Ok(v10) => {
                                let m1288 = f256({
                                    let (_, h1287) = v10;
                                    h1287
                                });
                                match m1288 {
                                    Ok(v12) => {
                                        let m1290 = f257({
                                            let (_, h1289) = v12;
                                            h1289
                                        }, Str::lit(";"));
                                        match m1290 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(4u64, {
                                                    let (h1291, _) = v12;
                                                    h1291
                                                }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), {
                                                    let (h1292, _) = v10;
                                                    h1292
                                                }), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f250(v1.clone(), Str::lit("callback")) {
                            if f250(f252(v1.clone()), Str::lit("interface")) {
                                f254(f252(f252(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f254(f252(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f250(v1.clone(), Str::lit("namespace")) {
                                f254(f252(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1293 = f256(v1);
                                match m1293 {
                                    Ok(v16) => {
                                        let m1295 = f257({
                                            let (_, h1294) = v16;
                                            h1294
                                        }, Str::lit("includes"));
                                        match m1295 {
                                            Ok(v18) => {
                                                let m1296 = f256(v18);
                                                match m1296 {
                                                    Ok(v20) => {
                                                        let m1298 = f257({
                                                            let (_, h1297) = v20;
                                                            h1297
                                                        }, Str::lit(";"));
                                                        match m1298 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(8u64, {
                                                                    let (h1299, _) = v16;
                                                                    h1299
                                                                }, {
                                                                    let (h1300, _) = v20;
                                                                    h1300
                                                                }, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), f259(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v15))
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

pub fn f181(p1308: List<Bytes>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> Bytes {
    let m1307 = p1308;
    {
        let mut m1307 = m1307;
        loop {
            let step = {
                let v0 = m1307;
                let m1301 = v0;
                match m1301.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1302 = f114(v1.clone(), v2.clone());
                        match m1302 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f35({
                                    let (h1303, _) = {
                                        let (_, h1304) = v4.clone();
                                        h1304
                                    };
                                    h1303
                                }, Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons({
                                        let (h1305, _) = {
                                            let (_, h1306) = v4;
                                            h1306
                                        };
                                        h1305
                                    }, List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes) -> bool {
    if f113(v0.clone(), f261()) {
        true
    } else {
        if f113(v0.clone(), f262()) {
            true
        } else {
            if f113(v0.clone(), f263()) {
                true
            } else {
                if f113(v0.clone(), f264()) {
                    true
                } else {
                    if f113(v0.clone(), f265()) {
                        true
                    } else {
                        if f113(v0.clone(), f266()) {
                            true
                        } else {
                            f113(v0, f267())
                        }
                    }
                }
            }
        }
    }
}

pub fn f183(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("the derived name ")), List::cons(v0, List::cons(f8(Str::lit(" is a reserved token of Lean 4.32.2, which LexLean 0.4.3 refuses as a name (lexlean 0.4.3 src/backend/lean_tokens.rs, SPEC.md 17.8)")), List::<Bytes>::nil()))))
}

pub fn f184(p1314: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
    let m1313 = p1314;
    {
        let mut m1313 = m1313;
        loop {
            let step = {
                let v0 = m1313;
                let m1309 = v0;
                match m1309.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1310 = f110(v3.clone());
                        let a1311 = 8u64;
                        if if nat_eq(a1310, a1311) {
                            f35(f112(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1312 = f117(v1.clone(), 1u64, f119(v3.clone()));
                            match m1312 {
                                None => {
                                    Err::<List<Adt12>, Option<Bytes>>(Some(f119(v3)))
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
                Ok(next) => { m1313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>> {
    let m1315 = v0;
    if m1315 == 0 {
        Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f112(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1315 - 1;
        let v4: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> = {
            let a1316 = f110(v2.clone());
            let a1317 = 2u64;
            if nat_eq(a1316, a1317) {
                f268(f269(f270(v2.clone()), f271(v1.clone(), 7u64, f112(v2.clone()))))?
            } else {
                f269(f270(v2.clone()), f269(f271(v1.clone(), 5u64, f112(v2.clone())), f272(v1.clone(), v1.clone(), f112(v2.clone()))))
            }
        };
        if f35(f119(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(v4))
        } else {
            let m1318 = f117(v1.clone(), f110(v2.clone()), f119(v2.clone()));
            match m1318 {
                None => {
                    Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f119(v2.clone()), List::cons({
                        let a1319 = f110(v2);
                        let a1320 = 2u64;
                        if nat_eq(a1319, a1320) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1321 = f185(v3, v1, v5)?;
                    match m1321 {
                        Ok(v7) => {
                            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f269(v7, v4)))
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

pub fn f186(v0: bool, v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1322 = v1;
    match m1322.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                let (h1323, _) = {
                    let (_, h1324) = v2.clone();
                    h1324
                };
                h1323
            };
            if !f35(f196(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                    let (h1325, _) = v2;
                    h1325
                }, List::cons(f8(Str::lit(": ")), List::cons(f196(v4), List::<Bytes>::nil())))))))
            } else {
                let m1327 = f273(v0, {
                    let (h1326, _) = v2.clone();
                    h1326
                })?;
                match m1327 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                            let (h1328, _) = v2;
                            h1328
                        }, List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f182(v5.clone()) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f183(v5)))
                        } else {
                            if f113(v5.clone(), f274(v0, f188(v3.clone()))?) {
                                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                            } else {
                                let m1329 = f186(v0, v3)?;
                                match m1329 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1332 = {
                                                let (_, h1330) = {
                                                    let (_, h1331) = v2;
                                                    h1331
                                                };
                                                h1330
                                            };
                                            let a1333 = 1u64;
                                            if nat_eq(a1332, a1333) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f198(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f198(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1334, _) = v7;
                                            h1334
                                        }), f115(f199(v4), {
                                            let (_, h1335) = v7;
                                            h1335
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

pub fn f187(p1339: List<Bytes>) -> R<bool> {
    let m1338 = p1339;
    {
        let mut m1338 = m1338;
        loop {
            let step = {
                let v0 = m1338;
                let m1336 = v0;
                match m1336.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1337 = f275(v1)?;
                        match m1337 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f113(v3, f274(false, v2.clone())?) {
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
                Ok(next) => { m1338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<Bytes> {
    let m1340 = v0;
    match m1340.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1341, _) = v1;
                h1341
            }, f188(v2))
        }
    }
}

pub fn f189(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f73(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f190(v0: Adt12) -> List<Bytes> {
    let m1342 = v0;
    match m1342 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f191(v0: bool, p1346: List<Bytes>) -> R<Option<Bytes>> {
    let m1345 = p1346;
    {
        let mut m1345 = m1345;
        loop {
            let step = {
                let v1 = m1345;
                let m1343 = v1;
                match m1343.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1344 = f273(v0, v2)?;
                        match m1344 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                            }
                            Some(v4) => {
                                if f182(v4.clone()) {
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
                Ok(next) => { m1345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: bool, v1: List<Adt14>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1347 = v1;
    match m1347.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1348 = f276(v0, v3.clone())?;
            match m1348 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f57(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f113(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1349 = f192(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1349 {
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

pub fn f193(v0: List<Bytes>) -> List<Adt14> {
    let m1350 = v0;
    match m1350.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt14::C3(v1), f193(v2))
        }
    }
}

pub fn f194(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f73(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f195(v0: Adt12) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let m1351 = v0;
    match m1351 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f196(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1352, _) = {
        let (_, h1353) = {
            let (_, h1354) = {
                let (_, h1355) = v0;
                h1355
            };
            h1354
        };
        h1353
    };
    h1352
}

pub fn f197(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> u64 {
    let (h1356, _) = v0;
    h1356
}

pub fn f198(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1357, _) = {
        let (_, h1358) = v0;
        h1358
    };
    h1357
}

pub fn f199(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> List<Bytes> {
    let (h1359, _) = {
        let (_, h1360) = {
            let (_, h1361) = v0;
            h1361
        };
        h1360
    };
    h1359
}

pub fn f200(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f201(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1362, _) = v2;
        h1362
    };
    let a1363 = v3;
    let a1364 = 0u64;
    if nat_eq(a1363, a1364) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1365 = v3;
            let a1366 = v1;
            let a1367 = 0u64;
            nat_quot(a1365, a1366, a1367)
        }, {
            let a1372 = f120(v0, {
                let a1368 = v3;
                let a1369 = v1;
                let a1370 = 0u64;
                nat_rem(a1368, a1369, a1370)
            });
            let a1373 = {
                let (_, h1371) = v2;
                h1371
            };
            append_bytes(a1372, a1373)
        }))
    }
}

pub fn f202(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1374, _) = v1;
        h1374
    };
    let a1376 = {
        let a1375 = v0.clone();
        length_bytes(a1375)
    };
    let a1377 = v2;
    if nat_le(a1376, a1377) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1378 = f85(v0.clone(), v2)?;
        let a1379 = 13u64;
        if nat_eq(a1378, a1379) {
            Ok(Some(({
                let a1382 = f85(v0, {
                    let a1380 = v2;
                    let a1381 = 1u64;
                    nat_add(a1380, a1381)?
                })?;
                let a1383 = 10u64;
                if nat_eq(a1382, a1383) {
                    let a1384 = v2;
                    let a1385 = 2u64;
                    nat_add(a1384, a1385)?
                } else {
                    let a1386 = v2;
                    let a1387 = 1u64;
                    nat_add(a1386, a1387)?
                }
            }, {
                let a1389 = {
                    let (_, h1388) = v1;
                    h1388
                };
                let a1390 = f8(Str::lit("\u{a}"));
                append_bytes(a1389, a1390)
            })))
        } else {
            let v4: u64 = {
                let m1391 = f127(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1391 {
                    None => {
                        let a1392 = v0.clone();
                        length_bytes(a1392)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1396 = {
                    let (_, h1393) = v1;
                    h1393
                };
                let a1397 = f32(v0, v2, {
                    let a1394 = v4;
                    let a1395 = v2;
                    nat_sub(a1394, a1395)
                });
                append_bytes(a1396, a1397)
            })))
        }
    }
}

pub fn f203(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1398 = f277(v0, v1)?;
    match m1398 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f278({
                let (h1399, _) = v2;
                h1399
            })? {
                Ok(Some({
                    let a1401 = v1;
                    let a1402 = {
                        let (_, h1400) = v2;
                        h1400
                    };
                    nat_add(a1401, a1402)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f204(v0: u64, v1: Bytes) -> R<bool> {
    f279(v0, v1.clone(), 0u64, {
        let a1404 = {
            let a1403 = v1;
            length_bytes(a1403)
        };
        let a1405 = 3u64;
        let a1406 = 0u64;
        nat_quot(a1404, a1405, a1406)
    }, true)
}

pub fn f205() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f206(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f280(v1, v0, v2)
}

pub fn f207(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f281(v0, v1)
}

pub fn f208(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f282(v0, v1, v2)
}

pub fn f209(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f210(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f84(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1409 = f85(v0.clone(), {
                let a1407 = v1;
                let a1408 = 2u64;
                nat_add(a1407, a1408)?
            })?;
            let a1410 = 120u64;
            nat_eq(a1409, a1410)
        };
        let v3: u64 = if v2 {
            let a1411 = v1;
            let a1412 = 3u64;
            nat_add(a1411, a1412)?
        } else {
            let a1413 = v1;
            let a1414 = 2u64;
            nat_add(a1413, a1414)?
        };
        let v4: (u64, u64) = f283(v0.clone(), v2, v3)?;
        let a1416 = {
            let (h1415, _) = v4;
            h1415
        };
        let a1417 = v3;
        if if nat_eq(a1416, a1417) {
            true
        } else {
            let a1419 = f85(v0, {
                let (h1418, _) = v4;
                h1418
            })?;
            let a1420 = 59u64;
            !nat_eq(a1419, a1420)
        } {
            Ok(f209(f81(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f278({
                let (_, h1421) = v4;
                h1421
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f284({
                    let (_, h1422) = v4;
                    h1422
                })?, {
                    let a1424 = {
                        let (h1423, _) = v4;
                        h1423
                    };
                    let a1425 = 1u64;
                    nat_add(a1424, a1425)?
                })))
            } else {
                Ok(f209(f81(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f84(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1426 = v1;
                let a1427 = 4u64;
                nat_add(a1426, a1427)?
            })))
        } else {
            if f84(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1428 = v1;
                    let a1429 = 4u64;
                    nat_add(a1428, a1429)?
                })))
            } else {
                if f84(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1430 = v1;
                        let a1431 = 5u64;
                        nat_add(a1430, a1431)?
                    })))
                } else {
                    if f84(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1432 = v1;
                            let a1433 = 6u64;
                            nat_add(a1432, a1433)?
                        })))
                    } else {
                        if f84(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1434 = v1;
                                let a1435 = 6u64;
                                nat_add(a1434, a1435)?
                            })))
                        } else {
                            Ok(f209(f81(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f211(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1436, _) = f124(Fn4::F285(v1, v2, v0.clone()), {
        let a1438 = {
            let a1437 = v0;
            length_bytes(a1437)
        };
        let a1439 = 1u64;
        nat_add(a1438, a1439)?
    }, v3)?;
    Ok(h1436)
}

pub fn f212(p1452: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt16>, Bytes>> {
    let m1451 = p1452;
    {
        let mut m1451 = m1451;
        loop {
            let step = {
                let v0 = m1451;
                let m1440 = v0;
                match m1440 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                    Adt13::C1(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                    Adt13::C3(r1449, r1450) => {
                        let v2 = (*r1449).clone();
                        let v3 = (*r1450).clone();
                        let m1441 = v2;
                        match m1441 {
                            Adt13::C0(v4, r1447, r1448) => {
                                let v5 = (*r1447).clone();
                                let v6 = (*r1448).clone();
                                let m1442 = f88(v5.clone(), v1.clone());
                                match m1442 {
                                    Ok(v8) => {
                                        let m1443 = f89(v8.clone(), v4)?;
                                        match m1443 {
                                            Ok(v10) => {
                                                if f168({
                                                    let (_, h1444) = v10.clone();
                                                    h1444
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1445, _) = v10;
                                                        h1445
                                                    }, f145()) {
                                                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(Some(Adt16::C0({
                                                            let (_, h1446) = v10;
                                                            h1446
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(f286(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(f286(v7)))
                                    }
                                }
                            }
                            Adt13::C1(_, _, _) => {
                                Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                            Adt13::C2(_) => {
                                Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                            Adt13::C3(_, _) => {
                                Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                            Adt13::C4 => {
                                Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt13::C4 => {
                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Bytes) -> Result<Adt15, Bytes> {
    Err::<Adt15, Bytes>(v0)
}

pub fn f214(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f215(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt15, Bytes>> {
    let m1453 = f89(v0, v2.clone())?;
    match m1453 {
        Ok(v4) => {
            if f35({
                let (h1454, _) = v4.clone();
                h1454
            }, f145()) {
                let m1456 = f287({
                    let (_, h1455) = v4;
                    h1455
                }, 16u64)?;
                match m1456 {
                    Ok(v6) => {
                        Ok(Ok::<Adt15, Bytes>(Adt15::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f213(v5))
                    }
                }
            } else {
                if f35({
                    let (h1457, _) = v4.clone();
                    h1457
                }, v1) {
                    let v7: Bytes = f148({
                        let (_, h1458) = v4;
                        h1458
                    })?;
                    Ok(Ok::<Adt15, Bytes>(Adt15::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f213(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1459, _) = v4;
                        h1459
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f213(v3))
        }
    }
}

pub fn f216(v0: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt14>, Bytes>> {
    let m1460 = v0;
    match m1460 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Adt13::C3(r1467, r1468) => {
            let v2 = (*r1467).clone();
            let v3 = (*r1468).clone();
            let m1461 = f216(v3, v1.clone())?;
            match m1461 {
                Ok(v5) => {
                    let m1462 = v2;
                    match m1462 {
                        Adt13::C0(v6, r1466, _) => {
                            let v7 = (*r1466).clone();
                            let m1463 = f88(v7.clone(), v1);
                            match m1463 {
                                Ok(v10) => {
                                    let m1464 = f89(v10, v6)?;
                                    match m1464 {
                                        Ok(v12) => {
                                            if f90(v12, Str::lit("enumeration")) {
                                                let m1465 = f91(v7, f8(Str::lit("value")));
                                                match m1465 {
                                                    None => {
                                                        Ok(f288(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt14>, Bytes>(List::cons(Adt14::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt14>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f288(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f288(v9))
                                }
                            }
                        }
                        Adt13::C1(_, _, _) => {
                            Ok(Ok::<List<Adt14>, Bytes>(v5))
                        }
                        Adt13::C2(_) => {
                            Ok(Ok::<List<Adt14>, Bytes>(v5))
                        }
                        Adt13::C3(_, _) => {
                            Ok(Ok::<List<Adt14>, Bytes>(v5))
                        }
                        Adt13::C4 => {
                            Ok(Ok::<List<Adt14>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f288(v4))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
    }
}

pub fn f217(v0: List<Adt14>, v1: List<Bytes>) -> R<List<Adt14>> {
    let m1469 = v0;
    match m1469.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f57(v2.clone())?;
            if f113(v4.clone(), v1.clone()) {
                f217(v3, v1)
            } else {
                Ok(List::cons(v2, f217(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f218(v0: List<Adt14>) -> List<Bytes> {
    let m1470 = v0;
    match m1470.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1471 = f289(Some(v1));
            match m1471 {
                None => {
                    f218(v2)
                }
                Some(v3) => {
                    List::cons(v3, f218(v2))
                }
            }
        }
    }
}

pub fn f219(v0: Bytes, p1492: u64, p1493: u64, p1494: bool, p1495: Bytes) -> R<Bytes> {
    let m1491 = (p1492, (p1493, (p1494, p1495)));
    {
        let mut m1491 = m1491;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1491;
                let m1472 = v2;
                if m1472 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1472 - 1;
                    let m1475 = {
                        let a1473 = v0.clone();
                        let a1474 = v1;
                        index_bytes(a1473, a1474)
                    };
                    match m1475 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f41(v6)?;
                            if f290(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1476 = v1;
                                    let a1477 = 1u64;
                                    nat_add(a1476, a1477)?
                                }, (v5, (true, v4))))
                            } else {
                                if f291(f292(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1478 = v1;
                                        let a1479 = 1u64;
                                        nat_add(a1478, a1479)?
                                    }, (v5, (false, {
                                        let a1483 = v4;
                                        let a1484 = f32({
                                            let a1480 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1480)
                                        }, {
                                            let a1481 = v7;
                                            let a1482 = 97u64;
                                            nat_sub(a1481, a1482)
                                        }, 1u64);
                                        append_bytes(a1483, a1484)
                                    }))))
                                } else {
                                    if f293(f292(v7), f293(f294(v7), f295(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1485 = v1;
                                            let a1486 = 1u64;
                                            nat_add(a1485, a1486)?
                                        }, (v5, (false, {
                                            let a1487 = v4;
                                            let a1488 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1487, a1488)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1489 = v1;
                                            let a1490 = 1u64;
                                            nat_add(a1489, a1490)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
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

pub fn f220(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1496 = f277(v0, v1)?;
    match m1496 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f296({
                let (h1497, _) = v2;
                h1497
            }) {
                Ok(Some({
                    let a1499 = v1;
                    let a1500 = {
                        let (_, h1498) = v2;
                        h1498
                    };
                    nat_add(a1499, a1500)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1502 = v1;
    let a1503 = {
        let a1501 = v0.clone();
        length_bytes(a1501)
    };
    if if nat_lt(a1502, a1503) {
        f297(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f163(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f222(v0: Bytes, v1: u64) -> R<u64> {
    let (h1504, _) = f124(Fn4::F298(v0), {
        let a1505 = v1;
        let a1506 = 1u64;
        nat_add(a1505, a1506)?
    }, v1)?;
    Ok(h1504)
}

pub fn f223(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f299(v0, v1)
}

pub fn f224(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f300(v0, v1)
}

pub fn f225(v0: u64, v1: bool) -> bool {
    f165(if if f153(v0) {
        true
    } else {
        f154(v0)
    } {
        true
    } else {
        if v1 {
            f301(v0)
        } else {
            false
        }
    })
}

pub fn f226(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt14, u64), Bytes>> {
    let v3: bool = {
        let a1507 = f100(v0.clone(), v1)?;
        let a1508 = 62u64;
        nat_eq(a1507, a1508)
    };
    let v4: u64 = f100(v0.clone(), {
        let a1509 = v1;
        let a1510 = 1u64;
        nat_add(a1509, a1510)?
    })?;
    let v5: u64 = f100(v0.clone(), {
        let a1511 = v1;
        let a1512 = 2u64;
        nat_add(a1511, a1512)?
    })?;
    let v6: u64 = f302(v4);
    let v7: u64 = f303(v4);
    let v8: bool = {
        let a1513 = 0u64;
        let a1514 = v6;
        if nat_lt(a1513, a1514) {
            true
        } else {
            let a1515 = 0u64;
            let a1516 = v7;
            nat_lt(a1515, a1516)
        }
    };
    let v9: u64 = {
        let a1517 = 0u64;
        let a1518 = v7;
        if nat_lt(a1517, a1518) {
            f302(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a1519 = 0u64;
        let a1520 = v6;
        if nat_lt(a1519, a1520) {
            f303(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a1521 = v6;
        let a1522 = v9;
        nat_add(a1521, a1522)?
    };
    let v12: u64 = {
        let a1523 = v7;
        let a1524 = v10;
        nat_add(a1523, a1524)?
    };
    let v13: u64 = {
        let a1533 = {
            let a1525 = v1;
            let a1526 = 1u64;
            nat_add(a1525, a1526)?
        };
        let a1534 = {
            let a1531 = if v8 {
                1u64
            } else {
                0u64
            };
            let a1532 = {
                let a1527 = 0u64;
                let a1528 = v9;
                if if nat_lt(a1527, a1528) {
                    true
                } else {
                    let a1529 = 0u64;
                    let a1530 = v10;
                    nat_lt(a1529, a1530)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1531, a1532)?
        };
        nat_add(a1533, a1534)?
    };
    let m1535 = f169(v0.clone(), v13)?;
    match m1535 {
        Ok(v15) => {
            let a1536 = 0u64;
            let a1537 = v12;
            if nat_lt(a1536, a1537) {
                f304(v0, v15, {
                    let a1540 = {
                        let a1538 = v2;
                        let a1539 = v12;
                        nat_add(a1538, a1539)?
                    };
                    let a1541 = 1u64;
                    nat_sub(a1540, a1541)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h1542, _) = f305(Fn7::F306(v0.clone()), {
                        let a1544 = {
                            let a1543 = v0.clone();
                            length_bytes(a1543)
                        };
                        let a1545 = 1u64;
                        nat_add(a1544, a1545)?
                    }, (v15, 0u64))?;
                    h1542
                };
                let v17: u64 = f156(v0.clone(), {
                    let (h1546, _) = v16;
                    h1546
                })?;
                let a1547 = f100(v0.clone(), v17)?;
                let a1548 = 256u64;
                if nat_eq(a1547, a1548) {
                    Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3({
                        let a1549 = v11;
                        let a1550 = 2u64;
                        if nat_eq(a1549, a1550) {
                            f307(f308(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a1551 = v0;
                        length_bytes(a1551)
                    })))
                } else {
                    let v18: u64 = f309({
                        let a1553 = v17;
                        let a1554 = {
                            let (h1552, _) = v16;
                            h1552
                        };
                        nat_sub(a1553, a1554)
                    }, v2);
                    let a1556 = v18;
                    let a1557 = {
                        let (_, h1555) = v16;
                        h1555
                    };
                    if nat_lt(a1556, a1557) {
                        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f304(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Err::<(Adt14, u64), Bytes>(v14))
        }
    }
}

pub fn f227(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<(Bytes, u64), Bytes>> {
    let a1558 = f100(v0.clone(), v1)?;
    let a1559 = 34u64;
    if nat_eq(a1558, a1559) {
        f310(v0.clone(), {
            let a1560 = v1;
            let a1561 = 1u64;
            nat_add(a1560, a1561)?
        }, v2, v3, {
            let a1563 = {
                let a1562 = v0;
                length_bytes(a1562)
            };
            let a1564 = 2u64;
            nat_add(a1563, a1564)?
        }, Bytes::lit(&[]))
    } else {
        f311(v0.clone(), {
            let a1565 = v1;
            let a1566 = 1u64;
            nat_add(a1565, a1566)?
        }, v2, v3, {
            let a1568 = {
                let a1567 = v0;
                length_bytes(a1567)
            };
            let a1569 = 2u64;
            nat_add(a1568, a1569)?
        }, Bytes::lit(&[]))
    }
}

pub fn f228(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Result<(Adt14, u64), Bytes>> {
    let m1570 = v3;
    if m1570 == 0 {
        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m1570 - 1;
        let a1571 = v4;
        let a1572 = 1u64;
        if nat_eq(a1571, a1572) {
            let m1573 = f228(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m1573 {
                Ok(v8) => {
                    let m1575 = f312(v0.clone(), {
                        let (_, h1574) = v8;
                        h1574
                    }, v2)?;
                    match m1575 {
                        Ok(v10) => {
                            let v11: u64 = f100(v0.clone(), v10)?;
                            let a1576 = v11;
                            let a1577 = 58u64;
                            if nat_eq(a1576, a1577) {
                                Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v10)?))
                            } else {
                                let a1578 = v11;
                                let a1579 = 93u64;
                                if nat_eq(a1578, a1579) {
                                    Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                        let (h1580, _) = v8;
                                        h1580
                                    }), std::rc::Rc::new(Adt14::C6)), {
                                        let a1581 = v10;
                                        let a1582 = 1u64;
                                        nat_add(a1581, a1582)?
                                    })))
                                } else {
                                    let a1583 = v11;
                                    let a1584 = 44u64;
                                    if nat_eq(a1583, a1584) {
                                        let m1587 = f312(v0.clone(), {
                                            let a1585 = v10;
                                            let a1586 = 1u64;
                                            nat_add(a1585, a1586)?
                                        }, v2)?;
                                        match m1587 {
                                            Ok(v13) => {
                                                let a1588 = f100(v0.clone(), v13)?;
                                                let a1589 = 93u64;
                                                if nat_eq(a1588, a1589) {
                                                    Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                                        let (h1590, _) = v8;
                                                        h1590
                                                    }), std::rc::Rc::new(Adt14::C6)), {
                                                        let a1591 = v13;
                                                        let a1592 = 1u64;
                                                        nat_add(a1591, a1592)?
                                                    })))
                                                } else {
                                                    let m1593 = f228(v0, v13, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m1593 {
                                                        Ok(v15) => {
                                                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                                                let (h1594, _) = v8;
                                                                h1594
                                                            }), std::rc::Rc::new({
                                                                let (h1595, _) = v15;
                                                                h1595
                                                            })), {
                                                                let (_, h1596) = v15;
                                                                h1596
                                                            })))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<(Adt14, u64), Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(Err::<(Adt14, u64), Bytes>(v12))
                                            }
                                        }
                                    } else {
                                        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v10)?))
                                    }
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt14, u64), Bytes>(v7))
                }
            }
        } else {
            let a1597 = v4;
            let a1598 = 2u64;
            if nat_eq(a1597, a1598) {
                let m1599 = f313(v0.clone(), v1, v2)?;
                match m1599 {
                    Ok(v17) => {
                        let m1601 = f312(v0.clone(), {
                            let (_, h1600) = v17.clone();
                            h1600
                        }, v2)?;
                        match m1601 {
                            Ok(v19) => {
                                if f165({
                                    let a1602 = f100(v0.clone(), v19)?;
                                    let a1603 = 58u64;
                                    nat_eq(a1602, a1603)
                                }) {
                                    Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v19)?))
                                } else {
                                    let m1606 = f312(v0.clone(), {
                                        let a1604 = v19;
                                        let a1605 = 1u64;
                                        nat_add(a1604, a1605)?
                                    }, v2)?;
                                    match m1606 {
                                        Ok(v21) => {
                                            let a1607 = f100(v0.clone(), v21)?;
                                            let a1608 = 44u64;
                                            if if nat_eq(a1607, a1608) {
                                                true
                                            } else {
                                                let a1609 = f100(v0.clone(), v21)?;
                                                let a1610 = 125u64;
                                                nat_eq(a1609, a1610)
                                            } {
                                                Ok(Err::<(Adt14, u64), Bytes>(f164(v21)?))
                                            } else {
                                                let m1611 = f228(v0.clone(), v21, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m1611 {
                                                    Ok(v23) => {
                                                        if f168({
                                                            let (h1612, _) = v17.clone();
                                                            h1612
                                                        }, v5.clone()) {
                                                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m1614 = f312(v0.clone(), {
                                                                let (_, h1613) = v23;
                                                                h1613
                                                            }, v2)?;
                                                            match m1614 {
                                                                Ok(v25) => {
                                                                    let v26: u64 = f100(v0.clone(), v25)?;
                                                                    let a1615 = v26;
                                                                    let a1616 = 125u64;
                                                                    if nat_eq(a1615, a1616) {
                                                                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8({
                                                                            let (h1617, _) = v17;
                                                                            h1617
                                                                        }, std::rc::Rc::new({
                                                                            let (h1618, _) = v23;
                                                                            h1618
                                                                        }), std::rc::Rc::new(Adt14::C6)), {
                                                                            let a1619 = v25;
                                                                            let a1620 = 1u64;
                                                                            nat_add(a1619, a1620)?
                                                                        })))
                                                                    } else {
                                                                        let a1621 = v26;
                                                                        let a1622 = 44u64;
                                                                        if nat_eq(a1621, a1622) {
                                                                            let m1625 = f312(v0.clone(), {
                                                                                let a1623 = v25;
                                                                                let a1624 = 1u64;
                                                                                nat_add(a1623, a1624)?
                                                                            }, v2)?;
                                                                            match m1625 {
                                                                                Ok(v28) => {
                                                                                    let a1626 = f100(v0.clone(), v28)?;
                                                                                    let a1627 = 125u64;
                                                                                    if nat_eq(a1626, a1627) {
                                                                                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8({
                                                                                            let (h1628, _) = v17;
                                                                                            h1628
                                                                                        }, std::rc::Rc::new({
                                                                                            let (h1629, _) = v23;
                                                                                            h1629
                                                                                        }), std::rc::Rc::new(Adt14::C6)), {
                                                                                            let a1630 = v28;
                                                                                            let a1631 = 1u64;
                                                                                            nat_add(a1630, a1631)?
                                                                                        })))
                                                                                    } else {
                                                                                        let m1633 = f228(v0, v28, v2, v6, 2u64, List::cons({
                                                                                            let (h1632, _) = v17.clone();
                                                                                            h1632
                                                                                        }, v5))?;
                                                                                        match m1633 {
                                                                                            Ok(v30) => {
                                                                                                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8({
                                                                                                    let (h1634, _) = v17;
                                                                                                    h1634
                                                                                                }, std::rc::Rc::new({
                                                                                                    let (h1635, _) = v23;
                                                                                                    h1635
                                                                                                }), std::rc::Rc::new({
                                                                                                    let (h1636, _) = v30;
                                                                                                    h1636
                                                                                                })), {
                                                                                                    let (_, h1637) = v30;
                                                                                                    h1637
                                                                                                })))
                                                                                            }
                                                                                            Err(v29) => {
                                                                                                Ok(Err::<(Adt14, u64), Bytes>(v29))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(v27) => {
                                                                                    Ok(Err::<(Adt14, u64), Bytes>(v27))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v25)?))
                                                                        }
                                                                    }
                                                                }
                                                                Err(v24) => {
                                                                    Ok(Err::<(Adt14, u64), Bytes>(v24))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Err(v22) => {
                                                        Ok(Err::<(Adt14, u64), Bytes>(v22))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v20) => {
                                            Ok(Err::<(Adt14, u64), Bytes>(v20))
                                        }
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<(Adt14, u64), Bytes>(v18))
                            }
                        }
                    }
                    Err(v16) => {
                        Ok(Err::<(Adt14, u64), Bytes>(v16))
                    }
                }
            } else {
                let v31: u64 = f100(v0.clone(), v1)?;
                let a1638 = v31;
                let a1639 = 91u64;
                if if nat_eq(a1638, a1639) {
                    true
                } else {
                    let a1640 = v31;
                    let a1641 = 123u64;
                    nat_eq(a1640, a1641)
                } {
                    let m1644 = f312(v0.clone(), {
                        let a1642 = v1;
                        let a1643 = 1u64;
                        nat_add(a1642, a1643)?
                    }, v2)?;
                    match m1644 {
                        Ok(v33) => {
                            let a1647 = f100(v0.clone(), v33)?;
                            let a1648 = {
                                let a1645 = v31;
                                let a1646 = 2u64;
                                nat_add(a1645, a1646)?
                            };
                            if nat_eq(a1647, a1648) {
                                Ok(Ok::<(Adt14, u64), Bytes>(({
                                    let a1649 = v31;
                                    let a1650 = 91u64;
                                    if nat_eq(a1649, a1650) {
                                        Adt14::C4(std::rc::Rc::new(Adt14::C6))
                                    } else {
                                        Adt14::C5(std::rc::Rc::new(Adt14::C6))
                                    }
                                }, {
                                    let a1651 = v33;
                                    let a1652 = 1u64;
                                    nat_add(a1651, a1652)?
                                })))
                            } else {
                                let m1655 = f228(v0, v33, v2, v6, {
                                    let a1653 = v31;
                                    let a1654 = 91u64;
                                    if nat_eq(a1653, a1654) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m1655 {
                                    Ok(v35) => {
                                        Ok(Ok::<(Adt14, u64), Bytes>(({
                                            let a1656 = v31;
                                            let a1657 = 91u64;
                                            if nat_eq(a1656, a1657) {
                                                Adt14::C4(std::rc::Rc::new({
                                                    let (h1658, _) = v35;
                                                    h1658
                                                }))
                                            } else {
                                                Adt14::C5(std::rc::Rc::new({
                                                    let (h1659, _) = v35;
                                                    h1659
                                                }))
                                            }
                                        }, {
                                            let (_, h1660) = v35;
                                            h1660
                                        })))
                                    }
                                    Err(v34) => {
                                        Ok(Err::<(Adt14, u64), Bytes>(v34))
                                    }
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v32))
                        }
                    }
                } else {
                    let a1661 = v31;
                    let a1662 = 34u64;
                    if if nat_eq(a1661, a1662) {
                        true
                    } else {
                        let a1663 = v31;
                        let a1664 = 39u64;
                        nat_eq(a1663, a1664)
                    } {
                        let m1665 = f227(v0, v1, v2, false)?;
                        match m1665 {
                            Ok(v37) => {
                                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3({
                                    let (h1666, _) = v37;
                                    h1666
                                }), {
                                    let (_, h1667) = v37;
                                    h1667
                                })))
                            }
                            Err(v36) => {
                                Ok(Err::<(Adt14, u64), Bytes>(v36))
                            }
                        }
                    } else {
                        let m1668 = f160(v0.clone(), v1)?;
                        match m1668 {
                            None => {
                                if f165(f229(v0.clone(), v1, true)?) {
                                    Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v39: u64 = f230(v0.clone(), v1, true)?;
                                    let v40: u64 = f157(v0.clone(), v39)?;
                                    let m1671 = {
                                        let a1669 = f100(v0.clone(), v40)?;
                                        let a1670 = 10u64;
                                        if nat_eq(a1669, a1670) {
                                            f312(v0.clone(), v40, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v40)
                                        }
                                    };
                                    match m1671 {
                                        Ok(v42) => {
                                            let a1672 = f100(v0.clone(), v40)?;
                                            let a1673 = 10u64;
                                            if if if nat_eq(a1672, a1673) {
                                                f165(f301(f100(v0.clone(), v42)?))
                                            } else {
                                                false
                                            } {
                                                f165({
                                                    let a1674 = f100(v0.clone(), v42)?;
                                                    let a1675 = 58u64;
                                                    nat_eq(a1674, a1675)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v40)?))
                                            } else {
                                                let m1676 = f233(f232(v0, v1, v39), v1)?;
                                                match m1676 {
                                                    Ok(v44) => {
                                                        Ok(Ok::<(Adt14, u64), Bytes>((v44, v39)))
                                                    }
                                                    Err(v43) => {
                                                        Ok(Err::<(Adt14, u64), Bytes>(v43))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v41) => {
                                            Ok(Err::<(Adt14, u64), Bytes>(v41))
                                        }
                                    }
                                }
                            }
                            Some(v38) => {
                                Ok(Err::<(Adt14, u64), Bytes>(v38))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f229(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f100(v0.clone(), v1)?;
    if if f153(v3) {
        true
    } else {
        f154(v3)
    } {
        Ok(false)
    } else {
        let a1677 = v3;
        let a1678 = 45u64;
        if if if nat_eq(a1677, a1678) {
            true
        } else {
            let a1679 = v3;
            let a1680 = 63u64;
            nat_eq(a1679, a1680)
        } {
            true
        } else {
            let a1681 = v3;
            let a1682 = 58u64;
            nat_eq(a1681, a1682)
        } {
            Ok(f225(f100(v0, {
                let a1683 = v1;
                let a1684 = 1u64;
                nat_add(a1683, a1684)?
            })?, v2))
        } else {
            Ok(f165(f314(v3)))
        }
    }
}

pub fn f230(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h1685) = {
        let (h1686, _) = f305(Fn7::F315(v2, v0.clone()), {
            let a1688 = {
                let a1687 = v0;
                length_bytes(a1687)
            };
            let a1689 = 1u64;
            nat_add(a1688, a1689)?
        }, (v1, v1))?;
        h1686
    };
    Ok(h1685)
}

pub fn f231(v0: Bytes, p1714: u64, v2: u64, p1715: u64, p1716: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m1713 = (p1714, (p1715, p1716));
    {
        let mut m1713 = m1713;
        loop {
            let step = {
                let (v1, (v3, v4)) = m1713;
                let m1690 = v3;
                if m1690 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m1690 - 1;
                    let v6: u64 = f157(v0.clone(), v1)?;
                    let v7: u64 = f100(v0.clone(), v6)?;
                    let a1691 = v7;
                    let a1692 = 58u64;
                    if nat_eq(a1691, a1692) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f316(v6)?)))
                    } else {
                        let a1693 = v7;
                        let a1694 = 35u64;
                        if if nat_eq(a1693, a1694) {
                            true
                        } else {
                            let a1695 = v7;
                            let a1696 = 256u64;
                            nat_eq(a1695, a1696)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, f163(v0.clone(), v6)?))))
                        } else {
                            let v8: (u64, u64) = f317(v0.clone(), {
                                let a1697 = v6;
                                let a1698 = 1u64;
                                nat_add(a1697, a1698)?
                            })?;
                            let v9: u64 = {
                                let (h1699, _) = v8;
                                h1699
                            };
                            let v10: u64 = f156(v0.clone(), v9)?;
                            let v11: u64 = f157(v0.clone(), v10)?;
                            let v12: u64 = f100(v0.clone(), v11)?;
                            let a1700 = v12;
                            let a1701 = 256u64;
                            if if if if nat_eq(a1700, a1701) {
                                true
                            } else {
                                let a1704 = {
                                    let a1702 = v10;
                                    let a1703 = v9;
                                    nat_sub(a1702, a1703)
                                };
                                let a1705 = v2;
                                nat_lt(a1704, a1705)
                            } {
                                true
                            } else {
                                let a1706 = v12;
                                let a1707 = 35u64;
                                nat_eq(a1706, a1707)
                            } {
                                true
                            } else {
                                f99(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, {
                                    let a1708 = v6;
                                    let a1709 = 1u64;
                                    nat_add(a1708, a1709)?
                                }))))
                            } else {
                                let v13: u64 = f230(v0.clone(), v11, false)?;
                                let a1710 = v13;
                                let a1711 = v11;
                                if nat_eq(a1710, a1711) {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f316(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v13, (v5, f7(List::cons(v4, List::cons(f318({
                                        let (_, h1712) = v8;
                                        h1712
                                    }), List::cons(f232(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a1717 = v2;
        let a1718 = v1;
        nat_sub(a1717, a1718)
    })
}

pub fn f233(v0: Bytes, v1: u64) -> R<Result<Adt14, Bytes>> {
    let a1720 = {
        let a1719 = v0.clone();
        length_bytes(a1719)
    };
    let a1721 = 0u64;
    if if if if nat_eq(a1720, a1721) {
        true
    } else {
        f319(v0.clone(), Str::lit("~"))
    } {
        true
    } else {
        if f319(v0.clone(), Str::lit("null")) {
            true
        } else {
            f319(v0.clone(), Str::lit("Null"))
        }
    } {
        true
    } else {
        f319(v0.clone(), Str::lit("NULL"))
    } {
        Ok(Err::<Adt14, Bytes>(f164(v1)?))
    } else {
        if if if f319(v0.clone(), Str::lit("true")) {
            true
        } else {
            f319(v0.clone(), Str::lit("True"))
        } {
            true
        } else {
            f319(v0.clone(), Str::lit("TRUE"))
        } {
            Ok(Ok::<Adt14, Bytes>(Adt14::C0(true)))
        } else {
            if if if f319(v0.clone(), Str::lit("false")) {
                true
            } else {
                f319(v0.clone(), Str::lit("False"))
            } {
                true
            } else {
                f319(v0.clone(), Str::lit("FALSE"))
            } {
                Ok(Ok::<Adt14, Bytes>(Adt14::C0(false)))
            } else {
                if if if if if f319(v0.clone(), Str::lit(".inf")) {
                    true
                } else {
                    f319(v0.clone(), Str::lit(".Inf"))
                } {
                    true
                } else {
                    if f319(v0.clone(), Str::lit(".INF")) {
                        true
                    } else {
                        f319(v0.clone(), Str::lit("+.inf"))
                    }
                } {
                    true
                } else {
                    if if f319(v0.clone(), Str::lit("+.Inf")) {
                        true
                    } else {
                        f319(v0.clone(), Str::lit("+.INF"))
                    } {
                        true
                    } else {
                        if f319(v0.clone(), Str::lit("-.inf")) {
                            true
                        } else {
                            f319(v0.clone(), Str::lit("-.Inf"))
                        }
                    }
                } {
                    true
                } else {
                    if if f319(v0.clone(), Str::lit("-.INF")) {
                        true
                    } else {
                        f319(v0.clone(), Str::lit(".nan"))
                    } {
                        true
                    } else {
                        if f319(v0.clone(), Str::lit(".NaN")) {
                            true
                        } else {
                            f319(v0.clone(), Str::lit(".NAN"))
                        }
                    }
                } {
                    Ok(Err::<Adt14, Bytes>(f81(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f84(v0.clone(), 0u64, Str::lit("0o")) {
                        let a1723 = 2u64;
                        let a1724 = {
                            let a1722 = v0.clone();
                            length_bytes(a1722)
                        };
                        nat_lt(a1723, a1724)
                    } else {
                        false
                    } {
                        f320(v0, 8u64, v1)
                    } else {
                        if if f84(v0.clone(), 0u64, Str::lit("0x")) {
                            let a1726 = 2u64;
                            let a1727 = {
                                let a1725 = v0.clone();
                                length_bytes(a1725)
                            };
                            nat_lt(a1726, a1727)
                        } else {
                            false
                        } {
                            f320(v0, 16u64, v1)
                        } else {
                            let m1728 = f321(v0.clone())?;
                            match m1728 {
                                None => {
                                    Ok(Ok::<Adt14, Bytes>(Adt14::C3(v0)))
                                }
                                Some(v2) => {
                                    let m1729 = f322(v2.clone(), 0u64)?;
                                    match m1729 {
                                        None => {
                                            Ok(Err::<Adt14, Bytes>(f323(v1)?))
                                        }
                                        Some(v3) => {
                                            let a1732 = {
                                                let (h1730, _) = v3;
                                                h1730
                                            };
                                            let a1733 = {
                                                let a1731 = v2;
                                                length_bytes(a1731)
                                            };
                                            if nat_eq(a1732, a1733) {
                                                Ok(Ok::<Adt14, Bytes>({
                                                    let (_, h1734) = v3;
                                                    h1734
                                                }))
                                            } else {
                                                Ok(Err::<Adt14, Bytes>(f323(v1)?))
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

pub fn f234(v0: Bytes, v1: u64) -> R<u64> {
    let (h1735, _) = f124(Fn4::F324(v0.clone()), {
        let a1737 = {
            let a1736 = v0;
            length_bytes(a1736)
        };
        let a1738 = 1u64;
        nat_add(a1737, a1738)?
    }, v1)?;
    Ok(h1735)
}

pub fn f235(v0: Bytes, v1: u64) -> R<bool> {
    let a1739 = f100(v0.clone(), v1)?;
    let a1740 = 58u64;
    if nat_eq(a1739, a1740) {
        if f153(f100(v0.clone(), {
            let a1741 = v1;
            let a1742 = 1u64;
            nat_add(a1741, a1742)?
        })?) {
            Ok(true)
        } else {
            Ok(f154(f100(v0, {
                let a1743 = v1;
                let a1744 = 1u64;
                nat_add(a1743, a1744)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f236(v0: u64) -> Bytes {
    let m1745 = v0;
    if m1745 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1745 - 1;
        let a1746 = f8(Str::lit("0"));
        let a1747 = f236(v1);
        append_bytes(a1746, a1747)
    }
}

pub fn f237(v0: Bytes, p1760: u64, p1761: u64) -> R<u64> {
    let m1759 = (p1760, p1761);
    {
        let mut m1759 = m1759;
        loop {
            let step = {
                let (v1, v2) = m1759;
                let m1748 = v2;
                if m1748 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1748 - 1;
                    let v4: u64 = f133(v0.clone(), v1)?;
                    let a1749 = v4;
                    let a1750 = 34u64;
                    if nat_eq(a1749, a1750) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1751 = v4;
                        let a1752 = 92u64;
                        if nat_eq(a1751, a1752) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1753 = v4;
                            let a1754 = 32u64;
                            if nat_lt(a1753, a1754) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1755 = v4;
                                let a1756 = 256u64;
                                if nat_eq(a1755, a1756) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1757 = v1;
                                        let a1758 = 1u64;
                                        nat_add(a1757, a1758)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1759 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: Bytes) -> R<Bytes> {
    let a1767 = {
        let a1765 = Bytes::lit(&[0x22]);
        let a1766 = {
            let (_, h1762) = {
                let (h1763, _) = f121(Fn3::F325(v0.clone()), {
                    let a1764 = v0;
                    length_bytes(a1764)
                }, (0u64, Bytes::lit(&[])))?;
                h1763
            };
            h1762
        };
        append_bytes(a1765, a1766)
    };
    let a1768 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1767, a1768))
}

pub fn f239(v0: Fn5, p1772: List<(Bytes, Bytes)>, p1773: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1771 = (p1772, p1773);
    {
        let mut m1771 = m1771;
        loop {
            let step = {
                let (v1, v2) = m1771;
                let m1769 = v2;
                match m1769.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1770: Fn5 = v0.clone();
                            c1770.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f326(v0, v1)
}

pub fn f241(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f327(v0)
}

pub fn f242(v0: Fn6, p1777: List<Bytes>, p1778: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1776 = (p1777, p1778);
    {
        let mut m1776 = m1776;
        loop {
            let step = {
                let (v1, v2) = m1776;
                let m1774 = v2;
                match m1774.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1775: Fn6 = v0.clone();
                            c1775.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1776 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f328(v1)?, v0))
}

pub fn f244(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1779 = v0;
        utf8_encode(a1779)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f245(p1782: List<(u64, Bytes)>, p1783: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m1781 = (p1782, p1783);
    {
        let mut m1781 = m1781;
        loop {
            let step = {
                let (v0, v1) = m1781;
                let m1780 = v0;
                match m1780.uncons() {
                    None => {
                        Err::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>((v3, List::cons(v2, v1)))
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

pub fn f246(v0: u64) -> bool {
    let a1784 = 65u64;
    let a1785 = v0;
    if if nat_le(a1784, a1785) {
        let a1786 = v0;
        let a1787 = 90u64;
        nat_le(a1786, a1787)
    } else {
        false
    } {
        true
    } else {
        let a1788 = 97u64;
        let a1789 = v0;
        if nat_le(a1788, a1789) {
            let a1790 = v0;
            let a1791 = 122u64;
            nat_le(a1790, a1791)
        } else {
            false
        }
    }
}

pub fn f247(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1792, _) = f124(Fn4::F329(v1, v0.clone()), {
        let a1794 = {
            let a1793 = v0;
            length_bytes(a1793)
        };
        let a1795 = 1u64;
        nat_add(a1794, a1795)?
    }, v2)?;
    Ok(h1792)
}

pub fn f248(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a1796 = v2;
    let a1797 = 48u64;
    if nat_eq(a1796, a1797) {
        let v3: u64 = f85(v0.clone(), {
            let a1798 = v1;
            let a1799 = 1u64;
            nat_add(a1798, a1799)?
        })?;
        let a1800 = v3;
        let a1801 = 88u64;
        if if if nat_eq(a1800, a1801) {
            true
        } else {
            let a1802 = v3;
            let a1803 = 120u64;
            nat_eq(a1802, a1803)
        } {
            f330(f85(v0.clone(), {
                let a1804 = v1;
                let a1805 = 2u64;
                nat_add(a1804, a1805)?
            })?)
        } else {
            false
        } {
            f247(v0, 1u64, {
                let a1806 = v1;
                let a1807 = 2u64;
                nat_add(a1806, a1807)?
            })
        } else {
            f247(v0, 2u64, {
                let a1808 = v1;
                let a1809 = 1u64;
                nat_add(a1808, a1809)?
            })
        }
    } else {
        if f331(v2) {
            f247(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f249(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f247(v0.clone(), 0u64, v1)?;
    let a1810 = f85(v0.clone(), v2)?;
    let a1811 = 46u64;
    if nat_eq(a1810, a1811) {
        let v3: u64 = f247(v0.clone(), 0u64, {
            let a1812 = v2;
            let a1813 = 1u64;
            nat_add(a1812, a1813)?
        })?;
        let a1814 = v1;
        let a1815 = v2;
        if if nat_lt(a1814, a1815) {
            true
        } else {
            let a1818 = {
                let a1816 = v2;
                let a1817 = 1u64;
                nat_add(a1816, a1817)?
            };
            let a1819 = v3;
            nat_lt(a1818, a1819)
        } {
            f332(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a1820 = v1;
        let a1821 = v2;
        if nat_lt(a1820, a1821) {
            let v4: u64 = f332(v0, v2)?;
            let a1822 = v2;
            let a1823 = v4;
            if nat_lt(a1822, a1823) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f250(v0: List<(u64, Bytes)>, v1: Str) -> bool {
    let m1824 = v0;
    match m1824.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f333(v2, {
                let a1825 = v1;
                utf8_encode(a1825)
            })
        }
    }
}

pub fn f251(p1840: u64, p1841: List<(u64, Bytes)>, p1842: List<Bytes>, v3: bool) -> Result<List<(u64, Bytes)>, Bytes> {
    let m1839 = (p1840, (p1841, p1842));
    {
        let mut m1839 = m1839;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1839;
                let m1826 = v0;
                if m1826 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m1826 - 1;
                    let m1827 = v1;
                    match m1827.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a1829 = {
                                let (h1828, _) = v5;
                                h1828
                            };
                            let a1830 = 3u64;
                            if nat_eq(a1829, a1830) {
                                Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = {
                                    let (_, h1831) = v5;
                                    h1831
                                };
                                let v8: Bytes = f334(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f335(v7.clone()) {
                                        let m1832 = v2;
                                        match m1832.uncons() {
                                            None => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a1834 = {
                                                            let a1833 = v10.clone();
                                                            length_list(a1833)
                                                        };
                                                        let a1835 = 0u64;
                                                        nat_eq(a1834, a1835)
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
                                            if f35(v7, f8(Str::lit(";"))) {
                                                let a1837 = {
                                                    let a1836 = v2.clone();
                                                    length_list(a1836)
                                                };
                                                let a1838 = 0u64;
                                                nat_eq(a1837, a1838)
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
                Ok(next) => { m1839 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(v0: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m1843 = v0;
    match m1843.uncons() {
        None => {
            List::<(u64, Bytes)>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f253(v0: u64, v1: u64, v2: List<(u64, Bytes)>, v3: bool) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    let m1844 = f256(v2);
    match m1844 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f250({
                    let (_, h1845) = v5.clone();
                    h1845
                }, Str::lit(":"))
            } else {
                false
            };
            let m1848 = if v6 {
                f256(f252({
                    let (_, h1846) = v5;
                    h1846
                }))
            } else {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((Bytes::lit(&[]), {
                    let (_, h1847) = v5;
                    h1847
                }))
            };
            match m1848 {
                Ok(v8) => {
                    let m1850 = f257({
                        let (_, h1849) = v8;
                        h1849
                    }, Str::lit("{"));
                    match m1850 {
                        Ok(v10) => {
                            let m1855 = {
                                let a1851 = v1;
                                let a1852 = 2u64;
                                if if nat_eq(a1851, a1852) {
                                    true
                                } else {
                                    let a1853 = v1;
                                    let a1854 = 7u64;
                                    nat_eq(a1853, a1854)
                                } {
                                    f336(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                } else {
                                    f337(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                }
                            };
                            match m1855 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(v1, {
                                        let (h1856, _) = v5;
                                        h1856
                                    }, {
                                        let (h1857, _) = v8;
                                        h1857
                                    }, {
                                        let (h1858, _) = v12;
                                        h1858
                                    }, List::<Bytes>::nil(), f259(Bytes::lit(&[]))), {
                                        let (_, h1859) = v12;
                                        h1859
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v4))
        }
    }
}

pub fn f254(v0: List<(u64, Bytes)>, v1: Bytes) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    let m1860 = f256(v0);
    match m1860 {
        Ok(v3) => {
            let m1862 = f338({
                let (_, h1861) = v3;
                h1861
            })?;
            match m1862 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(9u64, {
                        let (h1863, _) = v3;
                        h1863
                    }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::cons(v1, List::<Bytes>::nil()), f259(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v2))
        }
    }
}

pub fn f255(v0: Str, v1: List<(u64, Bytes)>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a1864 = v0;
        utf8_encode(a1864)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f339(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f256(v0: List<(u64, Bytes)>) -> Result<(Bytes, List<(u64, Bytes)>), Bytes> {
    let m1865 = v0.clone();
    match m1865.uncons() {
        None => {
            Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f255(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a1867 = {
                let (h1866, _) = v1;
                h1866
            };
            let a1868 = 0u64;
            if nat_eq(a1867, a1868) {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((f340({
                    let (_, h1869) = v1;
                    h1869
                }), v2))
            } else {
                Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f255(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f257(v0: List<(u64, Bytes)>, v1: Str) -> Result<List<(u64, Bytes)>, Bytes> {
    if f250(v0.clone(), v1.clone()) {
        Ok::<List<(u64, Bytes)>, Bytes>(f252(v0))
    } else {
        Err::<List<(u64, Bytes)>, Bytes>(f255(v1, v0))
    }
}

pub fn f258(p1881: u64, p1882: List<(u64, Bytes)>, p1883: List<Bytes>) -> Result<(List<Bytes>, List<(u64, Bytes)>), Bytes> {
    let m1880 = (p1881, (p1882, p1883));
    {
        let mut m1880 = m1880;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1880;
                let m1870 = v0;
                if m1870 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m1870 - 1;
                    let m1871 = v1.clone();
                    match m1871.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f255(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a1873 = {
                                let (h1872, _) = v4;
                                h1872
                            };
                            let a1874 = 3u64;
                            if !nat_eq(a1873, a1874) {
                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f255(Str::lit("a string"), v1)))
                            } else {
                                if f250(v5.clone(), Str::lit(",")) {
                                    if f250(f252(v5.clone()), Str::lit("}")) {
                                        let m1875 = f257(f252(f252(v5)), Str::lit(";"));
                                        match m1875 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f44(List::cons({
                                                    let (_, h1876) = v4;
                                                    h1876
                                                }, v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>((v3, (f252(v5), List::cons({
                                            let (_, h1877) = v4;
                                            h1877
                                        }, v2))))
                                    }
                                } else {
                                    if f250(v5.clone(), Str::lit("}")) {
                                        let m1878 = f257(f252(v5), Str::lit(";"));
                                        match m1878 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f44(List::cons({
                                                    let (_, h1879) = v4;
                                                    h1879
                                                }, v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f255(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1880 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (v0, (List::<Bytes>::nil(), (Bytes::lit(&[]), Bytes::lit(&[])))))
}

pub fn f260(v0: u64, v1: List<(u64, Bytes)>, v2: u64) -> R<Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>> {
    let m1884 = v0;
    if m1884 == 0 {
        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m1884 - 1;
        let a1885 = v2;
        let a1886 = 0u64;
        if nat_eq(a1885, a1886) {
            let m1887 = f179(v1)?;
            match m1887 {
                Ok(v5) => {
                    if f250(v5.clone(), Str::lit("(")) {
                        let m1888 = f260(v3, f252(v5), 1u64)?;
                        match m1888 {
                            Ok(v7) => {
                                if f250({
                                    let (_, h1889) = v7.clone();
                                    h1889
                                }, Str::lit("?")) {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((0u64, (Bytes::lit(&[]), (f199({
                                        let (h1890, _) = v7;
                                        h1890
                                    }), (f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[]))))), f252({
                                        let (_, h1891) = v7;
                                        h1891
                                    }))))
                                } else {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(({
                                        let (h1892, _) = v7;
                                        h1892
                                    }, {
                                        let (_, h1893) = v7;
                                        h1893
                                    })))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m1894 = v5.clone();
                        match m1894.uncons() {
                            None => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = {
                                    let (_, h1895) = v8;
                                    h1895
                                };
                                let a1897 = {
                                    let (h1896, _) = v8;
                                    h1896
                                };
                                let a1898 = 0u64;
                                if !nat_eq(a1897, a1898) {
                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("a type"), v5)))
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
                                        let m1899 = f257(v9, Str::lit("<"));
                                        match m1899 {
                                            Ok(v12) => {
                                                let m1900 = f260(v3, v12, 0u64)?;
                                                match m1900 {
                                                    Ok(v14) => {
                                                        let m1902 = f257({
                                                            let (_, h1901) = v14;
                                                            h1901
                                                        }, Str::lit(">"));
                                                        match m1902 {
                                                            Ok(v16) => {
                                                                Ok(f341(f342(Str::lit("List"), {
                                                                    let (h1903, _) = v14;
                                                                    h1903
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
                                        if f35(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m1904 = f257(v9, Str::lit("<"));
                                            match m1904 {
                                                Ok(v18) => {
                                                    let m1905 = f260(v3, v18, 0u64)?;
                                                    match m1905 {
                                                        Ok(v20) => {
                                                            let m1907 = f257({
                                                                let (_, h1906) = v20;
                                                                h1906
                                                            }, Str::lit(">"));
                                                            match m1907 {
                                                                Ok(v22) => {
                                                                    Ok(f341(f343(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
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
                                            if f35(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m1908 = f257(v9, Str::lit("<"));
                                                match m1908 {
                                                    Ok(v24) => {
                                                        let m1909 = f260(v3, v24, 0u64)?;
                                                        match m1909 {
                                                            Ok(v26) => {
                                                                let m1911 = f257({
                                                                    let (_, h1910) = v26;
                                                                    h1910
                                                                }, Str::lit(">"));
                                                                match m1911 {
                                                                    Ok(v28) => {
                                                                        Ok(f341(f343(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
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
                                                if f35(v10.clone(), f8(Str::lit("record"))) {
                                                    let m1912 = f257(v9, Str::lit("<"));
                                                    match m1912 {
                                                        Ok(v30) => {
                                                            let m1913 = v30.clone();
                                                            match m1913.uncons() {
                                                                None => {
                                                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35({
                                                                        let (_, h1914) = v31.clone();
                                                                        h1914
                                                                    }, f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35({
                                                                            let (_, h1915) = v31.clone();
                                                                            h1915
                                                                        }, f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35({
                                                                                let (_, h1916) = v31.clone();
                                                                                h1916
                                                                            }, f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m1917 = f257(v32, Str::lit(","));
                                                                        match m1917 {
                                                                            Ok(v34) => {
                                                                                let m1918 = f260(v3, v34, 0u64)?;
                                                                                match m1918 {
                                                                                    Ok(v36) => {
                                                                                        let m1920 = f257({
                                                                                            let (_, h1919) = v36;
                                                                                            h1919
                                                                                        }, Str::lit(">"));
                                                                                        match m1920 {
                                                                                            Ok(v38) => {
                                                                                                let m1922 = f344({
                                                                                                    let (_, h1921) = v31;
                                                                                                    h1921
                                                                                                });
                                                                                                match m1922 {
                                                                                                    None => {
                                                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f345({
                                                                                                            let (h1923, _) = v36;
                                                                                                            h1923
                                                                                                        });
                                                                                                        Ok(f341((0u64, (f7(List::cons(f8(Str::lit("(List (")), List::cons(f198(v39), List::cons(f8(Str::lit(" * ")), List::cons(f198(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), (f199(v40.clone()), (f196(v40), Bytes::lit(&[]))))), v38))
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
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f35(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f250(v9.clone(), Str::lit("short")) {
                                                            Ok(f341(f259(f8(Str::lit("Nat"))), f252(v9)))
                                                        } else {
                                                            if f250(v9.clone(), Str::lit("long")) {
                                                                let v41: List<(u64, Bytes)> = f252(v9);
                                                                Ok(f341(f259(f8(Str::lit("Nat"))), if f250(v41.clone(), Str::lit("long")) {
                                                                    f252(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f341(f259(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f341(f259(f8(Str::lit("Int"))), if f250(v9.clone(), Str::lit("long")) {
                                                                    f252(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f35(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f250(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f250(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f341(f343(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f252(v9)))
                                                                    } else {
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m1924 = f344(v10.clone());
                                                                    match m1924 {
                                                                        None => {
                                                                            let v42: Bytes = f340(v10);
                                                                            Ok(f341((0u64, (f148(v42.clone())?, (List::cons(v42.clone(), List::<Bytes>::nil()), (Bytes::lit(&[]), v42)))), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f341(v43, v9))
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
            let m1925 = f260(v3, v1, 0u64)?;
            match m1925 {
                Ok(v45) => {
                    let v46: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let (h1926, _) = v45.clone();
                        h1926
                    };
                    let v48: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let a1927 = f197(v46.clone());
                        let a1928 = 0u64;
                        if nat_eq(a1927, a1928) {
                            if f35(f196(v46.clone()), Bytes::lit(&[])) {
                                if f35(f346(v46.clone()), Bytes::lit(&[])) {
                                    (1u64, (Bytes::lit(&[]), (f199(v46), (f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[])))))
                                } else {
                                    let m1929 = f273(false, f346(v46.clone()))?;
                                    match m1929 {
                                        None => {
                                            (1u64, (Bytes::lit(&[]), (f199(v46.clone()), (f7(List::cons(f8(Str::lit("the union member ")), List::cons(f346(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[])))))
                                        }
                                        Some(v47) => {
                                            if f182(v47.clone()) {
                                                (1u64, (Bytes::lit(&[]), (f199(v46), (f183(v47), Bytes::lit(&[])))))
                                            } else {
                                                (1u64, (f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f198(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), (f199(v46), (Bytes::lit(&[]), Bytes::lit(&[])))))
                                            }
                                        }
                                    }
                                }
                            } else {
                                (1u64, (Bytes::lit(&[]), (f199(v46.clone()), (f196(v46), Bytes::lit(&[])))))
                            }
                        } else {
                            v46
                        }
                    };
                    if f250({
                        let (_, h1930) = v45.clone();
                        h1930
                    }, Str::lit("or")) {
                        let m1932 = f260(v3, f252({
                            let (_, h1931) = v45;
                            h1931
                        }), 2u64)?;
                        match m1932 {
                            Ok(v50) => {
                                Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((1u64, (if f35(f198(v48.clone()), Bytes::lit(&[])) {
                                    f198({
                                        let (h1933, _) = v50.clone();
                                        h1933
                                    })
                                } else {
                                    if f35(f198({
                                        let (h1934, _) = v50.clone();
                                        h1934
                                    }), Bytes::lit(&[])) {
                                        f198(v48.clone())
                                    } else {
                                        f7(List::cons(f198(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f198({
                                            let (h1935, _) = v50.clone();
                                            h1935
                                        }), List::<Bytes>::nil()))))
                                    }
                                }, (f115(f199(v48.clone()), f199({
                                    let (h1936, _) = v50.clone();
                                    h1936
                                })), (if f35(f196(v48.clone()), Bytes::lit(&[])) {
                                    f196({
                                        let (h1937, _) = v50;
                                        h1937
                                    })
                                } else {
                                    f196(v48)
                                }, Bytes::lit(&[]))))), {
                                    let (_, h1938) = v50;
                                    h1938
                                })))
                            }
                            Err(v49) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v49))
                            }
                        }
                    } else {
                        let a1939 = v2;
                        let a1940 = 2u64;
                        if if nat_eq(a1939, a1940) {
                            f250({
                                let (_, h1941) = v45.clone();
                                h1941
                            }, Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v48, f252({
                                let (_, h1942) = v45;
                                h1942
                            }))))
                        } else {
                            Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>({
                                let a1943 = v2;
                                let a1944 = 1u64;
                                if nat_eq(a1943, a1944) {
                                    f255(Str::lit("or"), {
                                        let (_, h1945) = v45;
                                        h1945
                                    })
                                } else {
                                    f255(Str::lit("or or )"), {
                                        let (_, h1946) = v45;
                                        h1946
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

pub fn f261() -> List<Bytes> {
    List::cons(f8(Str::lit("Prop")), List::cons(f8(Str::lit("Sort")), List::cons(f8(Str::lit("StateRefT")), List::cons(f8(Str::lit("Type")), List::cons(f8(Str::lit("abbrev")), List::cons(f8(Str::lit("at")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("bif")), List::cons(f8(Str::lit("break")), List::cons(f8(Str::lit("by")), List::cons(f8(Str::lit("calc")), List::cons(f8(Str::lit("catch")), List::cons(f8(Str::lit("class")), List::cons(f8(Str::lit("coinductive")), List::cons(f8(Str::lit("continue")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f262() -> List<Bytes> {
    List::cons(f8(Str::lit("def")), List::cons(f8(Str::lit("deriving")), List::cons(f8(Str::lit("do")), List::cons(f8(Str::lit("dsimproc")), List::cons(f8(Str::lit("elab")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("end")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("exists")), List::cons(f8(Str::lit("export")), List::cons(f8(Str::lit("extends")), List::cons(f8(Str::lit("finally")), List::cons(f8(Str::lit("for")), List::cons(f8(Str::lit("forall")), List::cons(f8(Str::lit("from")), List::cons(f8(Str::lit("fun")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f263() -> List<Bytes> {
    List::cons(f8(Str::lit("generalizing")), List::cons(f8(Str::lit("have")), List::cons(f8(Str::lit("haveI")), List::cons(f8(Str::lit("hiding")), List::cons(f8(Str::lit("idbg")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("import")), List::cons(f8(Str::lit("in")), List::cons(f8(Str::lit("include")), List::cons(f8(Str::lit("inductive")), List::cons(f8(Str::lit("inferInstanceAs")), List::cons(f8(Str::lit("infix")), List::cons(f8(Str::lit("infixl")), List::cons(f8(Str::lit("infixr")), List::cons(f8(Str::lit("initialize")), List::cons(f8(Str::lit("instance")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f264() -> List<Bytes> {
    List::cons(f8(Str::lit("let")), List::cons(f8(Str::lit("letI")), List::cons(f8(Str::lit("local")), List::cons(f8(Str::lit("logNamedError")), List::cons(f8(Str::lit("logNamedErrorAt")), List::cons(f8(Str::lit("logNamedWarning")), List::cons(f8(Str::lit("logNamedWarningAt")), List::cons(f8(Str::lit("macro")), List::cons(f8(Str::lit("match")), List::cons(f8(Str::lit("matches")), List::cons(f8(Str::lit("meta")), List::cons(f8(Str::lit("mut")), List::cons(f8(Str::lit("mutual")), List::cons(f8(Str::lit("namespace")), List::cons(f8(Str::lit("nofun")), List::cons(f8(Str::lit("nomatch")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f265() -> List<Bytes> {
    List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("nonrec")), List::cons(f8(Str::lit("notation")), List::cons(f8(Str::lit("omit")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("open")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("postfix")), List::cons(f8(Str::lit("prefix")), List::cons(f8(Str::lit("private")), List::cons(f8(Str::lit("protected")), List::cons(f8(Str::lit("public")), List::cons(f8(Str::lit("renaming")), List::cons(f8(Str::lit("repeat")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("return")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f266() -> List<Bytes> {
    List::cons(f8(Str::lit("scoped")), List::cons(f8(Str::lit("seal")), List::cons(f8(Str::lit("section")), List::cons(f8(Str::lit("show")), List::cons(f8(Str::lit("simproc")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("structure")), List::cons(f8(Str::lit("suffices")), List::cons(f8(Str::lit("syntax")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("theorem")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::cons(f8(Str::lit("throwNamedError")), List::cons(f8(Str::lit("throwNamedErrorAt")), List::cons(f8(Str::lit("try")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f267() -> List<Bytes> {
    List::cons(f8(Str::lit("universe")), List::cons(f8(Str::lit("unless")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("unseal")), List::cons(f8(Str::lit("until")), List::cons(f8(Str::lit("using")), List::cons(f8(Str::lit("variable")), List::cons(f8(Str::lit("where")), List::cons(f8(Str::lit("while")), List::cons(f8(Str::lit("with")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::<Bytes>::nil())))))))))))))
}

pub fn f268(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m1947 = v0;
    match m1947.uncons() {
        None => {
            Ok(List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())
        }
        Some((v1, v2)) => {
            f347(v1, f268(v2)?)
        }
    }
}

pub fn f269(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1948 = v0;
    match m1948.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f269(v3, v1))
        }
    }
}

pub fn f270(v0: Adt12) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1949 = v0;
    match m1949 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f271(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1950 = v0;
    match m1950.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a1951 = f110(v3.clone());
            let a1952 = v1;
            if if nat_eq(a1951, a1952) {
                f35(f112(v3.clone()), v2.clone())
            } else {
                false
            } {
                f269(f270(v3), f271(v4, v1, v2))
            } else {
                f271(v4, v1, v2)
            }
        }
    }
}

pub fn f272(v0: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1953 = v0;
    match m1953.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a1954 = f110(v3.clone());
            let a1955 = 8u64;
            if if nat_eq(a1954, a1955) {
                f35(f112(v3.clone()), v2.clone())
            } else {
                false
            } {
                f269(f348(v1.clone(), f119(v3)), f272(v4, v1, v2))
            } else {
                f272(v4, v1, v2)
            }
        }
    }
}

pub fn f273(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f349(v1)
    } else {
        f275(v1)
    }
}

pub fn f274(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1956 = v1;
    match m1956.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1957 = f273(v0, v2)?;
            match m1957 {
                None => {
                    f274(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f274(v0, v3)?))
                }
            }
        }
    }
}

pub fn f275(v0: Bytes) -> R<Option<Bytes>> {
    if f350({
        let a1958 = v0.clone();
        length_bytes(a1958)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f351(v0)?;
        if f352(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m1961 = {
                let a1959 = v1.clone();
                let a1960 = 0u64;
                index_bytes(a1959, a1960)
            };
            match m1961 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f295(f41(v2)?) {
                        Ok(Some({
                            let a1962 = f8(Str::lit("n"));
                            let a1963 = v1;
                            append_bytes(a1962, a1963)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f353({
            let a1964 = v0.clone();
            length_bytes(a1964)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f276(v0: bool, v1: Adt14) -> R<Option<Bytes>> {
    let m1965 = v1;
    match m1965 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v7, v8) => {
            Ok(Some({
                let a1976 = if f291(v7, {
                    let a1974 = 0u64;
                    let a1975 = v8;
                    nat_lt(a1974, a1975)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a1977 = f170(false, v8)?;
                append_bytes(a1976, a1977)
            }))
        }
        Adt14::C2(v3, v4, v5) => {
            let m1968 = f349({
                let a1966 = f8(Str::lit("n"));
                let a1967 = f171(false, v4, v5)?;
                append_bytes(a1966, a1967)
            })?;
            match m1968 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a1972 = f8(Str::lit("nMinus"));
                        let a1973 = f32(v6.clone(), 1u64, {
                            let a1970 = {
                                let a1969 = v6;
                                length_bytes(a1969)
                            };
                            let a1971 = 1u64;
                            nat_sub(a1970, a1971)
                        });
                        append_bytes(a1972, a1973)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt14::C3(v2) => {
            f273(v0, v2)
        }
        Adt14::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C6 => {
            Ok(None::<Bytes>)
        }
        Adt14::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt14::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f277(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m1980 = {
        let a1978 = v0.clone();
        let a1979 = v1;
        index_bytes(a1978, a1979)
    };
    match m1980 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            let a1981 = v3;
            let a1982 = 128u64;
            if nat_lt(a1981, a1982) {
                Ok(Some((v3, 1u64)))
            } else {
                if f291({
                    let a1983 = 194u64;
                    let a1984 = v3;
                    nat_le(a1983, a1984)
                }, {
                    let a1985 = v3;
                    let a1986 = 223u64;
                    nat_le(a1985, a1986)
                }) {
                    let m1989 = f354(v0, {
                        let a1987 = v1;
                        let a1988 = 1u64;
                        nat_add(a1987, a1988)?
                    })?;
                    match m1989 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f355({
                                let a1994 = {
                                    let a1992 = {
                                        let a1990 = v3;
                                        let a1991 = 192u64;
                                        nat_sub(a1990, a1991)
                                    };
                                    let a1993 = 64u64;
                                    nat_mul(a1992, a1993)?
                                };
                                let a1995 = v4;
                                nat_add(a1994, a1995)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f291({
                        let a1996 = 224u64;
                        let a1997 = v3;
                        nat_le(a1996, a1997)
                    }, {
                        let a1998 = v3;
                        let a1999 = 239u64;
                        nat_le(a1998, a1999)
                    }) {
                        let m2002 = f354(v0.clone(), {
                            let a2000 = v1;
                            let a2001 = 1u64;
                            nat_add(a2000, a2001)?
                        })?;
                        match m2002 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2005 = f354(v0, {
                                    let a2003 = v1;
                                    let a2004 = 2u64;
                                    nat_add(a2003, a2004)?
                                })?;
                                match m2005 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f355({
                                            let a2014 = {
                                                let a2008 = {
                                                    let a2006 = v3;
                                                    let a2007 = 224u64;
                                                    nat_sub(a2006, a2007)
                                                };
                                                let a2009 = 4096u64;
                                                nat_mul(a2008, a2009)?
                                            };
                                            let a2015 = {
                                                let a2012 = {
                                                    let a2010 = v5;
                                                    let a2011 = 64u64;
                                                    nat_mul(a2010, a2011)?
                                                };
                                                let a2013 = v6;
                                                nat_add(a2012, a2013)?
                                            };
                                            nat_add(a2014, a2015)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f291({
                            let a2016 = 240u64;
                            let a2017 = v3;
                            nat_le(a2016, a2017)
                        }, {
                            let a2018 = v3;
                            let a2019 = 244u64;
                            nat_le(a2018, a2019)
                        }) {
                            let m2022 = f354(v0.clone(), {
                                let a2020 = v1;
                                let a2021 = 1u64;
                                nat_add(a2020, a2021)?
                            })?;
                            match m2022 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2025 = f354(v0.clone(), {
                                        let a2023 = v1;
                                        let a2024 = 2u64;
                                        nat_add(a2023, a2024)?
                                    })?;
                                    match m2025 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2028 = f354(v0, {
                                                let a2026 = v1;
                                                let a2027 = 3u64;
                                                nat_add(a2026, a2027)?
                                            })?;
                                            match m2028 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f355({
                                                        let a2041 = {
                                                            let a2031 = {
                                                                let a2029 = v3;
                                                                let a2030 = 240u64;
                                                                nat_sub(a2029, a2030)
                                                            };
                                                            let a2032 = 262144u64;
                                                            nat_mul(a2031, a2032)?
                                                        };
                                                        let a2042 = {
                                                            let a2039 = {
                                                                let a2033 = v7;
                                                                let a2034 = 4096u64;
                                                                nat_mul(a2033, a2034)?
                                                            };
                                                            let a2040 = {
                                                                let a2037 = {
                                                                    let a2035 = v8;
                                                                    let a2036 = 64u64;
                                                                    nat_mul(a2035, a2036)?
                                                                };
                                                                let a2038 = v9;
                                                                nat_add(a2037, a2038)?
                                                            };
                                                            nat_add(a2039, a2040)?
                                                        };
                                                        nat_add(a2041, a2042)?
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

pub fn f278(v0: u64) -> R<bool> {
    f204(v0, f356())
}

pub fn f279(v0: u64, v1: Bytes, p2053: u64, p2054: u64, p2055: bool) -> R<bool> {
    let m2052 = (p2053, (p2054, p2055));
    {
        let mut m2052 = m2052;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2052;
                let m2043 = v3;
                if m2043 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2043 - 1;
                    let v6: u64 = f357(v1.clone(), v2)?;
                    if v4 {
                        let a2044 = v0;
                        let a2045 = v6;
                        if nat_lt(a2044, a2045) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2046 = v2;
                                let a2047 = 1u64;
                                nat_add(a2046, a2047)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2048 = v0;
                        let a2049 = v6;
                        if nat_le(a2048, a2049) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2050 = v2;
                                let a2051 = 1u64;
                                nat_add(a2050, a2051)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2052 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f77(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2057 = v2;
        let a2058 = {
            let a2056 = v0;
            length_bytes(a2056)
        };
        if nat_lt(a2057, a2058) {
            Ok(Some({
                let a2059 = v2;
                let a2060 = 1u64;
                nat_add(a2059, a2060)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f281(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f126(f85(v0, v1)?)? {
        Ok(Some({
            let a2061 = v1;
            let a2062 = 1u64;
            nat_add(a2061, a2062)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f282(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2063 = f277(v0, v2)?;
    match m2063 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2064 = v2;
            let a2065 = v1;
            if if nat_eq(a2064, a2065) {
                f358({
                    let (h2066, _) = v3;
                    h2066
                })?
            } else {
                f359({
                    let (h2067, _) = v3;
                    h2067
                })?
            } {
                Ok(Some({
                    let a2069 = v2;
                    let a2070 = {
                        let (_, h2068) = v3;
                        h2068
                    };
                    nat_add(a2069, a2070)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f283(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2071, _) = f305(Fn7::F360(v1, v0.clone()), {
        let a2073 = {
            let a2072 = v0;
            length_bytes(a2072)
        };
        let a2074 = 1u64;
        nat_add(a2073, a2074)?
    }, (v2, 0u64))?;
    Ok(h2071)
}

pub fn f284(v0: u64) -> R<Bytes> {
    let a2075 = v0;
    let a2076 = 128u64;
    if nat_lt(a2075, a2076) {
        Ok(f361(f362(), v0))
    } else {
        let a2077 = v0;
        let a2078 = 2048u64;
        if nat_lt(a2077, a2078) {
            let a2086 = f361(f363(), {
                let a2084 = {
                    let a2082 = {
                        let a2079 = v0;
                        let a2080 = 64u64;
                        let a2081 = 0u64;
                        nat_quot(a2079, a2080, a2081)
                    };
                    let a2083 = 2u64;
                    nat_sub(a2082, a2083)
                };
                let a2085 = 2u64;
                nat_mul(a2084, a2085)?
            });
            let a2087 = f364(v0)?;
            Ok(append_bytes(a2086, a2087))
        } else {
            let a2088 = v0;
            let a2089 = 65536u64;
            if nat_lt(a2088, a2089) {
                let a2100 = f361(f365(), {
                    let a2093 = {
                        let a2090 = v0;
                        let a2091 = 4096u64;
                        let a2092 = 0u64;
                        nat_quot(a2090, a2091, a2092)
                    };
                    let a2094 = 3u64;
                    nat_mul(a2093, a2094)?
                });
                let a2101 = {
                    let a2098 = f364({
                        let a2095 = v0;
                        let a2096 = 64u64;
                        let a2097 = 0u64;
                        nat_quot(a2095, a2096, a2097)
                    })?;
                    let a2099 = f364(v0)?;
                    append_bytes(a2098, a2099)
                };
                Ok(append_bytes(a2100, a2101))
            } else {
                let a2115 = f361(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2102 = v0;
                    let a2103 = 262144u64;
                    let a2104 = 0u64;
                    nat_quot(a2102, a2103, a2104)
                });
                let a2116 = {
                    let a2113 = f364({
                        let a2105 = v0;
                        let a2106 = 4096u64;
                        let a2107 = 0u64;
                        nat_quot(a2105, a2106, a2107)
                    })?;
                    let a2114 = {
                        let a2111 = f364({
                            let a2108 = v0;
                            let a2109 = 64u64;
                            let a2110 = 0u64;
                            nat_quot(a2108, a2109, a2110)
                        })?;
                        let a2112 = f364(v0)?;
                        append_bytes(a2111, a2112)
                    };
                    append_bytes(a2113, a2114)
                };
                Ok(append_bytes(a2115, a2116))
            }
        }
    }
}

pub fn f285(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f366(v2, v0, v1, v3)
}

pub fn f286(v0: Bytes) -> Result<Option<Adt16>, Bytes> {
    Err::<Option<Adt16>, Bytes>(v0)
}

pub fn f287(p2120: Bytes, p2121: u64) -> R<Result<Bytes, Bytes>> {
    let m2119 = (p2120, p2121);
    {
        let mut m2119 = m2119;
        loop {
            let step = {
                let (v0, v1) = m2119;
                let m2117 = v1;
                if m2117 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f367(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2117 - 1;
                    if f168(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f35(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f35(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f35(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f368())))
                                } else {
                                    if f35(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f168(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f367(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f168(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f367(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2118 = f369(v0.clone())?;
                                                match m2118 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f367(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m2119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Bytes) -> Result<List<Adt14>, Bytes> {
    Err::<List<Adt14>, Bytes>(v0)
}

pub fn f289(v0: Option<Adt14>) -> Option<Bytes> {
    let m2122 = v0;
    match m2122 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2123 = v1;
            match m2123 {
                Adt14::C0(_) => {
                    None::<Bytes>
                }
                Adt14::C1(_, _) => {
                    None::<Bytes>
                }
                Adt14::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt14::C3(v2) => {
                    Some(v2)
                }
                Adt14::C4(_) => {
                    None::<Bytes>
                }
                Adt14::C5(_) => {
                    None::<Bytes>
                }
                Adt14::C6 => {
                    None::<Bytes>
                }
                Adt14::C7(_, _) => {
                    None::<Bytes>
                }
                Adt14::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f290(v0: u64) -> bool {
    f293({
        let a2124 = v0;
        let a2125 = 95u64;
        nat_eq(a2124, a2125)
    }, f293({
        let a2126 = v0;
        let a2127 = 45u64;
        nat_eq(a2126, a2127)
    }, f293({
        let a2128 = v0;
        let a2129 = 32u64;
        nat_eq(a2128, a2129)
    }, f293({
        let a2130 = v0;
        let a2131 = 46u64;
        nat_eq(a2130, a2131)
    }, {
        let a2132 = v0;
        let a2133 = 47u64;
        nat_eq(a2132, a2133)
    }))))
}

pub fn f291(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f292(v0: u64) -> bool {
    f291({
        let a2134 = 97u64;
        let a2135 = v0;
        nat_le(a2134, a2135)
    }, {
        let a2136 = v0;
        let a2137 = 122u64;
        nat_le(a2136, a2137)
    })
}

pub fn f293(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f294(v0: u64) -> bool {
    f291({
        let a2138 = 65u64;
        let a2139 = v0;
        nat_le(a2138, a2139)
    }, {
        let a2140 = v0;
        let a2141 = 90u64;
        nat_le(a2140, a2141)
    })
}

pub fn f295(v0: u64) -> bool {
    f291({
        let a2142 = 48u64;
        let a2143 = v0;
        nat_le(a2142, a2143)
    }, {
        let a2144 = v0;
        let a2145 = 57u64;
        nat_le(a2144, a2145)
    })
}

pub fn f296(v0: u64) -> bool {
    let a2146 = v0;
    let a2147 = 9u64;
    if if if nat_eq(a2146, a2147) {
        true
    } else {
        let a2148 = v0;
        let a2149 = 10u64;
        nat_eq(a2148, a2149)
    } {
        true
    } else {
        let a2150 = v0;
        let a2151 = 13u64;
        if nat_eq(a2150, a2151) {
            true
        } else {
            let a2152 = 32u64;
            let a2153 = v0;
            if nat_le(a2152, a2153) {
                let a2154 = v0;
                let a2155 = 126u64;
                nat_le(a2154, a2155)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2156 = v0;
        let a2157 = 133u64;
        if if nat_eq(a2156, a2157) {
            true
        } else {
            let a2158 = 160u64;
            let a2159 = v0;
            if nat_le(a2158, a2159) {
                let a2160 = v0;
                let a2161 = 55295u64;
                nat_le(a2160, a2161)
            } else {
                false
            }
        } {
            true
        } else {
            let a2162 = 57344u64;
            let a2163 = v0;
            if if nat_le(a2162, a2163) {
                let a2164 = v0;
                let a2165 = 65533u64;
                nat_le(a2164, a2165)
            } else {
                false
            } {
                true
            } else {
                let a2166 = 65536u64;
                let a2167 = v0;
                if nat_le(a2166, a2167) {
                    let a2168 = v0;
                    let a2169 = 1114111u64;
                    nat_le(a2168, a2169)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f297(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f157(v0.clone(), v1)?;
    if f154(f100(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2170 = f100(v0, v2)?;
        let a2171 = 35u64;
        Ok(nat_eq(a2170, a2171))
    }
}

pub fn f298(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f370(v0, v1)
}

pub fn f299(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2172 = f100(v0, v1)?;
    let a2173 = 32u64;
    if nat_eq(a2172, a2173) {
        Ok(Some({
            let a2174 = v1;
            let a2175 = 1u64;
            nat_add(a2174, a2175)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f300(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f153(f100(v0, v1)?) {
        Ok(Some({
            let a2176 = v1;
            let a2177 = 1u64;
            nat_add(a2176, a2177)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f301(v0: u64) -> bool {
    let a2178 = v0;
    let a2179 = 44u64;
    if if if nat_eq(a2178, a2179) {
        true
    } else {
        let a2180 = v0;
        let a2181 = 91u64;
        nat_eq(a2180, a2181)
    } {
        true
    } else {
        let a2182 = v0;
        let a2183 = 93u64;
        if nat_eq(a2182, a2183) {
            true
        } else {
            let a2184 = v0;
            let a2185 = 123u64;
            nat_eq(a2184, a2185)
        }
    } {
        true
    } else {
        let a2186 = v0;
        let a2187 = 125u64;
        nat_eq(a2186, a2187)
    }
}

pub fn f302(v0: u64) -> u64 {
    let a2188 = v0;
    let a2189 = 45u64;
    if nat_eq(a2188, a2189) {
        1u64
    } else {
        let a2190 = v0;
        let a2191 = 43u64;
        if nat_eq(a2190, a2191) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f303(v0: u64) -> u64 {
    let a2192 = 49u64;
    let a2193 = v0;
    if if nat_le(a2192, a2193) {
        let a2194 = v0;
        let a2195 = 57u64;
        nat_le(a2194, a2195)
    } else {
        false
    } {
        let a2196 = v0;
        let a2197 = 48u64;
        nat_sub(a2196, a2197)
    } else {
        0u64
    }
}

pub fn f304(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Result<(Adt14, u64), Bytes>> {
    let m2198 = f371(v0.clone(), v1, v2)?;
    match m2198 {
        Ok(v6) => {
            let v7: u64 = {
                let (_, h2199) = v6;
                h2199
            };
            let v8: u64 = f372(v0.clone(), v7, v2)?;
            if f373(v0.clone(), v7, v2)? {
                let m2204 = f374(v0.clone(), v7, v2, v3, {
                    let a2201 = {
                        let a2200 = v0;
                        length_bytes(a2200)
                    };
                    let a2202 = 2u64;
                    nat_add(a2201, a2202)?
                }, Bytes::lit(&[]), {
                    let (h2203, _) = v6;
                    h2203
                })?;
                match m2204 {
                    Ok(v10) => {
                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3(f7(List::cons({
                            let (h2205, _) = v10.clone();
                            h2205
                        }, List::cons({
                            let a2206 = v4;
                            let a2207 = 1u64;
                            if nat_eq(a2206, a2207) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2208 = v4;
                            let a2209 = 2u64;
                            if nat_eq(a2208, a2209) {
                                f307({
                                    let (h2210, _) = {
                                        let (_, h2211) = v10.clone();
                                        h2211
                                    };
                                    h2210
                                })
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), {
                            let (_, h2212) = {
                                let (_, h2213) = v10;
                                h2213
                            };
                            h2212
                        })))
                    }
                    Err(v9) => {
                        Ok(Err::<(Adt14, u64), Bytes>(v9))
                    }
                }
            } else {
                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3({
                    let a2214 = v4;
                    let a2215 = 2u64;
                    if nat_eq(a2214, a2215) {
                        f307({
                            let (h2216, _) = v6;
                            h2216
                        })
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2217 = f100(v0, v8)?;
                    let a2218 = 256u64;
                    if nat_eq(a2217, a2218) {
                        v8
                    } else {
                        v7
                    }
                })))
            }
        }
        Err(v5) => {
            Ok(Err::<(Adt14, u64), Bytes>(v5))
        }
    }
}

pub fn f305(v0: Fn7, p2223: u64, p2224: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2222 = (p2223, p2224);
    {
        let mut m2222 = m2222;
        loop {
            let step = {
                let (v1, v2) = m2222;
                let m2219 = v1;
                if m2219 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2219 - 1;
                    let m2221 = {
                        let c2220: Fn7 = v0.clone();
                        c2220.apply(v2)?
                    };
                    match m2221 {
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
                Ok(next) => { m2222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f375(v0, v1)
}

pub fn f307(v0: u64) -> Bytes {
    let m2225 = v0;
    if m2225 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2225 - 1;
        let a2226 = Bytes::lit(&[0x0a]);
        let a2227 = f307(v1);
        append_bytes(a2226, a2227)
    }
}

pub fn f308(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2228) = {
        let (h2229, _) = f305(Fn7::F376(v0.clone()), {
            let a2231 = {
                let a2230 = v0;
                length_bytes(a2230)
            };
            let a2232 = 1u64;
            nat_add(a2231, a2232)?
        }, (v1, 0u64))?;
        h2229
    };
    Ok(h2228)
}

pub fn f309(v0: u64, v1: u64) -> u64 {
    let a2233 = v0;
    let a2234 = v1;
    if nat_lt(a2233, a2234) {
        v1
    } else {
        v0
    }
}

pub fn f310(v0: Bytes, p2297: u64, v2: u64, v3: bool, p2298: u64, p2299: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2296 = (p2297, (p2298, p2299));
    {
        let mut m2296 = m2296;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2296;
                let m2235 = v4;
                if m2235 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2235 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2236 = v7;
                    let a2237 = 34u64;
                    if nat_eq(a2236, a2237) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                            let a2238 = v1;
                            let a2239 = 1u64;
                            nat_add(a2238, a2239)?
                        }))))
                    } else {
                        let a2240 = v7;
                        let a2241 = 256u64;
                        if nat_eq(a2240, a2241) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2242 = v7;
                            let a2243 = 92u64;
                            if nat_eq(a2242, a2243) {
                                let v8: u64 = f100(v0.clone(), {
                                    let a2244 = v1;
                                    let a2245 = 1u64;
                                    nat_add(a2244, a2245)?
                                })?;
                                let m2246 = f377(v8);
                                match m2246 {
                                    None => {
                                        let a2251 = v8;
                                        let a2252 = 10u64;
                                        if nat_eq(a2251, a2252) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f378(v1)?)))
                                            } else {
                                                let m2255 = f379(v0.clone(), {
                                                    let a2253 = v1;
                                                    let a2254 = 1u64;
                                                    nat_add(a2253, a2254)?
                                                }, v2, v6, 0u64)?;
                                                match m2255 {
                                                    Ok(v11) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                            let (_, h2256) = v11;
                                                            h2256
                                                        }, (v6, {
                                                            let a2258 = v5;
                                                            let a2259 = f307({
                                                                let (h2257, _) = v11;
                                                                h2257
                                                            });
                                                            append_bytes(a2258, a2259)
                                                        })))
                                                    }
                                                    Err(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v12: u64 = {
                                                let a2260 = v8;
                                                let a2261 = 120u64;
                                                if nat_eq(a2260, a2261) {
                                                    2u64
                                                } else {
                                                    let a2262 = v8;
                                                    let a2263 = 117u64;
                                                    if nat_eq(a2262, a2263) {
                                                        4u64
                                                    } else {
                                                        let a2264 = v8;
                                                        let a2265 = 85u64;
                                                        if nat_eq(a2264, a2265) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2266 = v12;
                                            let a2267 = 0u64;
                                            if nat_eq(a2266, a2267) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2270 = f380(v0.clone(), {
                                                    let a2268 = v1;
                                                    let a2269 = 2u64;
                                                    nat_add(a2268, a2269)?
                                                }, v12, 0u64)?;
                                                match m2270 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v13) => {
                                                        let a2271 = 1114111u64;
                                                        let a2272 = v13;
                                                        if if nat_lt(a2271, a2272) {
                                                            true
                                                        } else {
                                                            let a2273 = 55296u64;
                                                            let a2274 = v13;
                                                            if nat_le(a2273, a2274) {
                                                                let a2275 = v13;
                                                                let a2276 = 57343u64;
                                                                nat_le(a2275, a2276)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                                let a2279 = {
                                                                    let a2277 = v1;
                                                                    let a2278 = 2u64;
                                                                    nat_add(a2277, a2278)?
                                                                };
                                                                let a2280 = v12;
                                                                nat_add(a2279, a2280)?
                                                            }, (v6, {
                                                                let a2281 = v5;
                                                                let a2282 = f284(v13)?;
                                                                append_bytes(a2281, a2282)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                            let a2247 = v1;
                                            let a2248 = 2u64;
                                            nat_add(a2247, a2248)?
                                        }, (v6, {
                                            let a2249 = v5;
                                            let a2250 = v9;
                                            append_bytes(a2249, a2250)
                                        })))
                                    }
                                }
                            } else {
                                if if f153(v7) {
                                    true
                                } else {
                                    let a2283 = v7;
                                    let a2284 = 10u64;
                                    nat_eq(a2283, a2284)
                                } {
                                    let v14: u64 = f157(v0.clone(), v1)?;
                                    let a2285 = f100(v0.clone(), v14)?;
                                    let a2286 = 10u64;
                                    if nat_eq(a2285, a2286) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f378(v1)?)))
                                        } else {
                                            let m2287 = f379(v0.clone(), v14, v2, v6, 0u64)?;
                                            match m2287 {
                                                Ok(v16) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                        let (_, h2288) = v16;
                                                        h2288
                                                    }, (v6, {
                                                        let a2290 = v5;
                                                        let a2291 = f318({
                                                            let (h2289, _) = v16;
                                                            h2289
                                                        });
                                                        append_bytes(a2290, a2291)
                                                    })))
                                                }
                                                Err(v15) => {
                                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v15)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v14, (v6, {
                                            let a2292 = v5;
                                            let a2293 = f232(v0.clone(), v1, v14);
                                            append_bytes(a2292, a2293)
                                        })))
                                    }
                                } else {
                                    let v17: u64 = f381(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v17, (v6, {
                                        let a2294 = v5;
                                        let a2295 = f232(v0.clone(), v1, v17);
                                        append_bytes(a2294, a2295)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: Bytes, p2329: u64, v2: u64, v3: bool, p2330: u64, p2331: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2328 = (p2329, (p2330, p2331));
    {
        let mut m2328 = m2328;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2328;
                let m2300 = v4;
                if m2300 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2300 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2301 = v7;
                    let a2302 = 39u64;
                    if nat_eq(a2301, a2302) {
                        let a2305 = f100(v0.clone(), {
                            let a2303 = v1;
                            let a2304 = 1u64;
                            nat_add(a2303, a2304)?
                        })?;
                        let a2306 = 39u64;
                        if nat_eq(a2305, a2306) {
                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                let a2307 = v1;
                                let a2308 = 2u64;
                                nat_add(a2307, a2308)?
                            }, (v6, {
                                let a2309 = v5;
                                let a2310 = Bytes::lit(&[0x27]);
                                append_bytes(a2309, a2310)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                                let a2311 = v1;
                                let a2312 = 1u64;
                                nat_add(a2311, a2312)?
                            }))))
                        }
                    } else {
                        let a2313 = v7;
                        let a2314 = 256u64;
                        if nat_eq(a2313, a2314) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f153(v7) {
                                true
                            } else {
                                let a2315 = v7;
                                let a2316 = 10u64;
                                nat_eq(a2315, a2316)
                            } {
                                let v8: u64 = f157(v0.clone(), v1)?;
                                let a2317 = f100(v0.clone(), v8)?;
                                let a2318 = 10u64;
                                if nat_eq(a2317, a2318) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f378(v1)?)))
                                    } else {
                                        let m2319 = f379(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2319 {
                                            Ok(v10) => {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let (_, h2320) = v10;
                                                    h2320
                                                }, (v6, {
                                                    let a2322 = v5;
                                                    let a2323 = f318({
                                                        let (h2321, _) = v10;
                                                        h2321
                                                    });
                                                    append_bytes(a2322, a2323)
                                                })))
                                            }
                                            Err(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v8, (v6, {
                                        let a2324 = v5;
                                        let a2325 = f232(v0.clone(), v1, v8);
                                        append_bytes(a2324, a2325)
                                    })))
                                }
                            } else {
                                let v11: u64 = f382(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                    let a2326 = v5;
                                    let a2327 = f232(v0.clone(), v1, v11);
                                    append_bytes(a2326, a2327)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2332, _) = f383(Fn8::F384(v2, v0.clone()), {
            let a2334 = {
                let a2333 = v0.clone();
                length_bytes(a2333)
            };
            let a2335 = 1u64;
            nat_add(a2334, a2335)?
        }, (v1, false))?;
        h2332
    };
    let (_, h2336) = v3;
    if h2336 {
        if f99(v0, {
            let (h2337, _) = v3;
            h2337
        })? {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2338, _) = v3;
                h2338
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2339, _) = v3;
                h2339
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2340, _) = v3;
            h2340
        }))
    }
}

pub fn f313(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a2341 = v3;
    let a2342 = 34u64;
    if if nat_eq(a2341, a2342) {
        true
    } else {
        let a2343 = v3;
        let a2344 = 39u64;
        nat_eq(a2343, a2344)
    } {
        f227(v0, v1, v2, false)
    } else {
        let m2345 = f160(v0.clone(), v1)?;
        match m2345 {
            None => {
                let a2346 = v3;
                let a2347 = 91u64;
                if if nat_eq(a2346, a2347) {
                    true
                } else {
                    let a2348 = v3;
                    let a2349 = 123u64;
                    nat_eq(a2348, a2349)
                } {
                    Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f229(v0.clone(), v1, true)? {
                        let v5: u64 = f230(v0.clone(), v1, true)?;
                        Ok(Ok::<(Bytes, u64), Bytes>((f232(v0, v1, v5), v5)))
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

pub fn f314(v0: u64) -> bool {
    if f301(v0) {
        true
    } else {
        let a2350 = v0;
        let a2351 = 45u64;
        if if if if nat_eq(a2350, a2351) {
            true
        } else {
            let a2352 = v0;
            let a2353 = 63u64;
            nat_eq(a2352, a2353)
        } {
            true
        } else {
            let a2354 = v0;
            let a2355 = 58u64;
            if nat_eq(a2354, a2355) {
                true
            } else {
                let a2356 = v0;
                let a2357 = 35u64;
                nat_eq(a2356, a2357)
            }
        } {
            true
        } else {
            let a2358 = v0;
            let a2359 = 38u64;
            if if nat_eq(a2358, a2359) {
                true
            } else {
                let a2360 = v0;
                let a2361 = 42u64;
                nat_eq(a2360, a2361)
            } {
                true
            } else {
                let a2362 = v0;
                let a2363 = 33u64;
                if nat_eq(a2362, a2363) {
                    true
                } else {
                    let a2364 = v0;
                    let a2365 = 124u64;
                    nat_eq(a2364, a2365)
                }
            }
        } {
            true
        } else {
            let a2366 = v0;
            let a2367 = 62u64;
            if if if nat_eq(a2366, a2367) {
                true
            } else {
                let a2368 = v0;
                let a2369 = 39u64;
                nat_eq(a2368, a2369)
            } {
                true
            } else {
                let a2370 = v0;
                let a2371 = 34u64;
                if nat_eq(a2370, a2371) {
                    true
                } else {
                    let a2372 = v0;
                    let a2373 = 37u64;
                    nat_eq(a2372, a2373)
                }
            } {
                true
            } else {
                let a2374 = v0;
                let a2375 = 64u64;
                if nat_eq(a2374, a2375) {
                    true
                } else {
                    let a2376 = v0;
                    let a2377 = 96u64;
                    nat_eq(a2376, a2377)
                }
            }
        }
    }
}

pub fn f315(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f385(v1, v0, v2)
}

pub fn f316(v0: u64) -> R<Bytes> {
    f81(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f317(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2378, _) = f305(Fn7::F386(v0.clone()), {
        let a2380 = {
            let a2379 = v0;
            length_bytes(a2379)
        };
        let a2381 = 1u64;
        nat_add(a2380, a2381)?
    }, (v1, 0u64))?;
    Ok(h2378)
}

pub fn f318(v0: u64) -> Bytes {
    let a2382 = v0;
    let a2383 = 0u64;
    if nat_eq(a2382, a2383) {
        Bytes::lit(&[0x20])
    } else {
        f307(v0)
    }
}

pub fn f319(v0: Bytes, v1: Str) -> bool {
    f35(v0, f8(v1))
}

pub fn f320(v0: Bytes, v1: u64, v2: u64) -> R<Result<Adt14, Bytes>> {
    let v3: (u64, u64) = f387(v0.clone(), v1, 2u64)?;
    let a2386 = {
        let (h2384, _) = v3;
        h2384
    };
    let a2387 = {
        let a2385 = v0.clone();
        length_bytes(a2385)
    };
    if nat_eq(a2386, a2387) {
        if f388(false, {
            let (_, h2388) = v3;
            h2388
        }) {
            Ok(Ok::<Adt14, Bytes>(Adt14::C1(false, {
                let (_, h2389) = v3;
                h2389
            })))
        } else {
            Ok(Err::<Adt14, Bytes>(f323(v2)?))
        }
    } else {
        Ok(Ok::<Adt14, Bytes>(Adt14::C3(v0)))
    }
}

pub fn f321(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2390 = v0.clone();
        length_bytes(a2390)
    };
    let v2: u64 = f100(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2391 = v2;
        let a2392 = 43u64;
        if if nat_eq(a2391, a2392) {
            true
        } else {
            let a2393 = v2;
            let a2394 = 45u64;
            nat_eq(a2393, a2394)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f389(v0.clone(), v3)?;
    let v5: bool = {
        let a2395 = f100(v0.clone(), v4)?;
        let a2396 = 46u64;
        nat_eq(a2395, a2396)
    };
    let v6: u64 = if v5 {
        let a2397 = v4;
        let a2398 = 1u64;
        nat_add(a2397, a2398)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f389(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f100(v0.clone(), v7)?;
    let v9: bool = {
        let a2399 = v8;
        let a2400 = 101u64;
        if nat_eq(a2399, a2400) {
            true
        } else {
            let a2401 = v8;
            let a2402 = 69u64;
            nat_eq(a2401, a2402)
        }
    };
    let v10: u64 = f100(v0.clone(), {
        let a2403 = v7;
        let a2404 = 1u64;
        nat_add(a2403, a2404)?
    })?;
    let v11: bool = if v9 {
        let a2405 = v10;
        let a2406 = 43u64;
        if nat_eq(a2405, a2406) {
            true
        } else {
            let a2407 = v10;
            let a2408 = 45u64;
            nat_eq(a2407, a2408)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2409 = v7;
            let a2410 = 2u64;
            nat_add(a2409, a2410)?
        } else {
            let a2411 = v7;
            let a2412 = 1u64;
            nat_add(a2411, a2412)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f389(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f232(v0.clone(), v3, v4);
    let v15: u64 = f390(v14.clone())?;
    let a2413 = v13;
    let a2414 = v1;
    if if if nat_eq(a2413, a2414) {
        let a2415 = v3;
        let a2416 = v4;
        if nat_lt(a2415, a2416) {
            true
        } else {
            if v5 {
                let a2417 = v6;
                let a2418 = v7;
                nat_lt(a2417, a2418)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f165(v9) {
            true
        } else {
            let a2419 = v12;
            let a2420 = v13;
            nat_lt(a2419, a2420)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2421 = v2;
            let a2422 = 45u64;
            if nat_eq(a2421, a2422) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2424 = v15;
            let a2425 = {
                let a2423 = v14.clone();
                length_bytes(a2423)
            };
            if nat_eq(a2424, a2425) {
                Bytes::lit(&[0x30])
            } else {
                f232(v14.clone(), v15, {
                    let a2426 = v14;
                    length_bytes(a2426)
                })
            }
        }, List::cons({
            let a2427 = v6;
            let a2428 = v7;
            if nat_lt(a2427, a2428) {
                let a2429 = Bytes::lit(&[0x2e]);
                let a2430 = f232(v0.clone(), v6, v7);
                append_bytes(a2429, a2430)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2433 = if if v11 {
                let a2431 = v10;
                let a2432 = 45u64;
                nat_eq(a2431, a2432)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2434 = f232(v0, v12, v13);
            append_bytes(a2433, a2434)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f322(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2435 = f133(v0.clone(), v1)?;
        let a2436 = 45u64;
        nat_eq(a2435, a2436)
    };
    let v3: u64 = if v2 {
        let a2437 = v1;
        let a2438 = 1u64;
        nat_add(a2437, a2438)?
    } else {
        v1
    };
    let v4: u64 = f389(v0.clone(), v3)?;
    let v5: u64 = {
        let a2439 = v4;
        let a2440 = v3;
        nat_sub(a2439, a2440)
    };
    let v6: bool = {
        let a2441 = f133(v0.clone(), v4)?;
        let a2442 = 46u64;
        nat_eq(a2441, a2442)
    };
    let v7: u64 = if v6 {
        let a2443 = v4;
        let a2444 = 1u64;
        nat_add(a2443, a2444)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f389(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f133(v0.clone(), v8)?;
    let v10: bool = {
        let a2445 = v9;
        let a2446 = 101u64;
        if nat_eq(a2445, a2446) {
            true
        } else {
            let a2447 = v9;
            let a2448 = 69u64;
            nat_eq(a2447, a2448)
        }
    };
    let v11: u64 = f133(v0.clone(), {
        let a2449 = v8;
        let a2450 = 1u64;
        nat_add(a2449, a2450)?
    })?;
    let v12: bool = f291(v10, f293({
        let a2451 = v11;
        let a2452 = 43u64;
        nat_eq(a2451, a2452)
    }, {
        let a2453 = v11;
        let a2454 = 45u64;
        nat_eq(a2453, a2454)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2455 = v8;
            let a2456 = 2u64;
            nat_add(a2455, a2456)?
        } else {
            let a2457 = v8;
            let a2458 = 1u64;
            nat_add(a2457, a2458)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f389(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2459 = v14;
        let a2460 = v13;
        nat_sub(a2459, a2460)
    });
    let v16: u64 = f390(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2462 = {
            let a2461 = v15;
            length_bytes(a2461)
        };
        let a2463 = v16;
        nat_sub(a2462, a2463)
    });
    let a2464 = v5;
    let a2465 = 0u64;
    if if nat_eq(a2464, a2465) {
        true
    } else {
        if f291({
            let a2466 = 1u64;
            let a2467 = v5;
            nat_lt(a2466, a2467)
        }, {
            let a2468 = f133(v0.clone(), v3)?;
            let a2469 = 48u64;
            nat_eq(a2468, a2469)
        }) {
            true
        } else {
            if f291(v6, {
                let a2470 = v8;
                let a2471 = v7;
                nat_eq(a2470, a2471)
            }) {
                true
            } else {
                f293(if v10 {
                    let a2472 = v14;
                    let a2473 = v13;
                    nat_eq(a2472, a2473)
                } else {
                    false
                }, {
                    let a2475 = 18u64;
                    let a2476 = {
                        let a2474 = v17.clone();
                        length_bytes(a2474)
                    };
                    nat_lt(a2475, a2476)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2480 = {
            let a2478 = {
                let a2477 = v17.clone();
                length_bytes(a2477)
            };
            let a2479 = 0u64;
            if nat_eq(a2478, a2479) {
                Some(0u64)
            } else {
                f391(v17)?
            }
        };
        match m2480 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2485 = f392(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2481 = v8;
                    let a2482 = v7;
                    nat_sub(a2481, a2482)
                }), f291(v12, {
                    let a2483 = v11;
                    let a2484 = 45u64;
                    nat_eq(a2483, a2484)
                }), v18)?;
                match m2485 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2488 = 19u64;
                        let a2489 = {
                            let a2487 = {
                                let (h2486, _) = v19.clone();
                                h2486
                            };
                            length_bytes(a2487)
                        };
                        if nat_lt(a2488, a2489) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2491 = f391({
                                let (h2490, _) = v19.clone();
                                h2490
                            })?;
                            match m2491 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f291(v2, {
                                        let a2492 = 0u64;
                                        let a2493 = v20;
                                        nat_lt(a2492, a2493)
                                    });
                                    if f388(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2495 = {
                                                let (_, h2494) = v19.clone();
                                                h2494
                                            };
                                            let a2496 = 0u64;
                                            if nat_eq(a2495, a2496) {
                                                Adt14::C1(v21, v20)
                                            } else {
                                                Adt14::C2(v21, v20, {
                                                    let (_, h2497) = v19;
                                                    h2497
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt14)>)
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

pub fn f323(v0: u64) -> R<Bytes> {
    f81(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f324(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f393(v0, v1)
}

pub fn f325(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f394(v0, v1)
}

pub fn f326(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2498 = v0.clone();
    match m2498.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2501 = {
                let (h2499, _) = v2;
                h2499
            };
            let a2502 = {
                let (h2500, _) = v1.clone();
                h2500
            };
            if equal(a2501, a2502) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f327(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f395(f396(Fn9::F397, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2507 = {
        let (h2503, _) = f398(Fn10::F399, {
            let a2505 = {
                let a2504 = v0;
                length_list(a2504)
            };
            let a2506 = 1u64;
            nat_add(a2505, a2506)?
        }, v1)?;
        h2503
    };
    match m2507.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f328(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2512 = {
        let a2509 = f238({
            let (h2508, _) = v0;
            h2508
        })?;
        let a2510 = Bytes::lit(&[0x3a]);
        append_bytes(a2509, a2510)
    };
    let a2513 = {
        let (_, h2511) = v0;
        h2511
    };
    Ok(append_bytes(a2512, a2513))
}

pub fn f329(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f400(v1, v0, v2)
}

pub fn f330(v0: u64) -> bool {
    if f331(v0) {
        true
    } else {
        let a2514 = 65u64;
        let a2515 = v0;
        if if nat_le(a2514, a2515) {
            let a2516 = v0;
            let a2517 = 70u64;
            nat_le(a2516, a2517)
        } else {
            false
        } {
            true
        } else {
            let a2518 = 97u64;
            let a2519 = v0;
            if nat_le(a2518, a2519) {
                let a2520 = v0;
                let a2521 = 102u64;
                nat_le(a2520, a2521)
            } else {
                false
            }
        }
    }
}

pub fn f331(v0: u64) -> bool {
    let a2522 = 48u64;
    let a2523 = v0;
    if nat_le(a2522, a2523) {
        let a2524 = v0;
        let a2525 = 57u64;
        nat_le(a2524, a2525)
    } else {
        false
    }
}

pub fn f332(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a2526 = v2;
    let a2527 = 69u64;
    if if nat_eq(a2526, a2527) {
        true
    } else {
        let a2528 = v2;
        let a2529 = 101u64;
        nat_eq(a2528, a2529)
    } {
        let v3: u64 = f85(v0.clone(), {
            let a2530 = v1;
            let a2531 = 1u64;
            nat_add(a2530, a2531)?
        })?;
        let v4: u64 = {
            let a2532 = v3;
            let a2533 = 43u64;
            if if nat_eq(a2532, a2533) {
                true
            } else {
                let a2534 = v3;
                let a2535 = 45u64;
                nat_eq(a2534, a2535)
            } {
                let a2536 = v1;
                let a2537 = 2u64;
                nat_add(a2536, a2537)?
            } else {
                let a2538 = v1;
                let a2539 = 1u64;
                nat_add(a2538, a2539)?
            }
        };
        let v5: u64 = f247(v0, 0u64, v4)?;
        let a2540 = v4;
        let a2541 = v5;
        if nat_lt(a2540, a2541) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f333(v0: (u64, Bytes), v1: Bytes) -> bool {
    let a2543 = {
        let (h2542, _) = v0;
        h2542
    };
    let a2544 = 3u64;
    if !nat_eq(a2543, a2544) {
        f35({
            let (_, h2545) = v0;
            h2545
        }, v1)
    } else {
        false
    }
}

pub fn f334(v0: Bytes) -> Bytes {
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

pub fn f335(v0: Bytes) -> bool {
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

pub fn f336(p2562: u64, p2563: List<(u64, Bytes)>, p2564: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2561 = (p2562, (p2563, p2564));
    {
        let mut m2561 = m2561;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2561;
                let m2546 = v0;
                if m2546 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2546 - 1;
                    if f250(v1.clone(), Str::lit("}")) {
                        let m2547 = f257(f252(v1), Str::lit(";"));
                        match m2547 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f401(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2548 = f179(v1)?;
                        match m2548 {
                            Ok(v7) => {
                                let v8: bool = f250(v7.clone(), Str::lit("required"));
                                let m2549 = f260(v3, if v8 {
                                    f252(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2549 {
                                    Ok(v10) => {
                                        let m2551 = f256({
                                            let (_, h2550) = v10;
                                            h2550
                                        });
                                        match m2551 {
                                            Ok(v12) => {
                                                if if f250({
                                                    let (_, h2552) = v12.clone();
                                                    h2552
                                                }, Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2554 = f338(f252({
                                                        let (_, h2553) = v12;
                                                        h2553
                                                    }))?;
                                                    match m2554 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v14, List::cons(({
                                                                let (h2555, _) = v12;
                                                                h2555
                                                            }, (f345({
                                                                let (h2556, _) = v10;
                                                                h2556
                                                            }), 0u64)), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2558 = f257({
                                                        let (_, h2557) = v12;
                                                        h2557
                                                    }, Str::lit(";"));
                                                    match m2558 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v16, List::cons(({
                                                                let (h2559, _) = v12;
                                                                h2559
                                                            }, (f345({
                                                                let (h2560, _) = v10;
                                                                h2560
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
                Ok(next) => { m2561 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(p2577: u64, p2578: List<(u64, Bytes)>, p2579: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2576 = (p2577, (p2578, p2579));
    {
        let mut m2576 = m2576;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2576;
                let m2565 = v0;
                if m2565 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2565 - 1;
                    if f250(v1.clone(), Str::lit("}")) {
                        let m2566 = f257(f252(v1), Str::lit(";"));
                        match m2566 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f401(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2567 = f179(v1)?;
                        match m2567 {
                            Ok(v7) => {
                                let v8: List<(u64, Bytes)> = if if f250(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f250(v7.clone(), Str::lit("inherit"))
                                } {
                                    f252(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<(u64, Bytes)> = if f250(v8.clone(), Str::lit("readonly")) {
                                    f252(v8)
                                } else {
                                    v8
                                };
                                if f250(v9.clone(), Str::lit("attribute")) {
                                    let m2568 = f260(v3, f252(v9), 0u64)?;
                                    match m2568 {
                                        Ok(v11) => {
                                            let m2570 = f256({
                                                let (_, h2569) = v11;
                                                h2569
                                            });
                                            match m2570 {
                                                Ok(v13) => {
                                                    let m2572 = f257({
                                                        let (_, h2571) = v13;
                                                        h2571
                                                    }, Str::lit(";"));
                                                    match m2572 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v15, List::cons(({
                                                                let (h2573, _) = v13;
                                                                h2573
                                                            }, (f345({
                                                                let (h2574, _) = v11;
                                                                h2574
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
                                    let m2575 = f338(v7)?;
                                    match m2575 {
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
                Ok(next) => { m2576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    Ok(f251({
        let a2581 = {
            let a2580 = v0.clone();
            length_list(a2580)
        };
        let a2582 = 1u64;
        nat_add(a2581, a2582)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f339(v0: List<(u64, Bytes)>) -> Bytes {
    let m2583 = v0;
    match m2583.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons({
                let (_, h2584) = v1;
                h2584
            }, List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f340(v0: Bytes) -> Bytes {
    if f141(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2586 = {
                let a2585 = v0;
                length_bytes(a2585)
            };
            let a2587 = 1u64;
            nat_sub(a2586, a2587)
        })
    } else {
        v0
    }
}

pub fn f341(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), v1: List<(u64, Bytes)>) -> Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes> {
    if f250(v1.clone(), Str::lit("?")) {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((f342(Str::lit("Option"), v0), f252(v1)))
    } else {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v0, v1))
    }
}

pub fn f342(v0: Str, v1: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let v2: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f345(v1);
    (0u64, (f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2588 = v0;
        utf8_encode(a2588)
    }, List::cons(f8(Str::lit(" ")), List::cons(f198(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), (f199(v2.clone()), (f196(v2), Bytes::lit(&[])))))
}

pub fn f343(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (Bytes::lit(&[]), (List::<Bytes>::nil(), (v0, Bytes::lit(&[])))))
}

pub fn f344(v0: Bytes) -> Option<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))> {
    let m2589 = f402(v0, f403());
    match m2589 {
        None => {
            None::<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))>
        }
        Some(v1) => {
            Some({
                let (h2590, _) = v1;
                if h2590 {
                    f259({
                        let (_, h2591) = v1;
                        h2591
                    })
                } else {
                    f343({
                        let (_, h2592) = v1;
                        h2592
                    })
                }
            })
        }
    }
}

pub fn f345(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let a2593 = f197(v0.clone());
    let a2594 = 0u64;
    if nat_eq(a2593, a2594) {
        v0
    } else {
        (0u64, (Bytes::lit(&[]), (f199(v0.clone()), (if f35(f196(v0.clone()), Bytes::lit(&[])) {
            f404()
        } else {
            f196(v0)
        }, Bytes::lit(&[])))))
    }
}

pub fn f346(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (_, h2595) = {
        let (_, h2596) = {
            let (_, h2597) = {
                let (_, h2598) = v0;
                h2598
            };
            h2597
        };
        h2596
    };
    h2595
}

pub fn f347(v0: (Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64)), v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m2599 = v1;
    match m2599.uncons() {
        None => {
            Ok(List::cons(v0, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()))
        }
        Some((v2, v3)) => {
            if f405({
                let a2602 = {
                    let a2601 = {
                        let (h2600, _) = v0.clone();
                        h2600
                    };
                    length_bytes(a2601)
                };
                let a2603 = 1u64;
                nat_add(a2602, a2603)?
            }, {
                let (h2604, _) = v0.clone();
                h2604
            }, {
                let (h2605, _) = v2.clone();
                h2605
            }, 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f347(v0, v3)?))
            }
        }
    }
}

pub fn f348(v0: List<Adt12>, v1: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2606 = f117(v0.clone(), 1u64, v1.clone());
    match m2606 {
        None => {
            f271(v0, 6u64, v1)
        }
        Some(v2) => {
            f269(f270(v2), f271(v0, 6u64, v1))
        }
    }
}

pub fn f349(v0: Bytes) -> R<Option<Bytes>> {
    let m2608 = f406({
        let a2607 = v0.clone();
        length_bytes(a2607)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m2608 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2610 = {
                let a2609 = v1.clone();
                length_bytes(a2609)
            };
            let a2611 = 0u64;
            if nat_eq(a2610, a2611) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f350(p2621: u64, v1: Bytes, p2622: u64) -> R<bool> {
    let m2620 = (p2621, p2622);
    {
        let mut m2620 = m2620;
        loop {
            let step = {
                let (v0, v2) = m2620;
                let m2612 = v0;
                if m2612 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2612 - 1;
                    let m2615 = {
                        let a2613 = v1.clone();
                        let a2614 = v2;
                        index_bytes(a2613, a2614)
                    };
                    match m2615 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2616 = f41(v4)?;
                            let a2617 = 128u64;
                            if nat_lt(a2616, a2617) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2618 = v2;
                                    let a2619 = 1u64;
                                    nat_add(a2618, a2619)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f219(v0.clone(), 0u64, {
        let a2623 = v0;
        length_bytes(a2623)
    }, false, Bytes::lit(&[]))?;
    let m2626 = {
        let a2624 = v1.clone();
        let a2625 = 0u64;
        index_bytes(a2624, a2625)
    };
    match m2626 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f294(v3) {
                let a2633 = f32({
                    let a2627 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2627)
                }, {
                    let a2628 = v3;
                    let a2629 = 65u64;
                    nat_sub(a2628, a2629)
                }, 1u64);
                let a2634 = f32(v1.clone(), 1u64, {
                    let a2631 = {
                        let a2630 = v1;
                        length_bytes(a2630)
                    };
                    let a2632 = 1u64;
                    nat_sub(a2631, a2632)
                });
                Ok(append_bytes(a2633, a2634))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f352(v0: Bytes) -> R<bool> {
    let m2637 = {
        let a2635 = v0;
        let a2636 = 0u64;
        index_bytes(a2635, a2636)
    };
    match m2637 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f293(f292(f41(v1)?), f294(f41(v1)?)))
        }
    }
}

pub fn f353(p2650: u64, v1: Bytes, p2651: u64, p2652: Bytes) -> R<Option<Bytes>> {
    let m2649 = (p2650, (p2651, p2652));
    {
        let mut m2649 = m2649;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2649;
                let m2638 = v0;
                if m2638 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2638 - 1;
                    let a2640 = {
                        let a2639 = v1.clone();
                        length_bytes(a2639)
                    };
                    let a2641 = v2;
                    if nat_le(a2640, a2641) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2642 = f277(v1.clone(), v2)?;
                        match m2642 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2644 = v2;
                                    let a2645 = {
                                        let (_, h2643) = v5;
                                        h2643
                                    };
                                    nat_add(a2644, a2645)?
                                }, {
                                    let a2647 = v3;
                                    let a2648 = f407({
                                        let (h2646, _) = v5;
                                        h2646
                                    });
                                    append_bytes(a2647, a2648)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2655 = {
        let a2653 = v0;
        let a2654 = v1;
        index_bytes(a2653, a2654)
    };
    match m2655 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f291({
                let a2656 = 128u64;
                let a2657 = v3;
                nat_le(a2656, a2657)
            }, {
                let a2658 = v3;
                let a2659 = 191u64;
                nat_le(a2658, a2659)
            }) {
                Ok(Some({
                    let a2660 = v3;
                    let a2661 = 128u64;
                    nat_sub(a2660, a2661)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f355(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f291({
        let a2662 = v1;
        let a2663 = v0;
        nat_le(a2662, a2663)
    }, f291({
        let a2664 = v0;
        let a2665 = v2;
        nat_le(a2664, a2665)
    }, f293({
        let a2666 = v0;
        let a2667 = 55296u64;
        nat_lt(a2666, a2667)
    }, {
        let a2668 = 57343u64;
        let a2669 = v0;
        nat_lt(a2668, a2669)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f356() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f357(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2670 = v1;
        let a2671 = 3u64;
        nat_mul(a2670, a2671)?
    };
    let a2682 = {
        let a2678 = {
            let a2672 = f133(v0.clone(), v2)?;
            let a2673 = 16384u64;
            nat_mul(a2672, a2673)?
        };
        let a2679 = {
            let a2676 = f133(v0.clone(), {
                let a2674 = v2;
                let a2675 = 1u64;
                nat_add(a2674, a2675)?
            })?;
            let a2677 = 128u64;
            nat_mul(a2676, a2677)?
        };
        nat_add(a2678, a2679)?
    };
    let a2683 = f133(v0, {
        let a2680 = v2;
        let a2681 = 2u64;
        nat_add(a2680, a2681)?
    })?;
    nat_add(a2682, a2683)
}

pub fn f358(v0: u64) -> R<bool> {
    f204(v0, f408())
}

pub fn f359(v0: u64) -> R<bool> {
    f204(v0, f409())
}

pub fn f360(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f410(v1, v0, v2)
}

pub fn f361(v0: Bytes, v1: u64) -> Bytes {
    f32(v0, v1, 1u64)
}

pub fn f362() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f363() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f364(v0: u64) -> R<Bytes> {
    Ok(f361(f411(), {
        let a2689 = {
            let a2687 = {
                let a2684 = v0;
                let a2685 = 64u64;
                let a2686 = 0u64;
                nat_rem(a2684, a2685, a2686)
            };
            let a2688 = 2u64;
            nat_mul(a2687, a2688)?
        };
        let a2690 = 1u64;
        nat_add(a2689, a2690)?
    }))
}

pub fn f365() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f366(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f412(f85(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2691 = v3;
            let a2692 = 1u64;
            nat_add(a2691, a2692)?
        }))
    }
}

pub fn f367(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f368() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f369(v0: Bytes) -> R<Option<Bytes>> {
    let m2693 = f127(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2693 {
        None => {
            let m2694 = f127(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2694 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2695 = f127(f413(), v3.clone(), 0u64)?;
                    match m2695 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2697 = v4;
                                let a2698 = {
                                    let a2696 = v3;
                                    length_bytes(a2696)
                                };
                                nat_add(a2697, a2698)?
                            };
                            let m2699 = f127(f413(), f8(Str::lit(" ")), v5)?;
                            match m2699 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f413(), v5, {
                                        let a2700 = v6;
                                        let a2701 = v5;
                                        nat_sub(a2700, a2701)
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

pub fn f370(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2702 = 0u64;
    let a2703 = v1;
    if if nat_lt(a2702, a2703) {
        f165({
            let a2706 = f100(v0, {
                let a2704 = v1;
                let a2705 = 1u64;
                nat_sub(a2704, a2705)
            })?;
            let a2707 = 10u64;
            nat_eq(a2706, a2707)
        })
    } else {
        false
    } {
        Ok(Some({
            let a2708 = v1;
            let a2709 = 1u64;
            nat_sub(a2708, a2709)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f371(v0: Bytes, v1: u64, v2: u64) -> R<Result<(u64, u64), Bytes>> {
    let v3: (u64, u64) = {
        let (h2710, _) = f305(Fn7::F414(v2, v0.clone()), {
            let a2712 = {
                let a2711 = v0.clone();
                length_bytes(a2711)
            };
            let a2713 = 1u64;
            nat_add(a2712, a2713)?
        }, (v1, 0u64))?;
        h2710
    };
    let v4: u64 = f372(v0.clone(), {
        let (h2714, _) = v3;
        h2714
    }, v2)?;
    let a2715 = f100(v0, v4)?;
    let a2716 = 9u64;
    if if nat_eq(a2715, a2716) {
        let a2720 = {
            let a2718 = v4;
            let a2719 = {
                let (h2717, _) = v3;
                h2717
            };
            nat_sub(a2718, a2719)
        };
        let a2721 = v2;
        nat_lt(a2720, a2721)
    } else {
        false
    } {
        Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Ok::<(u64, u64), Bytes>(({
            let (_, h2722) = v3;
            h2722
        }, {
            let (h2723, _) = v3;
            h2723
        })))
    }
}

pub fn f372(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2724, _) = f124(Fn4::F415(v2, v0.clone(), v1), {
        let a2726 = {
            let a2725 = v0;
            length_bytes(a2725)
        };
        let a2727 = 1u64;
        nat_add(a2726, a2727)?
    }, v1)?;
    Ok(h2724)
}

pub fn f373(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f372(v0.clone(), v1, v2)?;
    let a2730 = {
        let a2728 = v3;
        let a2729 = v1;
        nat_sub(a2728, a2729)
    };
    let a2731 = v2;
    if nat_eq(a2730, a2731) {
        Ok(f165({
            let a2732 = f100(v0, v3)?;
            let a2733 = 256u64;
            nat_eq(a2732, a2733)
        }))
    } else {
        Ok(false)
    }
}

pub fn f374(v0: Bytes, p2747: u64, v2: u64, v3: bool, p2748: u64, p2749: Bytes, p2750: u64) -> R<Result<(Bytes, (u64, u64)), Bytes>> {
    let m2746 = (p2747, (p2748, (p2749, p2750)));
    {
        let mut m2746 = m2746;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m2746;
                let m2734 = v4;
                if m2734 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Err::<(Bytes, (u64, u64)), Bytes>(f81(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m2734 - 1;
                    let v8: u64 = f372(v0.clone(), v1, v2)?;
                    let v9: bool = f165(f153(f100(v0.clone(), v8)?));
                    let v10: u64 = f234(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f307(v6), List::cons(f232(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m2735 = f371(v0.clone(), f163(v0.clone(), v10)?, v2)?;
                    match m2735 {
                        Ok(v13) => {
                            let v14: u64 = {
                                let (_, h2736) = v13;
                                h2736
                            };
                            let v15: u64 = f372(v0.clone(), v14, v2)?;
                            if f373(v0.clone(), v14, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>((v14, (v7, ({
                                    let a2740 = v11;
                                    let a2741 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f165(f153(f100(v0.clone(), v15)?))
                                    } else {
                                        false
                                    } {
                                        let a2738 = {
                                            let (h2737, _) = v13;
                                            h2737
                                        };
                                        let a2739 = 0u64;
                                        if nat_eq(a2738, a2739) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a2740, a2741)
                                }, {
                                    let (h2742, _) = v13;
                                    h2742
                                }))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Ok::<(Bytes, (u64, u64)), Bytes>((v11, ({
                                    let (h2743, _) = v13;
                                    h2743
                                }, {
                                    let a2744 = f100(v0.clone(), v15)?;
                                    let a2745 = 256u64;
                                    if nat_eq(a2744, a2745) {
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
                Ok(next) => { m2746 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f156(v0.clone(), {
        let (h2751, _) = v1;
        h2751
    })?;
    let a2752 = f100(v0, v2)?;
    let a2753 = 10u64;
    if nat_eq(a2752, a2753) {
        Ok(Some(({
            let a2754 = v2;
            let a2755 = 1u64;
            nat_add(a2754, a2755)?
        }, f309({
            let (_, h2756) = v1;
            h2756
        }, {
            let a2758 = v2;
            let a2759 = {
                let (h2757, _) = v1;
                h2757
            };
            nat_sub(a2758, a2759)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f376(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f416(v0, v1)
}

pub fn f377(v0: u64) -> Option<Bytes> {
    let a2760 = v0;
    let a2761 = 48u64;
    if nat_eq(a2760, a2761) {
        Some(Bytes::lit(&[0x00]))
    } else {
        let a2762 = v0;
        let a2763 = 97u64;
        if nat_eq(a2762, a2763) {
            Some(Bytes::lit(&[0x07]))
        } else {
            let a2764 = v0;
            let a2765 = 98u64;
            if nat_eq(a2764, a2765) {
                Some(Bytes::lit(&[0x08]))
            } else {
                let a2766 = v0;
                let a2767 = 116u64;
                if if nat_eq(a2766, a2767) {
                    true
                } else {
                    let a2768 = v0;
                    let a2769 = 9u64;
                    nat_eq(a2768, a2769)
                } {
                    Some(Bytes::lit(&[0x09]))
                } else {
                    let a2770 = v0;
                    let a2771 = 110u64;
                    if nat_eq(a2770, a2771) {
                        Some(Bytes::lit(&[0x0a]))
                    } else {
                        let a2772 = v0;
                        let a2773 = 118u64;
                        if nat_eq(a2772, a2773) {
                            Some(Bytes::lit(&[0x0b]))
                        } else {
                            let a2774 = v0;
                            let a2775 = 102u64;
                            if nat_eq(a2774, a2775) {
                                Some(Bytes::lit(&[0x0c]))
                            } else {
                                let a2776 = v0;
                                let a2777 = 114u64;
                                if nat_eq(a2776, a2777) {
                                    Some(Bytes::lit(&[0x0d]))
                                } else {
                                    let a2778 = v0;
                                    let a2779 = 101u64;
                                    if nat_eq(a2778, a2779) {
                                        Some(Bytes::lit(&[0x1b]))
                                    } else {
                                        let a2780 = v0;
                                        let a2781 = 32u64;
                                        if nat_eq(a2780, a2781) {
                                            Some(Bytes::lit(&[0x20]))
                                        } else {
                                            let a2782 = v0;
                                            let a2783 = 34u64;
                                            if nat_eq(a2782, a2783) {
                                                Some(Bytes::lit(&[0x22]))
                                            } else {
                                                let a2784 = v0;
                                                let a2785 = 47u64;
                                                if nat_eq(a2784, a2785) {
                                                    Some(Bytes::lit(&[0x2f]))
                                                } else {
                                                    let a2786 = v0;
                                                    let a2787 = 92u64;
                                                    if nat_eq(a2786, a2787) {
                                                        Some(Bytes::lit(&[0x5c]))
                                                    } else {
                                                        let a2788 = v0;
                                                        let a2789 = 78u64;
                                                        if nat_eq(a2788, a2789) {
                                                            Some(Bytes::lit(&[0xc2, 0x85]))
                                                        } else {
                                                            let a2790 = v0;
                                                            let a2791 = 95u64;
                                                            if nat_eq(a2790, a2791) {
                                                                Some(Bytes::lit(&[0xc2, 0xa0]))
                                                            } else {
                                                                let a2792 = v0;
                                                                let a2793 = 76u64;
                                                                if nat_eq(a2792, a2793) {
                                                                    Some(Bytes::lit(&[0xe2, 0x80, 0xa8]))
                                                                } else {
                                                                    let a2794 = v0;
                                                                    let a2795 = 80u64;
                                                                    if nat_eq(a2794, a2795) {
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

pub fn f378(v0: u64) -> R<Bytes> {
    f81(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f379(v0: Bytes, p2810: u64, v2: u64, p2811: u64, p2812: u64) -> R<Result<(u64, u64), Bytes>> {
    let m2809 = (p2810, (p2811, p2812));
    {
        let mut m2809 = m2809;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2809;
                let m2796 = v3;
                if m2796 == 0 {
                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m2796 - 1;
                    let v6: u64 = {
                        let a2797 = v1;
                        let a2798 = 1u64;
                        nat_add(a2797, a2798)?
                    };
                    let v7: u64 = f156(v0.clone(), v6)?;
                    let v8: u64 = f157(v0.clone(), v7)?;
                    let v9: u64 = f100(v0.clone(), v8)?;
                    if f99(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a2799 = v9;
                        let a2800 = 10u64;
                        if nat_eq(a2799, a2800) {
                            Ok::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>((v8, (v5, {
                                let a2801 = v4;
                                let a2802 = 1u64;
                                nat_add(a2801, a2802)?
                            })))
                        } else {
                            let a2803 = v9;
                            let a2804 = 256u64;
                            if nat_eq(a2803, a2804) {
                                Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a2807 = {
                                    let a2805 = v7;
                                    let a2806 = v6;
                                    nat_sub(a2805, a2806)
                                };
                                let a2808 = v2;
                                if nat_lt(a2807, a2808) {
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
                Ok(next) => { m2809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: Bytes, p2822: u64, p2823: u64, p2824: u64) -> R<Option<u64>> {
    let m2821 = (p2822, (p2823, p2824));
    {
        let mut m2821 = m2821;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2821;
                let m2813 = v2;
                if m2813 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2813 - 1;
                    let m2814 = f417(f100(v0.clone(), v1)?);
                    match m2814 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a2815 = v1;
                                let a2816 = 1u64;
                                nat_add(a2815, a2816)?
                            }, (v4, {
                                let a2819 = {
                                    let a2817 = v3;
                                    let a2818 = 16u64;
                                    nat_mul(a2817, a2818)?
                                };
                                let a2820 = v5;
                                nat_add(a2819, a2820)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381(v0: Bytes, v1: u64) -> R<u64> {
    let (h2825, _) = f124(Fn4::F418(v0.clone()), {
        let a2827 = {
            let a2826 = v0;
            length_bytes(a2826)
        };
        let a2828 = 1u64;
        nat_add(a2827, a2828)?
    }, v1)?;
    Ok(h2825)
}

pub fn f382(v0: Bytes, v1: u64) -> R<u64> {
    let (h2829, _) = f124(Fn4::F419(v0.clone()), {
        let a2831 = {
            let a2830 = v0;
            length_bytes(a2830)
        };
        let a2832 = 1u64;
        nat_add(a2831, a2832)?
    }, v1)?;
    Ok(h2829)
}

pub fn f383(v0: Fn8, p2837: u64, p2838: (u64, bool)) -> R<((u64, bool), bool)> {
    let m2836 = (p2837, p2838);
    {
        let mut m2836 = m2836;
        loop {
            let step = {
                let (v1, v2) = m2836;
                let m2833 = v1;
                if m2833 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2833 - 1;
                    let m2835 = {
                        let c2834: Fn8 = v0.clone();
                        c2834.apply(v2)?
                    };
                    match m2835 {
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
                Ok(next) => { m2836 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f420(v1, v0, v2)
}

pub fn f385(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h2839, _) = v2;
        h2839
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    if f154(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f153(v4) {
            Ok(Some(({
                let a2840 = v3;
                let a2841 = 1u64;
                nat_add(a2840, a2841)?
            }, {
                let (_, h2842) = v2;
                h2842
            })))
        } else {
            let a2843 = v4;
            let a2844 = 58u64;
            if if nat_eq(a2843, a2844) {
                f165(f225(f100(v0.clone(), {
                    let a2845 = v3;
                    let a2846 = 1u64;
                    nat_add(a2845, a2846)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a2847 = v4;
                let a2848 = 35u64;
                if if nat_eq(a2847, a2848) {
                    f153(f100(v0, {
                        let a2849 = v3;
                        let a2850 = 1u64;
                        nat_sub(a2849, a2850)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f301(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a2851 = v3;
                            let a2852 = 1u64;
                            nat_add(a2851, a2852)?
                        }, {
                            let a2853 = v3;
                            let a2854 = 1u64;
                            nat_add(a2853, a2854)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f386(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f421(v0, v1)
}

pub fn f387(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h2855, _) = f305(Fn7::F422(v1, v0.clone()), {
        let a2857 = {
            let a2856 = v0;
            length_bytes(a2856)
        };
        let a2858 = 1u64;
        nat_add(a2857, a2858)?
    }, (v2, 0u64))?;
    Ok(h2855)
}

pub fn f388(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2859 = 0u64;
        let a2860 = v1;
        if nat_lt(a2859, a2860) {
            let a2861 = v1;
            let a2862 = 9223372036854775808u64;
            nat_le(a2861, a2862)
        } else {
            false
        }
    } else {
        let a2863 = v1;
        let a2864 = 9223372036854775807u64;
        nat_le(a2863, a2864)
    }
}

pub fn f389(v0: Bytes, v1: u64) -> R<u64> {
    let (h2865, _) = f124(Fn4::F423(v0.clone()), {
        let a2867 = {
            let a2866 = v0;
            length_bytes(a2866)
        };
        let a2868 = 1u64;
        nat_add(a2867, a2868)?
    }, v1)?;
    Ok(h2865)
}

pub fn f390(v0: Bytes) -> R<u64> {
    let (h2869, _) = f124(Fn4::F424(v0.clone()), {
        let a2871 = {
            let a2870 = v0;
            length_bytes(a2870)
        };
        let a2872 = 1u64;
        nat_add(a2871, a2872)?
    }, 0u64)?;
    Ok(h2869)
}

pub fn f391(v0: Bytes) -> R<Option<u64>> {
    if f425(v0.clone())? {
        Ok(Some({
            let (_, h2873) = {
                let (h2874, _) = f305(Fn7::F426(v0.clone()), {
                    let a2875 = v0;
                    length_bytes(a2875)
                }, (0u64, 0u64))?;
                h2874
            };
            h2873
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f392(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2877 = {
            let a2876 = v1.clone();
            length_bytes(a2876)
        };
        let a2878 = f427(v1)?;
        nat_sub(a2877, a2878)
    });
    let v5: Bytes = {
        let a2879 = v0;
        let a2880 = v4.clone();
        append_bytes(a2879, a2880)
    };
    let v6: u64 = f390(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2882 = {
            let a2881 = v5;
            length_bytes(a2881)
        };
        let a2883 = v6;
        nat_sub(a2882, a2883)
    });
    let v8: u64 = {
        let a2884 = v4;
        length_bytes(a2884)
    };
    let v9: u64 = {
        let a2885 = v7.clone();
        length_bytes(a2885)
    };
    let a2886 = v9;
    let a2887 = 0u64;
    if nat_eq(a2886, a2887) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f293(v2, {
            let a2888 = v3;
            let a2889 = v8;
            nat_lt(a2888, a2889)
        }) {
            let v10: u64 = if v2 {
                let a2890 = v3;
                let a2891 = v8;
                nat_add(a2890, a2891)?
            } else {
                let a2892 = v8;
                let a2893 = v3;
                nat_sub(a2892, a2893)
            };
            let v11: u64 = f427(v7.clone())?;
            let a2894 = v10;
            let a2895 = v11;
            if nat_le(a2894, a2895) {
                Ok(Some((f32(v7, 0u64, {
                    let a2896 = v9;
                    let a2897 = v10;
                    nat_sub(a2896, a2897)
                }), 0u64)))
            } else {
                let a2900 = {
                    let a2898 = v10;
                    let a2899 = v11;
                    nat_sub(a2898, a2899)
                };
                let a2901 = 19u64;
                if nat_le(a2900, a2901) {
                    Ok(Some((f32(v7, 0u64, {
                        let a2902 = v9;
                        let a2903 = v11;
                        nat_sub(a2902, a2903)
                    }), {
                        let a2904 = v10;
                        let a2905 = v11;
                        nat_sub(a2904, a2905)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a2906 = v3;
                let a2907 = v8;
                nat_sub(a2906, a2907)
            };
            let a2910 = {
                let a2908 = v9;
                let a2909 = v12;
                nat_add(a2908, a2909)?
            };
            let a2911 = 19u64;
            if nat_le(a2910, a2911) {
                Ok(Some(({
                    let a2912 = v7;
                    let a2913 = f236(v12);
                    append_bytes(a2912, a2913)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f393(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f154(f100(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2914 = v1;
            let a2915 = 1u64;
            nat_add(a2914, a2915)?
        }))
    }
}

pub fn f394(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2919 = {
        let a2917 = v0.clone();
        let a2918 = {
            let (h2916, _) = v1.clone();
            h2916
        };
        index_bytes(a2917, a2918)
    };
    match m2919 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2921 = {
                    let (h2920, _) = v1.clone();
                    h2920
                };
                let a2922 = 1u64;
                nat_add(a2921, a2922)?
            }, {
                let a2925 = {
                    let (_, h2923) = v1;
                    h2923
                };
                let a2926 = f428(v0, {
                    let (h2924, _) = v1;
                    h2924
                }, v2)?;
                append_bytes(a2925, a2926)
            })))
        }
    }
}

pub fn f395(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f429(Fn11::F430, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f396(v0: Fn9, p2930: List<List<(Bytes, Bytes)>>, p2931: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2929 = (p2930, p2931);
    {
        let mut m2929 = m2929;
        loop {
            let step = {
                let (v1, v2) = m2929;
                let m2927 = v2;
                match m2927.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2928: Fn9 = v0.clone();
                            c2928.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f398(v0: Fn10, p2936: u64, p2937: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2935 = (p2936, p2937);
    {
        let mut m2935 = m2935;
        loop {
            let step = {
                let (v1, v2) = m2935;
                let m2932 = v1;
                if m2932 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2932 - 1;
                    let m2934 = {
                        let c2933: Fn10 = v0.clone();
                        c2933.apply(v2.clone())?
                    };
                    match m2934 {
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
                Ok(next) => { m2935 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f431(v0)
}

pub fn f400(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f432(v1, f85(v0, v2)?) {
        Ok(Some({
            let a2938 = v2;
            let a2939 = 1u64;
            nat_add(a2938, a2939)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f401(p2942: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, p2943: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2941 = (p2942, p2943);
    {
        let mut m2941 = m2941;
        loop {
            let step = {
                let (v0, v1) = m2941;
                let m2940 = v0;
                match m2940.uncons() {
                    None => {
                        Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: Bytes, p2948: List<(Bytes, (bool, Bytes))>) -> Option<(bool, Bytes)> {
    let m2947 = p2948;
    {
        let mut m2947 = m2947;
        loop {
            let step = {
                let v1 = m2947;
                let m2944 = v1;
                match m2944.uncons() {
                    None => {
                        Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(None::<(bool, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        if f35(v0.clone(), {
                            let (h2945, _) = v2;
                            h2945
                        }) {
                            Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(Some({
                                let (_, h2946) = v2;
                                h2946
                            }))
                        } else {
                            Ok::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2947 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403() -> List<(Bytes, (bool, Bytes))> {
    List::cons((f8(Str::lit("boolean")), (true, f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), (true, f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), (true, f368())), List::cons((f8(Str::lit("double")), (true, f368())), List::cons((f8(Str::lit("DOMString")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), (false, f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), (false, f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), (false, f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), (false, f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), (false, f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), (false, f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), (false, f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, (bool, Bytes))>::nil()))))))))))))))))))))))))))))
}

pub fn f404() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f405(p2959: u64, v1: Bytes, v2: Bytes, p2960: u64) -> R<bool> {
    let m2958 = (p2959, p2960);
    {
        let mut m2958 = m2958;
        loop {
            let step = {
                let (v0, v3) = m2958;
                let m2949 = v0;
                if m2949 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m2949 - 1;
                    let v5: u64 = f433(v1.clone(), v3)?;
                    let v6: u64 = f433(v2.clone(), v3)?;
                    let a2950 = v5;
                    let a2951 = v6;
                    if nat_lt(a2950, a2951) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a2952 = v6;
                        let a2953 = v5;
                        if nat_lt(a2952, a2953) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a2954 = v5;
                            let a2955 = 0u64;
                            if nat_eq(a2954, a2955) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a2956 = v3;
                                    let a2957 = 1u64;
                                    nat_add(a2956, a2957)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2958 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(p2973: u64, v1: Bytes, p2974: u64, p2975: Bytes) -> R<Option<Bytes>> {
    let m2972 = (p2973, (p2974, p2975));
    {
        let mut m2972 = m2972;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2972;
                let m2961 = v0;
                if m2961 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2961 - 1;
                    let a2963 = {
                        let a2962 = v1.clone();
                        length_bytes(a2962)
                    };
                    let a2964 = v2;
                    if nat_le(a2963, a2964) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2965 = f277(v1.clone(), v2)?;
                        match m2965 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2967 = v2;
                                    let a2968 = {
                                        let (_, h2966) = v5;
                                        h2966
                                    };
                                    nat_add(a2967, a2968)?
                                }, {
                                    let a2970 = v3;
                                    let a2971 = f434(v1.clone(), v2, {
                                        let (h2969, _) = v5;
                                        h2969
                                    });
                                    append_bytes(a2970, a2971)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2972 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: u64) -> Bytes {
    let a2976 = f8(Str::lit("u"));
    let a2977 = f435(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a2976, a2977)
}

pub fn f408() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f409() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f410(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f85(v0, {
        let (h2978, _) = v2;
        h2978
    })?;
    let a2980 = 1114111u64;
    let a2981 = {
        let (_, h2979) = v2;
        h2979
    };
    if nat_lt(a2980, a2981) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f436(v3, v1);
        let a2982 = v4;
        let a2983 = 16u64;
        if nat_eq(a2982, a2983) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a2985 = {
                    let (h2984, _) = v2;
                    h2984
                };
                let a2986 = 1u64;
                nat_add(a2985, a2986)?
            }, {
                let a2990 = {
                    let a2988 = {
                        let (_, h2987) = v2;
                        h2987
                    };
                    let a2989 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a2988, a2989)?
                };
                let a2991 = v4;
                nat_add(a2990, a2991)?
            })))
        }
    }
}

pub fn f411() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f412(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a2992 = v0;
        let a2993 = v2;
        if nat_eq(a2992, a2993) {
            Ok(true)
        } else {
            f204(v0, f437())
        }
    } else {
        f204(v0, f438())
    }
}

pub fn f413() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f414(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f439(v1, v0, v2)
}

pub fn f415(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f440(v1, v2, v0, v3)
}

pub fn f416(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a2996 = {
        let (h2994, _) = v1;
        h2994
    };
    let a2997 = {
        let a2995 = v0.clone();
        length_bytes(a2995)
    };
    if nat_lt(a2996, a2997) {
        Ok(Some((f163(v0, {
            let (h2998, _) = v1;
            h2998
        })?, {
            let a3000 = {
                let (_, h2999) = v1;
                h2999
            };
            let a3001 = 1u64;
            nat_add(a3000, a3001)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f417(v0: u64) -> Option<u64> {
    let a3002 = 48u64;
    let a3003 = v0;
    if if nat_le(a3002, a3003) {
        let a3004 = v0;
        let a3005 = 57u64;
        nat_le(a3004, a3005)
    } else {
        false
    } {
        Some({
            let a3006 = v0;
            let a3007 = 48u64;
            nat_sub(a3006, a3007)
        })
    } else {
        let a3008 = 97u64;
        let a3009 = v0;
        if if nat_le(a3008, a3009) {
            let a3010 = v0;
            let a3011 = 102u64;
            nat_le(a3010, a3011)
        } else {
            false
        } {
            Some({
                let a3012 = v0;
                let a3013 = 87u64;
                nat_sub(a3012, a3013)
            })
        } else {
            let a3014 = 65u64;
            let a3015 = v0;
            if if nat_le(a3014, a3015) {
                let a3016 = v0;
                let a3017 = 70u64;
                nat_le(a3016, a3017)
            } else {
                false
            } {
                Some({
                    let a3018 = v0;
                    let a3019 = 55u64;
                    nat_sub(a3018, a3019)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f418(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f441(v0, v1)
}

pub fn f419(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f442(v0, v1)
}

pub fn f420(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3020, _) = v2;
        h3020
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    let (_, h3021) = v2;
    if h3021 {
        Ok(None::<(u64, bool)>)
    } else {
        if f153(v4) {
            Ok(Some(({
                let a3022 = v3;
                let a3023 = 1u64;
                nat_add(a3022, a3023)?
            }, false)))
        } else {
            let a3024 = v4;
            let a3025 = 35u64;
            if if nat_eq(a3024, a3025) {
                let a3026 = v3;
                let a3027 = 0u64;
                if if nat_eq(a3026, a3027) {
                    true
                } else {
                    f153(f100(v0.clone(), {
                        let a3028 = v3;
                        let a3029 = 1u64;
                        nat_sub(a3028, a3029)
                    })?)
                } {
                    true
                } else {
                    let a3032 = f100(v0.clone(), {
                        let a3030 = v3;
                        let a3031 = 1u64;
                        nat_sub(a3030, a3031)
                    })?;
                    let a3033 = 10u64;
                    nat_eq(a3032, a3033)
                }
            } else {
                false
            } {
                Ok(Some((f234(v0, v3)?, false)))
            } else {
                let a3034 = v4;
                let a3035 = 10u64;
                if nat_eq(a3034, a3035) {
                    let v5: u64 = {
                        let a3036 = v3;
                        let a3037 = 1u64;
                        nat_add(a3036, a3037)?
                    };
                    let v6: u64 = f156(v0.clone(), v5)?;
                    let v7: u64 = f157(v0.clone(), v6)?;
                    if if f99(v0.clone(), v5)? {
                        true
                    } else {
                        let a3040 = {
                            let a3038 = v6;
                            let a3039 = v5;
                            nat_sub(a3038, a3039)
                        };
                        let a3041 = v1;
                        if if nat_lt(a3040, a3041) {
                            f165(f154(f100(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f165({
                                let a3042 = f100(v0, v7)?;
                                let a3043 = 35u64;
                                nat_eq(a3042, a3043)
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

pub fn f421(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f157(v0.clone(), {
        let (h3044, _) = v1;
        h3044
    })?;
    let a3045 = f100(v0, v2)?;
    let a3046 = 10u64;
    if nat_eq(a3045, a3046) {
        Ok(Some(({
            let a3047 = v2;
            let a3048 = 1u64;
            nat_add(a3047, a3048)?
        }, {
            let a3050 = {
                let (_, h3049) = v1;
                h3049
            };
            let a3051 = 1u64;
            nat_add(a3050, a3051)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f422(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f443(v1, v0, v2)
}

pub fn f423(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f444(v0, v1)
}

pub fn f424(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f445(v0, v1)
}

pub fn f425(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3052 = v0.clone();
        length_bytes(a3052)
    };
    let a3053 = 0u64;
    let a3054 = v1;
    if nat_lt(a3053, a3054) {
        let a3055 = f446(v0.clone())?;
        let a3056 = v1;
        if nat_eq(a3055, a3056) {
            let a3057 = 1u64;
            let a3058 = v1;
            if nat_lt(a3057, a3058) {
                let m3061 = {
                    let a3059 = v0;
                    let a3060 = 0u64;
                    index_bytes(a3059, a3060)
                };
                match m3061 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3062 = v2;
                        let a3063 = 48u8;
                        Ok(!equal(a3062, a3063))
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

pub fn f426(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f447(v0, v1)
}

pub fn f427(v0: Bytes) -> R<u64> {
    let (h3064, _) = f124(Fn4::F448(v0.clone()), {
        let a3066 = {
            let a3065 = v0;
            length_bytes(a3065)
        };
        let a3067 = 1u64;
        nat_add(a3066, a3067)?
    }, 0u64)?;
    Ok(h3064)
}

pub fn f428(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3112 = {
            let a3088 = {
                let a3076 = {
                    let a3070 = {
                        let a3068 = v3;
                        let a3069 = 128u8;
                        fixed_u8::bit_and(a3068, a3069)
                    };
                    let a3071 = 128u8;
                    if equal(a3070, a3071) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3077 = {
                    let a3074 = {
                        let a3072 = v3;
                        let a3073 = 64u8;
                        fixed_u8::bit_and(a3072, a3073)
                    };
                    let a3075 = 64u8;
                    if equal(a3074, a3075) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3076, a3077)?
            };
            let a3089 = {
                let a3086 = {
                    let a3080 = {
                        let a3078 = v3;
                        let a3079 = 32u8;
                        fixed_u8::bit_and(a3078, a3079)
                    };
                    let a3081 = 32u8;
                    if equal(a3080, a3081) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3087 = {
                    let a3084 = {
                        let a3082 = v3;
                        let a3083 = 16u8;
                        fixed_u8::bit_and(a3082, a3083)
                    };
                    let a3085 = 16u8;
                    if equal(a3084, a3085) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3086, a3087)?
            };
            nat_add(a3088, a3089)?
        };
        let a3113 = {
            let a3110 = {
                let a3098 = {
                    let a3092 = {
                        let a3090 = v3;
                        let a3091 = 8u8;
                        fixed_u8::bit_and(a3090, a3091)
                    };
                    let a3093 = 8u8;
                    if equal(a3092, a3093) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3099 = {
                    let a3096 = {
                        let a3094 = v3;
                        let a3095 = 4u8;
                        fixed_u8::bit_and(a3094, a3095)
                    };
                    let a3097 = 4u8;
                    if equal(a3096, a3097) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3098, a3099)?
            };
            let a3111 = {
                let a3108 = {
                    let a3102 = {
                        let a3100 = v3;
                        let a3101 = 2u8;
                        fixed_u8::bit_and(a3100, a3101)
                    };
                    let a3103 = 2u8;
                    if equal(a3102, a3103) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3109 = {
                    let a3106 = {
                        let a3104 = v3;
                        let a3105 = 1u8;
                        fixed_u8::bit_and(a3104, a3105)
                    };
                    let a3107 = 1u8;
                    if equal(a3106, a3107) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3108, a3109)?
            };
            nat_add(a3110, a3111)?
        };
        nat_add(a3112, a3113)?
    };
    let a3114 = v4;
    let a3115 = 34u64;
    if nat_eq(a3114, a3115) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3116 = v4;
        let a3117 = 92u64;
        if nat_eq(a3116, a3117) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3118 = v4;
            let a3119 = 8u64;
            if nat_eq(a3118, a3119) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3120 = v4;
                let a3121 = 12u64;
                if nat_eq(a3120, a3121) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3122 = v4;
                    let a3123 = 10u64;
                    if nat_eq(a3122, a3123) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3124 = v4;
                        let a3125 = 13u64;
                        if nat_eq(a3124, a3125) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3126 = v4;
                            let a3127 = 9u64;
                            if nat_eq(a3126, a3127) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3128 = v4;
                                let a3129 = 32u64;
                                if nat_lt(a3128, a3129) {
                                    let a3130 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3131 = f449(v2)?;
                                    Ok(append_bytes(a3130, a3131))
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

pub fn f429(v0: Fn11, p3135: List<List<(Bytes, Bytes)>>, p3136: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3134 = (p3135, p3136);
    {
        let mut m3134 = m3134;
        loop {
            let step = {
                let (v1, v2) = m3134;
                let m3132 = v2;
                match m3132.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3133: Fn11 = v0.clone();
                            c3133.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f430(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f431(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3138 = {
        let a3137 = v0.clone();
        length_list(a3137)
    };
    let a3139 = 1u64;
    if nat_le(a3138, a3139) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f450(v0)?))
    }
}

pub fn f432(v0: u64, v1: u64) -> bool {
    let a3140 = v0;
    let a3141 = 0u64;
    if nat_eq(a3140, a3141) {
        f331(v1)
    } else {
        let a3142 = v0;
        let a3143 = 1u64;
        if nat_eq(a3142, a3143) {
            f330(v1)
        } else {
            let a3144 = v0;
            let a3145 = 2u64;
            if nat_eq(a3144, a3145) {
                f451(v1)
            } else {
                f452(v1)
            }
        }
    }
}

pub fn f433(v0: Bytes, v1: u64) -> R<u64> {
    let a3147 = v1;
    let a3148 = {
        let a3146 = v0.clone();
        length_bytes(a3146)
    };
    if nat_lt(a3147, a3148) {
        let a3149 = f85(v0, v1)?;
        let a3150 = 1u64;
        nat_add(a3149, a3150)
    } else {
        Ok(0u64)
    }
}

pub fn f434(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f293(f291(f293(f292(v2), f294(v2)), f291({
        let a3151 = v2;
        let a3152 = 117u64;
        !nat_eq(a3151, a3152)
    }, {
        let a3153 = v2;
        let a3154 = 85u64;
        !nat_eq(a3153, a3154)
    })), f291(f295(v2), {
        let a3155 = 0u64;
        let a3156 = v1;
        nat_lt(a3155, a3156)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a3157 = v2;
        let a3158 = 65536u64;
        if nat_lt(a3157, a3158) {
            let a3159 = f8(Str::lit("u"));
            let a3160 = f435(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3159, a3160)
        } else {
            let a3161 = f8(Str::lit("U"));
            let a3162 = f435(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3161, a3162)
        }
    }
}

pub fn f435(p3179: u64, p3180: u64, p3181: u64, p3182: Bytes) -> Bytes {
    let m3178 = (p3179, (p3180, (p3181, p3182)));
    {
        let mut m3178 = m3178;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3178;
                let m3163 = v0;
                if m3163 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3163 - 1;
                    if f291({
                        let a3164 = v1;
                        let a3165 = 0u64;
                        nat_eq(a3164, a3165)
                    }, {
                        let a3166 = v2;
                        let a3167 = 0u64;
                        nat_eq(a3166, a3167)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3168 = v1;
                            let a3169 = 16u64;
                            let a3170 = 0u64;
                            nat_quot(a3168, a3169, a3170)
                        }, ({
                            let a3171 = v2;
                            let a3172 = 1u64;
                            nat_sub(a3171, a3172)
                        }, {
                            let a3176 = f453({
                                let a3173 = v1;
                                let a3174 = 16u64;
                                let a3175 = 0u64;
                                nat_rem(a3173, a3174, a3175)
                            });
                            let a3177 = v3;
                            append_bytes(a3176, a3177)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: u64, v1: bool) -> u64 {
    let a3183 = v0;
    let a3184 = 48u64;
    if nat_lt(a3183, a3184) {
        16u64
    } else {
        let a3185 = v0;
        let a3186 = 57u64;
        if nat_le(a3185, a3186) {
            let a3187 = v0;
            let a3188 = 48u64;
            nat_sub(a3187, a3188)
        } else {
            if v1 {
                let a3189 = v0;
                let a3190 = 65u64;
                if nat_lt(a3189, a3190) {
                    16u64
                } else {
                    let a3191 = v0;
                    let a3192 = 70u64;
                    if nat_le(a3191, a3192) {
                        let a3193 = v0;
                        let a3194 = 55u64;
                        nat_sub(a3193, a3194)
                    } else {
                        let a3195 = v0;
                        let a3196 = 97u64;
                        if nat_lt(a3195, a3196) {
                            16u64
                        } else {
                            let a3197 = v0;
                            let a3198 = 102u64;
                            if nat_le(a3197, a3198) {
                                let a3199 = v0;
                                let a3200 = 87u64;
                                nat_sub(a3199, a3200)
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

pub fn f437() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f438() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f439(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f372(v0.clone(), {
        let (h3201, _) = v2;
        h3201
    }, v1)?;
    let a3202 = f100(v0, v3)?;
    let a3203 = 10u64;
    if nat_eq(a3202, a3203) {
        Ok(Some(({
            let a3204 = v3;
            let a3205 = 1u64;
            nat_add(a3204, a3205)?
        }, {
            let a3207 = {
                let (_, h3206) = v2;
                h3206
            };
            let a3208 = 1u64;
            nat_add(a3207, a3208)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f440(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3209 = f100(v0, v3)?;
    let a3210 = 32u64;
    if if nat_eq(a3209, a3210) {
        let a3213 = {
            let a3211 = v3;
            let a3212 = v1;
            nat_sub(a3211, a3212)
        };
        let a3214 = v2;
        nat_lt(a3213, a3214)
    } else {
        false
    } {
        Ok(Some({
            let a3215 = v3;
            let a3216 = 1u64;
            nat_add(a3215, a3216)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f441(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3217 = v2;
    let a3218 = 34u64;
    if if if if nat_eq(a3217, a3218) {
        true
    } else {
        let a3219 = v2;
        let a3220 = 92u64;
        nat_eq(a3219, a3220)
    } {
        true
    } else {
        f153(v2)
    } {
        true
    } else {
        f154(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3221 = v1;
            let a3222 = 1u64;
            nat_add(a3221, a3222)?
        }))
    }
}

pub fn f442(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3223 = v2;
    let a3224 = 39u64;
    if if if nat_eq(a3223, a3224) {
        true
    } else {
        f153(v2)
    } {
        true
    } else {
        f154(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3225 = v1;
            let a3226 = 1u64;
            nat_add(a3225, a3226)?
        }))
    }
}

pub fn f443(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3228 = f417(f100(v0, {
        let (h3227, _) = v2;
        h3227
    })?);
    match m3228 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3229 = v3;
            let a3230 = v1;
            if nat_lt(a3229, a3230) {
                Ok(Some(({
                    let a3232 = {
                        let (h3231, _) = v2;
                        h3231
                    };
                    let a3233 = 1u64;
                    nat_add(a3232, a3233)?
                }, {
                    let a3237 = {
                        let a3235 = {
                            let (_, h3234) = v2;
                            h3234
                        };
                        let a3236 = v1;
                        nat_mul(a3235, a3236)?
                    };
                    let a3238 = v3;
                    nat_add(a3237, a3238)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f444(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f133(v0, v1)?;
    let a3239 = 47u64;
    let a3240 = v2;
    if if nat_lt(a3239, a3240) {
        let a3241 = v2;
        let a3242 = 58u64;
        nat_lt(a3241, a3242)
    } else {
        false
    } {
        Ok(Some({
            let a3243 = v1;
            let a3244 = 1u64;
            nat_add(a3243, a3244)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f445(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3245 = f133(v0, v1)?;
    let a3246 = 48u64;
    if nat_eq(a3245, a3246) {
        Ok(Some({
            let a3247 = v1;
            let a3248 = 1u64;
            nat_add(a3247, a3248)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f446(v0: Bytes) -> R<u64> {
    let (h3249, _) = f124(Fn4::F454(v0.clone()), {
        let a3250 = v0;
        length_bytes(a3250)
    }, 0u64)?;
    Ok(h3249)
}

pub fn f447(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3254 = {
        let a3252 = v0;
        let a3253 = {
            let (h3251, _) = v1;
            h3251
        };
        index_bytes(a3252, a3253)
    };
    match m3254 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3256 = {
                    let (h3255, _) = v1;
                    h3255
                };
                let a3257 = 1u64;
                nat_add(a3256, a3257)?
            }, {
                let a3261 = {
                    let a3259 = {
                        let (_, h3258) = v1;
                        h3258
                    };
                    let a3260 = 10u64;
                    nat_mul(a3259, a3260)?
                };
                let a3262 = f455(v2)?;
                nat_add(a3261, a3262)?
            })))
        }
    }
}

pub fn f448(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f456(v0, v1)
}

pub fn f449(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3307 = {
            let a3283 = {
                let a3271 = {
                    let a3265 = {
                        let a3263 = v1;
                        let a3264 = 128u8;
                        fixed_u8::bit_and(a3263, a3264)
                    };
                    let a3266 = 128u8;
                    if equal(a3265, a3266) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3272 = {
                    let a3269 = {
                        let a3267 = v1;
                        let a3268 = 64u8;
                        fixed_u8::bit_and(a3267, a3268)
                    };
                    let a3270 = 64u8;
                    if equal(a3269, a3270) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3271, a3272)?
            };
            let a3284 = {
                let a3281 = {
                    let a3275 = {
                        let a3273 = v1;
                        let a3274 = 32u8;
                        fixed_u8::bit_and(a3273, a3274)
                    };
                    let a3276 = 32u8;
                    if equal(a3275, a3276) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3282 = {
                    let a3279 = {
                        let a3277 = v1;
                        let a3278 = 16u8;
                        fixed_u8::bit_and(a3277, a3278)
                    };
                    let a3280 = 16u8;
                    if equal(a3279, a3280) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3281, a3282)?
            };
            nat_add(a3283, a3284)?
        };
        let a3308 = {
            let a3305 = {
                let a3293 = {
                    let a3287 = {
                        let a3285 = v1;
                        let a3286 = 8u8;
                        fixed_u8::bit_and(a3285, a3286)
                    };
                    let a3288 = 8u8;
                    if equal(a3287, a3288) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3294 = {
                    let a3291 = {
                        let a3289 = v1;
                        let a3290 = 4u8;
                        fixed_u8::bit_and(a3289, a3290)
                    };
                    let a3292 = 4u8;
                    if equal(a3291, a3292) {
                        4u64
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
                        let a3296 = 2u8;
                        fixed_u8::bit_and(a3295, a3296)
                    };
                    let a3298 = 2u8;
                    if equal(a3297, a3298) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3304 = {
                    let a3301 = {
                        let a3299 = v1;
                        let a3300 = 1u8;
                        fixed_u8::bit_and(a3299, a3300)
                    };
                    let a3302 = 1u8;
                    if equal(a3301, a3302) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3303, a3304)?
            };
            nat_add(a3305, a3306)?
        };
        nat_add(a3307, a3308)?
    };
    let a3315 = f120(f457(), {
        let a3309 = v2;
        let a3310 = 16u64;
        let a3311 = 0u64;
        nat_quot(a3309, a3310, a3311)
    });
    let a3316 = f120(f457(), {
        let a3312 = v2;
        let a3313 = 16u64;
        let a3314 = 0u64;
        nat_rem(a3312, a3313, a3314)
    });
    Ok(append_bytes(a3315, a3316))
}

pub fn f450(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f395({
        let (_, h3317) = {
            let (h3318, _) = f458(Fn12::F459, {
                let a3320 = {
                    let a3319 = v0.clone();
                    length_list(a3319)
                };
                let a3321 = 1u64;
                nat_add(a3320, a3321)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3318
        };
        h3317
    }))
}

pub fn f451(v0: u64) -> bool {
    let a3322 = 48u64;
    let a3323 = v0;
    if nat_le(a3322, a3323) {
        let a3324 = v0;
        let a3325 = 55u64;
        nat_le(a3324, a3325)
    } else {
        false
    }
}

pub fn f452(v0: u64) -> bool {
    if f246(v0) {
        true
    } else {
        if f331(v0) {
            true
        } else {
            let a3326 = v0;
            let a3327 = 95u64;
            if nat_eq(a3326, a3327) {
                true
            } else {
                let a3328 = v0;
                let a3329 = 45u64;
                nat_eq(a3328, a3329)
            }
        }
    }
}

pub fn f453(v0: u64) -> Bytes {
    f32({
        let a3330 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3330)
    }, v0, 1u64)
}

pub fn f454(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3333 = {
        let a3331 = v0;
        let a3332 = v1;
        index_bytes(a3331, a3332)
    };
    match m3333 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f460(v2)? {
                Ok(Some({
                    let a3334 = v1;
                    let a3335 = 1u64;
                    nat_add(a3334, a3335)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f455(v0: u8) -> R<u64> {
    let a3382 = {
        let v1: u8 = v0;
        let a3380 = {
            let a3356 = {
                let a3344 = {
                    let a3338 = {
                        let a3336 = v1;
                        let a3337 = 128u8;
                        fixed_u8::bit_and(a3336, a3337)
                    };
                    let a3339 = 128u8;
                    if equal(a3338, a3339) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3345 = {
                    let a3342 = {
                        let a3340 = v1;
                        let a3341 = 64u8;
                        fixed_u8::bit_and(a3340, a3341)
                    };
                    let a3343 = 64u8;
                    if equal(a3342, a3343) {
                        64u64
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
                        let a3347 = 32u8;
                        fixed_u8::bit_and(a3346, a3347)
                    };
                    let a3349 = 32u8;
                    if equal(a3348, a3349) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3355 = {
                    let a3352 = {
                        let a3350 = v1;
                        let a3351 = 16u8;
                        fixed_u8::bit_and(a3350, a3351)
                    };
                    let a3353 = 16u8;
                    if equal(a3352, a3353) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3354, a3355)?
            };
            nat_add(a3356, a3357)?
        };
        let a3381 = {
            let a3378 = {
                let a3366 = {
                    let a3360 = {
                        let a3358 = v1;
                        let a3359 = 8u8;
                        fixed_u8::bit_and(a3358, a3359)
                    };
                    let a3361 = 8u8;
                    if equal(a3360, a3361) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3367 = {
                    let a3364 = {
                        let a3362 = v1;
                        let a3363 = 4u8;
                        fixed_u8::bit_and(a3362, a3363)
                    };
                    let a3365 = 4u8;
                    if equal(a3364, a3365) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3366, a3367)?
            };
            let a3379 = {
                let a3376 = {
                    let a3370 = {
                        let a3368 = v1;
                        let a3369 = 2u8;
                        fixed_u8::bit_and(a3368, a3369)
                    };
                    let a3371 = 2u8;
                    if equal(a3370, a3371) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3377 = {
                    let a3374 = {
                        let a3372 = v1;
                        let a3373 = 1u8;
                        fixed_u8::bit_and(a3372, a3373)
                    };
                    let a3375 = 1u8;
                    if equal(a3374, a3375) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3376, a3377)?
            };
            nat_add(a3378, a3379)?
        };
        nat_add(a3380, a3381)?
    };
    let a3383 = 48u64;
    Ok(nat_sub(a3382, a3383))
}

pub fn f456(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3384 = v0.clone();
        length_bytes(a3384)
    };
    let a3385 = v1;
    let a3386 = v2;
    if if nat_lt(a3385, a3386) {
        let a3391 = f133(v0, {
            let a3389 = v2;
            let a3390 = {
                let a3387 = v1;
                let a3388 = 1u64;
                nat_add(a3387, a3388)?
            };
            nat_sub(a3389, a3390)
        })?;
        let a3392 = 48u64;
        nat_eq(a3391, a3392)
    } else {
        false
    } {
        Ok(Some({
            let a3393 = v1;
            let a3394 = 1u64;
            nat_add(a3393, a3394)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f457() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f458(v0: Fn12, p3399: u64, p3400: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3398 = (p3399, p3400);
    {
        let mut m3398 = m3398;
        loop {
            let step = {
                let (v1, v2) = m3398;
                let m3395 = v1;
                if m3395 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3395 - 1;
                    let m3397 = {
                        let c3396: Fn12 = v0.clone();
                        c3396.apply(v2.clone())?
                    };
                    match m3397 {
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
                Ok(next) => { m3398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f461(v0)
}

pub fn f460(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3445 = {
            let a3421 = {
                let a3409 = {
                    let a3403 = {
                        let a3401 = v1;
                        let a3402 = 128u8;
                        fixed_u8::bit_and(a3401, a3402)
                    };
                    let a3404 = 128u8;
                    if equal(a3403, a3404) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3410 = {
                    let a3407 = {
                        let a3405 = v1;
                        let a3406 = 64u8;
                        fixed_u8::bit_and(a3405, a3406)
                    };
                    let a3408 = 64u8;
                    if equal(a3407, a3408) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3409, a3410)?
            };
            let a3422 = {
                let a3419 = {
                    let a3413 = {
                        let a3411 = v1;
                        let a3412 = 32u8;
                        fixed_u8::bit_and(a3411, a3412)
                    };
                    let a3414 = 32u8;
                    if equal(a3413, a3414) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3420 = {
                    let a3417 = {
                        let a3415 = v1;
                        let a3416 = 16u8;
                        fixed_u8::bit_and(a3415, a3416)
                    };
                    let a3418 = 16u8;
                    if equal(a3417, a3418) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3419, a3420)?
            };
            nat_add(a3421, a3422)?
        };
        let a3446 = {
            let a3443 = {
                let a3431 = {
                    let a3425 = {
                        let a3423 = v1;
                        let a3424 = 8u8;
                        fixed_u8::bit_and(a3423, a3424)
                    };
                    let a3426 = 8u8;
                    if equal(a3425, a3426) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3432 = {
                    let a3429 = {
                        let a3427 = v1;
                        let a3428 = 4u8;
                        fixed_u8::bit_and(a3427, a3428)
                    };
                    let a3430 = 4u8;
                    if equal(a3429, a3430) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3431, a3432)?
            };
            let a3444 = {
                let a3441 = {
                    let a3435 = {
                        let a3433 = v1;
                        let a3434 = 2u8;
                        fixed_u8::bit_and(a3433, a3434)
                    };
                    let a3436 = 2u8;
                    if equal(a3435, a3436) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3442 = {
                    let a3439 = {
                        let a3437 = v1;
                        let a3438 = 1u8;
                        fixed_u8::bit_and(a3437, a3438)
                    };
                    let a3440 = 1u8;
                    if equal(a3439, a3440) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3441, a3442)?
            };
            nat_add(a3443, a3444)?
        };
        nat_add(a3445, a3446)?
    };
    let a3447 = 48u64;
    let a3448 = v2;
    if nat_le(a3447, a3448) {
        let a3449 = v2;
        let a3450 = 57u64;
        Ok(nat_le(a3449, a3450))
    } else {
        Ok(false)
    }
}

pub fn f461(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3451, _) = v0;
        h3451
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3452) = v0;
        h3452
    };
    let m3453 = v1;
    match m3453.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3454 = v4;
            match m3454.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f462(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f462(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3459 = {
            let a3457 = {
                let a3455 = v0.clone();
                length_list(a3455)
            };
            let a3458 = {
                let a3456 = v1.clone();
                length_list(a3456)
            };
            nat_add(a3457, a3458)?
        };
        let a3460 = 1u64;
        nat_add(a3459, a3460)?
    };
    Ok(f463({
        let (_, h3461) = {
            let (_, h3462) = {
                let (h3463, _) = f464(Fn13::F465, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3463
            };
            h3462
        };
        h3461
    }))
}

pub fn f463(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f239(Fn5::F466, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f464(v0: Fn13, p3468: u64, p3469: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3467 = (p3468, p3469);
    {
        let mut m3467 = m3467;
        loop {
            let step = {
                let (v1, v2) = m3467;
                let m3464 = v1;
                if m3464 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3464 - 1;
                    let m3466 = {
                        let c3465: Fn13 = v0.clone();
                        c3465.apply(v2.clone())
                    };
                    match m3466 {
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
                Ok(next) => { m3467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f467(v0)
}

pub fn f466(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f467(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3470, _) = v0.clone();
        h3470
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3471, _) = {
            let (_, h3472) = v0.clone();
            h3472
        };
        h3471
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3473) = {
            let (_, h3474) = v0;
            h3474
        };
        h3473
    };
    let m3475 = v1.clone();
    match m3475.uncons() {
        None => {
            let m3476 = v2;
            match m3476.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3477 = v2.clone();
            match m3477.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f468({
                        let (h3478, _) = v8.clone();
                        h3478
                    }, {
                        let (h3479, _) = v6.clone();
                        h3479
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

pub fn f468(v0: Bytes, v1: Bytes) -> bool {
    let a3484 = {
        let a3480 = v0;
        let a3481 = v1;
        compare_bytes(a3480, a3481)
    };
    let a3485 = {
        let a3482 = Bytes::lit(&[]);
        let a3483 = Bytes::lit(&[0x01]);
        compare_bytes(a3482, a3483)
    };
    equal(a3484, a3485)
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
