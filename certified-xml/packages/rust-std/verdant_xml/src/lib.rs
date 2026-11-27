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
    C0(Bytes, std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C1(Bytes, Bytes, std::rc::Rc<Adt13>),
    C2(Bytes),
    C3(std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C4,
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Bytes>),
    C1(Bytes),
    C2(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>, Adt13, u64),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Adt19>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Bytes, Bytes, Bytes, Adt20),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, Bytes, Adt13, Adt13, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt22, u64),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, Adt13, Adt13, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Adt17, List<Adt21>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<Adt27>, List<Adt21>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Bytes, Adt25),
}

#[derive(Clone)]
pub enum Fn0 {
    F64(u8, Bytes),
    F244(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn0::F244(k0) => f244(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F67(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F67(k0) => f67(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F110,
    F246,
    F319,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F110 => f110(p0, p1),
            Fn2::F246 => f246(p0, p1),
            Fn2::F319 => f319(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F165(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt16)) -> R<(u64, Adt16)> {
        match self {
            Fn3::F165(k0, k1, k2) => f165(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F177(Bytes),
    F192(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F177(k0) => f177(k0.clone(), p0),
            Fn4::F192(k0, k1) => Ok(f192(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F179(Bytes),
    F219(Bytes),
    F220(Bytes),
    F248(Bytes, u64, Bytes, u64),
    F253(Bytes, Bytes),
    F254(Bytes),
    F256(Bytes, u64),
    F297(Bytes),
    F332(bool, u64, Bytes),
    F359(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F179(k0) => f179(k0.clone(), p0),
            Fn5::F219(k0) => f219(k0.clone(), p0),
            Fn5::F220(k0) => f220(k0.clone(), p0),
            Fn5::F248(k0, k1, k2, k3) => f248(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn5::F253(k0, k1) => f253(k0.clone(), k1.clone(), p0),
            Fn5::F254(k0) => f254(k0.clone(), p0),
            Fn5::F256(k0, k1) => f256(k0.clone(), *k1, p0),
            Fn5::F297(k0) => f297(k0.clone(), p0),
            Fn5::F332(k0, k1, k2) => f332(*k0, *k1, k2.clone(), p0),
            Fn5::F359(k0) => f359(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F223(Bytes),
    F395(bool, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn6::F223(k0) => f223(k0.clone(), p0),
            Fn6::F395(k0, k1) => f395(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F299,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn7::F299 => f299(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F313,
    F537,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F313 => f313(p0, p1),
            Fn8::F537 => f537(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F316,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn9::F316 => f316(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F380,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn10::F380 => f380(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F383,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn11::F383 => f383(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F385,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn12::F385 => f385(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F388,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn13::F388 => f388(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F425(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn14::F425(k0) => f425(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F434,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F434 => f434(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F443,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F443 => f443(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F445,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F445 => f445(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F447,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn18::F447 => f447(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F450,
}

impl Fn19 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn19::F450 => f450(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F489,
}

impl Fn20 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn20::F489 => f489(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F513,
}

impl Fn21 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn21::F513 => f513(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F523,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F523 => f523(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F532,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F532 => f532(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F535,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F535 => f535(p0),
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
                    Ok(Adt12::C4(f8(Str::lit("name the verdant_xml command to run before the citation: read-admit NAME read-OP FILE ARG... CITATION...\u{a}"))))
                }
                Some((v5, v6)) => {
                    let m187 = v6.clone();
                    match m187.uncons() {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("name the verdant_xml command to run before the citation: read-admit NAME read-OP FILE ARG... CITATION...\u{a}"))))
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
                                Ok(Adt12::C4(f8(Str::lit("name the verdant_xml command to run before the citation: read-admit NAME read-OP FILE ARG... CITATION...\u{a}"))))
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

pub fn f35(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("read-xml"))) {
        f56(v3, v1)
    } else {
        if f32(v0.clone(), f8(Str::lit("read-xsd"))) {
            f57(v3, v1)
        } else {
            if f32(v0, f8(Str::lit("read-xsd-components"))) {
                f58(v3, v1)
            } else {
                Ok(Adt12::C4(f8(Str::lit("check-yaml, read-webidl and read-yaml run in verdant_readers, the command of the readers project\u{a}"))))
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
    let v1: u64 = f59(v0)?;
    let m199 = {
        let a197 = f60({
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
    let v1: Bytes = f61(v0)?;
    let m229 = f62(v1.clone(), 0u64, {
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
        let (h236, _) = f63(Fn0::F64(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h236
    };
    let v4: u64 = {
        let (h237, _) = {
            let (_, h238) = v3.clone();
            h238
        };
        h237
    };
    Ok(f65(List::cons(f46(v0, v4, {
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
    let m251 = f66(Fn1::F67(v1), None::<Bytes>, v0);
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
            let m253 = f68(f69(v0.clone(), v2.clone()), v2.clone());
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
                        let m255 = f70(v2.clone(), f8(Str::lit("translation")));
                        match m255 {
                            None => {
                                Ok::<List<Adt2>, R<Adt12>>(v3)
                            }
                            Some(v4) => {
                                let v6: Bytes = {
                                    let m256 = f71(f70(v2, f8(Str::lit("construct"))));
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
                                    let m257 = f71(f70(v4.clone(), f8(Str::lit("output_sha256"))));
                                    match m257 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v7) => {
                                            v7
                                        }
                                    }
                                };
                                let m258 = f72(f73(f70(v4, f8(Str::lit("command")))));
                                match m258.uncons() {
                                    None => {
                                        Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(" records no reader command\u{a}")), List::<Bytes>::nil()))))))
                                    }
                                    Some((v9, v10)) => {
                                        if f74(v9.clone()) {
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
                                        } else {
                                            Ok::<List<Adt2>, R<Adt12>>(v3)
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
    Ok(f73(f76(v0, v1)?))
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
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f89(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f90(f91(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f57(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m278 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m278.uncons() {
        None => {
            let m279 = f92(f88(v0, v1))?;
            match m279 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f90({
                        let (h280, _) = v5;
                        h280
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h281) = v5;
                        h281
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

pub fn f58(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m282 = f49(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::<Bytes>::nil())));
    match m282.uncons() {
        None => {
            let m283 = f93(f94(v0.clone()), f88(v0, f8(Str::lit("model/authorities.toml"))), v1)?;
            match m283 {
                Adt14::C0(v4) => {
                    Ok(Adt12::C0(v4))
                }
                Adt14::C1(v5) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v5, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
                Adt14::C2(v6, v7) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f90(v6)?, List::cons(f8(Str::lit(" declarations\u{a}")), List::cons(v7, List::<Bytes>::nil())))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f59(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a328 = {
        let a304 = {
            let a292 = {
                let a286 = {
                    let a284 = v1;
                    let a285 = 128u8;
                    fixed_u8::bit_and(a284, a285)
                };
                let a287 = 128u8;
                if equal(a286, a287) {
                    128u64
                } else {
                    0u64
                }
            };
            let a293 = {
                let a290 = {
                    let a288 = v1;
                    let a289 = 64u8;
                    fixed_u8::bit_and(a288, a289)
                };
                let a291 = 64u8;
                if equal(a290, a291) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a292, a293)?
        };
        let a305 = {
            let a302 = {
                let a296 = {
                    let a294 = v1;
                    let a295 = 32u8;
                    fixed_u8::bit_and(a294, a295)
                };
                let a297 = 32u8;
                if equal(a296, a297) {
                    32u64
                } else {
                    0u64
                }
            };
            let a303 = {
                let a300 = {
                    let a298 = v1;
                    let a299 = 16u8;
                    fixed_u8::bit_and(a298, a299)
                };
                let a301 = 16u8;
                if equal(a300, a301) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a302, a303)?
        };
        nat_add(a304, a305)?
    };
    let a329 = {
        let a326 = {
            let a314 = {
                let a308 = {
                    let a306 = v1;
                    let a307 = 8u8;
                    fixed_u8::bit_and(a306, a307)
                };
                let a309 = 8u8;
                if equal(a308, a309) {
                    8u64
                } else {
                    0u64
                }
            };
            let a315 = {
                let a312 = {
                    let a310 = v1;
                    let a311 = 4u8;
                    fixed_u8::bit_and(a310, a311)
                };
                let a313 = 4u8;
                if equal(a312, a313) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a314, a315)?
        };
        let a327 = {
            let a324 = {
                let a318 = {
                    let a316 = v1;
                    let a317 = 2u8;
                    fixed_u8::bit_and(a316, a317)
                };
                let a319 = 2u8;
                if equal(a318, a319) {
                    2u64
                } else {
                    0u64
                }
            };
            let a325 = {
                let a322 = {
                    let a320 = v1;
                    let a321 = 1u8;
                    fixed_u8::bit_and(a320, a321)
                };
                let a323 = 1u8;
                if equal(a322, a323) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a324, a325)?
        };
        nat_add(a326, a327)?
    };
    nat_add(a328, a329)
}

pub fn f60(v0: u64) -> List<Str> {
    let a330 = v0;
    let a331 = 0u64;
    if nat_eq(a330, a331) {
        f95()
    } else {
        let a332 = v0;
        let a333 = 1u64;
        if nat_eq(a332, a333) {
            f96()
        } else {
            let a334 = v0;
            let a335 = 2u64;
            if nat_eq(a334, a335) {
                f97()
            } else {
                let a336 = v0;
                let a337 = 3u64;
                if nat_eq(a336, a337) {
                    f98()
                } else {
                    let a338 = v0;
                    let a339 = 4u64;
                    if nat_eq(a338, a339) {
                        f99()
                    } else {
                        let a340 = v0;
                        let a341 = 5u64;
                        if nat_eq(a340, a341) {
                            f100()
                        } else {
                            let a342 = v0;
                            let a343 = 6u64;
                            if nat_eq(a342, a343) {
                                f101()
                            } else {
                                let a344 = v0;
                                let a345 = 7u64;
                                if nat_eq(a344, a345) {
                                    f102()
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

pub fn f61(v0: Bytes) -> R<Bytes> {
    f103(v0.clone(), 0u64, {
        let a346 = v0;
        length_bytes(a346)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f62(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m347 = v2;
    if m347 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m347 - 1;
        let v5: u64 = f104(v0.clone(), v1)?;
        let a348 = v3;
        let a349 = 1u64;
        if nat_eq(a348, a349) {
            let m350 = f62(v0.clone(), v1, v4, 0u64)?;
            match m350 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f104(v0.clone(), {
                        let (_, h351) = v6.clone();
                        h351
                    })?;
                    let a352 = v7;
                    let a353 = 44u64;
                    if nat_eq(a352, a353) {
                        let m357 = f62(v0, {
                            let a355 = {
                                let (_, h354) = v6;
                                h354
                            };
                            let a356 = 1u64;
                            nat_add(a355, a356)?
                        }, v4, 1u64)?;
                        match m357 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C10(std::rc::Rc::new({
                                    let (h358, _) = v6;
                                    h358
                                }), std::rc::Rc::new({
                                    let (h359, _) = v8;
                                    h359
                                })), {
                                    let (_, h360) = v8;
                                    h360
                                })))
                            }
                        }
                    } else {
                        let a361 = v7;
                        let a362 = 93u64;
                        if nat_eq(a361, a362) {
                            Ok(Some((Adt2::C10(std::rc::Rc::new({
                                let (h363, _) = v6;
                                h363
                            }), std::rc::Rc::new(Adt2::C9)), {
                                let a365 = {
                                    let (_, h364) = v6;
                                    h364
                                };
                                let a366 = 1u64;
                                nat_add(a365, a366)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a367 = v3;
            let a368 = 2u64;
            if nat_eq(a367, a368) {
                let m371 = f105(v0.clone(), {
                    let a369 = v1;
                    let a370 = 1u64;
                    nat_add(a369, a370)?
                })?;
                match m371 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f106({
                            let a372 = v5;
                            let a373 = 34u64;
                            nat_eq(a372, a373)
                        }, {
                            let a375 = f104(v0.clone(), {
                                let (h374, _) = v9.clone();
                                h374
                            })?;
                            let a376 = 58u64;
                            nat_eq(a375, a376)
                        }) {
                            let m380 = f62(v0.clone(), {
                                let a378 = {
                                    let (h377, _) = v9;
                                    h377
                                };
                                let a379 = 1u64;
                                nat_add(a378, a379)?
                            }, v4, 0u64)?;
                            match m380 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f104(v0.clone(), {
                                        let (_, h381) = v10.clone();
                                        h381
                                    })?;
                                    let a382 = v11;
                                    let a383 = 44u64;
                                    if nat_eq(a382, a383) {
                                        let m387 = f62(v0, {
                                            let a385 = {
                                                let (_, h384) = v10;
                                                h384
                                            };
                                            let a386 = 1u64;
                                            nat_add(a385, a386)?
                                        }, v4, 2u64)?;
                                        match m387 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C11({
                                                    let (_, h388) = v9;
                                                    h388
                                                }, std::rc::Rc::new({
                                                    let (h389, _) = v10;
                                                    h389
                                                }), std::rc::Rc::new({
                                                    let (h390, _) = v12;
                                                    h390
                                                })), {
                                                    let (_, h391) = v12;
                                                    h391
                                                })))
                                            }
                                        }
                                    } else {
                                        let a392 = v11;
                                        let a393 = 125u64;
                                        if nat_eq(a392, a393) {
                                            Ok(Some((Adt2::C11({
                                                let (_, h394) = v9;
                                                h394
                                            }, std::rc::Rc::new({
                                                let (h395, _) = v10;
                                                h395
                                            }), std::rc::Rc::new(Adt2::C9)), {
                                                let a397 = {
                                                    let (_, h396) = v10;
                                                    h396
                                                };
                                                let a398 = 1u64;
                                                nat_add(a397, a398)?
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
                let a399 = v5;
                let a400 = 34u64;
                if nat_eq(a399, a400) {
                    let m403 = f105(v0, {
                        let a401 = v1;
                        let a402 = 1u64;
                        nat_add(a401, a402)?
                    })?;
                    match m403 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C6({
                                let (_, h404) = v13;
                                h404
                            }), {
                                let (h405, _) = v13;
                                h405
                            })))
                        }
                    }
                } else {
                    if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a406 = v1;
                            let a407 = 4u64;
                            nat_add(a406, a407)?
                        })))
                    } else {
                        if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a408 = v1;
                                let a409 = 4u64;
                                nat_add(a408, a409)?
                            })))
                        } else {
                            if f32(f46(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a410 = v1;
                                    let a411 = 5u64;
                                    nat_add(a410, a411)?
                                })))
                            } else {
                                let a412 = v5;
                                let a413 = 91u64;
                                if nat_eq(a412, a413) {
                                    let a416 = f104(v0.clone(), {
                                        let a414 = v1;
                                        let a415 = 1u64;
                                        nat_add(a414, a415)?
                                    })?;
                                    let a417 = 93u64;
                                    if nat_eq(a416, a417) {
                                        Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C9)), {
                                            let a418 = v1;
                                            let a419 = 2u64;
                                            nat_add(a418, a419)?
                                        })))
                                    } else {
                                        let m422 = f62(v0, {
                                            let a420 = v1;
                                            let a421 = 1u64;
                                            nat_add(a420, a421)?
                                        }, v4, 1u64)?;
                                        match m422 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                    let (h423, _) = v14;
                                                    h423
                                                })), {
                                                    let (_, h424) = v14;
                                                    h424
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a425 = v5;
                                    let a426 = 123u64;
                                    if nat_eq(a425, a426) {
                                        let a429 = f104(v0.clone(), {
                                            let a427 = v1;
                                            let a428 = 1u64;
                                            nat_add(a427, a428)?
                                        })?;
                                        let a430 = 125u64;
                                        if nat_eq(a429, a430) {
                                            Ok(Some((Adt2::C8(std::rc::Rc::new(Adt2::C9)), {
                                                let a431 = v1;
                                                let a432 = 2u64;
                                                nat_add(a431, a432)?
                                            })))
                                        } else {
                                            let m435 = f62(v0, {
                                                let a433 = v1;
                                                let a434 = 1u64;
                                                nat_add(a433, a434)?
                                            }, v4, 2u64)?;
                                            match m435 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C8(std::rc::Rc::new({
                                                        let (h436, _) = v15;
                                                        h436
                                                    })), {
                                                        let (_, h437) = v15;
                                                        h437
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m438 = f107(v0, v1)?;
                                        match m438 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h439) = v16;
                                                    h439
                                                }, {
                                                    let (h440, _) = v16;
                                                    h440
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

pub fn f63(v0: Fn0, p445: u64, p446: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m444 = (p445, p446);
    {
        let mut m444 = m444;
        loop {
            let step = {
                let (v1, v2) = m444;
                let m441 = v1;
                if m441 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m441 - 1;
                    let m443 = {
                        let c442: Fn0 = v0.clone();
                        c442.apply(v2.clone())?
                    };
                    match m443 {
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
                Ok(next) => { m444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f108(v1, v0, v2)
}

pub fn f65(v0: List<Bytes>) -> List<Bytes> {
    f109(Fn2::F110, List::<Bytes>::nil(), v0)
}

pub fn f66(v0: Fn1, p450: Option<Bytes>, p451: List<Bytes>) -> Option<Bytes> {
    let m449 = (p450, p451);
    {
        let mut m449 = m449;
        loop {
            let step = {
                let (v1, v2) = m449;
                let m447 = v2;
                match m447.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c448: Fn1 = v0.clone();
                            c448.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m452 = v1;
    match m452 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a455 = {
                    let a453 = v3;
                    let a454 = v0;
                    append_bytes(a453, a454)
                };
                let a456 = v2;
                append_bytes(a455, a456)
            })
        }
    }
}

pub fn f68(p461: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m460 = p461;
    {
        let mut m460 = m460;
        loop {
            let step = {
                let v0 = m460;
                let m457 = v0;
                match m457.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h458, _) = v2;
                            h458
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h459) = v2;
                                h459
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m460 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m465 = {
        let a463 = match v0 {
            Adt1::C0(h462, _, _, _) => {
                h462
            }
        };
        let a464 = f40(v1);
        index_list(a463, a464)
    };
    match m465 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f70(p471: Adt2, v1: Bytes) -> Option<Adt2> {
    let m470 = p471;
    {
        let mut m470 = m470;
        loop {
            let step = {
                let v0 = m470;
                let m466 = v0;
                match m466 {
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
                    Adt2::C8(r467) => {
                        let v5 = (*r467).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C9 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(v2, r468, r469) => {
                        let v3 = (*r468).clone();
                        let v4 = (*r469).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: Option<Adt2>) -> Option<Bytes> {
    let m472 = v0;
    match m472 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m473 = v1;
            match m473 {
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

pub fn f72(v0: List<Adt2>) -> List<Bytes> {
    let m474 = v0;
    match m474.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m475 = f71(Some(v1));
            match m475 {
                None => {
                    f72(v2)
                }
                Some(v3) => {
                    List::cons(v3, f72(v2))
                }
            }
        }
    }
}

pub fn f73(v0: Option<Adt2>) -> List<Adt2> {
    let m476 = v0;
    match m476 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f111(v1)
        }
    }
}

pub fn f74(v0: Bytes) -> bool {
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

pub fn f75(v0: Bytes) -> R<Bytes> {
    Ok(f112(f113(v0)?))
}

pub fn f76(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    f114(v0, v1)
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
    let m477 = v0;
    match m477.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m478 = v2;
                match m478.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m479 = v2;
                    match m479.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m480 = v6;
                                match m480.uncons() {
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
                        let m481 = v2;
                        match m481.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m482 = v10;
                                match m482.uncons() {
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

pub fn f79(p485: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m484 = p485;
    {
        let mut m484 = m484;
        loop {
            let step = {
                let v0 = m484;
                let m483 = v0;
                match m483.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f115(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: List<Adt2>) -> List<Bytes> {
    let m486 = v0;
    match m486.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m487 = f70(v1, f8(Str::lit("std")));
            match m487 {
                None => {
                    f80(v2)
                }
                Some(v3) => {
                    List::cons(f116(v3, f8(Str::lit("row"))), f80(v2))
                }
            }
        }
    }
}

pub fn f81(v0: List<Adt2>) -> List<Bytes> {
    let m488 = v0;
    match m488.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f117(f72(f73(f70(v1, f8(Str::lit("defers"))))), f81(v2))
        }
    }
}

pub fn f82(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f83(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a489 = f8(Str::lit("Catalog."));
        let a490 = f118(v1.clone())?;
        append_bytes(a489, a490)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f118(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f119(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f120(Adt2::C7(std::rc::Rc::new(f121(f122(f51(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f120(Adt2::C7(std::rc::Rc::new(f121(f122(f51(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("path")), std::rc::Rc::new(Adt2::C6(v7.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("role")), std::rc::Rc::new(Adt2::C6(v4)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("stratum")), std::rc::Rc::new(Adt2::C3(false, 0u64)), std::rc::Rc::new(Adt2::C9)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a494 = f123(f88(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a492 = {
                let a491 = v7;
                length_bytes(a491)
            };
            let a493 = 4u64;
            nat_sub(a492, a493)
        }))?;
        let a495 = f8(Str::lit("\u{a}"));
        append_bytes(a494, a495)
    }), List::cons(f124(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f84(v0: List<Bytes>) -> Adt2 {
    let m496 = v0;
    match m496.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f84(v2)))
        }
    }
}

pub fn f85(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f118(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f86(v0: Bytes) -> R<Bytes> {
    f125({
        let a498 = {
            let a497 = v0.clone();
            length_bytes(a497)
        };
        let a499 = 1u64;
        nat_add(a498, a499)?
    }, v0)
}

pub fn f87(v0: Bytes) -> R<Result<Adt13, Bytes>> {
    let v1: Bytes = if f126(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f46(v0.clone(), 3u64, {
            let a501 = {
                let a500 = v0;
                length_bytes(a500)
            };
            let a502 = 3u64;
            nat_sub(a501, a502)
        })
    } else {
        v0
    };
    let v2: Bytes = f127(v1)?;
    let v3: u64 = {
        let a503 = v2.clone();
        length_bytes(a503)
    };
    let v4: u64 = f128(v2.clone())?;
    let a504 = v4;
    let a505 = v3;
    if nat_lt(a504, a505) {
        Ok(f129(f130(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m506 = f131(v2.clone())?;
        match m506 {
            Ok(v6) => {
                let m509 = f132(v2.clone(), v6, {
                    let a507 = v3;
                    let a508 = 2u64;
                    nat_add(a507, a508)?
                })?;
                match m509 {
                    Ok(v8) => {
                        if f133(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f129(f130(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a510 = f134(v2.clone(), v8)?;
                            let a511 = 60u64;
                            if nat_eq(a510, a511) {
                                let m514 = f135(v2.clone(), v8, {
                                    let a512 = v3;
                                    let a513 = 2u64;
                                    nat_add(a512, a513)?
                                }, 0u64)?;
                                match m514 {
                                    Ok(v10) => {
                                        let m518 = f132(v2, {
                                            let (_, h515) = v10;
                                            h515
                                        }, {
                                            let a516 = v3;
                                            let a517 = 2u64;
                                            nat_add(a516, a517)?
                                        })?;
                                        match m518 {
                                            Ok(v12) => {
                                                let a519 = v12;
                                                let a520 = v3;
                                                if nat_eq(a519, a520) {
                                                    Ok(Ok::<Adt13, Bytes>({
                                                        let (h521, _) = v10;
                                                        h521
                                                    }))
                                                } else {
                                                    Ok(f129(f130(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f129(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f129(v9))
                                    }
                                }
                            } else {
                                Ok(f129(f130(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f129(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f129(v5))
            }
        }
    }
}

pub fn f88(v0: Adt1, v1: Bytes) -> Bytes {
    let m522 = f68(f69(v0, v1.clone()), v1);
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

pub fn f89(v0: Adt13) -> Bytes {
    let m524 = v0;
    match m524 {
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

pub fn f90(v0: u64) -> R<Bytes> {
    f136(f137(), 10u64, v0)
}

pub fn f91(v0: Adt13) -> R<u64> {
    let m525 = v0;
    match m525 {
        Adt13::C0(_, _, r530) => {
            let v3 = (*r530).clone();
            let a526 = 1u64;
            let a527 = f91(v3)?;
            nat_add(a526, a527)
        }
        Adt13::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt13::C2(_) => {
            Ok(0u64)
        }
        Adt13::C3(r531, r532) => {
            let v8 = (*r531).clone();
            let v9 = (*r532).clone();
            let a528 = f91(v8)?;
            let a529 = f91(v9)?;
            nat_add(a528, a529)
        }
        Adt13::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f92(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m533 = f87(v0)?;
    match m533 {
        Ok(v2) => {
            let m534 = v2;
            match m534 {
                Adt13::C0(v3, r546, r547) => {
                    let v4 = (*r546).clone();
                    let v5 = (*r547).clone();
                    let m535 = f139(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m535 {
                        Ok(v7) => {
                            let m536 = f140(v7.clone(), v3)?;
                            match m536 {
                                Ok(v9) => {
                                    if f141(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m537 = f142(v4, f8(Str::lit("targetNamespace")));
                                            match m537 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m538 = f143(v5, v7, v11)?;
                                        match m538 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f144(v13.clone());
                                                let m539 = f145(v14.clone());
                                                match m539 {
                                                    None => {
                                                        let m540 = f146(v13.clone(), v14.clone());
                                                        match m540 {
                                                            None => {
                                                                let m544 = f147({
                                                                    let a542 = {
                                                                        let a541 = v14.clone();
                                                                        length_list(a541)
                                                                    };
                                                                    let a543 = 1u64;
                                                                    nat_add(a542, a543)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m544 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a545 = v14;
                                                                            length_list(a545)
                                                                        }, f148(f149(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f138(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f138(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f138(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f138(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f138(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f138(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f138(v6))
                        }
                    }
                }
                Adt13::C1(_, _, _) => {
                    Ok(f138(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C2(_) => {
                    Ok(f138(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C3(_, _) => {
                    Ok(f138(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C4 => {
                    Ok(f138(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f138(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f93(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt14> {
    let v3: List<(Bytes, Bytes)> = f150(v1)?;
    f151({
        let a549 = {
            let a548 = v3.clone();
            length_list(a548)
        };
        let a550 = 2u64;
        nat_add(a549, a550)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt15>::nil(), List::<Bytes>::nil())
}

pub fn f94(v0: Adt1) -> List<(Bytes, Option<Bytes>)> {
    f152(match v0 {
        Adt1::C0(h551, _, _, _) => {
            h551
        }
    })
}

pub fn f95() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f96() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f97() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f98() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f99() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f100() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f101() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f102() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f103(v0: Bytes, p579: u64, p580: u64, p581: bool, p582: bool, p583: u64, p584: Bytes) -> R<Bytes> {
    let m578 = (p579, (p580, (p581, (p582, (p583, p584)))));
    {
        let mut m578 = m578;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m578;
                let m552 = v2;
                if m552 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f153(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m552 - 1;
                    let m555 = {
                        let a553 = v0.clone();
                        let a554 = v1;
                        index_bytes(a553, a554)
                    };
                    match m555 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f153(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f59(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a556 = v1;
                                    let a557 = 1u64;
                                    nat_add(a556, a557)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a558 = v9;
                                    let a559 = 34u64;
                                    !nat_eq(a558, a559)
                                }, (if v4 {
                                    false
                                } else {
                                    let a560 = v9;
                                    let a561 = 92u64;
                                    nat_eq(a560, a561)
                                }, (v5, v6))))))
                            } else {
                                let a562 = v9;
                                let a563 = 32u64;
                                if if if if nat_eq(a562, a563) {
                                    true
                                } else {
                                    let a564 = v9;
                                    let a565 = 9u64;
                                    nat_eq(a564, a565)
                                } {
                                    true
                                } else {
                                    let a566 = v9;
                                    let a567 = 10u64;
                                    nat_eq(a566, a567)
                                } {
                                    true
                                } else {
                                    let a568 = v9;
                                    let a569 = 13u64;
                                    nat_eq(a568, a569)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a570 = v1;
                                        let a571 = 1u64;
                                        nat_add(a570, a571)?
                                    }, (v7, (false, (false, ({
                                        let a572 = v1;
                                        let a573 = 1u64;
                                        nat_add(a572, a573)?
                                    }, f153(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a574 = v1;
                                        let a575 = 1u64;
                                        nat_add(a574, a575)?
                                    }, (v7, ({
                                        let a576 = v9;
                                        let a577 = 34u64;
                                        nat_eq(a576, a577)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m578 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: Bytes, v1: u64) -> R<u64> {
    let m587 = {
        let a585 = v0;
        let a586 = v1;
        index_bytes(a585, a586)
    };
    match m587 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a632 = {
                let a608 = {
                    let a596 = {
                        let a590 = {
                            let a588 = v3;
                            let a589 = 128u8;
                            fixed_u8::bit_and(a588, a589)
                        };
                        let a591 = 128u8;
                        if equal(a590, a591) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a597 = {
                        let a594 = {
                            let a592 = v3;
                            let a593 = 64u8;
                            fixed_u8::bit_and(a592, a593)
                        };
                        let a595 = 64u8;
                        if equal(a594, a595) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a596, a597)?
                };
                let a609 = {
                    let a606 = {
                        let a600 = {
                            let a598 = v3;
                            let a599 = 32u8;
                            fixed_u8::bit_and(a598, a599)
                        };
                        let a601 = 32u8;
                        if equal(a600, a601) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a607 = {
                        let a604 = {
                            let a602 = v3;
                            let a603 = 16u8;
                            fixed_u8::bit_and(a602, a603)
                        };
                        let a605 = 16u8;
                        if equal(a604, a605) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a606, a607)?
                };
                nat_add(a608, a609)?
            };
            let a633 = {
                let a630 = {
                    let a618 = {
                        let a612 = {
                            let a610 = v3;
                            let a611 = 8u8;
                            fixed_u8::bit_and(a610, a611)
                        };
                        let a613 = 8u8;
                        if equal(a612, a613) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a619 = {
                        let a616 = {
                            let a614 = v3;
                            let a615 = 4u8;
                            fixed_u8::bit_and(a614, a615)
                        };
                        let a617 = 4u8;
                        if equal(a616, a617) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a618, a619)?
                };
                let a631 = {
                    let a628 = {
                        let a622 = {
                            let a620 = v3;
                            let a621 = 2u8;
                            fixed_u8::bit_and(a620, a621)
                        };
                        let a623 = 2u8;
                        if equal(a622, a623) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a629 = {
                        let a626 = {
                            let a624 = v3;
                            let a625 = 1u8;
                            fixed_u8::bit_and(a624, a625)
                        };
                        let a627 = 1u8;
                        if equal(a626, a627) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a628, a629)?
                };
                nat_add(a630, a631)?
            };
            nat_add(a632, a633)
        }
    }
}

pub fn f105(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f154(v0.clone(), v1, {
        let a635 = {
            let a634 = v0.clone();
            length_bytes(a634)
        };
        let a636 = 1u64;
        nat_add(a635, a636)?
    })?;
    let a637 = f104(v0.clone(), v2)?;
    let a638 = 34u64;
    if nat_eq(a637, a638) {
        Ok(Some(({
            let a639 = v2;
            let a640 = 1u64;
            nat_add(a639, a640)?
        }, f46(v0, v1, {
            let a641 = v2;
            let a642 = v1;
            nat_sub(a641, a642)
        }))))
    } else {
        f155(v0.clone(), v1, {
            let a644 = {
                let a643 = v0;
                length_bytes(a643)
            };
            let a645 = 1u64;
            nat_add(a644, a645)?
        }, Bytes::lit(&[]))
    }
}

pub fn f106(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f107(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a646 = f104(v0.clone(), v1)?;
        let a647 = 45u64;
        nat_eq(a646, a647)
    };
    let v3: u64 = if v2 {
        let a648 = v1;
        let a649 = 1u64;
        nat_add(a648, a649)?
    } else {
        v1
    };
    let v4: u64 = f156(v0.clone(), v3)?;
    let v5: u64 = {
        let a650 = v4;
        let a651 = v3;
        nat_sub(a650, a651)
    };
    let v6: bool = {
        let a652 = f104(v0.clone(), v4)?;
        let a653 = 46u64;
        nat_eq(a652, a653)
    };
    let v7: u64 = if v6 {
        let a654 = v4;
        let a655 = 1u64;
        nat_add(a654, a655)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f156(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f104(v0.clone(), v8)?;
    let v10: bool = {
        let a656 = v9;
        let a657 = 101u64;
        if nat_eq(a656, a657) {
            true
        } else {
            let a658 = v9;
            let a659 = 69u64;
            nat_eq(a658, a659)
        }
    };
    let v11: u64 = f104(v0.clone(), {
        let a660 = v8;
        let a661 = 1u64;
        nat_add(a660, a661)?
    })?;
    let v12: bool = f106(v10, f157({
        let a662 = v11;
        let a663 = 43u64;
        nat_eq(a662, a663)
    }, {
        let a664 = v11;
        let a665 = 45u64;
        nat_eq(a664, a665)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a666 = v8;
            let a667 = 2u64;
            nat_add(a666, a667)?
        } else {
            let a668 = v8;
            let a669 = 1u64;
            nat_add(a668, a669)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f156(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a670 = v14;
        let a671 = v13;
        nat_sub(a670, a671)
    });
    let v16: u64 = f158(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a673 = {
            let a672 = v15;
            length_bytes(a672)
        };
        let a674 = v16;
        nat_sub(a673, a674)
    });
    let a675 = v5;
    let a676 = 0u64;
    if if nat_eq(a675, a676) {
        true
    } else {
        if f106({
            let a677 = 1u64;
            let a678 = v5;
            nat_lt(a677, a678)
        }, {
            let a679 = f104(v0.clone(), v3)?;
            let a680 = 48u64;
            nat_eq(a679, a680)
        }) {
            true
        } else {
            if f106(v6, {
                let a681 = v8;
                let a682 = v7;
                nat_eq(a681, a682)
            }) {
                true
            } else {
                f157(if v10 {
                    let a683 = v14;
                    let a684 = v13;
                    nat_eq(a683, a684)
                } else {
                    false
                }, {
                    let a686 = 18u64;
                    let a687 = {
                        let a685 = v17.clone();
                        length_bytes(a685)
                    };
                    nat_lt(a686, a687)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m691 = {
            let a689 = {
                let a688 = v17.clone();
                length_bytes(a688)
            };
            let a690 = 0u64;
            if nat_eq(a689, a690) {
                Some(0u64)
            } else {
                f159(v17)?
            }
        };
        match m691 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m696 = f160(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a692 = v8;
                    let a693 = v7;
                    nat_sub(a692, a693)
                }), f106(v12, {
                    let a694 = v11;
                    let a695 = 45u64;
                    nat_eq(a694, a695)
                }), v18)?;
                match m696 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a699 = 19u64;
                        let a700 = {
                            let a698 = {
                                let (h697, _) = v19.clone();
                                h697
                            };
                            length_bytes(a698)
                        };
                        if nat_lt(a699, a700) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m702 = f159({
                                let (h701, _) = v19.clone();
                                h701
                            })?;
                            match m702 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f106(v2, {
                                        let a703 = 0u64;
                                        let a704 = v20;
                                        nat_lt(a703, a704)
                                    });
                                    if f161(v21, v20) {
                                        Ok(Some((v14, {
                                            let a706 = {
                                                let (_, h705) = v19.clone();
                                                h705
                                            };
                                            let a707 = 0u64;
                                            if nat_eq(a706, a707) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h708) = v19;
                                                    h708
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

pub fn f108(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h709, _) = v2.clone();
        h709
    };
    let v4: u64 = {
        let (h710, _) = {
            let (_, h711) = v2.clone();
            h711
        };
        h710
    };
    let v5: List<Bytes> = {
        let (_, h712) = {
            let (_, h713) = v2;
            h713
        };
        h712
    };
    let m714 = f162(v0.clone(), v3);
    match m714 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a715 = v6;
            let a716 = v1;
            if equal(a715, a716) {
                Ok(Some(({
                    let a717 = v3;
                    let a718 = 1u64;
                    nat_add(a717, a718)?
                }, ({
                    let a719 = v3;
                    let a720 = 1u64;
                    nat_add(a719, a720)?
                }, List::cons(f46(v0, v4, {
                    let a721 = v3;
                    let a722 = v4;
                    nat_sub(a721, a722)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a723 = v3;
                    let a724 = 1u64;
                    nat_add(a723, a724)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f109(v0: Fn2, p728: List<Bytes>, p729: List<Bytes>) -> List<Bytes> {
    let m727 = (p728, p729);
    {
        let mut m727 = m727;
        loop {
            let step = {
                let (v1, v2) = m727;
                let m725 = v2;
                match m725.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c726: Fn2 = v0.clone();
                            c726.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m727 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f111(v0: Adt2) -> List<Adt2> {
    let m730 = v0;
    match m730 {
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
        Adt2::C7(r731) => {
            let v3 = (*r731).clone();
            f111(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r732, r733) => {
            let v1 = (*r732).clone();
            let v2 = (*r733).clone();
            List::cons(v1, f111(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f112(v0: Adt16) -> Bytes {
    let a754 = f163(match v0.clone() {
        Adt16::C0(h734, _, _, _, _, _, _, _) => {
            h734
        }
    });
    let a755 = {
        let a752 = f163(match v0.clone() {
            Adt16::C0(_, h735, _, _, _, _, _, _) => {
                h735
            }
        });
        let a753 = {
            let a750 = f163(match v0.clone() {
                Adt16::C0(_, _, h736, _, _, _, _, _) => {
                    h736
                }
            });
            let a751 = {
                let a748 = f163(match v0.clone() {
                    Adt16::C0(_, _, _, h737, _, _, _, _) => {
                        h737
                    }
                });
                let a749 = {
                    let a746 = f163(match v0.clone() {
                        Adt16::C0(_, _, _, _, h738, _, _, _) => {
                            h738
                        }
                    });
                    let a747 = {
                        let a744 = f163(match v0.clone() {
                            Adt16::C0(_, _, _, _, _, h739, _, _) => {
                                h739
                            }
                        });
                        let a745 = {
                            let a742 = f163(match v0.clone() {
                                Adt16::C0(_, _, _, _, _, _, h740, _) => {
                                    h740
                                }
                            });
                            let a743 = f163(match v0 {
                                Adt16::C0(_, _, _, _, _, _, _, h741) => {
                                    h741
                                }
                            });
                            append_bytes(a742, a743)
                        };
                        append_bytes(a744, a745)
                    };
                    append_bytes(a746, a747)
                };
                append_bytes(a748, a749)
            };
            append_bytes(a750, a751)
        };
        append_bytes(a752, a753)
    };
    append_bytes(a754, a755)
}

pub fn f113(v0: Bytes) -> R<Adt16> {
    let v1: u64 = {
        let a756 = v0.clone();
        length_bytes(a756)
    };
    let v2: u64 = {
        let a762 = {
            let a759 = {
                let a757 = v1;
                let a758 = 72u64;
                nat_add(a757, a758)?
            };
            let a760 = 64u64;
            let a761 = 0u64;
            nat_quot(a759, a760, a761)
        };
        let a763 = 64u64;
        nat_mul(a762, a763)?
    };
    let (_, h764) = f164(Fn3::F165(v0, v1, v2), {
        let a765 = v2;
        let a766 = 64u64;
        let a767 = 0u64;
        nat_quot(a765, a766, a767)
    }, (0u64, f166()))?;
    Ok(h764)
}

pub fn f114(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m768 = f167(f168(v0.clone(), v1.clone()), v1.clone());
    match m768 {
        None => {
            f43(f88(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f115(v0: Bytes, p771: List<Bytes>) -> bool {
    let m770 = p771;
    {
        let mut m770 = m770;
        loop {
            let step = {
                let v1 = m770;
                let m769 = v1;
                match m769.uncons() {
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
                Ok(next) => { m770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: Adt2, v1: Bytes) -> Bytes {
    let m772 = f71(f70(v0, v1));
    match m772 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f117(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m773 = v0;
    match m773.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f117(v3, v1))
        }
    }
}

pub fn f118(v0: Bytes) -> R<Bytes> {
    f169(v0.clone(), 0u64, {
        let a774 = v0;
        length_bytes(a774)
    }, true, Bytes::lit(&[]))
}

pub fn f119(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m775 = v0;
    match m775.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f119(v3, v1))
        }
    }
}

pub fn f120(v0: Adt2) -> R<Bytes> {
    let a776 = f170(v0)?;
    let a777 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a776, a777))
}

pub fn f121(v0: List<Adt2>) -> Adt2 {
    let m778 = v0;
    match m778.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(v1), std::rc::Rc::new(f121(v2)))
        }
    }
}

pub fn f122(v0: List<Adt2>, v1: Bytes, v2: Bytes, v3: Adt2) -> List<Adt2> {
    let m779 = v0;
    match m779.uncons() {
        None => {
            List::cons(v3, List::<Adt2>::nil())
        }
        Some((v4, v5)) => {
            if f32(f116(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f171(v5, v1, v2))
            } else {
                List::cons(v4, f122(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f123(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f148(f172(f173(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f124(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f120(Adt2::C7(std::rc::Rc::new(f84(f174(f117(v1, f72(f51(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f125(p790: u64, p791: Bytes) -> R<Bytes> {
    let m789 = (p790, p791);
    {
        let mut m789 = m789;
        loop {
            let step = {
                let (v0, v1) = m789;
                let m780 = v0;
                if m780 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m780 - 1;
                    if f53(v1.clone(), f8(Str::lit("-- "))) {
                        let m781 = f175(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m781 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f46(v1.clone(), {
                                    let a782 = v3;
                                    let a783 = 1u64;
                                    nat_add(a782, a783)?
                                }, {
                                    let a787 = {
                                        let a784 = v1;
                                        length_bytes(a784)
                                    };
                                    let a788 = {
                                        let a785 = v3;
                                        let a786 = 1u64;
                                        nat_add(a785, a786)?
                                    };
                                    nat_sub(a787, a788)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
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

pub fn f126(v0: Bytes, v1: Bytes, v2: u64) -> bool {
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

pub fn f127(v0: Bytes) -> R<Bytes> {
    let (_, h799) = {
        let (h800, _) = f176(Fn4::F177(v0.clone()), {
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

pub fn f128(v0: Bytes) -> R<u64> {
    let (h804, _) = f178(Fn5::F179(v0.clone()), {
        let a806 = {
            let a805 = v0;
            length_bytes(a805)
        };
        let a807 = 1u64;
        nat_add(a806, a807)?
    }, 0u64)?;
    Ok(h804)
}

pub fn f129(v0: Bytes) -> Result<Adt13, Bytes> {
    Err::<Adt13, Bytes>(v0)
}

pub fn f130(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a808 = v0;
        utf8_encode(a808)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f90(v1)?, List::<Bytes>::nil())))))
}

pub fn f131(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f133(v0.clone(), 0u64, Str::lit("<?xml")) {
        f180(f134(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m810 = f181(v0.clone(), {
            let a809 = Str::lit("?>");
            utf8_encode(a809)
        }, 5u64)?;
        match m810 {
            None => {
                Ok(f182(f130(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m812 = f181(f46(v0.clone(), 0u64, v1), {
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
                        let v3: u64 = f183(v0.clone(), {
                            let a815 = v2;
                            let a816 = 8u64;
                            nat_add(a815, a816)?
                        })?;
                        let v4: u64 = f183(v0.clone(), {
                            let a817 = v3;
                            let a818 = 1u64;
                            nat_add(a817, a818)?
                        })?;
                        let _: u64 = f134(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m821 = f181(v0.clone(), f46(v0.clone(), v4, 1u64), {
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
                        if f184(f46(v0, {
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
                            Ok(f182(f130(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f132(v0: Bytes, p834: u64, p835: u64) -> R<Result<u64, Bytes>> {
    let m833 = (p834, p835);
    {
        let mut m833 = m833;
        loop {
            let step = {
                let (v1, v2) = m833;
                let m830 = v2;
                if m830 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f182(f130(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m830 - 1;
                    let v4: u64 = f183(v0.clone(), v1)?;
                    if f133(v0.clone(), v4, Str::lit("<!--")) {
                        let m831 = f185(v0.clone(), v4)?;
                        match m831 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f182(v5)))
                            }
                        }
                    } else {
                        if f133(v0.clone(), v4, Str::lit("<?")) {
                            let m832 = f186(v0.clone(), v4)?;
                            match m832 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f182(v7)))
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

pub fn f133(v0: Bytes, v1: u64, v2: Str) -> bool {
    f126(v0, {
        let a836 = v2;
        utf8_encode(a836)
    }, v1)
}

pub fn f134(v0: Bytes, v1: u64) -> R<u64> {
    f104(v0, v1)
}

pub fn f135(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt13, u64), Bytes>> {
    let m837 = v2;
    if m837 == 0 {
        Ok(f187(f130(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m837 - 1;
        let a838 = v3;
        let a839 = 0u64;
        if nat_eq(a838, a839) {
            let v5: u64 = f188(v0.clone(), {
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
                Ok(f187(f130(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f46(v0.clone(), {
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
                let m852 = f189(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m852 {
                    Ok(v8) => {
                        if f133(v0.clone(), {
                            let (_, h853) = v8.clone();
                            h853
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
                                let (h854, _) = v8;
                                h854
                            }), std::rc::Rc::new(Adt13::C4)), {
                                let a856 = {
                                    let (_, h855) = v8;
                                    h855
                                };
                                let a857 = 2u64;
                                nat_add(a856, a857)?
                            })))
                        } else {
                            let m861 = f135(v0.clone(), {
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
                                    let v12: u64 = f188(v0.clone(), v11)?;
                                    if f32(f46(v0.clone(), v11, {
                                        let a865 = v12;
                                        let a866 = v11;
                                        nat_sub(a865, a866)
                                    }), v6.clone()) {
                                        let v13: u64 = f183(v0.clone(), v12)?;
                                        let a867 = f134(v0, v13)?;
                                        let a868 = 62u64;
                                        if nat_eq(a867, a868) {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
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
                                            Ok(f187(f130(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f187(f130(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h873) = v10;
                                            h873
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f187(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f187(v7))
                    }
                }
            }
        } else {
            let a874 = f134(v0.clone(), v1)?;
            let a875 = 256u64;
            if nat_eq(a874, a875) {
                Ok(f187(f130(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f133(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v1)))
                } else {
                    if f133(v0.clone(), v1, Str::lit("<!--")) {
                        let m876 = f185(v0.clone(), v1)?;
                        match m876 {
                            Ok(v15) => {
                                f135(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f187(v14))
                            }
                        }
                    } else {
                        if f133(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m880 = f181(v0.clone(), {
                                let a877 = Str::lit("]]>");
                                utf8_encode(a877)
                            }, {
                                let a878 = v1;
                                let a879 = 9u64;
                                nat_add(a878, a879)?
                            })?;
                            match m880 {
                                None => {
                                    Ok(f187(f130(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m883 = f135(v0.clone(), {
                                        let a881 = v16;
                                        let a882 = 3u64;
                                        nat_add(a881, a882)?
                                    }, v4, 1u64)?;
                                    match m883 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2(f46(v0, {
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
                                            Ok(f187(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f133(v0.clone(), v1, Str::lit("<?")) {
                                let m892 = f186(v0.clone(), v1)?;
                                match m892 {
                                    Ok(v20) => {
                                        f135(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f187(v19))
                                    }
                                }
                            } else {
                                if f133(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f187(f130(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a893 = f134(v0.clone(), v1)?;
                                    let a894 = 60u64;
                                    if nat_eq(a893, a894) {
                                        let m895 = f135(v0.clone(), v1, v4, 0u64)?;
                                        match m895 {
                                            Ok(v22) => {
                                                let m897 = f135(v0, {
                                                    let (_, h896) = v22;
                                                    h896
                                                }, v4, 1u64)?;
                                                match m897 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new({
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
                                                        Ok(f187(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f187(v21))
                                            }
                                        }
                                    } else {
                                        let m901 = f190(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m901 {
                                            Ok(v26) => {
                                                let m903 = f135(v0, {
                                                    let (_, h902) = v26;
                                                    h902
                                                }, v4, 1u64)?;
                                                match m903 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2({
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
                                                        Ok(f187(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f187(v25))
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

pub fn f136(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a907 = v2;
    let a908 = 0u64;
    if nat_eq(a907, a908) {
        Ok(f191(v0, 0u64))
    } else {
        let (_, h909) = {
            let (h910, _) = f176(Fn4::F192(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h910
        };
        Ok(h909)
    }
}

pub fn f137() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f138(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f139(v0: Adt13, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m911 = v0;
    match m911 {
        Adt13::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt13::C1(v2, v3, r916) => {
            let v4 = (*r916).clone();
            let m912 = f139(v4, v1);
            match m912 {
                Ok(v6) => {
                    if f32(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f32(v3.clone(), f194()) {
                            true
                        } else {
                            f32(v3.clone(), f195())
                        } {
                            f193(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f53(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f46(v2.clone(), 6u64, {
                                let a914 = {
                                    let a913 = v2;
                                    length_bytes(a913)
                                };
                                let a915 = 6u64;
                                nat_sub(a914, a915)
                            });
                            if f32(v7.clone(), f8(Str::lit("xmlns"))) {
                                f193(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f32(v7.clone(), f8(Str::lit("xml"))) {
                                    !f32(v3.clone(), f194())
                                } else {
                                    false
                                } {
                                    f193(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f32(v3.clone(), f194()) {
                                        true
                                    } else {
                                        f32(v3.clone(), f195())
                                    } {
                                        !f32(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f193(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f32(v3.clone(), Bytes::lit(&[])) {
                                            f193(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f193(v5)
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

pub fn f140(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f196(v1.clone())?;
    if f32(v2.clone(), Bytes::lit(&[])) {
        let m917 = f197(v0, Bytes::lit(&[]));
        match m917 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f32(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f194(), f198(v1)?)))
        } else {
            let m918 = f197(v0, v2.clone());
            match m918 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f198(v1)?)))
                }
            }
        }
    }
}

pub fn f141(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f32({
        let (h919, _) = v0;
        h919
    }, f199()) {
        f32({
            let (_, h920) = v0;
            h920
        }, {
            let a921 = v1;
            utf8_encode(a921)
        })
    } else {
        false
    }
}

pub fn f142(p925: Adt13, v1: Bytes) -> Option<Bytes> {
    let m924 = p925;
    {
        let mut m924 = m924;
        loop {
            let step = {
                let v0 = m924;
                let m922 = v0;
                match m922 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt13::C1(v2, v3, r923) => {
                        let v4 = (*r923).clone();
                        if f32(v2, v1.clone()) {
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
                Ok(next) => { m924 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Adt13, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m926 = v0;
    match m926 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C3(r937, r938) => {
            let v3 = (*r937).clone();
            let v4 = (*r938).clone();
            let m927 = f143(v4, v1.clone(), v2.clone())?;
            match m927 {
                Ok(v6) => {
                    let m928 = v3;
                    match m928 {
                        Adt13::C0(v7, r935, r936) => {
                            let v8 = (*r935).clone();
                            let v9 = (*r936).clone();
                            let m929 = f139(v8.clone(), v1);
                            match m929 {
                                Ok(v11) => {
                                    let m930 = f140(v11.clone(), v7)?;
                                    match m930 {
                                        Ok(v13) => {
                                            if f141(v13, Str::lit("simpleType")) {
                                                let m931 = f142(v8, f8(Str::lit("name")));
                                                match m931 {
                                                    None => {
                                                        Ok(f200(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m932 = f201(v14.clone(), v9, v11, v2)?;
                                                        match m932 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f118(v14)?, (match v16.clone() {
                                                                    Adt17::C0(_, h933) => {
                                                                        h933
                                                                    }
                                                                }, match v16 {
                                                                    Adt17::C0(h934, _) => {
                                                                        h934
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f200(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f200(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f200(v10))
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
                    Ok(f200(v5))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f144(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m939 = v0;
    match m939.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h940, _) = v1;
                h940
            }, f144(v2))
        }
    }
}

pub fn f145(p943: List<Bytes>) -> Option<Bytes> {
    let m942 = p943;
    {
        let mut m942 = m942;
        loop {
            let step = {
                let v0 = m942;
                let m941 = v0;
                match m941.uncons() {
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
                Ok(next) => { m942 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(p949: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m948 = p949;
    {
        let mut m948 = m948;
        loop {
            let step = {
                let v0 = m948;
                let m944 = v0;
                match m944.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m947 = f202({
                            let (h945, _) = {
                                let (_, h946) = v2;
                                h946
                            };
                            h945
                        }, v1.clone());
                        match m947 {
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
                Ok(next) => { m948 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(p960: u64, p961: List<(Bytes, (List<Bytes>, Bytes))>, p962: List<Bytes>, p963: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m959 = (p960, (p961, (p962, p963)));
    {
        let mut m959 = m959;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m959;
                let m950 = v0;
                if m950 == 0 {
                    let m951 = v1;
                    match m951.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f203(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m950 - 1;
                    let m952 = v1.clone();
                    match m952.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m953 = f204(v1, v2.clone());
                            match m953 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f203(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h954) = v9.clone();
                                        h954
                                    }, (List::cons({
                                        let (h955, _) = {
                                            let (h956, _) = v9.clone();
                                            h956
                                        };
                                        h955
                                    }, v2), List::cons({
                                        let (_, h957) = {
                                            let (h958, _) = v9;
                                            h958
                                        };
                                        h957
                                    }, v3)))))
                                }
                            }
                        }
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

pub fn f148(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m964 = v0;
    match m964.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m965 = v3.clone();
            match m965.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a968 = v2;
                    let a969 = {
                        let a966 = v1.clone();
                        let a967 = f148(v3, v1);
                        append_bytes(a966, a967)
                    };
                    append_bytes(a968, a969)
                }
            }
        }
    }
}

pub fn f149(p972: List<Bytes>, p973: List<Bytes>) -> List<Bytes> {
    let m971 = (p972, p973);
    {
        let mut m971 = m971;
        loop {
            let step = {
                let (v0, v1) = m971;
                let m970 = v0;
                match m970.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m971 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f205(f173(v0)?, None::<Bytes>)
}

pub fn f151(p994: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p995: List<(Bytes, (Bytes, Bytes))>, p996: List<Adt15>, p997: List<Bytes>) -> R<Adt14> {
    let m993 = (p994, (p995, (p996, p997)));
    {
        let mut m993 = m993;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m993;
                let m974 = v0;
                if m974 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m974 - 1;
                    let m975 = v3;
                    match m975.uncons() {
                        None => {
                            let m976 = v5.clone();
                            match m976.uncons() {
                                None => {
                                    let m977 = f206(f207(v4, List::<Adt15>::nil()))?;
                                    match m977 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C2({
                                                let (h978, _) = v8;
                                                h978
                                            }, {
                                                let (_, h979) = v8;
                                                h979
                                            })))
                                        }
                                        Err(v7) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v7)))
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C0(v5)))
                                }
                            }
                        }
                        Some((v11, v12)) => {
                            let v13: Bytes = {
                                let (h980, _) = v11.clone();
                                h980
                            };
                            if if f115(v13.clone(), f208(v4.clone())) {
                                true
                            } else {
                                f115(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>((v6, (v12, (v4, v5))))
                            } else {
                                let m981 = f209(v1.clone(), v13.clone());
                                match m981 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m982 = v14;
                                        match m982 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(f210(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m983 = f211(v13.clone(), v15, f212(v2.clone(), v13))?;
                                                match m983 {
                                                    Ok(v17) => {
                                                        let m988 = f213(v17.clone(), {
                                                            let (h984, _) = {
                                                                let (_, h985) = v11.clone();
                                                                h985
                                                            };
                                                            h984
                                                        }, {
                                                            let (_, h986) = {
                                                                let (_, h987) = v11;
                                                                h987
                                                            };
                                                            h986
                                                        });
                                                        match m988 {
                                                            None => {
                                                                let m991 = f214(match v17.clone() {
                                                                    Adt15::C0(_, _, _, _, h989, _) => {
                                                                        h989
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt15::C0(_, _, _, h990, _, _) => {
                                                                        h990
                                                                    }
                                                                })?;
                                                                match m991 {
                                                                    Ok(v20) => {
                                                                        let m992 = f215(v20, v17.clone(), v2.clone())?;
                                                                        match m992 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>((v6, (f216(v12, v22), (List::cons(v17, v4), v5))))
                                                                            }
                                                                            Err(v21) => {
                                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v21)))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v19) => {
                                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v19)))
                                                                    }
                                                                }
                                                            }
                                                            Some(v18) => {
                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v18)))
                                                            }
                                                        }
                                                    }
                                                    Err(v16) => {
                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v16)))
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
                Ok(next) => { m993 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: List<List<(Bytes, Option<Bytes>)>>) -> List<(Bytes, Option<Bytes>)> {
    let m998 = v0;
    match m998.uncons() {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some((v1, v2)) => {
            f217(v1, f152(v2))
        }
    }
}

pub fn f153(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1001 = v3;
    let a1002 = f46(v0, v1, {
        let a999 = v2;
        let a1000 = v1;
        nat_sub(a999, a1000)
    });
    append_bytes(a1001, a1002)
}

pub fn f154(v0: Bytes, p1015: u64, p1016: u64) -> R<u64> {
    let m1014 = (p1015, p1016);
    {
        let mut m1014 = m1014;
        loop {
            let step = {
                let (v1, v2) = m1014;
                let m1003 = v2;
                if m1003 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1003 - 1;
                    let v4: u64 = f104(v0.clone(), v1)?;
                    let a1004 = v4;
                    let a1005 = 34u64;
                    if nat_eq(a1004, a1005) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1006 = v4;
                        let a1007 = 92u64;
                        if nat_eq(a1006, a1007) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1008 = v4;
                            let a1009 = 32u64;
                            if nat_lt(a1008, a1009) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1010 = v4;
                                let a1011 = 256u64;
                                if nat_eq(a1010, a1011) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1012 = v1;
                                        let a1013 = 1u64;
                                        nat_add(a1012, a1013)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1014 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Bytes, p1042: u64, p1043: u64, p1044: Bytes) -> R<Option<(u64, Bytes)>> {
    let m1041 = (p1042, (p1043, p1044));
    {
        let mut m1041 = m1041;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1041;
                let m1017 = v2;
                if m1017 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m1017 - 1;
                    let v5: u64 = f154(v0.clone(), v1, {
                        let a1021 = {
                            let a1019 = {
                                let a1018 = v0.clone();
                                length_bytes(a1018)
                            };
                            let a1020 = v1;
                            nat_sub(a1019, a1020)
                        };
                        let a1022 = 1u64;
                        nat_add(a1021, a1022)?
                    })?;
                    let v6: Bytes = {
                        let a1025 = v3;
                        let a1026 = f46(v0.clone(), v1, {
                            let a1023 = v5;
                            let a1024 = v1;
                            nat_sub(a1023, a1024)
                        });
                        append_bytes(a1025, a1026)
                    };
                    let v7: u64 = f104(v0.clone(), v5)?;
                    let a1027 = v7;
                    let a1028 = 34u64;
                    if nat_eq(a1027, a1028) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a1029 = v5;
                            let a1030 = 1u64;
                            nat_add(a1029, a1030)?
                        }, v6))))
                    } else {
                        let a1031 = v7;
                        let a1032 = 92u64;
                        if nat_eq(a1031, a1032) {
                            let m1033 = f218(v0.clone(), v5, v6)?;
                            match m1033 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a1036 = {
                                        let (_, h1034) = {
                                            let (_, h1035) = v8.clone();
                                            h1035
                                        };
                                        h1034
                                    };
                                    let a1037 = 0u64;
                                    if nat_eq(a1036, a1037) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h1038, _) = v8;
                                            h1038
                                        }, (v4, {
                                            let (h1039, _) = {
                                                let (_, h1040) = v8;
                                                h1040
                                            };
                                            h1039
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
                Ok(next) => { m1041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Bytes, v1: u64) -> R<u64> {
    let (h1045, _) = f178(Fn5::F219(v0.clone()), {
        let a1047 = {
            let a1046 = v0;
            length_bytes(a1046)
        };
        let a1048 = 1u64;
        nat_add(a1047, a1048)?
    }, v1)?;
    Ok(h1045)
}

pub fn f157(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f158(v0: Bytes) -> R<u64> {
    let (h1049, _) = f178(Fn5::F220(v0.clone()), {
        let a1051 = {
            let a1050 = v0;
            length_bytes(a1050)
        };
        let a1052 = 1u64;
        nat_add(a1051, a1052)?
    }, 0u64)?;
    Ok(h1049)
}

pub fn f159(v0: Bytes) -> R<Option<u64>> {
    if f221(v0.clone())? {
        Ok(Some({
            let (_, h1053) = {
                let (h1054, _) = f222(Fn6::F223(v0.clone()), {
                    let a1055 = v0;
                    length_bytes(a1055)
                }, (0u64, 0u64))?;
                h1054
            };
            h1053
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f160(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a1057 = {
            let a1056 = v1.clone();
            length_bytes(a1056)
        };
        let a1058 = f224(v1)?;
        nat_sub(a1057, a1058)
    });
    let v5: Bytes = {
        let a1059 = v0;
        let a1060 = v4.clone();
        append_bytes(a1059, a1060)
    };
    let v6: u64 = f158(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a1062 = {
            let a1061 = v5;
            length_bytes(a1061)
        };
        let a1063 = v6;
        nat_sub(a1062, a1063)
    });
    let v8: u64 = {
        let a1064 = v4;
        length_bytes(a1064)
    };
    let v9: u64 = {
        let a1065 = v7.clone();
        length_bytes(a1065)
    };
    let a1066 = v9;
    let a1067 = 0u64;
    if nat_eq(a1066, a1067) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f157(v2, {
            let a1068 = v3;
            let a1069 = v8;
            nat_lt(a1068, a1069)
        }) {
            let v10: u64 = if v2 {
                let a1070 = v3;
                let a1071 = v8;
                nat_add(a1070, a1071)?
            } else {
                let a1072 = v8;
                let a1073 = v3;
                nat_sub(a1072, a1073)
            };
            let v11: u64 = f224(v7.clone())?;
            let a1074 = v10;
            let a1075 = v11;
            if nat_le(a1074, a1075) {
                Ok(Some((f46(v7, 0u64, {
                    let a1076 = v9;
                    let a1077 = v10;
                    nat_sub(a1076, a1077)
                }), 0u64)))
            } else {
                let a1080 = {
                    let a1078 = v10;
                    let a1079 = v11;
                    nat_sub(a1078, a1079)
                };
                let a1081 = 19u64;
                if nat_le(a1080, a1081) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1082 = v9;
                        let a1083 = v11;
                        nat_sub(a1082, a1083)
                    }), {
                        let a1084 = v10;
                        let a1085 = v11;
                        nat_sub(a1084, a1085)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1086 = v3;
                let a1087 = v8;
                nat_sub(a1086, a1087)
            };
            let a1090 = {
                let a1088 = v9;
                let a1089 = v12;
                nat_add(a1088, a1089)?
            };
            let a1091 = 19u64;
            if nat_le(a1090, a1091) {
                Ok(Some(({
                    let a1092 = v7;
                    let a1093 = f225(v12);
                    append_bytes(a1092, a1093)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f161(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1094 = 0u64;
        let a1095 = v1;
        if nat_lt(a1094, a1095) {
            let a1096 = v1;
            let a1097 = 9223372036854775808u64;
            nat_le(a1096, a1097)
        } else {
            false
        }
    } else {
        let a1098 = v1;
        let a1099 = 9223372036854775807u64;
        nat_le(a1098, a1099)
    }
}

pub fn f162(v0: Bytes, v1: u64) -> Option<u8> {
    let a1100 = v0;
    let a1101 = v1;
    index_bytes(a1100, a1101)
}

pub fn f163(v0: u32) -> Bytes {
    let a1130 = f226({
        let a1102 = f227(v0, 28u32);
        let a1103 = 15u32;
        fixed_u32::bit_and(a1102, a1103)
    });
    let a1131 = {
        let a1128 = f226({
            let a1104 = f227(v0, 24u32);
            let a1105 = 15u32;
            fixed_u32::bit_and(a1104, a1105)
        });
        let a1129 = {
            let a1126 = f226({
                let a1106 = f227(v0, 20u32);
                let a1107 = 15u32;
                fixed_u32::bit_and(a1106, a1107)
            });
            let a1127 = {
                let a1124 = f226({
                    let a1108 = f227(v0, 16u32);
                    let a1109 = 15u32;
                    fixed_u32::bit_and(a1108, a1109)
                });
                let a1125 = {
                    let a1122 = f226({
                        let a1110 = f227(v0, 12u32);
                        let a1111 = 15u32;
                        fixed_u32::bit_and(a1110, a1111)
                    });
                    let a1123 = {
                        let a1120 = f226({
                            let a1112 = f227(v0, 8u32);
                            let a1113 = 15u32;
                            fixed_u32::bit_and(a1112, a1113)
                        });
                        let a1121 = {
                            let a1118 = f226({
                                let a1114 = f227(v0, 4u32);
                                let a1115 = 15u32;
                                fixed_u32::bit_and(a1114, a1115)
                            });
                            let a1119 = f226({
                                let a1116 = v0;
                                let a1117 = 15u32;
                                fixed_u32::bit_and(a1116, a1117)
                            });
                            append_bytes(a1118, a1119)
                        };
                        append_bytes(a1120, a1121)
                    };
                    append_bytes(a1122, a1123)
                };
                append_bytes(a1124, a1125)
            };
            append_bytes(a1126, a1127)
        };
        append_bytes(a1128, a1129)
    };
    append_bytes(a1130, a1131)
}

pub fn f164(v0: Fn3, p1135: u64, p1136: (u64, Adt16)) -> R<(u64, Adt16)> {
    let m1134 = (p1135, p1136);
    {
        let mut m1134 = m1134;
        loop {
            let step = {
                let (v1, v2) = m1134;
                let m1132 = v1;
                if m1132 == 0 {
                    Err::<(u64, (u64, Adt16)), R<(u64, Adt16)>>(Ok(v2))
                } else {
                    let v3 = m1132 - 1;
                    Ok::<(u64, (u64, Adt16)), R<(u64, Adt16)>>((v3, {
                        let c1133: Fn3 = v0.clone();
                        c1133.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt16)) -> R<(u64, Adt16)> {
    Ok(({
        let a1138 = {
            let (h1137, _) = v3.clone();
            h1137
        };
        let a1139 = 64u64;
        nat_add(a1138, a1139)?
    }, f228({
        let (_, h1140) = v3;
        h1140
    }, f229(v0, v1, v2, {
        let (h1141, _) = v3;
        h1141
    })?)?))
}

pub fn f166() -> Adt16 {
    Adt16::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f167(p1146: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m1145 = p1146;
    {
        let mut m1145 = m1145;
        loop {
            let step = {
                let v0 = m1145;
                let m1142 = v0;
                match m1142.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1143, _) = v2;
                            h1143
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h1144) = v2;
                                h1144
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m1150 = {
        let a1148 = match v0 {
            Adt1::C0(_, _, _, h1147) => {
                h1147
            }
        };
        let a1149 = f40(v1);
        index_list(a1148, a1149)
    };
    match m1150 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f169(v0: Bytes, p1171: u64, p1172: u64, p1173: bool, p1174: Bytes) -> R<Bytes> {
    let m1170 = (p1171, (p1172, (p1173, p1174)));
    {
        let mut m1170 = m1170;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1170;
                let m1151 = v2;
                if m1151 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1151 - 1;
                    let m1154 = {
                        let a1152 = v0.clone();
                        let a1153 = v1;
                        index_bytes(a1152, a1153)
                    };
                    match m1154 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f59(v6)?;
                            if f230(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1155 = v1;
                                    let a1156 = 1u64;
                                    nat_add(a1155, a1156)?
                                }, (v5, (true, v4))))
                            } else {
                                if f106(f231(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1157 = v1;
                                        let a1158 = 1u64;
                                        nat_add(a1157, a1158)?
                                    }, (v5, (false, {
                                        let a1162 = v4;
                                        let a1163 = f46({
                                            let a1159 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1159)
                                        }, {
                                            let a1160 = v7;
                                            let a1161 = 97u64;
                                            nat_sub(a1160, a1161)
                                        }, 1u64);
                                        append_bytes(a1162, a1163)
                                    }))))
                                } else {
                                    if f157(f231(v7), f157(f232(v7), f233(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1164 = v1;
                                            let a1165 = 1u64;
                                            nat_add(a1164, a1165)?
                                        }, (v5, (false, {
                                            let a1166 = v4;
                                            let a1167 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1166, a1167)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1168 = v1;
                                            let a1169 = 1u64;
                                            nat_add(a1168, a1169)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: Adt2) -> R<Bytes> {
    let m1175 = v0;
    match m1175 {
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
            f234(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f235(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f236(v9, v10)
        }
        Adt2::C6(v11) => {
            f237(v11)
        }
        Adt2::C7(r1176) => {
            let v12 = (*r1176).clone();
            Ok(f238(f239(v12)?))
        }
        Adt2::C8(r1177) => {
            let v13 = (*r1177).clone();
            f240(f241(v13)?)
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

pub fn f171(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1178 = v0;
    match m1178.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f32(f116(v3.clone(), v1.clone()), v2.clone()) {
                f171(v4, v1, v2)
            } else {
                List::cons(v3, f171(v4, v1, v2))
            }
        }
    }
}

pub fn f172(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1179 = v0;
    match m1179.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f53(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f148(f242(f174(List::cons(v1, f243(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f172(v3, v1)?))
            }
        }
    }
}

pub fn f173(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1180 = v0.clone();
        length_bytes(a1180)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1181, _) = f63(Fn0::F244(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1181
    };
    let v3: u64 = {
        let (h1182, _) = {
            let (_, h1183) = v2.clone();
            h1183
        };
        h1182
    };
    let v4: List<Bytes> = {
        let (_, h1184) = {
            let (_, h1185) = v2;
            h1185
        };
        h1184
    };
    let a1186 = v3;
    let a1187 = v1;
    if nat_lt(a1186, a1187) {
        Ok(f65(List::cons(f46(v0, v3, {
            let a1188 = v1;
            let a1189 = v3;
            nat_sub(a1188, a1189)
        }), v4)))
    } else {
        Ok(f65(v4))
    }
}

pub fn f174(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f245(f109(Fn2::F246, List::<Bytes>::nil(), f247(v0)?)))
}

pub fn f175(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1190 = v0.clone();
        length_bytes(a1190)
    };
    let v3: u64 = {
        let a1191 = v1.clone();
        length_bytes(a1191)
    };
    let v4: u64 = {
        let (h1192, _) = f178(Fn5::F248(v1.clone(), v3, v0.clone(), v2), {
            let a1193 = v2;
            let a1194 = 1u64;
            nat_add(a1193, a1194)?
        }, 0u64)?;
        h1192
    };
    if f126(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f176(v0: Fn4, p1199: u64, p1200: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1198 = (p1199, p1200);
    {
        let mut m1198 = m1198;
        loop {
            let step = {
                let (v1, v2) = m1198;
                let m1195 = v1;
                if m1195 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1195 - 1;
                    let m1197 = {
                        let c1196: Fn4 = v0.clone();
                        c1196.apply(v2.clone())?
                    };
                    match m1197 {
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
                Ok(next) => { m1198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f249(v0, v1)
}

pub fn f178(v0: Fn5, p1205: u64, p1206: u64) -> R<(u64, bool)> {
    let m1204 = (p1205, p1206);
    {
        let mut m1204 = m1204;
        loop {
            let step = {
                let (v1, v2) = m1204;
                let m1201 = v1;
                if m1201 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1201 - 1;
                    let m1203 = {
                        let c1202: Fn5 = v0.clone();
                        c1202.apply(v2)?
                    };
                    match m1203 {
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
                Ok(next) => { m1204 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f250(v0, v1)
}

pub fn f180(v0: u64) -> R<bool> {
    f251(v0, f252())
}

pub fn f181(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1207, _) = f178(Fn5::F253(v1.clone(), v0.clone()), {
            let a1209 = {
                let a1208 = v0.clone();
                length_bytes(a1208)
            };
            let a1210 = 1u64;
            nat_add(a1209, a1210)?
        }, v2)?;
        h1207
    };
    if f126(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f182(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f183(v0: Bytes, v1: u64) -> R<u64> {
    let (h1211, _) = f178(Fn5::F254(v0.clone()), {
        let a1213 = {
            let a1212 = v0;
            length_bytes(a1212)
        };
        let a1214 = 1u64;
        nat_add(a1213, a1214)?
    }, v1)?;
    Ok(h1211)
}

pub fn f184(v0: Bytes) -> bool {
    f255(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f185(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1218 = f181(v0.clone(), {
        let a1215 = Str::lit("--");
        utf8_encode(a1215)
    }, {
        let a1216 = v1;
        let a1217 = 4u64;
        nat_add(a1216, a1217)?
    })?;
    match m1218 {
        None => {
            Ok(f182(f130(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1221 = f134(v0, {
                let a1219 = v2;
                let a1220 = 2u64;
                nat_add(a1219, a1220)?
            })?;
            let a1222 = 62u64;
            if nat_eq(a1221, a1222) {
                Ok(Ok::<u64, Bytes>({
                    let a1223 = v2;
                    let a1224 = 3u64;
                    nat_add(a1223, a1224)?
                }))
            } else {
                Ok(f182(f130(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f186(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f188(v0.clone(), {
        let a1225 = v1;
        let a1226 = 2u64;
        nat_add(a1225, a1226)?
    })?;
    let a1229 = v2;
    let a1230 = {
        let a1227 = v1;
        let a1228 = 2u64;
        nat_add(a1227, a1228)?
    };
    if nat_eq(a1229, a1230) {
        Ok(f182(f130(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1233 = v2;
        let a1234 = {
            let a1231 = v1;
            let a1232 = 5u64;
            nat_add(a1231, a1232)?
        };
        if if nat_eq(a1233, a1234) {
            f255(f46(v0.clone(), {
                let a1235 = v1;
                let a1236 = 2u64;
                nat_add(a1235, a1236)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f182(f130(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1238 = f181(v0, {
                let a1237 = Str::lit("?>");
                utf8_encode(a1237)
            }, v2)?;
            match m1238 {
                None => {
                    Ok(f182(f130(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1239 = v3;
                        let a1240 = 2u64;
                        nat_add(a1239, a1240)?
                    }))
                }
            }
        }
    }
}

pub fn f187(v0: Bytes) -> Result<(Adt13, u64), Bytes> {
    Err::<(Adt13, u64), Bytes>(v0)
}

pub fn f188(v0: Bytes, v1: u64) -> R<u64> {
    let (h1241, _) = f178(Fn5::F256(v0.clone(), v1), {
        let a1243 = {
            let a1242 = v0;
            length_bytes(a1242)
        };
        let a1244 = 1u64;
        nat_add(a1243, a1244)?
    }, v1)?;
    Ok(h1241)
}

pub fn f189(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt13, u64), Bytes>> {
    let m1245 = v2;
    if m1245 == 0 {
        Ok(f187(f130(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1245 - 1;
        let v5: u64 = f183(v0.clone(), v1)?;
        let a1246 = f134(v0.clone(), v5)?;
        let a1247 = 62u64;
        if if nat_eq(a1246, a1247) {
            true
        } else {
            f133(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v5)))
        } else {
            let a1248 = v5;
            let a1249 = v1;
            if nat_eq(a1248, a1249) {
                Ok(f187(f130(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f188(v0.clone(), v5)?;
                let a1250 = v6;
                let a1251 = v5;
                if nat_eq(a1250, a1251) {
                    Ok(f187(f130(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f46(v0.clone(), v5, {
                        let a1252 = v6;
                        let a1253 = v5;
                        nat_sub(a1252, a1253)
                    });
                    if f255(v7.clone(), v3.clone()) {
                        Ok(f187(f130(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f183(v0.clone(), v6)?;
                        let a1254 = f134(v0.clone(), v8)?;
                        let a1255 = 61u64;
                        if nat_eq(a1254, a1255) {
                            let v9: u64 = f183(v0.clone(), {
                                let a1256 = v8;
                                let a1257 = 1u64;
                                nat_add(a1256, a1257)?
                            })?;
                            let v10: u64 = f134(v0.clone(), v9)?;
                            let a1258 = v10;
                            let a1259 = 34u64;
                            if if nat_eq(a1258, a1259) {
                                true
                            } else {
                                let a1260 = v10;
                                let a1261 = 39u64;
                                nat_eq(a1260, a1261)
                            } {
                                let m1264 = f190(v0.clone(), {
                                    let a1262 = v9;
                                    let a1263 = 1u64;
                                    nat_add(a1262, a1263)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1264 {
                                    Ok(v12) => {
                                        let m1266 = f189(v0, {
                                            let (_, h1265) = v12;
                                            h1265
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1266 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C1(v7, {
                                                    let (h1267, _) = v12;
                                                    h1267
                                                }, std::rc::Rc::new({
                                                    let (h1268, _) = v14;
                                                    h1268
                                                })), {
                                                    let (_, h1269) = v14;
                                                    h1269
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f187(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f187(v11))
                                    }
                                }
                            } else {
                                Ok(f187(f130(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f187(f130(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f190(v0: Bytes, p1309: u64, p1310: u64, p1311: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1308 = (p1309, (p1310, p1311));
    {
        let mut m1308 = m1308;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1308;
                let m1270 = v2;
                if m1270 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(f130(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1270 - 1;
                    let v7: u64 = f134(v0.clone(), v1)?;
                    if if v4 {
                        let a1271 = v7;
                        let a1272 = v5;
                        nat_eq(a1271, a1272)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1273 = v1;
                            let a1274 = 1u64;
                            nat_add(a1273, a1274)?
                        }))))
                    } else {
                        let a1275 = v7;
                        let a1276 = 256u64;
                        if nat_eq(a1275, a1276) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(f130(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1277 = v7;
                            let a1278 = 60u64;
                            if nat_eq(a1277, a1278) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(f130(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1279 = v7;
                                let a1280 = 38u64;
                                if nat_eq(a1279, a1280) {
                                    let m1281 = f258(v0.clone(), v1)?;
                                    match m1281 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1282) = v9;
                                                h1282
                                            }, (v6, {
                                                let a1284 = v3;
                                                let a1285 = {
                                                    let (h1283, _) = v9;
                                                    h1283
                                                };
                                                append_bytes(a1284, a1285)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1286 = v7;
                                        let a1287 = 9u64;
                                        if if nat_eq(a1286, a1287) {
                                            true
                                        } else {
                                            let a1288 = v7;
                                            let a1289 = 10u64;
                                            nat_eq(a1288, a1289)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1290 = v1;
                                                let a1291 = 1u64;
                                                nat_add(a1290, a1291)?
                                            }, (v6, {
                                                let a1292 = v3;
                                                let a1293 = f8(Str::lit(" "));
                                                append_bytes(a1292, a1293)
                                            })))
                                        } else {
                                            let v10: u64 = f259(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1296 = v3;
                                                let a1297 = f46(v0.clone(), v1, {
                                                    let a1294 = v10;
                                                    let a1295 = v1;
                                                    nat_sub(a1294, a1295)
                                                });
                                                append_bytes(a1296, a1297)
                                            })))
                                        }
                                    } else {
                                        let a1298 = v7;
                                        let a1299 = 93u64;
                                        if nat_eq(a1298, a1299) {
                                            if f133(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(f130(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1300 = v1;
                                                    let a1301 = 1u64;
                                                    nat_add(a1300, a1301)?
                                                }, (v6, {
                                                    let a1302 = v3;
                                                    let a1303 = f8(Str::lit("]"));
                                                    append_bytes(a1302, a1303)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f259(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1306 = v3;
                                                let a1307 = f46(v0.clone(), v1, {
                                                    let a1304 = v11;
                                                    let a1305 = v1;
                                                    nat_sub(a1304, a1305)
                                                });
                                                append_bytes(a1306, a1307)
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
                Ok(next) => { m1308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Bytes, v1: u64) -> Bytes {
    let m1315 = {
        let a1312 = v0;
        let a1313 = v1;
        let a1314 = 1u64;
        slice_bytes(a1312, a1313, a1314)
    };
    match m1315 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f192(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f260(v1, v0, v2)
}

pub fn f193(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f194() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f195() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f196(v0: Bytes) -> R<Bytes> {
    let m1316 = f181(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1316 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f46(v0, 0u64, v1))
        }
    }
}

pub fn f197(p1321: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1320 = p1321;
    {
        let mut m1320 = m1320;
        loop {
            let step = {
                let v0 = m1320;
                let m1317 = v0;
                match m1317.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1318, _) = v2;
                            h1318
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1319) = v2;
                                h1319
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
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

pub fn f198(v0: Bytes) -> R<Bytes> {
    let m1322 = f181(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1322 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f46(v0.clone(), {
                let a1323 = v1;
                let a1324 = 1u64;
                nat_add(a1323, a1324)?
            }, {
                let a1328 = {
                    let a1325 = v0;
                    length_bytes(a1325)
                };
                let a1329 = {
                    let a1326 = v1;
                    let a1327 = 1u64;
                    nat_add(a1326, a1327)?
                };
                nat_sub(a1328, a1329)
            }))
        }
    }
}

pub fn f199() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f200(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f201(v0: Bytes, v1: Adt13, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt17, Bytes>> {
    let v4: Bytes = f118(v0.clone())?;
    let m1330 = f261(v1, v2)?;
    match m1330 {
        Ok(v6) => {
            let m1331 = v6;
            match m1331 {
                None => {
                    Ok(f262(f263(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt24::C0(h1332, _, _, _) => {
                            h1332
                        }
                    };
                    let v9: Adt13 = match v7.clone() {
                        Adt24::C0(_, h1333, _, _) => {
                            h1333
                        }
                    };
                    let v10: Adt13 = match v7.clone() {
                        Adt24::C0(_, _, h1334, _) => {
                            h1334
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt24::C0(_, _, _, h1335) => {
                            h1335
                        }
                    };
                    if f32(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f262(f263(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f32(v8, f8(Str::lit("list"))) {
                            let m1336 = f142(v9, f8(Str::lit("itemType")));
                            match m1336 {
                                None => {
                                    Ok(f262(f263(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1337 = f264(v11, v3, v12)?;
                                    match m1337 {
                                        Ok(v14) => {
                                            if f53(match v14.clone() {
                                                Adt17::C0(h1338, _) => {
                                                    h1338
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f262(f263(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt17, Bytes>(Adt17::C0(f265(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt17::C0(h1339, _) => {
                                                        h1339
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt17::C0(_, h1340) => {
                                                        h1340
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f262(f263(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1341 = f142(v9, f8(Str::lit("base")));
                            match m1341 {
                                None => {
                                    Ok(f262(f263(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1342 = f264(v11.clone(), v3, v15)?;
                                    match m1342 {
                                        Ok(v17) => {
                                            let m1343 = f266(v10, v11)?;
                                            match m1343 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f267(v19, List::<Bytes>::nil())?;
                                                    let m1344 = v20.clone();
                                                    match m1344.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt17, Bytes>(Adt17::C0(f265(v4, match v17.clone() {
                                                                Adt17::C0(h1345, _) => {
                                                                    h1345
                                                                }
                                                            }), match v17 {
                                                                Adt17::C0(_, h1346) => {
                                                                    h1346
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1347 = f268(f269(f72(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1347 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt17, Bytes>(Adt17::C0(f270(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f262(f263(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f262(f263(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f262(f263(v16, v0)))
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
            Ok(f262(f263(v5, v0)))
        }
    }
}

pub fn f202(p1350: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1349 = p1350;
    {
        let mut m1349 = m1349;
        loop {
            let step = {
                let v0 = m1349;
                let m1348 = v0;
                match m1348.uncons() {
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
                Ok(next) => { m1349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f204(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1351 = v0;
    match m1351.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1354 = f202({
                let (h1352, _) = {
                    let (_, h1353) = v2.clone();
                    h1353
                };
                h1352
            }, v1.clone());
            match m1354 {
                None => {
                    Some((({
                        let (h1355, _) = v2;
                        h1355
                    }, {
                        let (_, h1356) = {
                            let (_, h1357) = v2;
                            h1357
                        };
                        h1356
                    }), v3))
                }
                Some(_) => {
                    let m1358 = f204(v3, v1);
                    match m1358 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1359, _) = v5;
                                h1359
                            }, List::cons(v2, {
                                let (_, h1360) = v5;
                                h1360
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f205(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1361 = v0;
    match m1361.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f32(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f205(v3, None::<Bytes>)
            } else {
                let m1362 = f271(v2.clone(), f8(Str::lit("url = ")))?;
                match m1362 {
                    None => {
                        let m1363 = f271(v2, f8(Str::lit("artifact = ")))?;
                        match m1363 {
                            None => {
                                f205(v3, v1)
                            }
                            Some(v5) => {
                                let m1364 = v1.clone();
                                match m1364 {
                                    None => {
                                        f205(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f205(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f205(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f206(v0: List<Adt15>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1365 = f272(v0.clone())?;
    match m1365 {
        Ok(v2) => {
            let v3: Adt18 = Adt18::C0(v2.clone(), f273(v0.clone()));
            let m1366 = f274(f275(v0)?, v3, v2)?;
            match m1366 {
                Ok(v5) => {
                    let m1367 = f276(f277(v5), List::<Adt21>::nil());
                    match m1367 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f278(v7.clone());
                            let m1368 = f279(v7.clone(), v8.clone());
                            match m1368 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f280(v7.clone(), v8.clone(), f281(v7.clone())?);
                                    let m1374 = f282({
                                        let a1372 = {
                                            let a1371 = v8.clone();
                                            length_list(a1371)
                                        };
                                        let a1373 = 1u64;
                                        nat_add(a1372, a1373)?
                                    }, v7.clone(), f283(v7, f284(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1374 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1375 = v12.clone();
                                                length_list(a1375)
                                            }, f148(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f210(f8(Str::lit("the type ")), {
                                        let (h1369, _) = v9;
                                        h1369
                                    }, f210(f8(Str::lit(" uses ")), {
                                        let (_, h1370) = v9;
                                        h1370
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

pub fn f207(p1378: List<Adt15>, p1379: List<Adt15>) -> List<Adt15> {
    let m1377 = (p1378, p1379);
    {
        let mut m1377 = m1377;
        loop {
            let step = {
                let (v0, v1) = m1377;
                let m1376 = v0;
                match m1376.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: List<Adt15>) -> List<Bytes> {
    let m1380 = v0;
    match m1380.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt15::C0(h1381, _, _, _, _, _) => {
                    h1381
                }
            }, f208(v2))
        }
    }
}

pub fn f209(p1386: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1385 = p1386;
    {
        let mut m1385 = m1385;
        loop {
            let step = {
                let v0 = m1385;
                let m1382 = v0;
                match m1382.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1383, _) = v2;
                            h1383
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1384) = v2;
                                h1384
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f210(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f211(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt15, Bytes>> {
    let m1387 = f87(v1.clone())?;
    match m1387 {
        Ok(v4) => {
            let m1388 = v4;
            match m1388 {
                Adt13::C0(v5, r1393, r1394) => {
                    let v6 = (*r1393).clone();
                    let v7 = (*r1394).clone();
                    let m1389 = f139(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1389 {
                        Ok(v9) => {
                            let m1390 = f140(v9.clone(), v5)?;
                            match m1390 {
                                Ok(v11) => {
                                    if f141(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt15, Bytes>(Adt15::C0(v0, v2, {
                                            let m1391 = f142(v6, f8(Str::lit("targetNamespace")));
                                            match m1391 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1392 = v1;
                                            length_bytes(a1392)
                                        })))
                                    } else {
                                        Ok(Err::<Adt15, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(": the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)")), List::<Bytes>::nil())))))
                                    }
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt15, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<Adt15, Bytes>(v8))
                        }
                    }
                }
                Adt13::C1(_, _, _) => {
                    Ok(Err::<Adt15, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C2(_) => {
                    Ok(Err::<Adt15, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C3(_, _) => {
                    Ok(Err::<Adt15, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C4 => {
                    Ok(Err::<Adt15, Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt15, Bytes>(f210(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f212(p1399: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1398 = p1399;
    {
        let mut m1398 = m1398;
        loop {
            let step = {
                let v0 = m1398;
                let m1395 = v0;
                match m1395.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (_, h1396) = v2;
                            h1396
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1397, _) = v2;
                                h1397
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Adt15, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f32(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f32(match v0.clone() {
            Adt15::C0(_, _, h1400, _, _, _) => {
                h1400
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt15::C0(h1401, _, _, _, _, _) => {
                    h1401
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt15::C0(_, _, h1402, _, _, _) => {
                    h1402
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f32(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f214(v0: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt20>, Bytes>> {
    let m1403 = v0;
    match m1403 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
        Adt13::C3(r1414, r1415) => {
            let v2 = (*r1414).clone();
            let v3 = (*r1415).clone();
            let m1404 = f214(v3, v1.clone())?;
            match m1404 {
                Ok(v5) => {
                    let m1405 = v2;
                    match m1405 {
                        Adt13::C0(v6, r1412, r1413) => {
                            let v7 = (*r1412).clone();
                            let v8 = (*r1413).clone();
                            let m1406 = f139(v7.clone(), v1);
                            match m1406 {
                                Ok(v10) => {
                                    let m1407 = f140(v10.clone(), v6)?;
                                    match m1407 {
                                        Ok(v12) => {
                                            if if f32({
                                                let (h1408, _) = v12.clone();
                                                h1408
                                            }, f199()) {
                                                f285(f32({
                                                    let (_, h1409) = v12.clone();
                                                    h1409
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt20>, Bytes>(List::cons(Adt20::C0({
                                                    let (h1410, _) = v12;
                                                    h1410
                                                }, {
                                                    let (_, h1411) = v12;
                                                    h1411
                                                }, v7, v8, v10), v5)))
                                            } else {
                                                Ok(Ok::<List<Adt20>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt20>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt20>, Bytes>(v9))
                                }
                            }
                        }
                        Adt13::C1(_, _, _) => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                        Adt13::C2(_) => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                        Adt13::C3(_, _) => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                        Adt13::C4 => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt20>, Bytes>(v4))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
    }
}

pub fn f215(v0: List<Adt20>, v1: Adt15, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1416 = v0;
    match m1416.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1417 = f215(v4, v1.clone(), v2.clone())?;
            match m1417 {
                Ok(v6) => {
                    if if f286(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f286(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f286(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt15::C0(h1418, _, _, _, _, _) => {
                                h1418
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt20::C0(_, h1419, _, _, _) => {
                                h1419
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f286(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f286(v3.clone(), Str::lit("include"))
                        } {
                            let m1420 = f287(v3.clone(), Str::lit("schemaLocation"));
                            match m1420 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt15::C0(h1421, _, _, _, _, _) => {
                                            h1421
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt20::C0(_, h1422, _, _, _) => {
                                            h1422
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1424 = f288(match v1.clone() {
                                        Adt15::C0(_, h1423, _, _, _, _) => {
                                            h1423
                                        }
                                    }, v7)?;
                                    match m1424 {
                                        Ok(v9) => {
                                            let m1426 = f289(v2, v9.clone());
                                            match m1426 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt15::C0(h1427, _, _, _, _, _) => {
                                                            h1427
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt20::C0(_, h1428, _, _, _) => {
                                                            h1428
                                                        }
                                                    }, if f286(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt15::C0(_, _, h1429, _, _, _) => {
                                                                h1429
                                                            }
                                                        }
                                                    } else {
                                                        f290(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f210(match v1 {
                                                Adt15::C0(h1425, _, _, _, _, _) => {
                                                    h1425
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

pub fn f216(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1430 = v0;
    match m1430.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f216(v3, v1))
        }
    }
}

pub fn f217(v0: List<(Bytes, Option<Bytes>)>, v1: List<(Bytes, Option<Bytes>)>) -> List<(Bytes, Option<Bytes>)> {
    let m1431 = v0;
    match m1431.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f217(v3, v1))
        }
    }
}

pub fn f218(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f104(v0.clone(), {
        let a1432 = v1;
        let a1433 = 1u64;
        nat_add(a1432, a1433)?
    })?;
    let v4: Bytes = {
        let a1434 = v3;
        let a1435 = 117u64;
        if nat_eq(a1434, a1435) {
            f291(v0, v1)?
        } else {
            f292(v3)
        }
    };
    let a1436 = v4.clone();
    let a1437 = Bytes::lit(&[]);
    if equal(a1436, a1437) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1438 = v3;
            let a1439 = 117u64;
            if nat_eq(a1438, a1439) {
                let a1440 = v1;
                let a1441 = 6u64;
                nat_add(a1440, a1441)?
            } else {
                let a1442 = v1;
                let a1443 = 2u64;
                nat_add(a1442, a1443)?
            }
        }, ({
            let a1444 = v2;
            let a1445 = v4;
            append_bytes(a1444, a1445)
        }, 0u64))))
    }
}

pub fn f219(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f293(v0, v1)
}

pub fn f220(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f294(v0, v1)
}

pub fn f221(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1446 = v0.clone();
        length_bytes(a1446)
    };
    let a1447 = 0u64;
    let a1448 = v1;
    if nat_lt(a1447, a1448) {
        let a1449 = f295(v0.clone())?;
        let a1450 = v1;
        if nat_eq(a1449, a1450) {
            let a1451 = 1u64;
            let a1452 = v1;
            if nat_lt(a1451, a1452) {
                let m1455 = {
                    let a1453 = v0;
                    let a1454 = 0u64;
                    index_bytes(a1453, a1454)
                };
                match m1455 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1456 = v2;
                        let a1457 = 48u8;
                        Ok(!equal(a1456, a1457))
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

pub fn f222(v0: Fn6, p1462: u64, p1463: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1461 = (p1462, p1463);
    {
        let mut m1461 = m1461;
        loop {
            let step = {
                let (v1, v2) = m1461;
                let m1458 = v1;
                if m1458 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1458 - 1;
                    let m1460 = {
                        let c1459: Fn6 = v0.clone();
                        c1459.apply(v2)?
                    };
                    match m1460 {
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
                Ok(next) => { m1461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f296(v0, v1)
}

pub fn f224(v0: Bytes) -> R<u64> {
    let (h1464, _) = f178(Fn5::F297(v0.clone()), {
        let a1466 = {
            let a1465 = v0;
            length_bytes(a1465)
        };
        let a1467 = 1u64;
        nat_add(a1466, a1467)?
    }, 0u64)?;
    Ok(h1464)
}

pub fn f225(v0: u64) -> Bytes {
    let m1468 = v0;
    if m1468 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1468 - 1;
        let a1469 = f8(Str::lit("0"));
        let a1470 = f225(v1);
        append_bytes(a1469, a1470)
    }
}

pub fn f226(v0: u32) -> Bytes {
    let a1473 = {
        let a1471 = v0;
        let a1472 = 8u32;
        fixed_u32::bit_and(a1471, a1472)
    };
    let a1474 = 0u32;
    if equal(a1473, a1474) {
        let a1477 = {
            let a1475 = v0;
            let a1476 = 4u32;
            fixed_u32::bit_and(a1475, a1476)
        };
        let a1478 = 0u32;
        if equal(a1477, a1478) {
            let a1481 = {
                let a1479 = v0;
                let a1480 = 2u32;
                fixed_u32::bit_and(a1479, a1480)
            };
            let a1482 = 0u32;
            if equal(a1481, a1482) {
                let a1485 = {
                    let a1483 = v0;
                    let a1484 = 1u32;
                    fixed_u32::bit_and(a1483, a1484)
                };
                let a1486 = 0u32;
                if equal(a1485, a1486) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1489 = {
                    let a1487 = v0;
                    let a1488 = 1u32;
                    fixed_u32::bit_and(a1487, a1488)
                };
                let a1490 = 0u32;
                if equal(a1489, a1490) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1493 = {
                let a1491 = v0;
                let a1492 = 2u32;
                fixed_u32::bit_and(a1491, a1492)
            };
            let a1494 = 0u32;
            if equal(a1493, a1494) {
                let a1497 = {
                    let a1495 = v0;
                    let a1496 = 1u32;
                    fixed_u32::bit_and(a1495, a1496)
                };
                let a1498 = 0u32;
                if equal(a1497, a1498) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1501 = {
                    let a1499 = v0;
                    let a1500 = 1u32;
                    fixed_u32::bit_and(a1499, a1500)
                };
                let a1502 = 0u32;
                if equal(a1501, a1502) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1505 = {
            let a1503 = v0;
            let a1504 = 4u32;
            fixed_u32::bit_and(a1503, a1504)
        };
        let a1506 = 0u32;
        if equal(a1505, a1506) {
            let a1509 = {
                let a1507 = v0;
                let a1508 = 2u32;
                fixed_u32::bit_and(a1507, a1508)
            };
            let a1510 = 0u32;
            if equal(a1509, a1510) {
                let a1513 = {
                    let a1511 = v0;
                    let a1512 = 1u32;
                    fixed_u32::bit_and(a1511, a1512)
                };
                let a1514 = 0u32;
                if equal(a1513, a1514) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1517 = {
                    let a1515 = v0;
                    let a1516 = 1u32;
                    fixed_u32::bit_and(a1515, a1516)
                };
                let a1518 = 0u32;
                if equal(a1517, a1518) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1521 = {
                let a1519 = v0;
                let a1520 = 2u32;
                fixed_u32::bit_and(a1519, a1520)
            };
            let a1522 = 0u32;
            if equal(a1521, a1522) {
                let a1525 = {
                    let a1523 = v0;
                    let a1524 = 1u32;
                    fixed_u32::bit_and(a1523, a1524)
                };
                let a1526 = 0u32;
                if equal(a1525, a1526) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1529 = {
                    let a1527 = v0;
                    let a1528 = 1u32;
                    fixed_u32::bit_and(a1527, a1528)
                };
                let a1530 = 0u32;
                if equal(a1529, a1530) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f227(v0: u32, v1: u32) -> u32 {
    let m1533 = {
        let a1531 = v0;
        let a1532 = v1;
        fixed_u32::shift_right(a1531, a1532)
    };
    match m1533 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f228(v0: Adt16, v1: Adt22) -> R<Adt16> {
    let v2: Adt23 = f298(Fn7::F299, 64u64, Adt23::C0(match v0.clone() {
        Adt16::C0(h1534, _, _, _, _, _, _, _) => {
            h1534
        }
    }, match v0.clone() {
        Adt16::C0(_, h1535, _, _, _, _, _, _) => {
            h1535
        }
    }, match v0.clone() {
        Adt16::C0(_, _, h1536, _, _, _, _, _) => {
            h1536
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h1537, _, _, _, _) => {
            h1537
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, h1538, _, _, _) => {
            h1538
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, h1539, _, _) => {
            h1539
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, h1540, _) => {
            h1540
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, h1541) => {
            h1541
        }
    }, v1, 0u64))?;
    Ok(Adt16::C0(f300(match v0.clone() {
        Adt16::C0(h1542, _, _, _, _, _, _, _) => {
            h1542
        }
    }, match v2.clone() {
        Adt23::C0(h1543, _, _, _, _, _, _, _, _, _) => {
            h1543
        }
    }), f300(match v0.clone() {
        Adt16::C0(_, h1544, _, _, _, _, _, _) => {
            h1544
        }
    }, match v2.clone() {
        Adt23::C0(_, h1545, _, _, _, _, _, _, _, _) => {
            h1545
        }
    }), f300(match v0.clone() {
        Adt16::C0(_, _, h1546, _, _, _, _, _) => {
            h1546
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1547, _, _, _, _, _, _, _) => {
            h1547
        }
    }), f300(match v0.clone() {
        Adt16::C0(_, _, _, h1548, _, _, _, _) => {
            h1548
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1549, _, _, _, _, _, _) => {
            h1549
        }
    }), f300(match v0.clone() {
        Adt16::C0(_, _, _, _, h1550, _, _, _) => {
            h1550
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1551, _, _, _, _, _) => {
            h1551
        }
    }), f300(match v0.clone() {
        Adt16::C0(_, _, _, _, _, h1552, _, _) => {
            h1552
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1553, _, _, _, _) => {
            h1553
        }
    }), f300(match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, h1554, _) => {
            h1554
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1555, _, _, _) => {
            h1555
        }
    }), f300(match v0 {
        Adt16::C0(_, _, _, _, _, _, _, h1556) => {
            h1556
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1557, _, _) => {
            h1557
        }
    })))
}

pub fn f229(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f301(v0.clone(), v1, v2, v3)?, f301(v0.clone(), v1, v2, {
        let a1558 = v3;
        let a1559 = 4u64;
        nat_add(a1558, a1559)?
    })?, f301(v0.clone(), v1, v2, {
        let a1560 = v3;
        let a1561 = 8u64;
        nat_add(a1560, a1561)?
    })?, f301(v0.clone(), v1, v2, {
        let a1562 = v3;
        let a1563 = 12u64;
        nat_add(a1562, a1563)?
    })?, f301(v0.clone(), v1, v2, {
        let a1564 = v3;
        let a1565 = 16u64;
        nat_add(a1564, a1565)?
    })?, f301(v0.clone(), v1, v2, {
        let a1566 = v3;
        let a1567 = 20u64;
        nat_add(a1566, a1567)?
    })?, f301(v0.clone(), v1, v2, {
        let a1568 = v3;
        let a1569 = 24u64;
        nat_add(a1568, a1569)?
    })?, f301(v0.clone(), v1, v2, {
        let a1570 = v3;
        let a1571 = 28u64;
        nat_add(a1570, a1571)?
    })?, f301(v0.clone(), v1, v2, {
        let a1572 = v3;
        let a1573 = 32u64;
        nat_add(a1572, a1573)?
    })?, f301(v0.clone(), v1, v2, {
        let a1574 = v3;
        let a1575 = 36u64;
        nat_add(a1574, a1575)?
    })?, f301(v0.clone(), v1, v2, {
        let a1576 = v3;
        let a1577 = 40u64;
        nat_add(a1576, a1577)?
    })?, f301(v0.clone(), v1, v2, {
        let a1578 = v3;
        let a1579 = 44u64;
        nat_add(a1578, a1579)?
    })?, f301(v0.clone(), v1, v2, {
        let a1580 = v3;
        let a1581 = 48u64;
        nat_add(a1580, a1581)?
    })?, f301(v0.clone(), v1, v2, {
        let a1582 = v3;
        let a1583 = 52u64;
        nat_add(a1582, a1583)?
    })?, f301(v0.clone(), v1, v2, {
        let a1584 = v3;
        let a1585 = 56u64;
        nat_add(a1584, a1585)?
    })?, f301(v0, v1, v2, {
        let a1586 = v3;
        let a1587 = 60u64;
        nat_add(a1586, a1587)?
    })?))
}

pub fn f230(v0: u64) -> bool {
    f157({
        let a1588 = v0;
        let a1589 = 95u64;
        nat_eq(a1588, a1589)
    }, f157({
        let a1590 = v0;
        let a1591 = 45u64;
        nat_eq(a1590, a1591)
    }, f157({
        let a1592 = v0;
        let a1593 = 32u64;
        nat_eq(a1592, a1593)
    }, f157({
        let a1594 = v0;
        let a1595 = 46u64;
        nat_eq(a1594, a1595)
    }, {
        let a1596 = v0;
        let a1597 = 47u64;
        nat_eq(a1596, a1597)
    }))))
}

pub fn f231(v0: u64) -> bool {
    f106({
        let a1598 = 97u64;
        let a1599 = v0;
        nat_le(a1598, a1599)
    }, {
        let a1600 = v0;
        let a1601 = 122u64;
        nat_le(a1600, a1601)
    })
}

pub fn f232(v0: u64) -> bool {
    f106({
        let a1602 = 65u64;
        let a1603 = v0;
        nat_le(a1602, a1603)
    }, {
        let a1604 = v0;
        let a1605 = 90u64;
        nat_le(a1604, a1605)
    })
}

pub fn f233(v0: u64) -> bool {
    f106({
        let a1606 = 48u64;
        let a1607 = v0;
        nat_le(a1606, a1607)
    }, {
        let a1608 = v0;
        let a1609 = 57u64;
        nat_le(a1608, a1609)
    })
}

pub fn f234(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1610 = 0u64;
        let a1611 = v1;
        nat_lt(a1610, a1611)
    } else {
        false
    } {
        let a1612 = Bytes::lit(&[0x2d]);
        let a1613 = f90(v1)?;
        Ok(append_bytes(a1612, a1613))
    } else {
        f90(v1)
    }
}

pub fn f235(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f90(v1)?;
    let v4: u64 = {
        let a1614 = v3.clone();
        length_bytes(a1614)
    };
    let v5: Bytes = {
        let a1615 = v2;
        let a1616 = v4;
        if nat_lt(a1615, a1616) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1617 = v4;
                let a1618 = v2;
                nat_sub(a1617, a1618)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1619 = v4;
                let a1620 = v2;
                nat_sub(a1619, a1620)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f225({
                let a1621 = v2;
                let a1622 = v4;
                nat_sub(a1621, a1622)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1623 = f8(Str::lit("-"));
        let a1624 = v5;
        Ok(append_bytes(a1623, a1624))
    } else {
        Ok(v5)
    }
}

pub fn f236(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f302(v0)?;
    let v3: bool = {
        let a1625 = f303(v0)?;
        let a1626 = v1;
        nat_le(a1625, a1626)
    };
    let v4: u64 = {
        let a1630 = {
            let a1627 = v1;
            let a1628 = v2;
            let a1629 = 0u64;
            nat_quot(a1627, a1628, a1629)
        };
        let a1631 = f304(f305(v0))?;
        let a1632 = 0u64;
        nat_rem(a1630, a1631, a1632)
    };
    let v5: u64 = {
        let a1633 = v1;
        let a1634 = v2;
        let a1635 = 0u64;
        nat_rem(a1633, a1634, a1635)
    };
    let a1636 = v4;
    let a1637 = f306(v0)?;
    if nat_eq(a1636, a1637) {
        let a1638 = v5;
        let a1639 = 0u64;
        if nat_eq(a1638, a1639) {
            Ok(f307(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f307(v3, {
                let a1643 = {
                    let a1640 = v2;
                    let a1641 = 2u64;
                    let a1642 = 0u64;
                    nat_quot(a1640, a1641, a1642)
                };
                let a1644 = v5;
                if nat_le(a1643, a1644) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f106({
            let a1645 = v4;
            let a1646 = 0u64;
            nat_eq(a1645, a1646)
        }, {
            let a1647 = v5;
            let a1648 = 0u64;
            nat_eq(a1647, a1648)
        }) {
            Ok(f307(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1649 = v4;
                let a1650 = 0u64;
                if nat_eq(a1649, a1650) {
                    v5
                } else {
                    let a1651 = v2;
                    let a1652 = v5;
                    nat_add(a1651, a1652)?
                }
            };
            let v7: u64 = {
                let a1653 = v4;
                let a1654 = 0u64;
                if nat_eq(a1653, a1654) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1661 = {
                    let a1657 = {
                        let a1655 = v7;
                        let a1656 = 2000u64;
                        nat_add(a1655, a1656)?
                    };
                    let a1658 = f308(v0);
                    nat_sub(a1657, a1658)
                };
                let a1662 = {
                    let a1659 = f309(v0);
                    let a1660 = 1u64;
                    nat_sub(a1659, a1660)
                };
                nat_sub(a1661, a1662)
            };
            Ok(f307(v3, {
                let a1663 = 2000u64;
                let a1664 = v8;
                if nat_le(a1663, a1664) {
                    f310(v6, true, {
                        let a1665 = v8;
                        let a1666 = 2000u64;
                        nat_sub(a1665, a1666)
                    })?
                } else {
                    f310(v6, false, {
                        let a1667 = 2000u64;
                        let a1668 = v8;
                        nat_sub(a1667, a1668)
                    })?
                }
            }))
        }
    }
}

pub fn f237(v0: Bytes) -> R<Bytes> {
    let a1673 = f154(v0.clone(), 0u64, {
        let a1670 = {
            let a1669 = v0.clone();
            length_bytes(a1669)
        };
        let a1671 = 1u64;
        nat_add(a1670, a1671)?
    })?;
    let a1674 = {
        let a1672 = v0.clone();
        length_bytes(a1672)
    };
    if nat_eq(a1673, a1674) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1680 = {
            let a1678 = Bytes::lit(&[0x22]);
            let a1679 = f311(v0.clone(), 0u64, {
                let a1676 = {
                    let a1675 = v0;
                    length_bytes(a1675)
                };
                let a1677 = 1u64;
                nat_add(a1676, a1677)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1678, a1679)
        };
        let a1681 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1680, a1681))
    }
}

pub fn f238(v0: List<Bytes>) -> Bytes {
    let a1684 = {
        let a1682 = Bytes::lit(&[0x5b]);
        let a1683 = f47(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1682, a1683)
    };
    let a1685 = Bytes::lit(&[0x5d]);
    append_bytes(a1684, a1685)
}

pub fn f239(v0: Adt2) -> R<List<Bytes>> {
    let m1686 = v0;
    match m1686 {
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
        Adt2::C10(r1687, r1688) => {
            let v14 = (*r1687).clone();
            let v15 = (*r1688).clone();
            Ok(List::cons(f170(v14)?, f239(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f240(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f312(Fn8::F313, List::<(Bytes, Bytes)>::nil(), f314(v0)?);
    let v2: List<Bytes> = f315(Fn9::F316, List::<Bytes>::nil(), v1)?;
    let a1691 = {
        let a1689 = Bytes::lit(&[0x7b]);
        let a1690 = f47(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1689, a1690)
    };
    let a1692 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1691, a1692))
}

pub fn f241(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1693 = v0;
    match m1693 {
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
        Adt2::C11(v16, r1694, r1695) => {
            let v17 = (*r1694).clone();
            let v18 = (*r1695).clone();
            Ok(List::cons((v16, f170(v17)?), f241(v18)?))
        }
    }
}

pub fn f242(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1696 = v0;
    match m1696.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f317(v1)?, f242(v2)?))
        }
    }
}

pub fn f243(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1697 = v0;
    match m1697.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f243(v3, false))
            } else {
                f243(v3, true)
            }
        }
    }
}

pub fn f244(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f318(v0, v1)
}

pub fn f245(v0: List<Bytes>) -> List<Bytes> {
    f109(Fn2::F319, List::<Bytes>::nil(), v0)
}

pub fn f246(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f320(v0, v1)
}

pub fn f247(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f321(f322(f323(v0))?))
}

pub fn f248(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f126(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1700 = {
            let a1698 = v4;
            let a1699 = v1;
            nat_add(a1698, a1699)?
        };
        let a1701 = v3;
        if nat_lt(a1700, a1701) {
            Ok(Some({
                let a1702 = v4;
                let a1703 = 1u64;
                nat_add(a1702, a1703)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f249(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1704, _) = v1;
        h1704
    };
    let a1706 = {
        let a1705 = v0.clone();
        length_bytes(a1705)
    };
    let a1707 = v2;
    if nat_le(a1706, a1707) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1708 = f134(v0.clone(), v2)?;
        let a1709 = 13u64;
        if nat_eq(a1708, a1709) {
            Ok(Some(({
                let a1712 = f134(v0, {
                    let a1710 = v2;
                    let a1711 = 1u64;
                    nat_add(a1710, a1711)?
                })?;
                let a1713 = 10u64;
                if nat_eq(a1712, a1713) {
                    let a1714 = v2;
                    let a1715 = 2u64;
                    nat_add(a1714, a1715)?
                } else {
                    let a1716 = v2;
                    let a1717 = 1u64;
                    nat_add(a1716, a1717)?
                }
            }, {
                let a1719 = {
                    let (_, h1718) = v1;
                    h1718
                };
                let a1720 = f8(Str::lit("\u{a}"));
                append_bytes(a1719, a1720)
            })))
        } else {
            let v4: u64 = {
                let m1721 = f181(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1721 {
                    None => {
                        let a1722 = v0.clone();
                        length_bytes(a1722)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1726 = {
                    let (_, h1723) = v1;
                    h1723
                };
                let a1727 = f46(v0, v2, {
                    let a1724 = v4;
                    let a1725 = v2;
                    nat_sub(a1724, a1725)
                });
                append_bytes(a1726, a1727)
            })))
        }
    }
}

pub fn f250(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1728 = f324(v0, v1)?;
    match m1728 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f325({
                let (h1729, _) = v2;
                h1729
            })? {
                Ok(Some({
                    let a1731 = v1;
                    let a1732 = {
                        let (_, h1730) = v2;
                        h1730
                    };
                    nat_add(a1731, a1732)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f251(v0: u64, v1: Bytes) -> R<bool> {
    f326(v0, v1.clone(), 0u64, {
        let a1734 = {
            let a1733 = v1;
            length_bytes(a1733)
        };
        let a1735 = 3u64;
        let a1736 = 0u64;
        nat_quot(a1734, a1735, a1736)
    }, true)
}

pub fn f252() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f253(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f327(v1, v0, v2)
}

pub fn f254(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f328(v0, v1)
}

pub fn f255(v0: Bytes, p1739: List<Bytes>) -> bool {
    let m1738 = p1739;
    {
        let mut m1738 = m1738;
        loop {
            let step = {
                let v1 = m1738;
                let m1737 = v1;
                match m1737.uncons() {
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
                Ok(next) => { m1738 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f329(v0, v1, v2)
}

pub fn f257(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f258(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f133(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1742 = f134(v0.clone(), {
                let a1740 = v1;
                let a1741 = 2u64;
                nat_add(a1740, a1741)?
            })?;
            let a1743 = 120u64;
            nat_eq(a1742, a1743)
        };
        let v3: u64 = if v2 {
            let a1744 = v1;
            let a1745 = 3u64;
            nat_add(a1744, a1745)?
        } else {
            let a1746 = v1;
            let a1747 = 2u64;
            nat_add(a1746, a1747)?
        };
        let v4: (u64, u64) = f330(v0.clone(), v2, v3)?;
        let a1749 = {
            let (h1748, _) = v4;
            h1748
        };
        let a1750 = v3;
        if if nat_eq(a1749, a1750) {
            true
        } else {
            let a1752 = f134(v0, {
                let (h1751, _) = v4;
                h1751
            })?;
            let a1753 = 59u64;
            !nat_eq(a1752, a1753)
        } {
            Ok(f257(f130(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f325({
                let (_, h1754) = v4;
                h1754
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f331({
                    let (_, h1755) = v4;
                    h1755
                })?, {
                    let a1757 = {
                        let (h1756, _) = v4;
                        h1756
                    };
                    let a1758 = 1u64;
                    nat_add(a1757, a1758)?
                })))
            } else {
                Ok(f257(f130(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f133(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1759 = v1;
                let a1760 = 4u64;
                nat_add(a1759, a1760)?
            })))
        } else {
            if f133(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1761 = v1;
                    let a1762 = 4u64;
                    nat_add(a1761, a1762)?
                })))
            } else {
                if f133(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1763 = v1;
                        let a1764 = 5u64;
                        nat_add(a1763, a1764)?
                    })))
                } else {
                    if f133(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1765 = v1;
                            let a1766 = 6u64;
                            nat_add(a1765, a1766)?
                        })))
                    } else {
                        if f133(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1767 = v1;
                                let a1768 = 6u64;
                                nat_add(a1767, a1768)?
                            })))
                        } else {
                            Ok(f257(f130(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f259(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1769, _) = f178(Fn5::F332(v1, v2, v0.clone()), {
        let a1771 = {
            let a1770 = v0;
            length_bytes(a1770)
        };
        let a1772 = 1u64;
        nat_add(a1771, a1772)?
    }, v3)?;
    Ok(h1769)
}

pub fn f260(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1773, _) = v2;
        h1773
    };
    let a1774 = v3;
    let a1775 = 0u64;
    if nat_eq(a1774, a1775) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1776 = v3;
            let a1777 = v1;
            let a1778 = 0u64;
            nat_quot(a1776, a1777, a1778)
        }, {
            let a1783 = f191(v0, {
                let a1779 = v3;
                let a1780 = v1;
                let a1781 = 0u64;
                nat_rem(a1779, a1780, a1781)
            });
            let a1784 = {
                let (_, h1782) = v2;
                h1782
            };
            append_bytes(a1783, a1784)
        }))
    }
}

pub fn f261(p1797: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt24>, Bytes>> {
    let m1796 = p1797;
    {
        let mut m1796 = m1796;
        loop {
            let step = {
                let v0 = m1796;
                let m1785 = v0;
                match m1785 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt24>, Bytes>>>(Ok(Ok::<Option<Adt24>, Bytes>(None::<Adt24>)))
                    }
                    Adt13::C1(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt24>, Bytes>>>(Ok(Ok::<Option<Adt24>, Bytes>(None::<Adt24>)))
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, R<Result<Option<Adt24>, Bytes>>>(Ok(Ok::<Option<Adt24>, Bytes>(None::<Adt24>)))
                    }
                    Adt13::C3(r1794, r1795) => {
                        let v2 = (*r1794).clone();
                        let v3 = (*r1795).clone();
                        let m1786 = v2;
                        match m1786 {
                            Adt13::C0(v4, r1792, r1793) => {
                                let v5 = (*r1792).clone();
                                let v6 = (*r1793).clone();
                                let m1787 = f139(v5.clone(), v1.clone());
                                match m1787 {
                                    Ok(v8) => {
                                        let m1788 = f140(v8.clone(), v4)?;
                                        match m1788 {
                                            Ok(v10) => {
                                                if f255({
                                                    let (_, h1789) = v10.clone();
                                                    h1789
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f32({
                                                        let (h1790, _) = v10;
                                                        h1790
                                                    }, f199()) {
                                                        Err::<Adt13, R<Result<Option<Adt24>, Bytes>>>(Ok(Ok::<Option<Adt24>, Bytes>(Some(Adt24::C0({
                                                            let (_, h1791) = v10;
                                                            h1791
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt13, R<Result<Option<Adt24>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt13, R<Result<Option<Adt24>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt13, R<Result<Option<Adt24>, Bytes>>>(Ok(f333(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt13, R<Result<Option<Adt24>, Bytes>>>(Ok(f333(v7)))
                                    }
                                }
                            }
                            Adt13::C1(_, _, _) => {
                                Ok::<Adt13, R<Result<Option<Adt24>, Bytes>>>(v3)
                            }
                            Adt13::C2(_) => {
                                Ok::<Adt13, R<Result<Option<Adt24>, Bytes>>>(v3)
                            }
                            Adt13::C3(_, _) => {
                                Ok::<Adt13, R<Result<Option<Adt24>, Bytes>>>(v3)
                            }
                            Adt13::C4 => {
                                Ok::<Adt13, R<Result<Option<Adt24>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt13::C4 => {
                        Err::<Adt13, R<Result<Option<Adt24>, Bytes>>>(Ok(Ok::<Option<Adt24>, Bytes>(None::<Adt24>)))
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

pub fn f262(v0: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(v0)
}

pub fn f263(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f264(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt17, Bytes>> {
    let m1798 = f140(v0, v2.clone())?;
    match m1798 {
        Ok(v4) => {
            if f32({
                let (h1799, _) = v4.clone();
                h1799
            }, f199()) {
                let m1801 = f334({
                    let (_, h1800) = v4;
                    h1800
                }, 16u64)?;
                match m1801 {
                    Ok(v6) => {
                        Ok(Ok::<Adt17, Bytes>(Adt17::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f262(v5))
                    }
                }
            } else {
                if f32({
                    let (h1802, _) = v4.clone();
                    h1802
                }, v1) {
                    let v7: Bytes = f118({
                        let (_, h1803) = v4;
                        h1803
                    })?;
                    Ok(Ok::<Adt17, Bytes>(Adt17::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f262(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1804, _) = v4;
                        h1804
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f262(v3))
        }
    }
}

pub fn f265(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f266(v0: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt2>, Bytes>> {
    let m1805 = v0;
    match m1805 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt13::C3(r1812, r1813) => {
            let v2 = (*r1812).clone();
            let v3 = (*r1813).clone();
            let m1806 = f266(v3, v1.clone())?;
            match m1806 {
                Ok(v5) => {
                    let m1807 = v2;
                    match m1807 {
                        Adt13::C0(v6, r1811, _) => {
                            let v7 = (*r1811).clone();
                            let m1808 = f139(v7.clone(), v1);
                            match m1808 {
                                Ok(v10) => {
                                    let m1809 = f140(v10, v6)?;
                                    match m1809 {
                                        Ok(v12) => {
                                            if f141(v12, Str::lit("enumeration")) {
                                                let m1810 = f142(v7, f8(Str::lit("value")));
                                                match m1810 {
                                                    None => {
                                                        Ok(f335(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f335(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f335(v9))
                                }
                            }
                        }
                        Adt13::C1(_, _, _) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt13::C2(_) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt13::C3(_, _) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt13::C4 => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f335(v4))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f267(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m1814 = v0;
    match m1814.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f336(v2.clone())?;
            if f115(v4.clone(), v1.clone()) {
                f267(v3, v1)
            } else {
                Ok(List::cons(v2, f267(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f268(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1815 = v1;
    match m1815.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1816 = f337(v0, v3.clone())?;
            match m1816 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f336(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f115(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1817 = f268(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1817 {
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

pub fn f269(p1821: List<Bytes>) -> R<bool> {
    let m1820 = p1821;
    {
        let mut m1820 = m1820;
        loop {
            let step = {
                let v0 = m1820;
                let m1818 = v0;
                match m1818.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1819 = f338(v1)?;
                        match m1819 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f115(v3, f339(false, v2.clone())?) {
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
                Ok(next) => { m1820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f148(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f271(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f53(v0.clone(), v1) {
        let m1822 = f44(v0, 34u8)?;
        match m1822.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1823 = v3;
                match m1823.uncons() {
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

pub fn f272(v0: List<Adt15>) -> R<Result<List<Adt19>, Bytes>> {
    let m1824 = v0;
    match m1824.uncons() {
        None => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Some((v1, v2)) => {
            let m1827 = f214(match v1.clone() {
                Adt15::C0(_, _, _, _, h1825, _) => {
                    h1825
                }
            }, match v1.clone() {
                Adt15::C0(_, _, _, h1826, _, _) => {
                    h1826
                }
            })?;
            match m1827 {
                Ok(v4) => {
                    let m1828 = f272(v2)?;
                    match m1828 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt19>, Bytes>(f340(f341(v4, match v1 {
                                Adt15::C0(_, _, h1829, _, _, _) => {
                                    h1829
                                }
                            }), v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt19>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt19>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f273(v0: List<Adt15>) -> List<Bytes> {
    let m1830 = v0;
    match m1830.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt15::C0(_, _, h1831, _, _, _) => {
                    h1831
                }
            }, f273(v2))
        }
    }
}

pub fn f274(v0: u64, v1: Adt18, v2: List<Adt19>) -> R<Result<List<Adt21>, Bytes>> {
    let m1832 = v2;
    match m1832.uncons() {
        None => {
            Ok(Ok::<List<Adt21>, Bytes>(List::<Adt21>::nil()))
        }
        Some((v3, v4)) => {
            let m1833 = f342(v0, v1.clone(), v3)?;
            match m1833 {
                Ok(v6) => {
                    let m1834 = f274(v0, v1, v4)?;
                    match m1834 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt21>, Bytes>(f343(v6, v8)))
                        }
                        Err(v7) => {
                            Ok(Err::<List<Adt21>, Bytes>(v7))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<Adt21>, Bytes>(v5))
                }
            }
        }
    }
}

pub fn f275(v0: List<Adt15>) -> R<u64> {
    let m1835 = v0;
    match m1835.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a1837 = match v1 {
                Adt15::C0(_, _, _, _, _, h1836) => {
                    h1836
                }
            };
            let a1838 = f275(v2)?;
            nat_add(a1837, a1838)
        }
    }
}

pub fn f276(p1846: List<Adt21>, p1847: List<Adt21>) -> Result<List<Adt21>, Bytes> {
    let m1845 = (p1846, p1847);
    {
        let mut m1845 = m1845;
        loop {
            let step = {
                let (v0, v1) = m1845;
                let m1839 = v0;
                match m1839.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), Result<List<Adt21>, Bytes>>(Ok::<List<Adt21>, Bytes>(f344(v1, List::<Adt21>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1841 = f345(v1.clone(), match v2.clone() {
                            Adt21::C0(h1840, _, _, _) => {
                                h1840
                            }
                        });
                        match m1841 {
                            None => {
                                Ok::<(List<Adt21>, List<Adt21>), Result<List<Adt21>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f32(match v4 {
                                    Adt21::C0(_, _, h1842, _) => {
                                        h1842
                                    }
                                }, match v2.clone() {
                                    Adt21::C0(_, _, h1843, _) => {
                                        h1843
                                    }
                                }) {
                                    Ok::<(List<Adt21>, List<Adt21>), Result<List<Adt21>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt21>, List<Adt21>), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f210(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt21::C0(h1844, _, _, _) => {
                                            h1844
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
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

pub fn f277(v0: List<Adt21>) -> List<Adt21> {
    if if f115(f346(), f347(v0.clone())) {
        let m1848 = f345(v0.clone(), f346());
        match m1848 {
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
        List::cons(Adt21::C0(f346(), List::<Bytes>::nil(), f270(f346(), f348()), f348()), v0)
    } else {
        v0
    }
}

pub fn f278(v0: List<Adt21>) -> List<Bytes> {
    let m1849 = v0;
    match m1849.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt21::C0(h1850, _, _, _) => {
                    h1850
                }
            }, f278(v2))
        }
    }
}

pub fn f279(p1856: List<Adt21>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m1855 = p1856;
    {
        let mut m1855 = m1855;
        loop {
            let step = {
                let v0 = m1855;
                let m1851 = v0;
                match m1851.uncons() {
                    None => {
                        Err::<List<Adt21>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m1853 = f202(match v2.clone() {
                            Adt21::C0(_, h1852, _, _) => {
                                h1852
                            }
                        }, v1.clone());
                        match m1853 {
                            None => {
                                Ok::<List<Adt21>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt21>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt21::C0(h1854, _, _, _) => {
                                        h1854
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1855 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: List<Adt21>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m1857 = v1;
    match m1857.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f349(v2, v0.clone(), f350(v0.clone(), v3), List::<Bytes>::nil())), f280(v0, v4, v2))
        }
    }
}

pub fn f281(v0: List<Adt21>) -> R<u64> {
    let m1858 = v0;
    match m1858.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a1863 = {
                let a1861 = {
                    let a1860 = match v1 {
                        Adt21::C0(_, h1859, _, _) => {
                            h1859
                        }
                    };
                    length_list(a1860)
                };
                let a1862 = 1u64;
                nat_add(a1861, a1862)?
            };
            let a1864 = f281(v2)?;
            nat_add(a1863, a1864)
        }
    }
}

pub fn f282(p1874: u64, v1: List<Adt21>, p1875: List<(List<Bytes>, List<Bytes>)>, p1876: List<Bytes>, p1877: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1873 = (p1874, (p1875, (p1876, p1877)));
    {
        let mut m1873 = m1873;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m1873;
                let m1865 = v0;
                if m1865 == 0 {
                    let m1866 = v2;
                    match m1866.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m1865 - 1;
                    let m1867 = v2.clone();
                    match m1867.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m1868 = f351(v2, v3.clone());
                            match m1868 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m1870 = f352(v1.clone(), {
                                        let (h1869, _) = v10.clone();
                                        h1869
                                    });
                                    match m1870 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h1871) = v10;
                                                h1871
                                            }, (f353({
                                                let (h1872, _) = v10;
                                                h1872
                                            }, v3), f353(v4, v12)))))
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
                Ok(next) => { m1873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: List<Adt21>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m1878 = v1;
    match m1878.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f354(v0.clone(), v2.clone(), v2)), f283(v0, v3))
        }
    }
}

pub fn f284(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m1879 = v2;
    match m1879.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f115(v4.clone(), v3.clone()) {
                f284(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f355(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f284(v0, v1, v5, f353(v6, v3)))
            }
        }
    }
}

pub fn f285(v0: bool) -> bool {
    !v0
}

pub fn f286(v0: Adt20, v1: Str) -> bool {
    f32(match v0 {
        Adt20::C0(_, h1880, _, _, _) => {
            h1880
        }
    }, f8(v1))
}

pub fn f287(v0: Adt20, v1: Str) -> Option<Bytes> {
    f142(match v0 {
        Adt20::C0(_, _, h1881, _, _) => {
            h1881
        }
    }, f8(v1))
}

pub fn f288(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1882 = f181(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m1882 {
        None => {
            let v3: Bytes = f210(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f53(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m1883 = f181(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m1883 {
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
                let m1884 = v0;
                match m1884 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m1886 = f356(v5.clone(), 0u64, {
                            let a1885 = v5.clone();
                            length_bytes(a1885)
                        }, None::<u64>)?;
                        match m1886 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a1889 = f46(v5, 0u64, {
                                        let a1887 = v6;
                                        let a1888 = 1u64;
                                        nat_add(a1887, a1888)?
                                    });
                                    let a1890 = v1;
                                    append_bytes(a1889, a1890)
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

pub fn f289(p1895: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1894 = p1895;
    {
        let mut m1894 = m1894;
        loop {
            let step = {
                let v0 = m1894;
                let m1891 = v0;
                match m1891.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1892, _) = v2;
                            h1892
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1893) = v2;
                                h1893
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
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

pub fn f290(v0: Adt20, v1: Str, v2: Str) -> Bytes {
    let m1896 = f287(v0, v1);
    match m1896 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f291(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1899 = f46(v0.clone(), {
        let a1897 = v1;
        let a1898 = 2u64;
        nat_add(a1897, a1898)?
    }, 2u64);
    let a1900 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1899, a1900) {
        let m1903 = f357(f46(v0, {
            let a1901 = v1;
            let a1902 = 4u64;
            nat_add(a1901, a1902)?
        }, 2u64))?;
        match m1903 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f104(v2.clone(), 0u64)?;
                let a1904 = v3;
                let a1905 = 32u64;
                if nat_lt(a1904, a1905) {
                    if f358(v3) {
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

pub fn f292(v0: u64) -> Bytes {
    let a1906 = v0;
    let a1907 = 34u64;
    if nat_eq(a1906, a1907) {
        Bytes::lit(&[0x22])
    } else {
        let a1908 = v0;
        let a1909 = 92u64;
        if nat_eq(a1908, a1909) {
            Bytes::lit(&[0x5c])
        } else {
            let a1910 = v0;
            let a1911 = 98u64;
            if nat_eq(a1910, a1911) {
                Bytes::lit(&[0x08])
            } else {
                let a1912 = v0;
                let a1913 = 102u64;
                if nat_eq(a1912, a1913) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1914 = v0;
                    let a1915 = 110u64;
                    if nat_eq(a1914, a1915) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1916 = v0;
                        let a1917 = 114u64;
                        if nat_eq(a1916, a1917) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1918 = v0;
                            let a1919 = 116u64;
                            if nat_eq(a1918, a1919) {
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

pub fn f293(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f104(v0, v1)?;
    let a1920 = 47u64;
    let a1921 = v2;
    if if nat_lt(a1920, a1921) {
        let a1922 = v2;
        let a1923 = 58u64;
        nat_lt(a1922, a1923)
    } else {
        false
    } {
        Ok(Some({
            let a1924 = v1;
            let a1925 = 1u64;
            nat_add(a1924, a1925)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f294(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1926 = f104(v0, v1)?;
    let a1927 = 48u64;
    if nat_eq(a1926, a1927) {
        Ok(Some({
            let a1928 = v1;
            let a1929 = 1u64;
            nat_add(a1928, a1929)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f295(v0: Bytes) -> R<u64> {
    let (h1930, _) = f178(Fn5::F359(v0.clone()), {
        let a1931 = v0;
        length_bytes(a1931)
    }, 0u64)?;
    Ok(h1930)
}

pub fn f296(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1935 = {
        let a1933 = v0;
        let a1934 = {
            let (h1932, _) = v1;
            h1932
        };
        index_bytes(a1933, a1934)
    };
    match m1935 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1937 = {
                    let (h1936, _) = v1;
                    h1936
                };
                let a1938 = 1u64;
                nat_add(a1937, a1938)?
            }, {
                let a1942 = {
                    let a1940 = {
                        let (_, h1939) = v1;
                        h1939
                    };
                    let a1941 = 10u64;
                    nat_mul(a1940, a1941)?
                };
                let a1943 = f360(v2)?;
                nat_add(a1942, a1943)?
            })))
        }
    }
}

pub fn f297(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f361(v0, v1)
}

pub fn f298(v0: Fn7, p1947: u64, p1948: Adt23) -> R<Adt23> {
    let m1946 = (p1947, p1948);
    {
        let mut m1946 = m1946;
        loop {
            let step = {
                let (v1, v2) = m1946;
                let m1944 = v1;
                if m1944 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1944 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1945: Fn7 = v0.clone();
                        c1945.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1946 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f299(v0: Adt23) -> R<Adt23> {
    f362(v0)
}

pub fn f300(v0: u32, v1: u32) -> u32 {
    let m1951 = {
        let a1949 = f363(v0);
        let a1950 = f363(v1);
        fixed_u64::checked_add(a1949, a1950)
    };
    match m1951 {
        None => {
            0u32
        }
        Some(v2) => {
            f364(v2)
        }
    }
}

pub fn f301(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1954 = {
        let a1952 = v3;
        let a1953 = 8u64;
        nat_add(a1952, a1953)?
    };
    let a1955 = v2;
    if nat_eq(a1954, a1955) {
        Ok(f365({
            let a1958 = {
                let a1956 = v1;
                let a1957 = 8u64;
                nat_mul(a1956, a1957)?
            };
            let a1959 = 4294967296u64;
            let a1960 = 0u64;
            nat_quot(a1958, a1959, a1960)
        }))
    } else {
        let a1963 = {
            let a1961 = v3;
            let a1962 = 4u64;
            nat_add(a1961, a1962)?
        };
        let a1964 = v2;
        if nat_eq(a1963, a1964) {
            Ok(f365({
                let a1967 = {
                    let a1965 = v1;
                    let a1966 = 8u64;
                    nat_mul(a1965, a1966)?
                };
                let a1968 = 4294967296u64;
                let a1969 = 0u64;
                nat_rem(a1967, a1968, a1969)
            }))
        } else {
            Ok(f366(f367(v0.clone(), v1, v3), f367(v0.clone(), v1, {
                let a1970 = v3;
                let a1971 = 1u64;
                nat_add(a1970, a1971)?
            }), f367(v0.clone(), v1, {
                let a1972 = v3;
                let a1973 = 2u64;
                nat_add(a1972, a1973)?
            }), f367(v0, v1, {
                let a1974 = v3;
                let a1975 = 3u64;
                nat_add(a1974, a1975)?
            })))
        }
    }
}

pub fn f302(v0: bool) -> R<u64> {
    f304({
        let a1976 = f309(v0);
        let a1977 = 1u64;
        nat_sub(a1976, a1977)
    })
}

pub fn f303(v0: bool) -> R<u64> {
    f304(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f304(v0: u64) -> R<u64> {
    let m1978 = v0;
    if m1978 == 0 {
        Ok(1u64)
    } else {
        let v1 = m1978 - 1;
        let a1979 = 2u64;
        let a1980 = f304(v1)?;
        nat_mul(a1979, a1980)
    }
}

pub fn f305(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f306(v0: bool) -> R<u64> {
    let a1981 = f304(f305(v0))?;
    let a1982 = 1u64;
    Ok(nat_sub(a1981, a1982))
}

pub fn f307(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a1983 = Bytes::lit(&[0x2d]);
        let a1984 = v1;
        append_bytes(a1983, a1984)
    } else {
        v1
    }
}

pub fn f308(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f309(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f310(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f368(f369(v2, f370(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f368(f369(v2, f370(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a1985 = v3.clone();
            length_bytes(a1985)
        };
        let v5: Bytes = {
            let a1986 = v2;
            let a1987 = v4;
            if nat_lt(a1986, a1987) {
                f46(v3.clone(), 0u64, {
                    let a1988 = v4;
                    let a1989 = v2;
                    nat_sub(a1988, a1989)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f371({
            let a1990 = v2;
            let a1991 = v4;
            if nat_lt(a1990, a1991) {
                f46(v3, {
                    let a1992 = v4;
                    let a1993 = v2;
                    nat_sub(a1992, a1993)
                }, v2)
            } else {
                let a1996 = f372({
                    let a1994 = v2;
                    let a1995 = v4;
                    nat_sub(a1994, a1995)
                });
                let a1997 = v3;
                append_bytes(a1996, a1997)
            }
        }, {
            let a1998 = v2;
            let a1999 = 1u64;
            nat_add(a1998, a1999)?
        })?;
        let a2001 = {
            let a2000 = v6.clone();
            length_bytes(a2000)
        };
        let a2002 = 0u64;
        if nat_eq(a2001, a2002) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f311(v0: Bytes, p2021: u64, p2022: u64, p2023: Bytes) -> R<Bytes> {
    let m2020 = (p2021, (p2022, p2023));
    {
        let mut m2020 = m2020;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2020;
                let m2003 = v2;
                if m2003 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2003 - 1;
                    let v5: u64 = f154(v0.clone(), v1, {
                        let a2007 = {
                            let a2005 = {
                                let a2004 = v0.clone();
                                length_bytes(a2004)
                            };
                            let a2006 = v1;
                            nat_sub(a2005, a2006)
                        };
                        let a2008 = 1u64;
                        nat_add(a2007, a2008)?
                    })?;
                    let v6: Bytes = {
                        let a2011 = v3;
                        let a2012 = f46(v0.clone(), v1, {
                            let a2009 = v5;
                            let a2010 = v1;
                            nat_sub(a2009, a2010)
                        });
                        append_bytes(a2011, a2012)
                    };
                    let m2015 = {
                        let a2013 = v0.clone();
                        let a2014 = v5;
                        index_bytes(a2013, a2014)
                    };
                    match m2015 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2016 = v5;
                                let a2017 = 1u64;
                                nat_add(a2016, a2017)?
                            }, (v4, {
                                let a2018 = v6;
                                let a2019 = f373(v0.clone(), v5, v7)?;
                                append_bytes(a2018, a2019)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Fn8, p2027: List<(Bytes, Bytes)>, p2028: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2026 = (p2027, p2028);
    {
        let mut m2026 = m2026;
        loop {
            let step = {
                let (v1, v2) = m2026;
                let m2024 = v2;
                match m2024.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2025: Fn8 = v0.clone();
                            c2025.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f374(v0, v1)
}

pub fn f314(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f375(v0)
}

pub fn f315(v0: Fn9, p2032: List<Bytes>, p2033: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2031 = (p2032, p2033);
    {
        let mut m2031 = m2031;
        loop {
            let step = {
                let (v1, v2) = m2031;
                let m2029 = v2;
                match m2029.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2030: Fn9 = v0.clone();
                            c2030.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2031 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f376(v1)?, v0))
}

pub fn f317(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f377(v0.clone(), 0u64, {
        let a2034 = v0;
        length_bytes(a2034)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f318(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2035, _) = v1.clone();
        h2035
    };
    let v3: u64 = {
        let (h2036, _) = {
            let (_, h2037) = v1.clone();
            h2037
        };
        h2036
    };
    let v4: List<Bytes> = {
        let (_, h2038) = {
            let (_, h2039) = v1;
            h2039
        };
        h2038
    };
    let m2040 = f162(v0.clone(), v2);
    match m2040 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2041 = v5;
            let a2042 = 10u8;
            if equal(a2041, a2042) {
                Ok(Some(({
                    let a2043 = v2;
                    let a2044 = 1u64;
                    nat_add(a2043, a2044)?
                }, ({
                    let a2045 = v2;
                    let a2046 = 1u64;
                    nat_add(a2045, a2046)?
                }, List::cons(f378(f46(v0, v3, {
                    let a2047 = v2;
                    let a2048 = v3;
                    nat_sub(a2047, a2048)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2049 = v2;
                    let a2050 = 1u64;
                    nat_add(a2049, a2050)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f319(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f320(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m2051 = v0.clone();
    match m2051.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a2052 = v2;
            let a2053 = v1.clone();
            if equal(a2052, a2053) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f321(v0: List<(Bytes, ())>) -> List<Bytes> {
    f245(f379(Fn10::F380, List::<Bytes>::nil(), v0))
}

pub fn f322(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f381(f382(Fn11::F383, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m2058 = {
        let (h2054, _) = f384(Fn12::F385, {
            let a2056 = {
                let a2055 = v0;
                length_list(a2055)
            };
            let a2057 = 1u64;
            nat_add(a2056, a2057)?
        }, v1)?;
        h2054
    };
    match m2058.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f323(v0: List<Bytes>) -> List<(Bytes, ())> {
    f386(f387(Fn13::F388, List::<(Bytes, ())>::nil(), v0))
}

pub fn f324(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2061 = {
        let a2059 = v0.clone();
        let a2060 = v1;
        index_bytes(a2059, a2060)
    };
    match m2061 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f59(v2)?;
            let a2062 = v3;
            let a2063 = 128u64;
            if nat_lt(a2062, a2063) {
                Ok(Some((v3, 1u64)))
            } else {
                if f106({
                    let a2064 = 194u64;
                    let a2065 = v3;
                    nat_le(a2064, a2065)
                }, {
                    let a2066 = v3;
                    let a2067 = 223u64;
                    nat_le(a2066, a2067)
                }) {
                    let m2070 = f389(v0, {
                        let a2068 = v1;
                        let a2069 = 1u64;
                        nat_add(a2068, a2069)?
                    })?;
                    match m2070 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f390({
                                let a2075 = {
                                    let a2073 = {
                                        let a2071 = v3;
                                        let a2072 = 192u64;
                                        nat_sub(a2071, a2072)
                                    };
                                    let a2074 = 64u64;
                                    nat_mul(a2073, a2074)?
                                };
                                let a2076 = v4;
                                nat_add(a2075, a2076)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f106({
                        let a2077 = 224u64;
                        let a2078 = v3;
                        nat_le(a2077, a2078)
                    }, {
                        let a2079 = v3;
                        let a2080 = 239u64;
                        nat_le(a2079, a2080)
                    }) {
                        let m2083 = f389(v0.clone(), {
                            let a2081 = v1;
                            let a2082 = 1u64;
                            nat_add(a2081, a2082)?
                        })?;
                        match m2083 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2086 = f389(v0, {
                                    let a2084 = v1;
                                    let a2085 = 2u64;
                                    nat_add(a2084, a2085)?
                                })?;
                                match m2086 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f390({
                                            let a2095 = {
                                                let a2089 = {
                                                    let a2087 = v3;
                                                    let a2088 = 224u64;
                                                    nat_sub(a2087, a2088)
                                                };
                                                let a2090 = 4096u64;
                                                nat_mul(a2089, a2090)?
                                            };
                                            let a2096 = {
                                                let a2093 = {
                                                    let a2091 = v5;
                                                    let a2092 = 64u64;
                                                    nat_mul(a2091, a2092)?
                                                };
                                                let a2094 = v6;
                                                nat_add(a2093, a2094)?
                                            };
                                            nat_add(a2095, a2096)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f106({
                            let a2097 = 240u64;
                            let a2098 = v3;
                            nat_le(a2097, a2098)
                        }, {
                            let a2099 = v3;
                            let a2100 = 244u64;
                            nat_le(a2099, a2100)
                        }) {
                            let m2103 = f389(v0.clone(), {
                                let a2101 = v1;
                                let a2102 = 1u64;
                                nat_add(a2101, a2102)?
                            })?;
                            match m2103 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2106 = f389(v0.clone(), {
                                        let a2104 = v1;
                                        let a2105 = 2u64;
                                        nat_add(a2104, a2105)?
                                    })?;
                                    match m2106 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2109 = f389(v0, {
                                                let a2107 = v1;
                                                let a2108 = 3u64;
                                                nat_add(a2107, a2108)?
                                            })?;
                                            match m2109 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f390({
                                                        let a2122 = {
                                                            let a2112 = {
                                                                let a2110 = v3;
                                                                let a2111 = 240u64;
                                                                nat_sub(a2110, a2111)
                                                            };
                                                            let a2113 = 262144u64;
                                                            nat_mul(a2112, a2113)?
                                                        };
                                                        let a2123 = {
                                                            let a2120 = {
                                                                let a2114 = v7;
                                                                let a2115 = 4096u64;
                                                                nat_mul(a2114, a2115)?
                                                            };
                                                            let a2121 = {
                                                                let a2118 = {
                                                                    let a2116 = v8;
                                                                    let a2117 = 64u64;
                                                                    nat_mul(a2116, a2117)?
                                                                };
                                                                let a2119 = v9;
                                                                nat_add(a2118, a2119)?
                                                            };
                                                            nat_add(a2120, a2121)?
                                                        };
                                                        nat_add(a2122, a2123)?
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

pub fn f325(v0: u64) -> R<bool> {
    f251(v0, f391())
}

pub fn f326(v0: u64, v1: Bytes, p2134: u64, p2135: u64, p2136: bool) -> R<bool> {
    let m2133 = (p2134, (p2135, p2136));
    {
        let mut m2133 = m2133;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2133;
                let m2124 = v3;
                if m2124 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2124 - 1;
                    let v6: u64 = f392(v1.clone(), v2)?;
                    if v4 {
                        let a2125 = v0;
                        let a2126 = v6;
                        if nat_lt(a2125, a2126) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2127 = v2;
                                let a2128 = 1u64;
                                nat_add(a2127, a2128)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2129 = v0;
                        let a2130 = v6;
                        if nat_le(a2129, a2130) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2131 = v2;
                                let a2132 = 1u64;
                                nat_add(a2131, a2132)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f126(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2138 = v2;
        let a2139 = {
            let a2137 = v0;
            length_bytes(a2137)
        };
        if nat_lt(a2138, a2139) {
            Ok(Some({
                let a2140 = v2;
                let a2141 = 1u64;
                nat_add(a2140, a2141)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f328(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f180(f134(v0, v1)?)? {
        Ok(Some({
            let a2142 = v1;
            let a2143 = 1u64;
            nat_add(a2142, a2143)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f329(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2144 = f324(v0, v2)?;
    match m2144 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2145 = v2;
            let a2146 = v1;
            if if nat_eq(a2145, a2146) {
                f393({
                    let (h2147, _) = v3;
                    h2147
                })?
            } else {
                f394({
                    let (h2148, _) = v3;
                    h2148
                })?
            } {
                Ok(Some({
                    let a2150 = v2;
                    let a2151 = {
                        let (_, h2149) = v3;
                        h2149
                    };
                    nat_add(a2150, a2151)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f330(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2152, _) = f222(Fn6::F395(v1, v0.clone()), {
        let a2154 = {
            let a2153 = v0;
            length_bytes(a2153)
        };
        let a2155 = 1u64;
        nat_add(a2154, a2155)?
    }, (v2, 0u64))?;
    Ok(h2152)
}

pub fn f331(v0: u64) -> R<Bytes> {
    let a2156 = v0;
    let a2157 = 128u64;
    if nat_lt(a2156, a2157) {
        Ok(f396(f397(), v0))
    } else {
        let a2158 = v0;
        let a2159 = 2048u64;
        if nat_lt(a2158, a2159) {
            let a2167 = f396(f398(), {
                let a2165 = {
                    let a2163 = {
                        let a2160 = v0;
                        let a2161 = 64u64;
                        let a2162 = 0u64;
                        nat_quot(a2160, a2161, a2162)
                    };
                    let a2164 = 2u64;
                    nat_sub(a2163, a2164)
                };
                let a2166 = 2u64;
                nat_mul(a2165, a2166)?
            });
            let a2168 = f399(v0)?;
            Ok(append_bytes(a2167, a2168))
        } else {
            let a2169 = v0;
            let a2170 = 65536u64;
            if nat_lt(a2169, a2170) {
                let a2181 = f396(f400(), {
                    let a2174 = {
                        let a2171 = v0;
                        let a2172 = 4096u64;
                        let a2173 = 0u64;
                        nat_quot(a2171, a2172, a2173)
                    };
                    let a2175 = 3u64;
                    nat_mul(a2174, a2175)?
                });
                let a2182 = {
                    let a2179 = f399({
                        let a2176 = v0;
                        let a2177 = 64u64;
                        let a2178 = 0u64;
                        nat_quot(a2176, a2177, a2178)
                    })?;
                    let a2180 = f399(v0)?;
                    append_bytes(a2179, a2180)
                };
                Ok(append_bytes(a2181, a2182))
            } else {
                let a2196 = f396(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2183 = v0;
                    let a2184 = 262144u64;
                    let a2185 = 0u64;
                    nat_quot(a2183, a2184, a2185)
                });
                let a2197 = {
                    let a2194 = f399({
                        let a2186 = v0;
                        let a2187 = 4096u64;
                        let a2188 = 0u64;
                        nat_quot(a2186, a2187, a2188)
                    })?;
                    let a2195 = {
                        let a2192 = f399({
                            let a2189 = v0;
                            let a2190 = 64u64;
                            let a2191 = 0u64;
                            nat_quot(a2189, a2190, a2191)
                        })?;
                        let a2193 = f399(v0)?;
                        append_bytes(a2192, a2193)
                    };
                    append_bytes(a2194, a2195)
                };
                Ok(append_bytes(a2196, a2197))
            }
        }
    }
}

pub fn f332(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f401(v2, v0, v1, v3)
}

pub fn f333(v0: Bytes) -> Result<Option<Adt24>, Bytes> {
    Err::<Option<Adt24>, Bytes>(v0)
}

pub fn f334(p2201: Bytes, p2202: u64) -> R<Result<Bytes, Bytes>> {
    let m2200 = (p2201, p2202);
    {
        let mut m2200 = m2200;
        loop {
            let step = {
                let (v0, v1) = m2200;
                let m2198 = v1;
                if m2198 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2198 - 1;
                    if f255(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f32(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f32(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f32(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f403())))
                                } else {
                                    if f32(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f255(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f255(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2199 = f404(v0.clone())?;
                                                match m2199 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m2200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f336(v0: Adt2) -> R<Bytes> {
    f170(v0)
}

pub fn f337(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2203 = v1;
    match m2203 {
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
                let a2214 = if f106(v9, {
                    let a2212 = 0u64;
                    let a2213 = v10;
                    nat_lt(a2212, a2213)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2215 = f234(false, v10)?;
                append_bytes(a2214, a2215)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m2206 = f406({
                let a2204 = f8(Str::lit("n"));
                let a2205 = f235(false, v4, v5)?;
                append_bytes(a2204, a2205)
            })?;
            match m2206 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2210 = f8(Str::lit("nMinus"));
                        let a2211 = f46(v6.clone(), 1u64, {
                            let a2208 = {
                                let a2207 = v6;
                                length_bytes(a2207)
                            };
                            let a2209 = 1u64;
                            nat_sub(a2208, a2209)
                        });
                        append_bytes(a2210, a2211)
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
            f405(v0, v2)
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

pub fn f338(v0: Bytes) -> R<Option<Bytes>> {
    let m2216 = f407(v0)?;
    match m2216 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f408(v1)?))
        }
    }
}

pub fn f339(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2217 = v1;
    match m2217.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2218 = f405(v0, v2)?;
            match m2218 {
                None => {
                    f339(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f339(v0, v3)?))
                }
            }
        }
    }
}

pub fn f340(v0: List<Adt19>, v1: List<Adt19>) -> List<Adt19> {
    let m2219 = v0;
    match m2219.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f340(v3, v1))
        }
    }
}

pub fn f341(v0: List<Adt20>, v1: Bytes) -> List<Adt19> {
    let m2220 = v0;
    match m2220.uncons() {
        None => {
            List::<Adt19>::nil()
        }
        Some((v2, v3)) => {
            if f115(match v2.clone() {
                Adt20::C0(_, h2221, _, _, _) => {
                    h2221
                }
            }, f409()) {
                let m2222 = f287(v2.clone(), Str::lit("name"));
                match m2222 {
                    None => {
                        f341(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt19::C0(match v2.clone() {
                            Adt20::C0(_, h2223, _, _, _) => {
                                h2223
                            }
                        }, v1.clone(), v4, v2), f341(v3, v1))
                    }
                }
            } else {
                f341(v3, v1)
            }
        }
    }
}

pub fn f342(v0: u64, v1: Adt18, v2: Adt19) -> R<Result<List<Adt21>, Bytes>> {
    let v3: Bytes = f118(match v2.clone() {
        Adt19::C0(_, _, h2224, _) => {
            h2224
        }
    })?;
    let v4: Adt20 = match v2.clone() {
        Adt19::C0(_, _, _, h2225) => {
            h2225
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt19::C0(_, h2226, _, _) => {
            h2226
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt19::C0(h2227, _, _, _) => {
            h2227
        }
    };
    if f32(v6.clone(), f8(Str::lit("complexType"))) {
        f410(v0, v1, v2)
    } else {
        if f32(v6.clone(), f8(Str::lit("simpleType"))) {
            let m2228 = f411(v3, v4, v5)?;
            match m2228 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt21>, Bytes>(List::cons(v8, List::<Adt21>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt21>, Bytes>(v7))
                }
            }
        } else {
            if f32(v6.clone(), f8(Str::lit("element"))) {
                let m2229 = f287(v4.clone(), Str::lit("type"));
                match m2229 {
                    None => {
                        let m2230 = f412(v0, v1, v3, v5, v4)?;
                        match m2230 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt21>, Bytes>(match v11 {
                                    Adt25::C0(_, h2231) => {
                                        h2231
                                    }
                                }))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt21>, Bytes>(v10))
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Ok::<List<Adt21>, Bytes>(List::<Adt21>::nil()))
                    }
                }
            } else {
                if f32(v6.clone(), f8(Str::lit("attribute"))) {
                    let m2232 = f287(v4.clone(), Str::lit("type"));
                    match m2232 {
                        None => {
                            let m2233 = f413(v1, v5, v3, v4)?;
                            match m2233 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt21>, Bytes>(match v14 {
                                        Adt25::C0(_, h2234) => {
                                            h2234
                                        }
                                    }))
                                }
                                Err(v13) => {
                                    Ok(Err::<List<Adt21>, Bytes>(v13))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<List<Adt21>, Bytes>(List::<Adt21>::nil()))
                        }
                    }
                } else {
                    if f32(v6, f8(Str::lit("group"))) {
                        let m2235 = f414(v4)?;
                        match m2235 {
                            Ok(v16) => {
                                let m2236 = f415(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m2236 {
                                    None => {
                                        Ok(Err::<List<Adt21>, Bytes>(f416(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f417(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt21>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m2237 = f414(v4)?;
                        match m2237 {
                            Ok(v19) => {
                                let m2238 = f418(v0, v1, v3, v5, v19, f419())?;
                                match m2238 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt21>, Bytes>(match v21 {
                                            Adt26::C0(_, h2239, _, _, _) => {
                                                h2239
                                            }
                                        }))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<List<Adt21>, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<List<Adt21>, Bytes>(v18))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f343(v0: List<Adt21>, v1: List<Adt21>) -> List<Adt21> {
    let m2240 = v0;
    match m2240.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f343(v3, v1))
        }
    }
}

pub fn f344(p2243: List<Adt21>, p2244: List<Adt21>) -> List<Adt21> {
    let m2242 = (p2243, p2244);
    {
        let mut m2242 = m2242;
        loop {
            let step = {
                let (v0, v1) = m2242;
                let m2241 = v0;
                match m2241.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(p2248: List<Adt21>, v1: Bytes) -> Option<Adt21> {
    let m2247 = p2248;
    {
        let mut m2247 = m2247;
        loop {
            let step = {
                let v0 = m2247;
                let m2245 = v0;
                match m2245.uncons() {
                    None => {
                        Err::<List<Adt21>, Option<Adt21>>(None::<Adt21>)
                    }
                    Some((v2, v3)) => {
                        if f32(match v2.clone() {
                            Adt21::C0(h2246, _, _, _) => {
                                h2246
                            }
                        }, v1.clone()) {
                            Err::<List<Adt21>, Option<Adt21>>(Some(v2))
                        } else {
                            Ok::<List<Adt21>, Option<Adt21>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f347(v0: List<Adt21>) -> List<Bytes> {
    let m2249 = v0;
    match m2249.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f353(match v1 {
                Adt21::C0(_, h2250, _, _) => {
                    h2250
                }
            }, f347(v2))
        }
    }
}

pub fn f348() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f349(p2254: u64, v1: List<Adt21>, p2255: List<Bytes>, p2256: List<Bytes>) -> List<Bytes> {
    let m2253 = (p2254, (p2255, p2256));
    {
        let mut m2253 = m2253;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2253;
                let m2251 = v0;
                if m2251 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2251 - 1;
                    let m2252 = v2;
                    match m2252.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f115(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f353(f350(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2253 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: List<Adt21>, v1: Bytes) -> List<Bytes> {
    let m2257 = f345(v0, v1);
    match m2257 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt21::C0(_, h2258, _, _) => {
                    h2258
                }
            }
        }
    }
}

pub fn f351(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2259 = v0;
    match m2259.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2261 = f202({
                let (_, h2260) = v2.clone();
                h2260
            }, v1.clone());
            match m2261 {
                None => {
                    Some(({
                        let (h2262, _) = v2;
                        h2262
                    }, v3))
                }
                Some(_) => {
                    let m2263 = f351(v3, v1);
                    match m2263 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2264, _) = v5;
                                h2264
                            }, List::cons(v2, {
                                let (_, h2265) = v5;
                                h2265
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f352(v0: List<Adt21>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2266 = v1.clone();
    match m2266.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2267 = v3;
            match m2267.uncons() {
                None => {
                    let m2270 = f345(v0, v2.clone());
                    match m2270 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f421(v2))
                        }
                        Some(v6) => {
                            if f115(v2.clone(), match v6.clone() {
                                Adt21::C0(_, h2271, _, _) => {
                                    h2271
                                }
                            }) {
                                let m2273 = match v6.clone() {
                                    Adt21::C0(_, _, _, h2272) => {
                                        h2272
                                    }
                                };
                                match m2273.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f421(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f270(v2, match v6 {
                                            Adt21::C0(_, _, _, h2274) => {
                                                h2274
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt21::C0(_, _, h2275, _) => {
                                        h2275
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f420(v0, v1, {
                        let a2268 = v2;
                        let a2269 = f8(Str::lit("Group"));
                        append_bytes(a2268, a2269)
                    })
                }
            }
        }
    }
}

pub fn f353(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2276 = v0;
    match m2276.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f353(v3, v1))
        }
    }
}

pub fn f354(v0: List<Adt21>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2277 = v1;
    match m2277.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f353(f422(f350(v0.clone(), v3), v2.clone()), f354(v0, v4, v2))
        }
    }
}

pub fn f355(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2278 = v1;
    match m2278.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f32(v3.clone(), v2.clone()) {
                true
            } else {
                if f115(v3.clone(), f423(v0.clone(), v2.clone())) {
                    f115(v2.clone(), f423(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f355(v0, v4, v2))
            } else {
                f355(v0, v4, v2)
            }
        }
    }
}

pub fn f356(v0: Bytes, p2287: u64, p2288: u64, p2289: Option<u64>) -> R<Option<u64>> {
    let m2286 = (p2287, (p2288, p2289));
    {
        let mut m2286 = m2286;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2286;
                let m2279 = v2;
                if m2279 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2279 - 1;
                    let v5: u64 = f134(v0.clone(), v1)?;
                    let a2280 = v5;
                    let a2281 = 256u64;
                    if nat_eq(a2280, a2281) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2282 = v1;
                            let a2283 = 1u64;
                            nat_add(a2282, a2283)?
                        }, (v4, {
                            let a2284 = v5;
                            let a2285 = 47u64;
                            if nat_eq(a2284, a2285) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2290 = v0.clone();
        length_bytes(a2290)
    };
    let a2294 = {
        let a2291 = v1;
        let a2292 = 2u64;
        let a2293 = 0u64;
        nat_rem(a2291, a2292, a2293)
    };
    let a2295 = 0u64;
    if nat_eq(a2294, a2295) {
        let (_, h2296) = {
            let (h2297, _) = f424(Fn14::F425(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2297
        };
        Ok(h2296)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f358(v0: u64) -> bool {
    let a2298 = v0;
    let a2299 = 8u64;
    if nat_eq(a2298, a2299) {
        true
    } else {
        let a2300 = v0;
        let a2301 = 9u64;
        if nat_eq(a2300, a2301) {
            true
        } else {
            let a2302 = v0;
            let a2303 = 10u64;
            if nat_eq(a2302, a2303) {
                true
            } else {
                let a2304 = v0;
                let a2305 = 12u64;
                if nat_eq(a2304, a2305) {
                    true
                } else {
                    let a2306 = v0;
                    let a2307 = 13u64;
                    nat_eq(a2306, a2307)
                }
            }
        }
    }
}

pub fn f359(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2310 = {
        let a2308 = v0;
        let a2309 = v1;
        index_bytes(a2308, a2309)
    };
    match m2310 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f426(v2)? {
                Ok(Some({
                    let a2311 = v1;
                    let a2312 = 1u64;
                    nat_add(a2311, a2312)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f360(v0: u8) -> R<u64> {
    let a2359 = {
        let v1: u8 = v0;
        let a2357 = {
            let a2333 = {
                let a2321 = {
                    let a2315 = {
                        let a2313 = v1;
                        let a2314 = 128u8;
                        fixed_u8::bit_and(a2313, a2314)
                    };
                    let a2316 = 128u8;
                    if equal(a2315, a2316) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2322 = {
                    let a2319 = {
                        let a2317 = v1;
                        let a2318 = 64u8;
                        fixed_u8::bit_and(a2317, a2318)
                    };
                    let a2320 = 64u8;
                    if equal(a2319, a2320) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2321, a2322)?
            };
            let a2334 = {
                let a2331 = {
                    let a2325 = {
                        let a2323 = v1;
                        let a2324 = 32u8;
                        fixed_u8::bit_and(a2323, a2324)
                    };
                    let a2326 = 32u8;
                    if equal(a2325, a2326) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2332 = {
                    let a2329 = {
                        let a2327 = v1;
                        let a2328 = 16u8;
                        fixed_u8::bit_and(a2327, a2328)
                    };
                    let a2330 = 16u8;
                    if equal(a2329, a2330) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2331, a2332)?
            };
            nat_add(a2333, a2334)?
        };
        let a2358 = {
            let a2355 = {
                let a2343 = {
                    let a2337 = {
                        let a2335 = v1;
                        let a2336 = 8u8;
                        fixed_u8::bit_and(a2335, a2336)
                    };
                    let a2338 = 8u8;
                    if equal(a2337, a2338) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2344 = {
                    let a2341 = {
                        let a2339 = v1;
                        let a2340 = 4u8;
                        fixed_u8::bit_and(a2339, a2340)
                    };
                    let a2342 = 4u8;
                    if equal(a2341, a2342) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2343, a2344)?
            };
            let a2356 = {
                let a2353 = {
                    let a2347 = {
                        let a2345 = v1;
                        let a2346 = 2u8;
                        fixed_u8::bit_and(a2345, a2346)
                    };
                    let a2348 = 2u8;
                    if equal(a2347, a2348) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2354 = {
                    let a2351 = {
                        let a2349 = v1;
                        let a2350 = 1u8;
                        fixed_u8::bit_and(a2349, a2350)
                    };
                    let a2352 = 1u8;
                    if equal(a2351, a2352) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2353, a2354)?
            };
            nat_add(a2355, a2356)?
        };
        nat_add(a2357, a2358)?
    };
    let a2360 = 48u64;
    Ok(nat_sub(a2359, a2360))
}

pub fn f361(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2361 = v0.clone();
        length_bytes(a2361)
    };
    let a2362 = v1;
    let a2363 = v2;
    if if nat_lt(a2362, a2363) {
        let a2368 = f104(v0, {
            let a2366 = v2;
            let a2367 = {
                let a2364 = v1;
                let a2365 = 1u64;
                nat_add(a2364, a2365)?
            };
            nat_sub(a2366, a2367)
        })?;
        let a2369 = 48u64;
        nat_eq(a2368, a2369)
    } else {
        false
    } {
        Ok(Some({
            let a2370 = v1;
            let a2371 = 1u64;
            nat_add(a2370, a2371)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f362(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f427(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h2372) => {
            h2372
        }
    });
    let v2: u32 = f300(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h2373, _, _) => {
            h2373
        }
    }, f300(f428(match v0.clone() {
        Adt23::C0(_, _, _, _, h2374, _, _, _, _, _) => {
            h2374
        }
    }), f300(f429(match v0.clone() {
        Adt23::C0(_, _, _, _, h2375, _, _, _, _, _) => {
            h2375
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2376, _, _, _, _) => {
            h2376
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2377, _, _, _) => {
            h2377
        }
    }), f300(v1, {
        let m2380 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h2379, _) => {
                h2379
            }
        };
        match m2380 {
            Adt22::C0(h2378, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2378
            }
        }
    }))));
    let v3: u32 = f300(f430(match v0.clone() {
        Adt23::C0(h2381, _, _, _, _, _, _, _, _, _) => {
            h2381
        }
    }), f431(match v0.clone() {
        Adt23::C0(h2382, _, _, _, _, _, _, _, _, _) => {
            h2382
        }
    }, match v0.clone() {
        Adt23::C0(_, h2383, _, _, _, _, _, _, _, _) => {
            h2383
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2384, _, _, _, _, _, _, _) => {
            h2384
        }
    }));
    Ok(Adt23::C0(f300(v2, v3), match v0.clone() {
        Adt23::C0(h2385, _, _, _, _, _, _, _, _, _) => {
            h2385
        }
    }, match v0.clone() {
        Adt23::C0(_, h2386, _, _, _, _, _, _, _, _) => {
            h2386
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2387, _, _, _, _, _, _, _) => {
            h2387
        }
    }, f300(match v0.clone() {
        Adt23::C0(_, _, _, h2388, _, _, _, _, _, _) => {
            h2388
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h2389, _, _, _, _, _) => {
            h2389
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2390, _, _, _, _) => {
            h2390
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2391, _, _, _) => {
            h2391
        }
    }, f432(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h2392, _) => {
            h2392
        }
    }), {
        let a2394 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h2393) => {
                h2393
            }
        };
        let a2395 = 1u64;
        nat_add(a2394, a2395)?
    }))
}

pub fn f363(v0: u32) -> u64 {
    let m2397 = {
        let a2396 = v0;
        fixed_u64::convert(i128::from(a2396))
    };
    match m2397 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f364(v0: u64) -> u32 {
    let m2401 = {
        let a2400 = {
            let a2398 = v0;
            let a2399 = 4294967295u64;
            fixed_u64::bit_and(a2398, a2399)
        };
        fixed_u32::convert(i128::from(a2400))
    };
    match m2401 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f365(v0: u64) -> u32 {
    let (_, h2402) = {
        let (_, h2403) = f433(Fn15::F434, 32u64, (v0, (1u32, 0u32)));
        h2403
    };
    h2402
}

pub fn f366(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2408 = f435(v0, 24u32);
    let a2409 = {
        let a2406 = f435(v1, 16u32);
        let a2407 = {
            let a2404 = f435(v2, 8u32);
            let a2405 = v3;
            fixed_u32::bit_or(a2404, a2405)
        };
        fixed_u32::bit_or(a2406, a2407)
    };
    fixed_u32::bit_or(a2408, a2409)
}

pub fn f367(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2410 = v2;
    let a2411 = v1;
    if nat_lt(a2410, a2411) {
        let m2414 = {
            let a2412 = v0;
            let a2413 = v2;
            index_bytes(a2412, a2413)
        };
        match m2414 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2416 = {
                    let a2415 = v3;
                    fixed_u32::convert(i128::from(a2415))
                };
                match m2416 {
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
        let a2417 = v2;
        let a2418 = v1;
        if nat_eq(a2417, a2418) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f368(v0: List<u64>) -> R<Bytes> {
    let m2419 = f436(v0, List::<u64>::nil());
    match m2419.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2420 = f90(v1)?;
            let a2421 = f437(v2)?;
            Ok(append_bytes(a2420, a2421))
        }
    }
}

pub fn f369(p2424: u64, p2425: List<u64>, v2: u64) -> R<List<u64>> {
    let m2423 = (p2424, p2425);
    {
        let mut m2423 = m2423;
        loop {
            let step = {
                let (v0, v1) = m2423;
                let m2422 = v0;
                if m2422 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2422 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f438(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f370(v0: u64, v1: u64) -> List<u64> {
    let m2426 = v0;
    if m2426 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2426 - 1;
        let a2427 = v1;
        let a2428 = 0u64;
        if nat_eq(a2427, a2428) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2429 = v1;
                let a2430 = f439();
                let a2431 = 0u64;
                nat_rem(a2429, a2430, a2431)
            }, f370(v2, {
                let a2432 = v1;
                let a2433 = f439();
                let a2434 = 0u64;
                nat_quot(a2432, a2433, a2434)
            }))
        }
    }
}

pub fn f371(p2445: Bytes, p2446: u64) -> R<Bytes> {
    let m2444 = (p2445, p2446);
    {
        let mut m2444 = m2444;
        loop {
            let step = {
                let (v0, v1) = m2444;
                let m2435 = v1;
                if m2435 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2435 - 1;
                    let a2439 = f104(v0.clone(), {
                        let a2437 = {
                            let a2436 = v0.clone();
                            length_bytes(a2436)
                        };
                        let a2438 = 1u64;
                        nat_sub(a2437, a2438)
                    })?;
                    let a2440 = 48u64;
                    if nat_eq(a2439, a2440) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2442 = {
                                let a2441 = v0;
                                length_bytes(a2441)
                            };
                            let a2443 = 1u64;
                            nat_sub(a2442, a2443)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: u64) -> Bytes {
    let m2447 = v0;
    if m2447 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2447 - 1;
        let a2448 = Bytes::lit(&[0x30]);
        let a2449 = f372(v1);
        append_bytes(a2448, a2449)
    }
}

pub fn f373(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2494 = {
            let a2470 = {
                let a2458 = {
                    let a2452 = {
                        let a2450 = v3;
                        let a2451 = 128u8;
                        fixed_u8::bit_and(a2450, a2451)
                    };
                    let a2453 = 128u8;
                    if equal(a2452, a2453) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2459 = {
                    let a2456 = {
                        let a2454 = v3;
                        let a2455 = 64u8;
                        fixed_u8::bit_and(a2454, a2455)
                    };
                    let a2457 = 64u8;
                    if equal(a2456, a2457) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2458, a2459)?
            };
            let a2471 = {
                let a2468 = {
                    let a2462 = {
                        let a2460 = v3;
                        let a2461 = 32u8;
                        fixed_u8::bit_and(a2460, a2461)
                    };
                    let a2463 = 32u8;
                    if equal(a2462, a2463) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2469 = {
                    let a2466 = {
                        let a2464 = v3;
                        let a2465 = 16u8;
                        fixed_u8::bit_and(a2464, a2465)
                    };
                    let a2467 = 16u8;
                    if equal(a2466, a2467) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2468, a2469)?
            };
            nat_add(a2470, a2471)?
        };
        let a2495 = {
            let a2492 = {
                let a2480 = {
                    let a2474 = {
                        let a2472 = v3;
                        let a2473 = 8u8;
                        fixed_u8::bit_and(a2472, a2473)
                    };
                    let a2475 = 8u8;
                    if equal(a2474, a2475) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2481 = {
                    let a2478 = {
                        let a2476 = v3;
                        let a2477 = 4u8;
                        fixed_u8::bit_and(a2476, a2477)
                    };
                    let a2479 = 4u8;
                    if equal(a2478, a2479) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2480, a2481)?
            };
            let a2493 = {
                let a2490 = {
                    let a2484 = {
                        let a2482 = v3;
                        let a2483 = 2u8;
                        fixed_u8::bit_and(a2482, a2483)
                    };
                    let a2485 = 2u8;
                    if equal(a2484, a2485) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2491 = {
                    let a2488 = {
                        let a2486 = v3;
                        let a2487 = 1u8;
                        fixed_u8::bit_and(a2486, a2487)
                    };
                    let a2489 = 1u8;
                    if equal(a2488, a2489) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2490, a2491)?
            };
            nat_add(a2492, a2493)?
        };
        nat_add(a2494, a2495)?
    };
    let a2496 = v4;
    let a2497 = 34u64;
    if nat_eq(a2496, a2497) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2498 = v4;
        let a2499 = 92u64;
        if nat_eq(a2498, a2499) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2500 = v4;
            let a2501 = 8u64;
            if nat_eq(a2500, a2501) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2502 = v4;
                let a2503 = 12u64;
                if nat_eq(a2502, a2503) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2504 = v4;
                    let a2505 = 10u64;
                    if nat_eq(a2504, a2505) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2506 = v4;
                        let a2507 = 13u64;
                        if nat_eq(a2506, a2507) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2508 = v4;
                            let a2509 = 9u64;
                            if nat_eq(a2508, a2509) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2510 = v4;
                                let a2511 = 32u64;
                                if nat_lt(a2510, a2511) {
                                    let a2512 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2513 = f440(v2)?;
                                    Ok(append_bytes(a2512, a2513))
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

pub fn f374(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2514 = v0.clone();
    match m2514.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2517 = {
                let (h2515, _) = v2;
                h2515
            };
            let a2518 = {
                let (h2516, _) = v1.clone();
                h2516
            };
            if equal(a2517, a2518) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f375(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f441(f442(Fn16::F443, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2523 = {
        let (h2519, _) = f444(Fn17::F445, {
            let a2521 = {
                let a2520 = v0;
                length_list(a2520)
            };
            let a2522 = 1u64;
            nat_add(a2521, a2522)?
        }, v1)?;
        h2519
    };
    match m2523.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f376(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2528 = {
        let a2525 = f237({
            let (h2524, _) = v0;
            h2524
        })?;
        let a2526 = Bytes::lit(&[0x3a]);
        append_bytes(a2525, a2526)
    };
    let a2529 = {
        let (_, h2527) = v0;
        h2527
    };
    Ok(append_bytes(a2528, a2529))
}

pub fn f377(v0: Bytes, p2555: u64, p2556: u64, p2557: Bytes) -> R<Bytes> {
    let m2554 = (p2555, (p2556, p2557));
    {
        let mut m2554 = m2554;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2554;
                let m2530 = v2;
                if m2530 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2530 - 1;
                    let m2533 = {
                        let a2531 = v0.clone();
                        let a2532 = v1;
                        index_bytes(a2531, a2532)
                    };
                    match m2533 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f59(v5)?;
                            if f157({
                                let a2534 = v6;
                                let a2535 = 34u64;
                                nat_eq(a2534, a2535)
                            }, {
                                let a2536 = v6;
                                let a2537 = 92u64;
                                nat_eq(a2536, a2537)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a2538 = v1;
                                    let a2539 = 1u64;
                                    nat_add(a2538, a2539)?
                                }, (v4, {
                                    let a2542 = v3;
                                    let a2543 = {
                                        let a2540 = f8(Str::lit("\\"));
                                        let a2541 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a2540, a2541)
                                    };
                                    append_bytes(a2542, a2543)
                                })))
                            } else {
                                let a2544 = v6;
                                let a2545 = 10u64;
                                if nat_eq(a2544, a2545) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a2546 = v1;
                                        let a2547 = 1u64;
                                        nat_add(a2546, a2547)?
                                    }, (v4, {
                                        let a2548 = v3;
                                        let a2549 = f8(Str::lit("\\n"));
                                        append_bytes(a2548, a2549)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a2550 = v1;
                                        let a2551 = 1u64;
                                        nat_add(a2550, a2551)?
                                    }, (v4, {
                                        let a2552 = v3;
                                        let a2553 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a2552, a2553)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2554 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a2559 = {
                let a2558 = v0;
                length_bytes(a2558)
            };
            let a2560 = 1u64;
            nat_sub(a2559, a2560)
        })
    } else {
        v0
    }
}

pub fn f379(v0: Fn10, p2564: List<Bytes>, p2565: List<(Bytes, ())>) -> List<Bytes> {
    let m2563 = (p2564, p2565);
    {
        let mut m2563 = m2563;
        loop {
            let step = {
                let (v1, v2) = m2563;
                let m2561 = v2;
                match m2561.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c2562: Fn10 = v0.clone();
                            c2562.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h2566, _) = v1;
        h2566
    }, v0)
}

pub fn f381(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f446(Fn18::F447, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f382(v0: Fn11, p2570: List<List<(Bytes, ())>>, p2571: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m2569 = (p2570, p2571);
    {
        let mut m2569 = m2569;
        loop {
            let step = {
                let (v1, v2) = m2569;
                let m2567 = v2;
                match m2567.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c2568: Fn11 = v0.clone();
                            c2568.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2569 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f384(v0: Fn12, p2576: u64, p2577: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m2575 = (p2576, p2577);
    {
        let mut m2575 = m2575;
        loop {
            let step = {
                let (v1, v2) = m2575;
                let m2572 = v1;
                if m2572 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2572 - 1;
                    let m2574 = {
                        let c2573: Fn12 = v0.clone();
                        c2573.apply(v2.clone())?
                    };
                    match m2574 {
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
                Ok(next) => { m2575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f448(v0)
}

pub fn f386(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f449(Fn19::F450, List::<(Bytes, ())>::nil(), v0)
}

pub fn f387(v0: Fn13, p2581: List<(Bytes, ())>, p2582: List<Bytes>) -> List<(Bytes, ())> {
    let m2580 = (p2581, p2582);
    {
        let mut m2580 = m2580;
        loop {
            let step = {
                let (v1, v2) = m2580;
                let m2578 = v2;
                match m2578.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c2579: Fn13 = v0.clone();
                            c2579.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2580 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f389(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2585 = {
        let a2583 = v0;
        let a2584 = v1;
        index_bytes(a2583, a2584)
    };
    match m2585 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f59(v2)?;
            if f106({
                let a2586 = 128u64;
                let a2587 = v3;
                nat_le(a2586, a2587)
            }, {
                let a2588 = v3;
                let a2589 = 191u64;
                nat_le(a2588, a2589)
            }) {
                Ok(Some({
                    let a2590 = v3;
                    let a2591 = 128u64;
                    nat_sub(a2590, a2591)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f390(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f106({
        let a2592 = v1;
        let a2593 = v0;
        nat_le(a2592, a2593)
    }, f106({
        let a2594 = v0;
        let a2595 = v2;
        nat_le(a2594, a2595)
    }, f157({
        let a2596 = v0;
        let a2597 = 55296u64;
        nat_lt(a2596, a2597)
    }, {
        let a2598 = 57343u64;
        let a2599 = v0;
        nat_lt(a2598, a2599)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f391() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f392(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2600 = v1;
        let a2601 = 3u64;
        nat_mul(a2600, a2601)?
    };
    let a2612 = {
        let a2608 = {
            let a2602 = f104(v0.clone(), v2)?;
            let a2603 = 16384u64;
            nat_mul(a2602, a2603)?
        };
        let a2609 = {
            let a2606 = f104(v0.clone(), {
                let a2604 = v2;
                let a2605 = 1u64;
                nat_add(a2604, a2605)?
            })?;
            let a2607 = 128u64;
            nat_mul(a2606, a2607)?
        };
        nat_add(a2608, a2609)?
    };
    let a2613 = f104(v0, {
        let a2610 = v2;
        let a2611 = 2u64;
        nat_add(a2610, a2611)?
    })?;
    nat_add(a2612, a2613)
}

pub fn f393(v0: u64) -> R<bool> {
    f251(v0, f451())
}

pub fn f394(v0: u64) -> R<bool> {
    f251(v0, f452())
}

pub fn f395(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f453(v1, v0, v2)
}

pub fn f396(v0: Bytes, v1: u64) -> Bytes {
    f46(v0, v1, 1u64)
}

pub fn f397() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f398() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f399(v0: u64) -> R<Bytes> {
    Ok(f396(f454(), {
        let a2619 = {
            let a2617 = {
                let a2614 = v0;
                let a2615 = 64u64;
                let a2616 = 0u64;
                nat_rem(a2614, a2615, a2616)
            };
            let a2618 = 2u64;
            nat_mul(a2617, a2618)?
        };
        let a2620 = 1u64;
        nat_add(a2619, a2620)?
    }))
}

pub fn f400() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f401(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f455(f134(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2621 = v3;
            let a2622 = 1u64;
            nat_add(a2621, a2622)?
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
    let m2623 = f181(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2623 {
        None => {
            let m2624 = f181(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2624 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2625 = f181(f456(), v3.clone(), 0u64)?;
                    match m2625 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2627 = v4;
                                let a2628 = {
                                    let a2626 = v3;
                                    length_bytes(a2626)
                                };
                                nat_add(a2627, a2628)?
                            };
                            let m2629 = f181(f456(), f8(Str::lit(" ")), v5)?;
                            match m2629 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f46(f456(), v5, {
                                        let a2630 = v6;
                                        let a2631 = v5;
                                        nat_sub(a2630, a2631)
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

pub fn f405(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f406(v1)
    } else {
        f338(v1)
    }
}

pub fn f406(v0: Bytes) -> R<Option<Bytes>> {
    let m2633 = f457({
        let a2632 = v0.clone();
        length_bytes(a2632)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m2633 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2635 = {
                let a2634 = v1.clone();
                length_bytes(a2634)
            };
            let a2636 = 0u64;
            if nat_eq(a2635, a2636) {
                let a2638 = {
                    let a2637 = v0;
                    length_bytes(a2637)
                };
                let a2639 = 0u64;
                if nat_eq(a2638, a2639) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f408(v1)?))
            }
        }
    }
}

pub fn f407(v0: Bytes) -> R<Option<Bytes>> {
    if f458({
        let a2640 = v0.clone();
        length_bytes(a2640)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f459(v0)?;
        if f460(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2643 = {
                let a2641 = v1.clone();
                let a2642 = 0u64;
                index_bytes(a2641, a2642)
            };
            match m2643 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f233(f59(v2)?) {
                        Ok(Some({
                            let a2644 = f8(Str::lit("n"));
                            let a2645 = v1;
                            append_bytes(a2644, a2645)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f461({
            let a2646 = v0.clone();
            length_bytes(a2646)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f408(v0: Bytes) -> R<Bytes> {
    if f115(v0.clone(), f462()) {
        f463(v0)
    } else {
        Ok(v0)
    }
}

pub fn f409() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f410(v0: u64, v1: Adt18, v2: Adt19) -> R<Result<List<Adt21>, Bytes>> {
    let v3: Bytes = f118(match v2.clone() {
        Adt19::C0(_, _, h2647, _) => {
            h2647
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt19::C0(_, h2648, _, _) => {
            h2648
        }
    }, match v2.clone() {
        Adt19::C0(_, _, h2649, _) => {
            h2649
        }
    });
    let v5: bool = f464(match v2.clone() {
        Adt19::C0(_, _, _, h2650) => {
            h2650
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m2652 = f465(match v1.clone() {
        Adt18::C0(h2651, _) => {
            h2651
        }
    }, v4, f285(v5))?;
    match m2652 {
        Ok(v7) => {
            if v5 {
                let m2653 = v7.clone();
                match m2653.uncons() {
                    None => {
                        Ok(Err::<List<Adt21>, Bytes>(f416(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m2654 = f466(v3, v7.clone(), f467(v7.clone())?, f467(v7)?)?;
                        match m2654 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt21>, Bytes>(List::cons(v11, List::<Adt21>::nil())))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt21>, Bytes>(v10))
                            }
                        }
                    }
                }
            } else {
                let m2655 = v7.clone();
                match m2655.uncons() {
                    None => {
                        f468(v0, v1, v3, match v2.clone() {
                            Adt19::C0(_, h2656, _, _) => {
                                h2656
                            }
                        }, match v2 {
                            Adt19::C0(_, _, _, h2657) => {
                                h2657
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f469(v3.clone());
                        let m2660 = f468(v0, v1, v14.clone(), match v2.clone() {
                            Adt19::C0(_, h2658, _, _) => {
                                h2658
                            }
                        }, match v2.clone() {
                            Adt19::C0(_, _, _, h2659) => {
                                h2659
                            }
                        })?;
                        match m2660 {
                            Ok(v16) => {
                                let m2662 = f466(v3, List::cons(match v2 {
                                    Adt19::C0(_, _, h2661, _) => {
                                        h2661
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f467(v7.clone())?), List::cons(v14, f467(v7)?))?;
                                match m2662 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt21>, Bytes>(f343(v16, List::cons(v18, List::<Adt21>::nil()))))
                                    }
                                    Err(v17) => {
                                        Ok(Err::<List<Adt21>, Bytes>(v17))
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt21>, Bytes>(v15))
                            }
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt21>, Bytes>(v6))
        }
    }
}

pub fn f411(v0: Bytes, v1: Adt20, v2: Bytes) -> R<Result<Adt21, Bytes>> {
    let m2665 = f201(v0.clone(), match v1.clone() {
        Adt20::C0(_, _, _, h2663, _) => {
            h2663
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, h2664) => {
            h2664
        }
    }, v2)?;
    match m2665 {
        Ok(v4) => {
            Ok(Ok::<Adt21, Bytes>(Adt21::C0(f118(v0)?, match v4.clone() {
                Adt17::C0(_, h2666) => {
                    h2666
                }
            }, match v4 {
                Adt17::C0(h2667, _) => {
                    h2667
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt21, Bytes>(v3))
        }
    }
}

pub fn f412(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20) -> R<Result<Adt25, Bytes>> {
    let m2668 = v0;
    if m2668 == 0 {
        Ok(Err::<Adt25, Bytes>(f416(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2668 - 1;
        let m2669 = f287(v4.clone(), Str::lit("type"));
        match m2669 {
            None => {
                let m2673 = f414(v4.clone())?;
                match m2673 {
                    Ok(v10) => {
                        let m2674 = f415(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m2674 {
                            None => {
                                Ok(Ok::<Adt25, Bytes>(Adt25::C0(Adt17::C0(f471(v4, f346()), List::cons(f346(), List::<Bytes>::nil())), List::<Adt21>::nil())))
                            }
                            Some(v11) => {
                                if f286(v11.clone(), Str::lit("complexType")) {
                                    let m2675 = f468(v5, v1, v2.clone(), v3, v11)?;
                                    match m2675 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt25, Bytes>(Adt25::C0(Adt17::C0(f471(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt25, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m2676 = f411(v2.clone(), v11, v3)?;
                                    match m2676 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt25, Bytes>(Adt25::C0(Adt17::C0(f471(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt21>::nil()))))
                                        }
                                        Err(v14) => {
                                            Ok(Err::<Adt25, Bytes>(v14))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v9) => {
                        Ok(Err::<Adt25, Bytes>(v9))
                    }
                }
            }
            Some(v6) => {
                let m2670 = f470(v1, v4.clone(), v6)?;
                match m2670 {
                    Ok(v8) => {
                        Ok(Ok::<Adt25, Bytes>(Adt25::C0(Adt17::C0(f471(v4, match v8.clone() {
                            Adt17::C0(h2671, _) => {
                                h2671
                            }
                        }), match v8 {
                            Adt17::C0(_, h2672) => {
                                h2672
                            }
                        }), List::<Adt21>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt25, Bytes>(f416(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f413(v0: Adt18, v1: Bytes, v2: Bytes, v3: Adt20) -> R<Result<Adt25, Bytes>> {
    let m2677 = f287(v3.clone(), Str::lit("type"));
    match m2677 {
        None => {
            let m2679 = f414(v3)?;
            match m2679 {
                Ok(v8) => {
                    let m2680 = f415(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m2680 {
                        None => {
                            Ok(Err::<Adt25, Bytes>(f416(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m2681 = f411(v2.clone(), v9, v1)?;
                            match m2681 {
                                Ok(v11) => {
                                    Ok(Ok::<Adt25, Bytes>(Adt25::C0(Adt17::C0(v2.clone(), List::cons(v2, List::<Bytes>::nil())), List::cons(v11, List::<Adt21>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt25, Bytes>(v10))
                                }
                            }
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt25, Bytes>(v7))
                }
            }
        }
        Some(v4) => {
            let m2678 = f470(v0, v3, v4)?;
            match m2678 {
                Ok(v6) => {
                    Ok(Ok::<Adt25, Bytes>(Adt25::C0(v6, List::<Adt21>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt25, Bytes>(f416(v5, v2)))
                }
            }
        }
    }
}

pub fn f414(v0: Adt20) -> R<Result<List<Adt20>, Bytes>> {
    f214(match v0.clone() {
        Adt20::C0(_, _, _, h2682, _) => {
            h2682
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, h2683) => {
            h2683
        }
    })
}

pub fn f415(p2687: List<Adt20>, v1: List<Bytes>) -> Option<Adt20> {
    let m2686 = p2687;
    {
        let mut m2686 = m2686;
        loop {
            let step = {
                let v0 = m2686;
                let m2684 = v0;
                match m2684.uncons() {
                    None => {
                        Err::<List<Adt20>, Option<Adt20>>(None::<Adt20>)
                    }
                    Some((v2, v3)) => {
                        if f115(match v2.clone() {
                            Adt20::C0(_, h2685, _, _, _) => {
                                h2685
                            }
                        }, v1.clone()) {
                            Err::<List<Adt20>, Option<Adt20>>(Some(v2))
                        } else {
                            Ok::<List<Adt20>, Option<Adt20>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(v0: Bytes, v1: Bytes) -> Bytes {
    f210(v0, f8(Str::lit(" at ")), v1)
}

pub fn f417(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20) -> R<Result<List<Adt21>, Bytes>> {
    let m2688 = v0;
    if m2688 == 0 {
        Ok(Err::<List<Adt21>, Bytes>(f416(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2688 - 1;
        let m2689 = f414(v4.clone())?;
        match m2689 {
            Ok(v7) => {
                if f286(v4, Str::lit("choice")) {
                    let m2690 = f472(v5, v1, v2.clone(), v3, false, v7, f419())?;
                    match m2690 {
                        Ok(v9) => {
                            let m2694 = f466(v2, f473(match v9.clone() {
                                Adt26::C0(h2691, _, _, _, _) => {
                                    h2691
                                }
                            }), f474(match v9.clone() {
                                Adt26::C0(h2692, _, _, _, _) => {
                                    h2692
                                }
                            }), f475(match v9.clone() {
                                Adt26::C0(h2693, _, _, _, _) => {
                                    h2693
                                }
                            }))?;
                            match m2694 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt21>, Bytes>(f343(match v9 {
                                        Adt26::C0(_, h2695, _, _, _) => {
                                            h2695
                                        }
                                    }, List::cons(v11, List::<Adt21>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<List<Adt21>, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<List<Adt21>, Bytes>(v8))
                        }
                    }
                } else {
                    let m2696 = f472(v5, v1, v2.clone(), v3, true, v7, f419())?;
                    match m2696 {
                        Ok(v13) => {
                            let m2698 = f476(v2, match v13.clone() {
                                Adt26::C0(h2697, _, _, _, _) => {
                                    h2697
                                }
                            })?;
                            match m2698 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt21>, Bytes>(f343(match v13 {
                                        Adt26::C0(_, h2699, _, _, _) => {
                                            h2699
                                        }
                                    }, List::cons(v15, List::<Adt21>::nil()))))
                                }
                                Err(v14) => {
                                    Ok(Err::<List<Adt21>, Bytes>(v14))
                                }
                            }
                        }
                        Err(v12) => {
                            Ok(Err::<List<Adt21>, Bytes>(v12))
                        }
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt21>, Bytes>(v6))
            }
        }
    }
}

pub fn f418(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: List<Adt20>, v5: Adt26) -> R<Result<Adt26, Bytes>> {
    let m2700 = v0;
    if m2700 == 0 {
        Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m2700 - 1;
        let m2701 = v4;
        match m2701.uncons() {
            None => {
                Ok(Ok::<Adt26, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f286(v7.clone(), Str::lit("attribute")) {
                    if f464(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f418(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f464(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m2702 = f287(v7.clone(), Str::lit("ref"));
                        match m2702 {
                            None => {
                                let m2712 = f287(v7.clone(), Str::lit("name"));
                                match m2712 {
                                    None => {
                                        Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m2715 = f413(v1.clone(), v3.clone(), {
                                            let a2713 = v2.clone();
                                            let a2714 = f118(v16.clone())?;
                                            append_bytes(a2713, a2714)
                                        }, v7)?;
                                        match m2715 {
                                            Ok(v18) => {
                                                f418(v6, v1, v2, v3, v8, f478(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt26, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m2704 = f140(match v7 {
                                    Adt20::C0(_, _, _, _, h2703) => {
                                        h2703
                                    }
                                }, v10.clone())?;
                                match m2704 {
                                    Ok(v12) => {
                                        if f32({
                                            let (h2705, _) = v12.clone();
                                            h2705
                                        }, f194()) {
                                            Ok(Err::<Adt26, Bytes>(f416(f210(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m2706 = f477(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m2706 {
                                                None => {
                                                    Ok(Err::<Adt26, Bytes>(f416(f210(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m2710 = f413(v1.clone(), match v13.clone() {
                                                        Adt19::C0(_, h2707, _, _) => {
                                                            h2707
                                                        }
                                                    }, f118({
                                                        let (_, h2708) = v12.clone();
                                                        h2708
                                                    })?, match v13 {
                                                        Adt19::C0(_, _, _, h2709) => {
                                                            h2709
                                                        }
                                                    })?;
                                                    match m2710 {
                                                        Ok(v15) => {
                                                            f418(v6, v1, v2, v3, v8, f478(v5, {
                                                                let (_, h2711) = v12;
                                                                h2711
                                                            }, v15, v9))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<Adt26, Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt26, Bytes>(f416(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f286(v7.clone(), Str::lit("attributeGroup")) {
                        let m2716 = f287(v7.clone(), Str::lit("ref"));
                        match m2716 {
                            None => {
                                Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m2718 = f140(match v7 {
                                    Adt20::C0(_, _, _, _, h2717) => {
                                        h2717
                                    }
                                }, v19.clone())?;
                                match m2718 {
                                    Ok(v21) => {
                                        let m2719 = f477(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m2719 {
                                            None => {
                                                Ok(Err::<Adt26, Bytes>(f416(f210(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m2721 = f414(match v22.clone() {
                                                    Adt19::C0(_, _, _, h2720) => {
                                                        h2720
                                                    }
                                                })?;
                                                match m2721 {
                                                    Ok(v24) => {
                                                        let m2724 = f418(v6, v1.clone(), f118({
                                                            let (_, h2722) = v21;
                                                            h2722
                                                        })?, match v22 {
                                                            Adt19::C0(_, h2723, _, _) => {
                                                                h2723
                                                            }
                                                        }, v24, v5)?;
                                                        match m2724 {
                                                            Ok(v26) => {
                                                                f418(v6, v1, v2, v3, v8, v26)
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt26, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt26, Bytes>(v23))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt26, Bytes>(f416(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f286(v7, Str::lit("anyAttribute")) {
                            f418(v6, v1, v2, v3, v8, f479(v5, Adt27::C0(f8(Str::lit("anyAttribute")), f480(), List::<Bytes>::nil())))
                        } else {
                            f418(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f419() -> Adt26 {
    Adt26::C0(List::<Adt27>::nil(), List::<Adt21>::nil(), 0u64, 0u64, 0u64)
}

pub fn f420(v0: List<Adt21>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m2725 = v1;
    match m2725.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m2726 = f345(v0.clone(), v3.clone());
            match m2726 {
                None => {
                    Err::<List<Bytes>, Bytes>(f421(v3))
                }
                Some(v5) => {
                    let m2728 = match v5.clone() {
                        Adt21::C0(_, _, _, h2727) => {
                            h2727
                        }
                    };
                    match m2728.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f421(v3))
                        }
                        Some((_, _)) => {
                            let m2729 = f420(v0, v4, v2.clone());
                            match m2729 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f481(v3, match v5 {
                                        Adt21::C0(_, _, _, h2730) => {
                                            h2730
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

pub fn f421(v0: Bytes) -> Bytes {
    f416(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f422(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2731 = v0;
    match m2731.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f115(v2.clone(), v1.clone()) {
                f422(v3, v1)
            } else {
                List::cons(v2, f422(v3, v1))
            }
        }
    }
}

pub fn f423(p2736: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m2735 = p2736;
    {
        let mut m2735 = m2735;
        loop {
            let step = {
                let v0 = m2735;
                let m2732 = v0;
                match m2732.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h2733, _) = v2;
                            h2733
                        }, v1.clone()) {
                            let (_, h2734) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h2734)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f424(v0: Fn14, p2741: u64, p2742: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2740 = (p2741, p2742);
    {
        let mut m2740 = m2740;
        loop {
            let step = {
                let (v1, v2) = m2740;
                let m2737 = v1;
                if m2737 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2737 - 1;
                    let m2739 = {
                        let c2738: Fn14 = v0.clone();
                        c2738.apply(v2.clone())?
                    };
                    match m2739 {
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
                Ok(next) => { m2740 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f482(v0, v1)
}

pub fn f426(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2787 = {
            let a2763 = {
                let a2751 = {
                    let a2745 = {
                        let a2743 = v1;
                        let a2744 = 128u8;
                        fixed_u8::bit_and(a2743, a2744)
                    };
                    let a2746 = 128u8;
                    if equal(a2745, a2746) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2752 = {
                    let a2749 = {
                        let a2747 = v1;
                        let a2748 = 64u8;
                        fixed_u8::bit_and(a2747, a2748)
                    };
                    let a2750 = 64u8;
                    if equal(a2749, a2750) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2751, a2752)?
            };
            let a2764 = {
                let a2761 = {
                    let a2755 = {
                        let a2753 = v1;
                        let a2754 = 32u8;
                        fixed_u8::bit_and(a2753, a2754)
                    };
                    let a2756 = 32u8;
                    if equal(a2755, a2756) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2762 = {
                    let a2759 = {
                        let a2757 = v1;
                        let a2758 = 16u8;
                        fixed_u8::bit_and(a2757, a2758)
                    };
                    let a2760 = 16u8;
                    if equal(a2759, a2760) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2761, a2762)?
            };
            nat_add(a2763, a2764)?
        };
        let a2788 = {
            let a2785 = {
                let a2773 = {
                    let a2767 = {
                        let a2765 = v1;
                        let a2766 = 8u8;
                        fixed_u8::bit_and(a2765, a2766)
                    };
                    let a2768 = 8u8;
                    if equal(a2767, a2768) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2774 = {
                    let a2771 = {
                        let a2769 = v1;
                        let a2770 = 4u8;
                        fixed_u8::bit_and(a2769, a2770)
                    };
                    let a2772 = 4u8;
                    if equal(a2771, a2772) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2773, a2774)?
            };
            let a2786 = {
                let a2783 = {
                    let a2777 = {
                        let a2775 = v1;
                        let a2776 = 2u8;
                        fixed_u8::bit_and(a2775, a2776)
                    };
                    let a2778 = 2u8;
                    if equal(a2777, a2778) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2784 = {
                    let a2781 = {
                        let a2779 = v1;
                        let a2780 = 1u8;
                        fixed_u8::bit_and(a2779, a2780)
                    };
                    let a2782 = 1u8;
                    if equal(a2781, a2782) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2783, a2784)?
            };
            nat_add(a2785, a2786)?
        };
        nat_add(a2787, a2788)?
    };
    let a2789 = 48u64;
    let a2790 = v2;
    if nat_le(a2789, a2790) {
        let a2791 = v2;
        let a2792 = 57u64;
        Ok(nat_le(a2791, a2792))
    } else {
        Ok(false)
    }
}

pub fn f427(v0: u64) -> u32 {
    let a2793 = v0;
    let a2794 = 32u64;
    if nat_lt(a2793, a2794) {
        let a2795 = v0;
        let a2796 = 16u64;
        if nat_lt(a2795, a2796) {
            let a2797 = v0;
            let a2798 = 8u64;
            if nat_lt(a2797, a2798) {
                let a2799 = v0;
                let a2800 = 4u64;
                if nat_lt(a2799, a2800) {
                    let a2801 = v0;
                    let a2802 = 2u64;
                    if nat_lt(a2801, a2802) {
                        let a2803 = v0;
                        let a2804 = 1u64;
                        if nat_lt(a2803, a2804) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2805 = v0;
                        let a2806 = 3u64;
                        if nat_lt(a2805, a2806) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2807 = v0;
                    let a2808 = 6u64;
                    if nat_lt(a2807, a2808) {
                        let a2809 = v0;
                        let a2810 = 5u64;
                        if nat_lt(a2809, a2810) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2811 = v0;
                        let a2812 = 7u64;
                        if nat_lt(a2811, a2812) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2813 = v0;
                let a2814 = 12u64;
                if nat_lt(a2813, a2814) {
                    let a2815 = v0;
                    let a2816 = 10u64;
                    if nat_lt(a2815, a2816) {
                        let a2817 = v0;
                        let a2818 = 9u64;
                        if nat_lt(a2817, a2818) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2819 = v0;
                        let a2820 = 11u64;
                        if nat_lt(a2819, a2820) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2821 = v0;
                    let a2822 = 14u64;
                    if nat_lt(a2821, a2822) {
                        let a2823 = v0;
                        let a2824 = 13u64;
                        if nat_lt(a2823, a2824) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2825 = v0;
                        let a2826 = 15u64;
                        if nat_lt(a2825, a2826) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2827 = v0;
            let a2828 = 24u64;
            if nat_lt(a2827, a2828) {
                let a2829 = v0;
                let a2830 = 20u64;
                if nat_lt(a2829, a2830) {
                    let a2831 = v0;
                    let a2832 = 18u64;
                    if nat_lt(a2831, a2832) {
                        let a2833 = v0;
                        let a2834 = 17u64;
                        if nat_lt(a2833, a2834) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2835 = v0;
                        let a2836 = 19u64;
                        if nat_lt(a2835, a2836) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2837 = v0;
                    let a2838 = 22u64;
                    if nat_lt(a2837, a2838) {
                        let a2839 = v0;
                        let a2840 = 21u64;
                        if nat_lt(a2839, a2840) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2841 = v0;
                        let a2842 = 23u64;
                        if nat_lt(a2841, a2842) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2843 = v0;
                let a2844 = 28u64;
                if nat_lt(a2843, a2844) {
                    let a2845 = v0;
                    let a2846 = 26u64;
                    if nat_lt(a2845, a2846) {
                        let a2847 = v0;
                        let a2848 = 25u64;
                        if nat_lt(a2847, a2848) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2849 = v0;
                        let a2850 = 27u64;
                        if nat_lt(a2849, a2850) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2851 = v0;
                    let a2852 = 30u64;
                    if nat_lt(a2851, a2852) {
                        let a2853 = v0;
                        let a2854 = 29u64;
                        if nat_lt(a2853, a2854) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2855 = v0;
                        let a2856 = 31u64;
                        if nat_lt(a2855, a2856) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2857 = v0;
        let a2858 = 48u64;
        if nat_lt(a2857, a2858) {
            let a2859 = v0;
            let a2860 = 40u64;
            if nat_lt(a2859, a2860) {
                let a2861 = v0;
                let a2862 = 36u64;
                if nat_lt(a2861, a2862) {
                    let a2863 = v0;
                    let a2864 = 34u64;
                    if nat_lt(a2863, a2864) {
                        let a2865 = v0;
                        let a2866 = 33u64;
                        if nat_lt(a2865, a2866) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2867 = v0;
                        let a2868 = 35u64;
                        if nat_lt(a2867, a2868) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2869 = v0;
                    let a2870 = 38u64;
                    if nat_lt(a2869, a2870) {
                        let a2871 = v0;
                        let a2872 = 37u64;
                        if nat_lt(a2871, a2872) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2873 = v0;
                        let a2874 = 39u64;
                        if nat_lt(a2873, a2874) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2875 = v0;
                let a2876 = 44u64;
                if nat_lt(a2875, a2876) {
                    let a2877 = v0;
                    let a2878 = 42u64;
                    if nat_lt(a2877, a2878) {
                        let a2879 = v0;
                        let a2880 = 41u64;
                        if nat_lt(a2879, a2880) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2881 = v0;
                        let a2882 = 43u64;
                        if nat_lt(a2881, a2882) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2883 = v0;
                    let a2884 = 46u64;
                    if nat_lt(a2883, a2884) {
                        let a2885 = v0;
                        let a2886 = 45u64;
                        if nat_lt(a2885, a2886) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2887 = v0;
                        let a2888 = 47u64;
                        if nat_lt(a2887, a2888) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2889 = v0;
            let a2890 = 56u64;
            if nat_lt(a2889, a2890) {
                let a2891 = v0;
                let a2892 = 52u64;
                if nat_lt(a2891, a2892) {
                    let a2893 = v0;
                    let a2894 = 50u64;
                    if nat_lt(a2893, a2894) {
                        let a2895 = v0;
                        let a2896 = 49u64;
                        if nat_lt(a2895, a2896) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2897 = v0;
                        let a2898 = 51u64;
                        if nat_lt(a2897, a2898) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2899 = v0;
                    let a2900 = 54u64;
                    if nat_lt(a2899, a2900) {
                        let a2901 = v0;
                        let a2902 = 53u64;
                        if nat_lt(a2901, a2902) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2903 = v0;
                        let a2904 = 55u64;
                        if nat_lt(a2903, a2904) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2905 = v0;
                let a2906 = 60u64;
                if nat_lt(a2905, a2906) {
                    let a2907 = v0;
                    let a2908 = 58u64;
                    if nat_lt(a2907, a2908) {
                        let a2909 = v0;
                        let a2910 = 57u64;
                        if nat_lt(a2909, a2910) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2911 = v0;
                        let a2912 = 59u64;
                        if nat_lt(a2911, a2912) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2913 = v0;
                    let a2914 = 62u64;
                    if nat_lt(a2913, a2914) {
                        let a2915 = v0;
                        let a2916 = 61u64;
                        if nat_lt(a2915, a2916) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2917 = v0;
                        let a2918 = 63u64;
                        if nat_lt(a2917, a2918) {
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

pub fn f428(v0: u32) -> u32 {
    let a2921 = {
        let a2919 = f483(v0, 6u32, 26u32);
        let a2920 = f483(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2919, a2920)
    };
    let a2922 = f483(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2921, a2922)
}

pub fn f429(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2928 = {
        let a2923 = v0;
        let a2924 = v1;
        fixed_u32::bit_and(a2923, a2924)
    };
    let a2929 = {
        let a2926 = {
            let a2925 = v0;
            fixed_u32::bit_not(a2925)
        };
        let a2927 = v2;
        fixed_u32::bit_and(a2926, a2927)
    };
    fixed_u32::bit_xor(a2928, a2929)
}

pub fn f430(v0: u32) -> u32 {
    let a2932 = {
        let a2930 = f483(v0, 2u32, 30u32);
        let a2931 = f483(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2930, a2931)
    };
    let a2933 = f483(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2932, a2933)
}

pub fn f431(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2942 = {
        let a2938 = {
            let a2934 = v0;
            let a2935 = v1;
            fixed_u32::bit_and(a2934, a2935)
        };
        let a2939 = {
            let a2936 = v0;
            let a2937 = v2;
            fixed_u32::bit_and(a2936, a2937)
        };
        fixed_u32::bit_xor(a2938, a2939)
    };
    let a2943 = {
        let a2940 = v1;
        let a2941 = v2;
        fixed_u32::bit_and(a2940, a2941)
    };
    fixed_u32::bit_xor(a2942, a2943)
}

pub fn f432(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h2944, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2944
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2945, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2945
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h2946, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2946
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h2947, _, _, _, _, _, _, _, _, _, _, _) => {
            h2947
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2948, _, _, _, _, _, _, _, _, _, _) => {
            h2948
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2949, _, _, _, _, _, _, _, _, _) => {
            h2949
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2950, _, _, _, _, _, _, _, _) => {
            h2950
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2951, _, _, _, _, _, _, _) => {
            h2951
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2952, _, _, _, _, _, _) => {
            h2952
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h2953, _, _, _, _, _) => {
            h2953
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h2954, _, _, _, _) => {
            h2954
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2955, _, _, _) => {
            h2955
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2956, _, _) => {
            h2956
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2957, _) => {
            h2957
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2958) => {
            h2958
        }
    }, f300(f484(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2959, _) => {
            h2959
        }
    }), f300(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2960, _, _, _, _, _, _) => {
            h2960
        }
    }, f300(f485(match v0.clone() {
        Adt22::C0(_, h2961, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2961
        }
    }), match v0 {
        Adt22::C0(h2962, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2962
        }
    }))))
}

pub fn f433(v0: Fn15, p2966: u64, p2967: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2965 = (p2966, p2967);
    {
        let mut m2965 = m2965;
        loop {
            let step = {
                let (v1, v2) = m2965;
                let m2963 = v1;
                if m2963 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2963 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2964: Fn15 = v0.clone();
                        c2964.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2965 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2968, _) = v0;
        h2968
    };
    let v2: u32 = {
        let (h2969, _) = {
            let (_, h2970) = v0;
            h2970
        };
        h2969
    };
    let v3: u32 = {
        let (_, h2971) = {
            let (_, h2972) = v0;
            h2972
        };
        h2971
    };
    ({
        let a2973 = v1;
        let a2974 = 2u64;
        let a2975 = 0u64;
        nat_quot(a2973, a2974, a2975)
    }, (f435(v2, 1u32), {
        let a2979 = {
            let a2976 = v1;
            let a2977 = 2u64;
            let a2978 = 0u64;
            nat_rem(a2976, a2977, a2978)
        };
        let a2980 = 1u64;
        if nat_eq(a2979, a2980) {
            let a2981 = v3;
            let a2982 = v2;
            fixed_u32::bit_or(a2981, a2982)
        } else {
            v3
        }
    }))
}

pub fn f435(v0: u32, v1: u32) -> u32 {
    let m2985 = {
        let a2983 = v0;
        let a2984 = v1;
        fixed_u32::shift_left(a2983, a2984)
    };
    match m2985 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f436(p2988: List<u64>, p2989: List<u64>) -> List<u64> {
    let m2987 = (p2988, p2989);
    {
        let mut m2987 = m2987;
        loop {
            let step = {
                let (v0, v1) = m2987;
                let m2986 = v0;
                match m2986.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2987 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(v0: List<u64>) -> R<Bytes> {
    let m2990 = v0;
    match m2990.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a2991 = f486(v1)?;
            let a2992 = f437(v2)?;
            Ok(append_bytes(a2991, a2992))
        }
    }
}

pub fn f438(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m2993 = v0;
    match m2993.uncons() {
        None => {
            let a2994 = v2;
            let a2995 = 0u64;
            if nat_eq(a2994, a2995) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a2998 = {
                    let a2996 = v3;
                    let a2997 = v1;
                    nat_mul(a2996, a2997)?
                };
                let a2999 = v2;
                nat_add(a2998, a2999)?
            };
            Ok(List::cons({
                let a3000 = v5;
                let a3001 = f439();
                let a3002 = 0u64;
                nat_rem(a3000, a3001, a3002)
            }, f438(v4, v1, {
                let a3003 = v5;
                let a3004 = f439();
                let a3005 = 0u64;
                nat_quot(a3003, a3004, a3005)
            })?))
        }
    }
}

pub fn f439() -> u64 {
    1000000000u64
}

pub fn f440(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3050 = {
            let a3026 = {
                let a3014 = {
                    let a3008 = {
                        let a3006 = v1;
                        let a3007 = 128u8;
                        fixed_u8::bit_and(a3006, a3007)
                    };
                    let a3009 = 128u8;
                    if equal(a3008, a3009) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3015 = {
                    let a3012 = {
                        let a3010 = v1;
                        let a3011 = 64u8;
                        fixed_u8::bit_and(a3010, a3011)
                    };
                    let a3013 = 64u8;
                    if equal(a3012, a3013) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3014, a3015)?
            };
            let a3027 = {
                let a3024 = {
                    let a3018 = {
                        let a3016 = v1;
                        let a3017 = 32u8;
                        fixed_u8::bit_and(a3016, a3017)
                    };
                    let a3019 = 32u8;
                    if equal(a3018, a3019) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3025 = {
                    let a3022 = {
                        let a3020 = v1;
                        let a3021 = 16u8;
                        fixed_u8::bit_and(a3020, a3021)
                    };
                    let a3023 = 16u8;
                    if equal(a3022, a3023) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3024, a3025)?
            };
            nat_add(a3026, a3027)?
        };
        let a3051 = {
            let a3048 = {
                let a3036 = {
                    let a3030 = {
                        let a3028 = v1;
                        let a3029 = 8u8;
                        fixed_u8::bit_and(a3028, a3029)
                    };
                    let a3031 = 8u8;
                    if equal(a3030, a3031) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3037 = {
                    let a3034 = {
                        let a3032 = v1;
                        let a3033 = 4u8;
                        fixed_u8::bit_and(a3032, a3033)
                    };
                    let a3035 = 4u8;
                    if equal(a3034, a3035) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3036, a3037)?
            };
            let a3049 = {
                let a3046 = {
                    let a3040 = {
                        let a3038 = v1;
                        let a3039 = 2u8;
                        fixed_u8::bit_and(a3038, a3039)
                    };
                    let a3041 = 2u8;
                    if equal(a3040, a3041) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3047 = {
                    let a3044 = {
                        let a3042 = v1;
                        let a3043 = 1u8;
                        fixed_u8::bit_and(a3042, a3043)
                    };
                    let a3045 = 1u8;
                    if equal(a3044, a3045) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3046, a3047)?
            };
            nat_add(a3048, a3049)?
        };
        nat_add(a3050, a3051)?
    };
    let a3058 = f191(f487(), {
        let a3052 = v2;
        let a3053 = 16u64;
        let a3054 = 0u64;
        nat_quot(a3052, a3053, a3054)
    });
    let a3059 = f191(f487(), {
        let a3055 = v2;
        let a3056 = 16u64;
        let a3057 = 0u64;
        nat_rem(a3055, a3056, a3057)
    });
    Ok(append_bytes(a3058, a3059))
}

pub fn f441(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f488(Fn20::F489, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f442(v0: Fn16, p3063: List<List<(Bytes, Bytes)>>, p3064: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3062 = (p3063, p3064);
    {
        let mut m3062 = m3062;
        loop {
            let step = {
                let (v1, v2) = m3062;
                let m3060 = v2;
                match m3060.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3061: Fn16 = v0.clone();
                            c3061.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f444(v0: Fn17, p3069: u64, p3070: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3068 = (p3069, p3070);
    {
        let mut m3068 = m3068;
        loop {
            let step = {
                let (v1, v2) = m3068;
                let m3065 = v1;
                if m3065 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3065 - 1;
                    let m3067 = {
                        let c3066: Fn17 = v0.clone();
                        c3066.apply(v2.clone())?
                    };
                    match m3067 {
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
                Ok(next) => { m3068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f490(v0)
}

pub fn f446(v0: Fn18, p3074: List<List<(Bytes, ())>>, p3075: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m3073 = (p3074, p3075);
    {
        let mut m3073 = m3073;
        loop {
            let step = {
                let (v1, v2) = m3073;
                let m3071 = v2;
                match m3071.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c3072: Fn18 = v0.clone();
                            c3072.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3073 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f448(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a3077 = {
        let a3076 = v0.clone();
        length_list(a3076)
    };
    let a3078 = 1u64;
    if nat_le(a3077, a3078) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f491(v0)?))
    }
}

pub fn f449(v0: Fn19, p3082: List<(Bytes, ())>, p3083: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m3081 = (p3082, p3083);
    {
        let mut m3081 = m3081;
        loop {
            let step = {
                let (v1, v2) = m3081;
                let m3079 = v2;
                match m3079.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c3080: Fn19 = v0.clone();
                            c3080.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3081 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f451() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f452() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f453(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f134(v0, {
        let (h3084, _) = v2;
        h3084
    })?;
    let a3086 = 1114111u64;
    let a3087 = {
        let (_, h3085) = v2;
        h3085
    };
    if nat_lt(a3086, a3087) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f492(v3, v1);
        let a3088 = v4;
        let a3089 = 16u64;
        if nat_eq(a3088, a3089) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a3091 = {
                    let (h3090, _) = v2;
                    h3090
                };
                let a3092 = 1u64;
                nat_add(a3091, a3092)?
            }, {
                let a3096 = {
                    let a3094 = {
                        let (_, h3093) = v2;
                        h3093
                    };
                    let a3095 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3094, a3095)?
                };
                let a3097 = v4;
                nat_add(a3096, a3097)?
            })))
        }
    }
}

pub fn f454() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f455(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a3098 = v0;
        let a3099 = v2;
        if nat_eq(a3098, a3099) {
            Ok(true)
        } else {
            f251(v0, f493())
        }
    } else {
        f251(v0, f494())
    }
}

pub fn f456() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f457(p3112: u64, v1: Bytes, p3113: u64, p3114: Bytes) -> R<Option<Bytes>> {
    let m3111 = (p3112, (p3113, p3114));
    {
        let mut m3111 = m3111;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3111;
                let m3100 = v0;
                if m3100 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3100 - 1;
                    let a3102 = {
                        let a3101 = v1.clone();
                        length_bytes(a3101)
                    };
                    let a3103 = v2;
                    if nat_le(a3102, a3103) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3104 = f324(v1.clone(), v2)?;
                        match m3104 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3106 = v2;
                                    let a3107 = {
                                        let (_, h3105) = v5;
                                        h3105
                                    };
                                    nat_add(a3106, a3107)?
                                }, {
                                    let a3109 = v3;
                                    let a3110 = f495(v1.clone(), v2, {
                                        let (h3108, _) = v5;
                                        h3108
                                    });
                                    append_bytes(a3109, a3110)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(p3124: u64, v1: Bytes, p3125: u64) -> R<bool> {
    let m3123 = (p3124, p3125);
    {
        let mut m3123 = m3123;
        loop {
            let step = {
                let (v0, v2) = m3123;
                let m3115 = v0;
                if m3115 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3115 - 1;
                    let m3118 = {
                        let a3116 = v1.clone();
                        let a3117 = v2;
                        index_bytes(a3116, a3117)
                    };
                    match m3118 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3119 = f59(v4)?;
                            let a3120 = 128u64;
                            if nat_lt(a3119, a3120) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3121 = v2;
                                    let a3122 = 1u64;
                                    nat_add(a3121, a3122)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f169(v0.clone(), 0u64, {
        let a3126 = v0;
        length_bytes(a3126)
    }, false, Bytes::lit(&[]))?;
    let m3129 = {
        let a3127 = v1.clone();
        let a3128 = 0u64;
        index_bytes(a3127, a3128)
    };
    match m3129 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f59(v2)?;
            if f232(v3) {
                let a3136 = f46({
                    let a3130 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3130)
                }, {
                    let a3131 = v3;
                    let a3132 = 65u64;
                    nat_sub(a3131, a3132)
                }, 1u64);
                let a3137 = f46(v1.clone(), 1u64, {
                    let a3134 = {
                        let a3133 = v1;
                        length_bytes(a3133)
                    };
                    let a3135 = 1u64;
                    nat_sub(a3134, a3135)
                });
                Ok(append_bytes(a3136, a3137))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f460(v0: Bytes) -> R<bool> {
    let m3140 = {
        let a3138 = v0;
        let a3139 = 0u64;
        index_bytes(a3138, a3139)
    };
    match m3140 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f157(f231(f59(v1)?), f232(f59(v1)?)))
        }
    }
}

pub fn f461(p3153: u64, v1: Bytes, p3154: u64, p3155: Bytes) -> R<Option<Bytes>> {
    let m3152 = (p3153, (p3154, p3155));
    {
        let mut m3152 = m3152;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3152;
                let m3141 = v0;
                if m3141 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3141 - 1;
                    let a3143 = {
                        let a3142 = v1.clone();
                        length_bytes(a3142)
                    };
                    let a3144 = v2;
                    if nat_le(a3143, a3144) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3145 = f324(v1.clone(), v2)?;
                        match m3145 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3147 = v2;
                                    let a3148 = {
                                        let (_, h3146) = v5;
                                        h3146
                                    };
                                    nat_add(a3147, a3148)?
                                }, {
                                    let a3150 = v3;
                                    let a3151 = f496({
                                        let (h3149, _) = v5;
                                        h3149
                                    });
                                    append_bytes(a3150, a3151)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("false")), List::<Bytes>::nil())))))))))))))))))))))
}

pub fn f463(v0: Bytes) -> R<Bytes> {
    let m3158 = {
        let a3156 = v0.clone();
        let a3157 = 0u64;
        index_bytes(a3156, a3157)
    };
    match m3158 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a3164 = {
                let a3159 = f8(Str::lit("u"));
                let a3160 = f497(8u64, f59(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a3159, a3160)
            };
            let a3165 = f46(v0.clone(), 1u64, {
                let a3162 = {
                    let a3161 = v0;
                    length_bytes(a3161)
                };
                let a3163 = 1u64;
                nat_sub(a3162, a3163)
            });
            Ok(append_bytes(a3164, a3165))
        }
    }
}

pub fn f464(v0: Adt20, v1: Str, v2: Str) -> bool {
    let m3166 = f287(v0, v1);
    match m3166 {
        None => {
            false
        }
        Some(v3) => {
            f32(v3, f8(v2))
        }
    }
}

pub fn f465(v0: List<Adt19>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m3167 = v0;
    match m3167.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3168 = f465(v4, v1.clone(), v2)?;
            match m3168 {
                Ok(v6) => {
                    if f32(match v3.clone() {
                        Adt19::C0(h3169, _, _, _) => {
                            h3169
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m3170 = f498(v3.clone())?;
                        match m3170 {
                            Ok(v8) => {
                                let m3171 = v8;
                                match m3171 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f32({
                                            let (h3172, _) = {
                                                let (_, h3173) = v9.clone();
                                                h3173
                                            };
                                            h3172
                                        }, {
                                            let (h3174, _) = v1;
                                            h3174
                                        }) {
                                            if f32({
                                                let (_, h3175) = {
                                                    let (_, h3176) = v9;
                                                    h3176
                                                };
                                                h3175
                                            }, {
                                                let (_, h3177) = v1;
                                                h3177
                                            }) {
                                                if f285(v2) {
                                                    true
                                                } else {
                                                    f286({
                                                        let (h3178, _) = v9;
                                                        h3178
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt19::C0(_, _, h3179, _) => {
                                                    h3179
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

pub fn f466(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt21, Bytes>> {
    let m3180 = v1.clone();
    match m3180.uncons() {
        None => {
            Ok(Err::<Adt21, Bytes>(f416(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m3181 = f499(v1)?;
            match m3181 {
                Ok(v7) => {
                    let v8: List<Bytes> = f500(v7, v2);
                    Ok(Ok::<Adt21, Bytes>(Adt21::C0(v0.clone(), v3, f270(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt21, Bytes>(f416(v6, v0)))
                }
            }
        }
    }
}

pub fn f467(v0: List<Bytes>) -> R<List<Bytes>> {
    let m3182 = v0;
    match m3182.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f118(v1)?, f467(v2)?))
        }
    }
}

pub fn f468(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20) -> R<Result<List<Adt21>, Bytes>> {
    let m3183 = v0;
    if m3183 == 0 {
        Ok(Err::<List<Adt21>, Bytes>(f416(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3183 - 1;
        let m3184 = f501(v5, v1, v2.clone(), v3, v4, f419())?;
        match m3184 {
            Ok(v7) => {
                let m3186 = f476(v2, match v7.clone() {
                    Adt26::C0(h3185, _, _, _, _) => {
                        h3185
                    }
                })?;
                match m3186 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt21>, Bytes>(f343(match v7 {
                            Adt26::C0(_, h3187, _, _, _) => {
                                h3187
                            }
                        }, List::cons(v9, List::<Adt21>::nil()))))
                    }
                    Err(v8) => {
                        Ok(Err::<List<Adt21>, Bytes>(v8))
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt21>, Bytes>(v6))
            }
        }
    }
}

pub fn f469(v0: Bytes) -> Bytes {
    let a3188 = v0;
    let a3189 = f8(Str::lit("Content"));
    append_bytes(a3188, a3189)
}

pub fn f470(v0: Adt18, v1: Adt20, v2: Bytes) -> R<Result<Adt17, Bytes>> {
    let m3191 = f140(match v1 {
        Adt20::C0(_, _, _, _, h3190) => {
            h3190
        }
    }, v2.clone())?;
    match m3191 {
        Ok(v4) => {
            if f32({
                let (h3192, _) = v4.clone();
                h3192
            }, f199()) {
                if f32({
                    let (_, h3193) = v4.clone();
                    h3193
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt17, Bytes>(Adt17::C0(f346(), List::cons(f346(), List::<Bytes>::nil()))))
                } else {
                    let m3195 = f334({
                        let (_, h3194) = v4;
                        h3194
                    }, 16u64)?;
                    match m3195 {
                        Ok(v6) => {
                            Ok(Ok::<Adt17, Bytes>(Adt17::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt17, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f115({
                    let (h3196, _) = v4.clone();
                    h3196
                }, match v0.clone() {
                    Adt18::C0(_, h3197) => {
                        h3197
                    }
                }) {
                    let v7: Bytes = f118({
                        let (_, h3198) = v4.clone();
                        h3198
                    })?;
                    let m3199 = f477(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m3199 {
                        None => {
                            let m3200 = f477(v0, Str::lit("simpleType"), v4);
                            match m3200 {
                                None => {
                                    Ok(Err::<Adt17, Bytes>(f210(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
                                }
                                Some(_) => {
                                    Ok(Ok::<Adt17, Bytes>(Adt17::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<Adt17, Bytes>(Adt17::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                        }
                    }
                } else {
                    Ok(Err::<Adt17, Bytes>(f210(f8(Str::lit("the type ")), v2, f210(f8(Str::lit(" is in the namespace ")), {
                        let (h3201, _) = v4;
                        h3201
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt17, Bytes>(v3))
        }
    }
}

pub fn f471(v0: Adt20, v1: Bytes) -> Bytes {
    if f464(v0, Str::lit("nillable"), Str::lit("true")) {
        f502(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f472(p3206: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: bool, p3207: List<Adt20>, p3208: Adt26) -> R<Result<Adt26, Bytes>> {
    let m3205 = (p3206, (p3207, p3208));
    {
        let mut m3205 = m3205;
        loop {
            let step = {
                let (v0, (v5, v6)) = m3205;
                let m3202 = v0;
                if m3202 == 0 {
                    Err::<(u64, (List<Adt20>, Adt26)), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m3202 - 1;
                    let m3203 = v5;
                    match m3203.uncons() {
                        None => {
                            Err::<(u64, (List<Adt20>, Adt26)), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m3204 = f503(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m3204 {
                                Ok(v11) => {
                                    Ok::<(u64, (List<Adt20>, Adt26)), R<Result<Adt26, Bytes>>>((v7, (v9, v11)))
                                }
                                Err(v10) => {
                                    Err::<(u64, (List<Adt20>, Adt26)), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(v10)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3205 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: List<Adt27>) -> List<Bytes> {
    let m3209 = v0;
    match m3209.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt27::C0(h3210, _, _) => {
                    h3210
                }
            }, f473(v2))
        }
    }
}

pub fn f474(v0: List<Adt27>) -> List<Bytes> {
    let m3211 = v0;
    match m3211.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt27::C0(_, h3212, _) => {
                    h3212
                }
            }, f474(v2))
        }
    }
}

pub fn f475(v0: List<Adt27>) -> List<Bytes> {
    let m3213 = v0;
    match m3213.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f353(match v1 {
                Adt27::C0(_, _, h3214) => {
                    h3214
                }
            }, f475(v2))
        }
    }
}

pub fn f476(v0: Bytes, v1: List<Adt27>) -> R<Result<Adt21, Bytes>> {
    let m3215 = f499(f473(v1.clone()))?;
    match m3215 {
        Ok(v3) => {
            let v4: List<Bytes> = f474(v1.clone());
            let v7: Bytes = {
                let m3216 = v1.clone();
                match m3216.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f210(f8(Str::lit("\u{a}")), f148(f504(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt21, Bytes>(Adt21::C0(v0.clone(), f475(v1), f210(f8(Str::lit("structure{\u{a}  name: ")), v0, f210(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f210(f8(Str::lit("    {name: mk, fields: [")), f148(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt21, Bytes>(f416(v2, v0)))
        }
    }
}

pub fn f477(v0: Adt18, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt19> {
    f505(match v0 {
        Adt18::C0(h3217, _) => {
            h3217
        }
    }, f8(v1), {
        let (h3218, _) = v2;
        h3218
    }, {
        let (_, h3219) = v2;
        h3219
    })
}

pub fn f478(v0: Adt26, v1: Bytes, v2: Adt25, v3: bool) -> Adt26 {
    f506(f479(v0, Adt27::C0(v1, if v3 {
        let m3222 = match v2.clone() {
            Adt25::C0(h3221, _) => {
                h3221
            }
        };
        match m3222 {
            Adt17::C0(h3220, _) => {
                h3220
            }
        }
    } else {
        f502(Str::lit("Option"), {
            let m3225 = match v2.clone() {
                Adt25::C0(h3224, _) => {
                    h3224
                }
            };
            match m3225 {
                Adt17::C0(h3223, _) => {
                    h3223
                }
            }
        })
    }, {
        let m3228 = match v2.clone() {
            Adt25::C0(h3227, _) => {
                h3227
            }
        };
        match m3228 {
            Adt17::C0(_, h3226) => {
                h3226
            }
        }
    })), match v2 {
        Adt25::C0(_, h3229) => {
            h3229
        }
    })
}

pub fn f479(v0: Adt26, v1: Adt27) -> Adt26 {
    Adt26::C0(f507(match v0.clone() {
        Adt26::C0(h3230, _, _, _, _) => {
            h3230
        }
    }, List::cons(v1, List::<Adt27>::nil())), match v0.clone() {
        Adt26::C0(_, h3231, _, _, _) => {
            h3231
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3232, _, _) => {
            h3232
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3233, _) => {
            h3233
        }
    }, match v0 {
        Adt26::C0(_, _, _, _, h3234) => {
            h3234
        }
    })
}

pub fn f480() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f481(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f210(f8(Str::lit("inductive{\u{a}  name: ")), v0, f210(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f148(v1, f8(Str::lit(",\u{a}"))), f210(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f482(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3235, _) = v1;
        h3235
    };
    let m3237 = {
        let (_, h3236) = v1;
        h3236
    };
    match m3237 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3240 = {
                let a3238 = v0.clone();
                let a3239 = v2;
                index_bytes(a3238, a3239)
            };
            match m3240 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3245 = {
                        let a3243 = v0;
                        let a3244 = {
                            let a3241 = v2;
                            let a3242 = 1u64;
                            nat_add(a3241, a3242)?
                        };
                        index_bytes(a3243, a3244)
                    };
                    match m3245 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f508(v4)? {
                                f508(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3246 = v2;
                                    let a3247 = 2u64;
                                    nat_add(a3246, a3247)?
                                }, Some({
                                    let a3252 = v3;
                                    let a3253 = f191(f509(), {
                                        let a3250 = {
                                            let a3248 = f510(v4)?;
                                            let a3249 = 16u64;
                                            nat_mul(a3248, a3249)?
                                        };
                                        let a3251 = f510(v5)?;
                                        nat_add(a3250, a3251)?
                                    });
                                    append_bytes(a3252, a3253)
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

pub fn f483(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3254 = f227(v0, v1);
    let a3255 = f435(v0, v2);
    fixed_u32::bit_or(a3254, a3255)
}

pub fn f484(v0: u32) -> u32 {
    let a3258 = {
        let a3256 = f483(v0, 17u32, 15u32);
        let a3257 = f483(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3256, a3257)
    };
    let a3259 = f227(v0, 10u32);
    fixed_u32::bit_xor(a3258, a3259)
}

pub fn f485(v0: u32) -> u32 {
    let a3262 = {
        let a3260 = f483(v0, 7u32, 25u32);
        let a3261 = f483(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3260, a3261)
    };
    let a3263 = f227(v0, 3u32);
    fixed_u32::bit_xor(a3262, a3263)
}

pub fn f486(v0: u64) -> R<Bytes> {
    let v1: Bytes = f90(v0)?;
    let a3267 = f372({
        let a3265 = 9u64;
        let a3266 = {
            let a3264 = v1.clone();
            length_bytes(a3264)
        };
        nat_sub(a3265, a3266)
    });
    let a3268 = v1;
    Ok(append_bytes(a3267, a3268))
}

pub fn f487() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f488(v0: Fn20, p3272: List<List<(Bytes, Bytes)>>, p3273: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3271 = (p3272, p3273);
    {
        let mut m3271 = m3271;
        loop {
            let step = {
                let (v1, v2) = m3271;
                let m3269 = v2;
                match m3269.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3270: Fn20 = v0.clone();
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

pub fn f489(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f490(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3275 = {
        let a3274 = v0.clone();
        length_list(a3274)
    };
    let a3276 = 1u64;
    if nat_le(a3275, a3276) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f511(v0)?))
    }
}

pub fn f491(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f381({
        let (_, h3277) = {
            let (h3278, _) = f512(Fn21::F513, {
                let a3280 = {
                    let a3279 = v0.clone();
                    length_list(a3279)
                };
                let a3281 = 1u64;
                nat_add(a3280, a3281)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h3278
        };
        h3277
    }))
}

pub fn f492(v0: u64, v1: bool) -> u64 {
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

pub fn f493() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f494() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f495(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f157(f106(f157(f231(v2), f232(v2)), f106({
        let a3300 = v2;
        let a3301 = 117u64;
        !nat_eq(a3300, a3301)
    }, {
        let a3302 = v2;
        let a3303 = 85u64;
        !nat_eq(a3302, a3303)
    })), f106(f233(v2), {
        let a3304 = 0u64;
        let a3305 = v1;
        nat_lt(a3304, a3305)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a3306 = v2;
        let a3307 = 65536u64;
        if nat_lt(a3306, a3307) {
            let a3308 = f8(Str::lit("u"));
            let a3309 = f497(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3308, a3309)
        } else {
            let a3310 = f8(Str::lit("U"));
            let a3311 = f497(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3310, a3311)
        }
    }
}

pub fn f496(v0: u64) -> Bytes {
    let a3312 = f8(Str::lit("u"));
    let a3313 = f497(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3312, a3313)
}

pub fn f497(p3330: u64, p3331: u64, p3332: u64, p3333: Bytes) -> Bytes {
    let m3329 = (p3330, (p3331, (p3332, p3333)));
    {
        let mut m3329 = m3329;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3329;
                let m3314 = v0;
                if m3314 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3314 - 1;
                    if f106({
                        let a3315 = v1;
                        let a3316 = 0u64;
                        nat_eq(a3315, a3316)
                    }, {
                        let a3317 = v2;
                        let a3318 = 0u64;
                        nat_eq(a3317, a3318)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3319 = v1;
                            let a3320 = 16u64;
                            let a3321 = 0u64;
                            nat_quot(a3319, a3320, a3321)
                        }, ({
                            let a3322 = v2;
                            let a3323 = 1u64;
                            nat_sub(a3322, a3323)
                        }, {
                            let a3327 = f514({
                                let a3324 = v1;
                                let a3325 = 16u64;
                                let a3326 = 0u64;
                                nat_rem(a3324, a3325, a3326)
                            });
                            let a3328 = v3;
                            append_bytes(a3327, a3328)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(v0: Adt19) -> R<Result<Option<(Adt20, (Bytes, Bytes))>, Bytes>> {
    let m3335 = f414(match v0 {
        Adt19::C0(_, _, _, h3334) => {
            h3334
        }
    })?;
    match m3335 {
        Ok(v2) => {
            let m3336 = f415(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m3336 {
                None => {
                    Ok(Ok::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(None::<(Adt20, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m3337 = f414(v3)?;
                    match m3337 {
                        Ok(v5) => {
                            let m3338 = f415(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m3338 {
                                None => {
                                    Ok(Ok::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(None::<(Adt20, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m3339 = f287(v6.clone(), Str::lit("base"));
                                    match m3339 {
                                        None => {
                                            Ok(Ok::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(None::<(Adt20, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m3341 = f140(match v6.clone() {
                                                Adt20::C0(_, _, _, _, h3340) => {
                                                    h3340
                                                }
                                            }, v7)?;
                                            match m3341 {
                                                Ok(v9) => {
                                                    Ok(Ok::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(Some((v6, v9))))
                                                }
                                                Err(v8) => {
                                                    Ok(Err::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(v8))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(v4) => {
                            Ok(Err::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Ok(Err::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(v1))
        }
    }
}

pub fn f499(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f515(f269(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f500(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3342 = v0;
    match m3342.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3343 = v1;
            match m3343.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f210(f8(Str::lit("    {name: ")), v2, f210(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f500(v3, v5))
                }
            }
        }
    }
}

pub fn f501(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20, v5: Adt26) -> R<Result<Adt26, Bytes>> {
    let m3344 = v0;
    if m3344 == 0 {
        Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m3344 - 1;
        if f464(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m3345 = f414(v4)?;
            match m3345 {
                Ok(v8) => {
                    let m3346 = f415(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m3346 {
                        None => {
                            let m3348 = f415(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m3348 {
                                None => {
                                    let m3349 = f472(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m3349 {
                                        Ok(v11) => {
                                            f418(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt26, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f464(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m3350 = f414(v12.clone())?;
                                        match m3350 {
                                            Ok(v14) => {
                                                let m3351 = f415(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m3351 {
                                                    None => {
                                                        Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m3352 = f287(v15.clone(), Str::lit("base"));
                                                        match m3352 {
                                                            None => {
                                                                Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m3354 = f140(match v15.clone() {
                                                                    Adt20::C0(_, _, _, _, h3353) => {
                                                                        h3353
                                                                    }
                                                                }, v16.clone())?;
                                                                match m3354 {
                                                                    Ok(v18) => {
                                                                        let m3355 = f414(v15.clone())?;
                                                                        match m3355 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f32({
                                                                                    let (h3356, _) = v18.clone();
                                                                                    h3356
                                                                                }, f199()) {
                                                                                    f32({
                                                                                        let (_, h3357) = v18.clone();
                                                                                        h3357
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f286(v15.clone(), Str::lit("restriction")) {
                                                                                    f285(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt26, Bytes>(f416(f210(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m3372 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f32(match v12 {
                                                                                            Adt20::C0(_, h3358, _, _, _) => {
                                                                                                h3358
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt26, Bytes>(v5)
                                                                                        } else {
                                                                                            let m3359 = f477(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m3359 {
                                                                                                None => {
                                                                                                    Err::<Adt26, Bytes>(f416(f210(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m3363 = f501(v6, v1.clone(), f118({
                                                                                                        let (_, h3360) = v18;
                                                                                                        h3360
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt19::C0(_, h3361, _, _) => {
                                                                                                            h3361
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt19::C0(_, _, _, h3362) => {
                                                                                                            h3362
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m3363 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt26, Bytes>(f516(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt26, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m3364 = f477(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m3364 {
                                                                                            None => {
                                                                                                let m3369 = f470(v1.clone(), v15, v16)?;
                                                                                                match m3369 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt26, Bytes>(f479(v5, Adt27::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt17::C0(h3370, _) => {
                                                                                                                h3370
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt17::C0(_, h3371) => {
                                                                                                                h3371
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt26, Bytes>(f416(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m3368 = f501(v6, v1.clone(), f118({
                                                                                                    let (_, h3365) = v18;
                                                                                                    h3365
                                                                                                })?, match v25.clone() {
                                                                                                    Adt19::C0(_, h3366, _, _) => {
                                                                                                        h3366
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt19::C0(_, _, _, h3367) => {
                                                                                                        h3367
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m3368 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt26, Bytes>(f516(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt26, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m3372 {
                                                                                        Ok(v31) => {
                                                                                            let m3373 = f472(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m3373 {
                                                                                                Ok(v33) => {
                                                                                                    f418(v6, v1, v2, v3, v20, v33)
                                                                                                }
                                                                                                Err(v32) => {
                                                                                                    Ok(Err::<Adt26, Bytes>(v32))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Err(v30) => {
                                                                                            Ok(Err::<Adt26, Bytes>(v30))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v19) => {
                                                                                Ok(Err::<Adt26, Bytes>(v19))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<Adt26, Bytes>(f416(v17, v2)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Err::<Adt26, Bytes>(v13))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v9) => {
                            Ok(Err::<Adt26, Bytes>(f416(f210(f8(Str::lit("xs:")), match v9 {
                                Adt20::C0(_, h3347, _, _, _) => {
                                    h3347
                                }
                            }, f8(Str::lit(" is not translated, because an open content wildcard or an XPath assertion constrains an instance beyond its fields (XSD 1.1 Part 1, sections 3.4.2.3 and 3.13)"))), v2)))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt26, Bytes>(v7))
                }
            }
        }
    }
}

pub fn f502(v0: Str, v1: Bytes) -> Bytes {
    f210(f8(Str::lit("(")), f8(v0), f210(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f503(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: bool, v5: Adt20, v6: Adt26) -> R<Result<Adt26, Bytes>> {
    let m3374 = v0;
    if m3374 == 0 {
        Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m3374 - 1;
        if f286(v5.clone(), Str::lit("element")) {
            let m3375 = f517(v7, v1, v2, v3, v5.clone())?;
            match m3375 {
                Ok(v9) => {
                    let m3381 = f518(v5, {
                        let m3380 = {
                            let m3379 = match v9.clone() {
                                Adt28::C0(_, h3378) => {
                                    h3378
                                }
                            };
                            match m3379 {
                                Adt25::C0(h3377, _) => {
                                    h3377
                                }
                            }
                        };
                        match m3380 {
                            Adt17::C0(h3376, _) => {
                                h3376
                            }
                        }
                    });
                    match m3381 {
                        None => {
                            Ok(Ok::<Adt26, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt26, Bytes>(f506(f479(v6, Adt27::C0(match v9.clone() {
                                Adt28::C0(h3382, _) => {
                                    h3382
                                }
                            }, v10, {
                                let m3387 = {
                                    let m3386 = match v9.clone() {
                                        Adt28::C0(_, h3385) => {
                                            h3385
                                        }
                                    };
                                    match m3386 {
                                        Adt25::C0(h3384, _) => {
                                            h3384
                                        }
                                    }
                                };
                                match m3387 {
                                    Adt17::C0(_, h3383) => {
                                        h3383
                                    }
                                }
                            })), {
                                let m3390 = match v9 {
                                    Adt28::C0(_, h3389) => {
                                        h3389
                                    }
                                };
                                match m3390 {
                                    Adt25::C0(_, h3388) => {
                                        h3388
                                    }
                                }
                            })))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<Adt26, Bytes>(v8))
                }
            }
        } else {
            if f286(v5.clone(), Str::lit("any")) {
                let m3391 = f518(v5, f346());
                match m3391 {
                    None => {
                        Ok(Ok::<Adt26, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt26, Bytes>(f479(v6, Adt27::C0(f8(Str::lit("any")), v11, List::cons(f346(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f286(v5.clone(), Str::lit("group")) {
                    let m3392 = f287(v5.clone(), Str::lit("ref"));
                    match m3392 {
                        None => {
                            Ok(Err::<Adt26, Bytes>(f416(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m3394 = f140(match v5.clone() {
                                Adt20::C0(_, _, _, _, h3393) => {
                                    h3393
                                }
                            }, v12.clone())?;
                            match m3394 {
                                Ok(v14) => {
                                    let m3395 = f477(v1, Str::lit("group"), v14.clone());
                                    match m3395 {
                                        None => {
                                            Ok(Err::<Adt26, Bytes>(f416(f210(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f118({
                                                let (_, h3396) = v14.clone();
                                                h3396
                                            })?;
                                            let m3397 = f518(v5, v16.clone());
                                            match m3397 {
                                                None => {
                                                    Ok(Ok::<Adt26, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt26, Bytes>(f479(v6, Adt27::C0({
                                                        let (_, h3398) = v14;
                                                        h3398
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt26, Bytes>(f416(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f286(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f286(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f286(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f519(v5.clone()) {
                                f285(f286(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m3399 = f414(v5)?;
                            match m3399 {
                                Ok(v19) => {
                                    f472(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt26, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt20::C0(_, h3400, _, _, _) => {
                                    h3400
                                }
                            };
                            let v21: u64 = f520(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a3401 = v21;
                                let a3402 = 1u64;
                                if nat_eq(a3401, a3402) {
                                    Bytes::lit(&[])
                                } else {
                                    f90(v21)?
                                }
                            };
                            let v23: Bytes = f210(v2, f118(v20.clone())?, v22.clone());
                            let m3403 = f417(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m3403 {
                                Ok(v25) => {
                                    let m3404 = f518(v5, v23.clone());
                                    match m3404 {
                                        None => {
                                            Ok(Ok::<Adt26, Bytes>(f521(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt26, Bytes>(f506(f479(f521(v6, v20.clone())?, Adt27::C0({
                                                let a3405 = v20;
                                                let a3406 = v22;
                                                append_bytes(a3405, a3406)
                                            }, v26, List::cons(v23, List::<Bytes>::nil()))), v25)))
                                        }
                                    }
                                }
                                Err(v24) => {
                                    Ok(Err::<Adt26, Bytes>(v24))
                                }
                            }
                        }
                    } else {
                        Ok(Ok::<Adt26, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f504(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3407 = v0;
    match m3407.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3408 = v1;
            match m3408.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f210(f8(Str::lit("    {name: ")), v2, f210(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f504(v3, v5))
                }
            }
        }
    }
}

pub fn f505(p3414: List<Adt19>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt19> {
    let m3413 = p3414;
    {
        let mut m3413 = m3413;
        loop {
            let step = {
                let v0 = m3413;
                let m3409 = v0;
                match m3409.uncons() {
                    None => {
                        Err::<List<Adt19>, Option<Adt19>>(None::<Adt19>)
                    }
                    Some((v4, v5)) => {
                        if if f32(match v4.clone() {
                            Adt19::C0(h3410, _, _, _) => {
                                h3410
                            }
                        }, v1.clone()) {
                            if f32(match v4.clone() {
                                Adt19::C0(_, h3411, _, _) => {
                                    h3411
                                }
                            }, v2.clone()) {
                                f32(match v4.clone() {
                                    Adt19::C0(_, _, h3412, _) => {
                                        h3412
                                    }
                                }, v3.clone())
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt19>, Option<Adt19>>(Some(v4))
                        } else {
                            Ok::<List<Adt19>, Option<Adt19>>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(v0: Adt26, v1: List<Adt21>) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(h3415, _, _, _, _) => {
            h3415
        }
    }, f343(match v0.clone() {
        Adt26::C0(_, h3416, _, _, _) => {
            h3416
        }
    }, v1), match v0.clone() {
        Adt26::C0(_, _, h3417, _, _) => {
            h3417
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3418, _) => {
            h3418
        }
    }, match v0 {
        Adt26::C0(_, _, _, _, h3419) => {
            h3419
        }
    })
}

pub fn f507(v0: List<Adt27>, v1: List<Adt27>) -> List<Adt27> {
    let m3420 = v0;
    match m3420.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f507(v3, v1))
        }
    }
}

pub fn f508(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3465 = {
            let a3441 = {
                let a3429 = {
                    let a3423 = {
                        let a3421 = v1;
                        let a3422 = 128u8;
                        fixed_u8::bit_and(a3421, a3422)
                    };
                    let a3424 = 128u8;
                    if equal(a3423, a3424) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3430 = {
                    let a3427 = {
                        let a3425 = v1;
                        let a3426 = 64u8;
                        fixed_u8::bit_and(a3425, a3426)
                    };
                    let a3428 = 64u8;
                    if equal(a3427, a3428) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3429, a3430)?
            };
            let a3442 = {
                let a3439 = {
                    let a3433 = {
                        let a3431 = v1;
                        let a3432 = 32u8;
                        fixed_u8::bit_and(a3431, a3432)
                    };
                    let a3434 = 32u8;
                    if equal(a3433, a3434) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3440 = {
                    let a3437 = {
                        let a3435 = v1;
                        let a3436 = 16u8;
                        fixed_u8::bit_and(a3435, a3436)
                    };
                    let a3438 = 16u8;
                    if equal(a3437, a3438) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3439, a3440)?
            };
            nat_add(a3441, a3442)?
        };
        let a3466 = {
            let a3463 = {
                let a3451 = {
                    let a3445 = {
                        let a3443 = v1;
                        let a3444 = 8u8;
                        fixed_u8::bit_and(a3443, a3444)
                    };
                    let a3446 = 8u8;
                    if equal(a3445, a3446) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3452 = {
                    let a3449 = {
                        let a3447 = v1;
                        let a3448 = 4u8;
                        fixed_u8::bit_and(a3447, a3448)
                    };
                    let a3450 = 4u8;
                    if equal(a3449, a3450) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3451, a3452)?
            };
            let a3464 = {
                let a3461 = {
                    let a3455 = {
                        let a3453 = v1;
                        let a3454 = 2u8;
                        fixed_u8::bit_and(a3453, a3454)
                    };
                    let a3456 = 2u8;
                    if equal(a3455, a3456) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3462 = {
                    let a3459 = {
                        let a3457 = v1;
                        let a3458 = 1u8;
                        fixed_u8::bit_and(a3457, a3458)
                    };
                    let a3460 = 1u8;
                    if equal(a3459, a3460) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3461, a3462)?
            };
            nat_add(a3463, a3464)?
        };
        nat_add(a3465, a3466)?
    };
    let a3467 = 48u64;
    let a3468 = v2;
    if if nat_le(a3467, a3468) {
        let a3469 = v2;
        let a3470 = 57u64;
        nat_le(a3469, a3470)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3471 = 97u64;
        let a3472 = v2;
        if nat_le(a3471, a3472) {
            let a3473 = v2;
            let a3474 = 102u64;
            Ok(nat_le(a3473, a3474))
        } else {
            Ok(false)
        }
    }
}

pub fn f509() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f510(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3519 = {
            let a3495 = {
                let a3483 = {
                    let a3477 = {
                        let a3475 = v1;
                        let a3476 = 128u8;
                        fixed_u8::bit_and(a3475, a3476)
                    };
                    let a3478 = 128u8;
                    if equal(a3477, a3478) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3484 = {
                    let a3481 = {
                        let a3479 = v1;
                        let a3480 = 64u8;
                        fixed_u8::bit_and(a3479, a3480)
                    };
                    let a3482 = 64u8;
                    if equal(a3481, a3482) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3483, a3484)?
            };
            let a3496 = {
                let a3493 = {
                    let a3487 = {
                        let a3485 = v1;
                        let a3486 = 32u8;
                        fixed_u8::bit_and(a3485, a3486)
                    };
                    let a3488 = 32u8;
                    if equal(a3487, a3488) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3494 = {
                    let a3491 = {
                        let a3489 = v1;
                        let a3490 = 16u8;
                        fixed_u8::bit_and(a3489, a3490)
                    };
                    let a3492 = 16u8;
                    if equal(a3491, a3492) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3493, a3494)?
            };
            nat_add(a3495, a3496)?
        };
        let a3520 = {
            let a3517 = {
                let a3505 = {
                    let a3499 = {
                        let a3497 = v1;
                        let a3498 = 8u8;
                        fixed_u8::bit_and(a3497, a3498)
                    };
                    let a3500 = 8u8;
                    if equal(a3499, a3500) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3506 = {
                    let a3503 = {
                        let a3501 = v1;
                        let a3502 = 4u8;
                        fixed_u8::bit_and(a3501, a3502)
                    };
                    let a3504 = 4u8;
                    if equal(a3503, a3504) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3505, a3506)?
            };
            let a3518 = {
                let a3515 = {
                    let a3509 = {
                        let a3507 = v1;
                        let a3508 = 2u8;
                        fixed_u8::bit_and(a3507, a3508)
                    };
                    let a3510 = 2u8;
                    if equal(a3509, a3510) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3516 = {
                    let a3513 = {
                        let a3511 = v1;
                        let a3512 = 1u8;
                        fixed_u8::bit_and(a3511, a3512)
                    };
                    let a3514 = 1u8;
                    if equal(a3513, a3514) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3515, a3516)?
            };
            nat_add(a3517, a3518)?
        };
        nat_add(a3519, a3520)?
    };
    let a3521 = v2;
    let a3522 = 57u64;
    if nat_le(a3521, a3522) {
        let a3523 = v2;
        let a3524 = 48u64;
        Ok(nat_sub(a3523, a3524))
    } else {
        let a3525 = v2;
        let a3526 = 87u64;
        Ok(nat_sub(a3525, a3526))
    }
}

pub fn f511(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f441({
        let (_, h3527) = {
            let (h3528, _) = f522(Fn22::F523, {
                let a3530 = {
                    let a3529 = v0.clone();
                    length_list(a3529)
                };
                let a3531 = 1u64;
                nat_add(a3530, a3531)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3528
        };
        h3527
    }))
}

pub fn f512(v0: Fn21, p3536: u64, p3537: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m3535 = (p3536, p3537);
    {
        let mut m3535 = m3535;
        loop {
            let step = {
                let (v1, v2) = m3535;
                let m3532 = v1;
                if m3532 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3532 - 1;
                    let m3534 = {
                        let c3533: Fn21 = v0.clone();
                        c3533.apply(v2.clone())?
                    };
                    match m3534 {
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
                Ok(next) => { m3535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f524(v0)
}

pub fn f514(v0: u64) -> Bytes {
    f46({
        let a3538 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3538)
    }, v0, 1u64)
}

pub fn f515(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3539 = v1;
    match m3539.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3540 = f405(v0, v3.clone())?;
            match m3540 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f210(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f115(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f210(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m3541 = f515(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3541 {
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

pub fn f516(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(h3542, _, _, _, _) => {
            h3542
        }
    }, match v0 {
        Adt26::C0(_, h3543, _, _, _) => {
            h3543
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f517(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20) -> R<Result<Adt28, Bytes>> {
    let m3544 = v0;
    if m3544 == 0 {
        Ok(Err::<Adt28, Bytes>(f416(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3544 - 1;
        let m3545 = f287(v4.clone(), Str::lit("ref"));
        match m3545 {
            None => {
                let m3556 = f287(v4.clone(), Str::lit("name"));
                match m3556 {
                    None => {
                        Ok(Err::<Adt28, Bytes>(f416(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m3559 = f412(v5, v1, {
                            let a3557 = v2;
                            let a3558 = f118(v12.clone())?;
                            append_bytes(a3557, a3558)
                        }, v3, v4)?;
                        match m3559 {
                            Ok(v14) => {
                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(v12, v14)))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt28, Bytes>(v13))
                            }
                        }
                    }
                }
            }
            Some(v6) => {
                let m3547 = f140(match v4 {
                    Adt20::C0(_, _, _, _, h3546) => {
                        h3546
                    }
                }, v6.clone())?;
                match m3547 {
                    Ok(v8) => {
                        let m3548 = f477(v1.clone(), Str::lit("element"), v8.clone());
                        match m3548 {
                            None => {
                                Ok(Err::<Adt28, Bytes>(f416(f210(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f464(match v9.clone() {
                                    Adt19::C0(_, _, _, h3549) => {
                                        h3549
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f525(match v1.clone() {
                                        Adt18::C0(h3550, _) => {
                                            h3550
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt28, Bytes>(f416(f210(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m3554 = f412(v5, v1, f118({
                                        let (_, h3551) = v8.clone();
                                        h3551
                                    })?, match v9.clone() {
                                        Adt19::C0(_, h3552, _, _) => {
                                            h3552
                                        }
                                    }, match v9 {
                                        Adt19::C0(_, _, _, h3553) => {
                                            h3553
                                        }
                                    })?;
                                    match m3554 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt28, Bytes>(Adt28::C0({
                                                let (_, h3555) = v8;
                                                h3555
                                            }, v11)))
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt28, Bytes>(v10))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt28, Bytes>(f416(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f518(v0: Adt20, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f290(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f32(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f32(v2, f8(Str::lit("1"))) {
            if f464(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f502(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f502(Str::lit("List"), v1))
        }
    }
}

pub fn f519(v0: Adt20) -> bool {
    if f32(f290(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f32(f290(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f520(v0: Adt26, v1: Bytes) -> R<u64> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        let a3561 = match v0 {
            Adt26::C0(_, _, h3560, _, _) => {
                h3560
            }
        };
        let a3562 = 1u64;
        nat_add(a3561, a3562)
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            let a3564 = match v0 {
                Adt26::C0(_, _, _, h3563, _) => {
                    h3563
                }
            };
            let a3565 = 1u64;
            nat_add(a3564, a3565)
        } else {
            let a3567 = match v0 {
                Adt26::C0(_, _, _, _, h3566) => {
                    h3566
                }
            };
            let a3568 = 1u64;
            nat_add(a3567, a3568)
        }
    }
}

pub fn f521(v0: Adt26, v1: Bytes) -> R<Adt26> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt26::C0(match v0.clone() {
            Adt26::C0(h3569, _, _, _, _) => {
                h3569
            }
        }, match v0.clone() {
            Adt26::C0(_, h3570, _, _, _) => {
                h3570
            }
        }, {
            let a3572 = match v0.clone() {
                Adt26::C0(_, _, h3571, _, _) => {
                    h3571
                }
            };
            let a3573 = 1u64;
            nat_add(a3572, a3573)?
        }, match v0.clone() {
            Adt26::C0(_, _, _, h3574, _) => {
                h3574
            }
        }, match v0 {
            Adt26::C0(_, _, _, _, h3575) => {
                h3575
            }
        }))
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            Ok(Adt26::C0(match v0.clone() {
                Adt26::C0(h3576, _, _, _, _) => {
                    h3576
                }
            }, match v0.clone() {
                Adt26::C0(_, h3577, _, _, _) => {
                    h3577
                }
            }, match v0.clone() {
                Adt26::C0(_, _, h3578, _, _) => {
                    h3578
                }
            }, {
                let a3580 = match v0.clone() {
                    Adt26::C0(_, _, _, h3579, _) => {
                        h3579
                    }
                };
                let a3581 = 1u64;
                nat_add(a3580, a3581)?
            }, match v0 {
                Adt26::C0(_, _, _, _, h3582) => {
                    h3582
                }
            }))
        } else {
            Ok(Adt26::C0(match v0.clone() {
                Adt26::C0(h3583, _, _, _, _) => {
                    h3583
                }
            }, match v0.clone() {
                Adt26::C0(_, h3584, _, _, _) => {
                    h3584
                }
            }, match v0.clone() {
                Adt26::C0(_, _, h3585, _, _) => {
                    h3585
                }
            }, match v0.clone() {
                Adt26::C0(_, _, _, h3586, _) => {
                    h3586
                }
            }, {
                let a3588 = match v0 {
                    Adt26::C0(_, _, _, _, h3587) => {
                        h3587
                    }
                };
                let a3589 = 1u64;
                nat_add(a3588, a3589)?
            }))
        }
    }
}

pub fn f522(v0: Fn22, p3594: u64, p3595: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3593 = (p3594, p3595);
    {
        let mut m3593 = m3593;
        loop {
            let step = {
                let (v1, v2) = m3593;
                let m3590 = v1;
                if m3590 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3590 - 1;
                    let m3592 = {
                        let c3591: Fn22 = v0.clone();
                        c3591.apply(v2.clone())?
                    };
                    match m3592 {
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
                Ok(next) => { m3593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f526(v0)
}

pub fn f524(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3596, _) = v0;
        h3596
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3597) = v0;
        h3597
    };
    let m3598 = v1;
    match m3598.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3599 = v4;
            match m3599.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f527(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f525(p3608: List<Adt19>, v1: (Bytes, Bytes)) -> R<bool> {
    let m3607 = p3608;
    {
        let mut m3607 = m3607;
        loop {
            let step = {
                let v0 = m3607;
                let m3600 = v0;
                match m3600.uncons() {
                    None => {
                        Err::<List<Adt19>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f32(match v2.clone() {
                            Adt19::C0(h3601, _, _, _) => {
                                h3601
                            }
                        }, f8(Str::lit("element"))) {
                            let m3603 = f287(match v2.clone() {
                                Adt19::C0(_, _, _, h3602) => {
                                    h3602
                                }
                            }, Str::lit("substitutionGroup"));
                            match m3603 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f528({
                                        let m3606 = match v2 {
                                            Adt19::C0(_, _, _, h3605) => {
                                                h3605
                                            }
                                        };
                                        match m3606 {
                                            Adt20::C0(_, _, _, _, h3604) => {
                                                h3604
                                            }
                                        }
                                    }, f529(v4)?, v1.clone())?
                                }
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt19>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt19>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3609, _) = v0;
        h3609
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3610) = v0;
        h3610
    };
    let m3611 = v1;
    match m3611.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3612 = v4;
            match m3612.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f530(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f527(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3617 = {
            let a3615 = {
                let a3613 = v0.clone();
                length_list(a3613)
            };
            let a3616 = {
                let a3614 = v1.clone();
                length_list(a3614)
            };
            nat_add(a3615, a3616)?
        };
        let a3618 = 1u64;
        nat_add(a3617, a3618)?
    };
    Ok(f386({
        let (_, h3619) = {
            let (_, h3620) = {
                let (h3621, _) = f531(Fn23::F532, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3621
            };
            h3620
        };
        h3619
    }))
}

pub fn f528(v0: List<(Bytes, Bytes)>, p3629: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m3628 = p3629;
    {
        let mut m3628 = m3628;
        loop {
            let step = {
                let v1 = m3628;
                let m3622 = v1;
                match m3622.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m3623 = f140(v0.clone(), v3)?;
                        match m3623 {
                            Ok(v6) => {
                                if if f32({
                                    let (h3624, _) = v6;
                                    h3624
                                }, {
                                    let (h3625, _) = v2.clone();
                                    h3625
                                }) {
                                    f32({
                                        let (_, h3626) = v6;
                                        h3626
                                    }, {
                                        let (_, h3627) = v2.clone();
                                        h3627
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
                Ok(next) => { m3628 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(v0: Bytes) -> R<List<Bytes>> {
    f44(v0, 32u8)
}

pub fn f530(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3634 = {
            let a3632 = {
                let a3630 = v0.clone();
                length_list(a3630)
            };
            let a3633 = {
                let a3631 = v1.clone();
                length_list(a3631)
            };
            nat_add(a3632, a3633)?
        };
        let a3635 = 1u64;
        nat_add(a3634, a3635)?
    };
    Ok(f533({
        let (_, h3636) = {
            let (_, h3637) = {
                let (h3638, _) = f534(Fn24::F535, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3638
            };
            h3637
        };
        h3636
    }))
}

pub fn f531(v0: Fn23, p3643: u64, p3644: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3642 = (p3643, p3644);
    {
        let mut m3642 = m3642;
        loop {
            let step = {
                let (v1, v2) = m3642;
                let m3639 = v1;
                if m3639 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3639 - 1;
                    let m3641 = {
                        let c3640: Fn23 = v0.clone();
                        c3640.apply(v2.clone())
                    };
                    match m3641 {
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
                Ok(next) => { m3642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f532(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f536(v0)
}

pub fn f533(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f312(Fn8::F537, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f534(v0: Fn24, p3649: u64, p3650: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3648 = (p3649, p3650);
    {
        let mut m3648 = m3648;
        loop {
            let step = {
                let (v1, v2) = m3648;
                let m3645 = v1;
                if m3645 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3645 - 1;
                    let m3647 = {
                        let c3646: Fn24 = v0.clone();
                        c3646.apply(v2.clone())
                    };
                    match m3647 {
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
                Ok(next) => { m3648 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f535(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f538(v0)
}

pub fn f536(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3651, _) = v0.clone();
        h3651
    };
    let v2: List<(Bytes, ())> = {
        let (h3652, _) = {
            let (_, h3653) = v0.clone();
            h3653
        };
        h3652
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3654) = {
            let (_, h3655) = v0;
            h3655
        };
        h3654
    };
    let m3656 = v1.clone();
    match m3656.uncons() {
        None => {
            let m3657 = v2;
            match m3657.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3658 = v2.clone();
            match m3658.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f539({
                        let (h3659, _) = v8.clone();
                        h3659
                    }, {
                        let (h3660, _) = v6.clone();
                        h3660
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

pub fn f537(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f538(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3661, _) = v0.clone();
        h3661
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3662, _) = {
            let (_, h3663) = v0.clone();
            h3663
        };
        h3662
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3664) = {
            let (_, h3665) = v0;
            h3665
        };
        h3664
    };
    let m3666 = v1.clone();
    match m3666.uncons() {
        None => {
            let m3667 = v2;
            match m3667.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3668 = v2.clone();
            match m3668.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f539({
                        let (h3669, _) = v8.clone();
                        h3669
                    }, {
                        let (h3670, _) = v6.clone();
                        h3670
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

pub fn f539(v0: Bytes, v1: Bytes) -> bool {
    let a3675 = {
        let a3671 = v0;
        let a3672 = v1;
        compare_bytes(a3671, a3672)
    };
    let a3676 = {
        let a3673 = Bytes::lit(&[]);
        let a3674 = Bytes::lit(&[0x01]);
        compare_bytes(a3673, a3674)
    };
    equal(a3675, a3676)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
