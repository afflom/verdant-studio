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
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt13 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt13>),
    C4(std::rc::Rc<Adt13>),
    C5,
    C6(std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C7(Bytes, std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Bytes>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Adt15, List<Adt13>, List<Adt13>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt18>),
    C5(std::rc::Rc<Adt18>),
    C6,
    C7(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C8(Bytes, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u64, Adt21),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, Option<u64>),
    C1(u64),
    C2(Adt22, List<u64>, u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt23 {
    C0(bool, bool, Bytes, List<Bytes>, List<List<Bytes>>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(List<Bytes>),
    C1(Bytes),
    C2(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>, Adt26, u64),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Bytes, std::rc::Rc<Adt26>, std::rc::Rc<Adt26>),
    C1(Bytes, Bytes, std::rc::Rc<Adt26>),
    C2(Bytes),
    C3(std::rc::Rc<Adt26>, std::rc::Rc<Adt26>),
    C4,
}

#[derive(Clone)]
pub enum Adt27 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Bytes, Adt18),
    C1(Bytes),
    C2,
}

#[derive(Clone)]
pub enum Adt29 {
    C0(List<Adt30>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Bytes, Bytes, Bytes, Adt31),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Bytes, Bytes, Adt26, Adt26, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt33, u64),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Bytes, List<Adt28>),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Adt37, List<Adt32>),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(List<Adt39>, List<Adt32>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(List<Bytes>, List<Adt28>),
}

#[derive(Clone)]
pub enum Adt41 {
    C0(Bytes, Adt26, Adt26, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt42 {
    C0(Bytes, Adt36),
}

#[derive(Clone)]
pub enum Fn0 {
    F64(u8, Bytes),
    F162(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn0::F162(k0) => f162(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F94,
    F209,
    F301,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F94 => f94(p0, p1),
            Fn1::F209 => f209(p0, p1),
            Fn1::F301 => f301(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F176(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F176(k0) => f176(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F288(Bytes, u64, Bytes, u64),
    F369(Bytes),
    F370(u64, Bytes),
    F505(Bytes),
    F506(Bytes),
    F578(Bytes),
    F614(Bytes),
    F625(Bytes, Bytes),
    F627(Bytes),
    F684(Bytes),
    F686(Bytes, u64),
    F745(bool, u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F288(k0, k1, k2, k3) => f288(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F369(k0) => Ok(f369(k0.clone(), p0)),
            Fn3::F370(k0, k1) => f370(*k0, k1.clone(), p0),
            Fn3::F505(k0) => f505(k0.clone(), p0),
            Fn3::F506(k0) => f506(k0.clone(), p0),
            Fn3::F578(k0) => f578(k0.clone(), p0),
            Fn3::F614(k0) => f614(k0.clone(), p0),
            Fn3::F625(k0, k1) => f625(k0.clone(), k1.clone(), p0),
            Fn3::F627(k0) => f627(k0.clone(), p0),
            Fn3::F684(k0) => f684(k0.clone(), p0),
            Fn3::F686(k0, k1) => f686(k0.clone(), *k1, p0),
            Fn3::F745(k0, k1, k2) => f745(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F359(Bytes),
    F512(u64, Bytes),
    F613(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F359(k0) => f359(k0.clone(), p0),
            Fn4::F512(k0, k1) => Ok(f512(*k0, k1.clone(), p0)),
            Fn4::F613(k0) => f613(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F361,
    F830,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F361 => f361(p0, p1),
            Fn5::F830 => f830(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F364,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F364 => f364(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F386,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F386 => f386(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F389,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F389 => f389(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F391,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F391 => f391(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F394,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F394 => f394(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F413(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt27)) -> R<(u64, Adt27)> {
        match self {
            Fn11::F413(k0, k1, k2) => f413(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F450,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F450 => f450(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F453,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F453 => f453(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F504(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn14::F504(k0) => f504(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F509(Bytes),
    F784(bool, Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn15::F509(k0) => f509(k0.clone(), p0),
            Fn15::F784(k0, k1) => f784(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F516,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F516 => f516(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F518,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F518 => f518(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F535,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt34) -> R<Adt34> {
        match self {
            Fn18::F535 => f535(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F582,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F582 => f582(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F585,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F585 => f585(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F659,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F659 => f659(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F695,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F695 => f695(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F750,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F750 => f750(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F792(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F792(k0) => f792(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F809,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F809 => f809(p0),
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
    let m136 = v0.clone();
    match m136.uncons() {
        None => {
            f27(v0, v1)
        }
        Some((v2, _)) => {
            if f28(v2.clone(), f8(Str::lit("stage-engine"))) {
                f29(f30(v1)?)
            } else {
                if f28(v2, f8(Str::lit("publish"))) {
                    Ok(f31(f27(v0, v1)?))
                } else {
                    f27(v0, v1)
                }
            }
        }
    }
}

pub fn f21(v0: Adt0, v1: List<Bytes>) -> Adt0 {
    Adt0::C0(v1, match v0.clone() {
        Adt0::C0(_, h137, _, _, _, _, _, _, _, _, _) => {
            h137
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h138, _, _, _, _, _, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h139, _, _, _, _, _, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h140, _, _, _, _, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h141, _, _, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h142, _, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h143, _, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h144, _, _) => {
            h144
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h145, _) => {
            h145
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h146) => {
            h146
        }
    })
}

pub fn f22(v0: Bytes, v1: u64, v2: u64) -> R<List<Str>> {
    let m147 = v2;
    if m147 == 0 {
        Ok(List::<Str>::nil())
    } else {
        let v3 = m147 - 1;
        let m150 = {
            let a148 = v0.clone();
            let a149 = v1;
            index_bytes(a148, a149)
        };
        match m150 {
            None => {
                Ok(List::<Str>::nil())
            }
            Some(v4) => {
                Ok(List::cons(f32(v4)?, f22(v0, {
                    let a151 = v1;
                    let a152 = 1u64;
                    nat_add(a151, a152)?
                }, v3)?))
            }
        }
    }
}

pub fn f23(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h153, _, _, _, _, _, _, _, _, _, _) => {
            h153
        }
    }, match v0.clone() {
        Adt0::C0(_, h154, _, _, _, _, _, _, _, _, _) => {
            h154
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h155, _, _, _, _, _, _, _, _) => {
            h155
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h156, _, _, _, _, _) => {
            h156
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h157, _, _, _, _) => {
            h157
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h158, _, _, _) => {
            h158
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h159, _, _) => {
            h159
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h160, _) => {
            h160
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h161) => {
            h161
        }
    })
}

pub fn f24(v0: List<Bytes>) -> R<List<Str>> {
    let m162 = v0;
    match m162.uncons() {
        None => {
            Ok(List::<Str>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f4(v1)?, f24(v2)?))
        }
    }
}

pub fn f25(v0: List<(Bytes, Bytes)>) -> R<List<(Str, Str)>> {
    let m163 = v0;
    match m163.uncons() {
        None => {
            Ok(List::<(Str, Str)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f4({
                let (h164, _) = v1;
                h164
            })?, f4({
                let (_, h165) = v1;
                h165
            })?), f25(v2)?))
        }
    }
}

pub fn f26(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f33(v0.clone(), 47u8)?;
    let v2: Bytes = f34(v1);
    let a168 = {
        let a166 = v0.clone();
        length_bytes(a166)
    };
    let a169 = {
        let a167 = v2.clone();
        length_bytes(a167)
    };
    if nat_eq(a168, a169) {
        Ok(f8(Str::lit(".")))
    } else {
        Ok(f35(v0.clone(), 0u64, {
            let a174 = {
                let a170 = v0;
                length_bytes(a170)
            };
            let a175 = {
                let a172 = {
                    let a171 = v2;
                    length_bytes(a171)
                };
                let a173 = 1u64;
                nat_add(a172, a173)?
            };
            nat_sub(a174, a175)
        }))
    }
}

pub fn f27(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    f29(f36(v0, v1)?)
}

pub fn f28(v0: Bytes, v1: Bytes) -> bool {
    let a176 = v0;
    let a177 = v1;
    equal(a176, a177)
}

pub fn f29(v0: Adt11) -> R<Adt11> {
    let m179 = {
        let m178 = v0.clone();
        match m178 {
            Adt11::C0(v1) => {
                f37(v1)?
            }
            Adt11::C1(v2) => {
                f37(v2)?
            }
            Adt11::C2(v3, v4, _) => {
                f37(List::cons(v3, v4))?
            }
            Adt11::C3(v6, _) => {
                f37(f38(v6))?
            }
            Adt11::C4(_) => {
                None::<Bytes>
            }
        }
    };
    match m179 {
        None => {
            Ok(v0)
        }
        Some(v9) => {
            Ok(Adt11::C4(f39(v9)))
        }
    }
}

pub fn f30(v0: Adt1) -> R<Adt11> {
    if f40(f41(f42(), match v0.clone() {
        Adt1::C0(_, _, h180) => {
            h180
        }
    })) {
        Ok(f43(f42(), f44(f8(Str::lit("releases/latest")))))
    } else {
        let m181 = f45(v0.clone(), List::cons(f42(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil()))));
        match m181.uncons() {
            None => {
                let v4: Bytes = {
                    let m182 = f46(v0.clone(), f42())?;
                    match m182 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v3) => {
                            f47(v3, f8(Str::lit("tag_name")))
                        }
                    }
                };
                let a184 = {
                    let a183 = v4.clone();
                    length_bytes(a183)
                };
                let a185 = 0u64;
                if nat_eq(a184, a185) {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("waiting engine: the latest verdant-engine release names no tag\u{a}"))))
                } else {
                    let v5: List<Bytes> = f48(f49(), f50(v4.clone()));
                    let a187 = {
                        let a186 = f51(v5.clone(), f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?);
                        length_list(a186)
                    };
                    let a188 = 0u64;
                    if nat_eq(a187, a188) {
                        f54(v0, v4)
                    } else {
                        let m189 = f45(v0.clone(), f55(v5));
                        match m189.uncons() {
                            None => {
                                if f56(v0.clone(), f57(f58(Adt12::C1, f50(v4.clone())))) {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting ingest: the verdant-engine ")), List::cons(v4, List::cons(f8(Str::lit(" bindings are staged, and advance ingests them\u{a}")), List::<Bytes>::nil()))))))
                                } else {
                                    f59(v0, v4)
                                }
                            }
                            Some((v6, v7)) => {
                                Ok(Adt11::C0(List::cons(v6, v7)))
                            }
                        }
                    }
                }
            }
            Some((v1, v2)) => {
                Ok(Adt11::C0(List::cons(v1, v2)))
            }
        }
    }
}

pub fn f31(v0: Adt11) -> Adt11 {
    let m190 = v0;
    match m190 {
        Adt11::C0(v3) => {
            Adt11::C0(v3)
        }
        Adt11::C1(v4) => {
            Adt11::C1(v4)
        }
        Adt11::C2(v5, v6, v7) => {
            Adt11::C2(v5, v6, v7)
        }
        Adt11::C3(v1, v2) => {
            Adt11::C3(List::cons((f8(Str::lit(".github/workflows/track.yml")), f60()), v1), v2)
        }
        Adt11::C4(v8) => {
            Adt11::C4(v8)
        }
    }
}

pub fn f32(v0: u8) -> R<Str> {
    let m193 = {
        let a191 = f61();
        let a192 = f62(v0)?;
        index_list(a191, a192)
    };
    match m193 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f33(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a194 = v0.clone();
        length_bytes(a194)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h195, _) = f63(Fn0::F64(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h195
    };
    let v4: u64 = {
        let (h196, _) = {
            let (_, h197) = v3.clone();
            h197
        };
        h196
    };
    Ok(f65(List::cons(f35(v0, v4, {
        let a198 = v2;
        let a199 = v4;
        nat_sub(a198, a199)
    }), {
        let (_, h200) = {
            let (_, h201) = v3;
            h201
        };
        h200
    })))
}

pub fn f34(p205: List<Bytes>) -> Bytes {
    let m204 = p205;
    {
        let mut m204 = m204;
        loop {
            let step = {
                let v0 = m204;
                let m202 = v0;
                match m202.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m203 = v2.clone();
                        match m203.uncons() {
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
                Ok(next) => { m204 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m209 = {
        let a206 = v0;
        let a207 = v1;
        let a208 = v2;
        slice_bytes(a206, a207, a208)
    };
    match m209 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f36(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m210 = v0;
    match m210.uncons() {
        None => {
            Ok(Adt11::C4(f66()))
        }
        Some((v2, v3)) => {
            if f28(v2.clone(), f8(Str::lit("agenda"))) {
                f67(v1)
            } else {
                if f28(v2.clone(), f8(Str::lit("build"))) {
                    Ok(f68(v1))
                } else {
                    if f28(v2.clone(), f8(Str::lit("check"))) {
                        f69(v1)
                    } else {
                        if f28(v2.clone(), f8(Str::lit("next"))) {
                            f70(v1, v3)
                        } else {
                            if f28(v2.clone(), f8(Str::lit("publish"))) {
                                f71(v1)
                            } else {
                                let m211 = v3;
                                match m211.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f66()))
                                    }
                                    Some((v4, v5)) => {
                                        if f72(v2.clone(), f8(Str::lit("read-"))) {
                                            Ok(Adt11::C4(f8(Str::lit("read-webidl, read-xml, read-xsd, read-xsd-components and read-yaml run in verdant_readers, the command of the readers project\u{a}"))))
                                        } else {
                                            if f72(v2.clone(), f8(Str::lit("stage-"))) {
                                                f73(v2, v4, v5, v1)
                                            } else {
                                                f74(v2, v4, v5, v1)
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

pub fn f37(p214: List<Bytes>) -> R<Option<Bytes>> {
    let m213 = p214;
    {
        let mut m213 = m213;
        loop {
            let step = {
                let v0 = m213;
                let m212 = v0;
                match m212.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f75(v1.clone())? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m215 = v0;
    match m215.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h216, _) = v1;
                h216
            }, f38(v2))
        }
    }
}

pub fn f39(v0: Bytes) -> Bytes {
    let a221 = {
        let a217 = Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ");
        utf8_encode(a217)
    };
    let a222 = {
        let a219 = v0;
        let a220 = {
            let a218 = Str::lit(" is not\u{a}");
            utf8_encode(a218)
        };
        append_bytes(a219, a220)
    };
    append_bytes(a221, a222)
}

pub fn f40(v0: bool) -> bool {
    !v0
}

pub fn f41(v0: Bytes, p225: List<Bytes>) -> bool {
    let m224 = p225;
    {
        let mut m224 = m224;
        loop {
            let step = {
                let v1 = m224;
                let m223 = v1;
                match m223.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f28(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42() -> Bytes {
    f8(Str::lit("target/engine/release.json"))
}

pub fn f43(v0: Bytes, v1: Bytes) -> Adt11 {
    Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v0.clone(), List::cons(v1, List::<Bytes>::nil()))))), v0)
}

pub fn f44(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://api.github.com/repos/afflom/verdant-engine/")), List::cons(v0, List::<Bytes>::nil())))
}

pub fn f45(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m226 = v1;
    match m226.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m228 = f76(match v0.clone() {
                Adt1::C0(h227, _, _) => {
                    h227
                }
            }, v2.clone());
            match m228 {
                None => {
                    List::cons(v2, f45(v0, v3))
                }
                Some(_) => {
                    f45(v0, v3)
                }
            }
        }
    }
}

pub fn f46(v0: Adt1, v1: Bytes) -> R<Option<Adt13>> {
    f77(f53(v0, v1))
}

pub fn f47(v0: Adt13, v1: Bytes) -> Bytes {
    let m229 = f78(f79(v0, v1));
    match m229 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f48(v0: List<Adt12>, v1: Bytes) -> List<Bytes> {
    let m230 = v0;
    match m230.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f48(v3, v1))
        }
    }
}

pub fn f49() -> List<Adt12> {
    List::cons(Adt12::C0, List::cons(Adt12::C1, List::cons(Adt12::C2, List::cons(Adt12::C3, List::<Adt12>::nil()))))
}

pub fn f50(v0: Bytes) -> Bytes {
    if f72(v0.clone(), f8(Str::lit("v"))) {
        f35(v0.clone(), 1u64, {
            let a232 = {
                let a231 = v0;
                length_bytes(a231)
            };
            let a233 = 1u64;
            nat_sub(a232, a233)
        })
    } else {
        v0
    }
}

pub fn f51(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m234 = v0;
    match m234.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f51(v3, v1)
            } else {
                List::cons(v2, f51(v3, v1))
            }
        }
    }
}

pub fn f52(v0: Bytes) -> R<List<Bytes>> {
    f80(f81(v0)?, f8(Str::lit("id = \"")))
}

pub fn f53(v0: Adt1, v1: Bytes) -> Bytes {
    let m236 = f76(match v0 {
        Adt1::C0(h235, _, _) => {
            h235
        }
    }, v1);
    match m236 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m237 = v2;
            match m237 {
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

pub fn f54(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let v2: List<Adt13> = f82(v0, f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f83(v2.clone(), f50(v1.clone()));
    if f84(f85(v2), f85(v3.clone())) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" is pinned, and every admission of its registers cites it\u{a}")), List::<Bytes>::nil()))))))
    } else {
        Ok(Adt11::C3(List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(v3))))?), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("re-cited the admissions of the verdant-engine registers at ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
    }
}

pub fn f55(v0: List<Bytes>) -> List<Bytes> {
    let m238 = v0;
    match m238.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f57(v1), f55(v2))
        }
    }
}

pub fn f56(v0: Adt1, v1: Bytes) -> bool {
    let m240 = f76(match v0 {
        Adt1::C0(h239, _, _) => {
            h239
        }
    }, v1);
    match m240 {
        None => {
            false
        }
        Some(v2) => {
            let m241 = v2;
            match m241 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f57(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f58(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f88(v0), List::cons(v1, List::<Bytes>::nil())))
}

pub fn f59(v0: Adt1, v1: Bytes) -> R<Adt11> {
    if f40(f41(f89(), match v0.clone() {
        Adt1::C0(_, _, h242) => {
            h242
        }
    })) {
        Ok(f43(f89(), f44(f7(List::cons(f8(Str::lit("git/ref/tags/")), List::cons(v1, List::<Bytes>::nil()))))))
    } else {
        let m243 = f45(v0.clone(), List::cons(f89(), List::<Bytes>::nil()));
        match m243.uncons() {
            None => {
                let v6: Adt13 = {
                    let m245 = f79({
                        let m244 = f46(v0.clone(), f89())?;
                        match m244 {
                            None => {
                                Adt13::C5
                            }
                            Some(v4) => {
                                v4
                            }
                        }
                    }, f8(Str::lit("object")));
                    match m245 {
                        None => {
                            Adt13::C5
                        }
                        Some(v5) => {
                            v5
                        }
                    }
                };
                if f28(f47(v6.clone(), f8(Str::lit("type"))), f8(Str::lit("tag"))) {
                    if f40(f41(f90(), match v0.clone() {
                        Adt1::C0(_, _, h246) => {
                            h246
                        }
                    })) {
                        Ok(f43(f90(), f44(f7(List::cons(f8(Str::lit("git/tags/")), List::cons(f47(v6, f8(Str::lit("sha"))), List::<Bytes>::nil()))))))
                    } else {
                        let m247 = f45(v0.clone(), List::cons(f90(), List::<Bytes>::nil()));
                        match m247.uncons() {
                            None => {
                                f91(v0.clone(), v1, {
                                    let m249 = f79({
                                        let m248 = f46(v0, f90())?;
                                        match m248 {
                                            None => {
                                                Adt13::C5
                                            }
                                            Some(v9) => {
                                                v9
                                            }
                                        }
                                    }, f8(Str::lit("object")));
                                    match m249 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v10) => {
                                            f47(v10, f8(Str::lit("sha")))
                                        }
                                    }
                                })
                            }
                            Some((v7, v8)) => {
                                Ok(Adt11::C0(List::cons(v7, v8)))
                            }
                        }
                    }
                } else {
                    f91(v0, v1, f47(v6, f8(Str::lit("sha"))))
                }
            }
            Some((v2, v3)) => {
                Ok(Adt11::C0(List::cons(v2, v3)))
            }
        }
    }
}

pub fn f60() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Engine.lex.tex. Do not edit.\u{a}# REQ-ENGINE-TRACKING: Verdant Studio tracks Verdant Engine releases. Every hour this runs\u{a}# operate stage-engine, which stages the latest release's four registers when they are new\u{a}# (advance then ingests them by sha256), and re-cites their admissions once they are ingested.\u{a}name: track\u{a}\u{a}on:\u{a}  schedule:\u{a}    - cron: '17 * * * *'\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}  actions: write\u{a}\u{a}jobs:\u{a}  track:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 5\u{a}    steps:\u{a}      - env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}        run: gh workflow run operate.yml --repo \"$GITHUB_REPOSITORY\" --ref main -f args=stage-engine\u{a}"))
}

pub fn f61() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f62(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a294 = {
        let a270 = {
            let a258 = {
                let a252 = {
                    let a250 = v1;
                    let a251 = 128u8;
                    fixed_u8::bit_and(a250, a251)
                };
                let a253 = 128u8;
                if equal(a252, a253) {
                    128u64
                } else {
                    0u64
                }
            };
            let a259 = {
                let a256 = {
                    let a254 = v1;
                    let a255 = 64u8;
                    fixed_u8::bit_and(a254, a255)
                };
                let a257 = 64u8;
                if equal(a256, a257) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a258, a259)?
        };
        let a271 = {
            let a268 = {
                let a262 = {
                    let a260 = v1;
                    let a261 = 32u8;
                    fixed_u8::bit_and(a260, a261)
                };
                let a263 = 32u8;
                if equal(a262, a263) {
                    32u64
                } else {
                    0u64
                }
            };
            let a269 = {
                let a266 = {
                    let a264 = v1;
                    let a265 = 16u8;
                    fixed_u8::bit_and(a264, a265)
                };
                let a267 = 16u8;
                if equal(a266, a267) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a268, a269)?
        };
        nat_add(a270, a271)?
    };
    let a295 = {
        let a292 = {
            let a280 = {
                let a274 = {
                    let a272 = v1;
                    let a273 = 8u8;
                    fixed_u8::bit_and(a272, a273)
                };
                let a275 = 8u8;
                if equal(a274, a275) {
                    8u64
                } else {
                    0u64
                }
            };
            let a281 = {
                let a278 = {
                    let a276 = v1;
                    let a277 = 4u8;
                    fixed_u8::bit_and(a276, a277)
                };
                let a279 = 4u8;
                if equal(a278, a279) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a280, a281)?
        };
        let a293 = {
            let a290 = {
                let a284 = {
                    let a282 = v1;
                    let a283 = 2u8;
                    fixed_u8::bit_and(a282, a283)
                };
                let a285 = 2u8;
                if equal(a284, a285) {
                    2u64
                } else {
                    0u64
                }
            };
            let a291 = {
                let a288 = {
                    let a286 = v1;
                    let a287 = 1u8;
                    fixed_u8::bit_and(a286, a287)
                };
                let a289 = 1u8;
                if equal(a288, a289) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a290, a291)?
        };
        nat_add(a292, a293)?
    };
    nat_add(a294, a295)
}

pub fn f63(v0: Fn0, p300: u64, p301: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m299 = (p300, p301);
    {
        let mut m299 = m299;
        loop {
            let step = {
                let (v1, v2) = m299;
                let m296 = v1;
                if m296 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m296 - 1;
                    let m298 = {
                        let c297: Fn0 = v0.clone();
                        c297.apply(v2.clone())?
                    };
                    match m298 {
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
                Ok(next) => { m299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f92(v1, v0, v2)
}

pub fn f65(v0: List<Bytes>) -> List<Bytes> {
    f93(Fn1::F94, List::<Bytes>::nil(), v0)
}

pub fn f66() -> Bytes {
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f95(f96(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f67(v0: Adt1) -> R<Adt11> {
    let m302 = f97(v0.clone());
    match m302.uncons() {
        None => {
            let m303 = f98(f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f99(v0.clone())?);
            match m303.uncons() {
                None => {
                    let v5: Adt14 = f100(v0.clone())?;
                    let v6: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f98(f101(f102(match v5.clone() {
                        Adt14::C0(h304, _, _) => {
                            h304
                        }
                    }, f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(v6.clone())), f105(f102(match v5.clone() {
                        Adt14::C0(h305, _, _) => {
                            h305
                        }
                    }, f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f106(v5.clone(), v6, f107(v5, f108(v0.clone(), f109(v0)?)?))?)))))
                }
                Some((v3, v4)) => {
                    Ok(Adt11::C0(List::cons(v3, v4)))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt11::C1(List::cons(v1, v2)))
        }
    }
}

pub fn f68(v0: Adt1) -> Adt11 {
    if f40(f41(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h306) => {
            h306
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f110(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f40(f41(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h307) => {
                h307
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f110(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f69(v0: Adt1) -> R<Adt11> {
    let m308 = f45(v0.clone(), f111());
    match m308.uncons() {
        None => {
            let m309 = f112(v0.clone(), f111())?;
            match m309.uncons() {
                None => {
                    let m312 = f113(match v0.clone() {
                        Adt1::C0(_, h310, _) => {
                            h310
                        }
                    }, f98(f114(), f115(match v0.clone() {
                        Adt1::C0(_, h311, _) => {
                            h311
                        }
                    }, false)));
                    match m312.uncons() {
                        None => {
                            let m314 = f45(v0.clone(), f115(match v0.clone() {
                                Adt1::C0(_, h313, _) => {
                                    h313
                                }
                            }, true));
                            match m314.uncons() {
                                None => {
                                    let m315 = f45(v0.clone(), f116());
                                    match m315.uncons() {
                                        None => {
                                            let m316 = f117(v0.clone())?;
                                            match m316 {
                                                None => {
                                                    let m318 = f118(v0.clone(), f115(match v0.clone() {
                                                        Adt1::C0(_, h317, _) => {
                                                            h317
                                                        }
                                                    }, true))?;
                                                    match m318 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f119(Adt16::C0(Adt15::C0(f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f120(v0.clone())?, f121(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f52(f53(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f122(f82(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f115(match v0 {
                                                                Adt1::C0(_, h319, _) => {
                                                                    h319
                                                                }
                                                            }, true)))?;
                                                            let (h320, _) = v13;
                                                            if h320 {
                                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h321) = v13;
                                                                    h321
                                                                }))
                                                            } else {
                                                                Ok(Adt11::C4({
                                                                    let (_, h322) = v13;
                                                                    h322
                                                                }))
                                                            }
                                                        }
                                                        Some(v12) => {
                                                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: classical choice is an assumption, and ")), List::cons(v12, List::cons(f8(Str::lit(" depends on it\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                    }
                                                }
                                                Some(v11) => {
                                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: a negated guard in a generated workflow is one command, because `if !` negates only the first command of a chain: ")), List::cons(v11, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                                }
                                            }
                                        }
                                        Some((v9, v10)) => {
                                            Ok(Adt11::C0(List::cons(v9, v10)))
                                        }
                                    }
                                }
                                Some((v7, v8)) => {
                                    Ok(Adt11::C0(List::cons(v7, v8)))
                                }
                            }
                        }
                        Some((v5, v6)) => {
                            Ok(Adt11::C1(List::cons(v5, v6)))
                        }
                    }
                }
                Some((v3, _)) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("register is not JSON: ")), List::cons(v3, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f70(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h323, _) => {
            h323
        }
    }, f124())) {
        Ok(Adt11::C1(List::cons(f124(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f125(f126(f124(), f127(match v0.clone() {
            Adt1::C0(_, h324, _) => {
                h324
            }
        }, f124())))?;
        let m325 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m325.uncons() {
            None => {
                let m326 = f128(v0.clone(), f129(f81(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f130(v2, v1.clone()))?;
                match m326 {
                    None => {
                        f131(v0, v1)
                    }
                    Some(v5) => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("ingest-authority ")), List::cons(v5, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f71(v0: Adt1) -> R<Adt11> {
    let m327 = f97(v0.clone());
    match m327.uncons() {
        None => {
            let m328 = f98(f45(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f132(), List::<Bytes>::nil())))), f99(v0.clone())?);
            match m328.uncons() {
                None => {
                    Ok(Adt11::C3(f133(f134(v0.clone())?, List::cons((f135(), f136()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f137()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f138()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f139()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f140()), List::cons((f8(Str::lit("docs/catalog.md")), f141(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f142(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f143(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f144()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
                }
                Some((v3, v4)) => {
                    Ok(Adt11::C0(List::cons(v3, v4)))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt11::C1(List::cons(v1, v2)))
        }
    }
}

pub fn f72(v0: Bytes, v1: Bytes) -> bool {
    let m333 = {
        let a330 = v0;
        let a331 = 0u64;
        let a332 = {
            let a329 = v1.clone();
            length_bytes(a329)
        };
        slice_bytes(a330, a331, a332)
    };
    match m333 {
        None => {
            false
        }
        Some(v2) => {
            let a334 = v2;
            let a335 = v1;
            equal(a334, a335)
        }
    }
}

pub fn f73(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f145(v3.clone(), v1.clone(), f146(v3, v1, false)?)
    } else {
        if f28(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f145(v3.clone(), v1.clone(), f147(v3, v1)?)
        } else {
            if f28(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f145(v3.clone(), v1.clone(), f146(v3, v1, true)?)
            } else {
                if f28(v0.clone(), f8(Str::lit("stage-encodings"))) {
                    f145(v3.clone(), v1.clone(), f148(v3, v1)?)
                } else {
                    if f28(v0.clone(), f8(Str::lit("stage-mappings"))) {
                        f145(v3.clone(), v1.clone(), f149(v3, v1, false)?)
                    } else {
                        if f28(v0, f8(Str::lit("stage-matrix"))) {
                            f145(v3.clone(), v1.clone(), f149(v3, v1, true)?)
                        } else {
                            Ok(Adt11::C4(f66()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("admit"))) {
        f150(v3, v1)
    } else {
        if f28(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f151(v3, v1)
        } else {
            if f28(v0.clone(), f8(Str::lit("ingest-authority"))) {
                f152(v3, v1)
            } else {
                if f28(v0.clone(), f8(Str::lit("ingest-index"))) {
                    f153(v3, v1)
                } else {
                    if f28(v0.clone(), f8(Str::lit("merge-gate"))) {
                        f154(v3, v1)
                    } else {
                        if f28(v0.clone(), f8(Str::lit("propose-encoding"))) {
                            let m336 = v2;
                            match m336.uncons() {
                                None => {
                                    Ok(Adt11::C4(f66()))
                                }
                                Some((v4, v5)) => {
                                    f155(v3, v1, v4, v5)
                                }
                            }
                        } else {
                            if f28(v0.clone(), f8(Str::lit("propose-xsd-encoding"))) {
                                let m337 = v2;
                                match m337.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f66()))
                                    }
                                    Some((v6, v7)) => {
                                        f156(v3, v1, v6, v7)
                                    }
                                }
                            } else {
                                if f28(v0, f8(Str::lit("record-decision"))) {
                                    f157(v3, v1)
                                } else {
                                    Ok(Adt11::C4(f66()))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f75(v0: Bytes) -> R<bool> {
    f158(v0.clone(), 0u64, {
        let a338 = v0;
        length_bytes(a338)
    })
}

pub fn f76(p343: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m342 = p343;
    {
        let mut m342 = m342;
        loop {
            let step = {
                let v0 = m342;
                let m339 = v0;
                match m339.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h340, _) = v2;
                            h340
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h341) = v2;
                                h341
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m342 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Bytes) -> R<Option<Adt13>> {
    let m344 = f159(v0)?;
    match m344 {
        None => {
            Ok(None::<Adt13>)
        }
        Some(v1) => {
            Ok(f160(v1))
        }
    }
}

pub fn f78(v0: Option<Adt13>) -> Option<Bytes> {
    let m345 = v0;
    match m345 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m346 = v1;
            match m346 {
                Adt13::C0(_) => {
                    None::<Bytes>
                }
                Adt13::C1(_, _) => {
                    None::<Bytes>
                }
                Adt13::C2(v2) => {
                    Some(v2)
                }
                Adt13::C3(_) => {
                    None::<Bytes>
                }
                Adt13::C4(_) => {
                    None::<Bytes>
                }
                Adt13::C5 => {
                    None::<Bytes>
                }
                Adt13::C6(_, _) => {
                    None::<Bytes>
                }
                Adt13::C7(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f79(p352: Adt13, v1: Bytes) -> Option<Adt13> {
    let m351 = p352;
    {
        let mut m351 = m351;
        loop {
            let step = {
                let v0 = m351;
                let m347 = v0;
                match m347 {
                    Adt13::C0(_) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C1(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C3(_) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C4(r348) => {
                        let v5 = (*r348).clone();
                        Ok::<Adt13, Option<Adt13>>(v5)
                    }
                    Adt13::C5 => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C6(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C7(v2, r349, r350) => {
                        let v3 = (*r349).clone();
                        let v4 = (*r350).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt13, Option<Adt13>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Adt13>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m353 = v0;
    match m353.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m354 = f161(v2, v1.clone())?;
            match m354 {
                None => {
                    f80(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f80(v3, v1)?))
                }
            }
        }
    }
}

pub fn f81(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a355 = v0.clone();
        length_bytes(a355)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h356, _) = f63(Fn0::F162(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h356
    };
    let v3: u64 = {
        let (h357, _) = {
            let (_, h358) = v2.clone();
            h358
        };
        h357
    };
    let v4: List<Bytes> = {
        let (_, h359) = {
            let (_, h360) = v2;
            h360
        };
        h359
    };
    let a361 = v3;
    let a362 = v1;
    if nat_lt(a361, a362) {
        Ok(f65(List::cons(f35(v0, v3, {
            let a363 = v1;
            let a364 = v3;
            nat_sub(a363, a364)
        }), v4)))
    } else {
        Ok(f65(v4))
    }
}

pub fn f82(v0: Adt1, v1: Bytes) -> R<List<Adt13>> {
    Ok(f163(f46(v0, v1)?))
}

pub fn f83(v0: List<Adt13>, v1: Bytes) -> List<Adt13> {
    let m365 = v0;
    match m365.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f164(v2, v1.clone()), f83(v3, v1))
        }
    }
}

pub fn f84(p370: List<Bytes>, p371: List<Bytes>) -> bool {
    let m369 = (p370, p371);
    {
        let mut m369 = m369;
        loop {
            let step = {
                let (v0, v1) = m369;
                let m366 = v0;
                match m366.uncons() {
                    None => {
                        let m367 = v1;
                        match m367.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m368 = v1;
                        match m368.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f28(v4, v6) {
                                    Ok::<(List<Bytes>, List<Bytes>), bool>((v5, v7))
                                } else {
                                    Err::<(List<Bytes>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m369 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<Adt13>) -> List<Bytes> {
    let m372 = v0;
    match m372.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m373 = f79(v1, f8(Str::lit("std")));
            match m373 {
                None => {
                    f85(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f85(v2))
                }
            }
        }
    }
}

pub fn f86(v0: Adt13) -> R<Bytes> {
    let a374 = f165(v0)?;
    let a375 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a374, a375))
}

pub fn f87(v0: List<Adt13>) -> Adt13 {
    let m376 = v0;
    match m376.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f87(v2)))
        }
    }
}

pub fn f88(v0: Adt12) -> Bytes {
    let m377 = v0;
    match m377 {
        Adt12::C0 => {
            f8(Str::lit("VERDANT-ENGINE-AUTHORITIES-"))
        }
        Adt12::C1 => {
            f8(Str::lit("VERDANT-ENGINE-IDS-"))
        }
        Adt12::C2 => {
            f8(Str::lit("VERDANT-ENGINE-RELEASE-LEDGER-"))
        }
        Adt12::C3 => {
            f8(Str::lit("VERDANT-ENGINE-STANDARDS-LOCK-"))
        }
    }
}

pub fn f89() -> Bytes {
    f8(Str::lit("target/engine/ref.json"))
}

pub fn f90() -> Bytes {
    f8(Str::lit("target/engine/tag.json"))
}

pub fn f91(v0: Adt1, v1: Bytes, v2: Bytes) -> R<Adt11> {
    let a379 = {
        let a378 = v2.clone();
        length_bytes(a378)
    };
    let a380 = 0u64;
    if nat_eq(a379, a380) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m381 = f166(v0.clone(), f49())?;
        match m381 {
            None => {
                if f40(f41(f169(), match v0.clone() {
                    Adt1::C0(_, _, h382) => {
                        h382
                    }
                })) {
                    Ok(Adt11::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f169()))
                } else {
                    let m383 = f45(v0.clone(), List::cons(f170(), f171(f49())?));
                    match m383.uncons() {
                        None => {
                            Ok(Adt11::C3(f172(v0.clone(), f49(), v1.clone(), v2.clone(), f173(f53(v0, f170()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt11::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f43(f167(v3.clone())?, f168(v3, v2)))
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h384, _) = v2.clone();
        h384
    };
    let v4: u64 = {
        let (h385, _) = {
            let (_, h386) = v2.clone();
            h386
        };
        h385
    };
    let v5: List<Bytes> = {
        let (_, h387) = {
            let (_, h388) = v2;
            h388
        };
        h387
    };
    let m389 = f174(v0.clone(), v3);
    match m389 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a390 = v6;
            let a391 = v1;
            if equal(a390, a391) {
                Ok(Some(({
                    let a392 = v3;
                    let a393 = 1u64;
                    nat_add(a392, a393)?
                }, ({
                    let a394 = v3;
                    let a395 = 1u64;
                    nat_add(a394, a395)?
                }, List::cons(f35(v0, v4, {
                    let a396 = v3;
                    let a397 = v4;
                    nat_sub(a396, a397)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a398 = v3;
                    let a399 = 1u64;
                    nat_add(a398, a399)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f93(v0: Fn1, p403: List<Bytes>, p404: List<Bytes>) -> List<Bytes> {
    let m402 = (p403, p404);
    {
        let mut m402 = m402;
        loop {
            let step = {
                let (v1, v2) = m402;
                let m400 = v2;
                match m400.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c401: Fn1 = v0.clone();
                            c401.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m405 = f175(Fn2::F176(v1), None::<Bytes>, v0);
    match m405 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f96() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("propose-xsd-encoding FILE NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))
}

pub fn f97(v0: Adt1) -> List<Bytes> {
    if f123(match v0.clone() {
        Adt1::C0(_, h406, _) => {
            h406
        }
    }, f177()) {
        if f123(match v0 {
            Adt1::C0(_, h407, _) => {
                h407
            }
        }, f178()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f178(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f177(), List::<Bytes>::nil())
    }
}

pub fn f98(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m408 = v0;
    match m408.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f98(v3, v1))
        }
    }
}

pub fn f99(v0: Adt1) -> R<List<Bytes>> {
    Ok(f45(v0.clone(), List::cons(f179(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f109(v0)?)))))
}

pub fn f100(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f120(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f121(f82(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f101(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m409 = v0;
    match m409.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f101(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f101(v3, v1))
            }
        }
    }
}

pub fn f102(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m410 = v0;
    match m410.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f102(v3, v1)
            } else {
                List::cons(v2, f102(v3, v1))
            }
        }
    }
}

pub fn f103(v0: List<Adt13>) -> List<Bytes> {
    let m411 = v0;
    match m411.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f98(f122(f163(f79(v1, f8(Str::lit("defers"))))), f103(v2))
        }
    }
}

pub fn f104(v0: List<Adt13>) -> List<Bytes> {
    let m412 = v0;
    match m412.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m413 = f79(v1, f8(Str::lit("std")));
            match m413 {
                None => {
                    f104(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("row"))), f104(v2))
                }
            }
        }
    }
}

pub fn f105(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m414 = v0;
    match m414.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f105(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f105(v3, v1))
            }
        }
    }
}

pub fn f106(v0: Adt14, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f133(f180(v2.clone()), f181(f182(v0, v1.clone(), v2)?));
    f183(f125(f184(v1))?, v3.clone(), {
        let a416 = {
            let a415 = v3;
            length_list(a415)
        };
        let a417 = 1u64;
        nat_add(a416, a417)?
    })
}

pub fn f107(v0: Adt14, v1: List<Adt13>) -> List<Adt13> {
    let m418 = v1;
    match m418.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            if if f185(v2.clone(), f8(Str::lit("subject"))) {
                f186(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f107(v0, v3))
            } else {
                f107(v0, v3)
            }
        }
    }
}

pub fn f108(v0: Adt1, v1: List<Bytes>) -> R<List<Adt13>> {
    let m419 = v1;
    match m419.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v2, v3)) => {
            let m420 = f187(f81(f53(v0.clone(), v2.clone()))?, f72(v2, f178()))?;
            match m420 {
                None => {
                    f108(v0, v3)
                }
                Some(v4) => {
                    Ok(f188(v4, f108(v0, v3)?))
                }
            }
        }
    }
}

pub fn f109(v0: Adt1) -> R<List<Bytes>> {
    Ok(f98(f125(f189(f177(), f127(match v0.clone() {
        Adt1::C0(_, h421, _) => {
            h421
        }
    }, f177())))?, f125(f189(f178(), f127(match v0 {
        Adt1::C0(_, h422, _) => {
            h422
        }
    }, f178())))?))
}

pub fn f110(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f111() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f179(), List::<Bytes>::nil())))))))
}

pub fn f112(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m423 = v1;
    match m423.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f56(v0.clone(), v2.clone()) {
                f190(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m424 = f46(v0.clone(), v2.clone())?;
                match m424 {
                    None => {
                        Ok(List::cons(v2, f112(v0, v3)?))
                    }
                    Some(_) => {
                        f112(v0, v3)
                    }
                }
            } else {
                f112(v0, v3)
            }
        }
    }
}

pub fn f113(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m425 = v1;
    match m425.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f123(v0.clone(), v2.clone()) {
                f113(v0, v3)
            } else {
                List::cons(v2, f113(v0, v3))
            }
        }
    }
}

pub fn f114() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f115(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m426 = v0;
    match m426.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f98(f191({
                let (h427, _) = v2;
                h427
            }, {
                let (_, h428) = v2;
                h428
            }, v1), f115(v3, v1))
        }
    }
}

pub fn f116() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f117(v0: Adt1) -> R<Option<Bytes>> {
    f192(f193(v0, f116())?)
}

pub fn f118(v0: Adt1, p432: List<Bytes>) -> R<Option<Bytes>> {
    let m431 = p432;
    {
        let mut m431 = m431;
        loop {
            let step = {
                let v1 = m431;
                let m429 = v1;
                match m429.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m430 = f194(f53(v0.clone(), v2.clone()), f195())?;
                        match m430 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                            }
                            Some(_) => {
                                Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h433, _, _, _, _) => {
            h433
        }
    };
    let v2: Adt17 = f196(v1.clone())?;
    let v3: List<Bytes> = f197(v1.clone());
    let v4: List<Bytes> = f121(match v0.clone() {
        Adt16::C0(_, _, h434, _, _) => {
            h434
        }
    }, f8(Str::lit("path")));
    let m436 = f198(v2, List::<u64>::nil(), f199(v1.clone(), match v0.clone() {
        Adt16::C0(_, h435, _, _, _) => {
            h435
        }
    }, 0u64)?);
    match m436 {
        None => {
            if f201(f202(match v0.clone() {
                Adt16::C0(_, _, _, _, h438) => {
                    h438
                }
            }, f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h439) => {
                    h439
                }
            }, v4.clone()))?, f202(v4.clone(), f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h440) => {
                    h440
                }
            }, v4.clone()))?, f203(match v0.clone() {
                Adt16::C0(_, _, h441, _, _) => {
                    h441
                }
            })) {
                let m445 = f204(f202(match v0.clone() {
                    Adt16::C0(_, _, _, h442, _) => {
                        h442
                    }
                }, f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h443, _) => {
                        h443
                    }
                }))?, f202(v3.clone(), f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h444, _) => {
                        h444
                    }
                }))?);
                match m445 {
                    None => {
                        let m448 = f205(f121(match v0 {
                            Adt16::C0(_, h447, _, _, _) => {
                                h447
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m448 {
                            None => {
                                let m449 = f205(v4, List::<Bytes>::nil());
                                match m449 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f95(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                    }
                                    Some(v8) => {
                                        Ok((false, f7(List::cons(f8(Str::lit("module with two stratum rows: ")), List::cons(v8, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                    }
                                }
                            }
                            Some(v7) => {
                                Ok((false, f7(List::cons(f8(Str::lit("construct admitted twice: ")), List::cons(v7, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                            }
                        }
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f200(f98(v3, match v0 {
                            Adt16::C0(_, _, _, h446, _) => {
                                h446
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f200(match v1 {
                Adt15::C0(_, _, _, _, h437) => {
                    h437
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f120(v0: Adt1) -> R<List<Bytes>> {
    let m450 = f206(f53(v0, f179()))?;
    match m450.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f207(v2))
        }
    }
}

pub fn f121(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m451 = v0;
    match m451.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f47(v2, v1.clone()), f121(v3, v1))
        }
    }
}

pub fn f122(v0: List<Adt13>) -> List<Bytes> {
    let m452 = v0;
    match m452.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m453 = f78(Some(v1));
            match m453 {
                None => {
                    f122(v2)
                }
                Some(v3) => {
                    List::cons(v3, f122(v2))
                }
            }
        }
    }
}

pub fn f123(p457: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m456 = p457;
    {
        let mut m456 = m456;
        loop {
            let step = {
                let v0 = m456;
                let m454 = v0;
                match m454.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h455, _) = v2;
                            h455
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f125(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f208(f93(Fn1::F209, List::<Bytes>::nil(), f210(v0)?)))
}

pub fn f126(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m458 = v1;
    match m458.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f190(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f211(v0.clone(), v2), f126(v0, v3))
            } else {
                f126(v0, v3)
            }
        }
    }
}

pub fn f127(p463: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m462 = p463;
    {
        let mut m462 = m462;
        loop {
            let step = {
                let v0 = m462;
                let m459 = v0;
                match m459.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h460, _) = v2;
                            h460
                        }, v1.clone()) {
                            let (_, h461) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h461)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: Adt1, v1: List<List<Bytes>>, p466: List<Bytes>) -> R<Option<Bytes>> {
    let m465 = p466;
    {
        let mut m465 = m465;
        loop {
            let step = {
                let v2 = m465;
                let m464 = v2;
                match m464.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f212(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(p469: List<Bytes>, p470: List<Bytes>, p471: List<List<Bytes>>) -> List<List<Bytes>> {
    let m468 = (p469, (p470, p471));
    {
        let mut m468 = m468;
        loop {
            let step = {
                let (v0, (v1, v2)) = m468;
                let m467 = v0;
                match m467.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f213(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f28(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f213(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m472 = v0;
    match m472.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f130(v3, v1)
            } else {
                List::cons(v2, f130(v3, v1))
            }
        }
    }
}

pub fn f131(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h473, _) => {
            h473
        }
    }, f214())) {
        Ok(Adt11::C1(List::cons(f214(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f130(f125(f126(f214(), f127(match v0.clone() {
            Adt1::C0(_, h474, _) => {
                h474
            }
        }, f214())))?, v1.clone());
        let m475 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m475.uncons() {
            None => {
                let v5: List<Bytes> = f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m476 = f45(v0.clone(), f215(v0.clone(), v2.clone(), v5.clone())?);
                match m476.uncons() {
                    None => {
                        let m477 = f216(v0.clone(), v2, v5)?;
                        match m477 {
                            None => {
                                f217(v0, v1)
                            }
                            Some(v8) => {
                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("admit-encoding ")), List::cons(v8, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                            }
                        }
                    }
                    Some((v6, v7)) => {
                        Ok(Adt11::C0(List::cons(v6, v7)))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f132() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f133(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m478 = v0;
    match m478.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f133(v3, v1))
        }
    }
}

pub fn f134(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m479 = f82(v0.clone(), f132())?;
    match m479.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f218(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f135() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f136() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f137() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f219(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f138() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f219(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f139() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f140() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f141(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f100(v0.clone())?;
    let v2: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f107(v1.clone(), f108(v0.clone(), f109(v0.clone())?)?);
    let v4: List<List<Bytes>> = f220(f206(f53(v0, f179()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f221(v4.clone(), f207(v4), f182(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f106(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f142(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f222(f100(v0.clone())?, f108(v0.clone(), f109(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f143(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f223(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f224(f82(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f144() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f145(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m480 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m480.uncons() {
        None => {
            if f225(f81(f53(v0, v1))?)? {
                Ok(Adt11::C4(f8(Str::lit("the input has a blank line; send one record per line and no trailing newline, because the workflow adds the final one\u{a}"))))
            } else {
                Ok(v2)
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f146(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m481 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m481.uncons() {
        None => {
            let v5: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
            if f40(f190(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f226(v5)?
                } else {
                    f227(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f211(f228(), f229(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f211(f228(), f229(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f230()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f147(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m482 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m482.uncons() {
        None => {
            let m483 = f231(f81(f53(v0.clone(), v1.clone()))?)?;
            match m483 {
                None => {
                    Ok(Adt11::C4(f232()))
                }
                Some(v4) => {
                    let m484 = f233(f81(f53(v0, v1))?)?;
                    match m484 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f95(f38(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f234(v5)))
                        }
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f148(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m485 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m485.uncons() {
        None => {
            let m486 = f235(f81(f53(v0, v1))?)?;
            match m486 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f95(f38(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f149(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m487 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m487.uncons() {
        None => {
            let v5: Bytes = f211(if v2 {
                f178()
            } else {
                f177()
            }, f229(v1.clone())?);
            if f40(f190(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m488 = f187(f81(f53(v0.clone(), v1.clone()))?, v2)?;
                match m488 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a490 = {
                            let a489 = v6;
                            length_list(a489)
                        };
                        let a491 = 0u64;
                        if nat_eq(a490, a491) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f53(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
                        }
                    }
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f150(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m492 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m492.uncons() {
        None => {
            let v4: List<Bytes> = f81(f53(v0.clone(), v1))?;
            let m493 = f236(v4.clone(), f8(Str::lit("requirement")))?;
            match m493 {
                None => {
                    Ok(Adt11::C4(f230()))
                }
                Some(v5) => {
                    let m494 = f236(v4, f8(Str::lit("admission")))?;
                    match m494 {
                        None => {
                            Ok(Adt11::C4(f230()))
                        }
                        Some(v6) => {
                            let v7: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f98(f121(v5.clone(), f8(Str::lit("id"))), f121(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f125(f98(f121(v7.clone(), f8(Str::lit("id"))), f121(v8.clone(), f8(Str::lit("construct")))))?;
                            let m495 = f237(f104(v6.clone()), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m495 {
                                None => {
                                    let a497 = {
                                        let a496 = v9.clone();
                                        length_list(a496)
                                    };
                                    let a498 = 0u64;
                                    if nat_eq(a497, a498) {
                                        Ok(Adt11::C4(f230()))
                                    } else {
                                        if f40({
                                            let a501 = {
                                                let a499 = f125(f98(v9.clone(), v10.clone()))?;
                                                length_list(a499)
                                            };
                                            let a502 = {
                                                let a500 = f98(v9.clone(), v10);
                                                length_list(a500)
                                            };
                                            nat_eq(a501, a502)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f239(f8(Str::lit("model/requirements.json")), f188(v7, v5))?, List::cons(f239(f8(Str::lit("model/admissions.json")), f188(v8, v6))?, List::cons(f240(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f95(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f238(v11))
                                }
                            }
                        }
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f151(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m503 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m503.uncons() {
        None => {
            let m504 = f241(v0.clone(), v1.clone())?;
            match m504 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m505 = f242(v4.clone(), f8(Str::lit("schema")));
                    match m505 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f243(v0, f244(v5.clone())?, v5, f245(v4.clone(), f8(Str::lit("name"))), f246(f247(f242(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
                        }
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f152(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m506 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m506.uncons() {
        None => {
            let m507 = f46(v0.clone(), v1.clone())?;
            match m507 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f40(f28(f47(v4.clone(), f8(Str::lit("schema"))), f248())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt13 = {
                            let m508 = f79(v4.clone(), f8(Str::lit("source")));
                            match m508 {
                                None => {
                                    Adt13::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f78(f79(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f211(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f211(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m509 = v8;
                        match m509 {
                            None => {
                                f249(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f40(f28(f47(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f249(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f211(v9, f229(f47(v7.clone(), f8(Str::lit("url"))))?);
                                    if f40(f41(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h510) => {
                                            h510
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f47(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m511 = f45(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m511.uncons() {
                                            None => {
                                                if f28(f250(f53(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f249(v0, v4, v5, v10, Some((v12, v11)))
                                                } else {
                                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("checksum mismatch: ")), List::cons(v12, List::cons(f8(Str::lit(" is not the sha256 the binding pins\u{a}")), List::<Bytes>::nil()))))))
                                                }
                                            }
                                            Some((v13, v14)) => {
                                                Ok(Adt11::C0(List::cons(v13, v14)))
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
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f153(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m512 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m512.uncons() {
        None => {
            let v4: Bytes = f53(v0.clone(), v1);
            let v5: List<List<Bytes>> = f206(v4.clone())?;
            if f40(f84(f251(v5.clone()), f252())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f253(Str::lit("id"), v6.clone())?, List::cons(f253(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f253(Str::lit("artifact"), f179())?, List::cons(f253(Str::lit("checksum"), f250(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f179(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f254(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f240(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f95(f207(f220(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f154(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m513 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m513.uncons() {
        None => {
            let m514 = f46(v0, v1)?;
            match m514 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f255(f163(Some(v4))) {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("merge\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("wait\u{a}"))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f155(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m515 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m515.uncons() {
        None => {
            let m516 = f241(v0.clone(), v1.clone())?;
            match m516 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f243(v0.clone(), f53(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f156(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m517 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil()))))))));
    match m517.uncons() {
        None => {
            let m518 = f256(v3);
            match m518 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v6) => {
                    let m519 = f237(f104(List::cons(Adt13::C4(std::rc::Rc::new(v6.clone())), List::<Adt13>::nil())), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                    match m519 {
                        None => {
                            let m521 = f257(match v0.clone() {
                                Adt1::C0(h520, _, _) => {
                                    h520
                                }
                            }, f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v1.clone())?;
                            match m521 {
                                Adt24::C0(v8) => {
                                    Ok(Adt11::C0(v8))
                                }
                                Adt24::C1(v9) => {
                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                }
                                Adt24::C2(_, v11) => {
                                    f258(v0, v2.clone(), v6, f259(v2, v11)?, f7(List::cons(f8(Str::lit("Encoding entered through propose-xsd-encoding from ")), List::cons(v1, List::cons(f8(Str::lit(".")), List::<Bytes>::nil())))), List::<(Bytes, Bytes)>::nil())
                                }
                            }
                        }
                        Some(v7) => {
                            Ok(f238(v7))
                        }
                    }
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f157(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m522 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f132(), List::<Bytes>::nil())));
    match m522.uncons() {
        None => {
            let m523 = f260(f81(f53(v0.clone(), v1))?)?;
            match m523 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt13> = f82(v0, f132())?;
                    let v6: List<Bytes> = f121(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f125(f121(v5.clone(), f8(Str::lit("at"))))?;
                    let a525 = {
                        let a524 = v6.clone();
                        length_list(a524)
                    };
                    let a526 = 0u64;
                    if if nat_eq(a525, a526) {
                        true
                    } else {
                        f40({
                            let a529 = {
                                let a527 = f125(f98(v6.clone(), v7.clone()))?;
                                length_list(a527)
                            };
                            let a530 = {
                                let a528 = f98(v6.clone(), v7);
                                length_list(a528)
                            };
                            nat_eq(a529, a530)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f239(f132(), f188(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f95(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f158(v0: Bytes, p584: u64, p585: u64) -> R<bool> {
    let m583 = (p584, p585);
    {
        let mut m583 = m583;
        loop {
            let step = {
                let (v1, v2) = m583;
                let m531 = v2;
                if m531 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m531 - 1;
                    let m534 = {
                        let a532 = v0.clone();
                        let a533 = v1;
                        index_bytes(a532, a533)
                    };
                    match m534 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f261({
                                let v5: u8 = v4;
                                let a579 = {
                                    let a555 = {
                                        let a543 = {
                                            let a537 = {
                                                let a535 = v5;
                                                let a536 = 128u8;
                                                fixed_u8::bit_and(a535, a536)
                                            };
                                            let a538 = 128u8;
                                            if equal(a537, a538) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a544 = {
                                            let a541 = {
                                                let a539 = v5;
                                                let a540 = 64u8;
                                                fixed_u8::bit_and(a539, a540)
                                            };
                                            let a542 = 64u8;
                                            if equal(a541, a542) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a543, a544)?
                                    };
                                    let a556 = {
                                        let a553 = {
                                            let a547 = {
                                                let a545 = v5;
                                                let a546 = 32u8;
                                                fixed_u8::bit_and(a545, a546)
                                            };
                                            let a548 = 32u8;
                                            if equal(a547, a548) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a554 = {
                                            let a551 = {
                                                let a549 = v5;
                                                let a550 = 16u8;
                                                fixed_u8::bit_and(a549, a550)
                                            };
                                            let a552 = 16u8;
                                            if equal(a551, a552) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a553, a554)?
                                    };
                                    nat_add(a555, a556)?
                                };
                                let a580 = {
                                    let a577 = {
                                        let a565 = {
                                            let a559 = {
                                                let a557 = v5;
                                                let a558 = 8u8;
                                                fixed_u8::bit_and(a557, a558)
                                            };
                                            let a560 = 8u8;
                                            if equal(a559, a560) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a566 = {
                                            let a563 = {
                                                let a561 = v5;
                                                let a562 = 4u8;
                                                fixed_u8::bit_and(a561, a562)
                                            };
                                            let a564 = 4u8;
                                            if equal(a563, a564) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a565, a566)?
                                    };
                                    let a578 = {
                                        let a575 = {
                                            let a569 = {
                                                let a567 = v5;
                                                let a568 = 2u8;
                                                fixed_u8::bit_and(a567, a568)
                                            };
                                            let a570 = 2u8;
                                            if equal(a569, a570) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a576 = {
                                            let a573 = {
                                                let a571 = v5;
                                                let a572 = 1u8;
                                                fixed_u8::bit_and(a571, a572)
                                            };
                                            let a574 = 1u8;
                                            if equal(a573, a574) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a575, a576)?
                                    };
                                    nat_add(a577, a578)?
                                };
                                nat_add(a579, a580)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a581 = v1;
                                    let a582 = 1u64;
                                    nat_add(a581, a582)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: Bytes) -> R<Option<Adt18>> {
    let v1: Bytes = f262(v0)?;
    let m589 = f263(v1.clone(), 0u64, {
        let a587 = {
            let a586 = v1.clone();
            length_bytes(a586)
        };
        let a588 = 2u64;
        nat_add(a587, a588)?
    }, 0u64)?;
    match m589 {
        None => {
            Ok(None::<Adt18>)
        }
        Some(v2) => {
            let a592 = {
                let (_, h590) = v2;
                h590
            };
            let a593 = {
                let a591 = v1;
                length_bytes(a591)
            };
            if nat_eq(a592, a593) {
                Ok(Some({
                    let (h594, _) = v2;
                    h594
                }))
            } else {
                Ok(None::<Adt18>)
            }
        }
    }
}

pub fn f160(v0: Adt18) -> Option<Adt13> {
    let m595 = v0;
    match m595 {
        Adt18::C0(v1) => {
            Some(Adt13::C0(v1))
        }
        Adt18::C1(v2, v3) => {
            Some(Adt13::C1(v2, v3))
        }
        Adt18::C2(_, _, _) => {
            None::<Adt13>
        }
        Adt18::C3(v7) => {
            Some(Adt13::C2(v7))
        }
        Adt18::C4(r602) => {
            let v8 = (*r602).clone();
            let m596 = f160(v8);
            match m596 {
                None => {
                    None::<Adt13>
                }
                Some(v9) => {
                    Some(Adt13::C3(std::rc::Rc::new(v9)))
                }
            }
        }
        Adt18::C5(r603) => {
            let v10 = (*r603).clone();
            let m597 = f160(v10);
            match m597 {
                None => {
                    None::<Adt13>
                }
                Some(v11) => {
                    Some(Adt13::C4(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt18::C6 => {
            Some(Adt13::C5)
        }
        Adt18::C7(r604, r605) => {
            let v12 = (*r604).clone();
            let v13 = (*r605).clone();
            let m598 = f160(v12);
            match m598 {
                None => {
                    None::<Adt13>
                }
                Some(v14) => {
                    let m599 = f160(v13);
                    match m599 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v15) => {
                            Some(Adt13::C6(std::rc::Rc::new(v14), std::rc::Rc::new(v15)))
                        }
                    }
                }
            }
        }
        Adt18::C8(v16, r606, r607) => {
            let v17 = (*r606).clone();
            let v18 = (*r607).clone();
            let m600 = f160(v17);
            match m600 {
                None => {
                    None::<Adt13>
                }
                Some(v19) => {
                    let m601 = f160(v18);
                    match m601 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v20) => {
                            Some(Adt13::C7(v16, std::rc::Rc::new(v19), std::rc::Rc::new(v20)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f161(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), v1) {
        let m608 = f33(v0, 34u8)?;
        match m608.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m609 = v3;
                match m609.uncons() {
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

pub fn f162(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f264(v0, v1)
}

pub fn f163(v0: Option<Adt13>) -> List<Adt13> {
    let m610 = v0;
    match m610 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            f265(v1)
        }
    }
}

pub fn f164(v0: Adt13, v1: Bytes) -> Adt13 {
    let m611 = v0;
    match m611 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r612) => {
            let v10 = (*r612).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r613) => {
            let v5 = (*r613).clone();
            Adt13::C4(std::rc::Rc::new(f164(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r614, r615) => {
            let v11 = (*r614).clone();
            let v12 = (*r615).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r616, r617) => {
            let v3 = (*r616).clone();
            let v4 = (*r617).clone();
            if f28(v2.clone(), f8(Str::lit("std"))) {
                Adt13::C7(v2, std::rc::Rc::new(f266(v3, v1.clone())), std::rc::Rc::new(f164(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f164(v4, v1)))
            }
        }
    }
}

pub fn f165(v0: Adt13) -> R<Bytes> {
    let m618 = v0;
    match m618 {
        Adt13::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt13::C1(v2, v3) => {
            f267(v2, v3)
        }
        Adt13::C2(v4) => {
            f268(v4)
        }
        Adt13::C3(r619) => {
            let v5 = (*r619).clone();
            Ok(f269(f270(v5)?))
        }
        Adt13::C4(r620) => {
            let v6 = (*r620).clone();
            f271(f272(v6)?)
        }
        Adt13::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt13::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt13::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f166(v0: Adt1, p624: List<Adt12>) -> R<Option<Adt12>> {
    let m623 = p624;
    {
        let mut m623 = m623;
        loop {
            let step = {
                let v1 = m623;
                let m621 = v1;
                match m621.uncons() {
                    None => {
                        Err::<List<Adt12>, R<Option<Adt12>>>(Ok(None::<Adt12>))
                    }
                    Some((v2, v3)) => {
                        if f41(f167(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h622) => {
                                h622
                            }
                        }) {
                            Ok::<List<Adt12>, R<Option<Adt12>>>(v3)
                        } else {
                            Err::<List<Adt12>, R<Option<Adt12>>>(Ok(Some(v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m623 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Adt12) -> R<Bytes> {
    Ok(f211(f273(), f229(f274(v0))?))
}

pub fn f168(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f274(v0), List::<Bytes>::nil())))))
}

pub fn f169() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f170() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f171(v0: List<Adt12>) -> R<List<Bytes>> {
    let m625 = v0;
    match m625.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f167(v1)?, f171(v2)?))
        }
    }
}

pub fn f172(v0: Adt1, v1: List<Adt12>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m626 = v1;
    match m626.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f57(f58(v5.clone(), f50(v2.clone()))), f275(v5.clone(), v2.clone(), v3.clone(), f250(f53(v0.clone(), f167(v5)?))?, v4.clone())?), f172(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f173(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f276(v0.clone())?;
    let v2: u64 = f277(v0.clone(), v1)?;
    Ok(f35(v0, v2, {
        let a627 = v1;
        let a628 = v2;
        nat_sub(a627, a628)
    }))
}

pub fn f174(v0: Bytes, v1: u64) -> Option<u8> {
    let a629 = v0;
    let a630 = v1;
    index_bytes(a629, a630)
}

pub fn f175(v0: Fn2, p634: Option<Bytes>, p635: List<Bytes>) -> Option<Bytes> {
    let m633 = (p634, p635);
    {
        let mut m633 = m633;
        loop {
            let step = {
                let (v1, v2) = m633;
                let m631 = v2;
                match m631.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c632: Fn2 = v0.clone();
                            c632.apply(v1, v3)
                        }, v4))
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

pub fn f176(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m636 = v1;
    match m636 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a639 = {
                    let a637 = v3;
                    let a638 = v0;
                    append_bytes(a637, a638)
                };
                let a640 = v2;
                append_bytes(a639, a640)
            })
        }
    }
}

pub fn f177() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f178() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f179() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f180(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m641 = v0;
    match m641.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f47(v1.clone(), f8(Str::lit("subject"))), f47(v1, f8(Str::lit("object")))), f180(v2))
        }
    }
}

pub fn f181(v0: List<Adt19>) -> List<(Bytes, Bytes)> {
    let m642 = v0;
    match m642.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt19::C0(h643, _, _) => {
                    h643
                }
            }, match v1 {
                Adt19::C0(_, _, h644) => {
                    h644
                }
            }), f181(v2))
        }
    }
}

pub fn f182(v0: Adt14, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Adt19>> {
    let v3: List<(Bytes, Bytes)> = f278(v1);
    let v4: List<(Bytes, Bytes)> = f279(v0, v2);
    f280(v3.clone(), f133(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f183(p651: List<Bytes>, v1: List<(Bytes, Bytes)>, p652: u64) -> R<List<Bytes>> {
    let m650 = (p651, p652);
    {
        let mut m650 = m650;
        loop {
            let step = {
                let (v0, v2) = m650;
                let m645 = v2;
                if m645 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m645 - 1;
                    let v4: List<Bytes> = f125(f98(v0.clone(), f281(v0.clone(), v1.clone())))?;
                    let a648 = {
                        let a646 = v4.clone();
                        length_list(a646)
                    };
                    let a649 = {
                        let a647 = v0.clone();
                        length_list(a647)
                    };
                    if nat_eq(a648, a649) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: List<Adt13>) -> List<Bytes> {
    let m653 = v0;
    match m653.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m654 = f79(v1, f8(Str::lit("std")));
            match m654 {
                None => {
                    f184(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3.clone(), f8(Str::lit("authority"))), if f185(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f47(v3, f8(Str::lit("row"))), f184(v2))
                    } else {
                        f184(v2)
                    })
                }
            }
        }
    }
}

pub fn f185(v0: Adt13, v1: Bytes) -> bool {
    f40({
        let a656 = {
            let a655 = f47(v0, v1);
            length_bytes(a655)
        };
        let a657 = 0u64;
        nat_eq(a656, a657)
    })
}

pub fn f186(v0: Adt14, v1: Adt13) -> bool {
    if f282(v0.clone(), v1.clone(), f47(v1.clone(), f8(Str::lit("subject")))) {
        f282(v0, v1.clone(), f47(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f187(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt13>>> {
    let m658 = v0;
    match m658.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f173(v2)?;
            let a660 = {
                let a659 = v4.clone();
                length_bytes(a659)
            };
            let a661 = 0u64;
            if nat_eq(a660, a661) {
                f187(v3, v1)
            } else {
                let m662 = f77(v4)?;
                match m662 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f283(v5.clone())
                        } else {
                            f284(v5.clone())
                        } {
                            let m663 = f187(v3, v1)?;
                            match m663 {
                                None => {
                                    Ok(None::<List<Adt13>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons(v5, v6)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt13>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f188(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m664 = v0;
    match m664.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f188(v3, v1))
        }
    }
}

pub fn f189(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m665 = v1;
    match m665.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f190(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f211(v0.clone(), v2), f189(v0, v3))
            } else {
                f189(v0, v3)
            }
        }
    }
}

pub fn f190(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a666 = v0.clone();
        length_bytes(a666)
    };
    let v3: u64 = {
        let a667 = v1.clone();
        length_bytes(a667)
    };
    let a668 = v3;
    let a669 = v2;
    if nat_le(a668, a669) {
        let m675 = {
            let a672 = v0;
            let a673 = {
                let a670 = v2;
                let a671 = v3;
                nat_sub(a670, a671)
            };
            let a674 = v3;
            slice_bytes(a672, a673, a674)
        };
        match m675 {
            None => {
                false
            }
            Some(v4) => {
                let a676 = v4;
                let a677 = v1;
                equal(a676, a677)
            }
        }
    } else {
        false
    }
}

pub fn f191(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m678 = v1;
    match m678.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f285(v3.clone())
            } else {
                f40(f285(v3.clone()))
            } {
                List::cons(f211(v0.clone(), v3), f191(v0, v4, v2))
            } else {
                f191(v0, v4, v2)
            }
        }
    }
}

pub fn f192(p681: List<Bytes>) -> R<Option<Bytes>> {
    let m680 = p681;
    {
        let mut m680 = m680;
        loop {
            let step = {
                let v0 = m680;
                let m679 = v0;
                match m679.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f286(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f173(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m682 = v1;
    match m682.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f98(f81(f53(v0.clone(), v2))?, f193(v0, v3)?))
        }
    }
}

pub fn f194(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a683 = v0.clone();
        length_bytes(a683)
    };
    let v3: u64 = {
        let a684 = v1.clone();
        length_bytes(a684)
    };
    let v4: u64 = {
        let (h685, _) = f287(Fn3::F288(v1.clone(), v3, v0.clone(), v2), {
            let a686 = v2;
            let a687 = 1u64;
            nat_add(a686, a687)?
        }, 0u64)?;
        h685
    };
    if f289(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f195() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f196(v0: Adt15) -> R<Adt17> {
    Ok(Adt17::C0(f290(match v0.clone() {
        Adt15::C0(h688, _, _, _, _) => {
            h688
        }
    }, 0u64)?, f290(match v0.clone() {
        Adt15::C0(_, h689, _, _, _) => {
            h689
        }
    }, 0u64)?, f290(match v0.clone() {
        Adt15::C0(_, _, h690, _, _) => {
            h690
        }
    }, 0u64)?, f290(match v0 {
        Adt15::C0(_, _, _, h691, _) => {
            h691
        }
    }, 0u64)?))
}

pub fn f197(v0: Adt15) -> List<Bytes> {
    f98(match v0.clone() {
        Adt15::C0(h692, _, _, _, _) => {
            h692
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, h693, _, _) => {
            h693
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, _, h694, _) => {
            h694
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h695) => {
            h695
        }
    })))
}

pub fn f198(v0: Adt17, p701: List<u64>, p702: List<Adt20>) -> Option<u64> {
    let m700 = (p701, p702);
    {
        let mut m700 = m700;
        loop {
            let step = {
                let (v1, v2) = m700;
                let m696 = v2;
                match m696.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f291(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h697) => {
                                h697
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h698, _) => {
                                    h698
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h699, _) => {
                                    h699
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: Adt15, v1: List<Adt13>, v2: u64) -> R<List<Adt20>> {
    let m703 = v1;
    match m703.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f292(v0.clone(), v3)?), f199(v0, v4, {
                let a704 = v2;
                let a705 = 1u64;
                nat_add(a704, a705)?
            })?))
        }
    }
}

pub fn f200(p709: List<Bytes>, p710: u64) -> Bytes {
    let m708 = (p709, p710);
    {
        let mut m708 = m708;
        loop {
            let step = {
                let (v0, v1) = m708;
                let m706 = v0;
                match m706.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m707 = v1;
                        if m707 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m707 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
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

pub fn f201(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f293(v0.clone(), v1.clone()) {
        if f293(v1, v0) {
            f294(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f202(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m711 = v0;
    match m711.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f295(v2, v1.clone())?, f202(v3, v1)?))
        }
    }
}

pub fn f203(v0: List<Adt13>) -> List<u64> {
    let m712 = v0;
    match m712.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f296(v1), f203(v2))
        }
    }
}

pub fn f204(p715: List<u64>, v1: List<u64>) -> Option<u64> {
    let m714 = p715;
    {
        let mut m714 = m714;
        loop {
            let step = {
                let v0 = m714;
                let m713 = v0;
                match m713.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f297(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m714 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(p718: List<Bytes>, p719: List<Bytes>) -> Option<Bytes> {
    let m717 = (p718, p719);
    {
        let mut m717 = m717;
        loop {
            let step = {
                let (v0, v1) = m717;
                let m716 = v0;
                match m716.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f41(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m717 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt23 = f298(v0.clone(), 0u64, {
        let a720 = v0;
        length_bytes(a720)
    }, Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt23 = {
        let a723 = {
            let a722 = match v1.clone() {
                Adt23::C0(_, _, h721, _, _) => {
                    h721
                }
            };
            length_bytes(a722)
        };
        let a724 = 0u64;
        if if nat_eq(a723, a724) {
            let m726 = match v1.clone() {
                Adt23::C0(_, _, _, h725, _) => {
                    h725
                }
            };
            match m726.uncons() {
                None => {
                    true
                }
                Some((_, _)) => {
                    false
                }
            }
        } else {
            false
        } {
            v1
        } else {
            f299(v1)
        }
    };
    Ok(f300(match v4 {
        Adt23::C0(_, _, _, _, h727) => {
            h727
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f207(v0: List<List<Bytes>>) -> List<Bytes> {
    let m728 = v0;
    match m728.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m729 = v1;
            match m729.uncons() {
                None => {
                    f207(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f207(v2))
                }
            }
        }
    }
}

pub fn f208(v0: List<Bytes>) -> List<Bytes> {
    f93(Fn1::F301, List::<Bytes>::nil(), v0)
}

pub fn f209(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f302(v0, v1)
}

pub fn f210(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f303(f304(f305(v0))?))
}

pub fn f211(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f212(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m730 = f46(v0, v2)?;
    match m730 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f306(v1, f47(v3.clone(), f8(Str::lit("id"))))?;
            let m731 = v4.clone();
            match m731.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m733 = f78(f79({
                        let m732 = f79(v3.clone(), f8(Str::lit("source")));
                        match m732 {
                            None => {
                                Adt13::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m733 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f40(f41(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f307(f308(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f213(p736: List<Bytes>, p737: List<Bytes>) -> List<Bytes> {
    let m735 = (p736, p737);
    {
        let mut m735 = m735;
        loop {
            let step = {
                let (v0, v1) = m735;
                let m734 = v0;
                match m734.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f215(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m738 = v1;
    match m738.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt18 = f309(v0.clone(), v3)?;
            if f41(f310(f245(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f311(v5)?, f215(v0, v4, v2)?))
            } else {
                f215(v0, v4, v2)
            }
        }
    }
}

pub fn f216(v0: Adt1, p742: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m741 = p742;
    {
        let mut m741 = m741;
        loop {
            let step = {
                let v1 = m741;
                let m739 = v1;
                match m739.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt18 = f309(v0.clone(), v3.clone())?;
                        if if f41(f310(f245(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f28(f244({
                                let m740 = f242(v5.clone(), f8(Str::lit("schema")));
                                match m740 {
                                    None => {
                                        Adt18::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f53(v0.clone(), f311(v5)?))
                        } else {
                            false
                        } {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        }
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

pub fn f217(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h743, _) => {
            h743
        }
    }, f228())) {
        Ok(Adt11::C1(List::cons(f228(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f130(f125(f312(f127(match v0.clone() {
            Adt1::C0(_, h744, _) => {
                h744
            }
        }, f228())))?, v1);
        let m745 = f45(v0.clone(), List::cons(f132(), List::cons(f179(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m745.uncons() {
            None => {
                let v5: (bool, Bytes) = f313(v0.clone(), v2)?;
                let (h746, _) = v5;
                if h746 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h747) = v5;
                        h747
                    }))
                } else {
                    let m748 = f97(v0.clone());
                    match m748.uncons() {
                        None => {
                            let m749 = f99(v0.clone())?;
                            match m749.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h750) = v5;
                                        h750
                                    }, List::cons(f7(f125(f314(f100(v0.clone())?, f108(v0.clone(), f109(v0.clone())?)?))?), List::cons(f315(v0)?, List::<Bytes>::nil()))))))
                                }
                                Some((v8, v9)) => {
                                    Ok(Adt11::C0(List::cons(v8, v9)))
                                }
                            }
                        }
                        Some((v6, v7)) => {
                            Ok(Adt11::C1(List::cons(v6, v7)))
                        }
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f218(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f316(f82(v0, f132())?)?)))
}

pub fn f219() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f220(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m751 = v0;
    match m751.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f221(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt19>, v3: List<Adt13>, v4: List<Bytes>) -> List<Bytes> {
    let m752 = v1;
    match m752.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f98(f317(v5.clone(), v2.clone()), f318(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f319(v0.clone(), v5.clone()), List::cons(if f41(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a754 = {
                    let a753 = v7.clone();
                    length_list(a753)
                };
                let a755 = 0u64;
                if nat_eq(a754, a755) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f95(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f221(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f222(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m756 = v1;
    match m756.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f185(v2.clone(), f8(Str::lit("domain"))) {
                f320(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f47(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m757 = f79(v2.clone(), f8(Str::lit("role")));
                    match m757 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f47(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f47(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f321(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m758 = f79(v2.clone(), f8(Str::lit("origin")));
                    match m758 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f47(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m759 = f79(v2, f8(Str::lit("evidence")));
                    match m759 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f47(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f222(v0, v3))
            } else {
                f222(v0, v3)
            }
        }
    }
}

pub fn f223(v0: List<Adt13>) -> List<Bytes> {
    let m760 = v0;
    match m760.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f47(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f47(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f47(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f223(v2))
        }
    }
}

pub fn f224(v0: List<Adt13>) -> List<Bytes> {
    let m761 = v0;
    match m761.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f47(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f322(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f224(v2))
        }
    }
}

pub fn f225(p767: List<Bytes>) -> R<bool> {
    let m766 = p767;
    {
        let mut m766 = m766;
        loop {
            let step = {
                let v0 = m766;
                let m762 = v0;
                match m762.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a764 = {
                            let a763 = f173(v1)?;
                            length_bytes(a763)
                        };
                        let a765 = 0u64;
                        if nat_eq(a764, a765) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m766 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: List<Bytes>) -> R<bool> {
    if f323(f260(v0.clone())?) {
        Ok(f40({
            let a769 = {
                let a768 = f324(f260(v0)?);
                length_list(a768)
            };
            let a770 = 0u64;
            nat_eq(a769, a770)
        }))
    } else {
        Ok(false)
    }
}

pub fn f227(v0: List<Bytes>) -> R<bool> {
    if f323(f236(v0.clone(), f8(Str::lit("requirement")))?) {
        if f323(f236(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f40({
                let a772 = {
                    let a771 = f188(f324(f236(v0.clone(), f8(Str::lit("requirement")))?), f324(f236(v0, f8(Str::lit("admission")))?));
                    length_list(a771)
                };
                let a773 = 0u64;
                nat_eq(a772, a773)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f228() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f229(v0: Bytes) -> R<Bytes> {
    Ok(f34(f33(v0, 47u8)?))
}

pub fn f230() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f231(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m774 = v0;
    match m774.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f173(v1)?;
            let a776 = {
                let a775 = v3.clone();
                length_bytes(a775)
            };
            let a777 = 0u64;
            if nat_eq(a776, a777) {
                f231(v2)
            } else {
                let m778 = f77(v3.clone())?;
                match m778 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        if f325(f47(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m779 = f231(v2)?;
                            match m779 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f326(v5), {
                                        let a780 = v3;
                                        let a781 = f8(Str::lit("\u{a}"));
                                        append_bytes(a780, a781)
                                    }), v6)))
                                }
                            }
                        } else {
                            Ok(None::<List<(Bytes, Bytes)>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f232() -> Bytes {
    let a782 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a782)
}

pub fn f233(p787: List<Bytes>) -> R<Option<Bytes>> {
    let m786 = p787;
    {
        let mut m786 = m786;
        loop {
            let step = {
                let v0 = m786;
                let m783 = v0;
                match m783.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m784 = f77(f173(v1)?)?;
                        match m784 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m785 = f79(v3.clone(), f8(Str::lit("source")));
                                match m785 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f327(f47(v4.clone(), f8(Str::lit("redistribution"))), f47(v4, f8(Str::lit("url"))))? {
                                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f47(v3, f8(Str::lit("id"))))))
                                        } else {
                                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m786 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: Bytes) -> Bytes {
    let a792 = {
        let a788 = Str::lit("binding ");
        utf8_encode(a788)
    };
    let a793 = {
        let a790 = v0;
        let a791 = {
            let a789 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a789)
        };
        append_bytes(a790, a791)
    };
    append_bytes(a792, a793)
}

pub fn f235(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m794 = v0;
    match m794.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f173(v1)?;
            let a796 = {
                let a795 = v3.clone();
                length_bytes(a795)
            };
            let a797 = 0u64;
            if nat_eq(a796, a797) {
                f235(v2)
            } else {
                let m798 = f159(v3.clone())?;
                match m798 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f328(v4.clone()) {
                            let m799 = f235(v2)?;
                            match m799 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f214(), List::cons(f8(Str::lit("/")), List::cons(f329(f245(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a800 = v3;
                                        let a801 = f8(Str::lit("\u{a}"));
                                        append_bytes(a800, a801)
                                    }), v5)))
                                }
                            }
                        } else {
                            Ok(None::<List<(Bytes, Bytes)>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f236(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt13>>> {
    let m802 = v0;
    match m802.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f173(v2)?;
            let a804 = {
                let a803 = v4.clone();
                length_bytes(a803)
            };
            let a805 = 0u64;
            if nat_eq(a804, a805) {
                f236(v3, v1)
            } else {
                let m806 = f77(v4)?;
                match m806 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f330(v5.clone());
                        let a808 = {
                            let a807 = v6.clone();
                            length_bytes(a807)
                        };
                        let a809 = 0u64;
                        if nat_eq(a808, a809) {
                            Ok(None::<List<Adt13>>)
                        } else {
                            let m810 = f236(v3, v1.clone())?;
                            match m810 {
                                None => {
                                    Ok(None::<List<Adt13>>)
                                }
                                Some(v7) => {
                                    if f28(v6, v1) {
                                        Ok(Some(List::cons(v5, v7)))
                                    } else {
                                        Ok(Some(v7))
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

pub fn f237(p813: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m812 = p813;
    {
        let mut m812 = m812;
        loop {
            let step = {
                let v0 = m812;
                let m811 = v0;
                match m811.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f41(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f239(v0: Bytes, v1: List<Adt13>) -> R<(Bytes, Bytes)> {
    Ok((v0, f86(Adt13::C3(std::rc::Rc::new(f87(v1))))?))
}

pub fn f240(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f86(Adt13::C3(std::rc::Rc::new(f331(f125(f98(v1, f122(f82(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f241(v0: Adt1, v1: Bytes) -> R<Option<Adt18>> {
    f159(f53(v0, v1))
}

pub fn f242(p819: Adt18, v1: Bytes) -> Option<Adt18> {
    let m818 = p819;
    {
        let mut m818 = m818;
        loop {
            let step = {
                let v0 = m818;
                let m814 = v0;
                match m814 {
                    Adt18::C0(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C1(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C2(_, _, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C3(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C4(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C5(r815) => {
                        let v5 = (*r815).clone();
                        Ok::<Adt18, Option<Adt18>>(v5)
                    }
                    Adt18::C6 => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C7(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C8(v2, r816, r817) => {
                        let v3 = (*r816).clone();
                        let v4 = (*r817).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt18, Option<Adt18>>(Some(v3))
                        } else {
                            Ok::<Adt18, Option<Adt18>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m818 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: Adt1, v1: Bytes, v2: Adt18, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m820 = f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m820.uncons() {
        None => {
            let m822 = f237(f104(List::cons(Adt13::C4(std::rc::Rc::new({
                let m821 = f256(v4.clone());
                match m821 {
                    None => {
                        Adt13::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt13>::nil())), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m822 {
                None => {
                    f332(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f238(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f244(v0: Adt18) -> R<Bytes> {
    let a823 = f333(v0)?;
    let a824 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a823, a824))
}

pub fn f245(v0: Adt18, v1: Bytes) -> Bytes {
    let m825 = f334(f242(v0, v1));
    match m825 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f246(v0: List<Adt18>) -> List<Bytes> {
    let m826 = v0;
    match m826.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m827 = f334(Some(v1));
            match m827 {
                None => {
                    f246(v2)
                }
                Some(v3) => {
                    List::cons(v3, f246(v2))
                }
            }
        }
    }
}

pub fn f247(v0: Option<Adt18>) -> List<Adt18> {
    let m828 = v0;
    match m828 {
        None => {
            List::<Adt18>::nil()
        }
        Some(v1) => {
            f335(v1)
        }
    }
}

pub fn f248() -> Bytes {
    f336()
}

pub fn f249(v0: Adt1, v1: Adt13, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f254(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f337(v1, v4)?)?), List::cons(f240(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f250(v0: Bytes) -> R<Bytes> {
    Ok(f338(f339(v0)?))
}

pub fn f251(v0: List<List<Bytes>>) -> List<Bytes> {
    let m829 = v0;
    match m829.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f252() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f253(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f307(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f254(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f340(f129(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f341(f342(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f255(p832: List<Adt13>) -> bool {
    let m831 = p832;
    {
        let mut m831 = m831;
        loop {
            let step = {
                let v0 = m831;
                let m830 = v0;
                match m830.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f28(f47(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f41(f8(Str::lit("studio")), f121(f343(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
                        } else {
                            false
                        } {
                            Err::<List<Adt13>, bool>(true)
                        } else {
                            Ok::<List<Adt13>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m831 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: List<Bytes>) -> Option<Adt13> {
    let m833 = v0;
    match m833.uncons() {
        None => {
            None::<Adt13>
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("req"))) {
                let m834 = v2;
                match m834.uncons() {
                    None => {
                        None::<Adt13>
                    }
                    Some((v3, _)) => {
                        Some(Adt13::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5)))
                    }
                }
            } else {
                if f28(v1.clone(), f8(Str::lit("std"))) {
                    let m835 = v2;
                    match m835.uncons() {
                        None => {
                            None::<Adt13>
                        }
                        Some((v5, v6)) => {
                            Some(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v5)), std::rc::Rc::new({
                                let m836 = v6;
                                match m836.uncons() {
                                    None => {
                                        Adt13::C5
                                    }
                                    Some((v7, _)) => {
                                        Adt13::C7(f8(Str::lit("row")), std::rc::Rc::new(Adt13::C2(v7)), std::rc::Rc::new(Adt13::C5))
                                    }
                                }
                            }))))), std::rc::Rc::new(Adt13::C5)))
                        }
                    }
                } else {
                    if f28(v1, f8(Str::lit("der"))) {
                        let m837 = v2;
                        match m837.uncons() {
                            None => {
                                None::<Adt13>
                            }
                            Some((v9, v10)) => {
                                let m838 = v10;
                                match m838.uncons() {
                                    None => {
                                        None::<Adt13>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt13::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt13::C2(v11)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f331(v12)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt13::C2(v9)), std::rc::Rc::new(Adt13::C5))))))))), std::rc::Rc::new(Adt13::C5)))
                                    }
                                }
                            }
                        }
                    } else {
                        None::<Adt13>
                    }
                }
            }
        }
    }
}

pub fn f257(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt24> {
    let v3: List<(Bytes, Bytes)> = f344(v1)?;
    f345({
        let a840 = {
            let a839 = v3.clone();
            length_list(a839)
        };
        let a841 = 2u64;
        nat_add(a840, a841)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt25>::nil(), List::<Bytes>::nil())
}

pub fn f258(v0: Adt1, v1: Bytes, v2: Adt13, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt11> {
    let v6: Bytes = {
        let a842 = f8(Str::lit("Catalog."));
        let a843 = f329(v1.clone())?;
        append_bytes(a842, a843)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f329(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt11::C3(f346(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f347(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f347(f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt13::C2(v7.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt13::C2(v4)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt13::C1(false, 0u64)), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a847 = f348(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f35(v7.clone(), 4u64, {
            let a845 = {
                let a844 = v7;
                length_bytes(a844)
            };
            let a846 = 4u64;
            nat_sub(a845, a846)
        }))?;
        let a848 = f8(Str::lit("\u{a}"));
        append_bytes(a847, a848)
    }), List::cons(f240(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f259(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f329(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f260(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m849 = v0;
    match m849.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f173(v1)?;
            let a851 = {
                let a850 = v3.clone();
                length_bytes(a850)
            };
            let a852 = 0u64;
            if nat_eq(a851, a852) {
                f260(v2)
            } else {
                let m853 = f77(v3)?;
                match m853 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if if f185(v4.clone(), f8(Str::lit("agent"))) {
                            if f185(v4.clone(), f8(Str::lit("at"))) {
                                if f185(v4.clone(), f8(Str::lit("quote"))) {
                                    f349(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m854 = f260(v2)?;
                            match m854 {
                                None => {
                                    Ok(None::<List<Adt13>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons(v4, v5)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt13>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f261(v0: u64) -> bool {
    let a855 = v0;
    let a856 = 127u64;
    nat_le(a855, a856)
}

pub fn f262(v0: Bytes) -> R<Bytes> {
    f350(v0.clone(), 0u64, {
        let a857 = v0;
        length_bytes(a857)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f263(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt18, u64)>> {
    let m858 = v2;
    if m858 == 0 {
        Ok(None::<(Adt18, u64)>)
    } else {
        let v4 = m858 - 1;
        let v5: u64 = f351(v0.clone(), v1)?;
        let a859 = v3;
        let a860 = 1u64;
        if nat_eq(a859, a860) {
            let m861 = f263(v0.clone(), v1, v4, 0u64)?;
            match m861 {
                None => {
                    Ok(None::<(Adt18, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f351(v0.clone(), {
                        let (_, h862) = v6.clone();
                        h862
                    })?;
                    let a863 = v7;
                    let a864 = 44u64;
                    if nat_eq(a863, a864) {
                        let m868 = f263(v0, {
                            let a866 = {
                                let (_, h865) = v6;
                                h865
                            };
                            let a867 = 1u64;
                            nat_add(a866, a867)?
                        }, v4, 1u64)?;
                        match m868 {
                            None => {
                                Ok(None::<(Adt18, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt18::C7(std::rc::Rc::new({
                                    let (h869, _) = v6;
                                    h869
                                }), std::rc::Rc::new({
                                    let (h870, _) = v8;
                                    h870
                                })), {
                                    let (_, h871) = v8;
                                    h871
                                })))
                            }
                        }
                    } else {
                        let a872 = v7;
                        let a873 = 93u64;
                        if nat_eq(a872, a873) {
                            Ok(Some((Adt18::C7(std::rc::Rc::new({
                                let (h874, _) = v6;
                                h874
                            }), std::rc::Rc::new(Adt18::C6)), {
                                let a876 = {
                                    let (_, h875) = v6;
                                    h875
                                };
                                let a877 = 1u64;
                                nat_add(a876, a877)?
                            })))
                        } else {
                            Ok(None::<(Adt18, u64)>)
                        }
                    }
                }
            }
        } else {
            let a878 = v3;
            let a879 = 2u64;
            if nat_eq(a878, a879) {
                let m882 = f352(v0.clone(), {
                    let a880 = v1;
                    let a881 = 1u64;
                    nat_add(a880, a881)?
                })?;
                match m882 {
                    None => {
                        Ok(None::<(Adt18, u64)>)
                    }
                    Some(v9) => {
                        if f353({
                            let a883 = v5;
                            let a884 = 34u64;
                            nat_eq(a883, a884)
                        }, {
                            let a886 = f351(v0.clone(), {
                                let (h885, _) = v9.clone();
                                h885
                            })?;
                            let a887 = 58u64;
                            nat_eq(a886, a887)
                        }) {
                            let m891 = f263(v0.clone(), {
                                let a889 = {
                                    let (h888, _) = v9;
                                    h888
                                };
                                let a890 = 1u64;
                                nat_add(a889, a890)?
                            }, v4, 0u64)?;
                            match m891 {
                                None => {
                                    Ok(None::<(Adt18, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f351(v0.clone(), {
                                        let (_, h892) = v10.clone();
                                        h892
                                    })?;
                                    let a893 = v11;
                                    let a894 = 44u64;
                                    if nat_eq(a893, a894) {
                                        let m898 = f263(v0, {
                                            let a896 = {
                                                let (_, h895) = v10;
                                                h895
                                            };
                                            let a897 = 1u64;
                                            nat_add(a896, a897)?
                                        }, v4, 2u64)?;
                                        match m898 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt18::C8({
                                                    let (_, h899) = v9;
                                                    h899
                                                }, std::rc::Rc::new({
                                                    let (h900, _) = v10;
                                                    h900
                                                }), std::rc::Rc::new({
                                                    let (h901, _) = v12;
                                                    h901
                                                })), {
                                                    let (_, h902) = v12;
                                                    h902
                                                })))
                                            }
                                        }
                                    } else {
                                        let a903 = v11;
                                        let a904 = 125u64;
                                        if nat_eq(a903, a904) {
                                            Ok(Some((Adt18::C8({
                                                let (_, h905) = v9;
                                                h905
                                            }, std::rc::Rc::new({
                                                let (h906, _) = v10;
                                                h906
                                            }), std::rc::Rc::new(Adt18::C6)), {
                                                let a908 = {
                                                    let (_, h907) = v10;
                                                    h907
                                                };
                                                let a909 = 1u64;
                                                nat_add(a908, a909)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt18, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt18, u64)>)
                        }
                    }
                }
            } else {
                let a910 = v5;
                let a911 = 34u64;
                if nat_eq(a910, a911) {
                    let m914 = f352(v0, {
                        let a912 = v1;
                        let a913 = 1u64;
                        nat_add(a912, a913)?
                    })?;
                    match m914 {
                        None => {
                            Ok(None::<(Adt18, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt18::C3({
                                let (_, h915) = v13;
                                h915
                            }), {
                                let (h916, _) = v13;
                                h916
                            })))
                        }
                    }
                } else {
                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt18::C0(true), {
                            let a917 = v1;
                            let a918 = 4u64;
                            nat_add(a917, a918)?
                        })))
                    } else {
                        if f28(f35(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt18::C0(false), {
                                let a919 = v1;
                                let a920 = 5u64;
                                nat_add(a919, a920)?
                            })))
                        } else {
                            let a921 = v5;
                            let a922 = 91u64;
                            if nat_eq(a921, a922) {
                                let a925 = f351(v0.clone(), {
                                    let a923 = v1;
                                    let a924 = 1u64;
                                    nat_add(a923, a924)?
                                })?;
                                let a926 = 93u64;
                                if nat_eq(a925, a926) {
                                    Ok(Some((Adt18::C4(std::rc::Rc::new(Adt18::C6)), {
                                        let a927 = v1;
                                        let a928 = 2u64;
                                        nat_add(a927, a928)?
                                    })))
                                } else {
                                    let m931 = f263(v0, {
                                        let a929 = v1;
                                        let a930 = 1u64;
                                        nat_add(a929, a930)?
                                    }, v4, 1u64)?;
                                    match m931 {
                                        None => {
                                            Ok(None::<(Adt18, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt18::C4(std::rc::Rc::new({
                                                let (h932, _) = v14;
                                                h932
                                            })), {
                                                let (_, h933) = v14;
                                                h933
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a934 = v5;
                                let a935 = 123u64;
                                if nat_eq(a934, a935) {
                                    let a938 = f351(v0.clone(), {
                                        let a936 = v1;
                                        let a937 = 1u64;
                                        nat_add(a936, a937)?
                                    })?;
                                    let a939 = 125u64;
                                    if nat_eq(a938, a939) {
                                        Ok(Some((Adt18::C5(std::rc::Rc::new(Adt18::C6)), {
                                            let a940 = v1;
                                            let a941 = 2u64;
                                            nat_add(a940, a941)?
                                        })))
                                    } else {
                                        let m944 = f263(v0, {
                                            let a942 = v1;
                                            let a943 = 1u64;
                                            nat_add(a942, a943)?
                                        }, v4, 2u64)?;
                                        match m944 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt18::C5(std::rc::Rc::new({
                                                    let (h945, _) = v15;
                                                    h945
                                                })), {
                                                    let (_, h946) = v15;
                                                    h946
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m947 = f354(v0, v1)?;
                                    match m947 {
                                        None => {
                                            Ok(None::<(Adt18, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h948) = v16;
                                                h948
                                            }, {
                                                let (h949, _) = v16;
                                                h949
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

pub fn f264(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h950, _) = v1.clone();
        h950
    };
    let v3: u64 = {
        let (h951, _) = {
            let (_, h952) = v1.clone();
            h952
        };
        h951
    };
    let v4: List<Bytes> = {
        let (_, h953) = {
            let (_, h954) = v1;
            h954
        };
        h953
    };
    let m955 = f174(v0.clone(), v2);
    match m955 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a956 = v5;
            let a957 = 10u8;
            if equal(a956, a957) {
                Ok(Some(({
                    let a958 = v2;
                    let a959 = 1u64;
                    nat_add(a958, a959)?
                }, ({
                    let a960 = v2;
                    let a961 = 1u64;
                    nat_add(a960, a961)?
                }, List::cons(f355(f35(v0, v3, {
                    let a962 = v2;
                    let a963 = v3;
                    nat_sub(a962, a963)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a964 = v2;
                    let a965 = 1u64;
                    nat_add(a964, a965)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f265(v0: Adt13) -> List<Adt13> {
    let m966 = v0;
    match m966 {
        Adt13::C0(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C1(_, _) => {
            List::<Adt13>::nil()
        }
        Adt13::C2(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C3(r967) => {
            let v3 = (*r967).clone();
            f265(v3)
        }
        Adt13::C4(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C5 => {
            List::<Adt13>::nil()
        }
        Adt13::C6(r968, r969) => {
            let v1 = (*r968).clone();
            let v2 = (*r969).clone();
            List::cons(v1, f265(v2))
        }
        Adt13::C7(_, _, _) => {
            List::<Adt13>::nil()
        }
    }
}

pub fn f266(v0: Adt13, v1: Bytes) -> Adt13 {
    let m970 = v0;
    match m970 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r971) => {
            let v10 = (*r971).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r972) => {
            let v5 = (*r972).clone();
            Adt13::C4(std::rc::Rc::new(f266(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r973, r974) => {
            let v11 = (*r973).clone();
            let v12 = (*r974).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r975, r976) => {
            let v3 = (*r975).clone();
            let v4 = (*r976).clone();
            if f28(v2.clone(), f8(Str::lit("authority"))) {
                Adt13::C7(v2, std::rc::Rc::new(f356(v3, v1.clone())), std::rc::Rc::new(f266(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f266(v4, v1)))
            }
        }
    }
}

pub fn f267(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a977 = 0u64;
        let a978 = v1;
        nat_lt(a977, a978)
    } else {
        false
    } {
        let a979 = Bytes::lit(&[0x2d]);
        let a980 = f357(v1)?;
        Ok(append_bytes(a979, a980))
    } else {
        f357(v1)
    }
}

pub fn f268(v0: Bytes) -> R<Bytes> {
    let a986 = {
        let a984 = Bytes::lit(&[0x22]);
        let a985 = {
            let (_, h981) = {
                let (h982, _) = f358(Fn4::F359(v0.clone()), {
                    let a983 = v0;
                    length_bytes(a983)
                }, (0u64, Bytes::lit(&[])))?;
                h982
            };
            h981
        };
        append_bytes(a984, a985)
    };
    let a987 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a986, a987))
}

pub fn f269(v0: List<Bytes>) -> Bytes {
    let a990 = {
        let a988 = Bytes::lit(&[0x5b]);
        let a989 = f95(v0, Bytes::lit(&[0x2c]));
        append_bytes(a988, a989)
    };
    let a991 = Bytes::lit(&[0x5d]);
    append_bytes(a990, a991)
}

pub fn f270(v0: Adt13) -> R<List<Bytes>> {
    let m992 = v0;
    match m992 {
        Adt13::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C6(r993, r994) => {
            let v7 = (*r993).clone();
            let v8 = (*r994).clone();
            Ok(List::cons(f165(v7)?, f270(v8)?))
        }
        Adt13::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f271(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f360(Fn5::F361, List::<(Bytes, Bytes)>::nil(), f362(v0)?);
    let v2: List<Bytes> = f363(Fn6::F364, List::<Bytes>::nil(), v1)?;
    let a997 = {
        let a995 = Bytes::lit(&[0x7b]);
        let a996 = f95(v2, Bytes::lit(&[0x2c]));
        append_bytes(a995, a996)
    };
    let a998 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a997, a998))
}

pub fn f272(v0: Adt13) -> R<List<(Bytes, Bytes)>> {
    let m999 = v0;
    match m999 {
        Adt13::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C7(v9, r1000, r1001) => {
            let v10 = (*r1000).clone();
            let v11 = (*r1001).clone();
            Ok(List::cons((v9, f165(v10)?), f272(v11)?))
        }
    }
}

pub fn f273() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f274(v0: Adt12) -> Bytes {
    let m1002 = v0;
    match m1002 {
        Adt12::C0 => {
            f8(Str::lit("model/authorities.toml"))
        }
        Adt12::C1 => {
            f8(Str::lit("model/ids.toml"))
        }
        Adt12::C2 => {
            f8(Str::lit("site/generated/release-ledger.json"))
        }
        Adt12::C3 => {
            f8(Str::lit("standards.lock"))
        }
    }
}

pub fn f275(v0: Adt12, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f365(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f365(f8(Str::lit("id")), f58(v0.clone(), f50(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f365(f8(Str::lit("name")), f366(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f365(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f365(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f365(f8(Str::lit("statement")), f367(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f365(f8(Str::lit("media_type")), f368(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f365(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f365(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f365(f8(Str::lit("url")), f168(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f276(v0: Bytes) -> R<u64> {
    let (h1003, _) = f287(Fn3::F369(v0.clone()), {
        let a1004 = v0.clone();
        length_bytes(a1004)
    }, {
        let a1005 = v0;
        length_bytes(a1005)
    })?;
    Ok(h1003)
}

pub fn f277(v0: Bytes, v1: u64) -> R<u64> {
    let (h1006, _) = f287(Fn3::F370(v1, v0), v1, 0u64)?;
    Ok(h1006)
}

pub fn f278(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m1007 = v0;
    match m1007.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1008 = f79(v1, f8(Str::lit("std")));
            match m1008 {
                None => {
                    f278(v2)
                }
                Some(v3) => {
                    if f185(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f47(v3.clone(), f8(Str::lit("row"))), f47(v3, f8(Str::lit("authority")))), f278(v2))
                    } else {
                        f278(v2)
                    }
                }
            }
        }
    }
}

pub fn f279(v0: Adt14, v1: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m1009 = v1;
    match m1009.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f47(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f47(v2, f8(Str::lit("object")));
            if if f41(v4.clone(), match v0.clone() {
                Adt14::C0(h1010, _, _) => {
                    h1010
                }
            }) {
                f41(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1011, _) => {
                        h1011
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f279(v0, v3))
            } else {
                if if f41(v5.clone(), match v0.clone() {
                    Adt14::C0(h1012, _, _) => {
                        h1012
                    }
                }) {
                    f41(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1013, _) => {
                            h1013
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f279(v0, v3))
                } else {
                    f279(v0, v3)
                }
            }
        }
    }
}

pub fn f280(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt19>> {
    let m1014 = v0;
    match m1014.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v4, v5)) => {
            if if f371(v4.clone(), v2.clone()) {
                true
            } else {
                f371(v4.clone(), v3.clone())
            } {
                f280(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt19::C0({
                    let (h1015, _) = v4.clone();
                    h1015
                }, {
                    let a1018 = {
                        let a1017 = f125(f372({
                            let (h1016, _) = v4.clone();
                            h1016
                        }, v1.clone()))?;
                        length_list(a1017)
                    };
                    let a1019 = 1u64;
                    if nat_eq(a1018, a1019) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1020) = v4.clone();
                    h1020
                }), f280(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f281(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1021 = v1;
    match m1021.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41({
                let (h1022, _) = v2.clone();
                h1022
            }, v0.clone()) {
                List::cons({
                    let (_, h1023) = v2;
                    h1023
                }, f281(v0, v3))
            } else {
                if f41({
                    let (_, h1024) = v2;
                    h1024
                }, v0.clone()) {
                    List::cons({
                        let (h1025, _) = v2;
                        h1025
                    }, f281(v0, v3))
                } else {
                    f281(v0, v3)
                }
            }
        }
    }
}

pub fn f282(v0: Adt14, v1: Adt13, v2: Bytes) -> bool {
    if f41(v2.clone(), match v0.clone() {
        Adt14::C0(h1026, _, _) => {
            h1026
        }
    }) {
        true
    } else {
        if f41(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1027, _) => {
                h1027
            }
        }) {
            true
        } else {
            if f41(v2, match v0.clone() {
                Adt14::C0(_, _, h1028) => {
                    h1028
                }
            }) {
                true
            } else {
                if f185(v1.clone(), f8(Str::lit("scheme"))) {
                    f41(f47(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1029, _) => {
                            h1029
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f283(v0: Adt13) -> bool {
    if f185(v0.clone(), f8(Str::lit("domain"))) {
        if f185(v0.clone(), f8(Str::lit("row"))) {
            if f41(f47(v0.clone(), f8(Str::lit("relation"))), f373()) {
                if f374(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f374(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f374(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f374(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f284(v0: Adt13) -> bool {
    if f185(v0.clone(), f8(Str::lit("subject"))) {
        if f185(v0.clone(), f8(Str::lit("object"))) {
            if f185(v0.clone(), f8(Str::lit("evidence"))) {
                f41(f47(v0, f8(Str::lit("relation"))), f375())
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f285(v0: Bytes) -> bool {
    f190(v0, f8(Str::lit(".lex.tex")))
}

pub fn f286(v0: Bytes) -> R<bool> {
    let v1: Bytes = f173(v0)?;
    let v2: u64 = if f72(v1.clone(), f8(Str::lit("if ! "))) {
        5u64
    } else {
        if f72(v1.clone(), f8(Str::lit("elif ! "))) {
            7u64
        } else {
            0u64
        }
    };
    let v3: Bytes = f35(v1.clone(), v2, {
        let a1031 = {
            let a1030 = v1;
            length_bytes(a1030)
        };
        let a1032 = v2;
        nat_sub(a1031, a1032)
    });
    let v5: Bytes = if f72(v3.clone(), f8(Str::lit("("))) {
        let m1033 = f194(v3.clone(), f8(Str::lit(")")))?;
        match m1033 {
            None => {
                v3
            }
            Some(v4) => {
                f35(v3.clone(), {
                    let a1034 = v4;
                    let a1035 = 1u64;
                    nat_add(a1034, a1035)?
                }, {
                    let a1039 = {
                        let a1036 = v3;
                        length_bytes(a1036)
                    };
                    let a1040 = {
                        let a1037 = v4;
                        let a1038 = 1u64;
                        nat_add(a1037, a1038)?
                    };
                    nat_sub(a1039, a1040)
                })
            }
        }
    } else {
        v3
    };
    let a1041 = 0u64;
    let a1042 = v2;
    if nat_lt(a1041, a1042) {
        f376(v5)
    } else {
        Ok(false)
    }
}

pub fn f287(v0: Fn3, p1047: u64, p1048: u64) -> R<(u64, bool)> {
    let m1046 = (p1047, p1048);
    {
        let mut m1046 = m1046;
        loop {
            let step = {
                let (v1, v2) = m1046;
                let m1043 = v1;
                if m1043 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1043 - 1;
                    let m1045 = {
                        let c1044: Fn3 = v0.clone();
                        c1044.apply(v2)?
                    };
                    match m1045 {
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
                Ok(next) => { m1046 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f289(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1051 = {
            let a1049 = v4;
            let a1050 = v1;
            nat_add(a1049, a1050)?
        };
        let a1052 = v3;
        if nat_lt(a1051, a1052) {
            Ok(Some({
                let a1053 = v4;
                let a1054 = 1u64;
                nat_add(a1053, a1054)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f289(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1059 = {
        let a1056 = v0;
        let a1057 = v2;
        let a1058 = {
            let a1055 = v1.clone();
            length_bytes(a1055)
        };
        slice_bytes(a1056, a1057, a1058)
    };
    match m1059 {
        None => {
            false
        }
        Some(v3) => {
            let a1060 = v3;
            let a1061 = v1;
            equal(a1060, a1061)
        }
    }
}

pub fn f290(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1062 = v0;
    match m1062.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f290(v3, {
                let a1063 = v1;
                let a1064 = 1u64;
                nat_add(a1063, a1064)?
            })?))
        }
    }
}

pub fn f291(v0: Adt17, v1: List<u64>, v2: Adt21) -> bool {
    let m1065 = v2;
    match m1065 {
        Adt21::C0(v3, v4) => {
            if f297(v3, match v0.clone() {
                Adt17::C0(h1066, _, _, _) => {
                    h1066
                }
            }) {
                f377(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f297(v5, match v0 {
                Adt17::C0(_, _, h1067, _) => {
                    h1067
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f293(v7, v1) {
                f297(v8, match v0 {
                    Adt17::C0(_, _, _, h1068) => {
                        h1068
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f292(v0: Adt15, v1: Adt13) -> R<Adt21> {
    let m1069 = f79(v1.clone(), f8(Str::lit("std")));
    match m1069 {
        None => {
            let m1074 = f78(f79(v1.clone(), f8(Str::lit("req"))));
            match m1074 {
                None => {
                    let m1076 = f79(v1, f8(Str::lit("der")));
                    match m1076 {
                        None => {
                            Ok(f378(v0))
                        }
                        Some(v5) => {
                            let m1077 = f379(f47(v5.clone(), f8(Str::lit("rule"))));
                            match m1077 {
                                None => {
                                    Ok(f378(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f202(f122(f163(f79(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1078) => {
                                            h1078
                                        }
                                    })?, f295(f47(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1079, _) => {
                                            h1079
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f295(v4, match v0 {
                        Adt15::C0(_, _, h1075, _, _) => {
                            h1075
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1070 = f78(f79(v2.clone(), f8(Str::lit("row"))));
            match m1070 {
                None => {
                    Ok(Adt21::C0(f295(f47(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1071, _, _, _, _) => {
                            h1071
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f295(f47(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1072, _, _, _, _) => {
                            h1072
                        }
                    })?, Some(f295(v3, match v0 {
                        Adt15::C0(_, h1073, _, _, _) => {
                            h1073
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f293(p1082: List<u64>, v1: List<u64>) -> bool {
    let m1081 = p1082;
    {
        let mut m1081 = m1081;
        loop {
            let step = {
                let v0 = m1081;
                let m1080 = v0;
                match m1080.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f297(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1081 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(p1087: List<u64>) -> bool {
    let m1086 = p1087;
    {
        let mut m1086 = m1086;
        loop {
            let step = {
                let v0 = m1086;
                let m1083 = v0;
                match m1083.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1084 = v1;
                        let a1085 = 7u64;
                        if nat_le(a1084, a1085) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1086 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1088 = f380(v0, v1.clone(), 0u64)?;
    match m1088 {
        None => {
            let a1089 = v1;
            Ok(length_list(a1089))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f296(v0: Adt13) -> u64 {
    let m1090 = f381(f79(v0, f8(Str::lit("stratum"))));
    match m1090 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f297(v0: u64, p1095: List<u64>) -> bool {
    let m1094 = p1095;
    {
        let mut m1094 = m1094;
        loop {
            let step = {
                let v1 = m1094;
                let m1091 = v1;
                match m1091.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1092 = v2;
                        let a1093 = v0;
                        if nat_eq(a1092, a1093) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1094 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: Bytes, p1153: u64, p1154: u64, p1155: Adt23) -> R<Adt23> {
    let m1152 = (p1153, (p1154, p1155));
    {
        let mut m1152 = m1152;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1152;
                let m1096 = v2;
                if m1096 == 0 {
                    Err::<(u64, (u64, Adt23)), R<Adt23>>(Ok(v3))
                } else {
                    let v4 = m1096 - 1;
                    let v5: u64 = f382(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt23::C0(_, h1097, _, _, _) => {
                            h1097
                        }
                    } {
                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                            let a1098 = v1;
                            let a1099 = 1u64;
                            nat_add(a1098, a1099)?
                        }, (v4, Adt23::C0(true, false, match v3.clone() {
                            Adt23::C0(_, _, h1100, _, _) => {
                                h1100
                            }
                        }, match v3.clone() {
                            Adt23::C0(_, _, _, h1101, _) => {
                                h1101
                            }
                        }, match v3 {
                            Adt23::C0(_, _, _, _, h1102) => {
                                h1102
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt23::C0(h1103, _, _, _, _) => {
                                h1103
                            }
                        } {
                            let a1104 = v5;
                            let a1105 = 34u64;
                            if nat_eq(a1104, a1105) {
                                let a1108 = f382(v0.clone(), {
                                    let a1106 = v1;
                                    let a1107 = 1u64;
                                    nat_add(a1106, a1107)?
                                })?;
                                let a1109 = 34u64;
                                if nat_eq(a1108, a1109) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1110 = v1;
                                        let a1111 = 1u64;
                                        nat_add(a1110, a1111)?
                                    }, (v4, Adt23::C0(true, true, f383(match v3.clone() {
                                        Adt23::C0(_, _, h1112, _, _) => {
                                            h1112
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt23::C0(_, _, _, h1113, _) => {
                                            h1113
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1114) => {
                                            h1114
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1115 = v1;
                                        let a1116 = 1u64;
                                        nat_add(a1115, a1116)?
                                    }, (v4, Adt23::C0(false, false, match v3.clone() {
                                        Adt23::C0(_, _, h1117, _, _) => {
                                            h1117
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1118, _) => {
                                            h1118
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1119) => {
                                            h1119
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1120 = v1;
                                    let a1121 = 1u64;
                                    nat_add(a1120, a1121)?
                                }, (v4, Adt23::C0(true, false, f383(match v3.clone() {
                                    Adt23::C0(_, _, h1122, _, _) => {
                                        h1122
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt23::C0(_, _, _, h1123, _) => {
                                        h1123
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1124) => {
                                        h1124
                                    }
                                }))))
                            }
                        } else {
                            let a1125 = v5;
                            let a1126 = 34u64;
                            if nat_eq(a1125, a1126) {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1127 = v1;
                                    let a1128 = 1u64;
                                    nat_add(a1127, a1128)?
                                }, (v4, Adt23::C0(true, false, match v3.clone() {
                                    Adt23::C0(_, _, h1129, _, _) => {
                                        h1129
                                    }
                                }, match v3.clone() {
                                    Adt23::C0(_, _, _, h1130, _) => {
                                        h1130
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1131) => {
                                        h1131
                                    }
                                }))))
                            } else {
                                let a1132 = v5;
                                let a1133 = 44u64;
                                if nat_eq(a1132, a1133) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1134 = v1;
                                        let a1135 = 1u64;
                                        nat_add(a1134, a1135)?
                                    }, (v4, Adt23::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt23::C0(_, _, h1136, _, _) => {
                                            h1136
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1137, _) => {
                                            h1137
                                        }
                                    }), match v3 {
                                        Adt23::C0(_, _, _, _, h1138) => {
                                            h1138
                                        }
                                    }))))
                                } else {
                                    let a1139 = v5;
                                    let a1140 = 13u64;
                                    if nat_eq(a1139, a1140) {
                                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                            let a1141 = v1;
                                            let a1142 = 1u64;
                                            nat_add(a1141, a1142)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1143 = v5;
                                        let a1144 = 10u64;
                                        if nat_eq(a1143, a1144) {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1145 = v1;
                                                let a1146 = 1u64;
                                                nat_add(a1145, a1146)?
                                            }, (v4, f299(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1147 = v1;
                                                let a1148 = 1u64;
                                                nat_add(a1147, a1148)?
                                            }, (v4, Adt23::C0(false, false, f383(match v3.clone() {
                                                Adt23::C0(_, _, h1149, _, _) => {
                                                    h1149
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt23::C0(_, _, _, h1150, _) => {
                                                    h1150
                                                }
                                            }, match v3 {
                                                Adt23::C0(_, _, _, _, h1151) => {
                                                    h1151
                                                }
                                            }))))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f299(v0: Adt23) -> Adt23 {
    Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f384(List::cons(match v0.clone() {
        Adt23::C0(_, _, h1156, _, _) => {
            h1156
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h1157, _) => {
            h1157
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt23::C0(_, _, _, _, h1158) => {
            h1158
        }
    }))
}

pub fn f300(p1161: List<List<Bytes>>, p1162: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1160 = (p1161, p1162);
    {
        let mut m1160 = m1160;
        loop {
            let step = {
                let (v0, v1) = m1160;
                let m1159 = v0;
                match m1159.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1160 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f301(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f302(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1163 = v0.clone();
    match m1163.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1164 = v2;
            let a1165 = v1.clone();
            if equal(a1164, a1165) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f303(v0: List<(Bytes, ())>) -> List<Bytes> {
    f208(f385(Fn7::F386, List::<Bytes>::nil(), v0))
}

pub fn f304(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f387(f388(Fn8::F389, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1170 = {
        let (h1166, _) = f390(Fn9::F391, {
            let a1168 = {
                let a1167 = v0;
                length_list(a1167)
            };
            let a1169 = 1u64;
            nat_add(a1168, a1169)?
        }, v1)?;
        h1166
    };
    match m1170.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f305(v0: List<Bytes>) -> List<(Bytes, ())> {
    f392(f393(Fn10::F394, List::<(Bytes, ())>::nil(), v0))
}

pub fn f306(v0: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1171 = f33(v1, 34u8)?;
    match m1171.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1172 = v3;
            match m1172.uncons() {
                None => {
                    f395(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f307(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f396(v0.clone(), 0u64, {
        let a1173 = v0;
        length_bytes(a1173)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f308(v0: Adt13, v1: Bytes) -> Bytes {
    if f28(f397(f79(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f309(v0: Adt1, v1: Bytes) -> R<Adt18> {
    let m1174 = f241(v0, v1)?;
    match m1174 {
        None => {
            Ok(Adt18::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f310(v0: Bytes) -> R<Bytes> {
    let a1175 = f8(Str::lit("Catalog."));
    let a1176 = f329(v0)?;
    Ok(append_bytes(a1175, a1176))
}

pub fn f311(v0: Adt18) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f329(f245(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f312(v0: List<Bytes>) -> List<Bytes> {
    let m1177 = v0;
    match m1177.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f190(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f211(f228(), v1), f312(v2))
            } else {
                f312(v2)
            }
        }
    }
}

pub fn f313(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1178 = v1;
    match m1178.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f398(v0.clone(), v2)?;
            let (h1179, _) = v4.clone();
            if h1179 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f313(v0, v3)?;
                let (h1180, _) = v5.clone();
                if h1180 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1183 = {
                            let (_, h1181) = v4;
                            h1181
                        };
                        let a1184 = {
                            let (_, h1182) = v5;
                            h1182
                        };
                        append_bytes(a1183, a1184)
                    }))
                }
            }
        }
    }
}

pub fn f314(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m1185 = v1;
    match m1185.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f185(v2.clone(), f8(Str::lit("subject"))) {
                if f186(v0.clone(), v2.clone()) {
                    f314(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f47(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f314(v0, v3))
                }
            } else {
                if f320(v0.clone(), v2.clone()) {
                    f314(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f321(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f314(v0, v3))
                }
            }
        }
    }
}

pub fn f315(v0: Adt1) -> R<Bytes> {
    let a1187 = {
        let a1186 = f101(f102(f207(f220(f206(f53(v0.clone(), f179()))?)), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(f82(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1186)
    };
    let a1188 = 0u64;
    if nat_eq(a1187, a1188) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f316(v0: List<Adt13>) -> R<List<Bytes>> {
    let m1189 = v0;
    match m1189.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f95(f122(f163(f79(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f47(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f47(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f399(f81(f47(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f316(v2)?))
        }
    }
}

pub fn f317(v0: Bytes, v1: List<Adt19>) -> List<Bytes> {
    let m1190 = v1;
    match m1190.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(match v2.clone() {
                Adt19::C0(h1191, _, _) => {
                    h1191
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt19::C0(_, h1192, _) => {
                        h1192
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt19::C0(_, _, h1193) => {
                        h1193
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f317(v0, v3))
            } else {
                f317(v0, v3)
            }
        }
    }
}

pub fn f318(v0: Bytes, v1: List<Adt13>) -> List<Bytes> {
    let m1194 = v1;
    match m1194.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(f47(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f318(v0, v3))
            } else {
                if f28(f47(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f47(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f318(v0, v3))
                } else {
                    f318(v0, v3)
                }
            }
        }
    }
}

pub fn f319(p1199: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1198 = p1199;
    {
        let mut m1198 = m1198;
        loop {
            let step = {
                let v0 = m1198;
                let m1195 = v0;
                match m1195.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1196 = v2;
                        match m1196.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f28(v4, v1.clone()) {
                                    let m1197 = v5;
                                    match m1197.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f400(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                } else {
                                    Ok::<List<List<Bytes>>, Bytes>(v3)
                                }
                            }
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

pub fn f320(v0: Adt14, v1: Adt13) -> bool {
    if f41(f47(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1200, _, _) => {
            h1200
        }
    }) {
        f41(f321(v1), match v0 {
            Adt14::C0(_, h1201, _) => {
                h1201
            }
        })
    } else {
        false
    }
}

pub fn f321(v0: Adt13) -> Bytes {
    let m1202 = f79(v0, f8(Str::lit("evidence")));
    match m1202 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f47(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f322(v0: Adt13) -> Bytes {
    if f185(v0.clone(), f8(Str::lit("req"))) {
        let a1203 = f8(Str::lit("req "));
        let a1204 = f47(v0, f8(Str::lit("req")));
        append_bytes(a1203, a1204)
    } else {
        let m1205 = f79(v0.clone(), f8(Str::lit("std")));
        match m1205 {
            None => {
                let m1208 = f79(v0, f8(Str::lit("der")));
                match m1208 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f47(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f95(f122(f163(f79(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f47(v1.clone(), f8(Str::lit("authority"))), List::cons(if f185(v1.clone(), f8(Str::lit("row"))) {
                    let a1206 = f8(Str::lit(" row "));
                    let a1207 = f47(v1, f8(Str::lit("row")));
                    append_bytes(a1206, a1207)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f323(v0: Option<List<Adt13>>) -> bool {
    let m1209 = v0;
    match m1209 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f324(v0: Option<List<Adt13>>) -> List<Adt13> {
    let m1210 = v0;
    match m1210 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f325(v0: Bytes, v1: Bytes) -> bool {
    let a1211 = v0;
    let a1212 = f336();
    if equal(a1211, a1212) {
        let a1216 = {
            let a1214 = {
                let a1213 = v1;
                length_bytes(a1213)
            };
            let a1215 = 0u64;
            nat_eq(a1214, a1215)
        };
        bool_not(a1216)
    } else {
        false
    }
}

pub fn f326(v0: Bytes) -> Bytes {
    f7(List::cons(f124(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f327(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1218 = v0;
    let a1219 = {
        let a1217 = Str::lit("redistributable");
        utf8_encode(a1217)
    };
    if equal(a1218, a1219) {
        f401(v1)
    } else {
        Ok(false)
    }
}

pub fn f328(v0: Adt18) -> bool {
    if f402(v0.clone(), f8(Str::lit("name"))) {
        if f403(v0.clone(), f8(Str::lit("schema"))) {
            let m1220 = f256(f246(f247(f242(v0, f8(Str::lit("citation"))))));
            match m1220 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f329(v0: Bytes) -> R<Bytes> {
    f404(v0.clone(), 0u64, {
        let a1221 = v0;
        length_bytes(a1221)
    }, true, Bytes::lit(&[]))
}

pub fn f330(v0: Adt13) -> Bytes {
    if f185(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f185(v0.clone(), f8(Str::lit("id"))) {
            if f185(v0.clone(), f8(Str::lit("statement"))) {
                if f185(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f349(v0, f8(Str::lit("evidence")))
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            f8(Str::lit("requirement"))
        } else {
            Bytes::lit(&[])
        }
    }
}

pub fn f331(v0: List<Bytes>) -> Adt13 {
    let m1222 = v0;
    match m1222.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(f331(v2)))
        }
    }
}

pub fn f332(v0: Adt1, v1: Bytes, v2: Adt18, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1223 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1223.uncons() {
        None => {
            let m1224 = f256(v4);
            match m1224 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1225 = f8(Str::lit("Catalog."));
                        let a1226 = f329(v3.clone())?;
                        append_bytes(a1225, a1226)
                    };
                    let m1227 = f405(v2, v9, f329(v3.clone())?)?;
                    match m1227 {
                        Ok(v11) => {
                            f258(v0, v3.clone(), v8, v11, f8(Str::lit("Encoding entered through propose-encoding.")), List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f329(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::<(Bytes, Bytes)>::nil()))
                        }
                        Err(v10) => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v10, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                    }
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f333(v0: Adt18) -> R<Bytes> {
    let m1228 = v0;
    match m1228 {
        Adt18::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt18::C1(v2, v3) => {
            f267(v2, v3)
        }
        Adt18::C2(v4, v5, v6) => {
            f406(v4, v5, v6)
        }
        Adt18::C3(v7) => {
            f407(v7)
        }
        Adt18::C4(r1229) => {
            let v8 = (*r1229).clone();
            Ok(f269(f408(v8)?))
        }
        Adt18::C5(r1230) => {
            let v9 = (*r1230).clone();
            f271(f409(v9)?)
        }
        Adt18::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f334(v0: Option<Adt18>) -> Option<Bytes> {
    let m1231 = v0;
    match m1231 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1232 = v1;
            match m1232 {
                Adt18::C0(_) => {
                    None::<Bytes>
                }
                Adt18::C1(_, _) => {
                    None::<Bytes>
                }
                Adt18::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt18::C3(v2) => {
                    Some(v2)
                }
                Adt18::C4(_) => {
                    None::<Bytes>
                }
                Adt18::C5(_) => {
                    None::<Bytes>
                }
                Adt18::C6 => {
                    None::<Bytes>
                }
                Adt18::C7(_, _) => {
                    None::<Bytes>
                }
                Adt18::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f335(v0: Adt18) -> List<Adt18> {
    let m1233 = v0;
    match m1233 {
        Adt18::C0(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C1(_, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C2(_, _, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C3(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C4(r1234) => {
            let v3 = (*r1234).clone();
            f335(v3)
        }
        Adt18::C5(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C6 => {
            List::<Adt18>::nil()
        }
        Adt18::C7(r1235, r1236) => {
            let v1 = (*r1235).clone();
            let v2 = (*r1236).clone();
            List::cons(v1, f335(v2))
        }
        Adt18::C8(_, _, _) => {
            List::<Adt18>::nil()
        }
    }
}

pub fn f336() -> Bytes {
    let a1237 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1237)
}

pub fn f337(v0: Adt13, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt13> = f79(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f253(Str::lit("id"), f47(v0.clone(), f8(Str::lit("id"))))?, List::cons(f253(Str::lit("name"), f47(v0.clone(), f8(Str::lit("name"))))?, List::cons(f253(Str::lit("issuer"), f47(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f253(Str::lit("edition"), f47(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f253(Str::lit("canonical_id"), f47(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f253(Str::lit("source_role"), f47(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f253(Str::lit("retrieval_date"), f47(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f253(Str::lit("url"), f397(v2.clone(), f8(Str::lit("url"))))?, List::cons(f253(Str::lit("media_type"), f397(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f253(Str::lit("license"), f397(v2.clone(), f8(Str::lit("license"))))?, List::cons(f253(Str::lit("redistribution"), f397(v2, f8(Str::lit("redistribution"))))?, List::cons(f253(Str::lit("statement"), f47(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1238 = v1;
        match m1238 {
            None => {
                f7(List::cons(f253(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f253(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f253(Str::lit("artifact"), {
                    let (h1239, _) = v3;
                    h1239
                })?, List::cons(f253(Str::lit("checksum"), {
                    let (_, h1240) = v3;
                    h1240
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f410(f122(f163(f79(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f338(v0: Adt27) -> Bytes {
    let a1261 = f411(match v0.clone() {
        Adt27::C0(h1241, _, _, _, _, _, _, _) => {
            h1241
        }
    });
    let a1262 = {
        let a1259 = f411(match v0.clone() {
            Adt27::C0(_, h1242, _, _, _, _, _, _) => {
                h1242
            }
        });
        let a1260 = {
            let a1257 = f411(match v0.clone() {
                Adt27::C0(_, _, h1243, _, _, _, _, _) => {
                    h1243
                }
            });
            let a1258 = {
                let a1255 = f411(match v0.clone() {
                    Adt27::C0(_, _, _, h1244, _, _, _, _) => {
                        h1244
                    }
                });
                let a1256 = {
                    let a1253 = f411(match v0.clone() {
                        Adt27::C0(_, _, _, _, h1245, _, _, _) => {
                            h1245
                        }
                    });
                    let a1254 = {
                        let a1251 = f411(match v0.clone() {
                            Adt27::C0(_, _, _, _, _, h1246, _, _) => {
                                h1246
                            }
                        });
                        let a1252 = {
                            let a1249 = f411(match v0.clone() {
                                Adt27::C0(_, _, _, _, _, _, h1247, _) => {
                                    h1247
                                }
                            });
                            let a1250 = f411(match v0 {
                                Adt27::C0(_, _, _, _, _, _, _, h1248) => {
                                    h1248
                                }
                            });
                            append_bytes(a1249, a1250)
                        };
                        append_bytes(a1251, a1252)
                    };
                    append_bytes(a1253, a1254)
                };
                append_bytes(a1255, a1256)
            };
            append_bytes(a1257, a1258)
        };
        append_bytes(a1259, a1260)
    };
    append_bytes(a1261, a1262)
}

pub fn f339(v0: Bytes) -> R<Adt27> {
    let v1: u64 = {
        let a1263 = v0.clone();
        length_bytes(a1263)
    };
    let v2: u64 = {
        let a1269 = {
            let a1266 = {
                let a1264 = v1;
                let a1265 = 72u64;
                nat_add(a1264, a1265)?
            };
            let a1267 = 64u64;
            let a1268 = 0u64;
            nat_quot(a1266, a1267, a1268)
        };
        let a1270 = 64u64;
        nat_mul(a1269, a1270)?
    };
    let (_, h1271) = f412(Fn11::F413(v0, v1, v2), {
        let a1272 = v2;
        let a1273 = 64u64;
        let a1274 = 0u64;
        nat_quot(a1272, a1273, a1274)
    }, (0u64, f414()))?;
    Ok(h1271)
}

pub fn f340(p1277: List<List<Bytes>>, v1: Bytes, p1278: List<Bytes>) -> R<List<Bytes>> {
    let m1276 = (p1277, p1278);
    {
        let mut m1276 = m1276;
        loop {
            let step = {
                let (v0, v2) = m1276;
                let m1275 = v0;
                match m1275.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f415(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f342(f416(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: Bytes) -> R<Bytes> {
    Ok(f35(v0.clone(), 0u64, f276(v0)?))
}

pub fn f342(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1279 = v0;
    match m1279.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1280 = v3.clone();
            match m1280.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1283 = v2;
                    let a1284 = {
                        let a1281 = v1.clone();
                        let a1282 = f342(v3, v1);
                        append_bytes(a1281, a1282)
                    };
                    append_bytes(a1283, a1284)
                }
            }
        }
    }
}

pub fn f343(v0: Adt13, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1285 = f79(v0, v1);
    match m1285 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v3) => {
            f163(f79(v3, v2))
        }
    }
}

pub fn f344(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f417(f81(v0)?, None::<Bytes>)
}

pub fn f345(p1306: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p1307: List<(Bytes, (Bytes, Bytes))>, p1308: List<Adt25>, p1309: List<Bytes>) -> R<Adt24> {
    let m1305 = (p1306, (p1307, (p1308, p1309)));
    {
        let mut m1305 = m1305;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m1305;
                let m1286 = v0;
                if m1286 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>(Ok(Adt24::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m1286 - 1;
                    let m1287 = v3;
                    match m1287.uncons() {
                        None => {
                            let m1288 = v5.clone();
                            match m1288.uncons() {
                                None => {
                                    let m1289 = f418(f419(v4, List::<Adt25>::nil()))?;
                                    match m1289 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>(Ok(Adt24::C2({
                                                let (h1290, _) = v8;
                                                h1290
                                            }, {
                                                let (_, h1291) = v8;
                                                h1291
                                            })))
                                        }
                                        Err(v7) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>(Ok(Adt24::C1(v7)))
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>(Ok(Adt24::C0(v5)))
                                }
                            }
                        }
                        Some((v11, v12)) => {
                            let v13: Bytes = {
                                let (h1292, _) = v11.clone();
                                h1292
                            };
                            if if f41(v13.clone(), f420(v4.clone())) {
                                true
                            } else {
                                f41(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>((v6, (v12, (v4, v5))))
                            } else {
                                let m1293 = f421(v1.clone(), v13.clone());
                                match m1293 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m1294 = v14;
                                        match m1294 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>(Ok(Adt24::C1(f422(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m1295 = f423(v13.clone(), v15, f424(v2.clone(), v13))?;
                                                match m1295 {
                                                    Ok(v17) => {
                                                        let m1300 = f425(v17.clone(), {
                                                            let (h1296, _) = {
                                                                let (_, h1297) = v11.clone();
                                                                h1297
                                                            };
                                                            h1296
                                                        }, {
                                                            let (_, h1298) = {
                                                                let (_, h1299) = v11;
                                                                h1299
                                                            };
                                                            h1298
                                                        });
                                                        match m1300 {
                                                            None => {
                                                                let m1303 = f426(match v17.clone() {
                                                                    Adt25::C0(_, _, _, _, h1301, _) => {
                                                                        h1301
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt25::C0(_, _, _, h1302, _, _) => {
                                                                        h1302
                                                                    }
                                                                })?;
                                                                match m1303 {
                                                                    Ok(v20) => {
                                                                        let m1304 = f427(v20, v17.clone(), v2.clone())?;
                                                                        match m1304 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>((v6, (f428(v12, v22), (List::cons(v17, v4), v5))))
                                                                            }
                                                                            Err(v21) => {
                                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>(Ok(Adt24::C1(v21)))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v19) => {
                                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>(Ok(Adt24::C1(v19)))
                                                                    }
                                                                }
                                                            }
                                                            Some(v18) => {
                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>(Ok(Adt24::C1(v18)))
                                                            }
                                                        }
                                                    }
                                                    Err(v16) => {
                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt25>, List<Bytes>))), R<Adt24>>(Ok(Adt24::C1(v16)))
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
                Ok(next) => { m1305 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1310 = v0;
    match m1310.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f346(v3, v1))
        }
    }
}

pub fn f347(v0: List<Adt13>, v1: Bytes, v2: Bytes, v3: Adt13) -> List<Adt13> {
    let m1311 = v0;
    match m1311.uncons() {
        None => {
            List::cons(v3, List::<Adt13>::nil())
        }
        Some((v4, v5)) => {
            if f28(f47(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f429(v5, v1, v2))
            } else {
                List::cons(v4, f347(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f348(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f342(f430(f81(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f349(v0: Adt13, v1: Bytes) -> bool {
    let m1312 = f79(v0, v1);
    match m1312 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f350(v0: Bytes, p1348: u64, p1349: u64, p1350: bool, p1351: bool, p1352: u64, p1353: Bytes) -> R<Bytes> {
    let m1347 = (p1348, (p1349, (p1350, (p1351, (p1352, p1353)))));
    {
        let mut m1347 = m1347;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m1347;
                let m1313 = v2;
                if m1313 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f431(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m1313 - 1;
                    let m1316 = {
                        let a1314 = v0.clone();
                        let a1315 = v1;
                        index_bytes(a1314, a1315)
                    };
                    match m1316 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f431(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f62(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a1317 = v1;
                                    let a1318 = 1u64;
                                    nat_add(a1317, a1318)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a1319 = v9;
                                    let a1320 = 34u64;
                                    !nat_eq(a1319, a1320)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1321 = v9;
                                    let a1322 = 92u64;
                                    nat_eq(a1321, a1322)
                                }, (v5, v6))))))
                            } else {
                                let a1323 = v9;
                                let a1324 = 32u64;
                                if if if if nat_eq(a1323, a1324) {
                                    true
                                } else {
                                    let a1325 = v9;
                                    let a1326 = 9u64;
                                    nat_eq(a1325, a1326)
                                } {
                                    true
                                } else {
                                    let a1327 = v9;
                                    let a1328 = 10u64;
                                    nat_eq(a1327, a1328)
                                } {
                                    true
                                } else {
                                    let a1329 = v9;
                                    let a1330 = 13u64;
                                    nat_eq(a1329, a1330)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1331 = v1;
                                        let a1332 = 1u64;
                                        nat_add(a1331, a1332)?
                                    }, (v7, (false, (false, ({
                                        let a1333 = v1;
                                        let a1334 = 1u64;
                                        nat_add(a1333, a1334)?
                                    }, f431(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    let a1335 = v9;
                                    let a1336 = 110u64;
                                    if if nat_eq(a1335, a1336) {
                                        f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("null")))
                                    } else {
                                        false
                                    } {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a1337 = v1;
                                            let a1338 = 4u64;
                                            nat_add(a1337, a1338)?
                                        }, (v7, (false, (false, ({
                                            let a1339 = v1;
                                            let a1340 = 4u64;
                                            nat_add(a1339, a1340)?
                                        }, {
                                            let a1341 = f431(v0.clone(), v5, v1, v6);
                                            let a1342 = f8(Str::lit("[]"));
                                            append_bytes(a1341, a1342)
                                        }))))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a1343 = v1;
                                            let a1344 = 1u64;
                                            nat_add(a1343, a1344)?
                                        }, (v7, ({
                                            let a1345 = v9;
                                            let a1346 = 34u64;
                                            nat_eq(a1345, a1346)
                                        }, (false, (v5, v6))))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: Bytes, v1: u64) -> R<u64> {
    let m1356 = {
        let a1354 = v0;
        let a1355 = v1;
        index_bytes(a1354, a1355)
    };
    match m1356 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1401 = {
                let a1377 = {
                    let a1365 = {
                        let a1359 = {
                            let a1357 = v3;
                            let a1358 = 128u8;
                            fixed_u8::bit_and(a1357, a1358)
                        };
                        let a1360 = 128u8;
                        if equal(a1359, a1360) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1366 = {
                        let a1363 = {
                            let a1361 = v3;
                            let a1362 = 64u8;
                            fixed_u8::bit_and(a1361, a1362)
                        };
                        let a1364 = 64u8;
                        if equal(a1363, a1364) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1365, a1366)?
                };
                let a1378 = {
                    let a1375 = {
                        let a1369 = {
                            let a1367 = v3;
                            let a1368 = 32u8;
                            fixed_u8::bit_and(a1367, a1368)
                        };
                        let a1370 = 32u8;
                        if equal(a1369, a1370) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1376 = {
                        let a1373 = {
                            let a1371 = v3;
                            let a1372 = 16u8;
                            fixed_u8::bit_and(a1371, a1372)
                        };
                        let a1374 = 16u8;
                        if equal(a1373, a1374) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1375, a1376)?
                };
                nat_add(a1377, a1378)?
            };
            let a1402 = {
                let a1399 = {
                    let a1387 = {
                        let a1381 = {
                            let a1379 = v3;
                            let a1380 = 8u8;
                            fixed_u8::bit_and(a1379, a1380)
                        };
                        let a1382 = 8u8;
                        if equal(a1381, a1382) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1388 = {
                        let a1385 = {
                            let a1383 = v3;
                            let a1384 = 4u8;
                            fixed_u8::bit_and(a1383, a1384)
                        };
                        let a1386 = 4u8;
                        if equal(a1385, a1386) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1387, a1388)?
                };
                let a1400 = {
                    let a1397 = {
                        let a1391 = {
                            let a1389 = v3;
                            let a1390 = 2u8;
                            fixed_u8::bit_and(a1389, a1390)
                        };
                        let a1392 = 2u8;
                        if equal(a1391, a1392) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1398 = {
                        let a1395 = {
                            let a1393 = v3;
                            let a1394 = 1u8;
                            fixed_u8::bit_and(a1393, a1394)
                        };
                        let a1396 = 1u8;
                        if equal(a1395, a1396) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1397, a1398)?
                };
                nat_add(a1399, a1400)?
            };
            nat_add(a1401, a1402)
        }
    }
}

pub fn f352(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f432(v0.clone(), v1, {
        let a1404 = {
            let a1403 = v0.clone();
            length_bytes(a1403)
        };
        let a1405 = 1u64;
        nat_add(a1404, a1405)?
    })?;
    let a1406 = f351(v0.clone(), v2)?;
    let a1407 = 34u64;
    if nat_eq(a1406, a1407) {
        Ok(Some(({
            let a1408 = v2;
            let a1409 = 1u64;
            nat_add(a1408, a1409)?
        }, f35(v0, v1, {
            let a1410 = v2;
            let a1411 = v1;
            nat_sub(a1410, a1411)
        }))))
    } else {
        f433(v0, v1)
    }
}

pub fn f353(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f354(v0: Bytes, v1: u64) -> R<Option<(u64, Adt18)>> {
    let v2: bool = {
        let a1412 = f351(v0.clone(), v1)?;
        let a1413 = 45u64;
        nat_eq(a1412, a1413)
    };
    let v3: u64 = if v2 {
        let a1414 = v1;
        let a1415 = 1u64;
        nat_add(a1414, a1415)?
    } else {
        v1
    };
    let v4: u64 = f434(v0.clone(), v3)?;
    let v5: u64 = {
        let a1416 = v4;
        let a1417 = v3;
        nat_sub(a1416, a1417)
    };
    let v6: bool = {
        let a1418 = f351(v0.clone(), v4)?;
        let a1419 = 46u64;
        nat_eq(a1418, a1419)
    };
    let v7: u64 = if v6 {
        let a1420 = v4;
        let a1421 = 1u64;
        nat_add(a1420, a1421)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f434(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f351(v0.clone(), v8)?;
    let v10: bool = {
        let a1422 = v9;
        let a1423 = 101u64;
        if nat_eq(a1422, a1423) {
            true
        } else {
            let a1424 = v9;
            let a1425 = 69u64;
            nat_eq(a1424, a1425)
        }
    };
    let v11: u64 = f351(v0.clone(), {
        let a1426 = v8;
        let a1427 = 1u64;
        nat_add(a1426, a1427)?
    })?;
    let v12: bool = f353(v10, f435({
        let a1428 = v11;
        let a1429 = 43u64;
        nat_eq(a1428, a1429)
    }, {
        let a1430 = v11;
        let a1431 = 45u64;
        nat_eq(a1430, a1431)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a1432 = v8;
            let a1433 = 2u64;
            nat_add(a1432, a1433)?
        } else {
            let a1434 = v8;
            let a1435 = 1u64;
            nat_add(a1434, a1435)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f434(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f35(v0.clone(), v13, {
        let a1436 = v14;
        let a1437 = v13;
        nat_sub(a1436, a1437)
    });
    let v16: u64 = f436(v15.clone())?;
    let v17: Bytes = f35(v15.clone(), v16, {
        let a1439 = {
            let a1438 = v15;
            length_bytes(a1438)
        };
        let a1440 = v16;
        nat_sub(a1439, a1440)
    });
    let a1441 = v5;
    let a1442 = 0u64;
    if if nat_eq(a1441, a1442) {
        true
    } else {
        if f353({
            let a1443 = 1u64;
            let a1444 = v5;
            nat_lt(a1443, a1444)
        }, {
            let a1445 = f351(v0.clone(), v3)?;
            let a1446 = 48u64;
            nat_eq(a1445, a1446)
        }) {
            true
        } else {
            if f353(v6, {
                let a1447 = v8;
                let a1448 = v7;
                nat_eq(a1447, a1448)
            }) {
                true
            } else {
                f435(if v10 {
                    let a1449 = v14;
                    let a1450 = v13;
                    nat_eq(a1449, a1450)
                } else {
                    false
                }, {
                    let a1452 = 18u64;
                    let a1453 = {
                        let a1451 = v17.clone();
                        length_bytes(a1451)
                    };
                    nat_lt(a1452, a1453)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt18)>)
    } else {
        let m1457 = {
            let a1455 = {
                let a1454 = v17.clone();
                length_bytes(a1454)
            };
            let a1456 = 0u64;
            if nat_eq(a1455, a1456) {
                Some(0u64)
            } else {
                f437(v17)?
            }
        };
        match m1457 {
            None => {
                Ok(None::<(u64, Adt18)>)
            }
            Some(v18) => {
                let m1462 = f438(f35(v0.clone(), v3, v5), f35(v0, v7, {
                    let a1458 = v8;
                    let a1459 = v7;
                    nat_sub(a1458, a1459)
                }), f353(v12, {
                    let a1460 = v11;
                    let a1461 = 45u64;
                    nat_eq(a1460, a1461)
                }), v18)?;
                match m1462 {
                    None => {
                        Ok(None::<(u64, Adt18)>)
                    }
                    Some(v19) => {
                        let a1465 = 19u64;
                        let a1466 = {
                            let a1464 = {
                                let (h1463, _) = v19.clone();
                                h1463
                            };
                            length_bytes(a1464)
                        };
                        if nat_lt(a1465, a1466) {
                            Ok(None::<(u64, Adt18)>)
                        } else {
                            let m1468 = f437({
                                let (h1467, _) = v19.clone();
                                h1467
                            })?;
                            match m1468 {
                                None => {
                                    Ok(None::<(u64, Adt18)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f353(v2, {
                                        let a1469 = 0u64;
                                        let a1470 = v20;
                                        nat_lt(a1469, a1470)
                                    });
                                    if f439(v21, v20) {
                                        Ok(Some((v14, {
                                            let a1472 = {
                                                let (_, h1471) = v19.clone();
                                                h1471
                                            };
                                            let a1473 = 0u64;
                                            if nat_eq(a1472, a1473) {
                                                Adt18::C1(v21, v20)
                                            } else {
                                                Adt18::C2(v21, v20, {
                                                    let (_, h1474) = v19;
                                                    h1474
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt18)>)
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

pub fn f355(v0: Bytes) -> Bytes {
    if f190(v0.clone(), Bytes::lit(&[0x0d])) {
        f35(v0.clone(), 0u64, {
            let a1476 = {
                let a1475 = v0;
                length_bytes(a1475)
            };
            let a1477 = 1u64;
            nat_sub(a1476, a1477)
        })
    } else {
        v0
    }
}

pub fn f356(v0: Adt13, v1: Bytes) -> Adt13 {
    let m1478 = v0;
    match m1478 {
        Adt13::C0(v3) => {
            Adt13::C0(v3)
        }
        Adt13::C1(v4, v5) => {
            Adt13::C1(v4, v5)
        }
        Adt13::C2(v2) => {
            Adt13::C2(f440(f49(), v2, v1))
        }
        Adt13::C3(r1479) => {
            let v6 = (*r1479).clone();
            Adt13::C3(std::rc::Rc::new(v6))
        }
        Adt13::C4(r1480) => {
            let v7 = (*r1480).clone();
            Adt13::C4(std::rc::Rc::new(v7))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r1481, r1482) => {
            let v8 = (*r1481).clone();
            let v9 = (*r1482).clone();
            Adt13::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt13::C7(v10, r1483, r1484) => {
            let v11 = (*r1483).clone();
            let v12 = (*r1484).clone();
            Adt13::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f357(v0: u64) -> R<Bytes> {
    f441(f442(), 10u64, v0)
}

pub fn f358(v0: Fn4, p1489: u64, p1490: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1488 = (p1489, p1490);
    {
        let mut m1488 = m1488;
        loop {
            let step = {
                let (v1, v2) = m1488;
                let m1485 = v1;
                if m1485 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1485 - 1;
                    let m1487 = {
                        let c1486: Fn4 = v0.clone();
                        c1486.apply(v2.clone())?
                    };
                    match m1487 {
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
                Ok(next) => { m1488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f443(v0, v1)
}

pub fn f360(v0: Fn5, p1494: List<(Bytes, Bytes)>, p1495: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1493 = (p1494, p1495);
    {
        let mut m1493 = m1493;
        loop {
            let step = {
                let (v1, v2) = m1493;
                let m1491 = v2;
                match m1491.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1492: Fn5 = v0.clone();
                            c1492.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f444(v0, v1)
}

pub fn f362(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f445(v0)
}

pub fn f363(v0: Fn6, p1499: List<Bytes>, p1500: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1498 = (p1499, p1500);
    {
        let mut m1498 = m1498;
        loop {
            let step = {
                let (v1, v2) = m1498;
                let m1496 = v2;
                match m1496.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1497: Fn6 = v0.clone();
                            c1497.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f446(v1)?, v0))
}

pub fn f365(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f268(v0)?, List::cons(f8(Str::lit(":")), List::cons(f268(v1)?, List::<Bytes>::nil())))))
}

pub fn f366(v0: Adt12) -> Bytes {
    let m1501 = v0;
    match m1501 {
        Adt12::C0 => {
            f8(Str::lit("Verdant Engine authority register"))
        }
        Adt12::C1 => {
            f8(Str::lit("Verdant Engine conformance ID register"))
        }
        Adt12::C2 => {
            f8(Str::lit("Verdant Engine release ledger"))
        }
        Adt12::C3 => {
            f8(Str::lit("Verdant Engine standards lock"))
        }
    }
}

pub fn f367(v0: Adt12) -> Bytes {
    let m1502 = v0;
    match m1502 {
        Adt12::C0 => {
            f8(Str::lit("The engine's imported authorities, each catalog pinned by sha256 and mapped to the IDs that realize it (realized_by)."))
        }
        Adt12::C1 => {
            f8(Str::lit("The engine's conformance IDs, each with level, suite, refusal and statement. Every engine capability Verdant Studio uses is cited by one of these IDs."))
        }
        Adt12::C2 => {
            f8(Str::lit("The engine's verdant/release-ledger/1: its current SemVer version, release epoch and change history."))
        }
        Adt12::C3 => {
            f8(Str::lit("The engine's prismpm/standards-lock/1: the standards it binds and the executable oracles, corpora and failure mappings it runs against them."))
        }
    }
}

pub fn f368(v0: Adt12) -> Bytes {
    let m1503 = v0;
    match m1503 {
        Adt12::C0 => {
            f8(Str::lit("application/toml"))
        }
        Adt12::C1 => {
            f8(Str::lit("application/toml"))
        }
        Adt12::C2 => {
            f8(Str::lit("application/json"))
        }
        Adt12::C3 => {
            f8(Str::lit("application/json"))
        }
    }
}

pub fn f369(v0: Bytes, v1: u64) -> Option<u64> {
    let a1504 = 0u64;
    let a1505 = v1;
    if nat_lt(a1504, a1505) {
        if f447(v0, {
            let a1506 = v1;
            let a1507 = 1u64;
            nat_sub(a1506, a1507)
        }) {
            Some({
                let a1508 = v1;
                let a1509 = 1u64;
                nat_sub(a1508, a1509)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f370(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1510 = v2;
    let a1511 = v0;
    if nat_lt(a1510, a1511) {
        if f447(v1, v2) {
            Ok(Some({
                let a1512 = v2;
                let a1513 = 1u64;
                nat_add(a1512, a1513)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f371(v0: (Bytes, Bytes), p1516: List<(Bytes, Bytes)>) -> bool {
    let m1515 = p1516;
    {
        let mut m1515 = m1515;
        loop {
            let step = {
                let v1 = m1515;
                let m1514 = v1;
                match m1514.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f448(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1517 = v1;
    match m1517.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h1518, _) = v2;
                h1518
            }, v0.clone()) {
                List::cons({
                    let (_, h1519) = v2;
                    h1519
                }, f372(v0, v3))
            } else {
                f372(v0, v3)
            }
        }
    }
}

pub fn f373() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f374(v0: Adt13, v1: Bytes, v2: Bytes) -> bool {
    let m1520 = f79(v0, v1);
    match m1520 {
        None => {
            false
        }
        Some(v3) => {
            f185(v3, v2)
        }
    }
}

pub fn f375() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f376(v0: Bytes) -> R<bool> {
    let m1521 = f194(v0.clone(), f8(Str::lit("&&")))?;
    if match m1521 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1522 = f194(v0, f8(Str::lit("||")))?;
        match m1522 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f377(v0: Adt17, v1: Option<u64>) -> bool {
    let m1523 = v1;
    match m1523 {
        None => {
            true
        }
        Some(v2) => {
            f297(v2, match v0 {
                Adt17::C0(_, h1524, _, _) => {
                    h1524
                }
            })
        }
    }
}

pub fn f378(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a1526 = match v0 {
            Adt15::C0(_, _, h1525, _, _) => {
                h1525
            }
        };
        length_list(a1526)
    })
}

pub fn f379(v0: Bytes) -> Option<Adt22> {
    if f28(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt22::C0)
    } else {
        if f28(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt22::C1)
        } else {
            if f28(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt22::C2)
            } else {
                None::<Adt22>
            }
        }
    }
}

pub fn f380(v0: Bytes, p1531: List<Bytes>, p1532: u64) -> R<Option<u64>> {
    let m1530 = (p1531, p1532);
    {
        let mut m1530 = m1530;
        loop {
            let step = {
                let (v1, v2) = m1530;
                let m1527 = v1;
                match m1527.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f28(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1528 = v2;
                                let a1529 = 1u64;
                                nat_add(a1528, a1529)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381(v0: Option<Adt13>) -> Option<u64> {
    let m1533 = v0;
    match m1533 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1534 = v1;
            match m1534 {
                Adt13::C0(_) => {
                    None::<u64>
                }
                Adt13::C1(v2, v3) => {
                    if v2 {
                        None::<u64>
                    } else {
                        Some(v3)
                    }
                }
                Adt13::C2(_) => {
                    None::<u64>
                }
                Adt13::C3(_) => {
                    None::<u64>
                }
                Adt13::C4(_) => {
                    None::<u64>
                }
                Adt13::C5 => {
                    None::<u64>
                }
                Adt13::C6(_, _) => {
                    None::<u64>
                }
                Adt13::C7(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f382(v0: Bytes, v1: u64) -> R<u64> {
    let m1537 = {
        let a1535 = v0;
        let a1536 = v1;
        index_bytes(a1535, a1536)
    };
    match m1537 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f62(v2)
        }
    }
}

pub fn f383(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1541 = {
        let a1538 = v1;
        let a1539 = v2;
        let a1540 = 1u64;
        slice_bytes(a1538, a1539, a1540)
    };
    match m1541 {
        None => {
            v0
        }
        Some(v3) => {
            let a1542 = v0;
            let a1543 = v3;
            append_bytes(a1542, a1543)
        }
    }
}

pub fn f384(p1546: List<Bytes>, p1547: List<Bytes>) -> List<Bytes> {
    let m1545 = (p1546, p1547);
    {
        let mut m1545 = m1545;
        loop {
            let step = {
                let (v0, v1) = m1545;
                let m1544 = v0;
                match m1544.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385(v0: Fn7, p1551: List<Bytes>, p1552: List<(Bytes, ())>) -> List<Bytes> {
    let m1550 = (p1551, p1552);
    {
        let mut m1550 = m1550;
        loop {
            let step = {
                let (v1, v2) = m1550;
                let m1548 = v2;
                match m1548.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1549: Fn7 = v0.clone();
                            c1549.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1553, _) = v1;
        h1553
    }, v0)
}

pub fn f387(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f449(Fn12::F450, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f388(v0: Fn8, p1557: List<List<(Bytes, ())>>, p1558: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1556 = (p1557, p1558);
    {
        let mut m1556 = m1556;
        loop {
            let step = {
                let (v1, v2) = m1556;
                let m1554 = v2;
                match m1554.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1555: Fn8 = v0.clone();
                            c1555.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f389(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f390(v0: Fn9, p1563: u64, p1564: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1562 = (p1563, p1564);
    {
        let mut m1562 = m1562;
        loop {
            let step = {
                let (v1, v2) = m1562;
                let m1559 = v1;
                if m1559 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1559 - 1;
                    let m1561 = {
                        let c1560: Fn9 = v0.clone();
                        c1560.apply(v2.clone())?
                    };
                    match m1561 {
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
                Ok(next) => { m1562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f451(v0)
}

pub fn f392(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f452(Fn13::F453, List::<(Bytes, ())>::nil(), v0)
}

pub fn f393(v0: Fn10, p1568: List<(Bytes, ())>, p1569: List<Bytes>) -> List<(Bytes, ())> {
    let m1567 = (p1568, p1569);
    {
        let mut m1567 = m1567;
        loop {
            let step = {
                let (v1, v2) = m1567;
                let m1565 = v2;
                match m1565.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1566: Fn10 = v0.clone();
                            c1566.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f395(p1572: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1571 = p1572;
    {
        let mut m1571 = m1571;
        loop {
            let step = {
                let v0 = m1571;
                let m1570 = v0;
                match m1570.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f454(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(v0: Bytes, p1598: u64, p1599: u64, p1600: Bytes) -> R<Bytes> {
    let m1597 = (p1598, (p1599, p1600));
    {
        let mut m1597 = m1597;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1597;
                let m1573 = v2;
                if m1573 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1573 - 1;
                    let m1576 = {
                        let a1574 = v0.clone();
                        let a1575 = v1;
                        index_bytes(a1574, a1575)
                    };
                    match m1576 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            if f435({
                                let a1577 = v6;
                                let a1578 = 34u64;
                                nat_eq(a1577, a1578)
                            }, {
                                let a1579 = v6;
                                let a1580 = 92u64;
                                nat_eq(a1579, a1580)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1581 = v1;
                                    let a1582 = 1u64;
                                    nat_add(a1581, a1582)?
                                }, (v4, {
                                    let a1585 = v3;
                                    let a1586 = {
                                        let a1583 = f8(Str::lit("\\"));
                                        let a1584 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1583, a1584)
                                    };
                                    append_bytes(a1585, a1586)
                                })))
                            } else {
                                let a1587 = v6;
                                let a1588 = 10u64;
                                if nat_eq(a1587, a1588) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1589 = v1;
                                        let a1590 = 1u64;
                                        nat_add(a1589, a1590)?
                                    }, (v4, {
                                        let a1591 = v3;
                                        let a1592 = f8(Str::lit("\\n"));
                                        append_bytes(a1591, a1592)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1593 = v1;
                                        let a1594 = 1u64;
                                        nat_add(a1593, a1594)?
                                    }, (v4, {
                                        let a1595 = v3;
                                        let a1596 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1595, a1596)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: Option<Adt13>, v1: Bytes) -> Bytes {
    let m1601 = v0;
    match m1601 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f47(v2, v1)
        }
    }
}

pub fn f398(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
    let m1602 = f260(v2.clone())?;
    match m1602 {
        None => {
            f456(v0, v1, v2)
        }
        Some(v3) => {
            f455(v0, v1, v3)
        }
    }
}

pub fn f399(v0: List<Bytes>) -> List<Bytes> {
    let m1603 = v0;
    match m1603.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1605 = {
                    let a1604 = v1.clone();
                    length_bytes(a1604)
                };
                let a1606 = 0u64;
                if nat_eq(a1605, a1606) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f399(v2))
        }
    }
}

pub fn f400(v0: List<Bytes>) -> Bytes {
    let m1607 = v0;
    match m1607.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f401(v0: Bytes) -> R<bool> {
    let m1609 = {
        let a1608 = v0.clone();
        length_bytes(a1608)
    };
    if m1609 == 0 {
        Ok(true)
    } else {
        let v1 = m1609 - 1;
        let m1612 = {
            let a1610 = v0;
            let a1611 = v1;
            index_bytes(a1610, a1611)
        };
        match m1612 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f457({
                    let v3: u8 = v2;
                    let a1657 = {
                        let a1633 = {
                            let a1621 = {
                                let a1615 = {
                                    let a1613 = v3;
                                    let a1614 = 128u8;
                                    fixed_u8::bit_and(a1613, a1614)
                                };
                                let a1616 = 128u8;
                                if equal(a1615, a1616) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1622 = {
                                let a1619 = {
                                    let a1617 = v3;
                                    let a1618 = 64u8;
                                    fixed_u8::bit_and(a1617, a1618)
                                };
                                let a1620 = 64u8;
                                if equal(a1619, a1620) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1621, a1622)?
                        };
                        let a1634 = {
                            let a1631 = {
                                let a1625 = {
                                    let a1623 = v3;
                                    let a1624 = 32u8;
                                    fixed_u8::bit_and(a1623, a1624)
                                };
                                let a1626 = 32u8;
                                if equal(a1625, a1626) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1632 = {
                                let a1629 = {
                                    let a1627 = v3;
                                    let a1628 = 16u8;
                                    fixed_u8::bit_and(a1627, a1628)
                                };
                                let a1630 = 16u8;
                                if equal(a1629, a1630) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1631, a1632)?
                        };
                        nat_add(a1633, a1634)?
                    };
                    let a1658 = {
                        let a1655 = {
                            let a1643 = {
                                let a1637 = {
                                    let a1635 = v3;
                                    let a1636 = 8u8;
                                    fixed_u8::bit_and(a1635, a1636)
                                };
                                let a1638 = 8u8;
                                if equal(a1637, a1638) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1644 = {
                                let a1641 = {
                                    let a1639 = v3;
                                    let a1640 = 4u8;
                                    fixed_u8::bit_and(a1639, a1640)
                                };
                                let a1642 = 4u8;
                                if equal(a1641, a1642) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1643, a1644)?
                        };
                        let a1656 = {
                            let a1653 = {
                                let a1647 = {
                                    let a1645 = v3;
                                    let a1646 = 2u8;
                                    fixed_u8::bit_and(a1645, a1646)
                                };
                                let a1648 = 2u8;
                                if equal(a1647, a1648) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1654 = {
                                let a1651 = {
                                    let a1649 = v3;
                                    let a1650 = 1u8;
                                    fixed_u8::bit_and(a1649, a1650)
                                };
                                let a1652 = 1u8;
                                if equal(a1651, a1652) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1653, a1654)?
                        };
                        nat_add(a1655, a1656)?
                    };
                    nat_add(a1657, a1658)?
                }))
            }
        }
    }
}

pub fn f402(v0: Adt18, v1: Bytes) -> bool {
    let a1660 = {
        let a1659 = f245(v0, v1);
        length_bytes(a1659)
    };
    let a1661 = 0u64;
    !nat_eq(a1660, a1661)
}

pub fn f403(v0: Adt18, v1: Bytes) -> bool {
    let m1662 = f242(v0, v1);
    match m1662 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f404(v0: Bytes, p1683: u64, p1684: u64, p1685: bool, p1686: Bytes) -> R<Bytes> {
    let m1682 = (p1683, (p1684, (p1685, p1686)));
    {
        let mut m1682 = m1682;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1682;
                let m1663 = v2;
                if m1663 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1663 - 1;
                    let m1666 = {
                        let a1664 = v0.clone();
                        let a1665 = v1;
                        index_bytes(a1664, a1665)
                    };
                    match m1666 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f62(v6)?;
                            if f458(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1667 = v1;
                                    let a1668 = 1u64;
                                    nat_add(a1667, a1668)?
                                }, (v5, (true, v4))))
                            } else {
                                if f353(f459(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1669 = v1;
                                        let a1670 = 1u64;
                                        nat_add(a1669, a1670)?
                                    }, (v5, (false, {
                                        let a1674 = v4;
                                        let a1675 = f35({
                                            let a1671 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1671)
                                        }, {
                                            let a1672 = v7;
                                            let a1673 = 97u64;
                                            nat_sub(a1672, a1673)
                                        }, 1u64);
                                        append_bytes(a1674, a1675)
                                    }))))
                                } else {
                                    if f435(f459(v7), f435(f460(v7), f461(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1676 = v1;
                                            let a1677 = 1u64;
                                            nat_add(a1676, a1677)?
                                        }, (v5, (false, {
                                            let a1678 = v4;
                                            let a1679 = f35(v0.clone(), v1, 1u64);
                                            append_bytes(a1678, a1679)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1680 = v1;
                                            let a1681 = 1u64;
                                            nat_add(a1680, a1681)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(v0: Adt18, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt18)> = f462(f463(f242(v0.clone(), f8(Str::lit("$defs")))), f463(f242(v0.clone(), f8(Str::lit("definitions")))));
    let m1687 = f464(4096u64, v3.clone(), List::<Bytes>::nil(), f465(v3)?)?;
    match m1687 {
        Ok(v5) => {
            let m1688 = f466(4096u64, f467(f468(v5)?, List::cons(Adt28::C0(v2, v0), List::<Adt28>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1688 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f342(f213(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
                }
                Err(v6) => {
                    Ok(Err::<Bytes, Bytes>(v6))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<Bytes, Bytes>(v4))
        }
    }
}

pub fn f406(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f357(v1)?;
    let v4: u64 = {
        let a1689 = v3.clone();
        length_bytes(a1689)
    };
    let v5: Bytes = {
        let a1690 = v2;
        let a1691 = v4;
        if nat_lt(a1690, a1691) {
            f7(List::cons(f35(v3.clone(), 0u64, {
                let a1692 = v4;
                let a1693 = v2;
                nat_sub(a1692, a1693)
            }), List::cons(f8(Str::lit(".")), List::cons(f35(v3, {
                let a1694 = v4;
                let a1695 = v2;
                nat_sub(a1694, a1695)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f469({
                let a1696 = v2;
                let a1697 = v4;
                nat_sub(a1696, a1697)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1698 = f8(Str::lit("-"));
        let a1699 = v5;
        Ok(append_bytes(a1698, a1699))
    } else {
        Ok(v5)
    }
}

pub fn f407(v0: Bytes) -> R<Bytes> {
    let a1704 = f432(v0.clone(), 0u64, {
        let a1701 = {
            let a1700 = v0.clone();
            length_bytes(a1700)
        };
        let a1702 = 1u64;
        nat_add(a1701, a1702)?
    })?;
    let a1705 = {
        let a1703 = v0.clone();
        length_bytes(a1703)
    };
    if nat_eq(a1704, a1705) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f268(v0)
    }
}

pub fn f408(v0: Adt18) -> R<List<Bytes>> {
    let m1706 = v0;
    match m1706 {
        Adt18::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C7(r1707, r1708) => {
            let v10 = (*r1707).clone();
            let v11 = (*r1708).clone();
            Ok(List::cons(f333(v10)?, f408(v11)?))
        }
        Adt18::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f409(v0: Adt18) -> R<List<(Bytes, Bytes)>> {
    let m1709 = v0;
    match m1709 {
        Adt18::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C8(v12, r1710, r1711) => {
            let v13 = (*r1710).clone();
            let v14 = (*r1711).clone();
            Ok(List::cons((v12, f333(v13)?), f409(v14)?))
        }
    }
}

pub fn f410(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f342(f470(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f411(v0: u32) -> Bytes {
    let a1740 = f471({
        let a1712 = f472(v0, 28u32);
        let a1713 = 15u32;
        fixed_u32::bit_and(a1712, a1713)
    });
    let a1741 = {
        let a1738 = f471({
            let a1714 = f472(v0, 24u32);
            let a1715 = 15u32;
            fixed_u32::bit_and(a1714, a1715)
        });
        let a1739 = {
            let a1736 = f471({
                let a1716 = f472(v0, 20u32);
                let a1717 = 15u32;
                fixed_u32::bit_and(a1716, a1717)
            });
            let a1737 = {
                let a1734 = f471({
                    let a1718 = f472(v0, 16u32);
                    let a1719 = 15u32;
                    fixed_u32::bit_and(a1718, a1719)
                });
                let a1735 = {
                    let a1732 = f471({
                        let a1720 = f472(v0, 12u32);
                        let a1721 = 15u32;
                        fixed_u32::bit_and(a1720, a1721)
                    });
                    let a1733 = {
                        let a1730 = f471({
                            let a1722 = f472(v0, 8u32);
                            let a1723 = 15u32;
                            fixed_u32::bit_and(a1722, a1723)
                        });
                        let a1731 = {
                            let a1728 = f471({
                                let a1724 = f472(v0, 4u32);
                                let a1725 = 15u32;
                                fixed_u32::bit_and(a1724, a1725)
                            });
                            let a1729 = f471({
                                let a1726 = v0;
                                let a1727 = 15u32;
                                fixed_u32::bit_and(a1726, a1727)
                            });
                            append_bytes(a1728, a1729)
                        };
                        append_bytes(a1730, a1731)
                    };
                    append_bytes(a1732, a1733)
                };
                append_bytes(a1734, a1735)
            };
            append_bytes(a1736, a1737)
        };
        append_bytes(a1738, a1739)
    };
    append_bytes(a1740, a1741)
}

pub fn f412(v0: Fn11, p1745: u64, p1746: (u64, Adt27)) -> R<(u64, Adt27)> {
    let m1744 = (p1745, p1746);
    {
        let mut m1744 = m1744;
        loop {
            let step = {
                let (v1, v2) = m1744;
                let m1742 = v1;
                if m1742 == 0 {
                    Err::<(u64, (u64, Adt27)), R<(u64, Adt27)>>(Ok(v2))
                } else {
                    let v3 = m1742 - 1;
                    Ok::<(u64, (u64, Adt27)), R<(u64, Adt27)>>((v3, {
                        let c1743: Fn11 = v0.clone();
                        c1743.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt27)) -> R<(u64, Adt27)> {
    Ok(({
        let a1748 = {
            let (h1747, _) = v3.clone();
            h1747
        };
        let a1749 = 64u64;
        nat_add(a1748, a1749)?
    }, f473({
        let (_, h1750) = v3;
        h1750
    }, f474(v0, v1, v2, {
        let (h1751, _) = v3;
        h1751
    })?)?))
}

pub fn f414() -> Adt27 {
    Adt27::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f415(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1752 = f33(v1, 34u8)?;
    match m1752.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1753 = v3;
            match m1753.uncons() {
                None => {
                    Ok(f475(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f416(v0: List<Bytes>) -> List<Bytes> {
    let m1754 = v0;
    match m1754.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("authority = []"))) {
                f416(v2)
            } else {
                List::cons(v1, f416(v2))
            }
        }
    }
}

pub fn f417(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1755 = v0;
    match m1755.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f28(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f417(v3, None::<Bytes>)
            } else {
                let m1756 = f476(v2.clone(), f8(Str::lit("url = ")))?;
                match m1756 {
                    None => {
                        let m1757 = f476(v2, f8(Str::lit("artifact = ")))?;
                        match m1757 {
                            None => {
                                f417(v3, v1)
                            }
                            Some(v5) => {
                                let m1758 = v1.clone();
                                match m1758 {
                                    None => {
                                        f417(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f417(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f417(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f418(v0: List<Adt25>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1759 = f477(v0.clone())?;
    match m1759 {
        Ok(v2) => {
            let v3: Adt29 = Adt29::C0(v2.clone(), f478(v0.clone()));
            let m1760 = f479(f480(v0)?, v3, v2)?;
            match m1760 {
                Ok(v5) => {
                    let m1761 = f481(f482(v5), List::<Adt32>::nil());
                    match m1761 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f483(v7.clone());
                            let m1762 = f484(v7.clone(), v8.clone());
                            match m1762 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f485(v7.clone(), v8.clone(), f486(v7.clone())?);
                                    let m1768 = f487({
                                        let a1766 = {
                                            let a1765 = v8.clone();
                                            length_list(a1765)
                                        };
                                        let a1767 = 1u64;
                                        nat_add(a1766, a1767)?
                                    }, v7.clone(), f488(v7, f489(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1768 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1769 = v12.clone();
                                                length_list(a1769)
                                            }, f342(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f422(f8(Str::lit("the type ")), {
                                        let (h1763, _) = v9;
                                        h1763
                                    }, f422(f8(Str::lit(" uses ")), {
                                        let (_, h1764) = v9;
                                        h1764
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

pub fn f419(p1772: List<Adt25>, p1773: List<Adt25>) -> List<Adt25> {
    let m1771 = (p1772, p1773);
    {
        let mut m1771 = m1771;
        loop {
            let step = {
                let (v0, v1) = m1771;
                let m1770 = v0;
                match m1770.uncons() {
                    None => {
                        Err::<(List<Adt25>, List<Adt25>), List<Adt25>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt25>, List<Adt25>), List<Adt25>>((v3, List::cons(v2, v1)))
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

pub fn f420(v0: List<Adt25>) -> List<Bytes> {
    let m1774 = v0;
    match m1774.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt25::C0(h1775, _, _, _, _, _) => {
                    h1775
                }
            }, f420(v2))
        }
    }
}

pub fn f421(p1780: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1779 = p1780;
    {
        let mut m1779 = m1779;
        loop {
            let step = {
                let v0 = m1779;
                let m1776 = v0;
                match m1776.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h1777, _) = v2;
                            h1777
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1778) = v2;
                                h1778
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f422(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f423(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt25, Bytes>> {
    let m1781 = f490(v1.clone())?;
    match m1781 {
        Ok(v4) => {
            let m1782 = v4;
            match m1782 {
                Adt26::C0(v5, r1787, r1788) => {
                    let v6 = (*r1787).clone();
                    let v7 = (*r1788).clone();
                    let m1783 = f491(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1783 {
                        Ok(v9) => {
                            let m1784 = f492(v9.clone(), v5)?;
                            match m1784 {
                                Ok(v11) => {
                                    if f493(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt25, Bytes>(Adt25::C0(v0, v2, {
                                            let m1785 = f494(v6, f8(Str::lit("targetNamespace")));
                                            match m1785 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1786 = v1;
                                            length_bytes(a1786)
                                        })))
                                    } else {
                                        Ok(Err::<Adt25, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(": the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)")), List::<Bytes>::nil())))))
                                    }
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt25, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<Adt25, Bytes>(v8))
                        }
                    }
                }
                Adt26::C1(_, _, _) => {
                    Ok(Err::<Adt25, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt26::C2(_) => {
                    Ok(Err::<Adt25, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt26::C3(_, _) => {
                    Ok(Err::<Adt25, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt26::C4 => {
                    Ok(Err::<Adt25, Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt25, Bytes>(f422(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f424(p1793: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1792 = p1793;
    {
        let mut m1792 = m1792;
        loop {
            let step = {
                let v0 = m1792;
                let m1789 = v0;
                match m1789.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (_, h1790) = v2;
                            h1790
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1791, _) = v2;
                                h1791
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(v0: Adt25, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f28(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f28(match v0.clone() {
            Adt25::C0(_, _, h1794, _, _, _) => {
                h1794
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt25::C0(h1795, _, _, _, _, _) => {
                    h1795
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt25::C0(_, _, h1796, _, _, _) => {
                    h1796
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f28(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f426(v0: Adt26, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt31>, Bytes>> {
    let m1797 = v0;
    match m1797 {
        Adt26::C0(_, _, _) => {
            Ok(Ok::<List<Adt31>, Bytes>(List::<Adt31>::nil()))
        }
        Adt26::C1(_, _, _) => {
            Ok(Ok::<List<Adt31>, Bytes>(List::<Adt31>::nil()))
        }
        Adt26::C2(_) => {
            Ok(Ok::<List<Adt31>, Bytes>(List::<Adt31>::nil()))
        }
        Adt26::C3(r1808, r1809) => {
            let v2 = (*r1808).clone();
            let v3 = (*r1809).clone();
            let m1798 = f426(v3, v1.clone())?;
            match m1798 {
                Ok(v5) => {
                    let m1799 = v2;
                    match m1799 {
                        Adt26::C0(v6, r1806, r1807) => {
                            let v7 = (*r1806).clone();
                            let v8 = (*r1807).clone();
                            let m1800 = f491(v7.clone(), v1);
                            match m1800 {
                                Ok(v10) => {
                                    let m1801 = f492(v10.clone(), v6)?;
                                    match m1801 {
                                        Ok(v12) => {
                                            if if f28({
                                                let (h1802, _) = v12.clone();
                                                h1802
                                            }, f495()) {
                                                f496(f28({
                                                    let (_, h1803) = v12.clone();
                                                    h1803
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt31>, Bytes>(List::cons(Adt31::C0({
                                                    let (h1804, _) = v12;
                                                    h1804
                                                }, {
                                                    let (_, h1805) = v12;
                                                    h1805
                                                }, v7, v8, v10), v5)))
                                            } else {
                                                Ok(Ok::<List<Adt31>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt31>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt31>, Bytes>(v9))
                                }
                            }
                        }
                        Adt26::C1(_, _, _) => {
                            Ok(Ok::<List<Adt31>, Bytes>(v5))
                        }
                        Adt26::C2(_) => {
                            Ok(Ok::<List<Adt31>, Bytes>(v5))
                        }
                        Adt26::C3(_, _) => {
                            Ok(Ok::<List<Adt31>, Bytes>(v5))
                        }
                        Adt26::C4 => {
                            Ok(Ok::<List<Adt31>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt31>, Bytes>(v4))
                }
            }
        }
        Adt26::C4 => {
            Ok(Ok::<List<Adt31>, Bytes>(List::<Adt31>::nil()))
        }
    }
}

pub fn f427(v0: List<Adt31>, v1: Adt25, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1810 = v0;
    match m1810.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1811 = f427(v4, v1.clone(), v2.clone())?;
            match m1811 {
                Ok(v6) => {
                    if if f497(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f497(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f497(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt25::C0(h1812, _, _, _, _, _) => {
                                h1812
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt31::C0(_, h1813, _, _, _) => {
                                h1813
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f497(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f497(v3.clone(), Str::lit("include"))
                        } {
                            let m1814 = f498(v3.clone(), Str::lit("schemaLocation"));
                            match m1814 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt25::C0(h1815, _, _, _, _, _) => {
                                            h1815
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt31::C0(_, h1816, _, _, _) => {
                                            h1816
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1818 = f499(match v1.clone() {
                                        Adt25::C0(_, h1817, _, _, _, _) => {
                                            h1817
                                        }
                                    }, v7)?;
                                    match m1818 {
                                        Ok(v9) => {
                                            let m1820 = f500(v2, v9.clone());
                                            match m1820 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt25::C0(h1821, _, _, _, _, _) => {
                                                            h1821
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt31::C0(_, h1822, _, _, _) => {
                                                            h1822
                                                        }
                                                    }, if f497(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt25::C0(_, _, h1823, _, _, _) => {
                                                                h1823
                                                            }
                                                        }
                                                    } else {
                                                        f501(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f422(match v1 {
                                                Adt25::C0(h1819, _, _, _, _, _) => {
                                                    h1819
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

pub fn f428(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1824 = v0;
    match m1824.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f428(v3, v1))
        }
    }
}

pub fn f429(v0: List<Adt13>, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1825 = v0;
    match m1825.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            if f28(f47(v3.clone(), v1.clone()), v2.clone()) {
                f429(v4, v1, v2)
            } else {
                List::cons(v3, f429(v4, v1, v2))
            }
        }
    }
}

pub fn f430(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1826 = v0;
    match m1826.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f72(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f342(f470(f125(List::cons(v1, f502(f33(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f430(v3, v1)?))
            }
        }
    }
}

pub fn f431(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1829 = v3;
    let a1830 = f35(v0, v1, {
        let a1827 = v2;
        let a1828 = v1;
        nat_sub(a1827, a1828)
    });
    append_bytes(a1829, a1830)
}

pub fn f432(v0: Bytes, p1843: u64, p1844: u64) -> R<u64> {
    let m1842 = (p1843, p1844);
    {
        let mut m1842 = m1842;
        loop {
            let step = {
                let (v1, v2) = m1842;
                let m1831 = v2;
                if m1831 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1831 - 1;
                    let v4: u64 = f351(v0.clone(), v1)?;
                    let a1832 = v4;
                    let a1833 = 34u64;
                    if nat_eq(a1832, a1833) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1834 = v4;
                        let a1835 = 92u64;
                        if nat_eq(a1834, a1835) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1836 = v4;
                            let a1837 = 32u64;
                            if nat_lt(a1836, a1837) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1838 = v4;
                                let a1839 = 256u64;
                                if nat_eq(a1838, a1839) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1840 = v1;
                                        let a1841 = 1u64;
                                        nat_add(a1840, a1841)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f503(Fn14::F504(v0.clone()), {
        let a1846 = {
            let a1845 = v0;
            length_bytes(a1845)
        };
        let a1847 = 1u64;
        nat_add(a1846, a1847)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1851 = {
        let (_, h1848) = {
            let (_, h1849) = {
                let (h1850, _) = v2.clone();
                h1850
            };
            h1849
        };
        h1848
    };
    let a1852 = 1u64;
    if nat_eq(a1851, a1852) {
        Ok(Some(({
            let (h1853, _) = {
                let (h1854, _) = v2.clone();
                h1854
            };
            h1853
        }, {
            let (h1855, _) = {
                let (_, h1856) = {
                    let (h1857, _) = v2;
                    h1857
                };
                h1856
            };
            h1855
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f434(v0: Bytes, v1: u64) -> R<u64> {
    let (h1858, _) = f287(Fn3::F505(v0.clone()), {
        let a1860 = {
            let a1859 = v0;
            length_bytes(a1859)
        };
        let a1861 = 1u64;
        nat_add(a1860, a1861)?
    }, v1)?;
    Ok(h1858)
}

pub fn f435(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f436(v0: Bytes) -> R<u64> {
    let (h1862, _) = f287(Fn3::F506(v0.clone()), {
        let a1864 = {
            let a1863 = v0;
            length_bytes(a1863)
        };
        let a1865 = 1u64;
        nat_add(a1864, a1865)?
    }, 0u64)?;
    Ok(h1862)
}

pub fn f437(v0: Bytes) -> R<Option<u64>> {
    if f507(v0.clone())? {
        Ok(Some({
            let (_, h1866) = {
                let (h1867, _) = f508(Fn15::F509(v0.clone()), {
                    let a1868 = v0;
                    length_bytes(a1868)
                }, (0u64, 0u64))?;
                h1867
            };
            h1866
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f438(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f35(v1.clone(), 0u64, {
        let a1870 = {
            let a1869 = v1.clone();
            length_bytes(a1869)
        };
        let a1871 = f510(v1)?;
        nat_sub(a1870, a1871)
    });
    let v5: Bytes = {
        let a1872 = v0;
        let a1873 = v4.clone();
        append_bytes(a1872, a1873)
    };
    let v6: u64 = f436(v5.clone())?;
    let v7: Bytes = f35(v5.clone(), v6, {
        let a1875 = {
            let a1874 = v5;
            length_bytes(a1874)
        };
        let a1876 = v6;
        nat_sub(a1875, a1876)
    });
    let v8: u64 = {
        let a1877 = v4;
        length_bytes(a1877)
    };
    let v9: u64 = {
        let a1878 = v7.clone();
        length_bytes(a1878)
    };
    let a1879 = v9;
    let a1880 = 0u64;
    if nat_eq(a1879, a1880) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f435(v2, {
            let a1881 = v3;
            let a1882 = v8;
            nat_lt(a1881, a1882)
        }) {
            let v10: u64 = if v2 {
                let a1883 = v3;
                let a1884 = v8;
                nat_add(a1883, a1884)?
            } else {
                let a1885 = v8;
                let a1886 = v3;
                nat_sub(a1885, a1886)
            };
            let v11: u64 = f510(v7.clone())?;
            let a1887 = v10;
            let a1888 = v11;
            if nat_le(a1887, a1888) {
                Ok(Some((f35(v7, 0u64, {
                    let a1889 = v9;
                    let a1890 = v10;
                    nat_sub(a1889, a1890)
                }), 0u64)))
            } else {
                let a1893 = {
                    let a1891 = v10;
                    let a1892 = v11;
                    nat_sub(a1891, a1892)
                };
                let a1894 = 19u64;
                if nat_le(a1893, a1894) {
                    Ok(Some((f35(v7, 0u64, {
                        let a1895 = v9;
                        let a1896 = v11;
                        nat_sub(a1895, a1896)
                    }), {
                        let a1897 = v10;
                        let a1898 = v11;
                        nat_sub(a1897, a1898)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1899 = v3;
                let a1900 = v8;
                nat_sub(a1899, a1900)
            };
            let a1903 = {
                let a1901 = v9;
                let a1902 = v12;
                nat_add(a1901, a1902)?
            };
            let a1904 = 19u64;
            if nat_le(a1903, a1904) {
                Ok(Some(({
                    let a1905 = v7;
                    let a1906 = f469(v12);
                    append_bytes(a1905, a1906)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f439(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1907 = 0u64;
        let a1908 = v1;
        if nat_lt(a1907, a1908) {
            let a1909 = v1;
            let a1910 = 9223372036854775808u64;
            nat_le(a1909, a1910)
        } else {
            false
        }
    } else {
        let a1911 = v1;
        let a1912 = 9223372036854775807u64;
        nat_le(a1911, a1912)
    }
}

pub fn f440(p1915: List<Adt12>, v1: Bytes, v2: Bytes) -> Bytes {
    let m1914 = p1915;
    {
        let mut m1914 = m1914;
        loop {
            let step = {
                let v0 = m1914;
                let m1913 = v0;
                match m1913.uncons() {
                    None => {
                        Err::<List<Adt12>, Bytes>(v1.clone())
                    }
                    Some((v3, v4)) => {
                        if f72(v1.clone(), f88(v3.clone())) {
                            Err::<List<Adt12>, Bytes>(f58(v3, v2.clone()))
                        } else {
                            Ok::<List<Adt12>, Bytes>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1914 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1916 = v2;
    let a1917 = 0u64;
    if nat_eq(a1916, a1917) {
        Ok(f511(v0, 0u64))
    } else {
        let (_, h1918) = {
            let (h1919, _) = f358(Fn4::F512(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1919
        };
        Ok(h1918)
    }
}

pub fn f442() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f443(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1923 = {
        let a1921 = v0.clone();
        let a1922 = {
            let (h1920, _) = v1.clone();
            h1920
        };
        index_bytes(a1921, a1922)
    };
    match m1923 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1925 = {
                    let (h1924, _) = v1.clone();
                    h1924
                };
                let a1926 = 1u64;
                nat_add(a1925, a1926)?
            }, {
                let a1929 = {
                    let (_, h1927) = v1;
                    h1927
                };
                let a1930 = f513(v0, {
                    let (h1928, _) = v1;
                    h1928
                }, v2)?;
                append_bytes(a1929, a1930)
            })))
        }
    }
}

pub fn f444(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1931 = v0.clone();
    match m1931.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1934 = {
                let (h1932, _) = v2;
                h1932
            };
            let a1935 = {
                let (h1933, _) = v1.clone();
                h1933
            };
            if equal(a1934, a1935) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f445(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f514(f515(Fn16::F516, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1940 = {
        let (h1936, _) = f517(Fn17::F518, {
            let a1938 = {
                let a1937 = v0;
                length_list(a1937)
            };
            let a1939 = 1u64;
            nat_add(a1938, a1939)?
        }, v1)?;
        h1936
    };
    match m1940.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f446(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1945 = {
        let a1942 = f268({
            let (h1941, _) = v0;
            h1941
        })?;
        let a1943 = Bytes::lit(&[0x3a]);
        append_bytes(a1942, a1943)
    };
    let a1946 = {
        let (_, h1944) = v0;
        h1944
    };
    Ok(append_bytes(a1945, a1946))
}

pub fn f447(v0: Bytes, v1: u64) -> bool {
    let m1947 = f174(v0, v1);
    match m1947 {
        None => {
            false
        }
        Some(v2) => {
            f519(v2)
        }
    }
}

pub fn f448(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f353(f28({
        let (h1948, _) = v0;
        h1948
    }, {
        let (h1949, _) = v1;
        h1949
    }), f28({
        let (_, h1950) = v0;
        h1950
    }, {
        let (_, h1951) = v1;
        h1951
    }))
}

pub fn f449(v0: Fn12, p1955: List<List<(Bytes, ())>>, p1956: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1954 = (p1955, p1956);
    {
        let mut m1954 = m1954;
        loop {
            let step = {
                let (v1, v2) = m1954;
                let m1952 = v2;
                match m1952.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1953: Fn12 = v0.clone();
                            c1953.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f451(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1958 = {
        let a1957 = v0.clone();
        length_list(a1957)
    };
    let a1959 = 1u64;
    if nat_le(a1958, a1959) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f520(v0)?))
    }
}

pub fn f452(v0: Fn13, p1963: List<(Bytes, ())>, p1964: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1962 = (p1963, p1964);
    {
        let mut m1962 = m1962;
        loop {
            let step = {
                let (v1, v2) = m1962;
                let m1960 = v2;
                match m1960.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1961: Fn13 = v0.clone();
                            c1961.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1962 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f454(p1968: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1967 = p1968;
    {
        let mut m1967 = m1967;
        loop {
            let step = {
                let v0 = m1967;
                let m1965 = v0;
                match m1965.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f521(v2, v1.clone(), 0u64, {
                            let a1966 = v1.clone();
                            length_bytes(a1966)
                        })? {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v3)
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

pub fn f455(v0: Adt1, v1: Bytes, v2: List<Adt13>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f121(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f51(v3.clone(), f121(f82(v0, f132())?, f8(Str::lit("at"))));
    let a1970 = {
        let a1969 = v4.clone();
        length_list(a1969)
    };
    let a1971 = 0u64;
    if nat_eq(a1970, a1971) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1974 = {
            let a1972 = v4;
            length_list(a1972)
        };
        let a1975 = {
            let a1973 = v3;
            length_list(a1973)
        };
        if nat_eq(a1974, a1975) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f522(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f456(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt13> = f324(f236(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt13> = f324(f236(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f98(f121(v3.clone(), f8(Str::lit("id"))), f121(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f121(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f51(v5.clone(), f98(v6.clone(), f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1977 = {
        let a1976 = v7.clone();
        length_list(a1976)
    };
    let a1978 = 0u64;
    if nat_eq(a1977, a1978) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f40({
            let a1981 = {
                let a1979 = v7;
                length_list(a1979)
            };
            let a1982 = {
                let a1980 = v5;
                length_list(a1980)
            };
            nat_eq(a1981, a1982)
        }) {
            Ok((false, f522(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f125(f98(f51(f523(v4.clone()), f52(f53(v0, f8(Str::lit("model/authorities.toml"))))?), f51(f524(v4), f98(v6, f121(v3, f8(Str::lit("id")))))))?;
            let a1984 = {
                let a1983 = v8.clone();
                length_list(a1983)
            };
            let a1985 = 0u64;
            if nat_eq(a1984, a1985) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f522(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f95(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f457(v0: u64) -> bool {
    let a1986 = v0;
    let a1987 = 47u64;
    nat_eq(a1986, a1987)
}

pub fn f458(v0: u64) -> bool {
    f435({
        let a1988 = v0;
        let a1989 = 95u64;
        nat_eq(a1988, a1989)
    }, f435({
        let a1990 = v0;
        let a1991 = 45u64;
        nat_eq(a1990, a1991)
    }, f435({
        let a1992 = v0;
        let a1993 = 32u64;
        nat_eq(a1992, a1993)
    }, f435({
        let a1994 = v0;
        let a1995 = 46u64;
        nat_eq(a1994, a1995)
    }, {
        let a1996 = v0;
        let a1997 = 47u64;
        nat_eq(a1996, a1997)
    }))))
}

pub fn f459(v0: u64) -> bool {
    f353({
        let a1998 = 97u64;
        let a1999 = v0;
        nat_le(a1998, a1999)
    }, {
        let a2000 = v0;
        let a2001 = 122u64;
        nat_le(a2000, a2001)
    })
}

pub fn f460(v0: u64) -> bool {
    f353({
        let a2002 = 65u64;
        let a2003 = v0;
        nat_le(a2002, a2003)
    }, {
        let a2004 = v0;
        let a2005 = 90u64;
        nat_le(a2004, a2005)
    })
}

pub fn f461(v0: u64) -> bool {
    f353({
        let a2006 = 48u64;
        let a2007 = v0;
        nat_le(a2006, a2007)
    }, {
        let a2008 = v0;
        let a2009 = 57u64;
        nat_le(a2008, a2009)
    })
}

pub fn f462(v0: List<(Bytes, Adt18)>, v1: List<(Bytes, Adt18)>) -> List<(Bytes, Adt18)> {
    let m2010 = v0;
    match m2010.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f462(v3, v1))
        }
    }
}

pub fn f463(v0: Option<Adt18>) -> List<(Bytes, Adt18)> {
    let m2011 = v0;
    match m2011 {
        None => {
            List::<(Bytes, Adt18)>::nil()
        }
        Some(v1) => {
            f525(v1)
        }
    }
}

pub fn f464(v0: u64, v1: List<(Bytes, Adt18)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt18)>, Bytes>> {
    let m2012 = v0;
    if m2012 == 0 {
        let m2013 = v1;
        match m2013.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::<(Bytes, Adt18)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt18)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2012 - 1;
        let m2014 = v1.clone();
        match m2014.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::<(Bytes, Adt18)>::nil()))
            }
            Some((_, _)) => {
                let m2015 = f526(v1.clone(), v2.clone(), v3.clone())?;
                match m2015 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt18)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f342(f465(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2018 = f464(v6, f527(v1, {
                            let (h2016, _) = v9.clone();
                            h2016
                        }), List::cons(f528({
                            let (h2017, _) = v9.clone();
                            h2017
                        })?, v2), v3)?;
                        match m2018 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt18)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f465(v0: List<(Bytes, Adt18)>) -> R<List<Bytes>> {
    let m2019 = v0;
    match m2019.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f528({
                let (h2020, _) = v1;
                h2020
            })?, f465(v2)?))
        }
    }
}

pub fn f466(p2026: u64, p2027: List<Adt28>, p2028: List<Bytes>, p2029: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2025 = (p2026, (p2027, (p2028, p2029)));
    {
        let mut m2025 = m2025;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2025;
                let m2021 = v0;
                if m2021 == 0 {
                    Err::<(u64, (List<Adt28>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2021 - 1;
                    let m2022 = v1;
                    match m2022.uncons() {
                        None => {
                            Err::<(u64, (List<Adt28>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2023 = v5;
                            match m2023 {
                                Adt28::C0(v8, v9) => {
                                    if f28(v8.clone(), f529()) {
                                        Err::<(u64, (List<Adt28>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f41(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt28>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2024 = f531(v8.clone(), v9)?;
                                            match m2024 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt28>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f467(f532(v11.clone()), List::cons(Adt28::C1(f533(v11)), v6)), (List::cons(v8, v2), v3))))
                                                }
                                                Err(v10) => {
                                                    Err::<(u64, (List<Adt28>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt28::C1(v7) => {
                                    Ok::<(u64, (List<Adt28>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                                Adt28::C2 => {
                                    if f41(f529(), v2.clone()) {
                                        Ok::<(u64, (List<Adt28>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt28>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f529(), v2), List::cons(f530(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2025 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: List<Adt28>, v1: List<Adt28>) -> List<Adt28> {
    let m2030 = v0;
    match m2030.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f467(v3, v1))
        }
    }
}

pub fn f468(v0: List<(Bytes, Adt18)>) -> R<List<Adt28>> {
    let m2031 = v0;
    match m2031.uncons() {
        None => {
            Ok(List::<Adt28>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt28::C0(f528({
                let (h2032, _) = v1;
                h2032
            })?, {
                let (_, h2033) = v1;
                h2033
            }), f468(v2)?))
        }
    }
}

pub fn f469(v0: u64) -> Bytes {
    let m2034 = v0;
    if m2034 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2034 - 1;
        let a2035 = f8(Str::lit("0"));
        let a2036 = f469(v1);
        append_bytes(a2035, a2036)
    }
}

pub fn f470(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2037 = v0;
    match m2037.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f307(v1)?, f470(v2)?))
        }
    }
}

pub fn f471(v0: u32) -> Bytes {
    let a2040 = {
        let a2038 = v0;
        let a2039 = 8u32;
        fixed_u32::bit_and(a2038, a2039)
    };
    let a2041 = 0u32;
    if equal(a2040, a2041) {
        let a2044 = {
            let a2042 = v0;
            let a2043 = 4u32;
            fixed_u32::bit_and(a2042, a2043)
        };
        let a2045 = 0u32;
        if equal(a2044, a2045) {
            let a2048 = {
                let a2046 = v0;
                let a2047 = 2u32;
                fixed_u32::bit_and(a2046, a2047)
            };
            let a2049 = 0u32;
            if equal(a2048, a2049) {
                let a2052 = {
                    let a2050 = v0;
                    let a2051 = 1u32;
                    fixed_u32::bit_and(a2050, a2051)
                };
                let a2053 = 0u32;
                if equal(a2052, a2053) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2056 = {
                    let a2054 = v0;
                    let a2055 = 1u32;
                    fixed_u32::bit_and(a2054, a2055)
                };
                let a2057 = 0u32;
                if equal(a2056, a2057) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2060 = {
                let a2058 = v0;
                let a2059 = 2u32;
                fixed_u32::bit_and(a2058, a2059)
            };
            let a2061 = 0u32;
            if equal(a2060, a2061) {
                let a2064 = {
                    let a2062 = v0;
                    let a2063 = 1u32;
                    fixed_u32::bit_and(a2062, a2063)
                };
                let a2065 = 0u32;
                if equal(a2064, a2065) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2068 = {
                    let a2066 = v0;
                    let a2067 = 1u32;
                    fixed_u32::bit_and(a2066, a2067)
                };
                let a2069 = 0u32;
                if equal(a2068, a2069) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2072 = {
            let a2070 = v0;
            let a2071 = 4u32;
            fixed_u32::bit_and(a2070, a2071)
        };
        let a2073 = 0u32;
        if equal(a2072, a2073) {
            let a2076 = {
                let a2074 = v0;
                let a2075 = 2u32;
                fixed_u32::bit_and(a2074, a2075)
            };
            let a2077 = 0u32;
            if equal(a2076, a2077) {
                let a2080 = {
                    let a2078 = v0;
                    let a2079 = 1u32;
                    fixed_u32::bit_and(a2078, a2079)
                };
                let a2081 = 0u32;
                if equal(a2080, a2081) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2084 = {
                    let a2082 = v0;
                    let a2083 = 1u32;
                    fixed_u32::bit_and(a2082, a2083)
                };
                let a2085 = 0u32;
                if equal(a2084, a2085) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2088 = {
                let a2086 = v0;
                let a2087 = 2u32;
                fixed_u32::bit_and(a2086, a2087)
            };
            let a2089 = 0u32;
            if equal(a2088, a2089) {
                let a2092 = {
                    let a2090 = v0;
                    let a2091 = 1u32;
                    fixed_u32::bit_and(a2090, a2091)
                };
                let a2093 = 0u32;
                if equal(a2092, a2093) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2096 = {
                    let a2094 = v0;
                    let a2095 = 1u32;
                    fixed_u32::bit_and(a2094, a2095)
                };
                let a2097 = 0u32;
                if equal(a2096, a2097) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f472(v0: u32, v1: u32) -> u32 {
    let m2100 = {
        let a2098 = v0;
        let a2099 = v1;
        fixed_u32::shift_right(a2098, a2099)
    };
    match m2100 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f473(v0: Adt27, v1: Adt33) -> R<Adt27> {
    let v2: Adt34 = f534(Fn18::F535, 64u64, Adt34::C0(match v0.clone() {
        Adt27::C0(h2101, _, _, _, _, _, _, _) => {
            h2101
        }
    }, match v0.clone() {
        Adt27::C0(_, h2102, _, _, _, _, _, _) => {
            h2102
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2103, _, _, _, _, _) => {
            h2103
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, h2104, _, _, _, _) => {
            h2104
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, h2105, _, _, _) => {
            h2105
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2106, _, _) => {
            h2106
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2107, _) => {
            h2107
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2108) => {
            h2108
        }
    }, v1, 0u64))?;
    Ok(Adt27::C0(f536(match v0.clone() {
        Adt27::C0(h2109, _, _, _, _, _, _, _) => {
            h2109
        }
    }, match v2.clone() {
        Adt34::C0(h2110, _, _, _, _, _, _, _, _, _) => {
            h2110
        }
    }), f536(match v0.clone() {
        Adt27::C0(_, h2111, _, _, _, _, _, _) => {
            h2111
        }
    }, match v2.clone() {
        Adt34::C0(_, h2112, _, _, _, _, _, _, _, _) => {
            h2112
        }
    }), f536(match v0.clone() {
        Adt27::C0(_, _, h2113, _, _, _, _, _) => {
            h2113
        }
    }, match v2.clone() {
        Adt34::C0(_, _, h2114, _, _, _, _, _, _, _) => {
            h2114
        }
    }), f536(match v0.clone() {
        Adt27::C0(_, _, _, h2115, _, _, _, _) => {
            h2115
        }
    }, match v2.clone() {
        Adt34::C0(_, _, _, h2116, _, _, _, _, _, _) => {
            h2116
        }
    }), f536(match v0.clone() {
        Adt27::C0(_, _, _, _, h2117, _, _, _) => {
            h2117
        }
    }, match v2.clone() {
        Adt34::C0(_, _, _, _, h2118, _, _, _, _, _) => {
            h2118
        }
    }), f536(match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2119, _, _) => {
            h2119
        }
    }, match v2.clone() {
        Adt34::C0(_, _, _, _, _, h2120, _, _, _, _) => {
            h2120
        }
    }), f536(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2121, _) => {
            h2121
        }
    }, match v2.clone() {
        Adt34::C0(_, _, _, _, _, _, h2122, _, _, _) => {
            h2122
        }
    }), f536(match v0 {
        Adt27::C0(_, _, _, _, _, _, _, h2123) => {
            h2123
        }
    }, match v2 {
        Adt34::C0(_, _, _, _, _, _, _, h2124, _, _) => {
            h2124
        }
    })))
}

pub fn f474(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt33> {
    Ok(Adt33::C0(f537(v0.clone(), v1, v2, v3)?, f537(v0.clone(), v1, v2, {
        let a2125 = v3;
        let a2126 = 4u64;
        nat_add(a2125, a2126)?
    })?, f537(v0.clone(), v1, v2, {
        let a2127 = v3;
        let a2128 = 8u64;
        nat_add(a2127, a2128)?
    })?, f537(v0.clone(), v1, v2, {
        let a2129 = v3;
        let a2130 = 12u64;
        nat_add(a2129, a2130)?
    })?, f537(v0.clone(), v1, v2, {
        let a2131 = v3;
        let a2132 = 16u64;
        nat_add(a2131, a2132)?
    })?, f537(v0.clone(), v1, v2, {
        let a2133 = v3;
        let a2134 = 20u64;
        nat_add(a2133, a2134)?
    })?, f537(v0.clone(), v1, v2, {
        let a2135 = v3;
        let a2136 = 24u64;
        nat_add(a2135, a2136)?
    })?, f537(v0.clone(), v1, v2, {
        let a2137 = v3;
        let a2138 = 28u64;
        nat_add(a2137, a2138)?
    })?, f537(v0.clone(), v1, v2, {
        let a2139 = v3;
        let a2140 = 32u64;
        nat_add(a2139, a2140)?
    })?, f537(v0.clone(), v1, v2, {
        let a2141 = v3;
        let a2142 = 36u64;
        nat_add(a2141, a2142)?
    })?, f537(v0.clone(), v1, v2, {
        let a2143 = v3;
        let a2144 = 40u64;
        nat_add(a2143, a2144)?
    })?, f537(v0.clone(), v1, v2, {
        let a2145 = v3;
        let a2146 = 44u64;
        nat_add(a2145, a2146)?
    })?, f537(v0.clone(), v1, v2, {
        let a2147 = v3;
        let a2148 = 48u64;
        nat_add(a2147, a2148)?
    })?, f537(v0.clone(), v1, v2, {
        let a2149 = v3;
        let a2150 = 52u64;
        nat_add(a2149, a2150)?
    })?, f537(v0.clone(), v1, v2, {
        let a2151 = v3;
        let a2152 = 56u64;
        nat_add(a2151, a2152)?
    })?, f537(v0, v1, v2, {
        let a2153 = v3;
        let a2154 = 60u64;
        nat_add(a2153, a2154)?
    })?))
}

pub fn f475(p2157: List<Bytes>, v1: Bytes) -> bool {
    let m2156 = p2157;
    {
        let mut m2156 = m2156;
        loop {
            let step = {
                let v0 = m2156;
                let m2155 = v0;
                match m2155.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f72(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), v1) {
        let m2158 = f33(v0, 34u8)?;
        match m2158.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m2159 = v3;
                match m2159.uncons() {
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

pub fn f477(v0: List<Adt25>) -> R<Result<List<Adt30>, Bytes>> {
    let m2160 = v0;
    match m2160.uncons() {
        None => {
            Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
        }
        Some((v1, v2)) => {
            let m2163 = f426(match v1.clone() {
                Adt25::C0(_, _, _, _, h2161, _) => {
                    h2161
                }
            }, match v1.clone() {
                Adt25::C0(_, _, _, h2162, _, _) => {
                    h2162
                }
            })?;
            match m2163 {
                Ok(v4) => {
                    let m2164 = f477(v2)?;
                    match m2164 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt30>, Bytes>(f538(f539(v4, match v1 {
                                Adt25::C0(_, _, h2165, _, _, _) => {
                                    h2165
                                }
                            }), v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt30>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt30>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f478(v0: List<Adt25>) -> List<Bytes> {
    let m2166 = v0;
    match m2166.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt25::C0(_, _, h2167, _, _, _) => {
                    h2167
                }
            }, f478(v2))
        }
    }
}

pub fn f479(v0: u64, v1: Adt29, v2: List<Adt30>) -> R<Result<List<Adt32>, Bytes>> {
    let m2168 = v2;
    match m2168.uncons() {
        None => {
            Ok(Ok::<List<Adt32>, Bytes>(List::<Adt32>::nil()))
        }
        Some((v3, v4)) => {
            let m2169 = f540(v0, v1.clone(), v3)?;
            match m2169 {
                Ok(v6) => {
                    let m2170 = f479(v0, v1, v4)?;
                    match m2170 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt32>, Bytes>(f541(v6, v8)))
                        }
                        Err(v7) => {
                            Ok(Err::<List<Adt32>, Bytes>(v7))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<Adt32>, Bytes>(v5))
                }
            }
        }
    }
}

pub fn f480(v0: List<Adt25>) -> R<u64> {
    let m2171 = v0;
    match m2171.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a2173 = match v1 {
                Adt25::C0(_, _, _, _, _, h2172) => {
                    h2172
                }
            };
            let a2174 = f480(v2)?;
            nat_add(a2173, a2174)
        }
    }
}

pub fn f481(p2182: List<Adt32>, p2183: List<Adt32>) -> Result<List<Adt32>, Bytes> {
    let m2181 = (p2182, p2183);
    {
        let mut m2181 = m2181;
        loop {
            let step = {
                let (v0, v1) = m2181;
                let m2175 = v0;
                match m2175.uncons() {
                    None => {
                        Err::<(List<Adt32>, List<Adt32>), Result<List<Adt32>, Bytes>>(Ok::<List<Adt32>, Bytes>(f542(v1, List::<Adt32>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2177 = f543(v1.clone(), match v2.clone() {
                            Adt32::C0(h2176, _, _, _) => {
                                h2176
                            }
                        });
                        match m2177 {
                            None => {
                                Ok::<(List<Adt32>, List<Adt32>), Result<List<Adt32>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f28(match v4 {
                                    Adt32::C0(_, _, h2178, _) => {
                                        h2178
                                    }
                                }, match v2.clone() {
                                    Adt32::C0(_, _, h2179, _) => {
                                        h2179
                                    }
                                }) {
                                    Ok::<(List<Adt32>, List<Adt32>), Result<List<Adt32>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt32>, List<Adt32>), Result<List<Adt32>, Bytes>>(Err::<List<Adt32>, Bytes>(f422(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt32::C0(h2180, _, _, _) => {
                                            h2180
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f482(v0: List<Adt32>) -> List<Adt32> {
    if if f41(f544(), f545(v0.clone())) {
        let m2184 = f543(v0.clone(), f544());
        match m2184 {
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
        List::cons(Adt32::C0(f544(), List::<Bytes>::nil(), f546(f544(), f547()), f547()), v0)
    } else {
        v0
    }
}

pub fn f483(v0: List<Adt32>) -> List<Bytes> {
    let m2185 = v0;
    match m2185.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt32::C0(h2186, _, _, _) => {
                    h2186
                }
            }, f483(v2))
        }
    }
}

pub fn f484(p2192: List<Adt32>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m2191 = p2192;
    {
        let mut m2191 = m2191;
        loop {
            let step = {
                let v0 = m2191;
                let m2187 = v0;
                match m2187.uncons() {
                    None => {
                        Err::<List<Adt32>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m2189 = f548(match v2.clone() {
                            Adt32::C0(_, h2188, _, _) => {
                                h2188
                            }
                        }, v1.clone());
                        match m2189 {
                            None => {
                                Ok::<List<Adt32>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt32>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt32::C0(h2190, _, _, _) => {
                                        h2190
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2191 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(v0: List<Adt32>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m2193 = v1;
    match m2193.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f549(v2, v0.clone(), f550(v0.clone(), v3), List::<Bytes>::nil())), f485(v0, v4, v2))
        }
    }
}

pub fn f486(v0: List<Adt32>) -> R<u64> {
    let m2194 = v0;
    match m2194.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a2199 = {
                let a2197 = {
                    let a2196 = match v1 {
                        Adt32::C0(_, h2195, _, _) => {
                            h2195
                        }
                    };
                    length_list(a2196)
                };
                let a2198 = 1u64;
                nat_add(a2197, a2198)?
            };
            let a2200 = f486(v2)?;
            nat_add(a2199, a2200)
        }
    }
}

pub fn f487(p2210: u64, v1: List<Adt32>, p2211: List<(List<Bytes>, List<Bytes>)>, p2212: List<Bytes>, p2213: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2209 = (p2210, (p2211, (p2212, p2213)));
    {
        let mut m2209 = m2209;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m2209;
                let m2201 = v0;
                if m2201 == 0 {
                    let m2202 = v2;
                    match m2202.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m2201 - 1;
                    let m2203 = v2.clone();
                    match m2203.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m2204 = f551(v2, v3.clone());
                            match m2204 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m2206 = f552(v1.clone(), {
                                        let (h2205, _) = v10.clone();
                                        h2205
                                    });
                                    match m2206 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h2207) = v10;
                                                h2207
                                            }, (f553({
                                                let (h2208, _) = v10;
                                                h2208
                                            }, v3), f553(v4, v12)))))
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
                Ok(next) => { m2209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: List<Adt32>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m2214 = v1;
    match m2214.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f554(v0.clone(), v2.clone(), v2)), f488(v0, v3))
        }
    }
}

pub fn f489(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m2215 = v2;
    match m2215.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f41(v4.clone(), v3.clone()) {
                f489(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f555(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f489(v0, v1, v5, f553(v6, v3)))
            }
        }
    }
}

pub fn f490(v0: Bytes) -> R<Result<Adt26, Bytes>> {
    let v1: Bytes = if f289(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f35(v0.clone(), 3u64, {
            let a2217 = {
                let a2216 = v0;
                length_bytes(a2216)
            };
            let a2218 = 3u64;
            nat_sub(a2217, a2218)
        })
    } else {
        v0
    };
    let v2: Bytes = f556(v1)?;
    let v3: u64 = {
        let a2219 = v2.clone();
        length_bytes(a2219)
    };
    let v4: u64 = f557(v2.clone())?;
    let a2220 = v4;
    let a2221 = v3;
    if nat_lt(a2220, a2221) {
        Ok(f558(f559(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m2222 = f560(v2.clone())?;
        match m2222 {
            Ok(v6) => {
                let m2225 = f561(v2.clone(), v6, {
                    let a2223 = v3;
                    let a2224 = 2u64;
                    nat_add(a2223, a2224)?
                })?;
                match m2225 {
                    Ok(v8) => {
                        if f562(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f558(f559(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a2226 = f563(v2.clone(), v8)?;
                            let a2227 = 60u64;
                            if nat_eq(a2226, a2227) {
                                let m2230 = f564(v2.clone(), v8, {
                                    let a2228 = v3;
                                    let a2229 = 2u64;
                                    nat_add(a2228, a2229)?
                                }, 0u64)?;
                                match m2230 {
                                    Ok(v10) => {
                                        let m2234 = f561(v2, {
                                            let (_, h2231) = v10;
                                            h2231
                                        }, {
                                            let a2232 = v3;
                                            let a2233 = 2u64;
                                            nat_add(a2232, a2233)?
                                        })?;
                                        match m2234 {
                                            Ok(v12) => {
                                                let a2235 = v12;
                                                let a2236 = v3;
                                                if nat_eq(a2235, a2236) {
                                                    Ok(Ok::<Adt26, Bytes>({
                                                        let (h2237, _) = v10;
                                                        h2237
                                                    }))
                                                } else {
                                                    Ok(f558(f559(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f558(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f558(v9))
                                    }
                                }
                            } else {
                                Ok(f558(f559(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f558(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f558(v5))
            }
        }
    }
}

pub fn f491(v0: Adt26, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m2238 = v0;
    match m2238 {
        Adt26::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt26::C1(v2, v3, r2243) => {
            let v4 = (*r2243).clone();
            let m2239 = f491(v4, v1);
            match m2239 {
                Ok(v6) => {
                    if f28(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f28(v3.clone(), f566()) {
                            true
                        } else {
                            f28(v3.clone(), f567())
                        } {
                            f565(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f72(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f35(v2.clone(), 6u64, {
                                let a2241 = {
                                    let a2240 = v2;
                                    length_bytes(a2240)
                                };
                                let a2242 = 6u64;
                                nat_sub(a2241, a2242)
                            });
                            if f28(v7.clone(), f8(Str::lit("xmlns"))) {
                                f565(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f28(v7.clone(), f8(Str::lit("xml"))) {
                                    !f28(v3.clone(), f566())
                                } else {
                                    false
                                } {
                                    f565(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f28(v3.clone(), f566()) {
                                        true
                                    } else {
                                        f28(v3.clone(), f567())
                                    } {
                                        !f28(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f565(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f28(v3.clone(), Bytes::lit(&[])) {
                                            f565(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f565(v5)
                }
            }
        }
        Adt26::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt26::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt26::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f492(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f568(v1.clone())?;
    if f28(v2.clone(), Bytes::lit(&[])) {
        let m2244 = f569(v0, Bytes::lit(&[]));
        match m2244 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f28(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f566(), f570(v1)?)))
        } else {
            let m2245 = f569(v0, v2.clone());
            match m2245 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f570(v1)?)))
                }
            }
        }
    }
}

pub fn f493(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f28({
        let (h2246, _) = v0;
        h2246
    }, f495()) {
        f28({
            let (_, h2247) = v0;
            h2247
        }, {
            let a2248 = v1;
            utf8_encode(a2248)
        })
    } else {
        false
    }
}

pub fn f494(p2252: Adt26, v1: Bytes) -> Option<Bytes> {
    let m2251 = p2252;
    {
        let mut m2251 = m2251;
        loop {
            let step = {
                let v0 = m2251;
                let m2249 = v0;
                match m2249 {
                    Adt26::C0(_, _, _) => {
                        Err::<Adt26, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt26::C1(v2, v3, r2250) => {
                        let v4 = (*r2250).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt26, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt26, Option<Bytes>>(v4)
                        }
                    }
                    Adt26::C2(_) => {
                        Err::<Adt26, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt26::C3(_, _) => {
                        Err::<Adt26, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt26::C4 => {
                        Err::<Adt26, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m2251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f496(v0: bool) -> bool {
    !v0
}

pub fn f497(v0: Adt31, v1: Str) -> bool {
    f28(match v0 {
        Adt31::C0(_, h2253, _, _, _) => {
            h2253
        }
    }, f8(v1))
}

pub fn f498(v0: Adt31, v1: Str) -> Option<Bytes> {
    f494(match v0 {
        Adt31::C0(_, _, h2254, _, _) => {
            h2254
        }
    }, f8(v1))
}

pub fn f499(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m2255 = f571(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m2255 {
        None => {
            let v3: Bytes = f422(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f72(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m2256 = f571(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m2256 {
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
                let m2257 = v0;
                match m2257 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m2259 = f572(v5.clone(), 0u64, {
                            let a2258 = v5.clone();
                            length_bytes(a2258)
                        }, None::<u64>)?;
                        match m2259 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a2262 = f35(v5, 0u64, {
                                        let a2260 = v6;
                                        let a2261 = 1u64;
                                        nat_add(a2260, a2261)?
                                    });
                                    let a2263 = v1;
                                    append_bytes(a2262, a2263)
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

pub fn f500(p2268: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m2267 = p2268;
    {
        let mut m2267 = m2267;
        loop {
            let step = {
                let v0 = m2267;
                let m2264 = v0;
                match m2264.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h2265, _) = v2;
                            h2265
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h2266) = v2;
                                h2266
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2267 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(v0: Adt31, v1: Str, v2: Str) -> Bytes {
    let m2269 = f498(v0, v1);
    match m2269 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f502(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2270 = v0;
    match m2270.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f502(v3, false))
            } else {
                f502(v3, true)
            }
        }
    }
}

pub fn f503(v0: Fn14, p2275: u64, p2276: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2274 = (p2275, p2276);
    {
        let mut m2274 = m2274;
        loop {
            let step = {
                let (v1, v2) = m2274;
                let m2271 = v1;
                if m2271 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2271 - 1;
                    let m2273 = {
                        let c2272: Fn14 = v0.clone();
                        c2272.apply(v2.clone())?
                    };
                    match m2273 {
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
                Ok(next) => { m2274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f573(v0, v1)
}

pub fn f505(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f574(v0, v1)
}

pub fn f506(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f575(v0, v1)
}

pub fn f507(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2277 = v0.clone();
        length_bytes(a2277)
    };
    let a2278 = 0u64;
    let a2279 = v1;
    if nat_lt(a2278, a2279) {
        let a2280 = f576(v0.clone())?;
        let a2281 = v1;
        if nat_eq(a2280, a2281) {
            let a2282 = 1u64;
            let a2283 = v1;
            if nat_lt(a2282, a2283) {
                let m2286 = {
                    let a2284 = v0;
                    let a2285 = 0u64;
                    index_bytes(a2284, a2285)
                };
                match m2286 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2287 = v2;
                        let a2288 = 48u8;
                        Ok(!equal(a2287, a2288))
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

pub fn f508(v0: Fn15, p2293: u64, p2294: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2292 = (p2293, p2294);
    {
        let mut m2292 = m2292;
        loop {
            let step = {
                let (v1, v2) = m2292;
                let m2289 = v1;
                if m2289 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2289 - 1;
                    let m2291 = {
                        let c2290: Fn15 = v0.clone();
                        c2290.apply(v2)?
                    };
                    match m2291 {
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
                Ok(next) => { m2292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f577(v0, v1)
}

pub fn f510(v0: Bytes) -> R<u64> {
    let (h2295, _) = f287(Fn3::F578(v0.clone()), {
        let a2297 = {
            let a2296 = v0;
            length_bytes(a2296)
        };
        let a2298 = 1u64;
        nat_add(a2297, a2298)?
    }, 0u64)?;
    Ok(h2295)
}

pub fn f511(v0: Bytes, v1: u64) -> Bytes {
    let m2302 = {
        let a2299 = v0;
        let a2300 = v1;
        let a2301 = 1u64;
        slice_bytes(a2299, a2300, a2301)
    };
    match m2302 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f512(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f579(v1, v0, v2)
}

pub fn f513(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2347 = {
            let a2323 = {
                let a2311 = {
                    let a2305 = {
                        let a2303 = v3;
                        let a2304 = 128u8;
                        fixed_u8::bit_and(a2303, a2304)
                    };
                    let a2306 = 128u8;
                    if equal(a2305, a2306) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2312 = {
                    let a2309 = {
                        let a2307 = v3;
                        let a2308 = 64u8;
                        fixed_u8::bit_and(a2307, a2308)
                    };
                    let a2310 = 64u8;
                    if equal(a2309, a2310) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2311, a2312)?
            };
            let a2324 = {
                let a2321 = {
                    let a2315 = {
                        let a2313 = v3;
                        let a2314 = 32u8;
                        fixed_u8::bit_and(a2313, a2314)
                    };
                    let a2316 = 32u8;
                    if equal(a2315, a2316) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2322 = {
                    let a2319 = {
                        let a2317 = v3;
                        let a2318 = 16u8;
                        fixed_u8::bit_and(a2317, a2318)
                    };
                    let a2320 = 16u8;
                    if equal(a2319, a2320) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2321, a2322)?
            };
            nat_add(a2323, a2324)?
        };
        let a2348 = {
            let a2345 = {
                let a2333 = {
                    let a2327 = {
                        let a2325 = v3;
                        let a2326 = 8u8;
                        fixed_u8::bit_and(a2325, a2326)
                    };
                    let a2328 = 8u8;
                    if equal(a2327, a2328) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2334 = {
                    let a2331 = {
                        let a2329 = v3;
                        let a2330 = 4u8;
                        fixed_u8::bit_and(a2329, a2330)
                    };
                    let a2332 = 4u8;
                    if equal(a2331, a2332) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2333, a2334)?
            };
            let a2346 = {
                let a2343 = {
                    let a2337 = {
                        let a2335 = v3;
                        let a2336 = 2u8;
                        fixed_u8::bit_and(a2335, a2336)
                    };
                    let a2338 = 2u8;
                    if equal(a2337, a2338) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2344 = {
                    let a2341 = {
                        let a2339 = v3;
                        let a2340 = 1u8;
                        fixed_u8::bit_and(a2339, a2340)
                    };
                    let a2342 = 1u8;
                    if equal(a2341, a2342) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2343, a2344)?
            };
            nat_add(a2345, a2346)?
        };
        nat_add(a2347, a2348)?
    };
    let a2349 = v4;
    let a2350 = 34u64;
    if nat_eq(a2349, a2350) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2351 = v4;
        let a2352 = 92u64;
        if nat_eq(a2351, a2352) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2353 = v4;
            let a2354 = 8u64;
            if nat_eq(a2353, a2354) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2355 = v4;
                let a2356 = 12u64;
                if nat_eq(a2355, a2356) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2357 = v4;
                    let a2358 = 10u64;
                    if nat_eq(a2357, a2358) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2359 = v4;
                        let a2360 = 13u64;
                        if nat_eq(a2359, a2360) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2361 = v4;
                            let a2362 = 9u64;
                            if nat_eq(a2361, a2362) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2363 = v4;
                                let a2364 = 32u64;
                                if nat_lt(a2363, a2364) {
                                    let a2365 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2366 = f580(v2)?;
                                    Ok(append_bytes(a2365, a2366))
                                } else {
                                    Ok(f35(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f514(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f581(Fn19::F582, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f515(v0: Fn16, p2370: List<List<(Bytes, Bytes)>>, p2371: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2369 = (p2370, p2371);
    {
        let mut m2369 = m2369;
        loop {
            let step = {
                let (v1, v2) = m2369;
                let m2367 = v2;
                match m2367.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2368: Fn16 = v0.clone();
                            c2368.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2369 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f517(v0: Fn17, p2376: u64, p2377: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2375 = (p2376, p2377);
    {
        let mut m2375 = m2375;
        loop {
            let step = {
                let (v1, v2) = m2375;
                let m2372 = v1;
                if m2372 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2372 - 1;
                    let m2374 = {
                        let c2373: Fn17 = v0.clone();
                        c2373.apply(v2.clone())?
                    };
                    match m2374 {
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
                Ok(next) => { m2375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f518(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f583(v0)
}

pub fn f519(v0: u8) -> bool {
    let a2378 = v0;
    let a2379 = 32u8;
    if if if equal(a2378, a2379) {
        true
    } else {
        let a2380 = v0;
        let a2381 = 9u8;
        equal(a2380, a2381)
    } {
        true
    } else {
        let a2382 = v0;
        let a2383 = 10u8;
        equal(a2382, a2383)
    } {
        true
    } else {
        let a2384 = v0;
        let a2385 = 12u8;
        if equal(a2384, a2385) {
            true
        } else {
            let a2386 = v0;
            let a2387 = 13u8;
            equal(a2386, a2387)
        }
    }
}

pub fn f520(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f387({
        let (_, h2388) = {
            let (h2389, _) = f584(Fn20::F585, {
                let a2391 = {
                    let a2390 = v0.clone();
                    length_list(a2390)
                };
                let a2392 = 1u64;
                nat_add(a2391, a2392)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2389
        };
        h2388
    }))
}

pub fn f521(v0: Bytes, v1: Bytes, p2405: u64, p2406: u64) -> R<bool> {
    let m2404 = (p2405, p2406);
    {
        let mut m2404 = m2404;
        loop {
            let step = {
                let (v2, v3) = m2404;
                let m2393 = v3;
                if m2393 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m2393 - 1;
                    let m2396 = {
                        let a2394 = v1.clone();
                        let a2395 = v2;
                        index_bytes(a2394, a2395)
                    };
                    match m2396 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m2399 = {
                                let a2397 = v0.clone();
                                let a2398 = v2;
                                index_bytes(a2397, a2398)
                            };
                            match m2399 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a2400 = v5;
                                    let a2401 = v6;
                                    if equal(a2400, a2401) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a2402 = v2;
                                            let a2403 = 1u64;
                                            nat_add(a2402, a2403)?
                                        }, v4))
                                    } else {
                                        Err::<(u64, u64), R<bool>>(Ok(false))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f523(v0: List<Adt13>) -> List<Bytes> {
    let m2407 = v0;
    match m2407.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2408 = f79(v1, f8(Str::lit("std")));
            match m2408 {
                None => {
                    f523(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f523(v2))
                }
            }
        }
    }
}

pub fn f524(v0: List<Adt13>) -> List<Bytes> {
    let m2409 = v0;
    match m2409.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2410 = f78(f79(v1, f8(Str::lit("req"))));
            match m2410 {
                None => {
                    f524(v2)
                }
                Some(v3) => {
                    List::cons(v3, f524(v2))
                }
            }
        }
    }
}

pub fn f525(v0: Adt18) -> List<(Bytes, Adt18)> {
    let m2411 = v0;
    match m2411 {
        Adt18::C0(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C1(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C2(_, _, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C3(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C4(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C5(r2412) => {
            let v4 = (*r2412).clone();
            f525(v4)
        }
        Adt18::C6 => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C7(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C8(v1, r2413, r2414) => {
            let v2 = (*r2413).clone();
            let v3 = (*r2414).clone();
            List::cons((v1, v2), f525(v3))
        }
    }
}

pub fn f526(p2419: List<(Bytes, Adt18)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt18)>> {
    let m2418 = p2419;
    {
        let mut m2418 = m2418;
        loop {
            let step = {
                let v0 = m2418;
                let m2415 = v0;
                match m2415.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(Ok(None::<(Bytes, Adt18)>))
                    }
                    Some((v3, v4)) => {
                        if f586(f587({
                            let (_, h2416) = v3.clone();
                            h2416
                        }, List::<Bytes>::nil())?, f528({
                            let (h2417, _) = v3.clone();
                            h2417
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(v0: List<(Bytes, Adt18)>, v1: Bytes) -> List<(Bytes, Adt18)> {
    let m2420 = v0;
    match m2420.uncons() {
        None => {
            List::<(Bytes, Adt18)>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h2421, _) = v2.clone();
                h2421
            }, v1.clone()) {
                f527(v3, v1)
            } else {
                List::cons(v2, f527(v3, v1))
            }
        }
    }
}

pub fn f528(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f329(v0)?;
    if f41(v1.clone(), f588()) {
        f589(v1)
    } else {
        Ok(v1)
    }
}

pub fn f529() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f530() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f531(v0: Bytes, v1: Adt18) -> R<Result<Adt35, Bytes>> {
    f590(v0, f591(v1))
}

pub fn f532(v0: Adt35) -> List<Adt28> {
    let m2422 = v0;
    match m2422 {
        Adt35::C0(_, v2) => {
            v2
        }
    }
}

pub fn f533(v0: Adt35) -> Bytes {
    let m2423 = v0;
    match m2423 {
        Adt35::C0(v1, _) => {
            v1
        }
    }
}

pub fn f534(v0: Fn18, p2427: u64, p2428: Adt34) -> R<Adt34> {
    let m2426 = (p2427, p2428);
    {
        let mut m2426 = m2426;
        loop {
            let step = {
                let (v1, v2) = m2426;
                let m2424 = v1;
                if m2424 == 0 {
                    Err::<(u64, Adt34), R<Adt34>>(Ok(v2))
                } else {
                    let v3 = m2424 - 1;
                    Ok::<(u64, Adt34), R<Adt34>>((v3, {
                        let c2425: Fn18 = v0.clone();
                        c2425.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f535(v0: Adt34) -> R<Adt34> {
    f592(v0)
}

pub fn f536(v0: u32, v1: u32) -> u32 {
    let m2431 = {
        let a2429 = f593(v0);
        let a2430 = f593(v1);
        fixed_u64::checked_add(a2429, a2430)
    };
    match m2431 {
        None => {
            0u32
        }
        Some(v2) => {
            f594(v2)
        }
    }
}

pub fn f537(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2434 = {
        let a2432 = v3;
        let a2433 = 8u64;
        nat_add(a2432, a2433)?
    };
    let a2435 = v2;
    if nat_eq(a2434, a2435) {
        Ok(f595({
            let a2438 = {
                let a2436 = v1;
                let a2437 = 8u64;
                nat_mul(a2436, a2437)?
            };
            let a2439 = 4294967296u64;
            let a2440 = 0u64;
            nat_quot(a2438, a2439, a2440)
        }))
    } else {
        let a2443 = {
            let a2441 = v3;
            let a2442 = 4u64;
            nat_add(a2441, a2442)?
        };
        let a2444 = v2;
        if nat_eq(a2443, a2444) {
            Ok(f595({
                let a2447 = {
                    let a2445 = v1;
                    let a2446 = 8u64;
                    nat_mul(a2445, a2446)?
                };
                let a2448 = 4294967296u64;
                let a2449 = 0u64;
                nat_rem(a2447, a2448, a2449)
            }))
        } else {
            Ok(f596(f597(v0.clone(), v1, v3), f597(v0.clone(), v1, {
                let a2450 = v3;
                let a2451 = 1u64;
                nat_add(a2450, a2451)?
            }), f597(v0.clone(), v1, {
                let a2452 = v3;
                let a2453 = 2u64;
                nat_add(a2452, a2453)?
            }), f597(v0, v1, {
                let a2454 = v3;
                let a2455 = 3u64;
                nat_add(a2454, a2455)?
            })))
        }
    }
}

pub fn f538(v0: List<Adt30>, v1: List<Adt30>) -> List<Adt30> {
    let m2456 = v0;
    match m2456.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f538(v3, v1))
        }
    }
}

pub fn f539(v0: List<Adt31>, v1: Bytes) -> List<Adt30> {
    let m2457 = v0;
    match m2457.uncons() {
        None => {
            List::<Adt30>::nil()
        }
        Some((v2, v3)) => {
            if f41(match v2.clone() {
                Adt31::C0(_, h2458, _, _, _) => {
                    h2458
                }
            }, f598()) {
                let m2459 = f498(v2.clone(), Str::lit("name"));
                match m2459 {
                    None => {
                        f539(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt30::C0(match v2.clone() {
                            Adt31::C0(_, h2460, _, _, _) => {
                                h2460
                            }
                        }, v1.clone(), v4, v2), f539(v3, v1))
                    }
                }
            } else {
                f539(v3, v1)
            }
        }
    }
}

pub fn f540(v0: u64, v1: Adt29, v2: Adt30) -> R<Result<List<Adt32>, Bytes>> {
    let v3: Bytes = f329(match v2.clone() {
        Adt30::C0(_, _, h2461, _) => {
            h2461
        }
    })?;
    let v4: Adt31 = match v2.clone() {
        Adt30::C0(_, _, _, h2462) => {
            h2462
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt30::C0(_, h2463, _, _) => {
            h2463
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt30::C0(h2464, _, _, _) => {
            h2464
        }
    };
    if f28(v6.clone(), f8(Str::lit("complexType"))) {
        f599(v0, v1, v2)
    } else {
        if f28(v6.clone(), f8(Str::lit("simpleType"))) {
            let m2465 = f600(v3, v4, v5)?;
            match m2465 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt32>, Bytes>(List::cons(v8, List::<Adt32>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt32>, Bytes>(v7))
                }
            }
        } else {
            if f28(v6.clone(), f8(Str::lit("element"))) {
                let m2466 = f498(v4.clone(), Str::lit("type"));
                match m2466 {
                    None => {
                        let m2467 = f601(v0, v1, v3, v5, v4)?;
                        match m2467 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt32>, Bytes>(match v11 {
                                    Adt36::C0(_, h2468) => {
                                        h2468
                                    }
                                }))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt32>, Bytes>(v10))
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Ok::<List<Adt32>, Bytes>(List::<Adt32>::nil()))
                    }
                }
            } else {
                if f28(v6.clone(), f8(Str::lit("attribute"))) {
                    let m2469 = f498(v4.clone(), Str::lit("type"));
                    match m2469 {
                        None => {
                            let m2470 = f602(v1, v5, v3, v4)?;
                            match m2470 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt32>, Bytes>(match v14 {
                                        Adt36::C0(_, h2471) => {
                                            h2471
                                        }
                                    }))
                                }
                                Err(v13) => {
                                    Ok(Err::<List<Adt32>, Bytes>(v13))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<List<Adt32>, Bytes>(List::<Adt32>::nil()))
                        }
                    }
                } else {
                    if f28(v6, f8(Str::lit("group"))) {
                        let m2472 = f603(v4)?;
                        match m2472 {
                            Ok(v16) => {
                                let m2473 = f604(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m2473 {
                                    None => {
                                        Ok(Err::<List<Adt32>, Bytes>(f605(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f606(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt32>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m2474 = f603(v4)?;
                        match m2474 {
                            Ok(v19) => {
                                let m2475 = f607(v0, v1, v3, v5, v19, f608())?;
                                match m2475 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt32>, Bytes>(match v21 {
                                            Adt38::C0(_, h2476, _, _, _) => {
                                                h2476
                                            }
                                        }))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<List<Adt32>, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<List<Adt32>, Bytes>(v18))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f541(v0: List<Adt32>, v1: List<Adt32>) -> List<Adt32> {
    let m2477 = v0;
    match m2477.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f541(v3, v1))
        }
    }
}

pub fn f542(p2480: List<Adt32>, p2481: List<Adt32>) -> List<Adt32> {
    let m2479 = (p2480, p2481);
    {
        let mut m2479 = m2479;
        loop {
            let step = {
                let (v0, v1) = m2479;
                let m2478 = v0;
                match m2478.uncons() {
                    None => {
                        Err::<(List<Adt32>, List<Adt32>), List<Adt32>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt32>, List<Adt32>), List<Adt32>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f543(p2485: List<Adt32>, v1: Bytes) -> Option<Adt32> {
    let m2484 = p2485;
    {
        let mut m2484 = m2484;
        loop {
            let step = {
                let v0 = m2484;
                let m2482 = v0;
                match m2482.uncons() {
                    None => {
                        Err::<List<Adt32>, Option<Adt32>>(None::<Adt32>)
                    }
                    Some((v2, v3)) => {
                        if f28(match v2.clone() {
                            Adt32::C0(h2483, _, _, _) => {
                                h2483
                            }
                        }, v1.clone()) {
                            Err::<List<Adt32>, Option<Adt32>>(Some(v2))
                        } else {
                            Ok::<List<Adt32>, Option<Adt32>>(v3)
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

pub fn f544() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f545(v0: List<Adt32>) -> List<Bytes> {
    let m2486 = v0;
    match m2486.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f553(match v1 {
                Adt32::C0(_, h2487, _, _) => {
                    h2487
                }
            }, f545(v2))
        }
    }
}

pub fn f546(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f342(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f547() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f548(p2490: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m2489 = p2490;
    {
        let mut m2489 = m2489;
        loop {
            let step = {
                let v0 = m2489;
                let m2488 = v0;
                match m2488.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f41(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2489 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549(p2494: u64, v1: List<Adt32>, p2495: List<Bytes>, p2496: List<Bytes>) -> List<Bytes> {
    let m2493 = (p2494, (p2495, p2496));
    {
        let mut m2493 = m2493;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2493;
                let m2491 = v0;
                if m2491 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2491 - 1;
                    let m2492 = v2;
                    match m2492.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f41(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f553(f550(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: List<Adt32>, v1: Bytes) -> List<Bytes> {
    let m2497 = f543(v0, v1);
    match m2497 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt32::C0(_, h2498, _, _) => {
                    h2498
                }
            }
        }
    }
}

pub fn f551(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2499 = v0;
    match m2499.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2501 = f548({
                let (_, h2500) = v2.clone();
                h2500
            }, v1.clone());
            match m2501 {
                None => {
                    Some(({
                        let (h2502, _) = v2;
                        h2502
                    }, v3))
                }
                Some(_) => {
                    let m2503 = f551(v3, v1);
                    match m2503 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2504, _) = v5;
                                h2504
                            }, List::cons(v2, {
                                let (_, h2505) = v5;
                                h2505
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f552(v0: List<Adt32>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2506 = v1.clone();
    match m2506.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2507 = v3;
            match m2507.uncons() {
                None => {
                    let m2510 = f543(v0, v2.clone());
                    match m2510 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f610(v2))
                        }
                        Some(v6) => {
                            if f41(v2.clone(), match v6.clone() {
                                Adt32::C0(_, h2511, _, _) => {
                                    h2511
                                }
                            }) {
                                let m2513 = match v6.clone() {
                                    Adt32::C0(_, _, _, h2512) => {
                                        h2512
                                    }
                                };
                                match m2513.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f610(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f546(v2, match v6 {
                                            Adt32::C0(_, _, _, h2514) => {
                                                h2514
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt32::C0(_, _, h2515, _) => {
                                        h2515
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f609(v0, v1, {
                        let a2508 = v2;
                        let a2509 = f8(Str::lit("Group"));
                        append_bytes(a2508, a2509)
                    })
                }
            }
        }
    }
}

pub fn f553(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2516 = v0;
    match m2516.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f553(v3, v1))
        }
    }
}

pub fn f554(v0: List<Adt32>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2517 = v1;
    match m2517.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f553(f611(f550(v0.clone(), v3), v2.clone()), f554(v0, v4, v2))
        }
    }
}

pub fn f555(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2518 = v1;
    match m2518.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f28(v3.clone(), v2.clone()) {
                true
            } else {
                if f41(v3.clone(), f612(v0.clone(), v2.clone())) {
                    f41(v2.clone(), f612(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f555(v0, v4, v2))
            } else {
                f555(v0, v4, v2)
            }
        }
    }
}

pub fn f556(v0: Bytes) -> R<Bytes> {
    let (_, h2519) = {
        let (h2520, _) = f358(Fn4::F613(v0.clone()), {
            let a2522 = {
                let a2521 = v0;
                length_bytes(a2521)
            };
            let a2523 = 1u64;
            nat_add(a2522, a2523)?
        }, (0u64, Bytes::lit(&[])))?;
        h2520
    };
    Ok(h2519)
}

pub fn f557(v0: Bytes) -> R<u64> {
    let (h2524, _) = f287(Fn3::F614(v0.clone()), {
        let a2526 = {
            let a2525 = v0;
            length_bytes(a2525)
        };
        let a2527 = 1u64;
        nat_add(a2526, a2527)?
    }, 0u64)?;
    Ok(h2524)
}

pub fn f558(v0: Bytes) -> Result<Adt26, Bytes> {
    Err::<Adt26, Bytes>(v0)
}

pub fn f559(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2528 = v0;
        utf8_encode(a2528)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f357(v1)?, List::<Bytes>::nil())))))
}

pub fn f560(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f562(v0.clone(), 0u64, Str::lit("<?xml")) {
        f615(f563(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m2530 = f571(v0.clone(), {
            let a2529 = Str::lit("?>");
            utf8_encode(a2529)
        }, 5u64)?;
        match m2530 {
            None => {
                Ok(f616(f559(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m2532 = f571(f35(v0.clone(), 0u64, v1), {
                    let a2531 = Str::lit("encoding");
                    utf8_encode(a2531)
                }, 5u64)?;
                match m2532 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a2533 = v1;
                            let a2534 = 2u64;
                            nat_add(a2533, a2534)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f617(v0.clone(), {
                            let a2535 = v2;
                            let a2536 = 8u64;
                            nat_add(a2535, a2536)?
                        })?;
                        let v4: u64 = f617(v0.clone(), {
                            let a2537 = v3;
                            let a2538 = 1u64;
                            nat_add(a2537, a2538)?
                        })?;
                        let _: u64 = f563(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m2541 = f571(v0.clone(), f35(v0.clone(), v4, 1u64), {
                                let a2539 = v4;
                                let a2540 = 1u64;
                                nat_add(a2539, a2540)?
                            })?;
                            match m2541 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f618(f35(v0, {
                            let a2542 = v4;
                            let a2543 = 1u64;
                            nat_add(a2542, a2543)?
                        }, {
                            let a2546 = v7;
                            let a2547 = {
                                let a2544 = v4;
                                let a2545 = 1u64;
                                nat_add(a2544, a2545)?
                            };
                            nat_sub(a2546, a2547)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a2548 = v1;
                                let a2549 = 2u64;
                                nat_add(a2548, a2549)?
                            }))
                        } else {
                            Ok(f616(f559(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f561(v0: Bytes, p2554: u64, p2555: u64) -> R<Result<u64, Bytes>> {
    let m2553 = (p2554, p2555);
    {
        let mut m2553 = m2553;
        loop {
            let step = {
                let (v1, v2) = m2553;
                let m2550 = v2;
                if m2550 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f616(f559(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m2550 - 1;
                    let v4: u64 = f617(v0.clone(), v1)?;
                    if f562(v0.clone(), v4, Str::lit("<!--")) {
                        let m2551 = f619(v0.clone(), v4)?;
                        match m2551 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f616(v5)))
                            }
                        }
                    } else {
                        if f562(v0.clone(), v4, Str::lit("<?")) {
                            let m2552 = f620(v0.clone(), v4)?;
                            match m2552 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f616(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2553 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f562(v0: Bytes, v1: u64, v2: Str) -> bool {
    f289(v0, {
        let a2556 = v2;
        utf8_encode(a2556)
    }, v1)
}

pub fn f563(v0: Bytes, v1: u64) -> R<u64> {
    f351(v0, v1)
}

pub fn f564(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt26, u64), Bytes>> {
    let m2557 = v2;
    if m2557 == 0 {
        Ok(f621(f559(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m2557 - 1;
        let a2558 = v3;
        let a2559 = 0u64;
        if nat_eq(a2558, a2559) {
            let v5: u64 = f622(v0.clone(), {
                let a2560 = v1;
                let a2561 = 1u64;
                nat_add(a2560, a2561)?
            })?;
            let a2564 = v5;
            let a2565 = {
                let a2562 = v1;
                let a2563 = 1u64;
                nat_add(a2562, a2563)?
            };
            if nat_eq(a2564, a2565) {
                Ok(f621(f559(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f35(v0.clone(), {
                    let a2566 = v1;
                    let a2567 = 1u64;
                    nat_add(a2566, a2567)?
                }, {
                    let a2570 = v5;
                    let a2571 = {
                        let a2568 = v1;
                        let a2569 = 1u64;
                        nat_add(a2568, a2569)?
                    };
                    nat_sub(a2570, a2571)
                });
                let m2572 = f623(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m2572 {
                    Ok(v8) => {
                        if f562(v0.clone(), {
                            let (_, h2573) = v8.clone();
                            h2573
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt26, u64), Bytes>((Adt26::C0(v6, std::rc::Rc::new({
                                let (h2574, _) = v8;
                                h2574
                            }), std::rc::Rc::new(Adt26::C4)), {
                                let a2576 = {
                                    let (_, h2575) = v8;
                                    h2575
                                };
                                let a2577 = 2u64;
                                nat_add(a2576, a2577)?
                            })))
                        } else {
                            let m2581 = f564(v0.clone(), {
                                let a2579 = {
                                    let (_, h2578) = v8;
                                    h2578
                                };
                                let a2580 = 1u64;
                                nat_add(a2579, a2580)?
                            }, v4, 1u64)?;
                            match m2581 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a2583 = {
                                            let (_, h2582) = v10.clone();
                                            h2582
                                        };
                                        let a2584 = 2u64;
                                        nat_add(a2583, a2584)?
                                    };
                                    let v12: u64 = f622(v0.clone(), v11)?;
                                    if f28(f35(v0.clone(), v11, {
                                        let a2585 = v12;
                                        let a2586 = v11;
                                        nat_sub(a2585, a2586)
                                    }), v6.clone()) {
                                        let v13: u64 = f617(v0.clone(), v12)?;
                                        let a2587 = f563(v0, v13)?;
                                        let a2588 = 62u64;
                                        if nat_eq(a2587, a2588) {
                                            Ok(Ok::<(Adt26, u64), Bytes>((Adt26::C0(v6, std::rc::Rc::new({
                                                let (h2589, _) = v8;
                                                h2589
                                            }), std::rc::Rc::new({
                                                let (h2590, _) = v10;
                                                h2590
                                            })), {
                                                let a2591 = v13;
                                                let a2592 = 1u64;
                                                nat_add(a2591, a2592)?
                                            })))
                                        } else {
                                            Ok(f621(f559(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f621(f559(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h2593) = v10;
                                            h2593
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f621(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f621(v7))
                    }
                }
            }
        } else {
            let a2594 = f563(v0.clone(), v1)?;
            let a2595 = 256u64;
            if nat_eq(a2594, a2595) {
                Ok(f621(f559(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f562(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt26, u64), Bytes>((Adt26::C4, v1)))
                } else {
                    if f562(v0.clone(), v1, Str::lit("<!--")) {
                        let m2596 = f619(v0.clone(), v1)?;
                        match m2596 {
                            Ok(v15) => {
                                f564(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f621(v14))
                            }
                        }
                    } else {
                        if f562(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m2600 = f571(v0.clone(), {
                                let a2597 = Str::lit("]]>");
                                utf8_encode(a2597)
                            }, {
                                let a2598 = v1;
                                let a2599 = 9u64;
                                nat_add(a2598, a2599)?
                            })?;
                            match m2600 {
                                None => {
                                    Ok(f621(f559(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m2603 = f564(v0.clone(), {
                                        let a2601 = v16;
                                        let a2602 = 3u64;
                                        nat_add(a2601, a2602)?
                                    }, v4, 1u64)?;
                                    match m2603 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt26, u64), Bytes>((Adt26::C3(std::rc::Rc::new(Adt26::C2(f35(v0, {
                                                let a2604 = v1;
                                                let a2605 = 9u64;
                                                nat_add(a2604, a2605)?
                                            }, {
                                                let a2608 = v16;
                                                let a2609 = {
                                                    let a2606 = v1;
                                                    let a2607 = 9u64;
                                                    nat_add(a2606, a2607)?
                                                };
                                                nat_sub(a2608, a2609)
                                            }))), std::rc::Rc::new({
                                                let (h2610, _) = v18;
                                                h2610
                                            })), {
                                                let (_, h2611) = v18;
                                                h2611
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f621(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f562(v0.clone(), v1, Str::lit("<?")) {
                                let m2612 = f620(v0.clone(), v1)?;
                                match m2612 {
                                    Ok(v20) => {
                                        f564(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f621(v19))
                                    }
                                }
                            } else {
                                if f562(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f621(f559(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a2613 = f563(v0.clone(), v1)?;
                                    let a2614 = 60u64;
                                    if nat_eq(a2613, a2614) {
                                        let m2615 = f564(v0.clone(), v1, v4, 0u64)?;
                                        match m2615 {
                                            Ok(v22) => {
                                                let m2617 = f564(v0, {
                                                    let (_, h2616) = v22;
                                                    h2616
                                                }, v4, 1u64)?;
                                                match m2617 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt26, u64), Bytes>((Adt26::C3(std::rc::Rc::new({
                                                            let (h2618, _) = v22;
                                                            h2618
                                                        }), std::rc::Rc::new({
                                                            let (h2619, _) = v24;
                                                            h2619
                                                        })), {
                                                            let (_, h2620) = v24;
                                                            h2620
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f621(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f621(v21))
                                            }
                                        }
                                    } else {
                                        let m2621 = f624(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m2621 {
                                            Ok(v26) => {
                                                let m2623 = f564(v0, {
                                                    let (_, h2622) = v26;
                                                    h2622
                                                }, v4, 1u64)?;
                                                match m2623 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt26, u64), Bytes>((Adt26::C3(std::rc::Rc::new(Adt26::C2({
                                                            let (h2624, _) = v26;
                                                            h2624
                                                        })), std::rc::Rc::new({
                                                            let (h2625, _) = v28;
                                                            h2625
                                                        })), {
                                                            let (_, h2626) = v28;
                                                            h2626
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f621(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f621(v25))
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

pub fn f565(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f566() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f567() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f568(v0: Bytes) -> R<Bytes> {
    let m2627 = f571(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m2627 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f35(v0, 0u64, v1))
        }
    }
}

pub fn f569(p2632: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m2631 = p2632;
    {
        let mut m2631 = m2631;
        loop {
            let step = {
                let v0 = m2631;
                let m2628 = v0;
                match m2628.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h2629, _) = v2;
                            h2629
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h2630) = v2;
                                h2630
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f570(v0: Bytes) -> R<Bytes> {
    let m2633 = f571(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m2633 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f35(v0.clone(), {
                let a2634 = v1;
                let a2635 = 1u64;
                nat_add(a2634, a2635)?
            }, {
                let a2639 = {
                    let a2636 = v0;
                    length_bytes(a2636)
                };
                let a2640 = {
                    let a2637 = v1;
                    let a2638 = 1u64;
                    nat_add(a2637, a2638)?
                };
                nat_sub(a2639, a2640)
            }))
        }
    }
}

pub fn f571(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h2641, _) = f287(Fn3::F625(v1.clone(), v0.clone()), {
            let a2643 = {
                let a2642 = v0.clone();
                length_bytes(a2642)
            };
            let a2644 = 1u64;
            nat_add(a2643, a2644)?
        }, v2)?;
        h2641
    };
    if f289(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f572(v0: Bytes, p2653: u64, p2654: u64, p2655: Option<u64>) -> R<Option<u64>> {
    let m2652 = (p2653, (p2654, p2655));
    {
        let mut m2652 = m2652;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2652;
                let m2645 = v2;
                if m2645 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2645 - 1;
                    let v5: u64 = f563(v0.clone(), v1)?;
                    let a2646 = v5;
                    let a2647 = 256u64;
                    if nat_eq(a2646, a2647) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2648 = v1;
                            let a2649 = 1u64;
                            nat_add(a2648, a2649)?
                        }, (v4, {
                            let a2650 = v5;
                            let a2651 = 47u64;
                            if nat_eq(a2650, a2651) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2656, _) = v1.clone();
        h2656
    };
    let v3: Bytes = {
        let (h2657, _) = {
            let (_, h2658) = v1.clone();
            h2658
        };
        h2657
    };
    let a2661 = {
        let (_, h2659) = {
            let (_, h2660) = v1;
            h2660
        };
        h2659
    };
    let a2662 = 0u64;
    if nat_eq(a2661, a2662) {
        let v4: u64 = f351(v0.clone(), v2)?;
        let a2663 = v4;
        let a2664 = 34u64;
        if nat_eq(a2663, a2664) {
            Ok(Some(({
                let a2665 = v2;
                let a2666 = 1u64;
                nat_add(a2665, a2666)?
            }, (v3, 1u64))))
        } else {
            let a2667 = v4;
            let a2668 = 92u64;
            if nat_eq(a2667, a2668) {
                f626(v0, v2, v3)
            } else {
                let a2669 = v4;
                let a2670 = 32u64;
                if if nat_lt(a2669, a2670) {
                    true
                } else {
                    let a2671 = v4;
                    let a2672 = 256u64;
                    nat_eq(a2671, a2672)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2673 = v2;
                        let a2674 = 1u64;
                        nat_add(a2673, a2674)?
                    }, ({
                        let a2675 = v3;
                        let a2676 = f35(v0, v2, 1u64);
                        append_bytes(a2675, a2676)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f574(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f351(v0, v1)?;
    let a2677 = 47u64;
    let a2678 = v2;
    if if nat_lt(a2677, a2678) {
        let a2679 = v2;
        let a2680 = 58u64;
        nat_lt(a2679, a2680)
    } else {
        false
    } {
        Ok(Some({
            let a2681 = v1;
            let a2682 = 1u64;
            nat_add(a2681, a2682)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f575(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2683 = f351(v0, v1)?;
    let a2684 = 48u64;
    if nat_eq(a2683, a2684) {
        Ok(Some({
            let a2685 = v1;
            let a2686 = 1u64;
            nat_add(a2685, a2686)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f576(v0: Bytes) -> R<u64> {
    let (h2687, _) = f287(Fn3::F627(v0.clone()), {
        let a2688 = v0;
        length_bytes(a2688)
    }, 0u64)?;
    Ok(h2687)
}

pub fn f577(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2692 = {
        let a2690 = v0;
        let a2691 = {
            let (h2689, _) = v1;
            h2689
        };
        index_bytes(a2690, a2691)
    };
    match m2692 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2694 = {
                    let (h2693, _) = v1;
                    h2693
                };
                let a2695 = 1u64;
                nat_add(a2694, a2695)?
            }, {
                let a2699 = {
                    let a2697 = {
                        let (_, h2696) = v1;
                        h2696
                    };
                    let a2698 = 10u64;
                    nat_mul(a2697, a2698)?
                };
                let a2700 = f628(v2)?;
                nat_add(a2699, a2700)?
            })))
        }
    }
}

pub fn f578(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f629(v0, v1)
}

pub fn f579(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2701, _) = v2;
        h2701
    };
    let a2702 = v3;
    let a2703 = 0u64;
    if nat_eq(a2702, a2703) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2704 = v3;
            let a2705 = v1;
            let a2706 = 0u64;
            nat_quot(a2704, a2705, a2706)
        }, {
            let a2711 = f511(v0, {
                let a2707 = v3;
                let a2708 = v1;
                let a2709 = 0u64;
                nat_rem(a2707, a2708, a2709)
            });
            let a2712 = {
                let (_, h2710) = v2;
                h2710
            };
            append_bytes(a2711, a2712)
        }))
    }
}

pub fn f580(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2757 = {
            let a2733 = {
                let a2721 = {
                    let a2715 = {
                        let a2713 = v1;
                        let a2714 = 128u8;
                        fixed_u8::bit_and(a2713, a2714)
                    };
                    let a2716 = 128u8;
                    if equal(a2715, a2716) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2722 = {
                    let a2719 = {
                        let a2717 = v1;
                        let a2718 = 64u8;
                        fixed_u8::bit_and(a2717, a2718)
                    };
                    let a2720 = 64u8;
                    if equal(a2719, a2720) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2721, a2722)?
            };
            let a2734 = {
                let a2731 = {
                    let a2725 = {
                        let a2723 = v1;
                        let a2724 = 32u8;
                        fixed_u8::bit_and(a2723, a2724)
                    };
                    let a2726 = 32u8;
                    if equal(a2725, a2726) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2732 = {
                    let a2729 = {
                        let a2727 = v1;
                        let a2728 = 16u8;
                        fixed_u8::bit_and(a2727, a2728)
                    };
                    let a2730 = 16u8;
                    if equal(a2729, a2730) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2731, a2732)?
            };
            nat_add(a2733, a2734)?
        };
        let a2758 = {
            let a2755 = {
                let a2743 = {
                    let a2737 = {
                        let a2735 = v1;
                        let a2736 = 8u8;
                        fixed_u8::bit_and(a2735, a2736)
                    };
                    let a2738 = 8u8;
                    if equal(a2737, a2738) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2744 = {
                    let a2741 = {
                        let a2739 = v1;
                        let a2740 = 4u8;
                        fixed_u8::bit_and(a2739, a2740)
                    };
                    let a2742 = 4u8;
                    if equal(a2741, a2742) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2743, a2744)?
            };
            let a2756 = {
                let a2753 = {
                    let a2747 = {
                        let a2745 = v1;
                        let a2746 = 2u8;
                        fixed_u8::bit_and(a2745, a2746)
                    };
                    let a2748 = 2u8;
                    if equal(a2747, a2748) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2754 = {
                    let a2751 = {
                        let a2749 = v1;
                        let a2750 = 1u8;
                        fixed_u8::bit_and(a2749, a2750)
                    };
                    let a2752 = 1u8;
                    if equal(a2751, a2752) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2753, a2754)?
            };
            nat_add(a2755, a2756)?
        };
        nat_add(a2757, a2758)?
    };
    let a2765 = f511(f630(), {
        let a2759 = v2;
        let a2760 = 16u64;
        let a2761 = 0u64;
        nat_quot(a2759, a2760, a2761)
    });
    let a2766 = f511(f630(), {
        let a2762 = v2;
        let a2763 = 16u64;
        let a2764 = 0u64;
        nat_rem(a2762, a2763, a2764)
    });
    Ok(append_bytes(a2765, a2766))
}

pub fn f581(v0: Fn19, p2770: List<List<(Bytes, Bytes)>>, p2771: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2769 = (p2770, p2771);
    {
        let mut m2769 = m2769;
        loop {
            let step = {
                let (v1, v2) = m2769;
                let m2767 = v2;
                match m2767.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2768: Fn19 = v0.clone();
                            c2768.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f583(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2773 = {
        let a2772 = v0.clone();
        length_list(a2772)
    };
    let a2774 = 1u64;
    if nat_le(a2773, a2774) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f631(v0)?))
    }
}

pub fn f584(v0: Fn20, p2779: u64, p2780: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2778 = (p2779, p2780);
    {
        let mut m2778 = m2778;
        loop {
            let step = {
                let (v1, v2) = m2778;
                let m2775 = v1;
                if m2775 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2775 - 1;
                    let m2777 = {
                        let c2776: Fn20 = v0.clone();
                        c2776.apply(v2.clone())?
                    };
                    match m2777 {
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
                Ok(next) => { m2778 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f585(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f632(v0)
}

pub fn f586(p2783: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2782 = p2783;
    {
        let mut m2782 = m2782;
        loop {
            let step = {
                let v0 = m2782;
                let m2781 = v0;
                match m2781.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f28(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f41(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f41(v4, v3.clone())
                            }
                        } {
                            Ok::<List<Bytes>, bool>(v5)
                        } else {
                            Err::<List<Bytes>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(v0: Adt18, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2784 = v0;
    match m2784 {
        Adt18::C0(_) => {
            Ok(v1)
        }
        Adt18::C1(_, _) => {
            Ok(v1)
        }
        Adt18::C2(_, _, _) => {
            Ok(v1)
        }
        Adt18::C3(_) => {
            Ok(v1)
        }
        Adt18::C4(r2787) => {
            let v2 = (*r2787).clone();
            f587(v2, v1)
        }
        Adt18::C5(r2788) => {
            let v3 = (*r2788).clone();
            f587(v3, v1)
        }
        Adt18::C6 => {
            Ok(v1)
        }
        Adt18::C7(r2789, r2790) => {
            let v4 = (*r2789).clone();
            let v5 = (*r2790).clone();
            f587(v4, f587(v5, v1)?)
        }
        Adt18::C8(v6, r2791, r2792) => {
            let v7 = (*r2791).clone();
            let v8 = (*r2792).clone();
            let v9: List<Bytes> = f587(v7.clone(), f587(v8, v1)?)?;
            if f28(v6, f8(Str::lit("$ref"))) {
                let m2785 = f334(Some(v7));
                match m2785 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2786 = f633(v10)?;
                        match m2786 {
                            None => {
                                Ok(v9)
                            }
                            Some(v11) => {
                                Ok(List::cons(v11, v9))
                            }
                        }
                    }
                }
            } else {
                Ok(v9)
            }
        }
    }
}

pub fn f588() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f589(v0: Bytes) -> R<Bytes> {
    let m2795 = {
        let a2793 = v0.clone();
        let a2794 = 0u64;
        index_bytes(a2793, a2794)
    };
    match m2795 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2801 = {
                let a2796 = f8(Str::lit("u"));
                let a2797 = f634(8u64, f62(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2796, a2797)
            };
            let a2802 = f35(v0.clone(), 1u64, {
                let a2799 = {
                    let a2798 = v0;
                    length_bytes(a2798)
                };
                let a2800 = 1u64;
                nat_sub(a2799, a2800)
            });
            Ok(append_bytes(a2801, a2802))
        }
    }
}

pub fn f590(v0: Bytes, v1: Adt18) -> R<Result<Adt35, Bytes>> {
    let m2803 = f242(v1.clone(), f8(Str::lit("enum")));
    match m2803 {
        None => {
            let m2805 = f242(v1.clone(), f8(Str::lit("oneOf")));
            match m2805 {
                None => {
                    if f639(v1.clone(), v0.clone())? {
                        let m2806 = f640(v0.clone(), f463(f242(v1.clone(), f8(Str::lit("properties")))), f246(f247(f242(v1, f8(Str::lit("required"))))), true)?;
                        match m2806 {
                            Ok(v7) => {
                                Ok(f637(f546(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f342(f641(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f642(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt35, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2807 = f463(f242(v1.clone(), f8(Str::lit("properties"))));
                        match m2807.uncons() {
                            None => {
                                if f643(v1.clone()) {
                                    Ok(f637(f644(v0), List::<Adt28>::nil()))
                                } else {
                                    let m2810 = f645(v1, {
                                        let a2808 = v0.clone();
                                        let a2809 = f8(Str::lit("Value"));
                                        append_bytes(a2808, a2809)
                                    })?;
                                    match m2810 {
                                        Ok(v9) => {
                                            Ok(f637(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f533(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f532(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt35, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2811 = f640(v0.clone(), f463(f242(v1.clone(), f8(Str::lit("properties")))), f246(f247(f242(v1, f8(Str::lit("required"))))), false)?;
                                match m2811 {
                                    Ok(v13) => {
                                        Ok(f637(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2812 = f641(v13.clone());
                                            match m2812.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f342(f641(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f642(v13)))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<Adt35, Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f638(v0, f335(v5))
                }
            }
        }
        Some(v2) => {
            let m2804 = f635(f335(v2), List::<Bytes>::nil())?;
            match m2804 {
                Ok(v4) => {
                    Ok(f637(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f342(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt28>::nil()))
                }
                Err(v3) => {
                    Ok(f636(v3, v0))
                }
            }
        }
    }
}

pub fn f591(v0: Adt18) -> Adt18 {
    let v2: Adt18 = {
        let m2813 = f242(v0.clone(), f8(Str::lit("anyOf")));
        match m2813 {
            None => {
                v0
            }
            Some(v1) => {
                if f646(f335(v1.clone())) {
                    f647(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f648(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f649(f335(v1.clone()))
                    } {
                        v0
                    } else {
                        f650(f647(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt18 = {
        let m2814 = f242(v2.clone(), f8(Str::lit("oneOf")));
        match m2814 {
            None => {
                v2
            }
            Some(v3) => {
                if f646(f335(v3)) {
                    f647(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2815 = f242(v4.clone(), f8(Str::lit("allOf")));
    match m2815 {
        None => {
            v4
        }
        Some(v5) => {
            if f651(f335(v5)) {
                f647(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f592(v0: Adt34) -> R<Adt34> {
    let v1: u32 = f652(match v0.clone() {
        Adt34::C0(_, _, _, _, _, _, _, _, _, h2816) => {
            h2816
        }
    });
    let v2: u32 = f536(match v0.clone() {
        Adt34::C0(_, _, _, _, _, _, _, h2817, _, _) => {
            h2817
        }
    }, f536(f653(match v0.clone() {
        Adt34::C0(_, _, _, _, h2818, _, _, _, _, _) => {
            h2818
        }
    }), f536(f654(match v0.clone() {
        Adt34::C0(_, _, _, _, h2819, _, _, _, _, _) => {
            h2819
        }
    }, match v0.clone() {
        Adt34::C0(_, _, _, _, _, h2820, _, _, _, _) => {
            h2820
        }
    }, match v0.clone() {
        Adt34::C0(_, _, _, _, _, _, h2821, _, _, _) => {
            h2821
        }
    }), f536(v1, {
        let m2824 = match v0.clone() {
            Adt34::C0(_, _, _, _, _, _, _, _, h2823, _) => {
                h2823
            }
        };
        match m2824 {
            Adt33::C0(h2822, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2822
            }
        }
    }))));
    let v3: u32 = f536(f655(match v0.clone() {
        Adt34::C0(h2825, _, _, _, _, _, _, _, _, _) => {
            h2825
        }
    }), f656(match v0.clone() {
        Adt34::C0(h2826, _, _, _, _, _, _, _, _, _) => {
            h2826
        }
    }, match v0.clone() {
        Adt34::C0(_, h2827, _, _, _, _, _, _, _, _) => {
            h2827
        }
    }, match v0.clone() {
        Adt34::C0(_, _, h2828, _, _, _, _, _, _, _) => {
            h2828
        }
    }));
    Ok(Adt34::C0(f536(v2, v3), match v0.clone() {
        Adt34::C0(h2829, _, _, _, _, _, _, _, _, _) => {
            h2829
        }
    }, match v0.clone() {
        Adt34::C0(_, h2830, _, _, _, _, _, _, _, _) => {
            h2830
        }
    }, match v0.clone() {
        Adt34::C0(_, _, h2831, _, _, _, _, _, _, _) => {
            h2831
        }
    }, f536(match v0.clone() {
        Adt34::C0(_, _, _, h2832, _, _, _, _, _, _) => {
            h2832
        }
    }, v2), match v0.clone() {
        Adt34::C0(_, _, _, _, h2833, _, _, _, _, _) => {
            h2833
        }
    }, match v0.clone() {
        Adt34::C0(_, _, _, _, _, h2834, _, _, _, _) => {
            h2834
        }
    }, match v0.clone() {
        Adt34::C0(_, _, _, _, _, _, h2835, _, _, _) => {
            h2835
        }
    }, f657(match v0.clone() {
        Adt34::C0(_, _, _, _, _, _, _, _, h2836, _) => {
            h2836
        }
    }), {
        let a2838 = match v0 {
            Adt34::C0(_, _, _, _, _, _, _, _, _, h2837) => {
                h2837
            }
        };
        let a2839 = 1u64;
        nat_add(a2838, a2839)?
    }))
}

pub fn f593(v0: u32) -> u64 {
    let m2841 = {
        let a2840 = v0;
        fixed_u64::convert(i128::from(a2840))
    };
    match m2841 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f594(v0: u64) -> u32 {
    let m2845 = {
        let a2844 = {
            let a2842 = v0;
            let a2843 = 4294967295u64;
            fixed_u64::bit_and(a2842, a2843)
        };
        fixed_u32::convert(i128::from(a2844))
    };
    match m2845 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f595(v0: u64) -> u32 {
    let (_, h2846) = {
        let (_, h2847) = f658(Fn21::F659, 32u64, (v0, (1u32, 0u32)));
        h2847
    };
    h2846
}

pub fn f596(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2852 = f660(v0, 24u32);
    let a2853 = {
        let a2850 = f660(v1, 16u32);
        let a2851 = {
            let a2848 = f660(v2, 8u32);
            let a2849 = v3;
            fixed_u32::bit_or(a2848, a2849)
        };
        fixed_u32::bit_or(a2850, a2851)
    };
    fixed_u32::bit_or(a2852, a2853)
}

pub fn f597(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2854 = v2;
    let a2855 = v1;
    if nat_lt(a2854, a2855) {
        let m2858 = {
            let a2856 = v0;
            let a2857 = v2;
            index_bytes(a2856, a2857)
        };
        match m2858 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2860 = {
                    let a2859 = v3;
                    fixed_u32::convert(i128::from(a2859))
                };
                match m2860 {
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
        let a2861 = v2;
        let a2862 = v1;
        if nat_eq(a2861, a2862) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f598() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f599(v0: u64, v1: Adt29, v2: Adt30) -> R<Result<List<Adt32>, Bytes>> {
    let v3: Bytes = f329(match v2.clone() {
        Adt30::C0(_, _, h2863, _) => {
            h2863
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt30::C0(_, h2864, _, _) => {
            h2864
        }
    }, match v2.clone() {
        Adt30::C0(_, _, h2865, _) => {
            h2865
        }
    });
    let v5: bool = f661(match v2.clone() {
        Adt30::C0(_, _, _, h2866) => {
            h2866
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m2868 = f662(match v1.clone() {
        Adt29::C0(h2867, _) => {
            h2867
        }
    }, v4, f496(v5))?;
    match m2868 {
        Ok(v7) => {
            if v5 {
                let m2869 = v7.clone();
                match m2869.uncons() {
                    None => {
                        Ok(Err::<List<Adt32>, Bytes>(f605(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m2870 = f663(v3, v7.clone(), f664(v7.clone())?, f664(v7)?)?;
                        match m2870 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt32>, Bytes>(List::cons(v11, List::<Adt32>::nil())))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt32>, Bytes>(v10))
                            }
                        }
                    }
                }
            } else {
                let m2871 = v7.clone();
                match m2871.uncons() {
                    None => {
                        f665(v0, v1, v3, match v2.clone() {
                            Adt30::C0(_, h2872, _, _) => {
                                h2872
                            }
                        }, match v2 {
                            Adt30::C0(_, _, _, h2873) => {
                                h2873
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f666(v3.clone());
                        let m2876 = f665(v0, v1, v14.clone(), match v2.clone() {
                            Adt30::C0(_, h2874, _, _) => {
                                h2874
                            }
                        }, match v2.clone() {
                            Adt30::C0(_, _, _, h2875) => {
                                h2875
                            }
                        })?;
                        match m2876 {
                            Ok(v16) => {
                                let m2878 = f663(v3, List::cons(match v2 {
                                    Adt30::C0(_, _, h2877, _) => {
                                        h2877
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f664(v7.clone())?), List::cons(v14, f664(v7)?))?;
                                match m2878 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt32>, Bytes>(f541(v16, List::cons(v18, List::<Adt32>::nil()))))
                                    }
                                    Err(v17) => {
                                        Ok(Err::<List<Adt32>, Bytes>(v17))
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt32>, Bytes>(v15))
                            }
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt32>, Bytes>(v6))
        }
    }
}

pub fn f600(v0: Bytes, v1: Adt31, v2: Bytes) -> R<Result<Adt32, Bytes>> {
    let m2881 = f667(v0.clone(), match v1.clone() {
        Adt31::C0(_, _, _, h2879, _) => {
            h2879
        }
    }, match v1 {
        Adt31::C0(_, _, _, _, h2880) => {
            h2880
        }
    }, v2)?;
    match m2881 {
        Ok(v4) => {
            Ok(Ok::<Adt32, Bytes>(Adt32::C0(f329(v0)?, match v4.clone() {
                Adt37::C0(_, h2882) => {
                    h2882
                }
            }, match v4 {
                Adt37::C0(h2883, _) => {
                    h2883
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt32, Bytes>(v3))
        }
    }
}

pub fn f601(v0: u64, v1: Adt29, v2: Bytes, v3: Bytes, v4: Adt31) -> R<Result<Adt36, Bytes>> {
    let m2884 = v0;
    if m2884 == 0 {
        Ok(Err::<Adt36, Bytes>(f605(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2884 - 1;
        let m2885 = f498(v4.clone(), Str::lit("type"));
        match m2885 {
            None => {
                let m2889 = f603(v4.clone())?;
                match m2889 {
                    Ok(v10) => {
                        let m2890 = f604(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m2890 {
                            None => {
                                Ok(Ok::<Adt36, Bytes>(Adt36::C0(Adt37::C0(f669(v4, f544()), List::cons(f544(), List::<Bytes>::nil())), List::<Adt32>::nil())))
                            }
                            Some(v11) => {
                                if f497(v11.clone(), Str::lit("complexType")) {
                                    let m2891 = f665(v5, v1, v2.clone(), v3, v11)?;
                                    match m2891 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt36, Bytes>(Adt36::C0(Adt37::C0(f669(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt36, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m2892 = f600(v2.clone(), v11, v3)?;
                                    match m2892 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt36, Bytes>(Adt36::C0(Adt37::C0(f669(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt32>::nil()))))
                                        }
                                        Err(v14) => {
                                            Ok(Err::<Adt36, Bytes>(v14))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v9) => {
                        Ok(Err::<Adt36, Bytes>(v9))
                    }
                }
            }
            Some(v6) => {
                let m2886 = f668(v1, v4.clone(), v6)?;
                match m2886 {
                    Ok(v8) => {
                        Ok(Ok::<Adt36, Bytes>(Adt36::C0(Adt37::C0(f669(v4, match v8.clone() {
                            Adt37::C0(h2887, _) => {
                                h2887
                            }
                        }), match v8 {
                            Adt37::C0(_, h2888) => {
                                h2888
                            }
                        }), List::<Adt32>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt36, Bytes>(f605(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f602(v0: Adt29, v1: Bytes, v2: Bytes, v3: Adt31) -> R<Result<Adt36, Bytes>> {
    let m2893 = f498(v3.clone(), Str::lit("type"));
    match m2893 {
        None => {
            let m2895 = f603(v3)?;
            match m2895 {
                Ok(v8) => {
                    let m2896 = f604(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m2896 {
                        None => {
                            Ok(Err::<Adt36, Bytes>(f605(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m2897 = f600(v2.clone(), v9, v1)?;
                            match m2897 {
                                Ok(v11) => {
                                    Ok(Ok::<Adt36, Bytes>(Adt36::C0(Adt37::C0(v2.clone(), List::cons(v2, List::<Bytes>::nil())), List::cons(v11, List::<Adt32>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt36, Bytes>(v10))
                                }
                            }
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt36, Bytes>(v7))
                }
            }
        }
        Some(v4) => {
            let m2894 = f668(v0, v3, v4)?;
            match m2894 {
                Ok(v6) => {
                    Ok(Ok::<Adt36, Bytes>(Adt36::C0(v6, List::<Adt32>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt36, Bytes>(f605(v5, v2)))
                }
            }
        }
    }
}

pub fn f603(v0: Adt31) -> R<Result<List<Adt31>, Bytes>> {
    f426(match v0.clone() {
        Adt31::C0(_, _, _, h2898, _) => {
            h2898
        }
    }, match v0 {
        Adt31::C0(_, _, _, _, h2899) => {
            h2899
        }
    })
}

pub fn f604(p2903: List<Adt31>, v1: List<Bytes>) -> Option<Adt31> {
    let m2902 = p2903;
    {
        let mut m2902 = m2902;
        loop {
            let step = {
                let v0 = m2902;
                let m2900 = v0;
                match m2900.uncons() {
                    None => {
                        Err::<List<Adt31>, Option<Adt31>>(None::<Adt31>)
                    }
                    Some((v2, v3)) => {
                        if f41(match v2.clone() {
                            Adt31::C0(_, h2901, _, _, _) => {
                                h2901
                            }
                        }, v1.clone()) {
                            Err::<List<Adt31>, Option<Adt31>>(Some(v2))
                        } else {
                            Ok::<List<Adt31>, Option<Adt31>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2902 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f605(v0: Bytes, v1: Bytes) -> Bytes {
    f422(v0, f8(Str::lit(" at ")), v1)
}

pub fn f606(v0: u64, v1: Adt29, v2: Bytes, v3: Bytes, v4: Adt31) -> R<Result<List<Adt32>, Bytes>> {
    let m2904 = v0;
    if m2904 == 0 {
        Ok(Err::<List<Adt32>, Bytes>(f605(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2904 - 1;
        let m2905 = f603(v4.clone())?;
        match m2905 {
            Ok(v7) => {
                if f497(v4, Str::lit("choice")) {
                    let m2906 = f670(v5, v1, v2.clone(), v3, false, v7, f608())?;
                    match m2906 {
                        Ok(v9) => {
                            let m2910 = f663(v2, f671(match v9.clone() {
                                Adt38::C0(h2907, _, _, _, _) => {
                                    h2907
                                }
                            }), f672(match v9.clone() {
                                Adt38::C0(h2908, _, _, _, _) => {
                                    h2908
                                }
                            }), f673(match v9.clone() {
                                Adt38::C0(h2909, _, _, _, _) => {
                                    h2909
                                }
                            }))?;
                            match m2910 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt32>, Bytes>(f541(match v9 {
                                        Adt38::C0(_, h2911, _, _, _) => {
                                            h2911
                                        }
                                    }, List::cons(v11, List::<Adt32>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<List<Adt32>, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<List<Adt32>, Bytes>(v8))
                        }
                    }
                } else {
                    let m2912 = f670(v5, v1, v2.clone(), v3, true, v7, f608())?;
                    match m2912 {
                        Ok(v13) => {
                            let m2914 = f674(v2, match v13.clone() {
                                Adt38::C0(h2913, _, _, _, _) => {
                                    h2913
                                }
                            })?;
                            match m2914 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt32>, Bytes>(f541(match v13 {
                                        Adt38::C0(_, h2915, _, _, _) => {
                                            h2915
                                        }
                                    }, List::cons(v15, List::<Adt32>::nil()))))
                                }
                                Err(v14) => {
                                    Ok(Err::<List<Adt32>, Bytes>(v14))
                                }
                            }
                        }
                        Err(v12) => {
                            Ok(Err::<List<Adt32>, Bytes>(v12))
                        }
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt32>, Bytes>(v6))
            }
        }
    }
}

pub fn f607(v0: u64, v1: Adt29, v2: Bytes, v3: Bytes, v4: List<Adt31>, v5: Adt38) -> R<Result<Adt38, Bytes>> {
    let m2916 = v0;
    if m2916 == 0 {
        Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m2916 - 1;
        let m2917 = v4;
        match m2917.uncons() {
            None => {
                Ok(Ok::<Adt38, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f497(v7.clone(), Str::lit("attribute")) {
                    if f661(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f607(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f661(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m2918 = f498(v7.clone(), Str::lit("ref"));
                        match m2918 {
                            None => {
                                let m2928 = f498(v7.clone(), Str::lit("name"));
                                match m2928 {
                                    None => {
                                        Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m2931 = f602(v1.clone(), v3.clone(), {
                                            let a2929 = v2.clone();
                                            let a2930 = f329(v16.clone())?;
                                            append_bytes(a2929, a2930)
                                        }, v7)?;
                                        match m2931 {
                                            Ok(v18) => {
                                                f607(v6, v1, v2, v3, v8, f676(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt38, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m2920 = f492(match v7 {
                                    Adt31::C0(_, _, _, _, h2919) => {
                                        h2919
                                    }
                                }, v10.clone())?;
                                match m2920 {
                                    Ok(v12) => {
                                        if f28({
                                            let (h2921, _) = v12.clone();
                                            h2921
                                        }, f566()) {
                                            Ok(Err::<Adt38, Bytes>(f605(f422(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m2922 = f675(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m2922 {
                                                None => {
                                                    Ok(Err::<Adt38, Bytes>(f605(f422(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m2926 = f602(v1.clone(), match v13.clone() {
                                                        Adt30::C0(_, h2923, _, _) => {
                                                            h2923
                                                        }
                                                    }, f329({
                                                        let (_, h2924) = v12.clone();
                                                        h2924
                                                    })?, match v13 {
                                                        Adt30::C0(_, _, _, h2925) => {
                                                            h2925
                                                        }
                                                    })?;
                                                    match m2926 {
                                                        Ok(v15) => {
                                                            f607(v6, v1, v2, v3, v8, f676(v5, {
                                                                let (_, h2927) = v12;
                                                                h2927
                                                            }, v15, v9))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<Adt38, Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt38, Bytes>(f605(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f497(v7.clone(), Str::lit("attributeGroup")) {
                        let m2932 = f498(v7.clone(), Str::lit("ref"));
                        match m2932 {
                            None => {
                                Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m2934 = f492(match v7 {
                                    Adt31::C0(_, _, _, _, h2933) => {
                                        h2933
                                    }
                                }, v19.clone())?;
                                match m2934 {
                                    Ok(v21) => {
                                        let m2935 = f675(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m2935 {
                                            None => {
                                                Ok(Err::<Adt38, Bytes>(f605(f422(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m2937 = f603(match v22.clone() {
                                                    Adt30::C0(_, _, _, h2936) => {
                                                        h2936
                                                    }
                                                })?;
                                                match m2937 {
                                                    Ok(v24) => {
                                                        let m2940 = f607(v6, v1.clone(), f329({
                                                            let (_, h2938) = v21;
                                                            h2938
                                                        })?, match v22 {
                                                            Adt30::C0(_, h2939, _, _) => {
                                                                h2939
                                                            }
                                                        }, v24, v5)?;
                                                        match m2940 {
                                                            Ok(v26) => {
                                                                f607(v6, v1, v2, v3, v8, v26)
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt38, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt38, Bytes>(v23))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt38, Bytes>(f605(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f497(v7, Str::lit("anyAttribute")) {
                            f607(v6, v1, v2, v3, v8, f677(v5, Adt39::C0(f8(Str::lit("anyAttribute")), f678(), List::<Bytes>::nil())))
                        } else {
                            f607(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f608() -> Adt38 {
    Adt38::C0(List::<Adt39>::nil(), List::<Adt32>::nil(), 0u64, 0u64, 0u64)
}

pub fn f609(v0: List<Adt32>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m2941 = v1;
    match m2941.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m2942 = f543(v0.clone(), v3.clone());
            match m2942 {
                None => {
                    Err::<List<Bytes>, Bytes>(f610(v3))
                }
                Some(v5) => {
                    let m2944 = match v5.clone() {
                        Adt32::C0(_, _, _, h2943) => {
                            h2943
                        }
                    };
                    match m2944.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f610(v3))
                        }
                        Some((_, _)) => {
                            let m2945 = f609(v0, v4, v2.clone());
                            match m2945 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f679(v3, match v5 {
                                        Adt32::C0(_, _, _, h2946) => {
                                            h2946
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

pub fn f610(v0: Bytes) -> Bytes {
    f605(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f611(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2947 = v0;
    match m2947.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f611(v3, v1)
            } else {
                List::cons(v2, f611(v3, v1))
            }
        }
    }
}

pub fn f612(p2952: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m2951 = p2952;
    {
        let mut m2951 = m2951;
        loop {
            let step = {
                let v0 = m2951;
                let m2948 = v0;
                match m2948.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h2949, _) = v2;
                            h2949
                        }, v1.clone()) {
                            let (_, h2950) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h2950)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2951 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f613(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f680(v0, v1)
}

pub fn f614(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f681(v0, v1)
}

pub fn f615(v0: u64) -> R<bool> {
    f682(v0, f683())
}

pub fn f616(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f617(v0: Bytes, v1: u64) -> R<u64> {
    let (h2953, _) = f287(Fn3::F684(v0.clone()), {
        let a2955 = {
            let a2954 = v0;
            length_bytes(a2954)
        };
        let a2956 = 1u64;
        nat_add(a2955, a2956)?
    }, v1)?;
    Ok(h2953)
}

pub fn f618(v0: Bytes) -> bool {
    f685(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f619(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m2960 = f571(v0.clone(), {
        let a2957 = Str::lit("--");
        utf8_encode(a2957)
    }, {
        let a2958 = v1;
        let a2959 = 4u64;
        nat_add(a2958, a2959)?
    })?;
    match m2960 {
        None => {
            Ok(f616(f559(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a2963 = f563(v0, {
                let a2961 = v2;
                let a2962 = 2u64;
                nat_add(a2961, a2962)?
            })?;
            let a2964 = 62u64;
            if nat_eq(a2963, a2964) {
                Ok(Ok::<u64, Bytes>({
                    let a2965 = v2;
                    let a2966 = 3u64;
                    nat_add(a2965, a2966)?
                }))
            } else {
                Ok(f616(f559(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f620(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f622(v0.clone(), {
        let a2967 = v1;
        let a2968 = 2u64;
        nat_add(a2967, a2968)?
    })?;
    let a2971 = v2;
    let a2972 = {
        let a2969 = v1;
        let a2970 = 2u64;
        nat_add(a2969, a2970)?
    };
    if nat_eq(a2971, a2972) {
        Ok(f616(f559(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a2975 = v2;
        let a2976 = {
            let a2973 = v1;
            let a2974 = 5u64;
            nat_add(a2973, a2974)?
        };
        if if nat_eq(a2975, a2976) {
            f685(f35(v0.clone(), {
                let a2977 = v1;
                let a2978 = 2u64;
                nat_add(a2977, a2978)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f616(f559(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m2980 = f571(v0, {
                let a2979 = Str::lit("?>");
                utf8_encode(a2979)
            }, v2)?;
            match m2980 {
                None => {
                    Ok(f616(f559(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a2981 = v3;
                        let a2982 = 2u64;
                        nat_add(a2981, a2982)?
                    }))
                }
            }
        }
    }
}

pub fn f621(v0: Bytes) -> Result<(Adt26, u64), Bytes> {
    Err::<(Adt26, u64), Bytes>(v0)
}

pub fn f622(v0: Bytes, v1: u64) -> R<u64> {
    let (h2983, _) = f287(Fn3::F686(v0.clone(), v1), {
        let a2985 = {
            let a2984 = v0;
            length_bytes(a2984)
        };
        let a2986 = 1u64;
        nat_add(a2985, a2986)?
    }, v1)?;
    Ok(h2983)
}

pub fn f623(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt26, u64), Bytes>> {
    let m2987 = v2;
    if m2987 == 0 {
        Ok(f621(f559(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m2987 - 1;
        let v5: u64 = f617(v0.clone(), v1)?;
        let a2988 = f563(v0.clone(), v5)?;
        let a2989 = 62u64;
        if if nat_eq(a2988, a2989) {
            true
        } else {
            f562(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt26, u64), Bytes>((Adt26::C4, v5)))
        } else {
            let a2990 = v5;
            let a2991 = v1;
            if nat_eq(a2990, a2991) {
                Ok(f621(f559(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f622(v0.clone(), v5)?;
                let a2992 = v6;
                let a2993 = v5;
                if nat_eq(a2992, a2993) {
                    Ok(f621(f559(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f35(v0.clone(), v5, {
                        let a2994 = v6;
                        let a2995 = v5;
                        nat_sub(a2994, a2995)
                    });
                    if f685(v7.clone(), v3.clone()) {
                        Ok(f621(f559(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f617(v0.clone(), v6)?;
                        let a2996 = f563(v0.clone(), v8)?;
                        let a2997 = 61u64;
                        if nat_eq(a2996, a2997) {
                            let v9: u64 = f617(v0.clone(), {
                                let a2998 = v8;
                                let a2999 = 1u64;
                                nat_add(a2998, a2999)?
                            })?;
                            let v10: u64 = f563(v0.clone(), v9)?;
                            let a3000 = v10;
                            let a3001 = 34u64;
                            if if nat_eq(a3000, a3001) {
                                true
                            } else {
                                let a3002 = v10;
                                let a3003 = 39u64;
                                nat_eq(a3002, a3003)
                            } {
                                let m3006 = f624(v0.clone(), {
                                    let a3004 = v9;
                                    let a3005 = 1u64;
                                    nat_add(a3004, a3005)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m3006 {
                                    Ok(v12) => {
                                        let m3008 = f623(v0, {
                                            let (_, h3007) = v12;
                                            h3007
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m3008 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt26, u64), Bytes>((Adt26::C1(v7, {
                                                    let (h3009, _) = v12;
                                                    h3009
                                                }, std::rc::Rc::new({
                                                    let (h3010, _) = v14;
                                                    h3010
                                                })), {
                                                    let (_, h3011) = v14;
                                                    h3011
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f621(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f621(v11))
                                    }
                                }
                            } else {
                                Ok(f621(f559(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f621(f559(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f624(v0: Bytes, p3051: u64, p3052: u64, p3053: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m3050 = (p3051, (p3052, p3053));
    {
        let mut m3050 = m3050;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3050;
                let m3012 = v2;
                if m3012 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f687(f559(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m3012 - 1;
                    let v7: u64 = f563(v0.clone(), v1)?;
                    if if v4 {
                        let a3013 = v7;
                        let a3014 = v5;
                        nat_eq(a3013, a3014)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a3015 = v1;
                            let a3016 = 1u64;
                            nat_add(a3015, a3016)?
                        }))))
                    } else {
                        let a3017 = v7;
                        let a3018 = 256u64;
                        if nat_eq(a3017, a3018) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f687(f559(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a3019 = v7;
                            let a3020 = 60u64;
                            if nat_eq(a3019, a3020) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f687(f559(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a3021 = v7;
                                let a3022 = 38u64;
                                if nat_eq(a3021, a3022) {
                                    let m3023 = f688(v0.clone(), v1)?;
                                    match m3023 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h3024) = v9;
                                                h3024
                                            }, (v6, {
                                                let a3026 = v3;
                                                let a3027 = {
                                                    let (h3025, _) = v9;
                                                    h3025
                                                };
                                                append_bytes(a3026, a3027)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f687(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a3028 = v7;
                                        let a3029 = 9u64;
                                        if if nat_eq(a3028, a3029) {
                                            true
                                        } else {
                                            let a3030 = v7;
                                            let a3031 = 10u64;
                                            nat_eq(a3030, a3031)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a3032 = v1;
                                                let a3033 = 1u64;
                                                nat_add(a3032, a3033)?
                                            }, (v6, {
                                                let a3034 = v3;
                                                let a3035 = f8(Str::lit(" "));
                                                append_bytes(a3034, a3035)
                                            })))
                                        } else {
                                            let v10: u64 = f689(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a3038 = v3;
                                                let a3039 = f35(v0.clone(), v1, {
                                                    let a3036 = v10;
                                                    let a3037 = v1;
                                                    nat_sub(a3036, a3037)
                                                });
                                                append_bytes(a3038, a3039)
                                            })))
                                        }
                                    } else {
                                        let a3040 = v7;
                                        let a3041 = 93u64;
                                        if nat_eq(a3040, a3041) {
                                            if f562(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f687(f559(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a3042 = v1;
                                                    let a3043 = 1u64;
                                                    nat_add(a3042, a3043)?
                                                }, (v6, {
                                                    let a3044 = v3;
                                                    let a3045 = f8(Str::lit("]"));
                                                    append_bytes(a3044, a3045)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f689(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a3048 = v3;
                                                let a3049 = f35(v0.clone(), v1, {
                                                    let a3046 = v11;
                                                    let a3047 = v1;
                                                    nat_sub(a3046, a3047)
                                                });
                                                append_bytes(a3048, a3049)
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
                Ok(next) => { m3050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f625(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f690(v1, v0, v2)
}

pub fn f626(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f351(v0.clone(), {
        let a3054 = v1;
        let a3055 = 1u64;
        nat_add(a3054, a3055)?
    })?;
    let v4: Bytes = {
        let a3056 = v3;
        let a3057 = 117u64;
        if nat_eq(a3056, a3057) {
            f691(v0, v1)?
        } else {
            f692(v3)
        }
    };
    let a3058 = v4.clone();
    let a3059 = Bytes::lit(&[]);
    if equal(a3058, a3059) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a3060 = v3;
            let a3061 = 117u64;
            if nat_eq(a3060, a3061) {
                let a3062 = v1;
                let a3063 = 6u64;
                nat_add(a3062, a3063)?
            } else {
                let a3064 = v1;
                let a3065 = 2u64;
                nat_add(a3064, a3065)?
            }
        }, ({
            let a3066 = v2;
            let a3067 = v4;
            append_bytes(a3066, a3067)
        }, 0u64))))
    }
}

pub fn f627(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3070 = {
        let a3068 = v0;
        let a3069 = v1;
        index_bytes(a3068, a3069)
    };
    match m3070 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f693(v2)? {
                Ok(Some({
                    let a3071 = v1;
                    let a3072 = 1u64;
                    nat_add(a3071, a3072)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f628(v0: u8) -> R<u64> {
    let a3119 = {
        let v1: u8 = v0;
        let a3117 = {
            let a3093 = {
                let a3081 = {
                    let a3075 = {
                        let a3073 = v1;
                        let a3074 = 128u8;
                        fixed_u8::bit_and(a3073, a3074)
                    };
                    let a3076 = 128u8;
                    if equal(a3075, a3076) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3082 = {
                    let a3079 = {
                        let a3077 = v1;
                        let a3078 = 64u8;
                        fixed_u8::bit_and(a3077, a3078)
                    };
                    let a3080 = 64u8;
                    if equal(a3079, a3080) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3081, a3082)?
            };
            let a3094 = {
                let a3091 = {
                    let a3085 = {
                        let a3083 = v1;
                        let a3084 = 32u8;
                        fixed_u8::bit_and(a3083, a3084)
                    };
                    let a3086 = 32u8;
                    if equal(a3085, a3086) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3092 = {
                    let a3089 = {
                        let a3087 = v1;
                        let a3088 = 16u8;
                        fixed_u8::bit_and(a3087, a3088)
                    };
                    let a3090 = 16u8;
                    if equal(a3089, a3090) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3091, a3092)?
            };
            nat_add(a3093, a3094)?
        };
        let a3118 = {
            let a3115 = {
                let a3103 = {
                    let a3097 = {
                        let a3095 = v1;
                        let a3096 = 8u8;
                        fixed_u8::bit_and(a3095, a3096)
                    };
                    let a3098 = 8u8;
                    if equal(a3097, a3098) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3104 = {
                    let a3101 = {
                        let a3099 = v1;
                        let a3100 = 4u8;
                        fixed_u8::bit_and(a3099, a3100)
                    };
                    let a3102 = 4u8;
                    if equal(a3101, a3102) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3103, a3104)?
            };
            let a3116 = {
                let a3113 = {
                    let a3107 = {
                        let a3105 = v1;
                        let a3106 = 2u8;
                        fixed_u8::bit_and(a3105, a3106)
                    };
                    let a3108 = 2u8;
                    if equal(a3107, a3108) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3114 = {
                    let a3111 = {
                        let a3109 = v1;
                        let a3110 = 1u8;
                        fixed_u8::bit_and(a3109, a3110)
                    };
                    let a3112 = 1u8;
                    if equal(a3111, a3112) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3113, a3114)?
            };
            nat_add(a3115, a3116)?
        };
        nat_add(a3117, a3118)?
    };
    let a3120 = 48u64;
    Ok(nat_sub(a3119, a3120))
}

pub fn f629(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3121 = v0.clone();
        length_bytes(a3121)
    };
    let a3122 = v1;
    let a3123 = v2;
    if if nat_lt(a3122, a3123) {
        let a3128 = f351(v0, {
            let a3126 = v2;
            let a3127 = {
                let a3124 = v1;
                let a3125 = 1u64;
                nat_add(a3124, a3125)?
            };
            nat_sub(a3126, a3127)
        })?;
        let a3129 = 48u64;
        nat_eq(a3128, a3129)
    } else {
        false
    } {
        Ok(Some({
            let a3130 = v1;
            let a3131 = 1u64;
            nat_add(a3130, a3131)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f630() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f631(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f514({
        let (_, h3132) = {
            let (h3133, _) = f694(Fn22::F695, {
                let a3135 = {
                    let a3134 = v0.clone();
                    length_list(a3134)
                };
                let a3136 = 1u64;
                nat_add(a3135, a3136)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3133
        };
        h3132
    }))
}

pub fn f632(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3137, _) = v0;
        h3137
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3138) = v0;
        h3138
    };
    let m3139 = v1;
    match m3139.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3140 = v4;
            match m3140.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f696(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f633(v0: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f528(f35(v0.clone(), 8u64, {
            let a3142 = {
                let a3141 = v0;
                length_bytes(a3141)
            };
            let a3143 = 8u64;
            nat_sub(a3142, a3143)
        }))?))
    } else {
        if f72(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f528(f35(v0.clone(), 14u64, {
                let a3145 = {
                    let a3144 = v0;
                    length_bytes(a3144)
                };
                let a3146 = 14u64;
                nat_sub(a3145, a3146)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f634(p3163: u64, p3164: u64, p3165: u64, p3166: Bytes) -> Bytes {
    let m3162 = (p3163, (p3164, (p3165, p3166)));
    {
        let mut m3162 = m3162;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3162;
                let m3147 = v0;
                if m3147 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3147 - 1;
                    if f353({
                        let a3148 = v1;
                        let a3149 = 0u64;
                        nat_eq(a3148, a3149)
                    }, {
                        let a3150 = v2;
                        let a3151 = 0u64;
                        nat_eq(a3150, a3151)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3152 = v1;
                            let a3153 = 16u64;
                            let a3154 = 0u64;
                            nat_quot(a3152, a3153, a3154)
                        }, ({
                            let a3155 = v2;
                            let a3156 = 1u64;
                            nat_sub(a3155, a3156)
                        }, {
                            let a3160 = f697({
                                let a3157 = v1;
                                let a3158 = 16u64;
                                let a3159 = 0u64;
                                nat_rem(a3157, a3158, a3159)
                            });
                            let a3161 = v3;
                            append_bytes(a3160, a3161)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f635(v0: List<Adt18>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f698(f699(f246(v0.clone()))?, v0, v1)
}

pub fn f636(v0: Bytes, v1: Bytes) -> Result<Adt35, Bytes> {
    Err::<Adt35, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f637(v0: Bytes, v1: List<Adt28>) -> Result<Adt35, Bytes> {
    Ok::<Adt35, Bytes>(Adt35::C0(v0, v1))
}

pub fn f638(v0: Bytes, v1: List<Adt18>) -> R<Result<Adt35, Bytes>> {
    let m3167 = v1.clone();
    match m3167.uncons() {
        None => {
            Ok(f636(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3168 = f700(f463(f242(v2, f8(Str::lit("properties")))), v1.clone());
            match m3168.uncons() {
                None => {
                    let m3169 = f701(f699(f702(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3169 {
                        Ok(v5) => {
                            Ok(f637(f546(v0, f641(v5.clone())), f642(v5)))
                        }
                        Err(v4) => {
                            Ok(f636(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3170 = f703(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3170 {
                        Ok(v9) => {
                            Ok(f637(f546(v0, f641(v9.clone())), f642(v9)))
                        }
                        Err(v8) => {
                            Ok(f636(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f639(v0: Adt18, v1: Bytes) -> R<bool> {
    let m3171 = v0;
    match m3171 {
        Adt18::C0(_) => {
            Ok(false)
        }
        Adt18::C1(_, _) => {
            Ok(false)
        }
        Adt18::C2(_, _, _) => {
            Ok(false)
        }
        Adt18::C3(_) => {
            Ok(false)
        }
        Adt18::C4(r3172) => {
            let v2 = (*r3172).clone();
            f639(v2, v1)
        }
        Adt18::C5(r3173) => {
            let v3 = (*r3173).clone();
            f639(v3, v1)
        }
        Adt18::C6 => {
            Ok(false)
        }
        Adt18::C7(r3174, r3175) => {
            let v4 = (*r3174).clone();
            let v5 = (*r3175).clone();
            if f639(v4, v1.clone())? {
                Ok(true)
            } else {
                f639(v5, v1)
            }
        }
        Adt18::C8(v6, r3176, r3177) => {
            let v7 = (*r3176).clone();
            let v8 = (*r3177).clone();
            if if f28(v6, f8(Str::lit("$ref"))) {
                f704(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f639(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f639(v8, v1)
                }
            }
        }
    }
}

pub fn f640(v0: Bytes, v1: List<(Bytes, Adt18)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt40, Bytes>> {
    f705(f699(f706(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f641(v0: Adt40) -> List<Bytes> {
    let m3178 = v0;
    match m3178 {
        Adt40::C0(v1, _) => {
            v1
        }
    }
}

pub fn f642(v0: Adt40) -> List<Adt28> {
    let m3179 = v0;
    match m3179 {
        Adt40::C0(_, v2) => {
            v2
        }
    }
}

pub fn f643(v0: Adt18) -> bool {
    let m3180 = f463(f242(v0.clone(), f8(Str::lit("properties"))));
    match m3180.uncons() {
        None => {
            let m3181 = f707(v0);
            match m3181 {
                None => {
                    false
                }
                Some(v3) => {
                    f708(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f644(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f645(v0: Adt18, v1: Bytes) -> R<Result<Adt35, Bytes>> {
    let m3182 = f709(v0.clone());
    match m3182 {
        None => {
            if f28(f245(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3183 = f242(v0, f8(Str::lit("items")));
                match m3183 {
                    None => {
                        Ok(f710(f8(Str::lit("List")), f712()))
                    }
                    Some(v3) => {
                        Ok(f710(f8(Str::lit("List")), f711(v3, {
                            let a3184 = v1;
                            let a3185 = f8(Str::lit("Item"));
                            append_bytes(a3184, a3185)
                        })?))
                    }
                }
            } else {
                f711(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f710(f8(Str::lit("Option")), f711(v2, v1)?))
        }
    }
}

pub fn f646(p3188: List<Adt18>) -> bool {
    let m3187 = p3188;
    {
        let mut m3187 = m3187;
        loop {
            let step = {
                let v0 = m3187;
                let m3186 = v0;
                match m3186.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f713(f525(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt18>, bool>(v2)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f647(v0: Adt18, v1: Bytes) -> Adt18 {
    let m3189 = v0;
    match m3189 {
        Adt18::C0(v6) => {
            Adt18::C0(v6)
        }
        Adt18::C1(v10, v11) => {
            Adt18::C1(v10, v11)
        }
        Adt18::C2(v7, v8, v9) => {
            Adt18::C2(v7, v8, v9)
        }
        Adt18::C3(v12) => {
            Adt18::C3(v12)
        }
        Adt18::C4(r3190) => {
            let v13 = (*r3190).clone();
            Adt18::C4(std::rc::Rc::new(v13))
        }
        Adt18::C5(r3191) => {
            let v5 = (*r3191).clone();
            Adt18::C5(std::rc::Rc::new(f647(v5, v1)))
        }
        Adt18::C6 => {
            Adt18::C6
        }
        Adt18::C7(r3192, r3193) => {
            let v14 = (*r3192).clone();
            let v15 = (*r3193).clone();
            Adt18::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt18::C8(v2, r3194, r3195) => {
            let v3 = (*r3194).clone();
            let v4 = (*r3195).clone();
            if f28(v2.clone(), v1.clone()) {
                f647(v4, v1)
            } else {
                Adt18::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f647(v4, v1)))
            }
        }
    }
}

pub fn f648(v0: Adt18, v1: Bytes) -> bool {
    let m3196 = f242(v0, v1);
    match m3196 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f649(v0: List<Adt18>) -> bool {
    let m3197 = v0.clone();
    match m3197.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3198 = f700(f463(f242(v1, f8(Str::lit("properties")))), v0.clone());
            match m3198.uncons() {
                None => {
                    f715(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f714(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f650(v0: Adt18, v1: Bytes, v2: Adt18) -> Adt18 {
    let m3199 = v0;
    match m3199 {
        Adt18::C0(v7) => {
            Adt18::C0(v7)
        }
        Adt18::C1(v11, v12) => {
            Adt18::C1(v11, v12)
        }
        Adt18::C2(v8, v9, v10) => {
            Adt18::C2(v8, v9, v10)
        }
        Adt18::C3(v13) => {
            Adt18::C3(v13)
        }
        Adt18::C4(r3200) => {
            let v14 = (*r3200).clone();
            Adt18::C4(std::rc::Rc::new(v14))
        }
        Adt18::C5(r3201) => {
            let v3 = (*r3201).clone();
            Adt18::C5(std::rc::Rc::new(Adt18::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt18::C6 => {
            Adt18::C6
        }
        Adt18::C7(r3202, r3203) => {
            let v15 = (*r3202).clone();
            let v16 = (*r3203).clone();
            Adt18::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt18::C8(v4, r3204, r3205) => {
            let v5 = (*r3204).clone();
            let v6 = (*r3205).clone();
            Adt18::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f651(p3208: List<Adt18>) -> bool {
    let m3207 = p3208;
    {
        let mut m3207 = m3207;
        loop {
            let step = {
                let v0 = m3207;
                let m3206 = v0;
                match m3206.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f716(v1) {
                            Ok::<List<Adt18>, bool>(v2)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f652(v0: u64) -> u32 {
    let a3209 = v0;
    let a3210 = 32u64;
    if nat_lt(a3209, a3210) {
        let a3211 = v0;
        let a3212 = 16u64;
        if nat_lt(a3211, a3212) {
            let a3213 = v0;
            let a3214 = 8u64;
            if nat_lt(a3213, a3214) {
                let a3215 = v0;
                let a3216 = 4u64;
                if nat_lt(a3215, a3216) {
                    let a3217 = v0;
                    let a3218 = 2u64;
                    if nat_lt(a3217, a3218) {
                        let a3219 = v0;
                        let a3220 = 1u64;
                        if nat_lt(a3219, a3220) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3221 = v0;
                        let a3222 = 3u64;
                        if nat_lt(a3221, a3222) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3223 = v0;
                    let a3224 = 6u64;
                    if nat_lt(a3223, a3224) {
                        let a3225 = v0;
                        let a3226 = 5u64;
                        if nat_lt(a3225, a3226) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3227 = v0;
                        let a3228 = 7u64;
                        if nat_lt(a3227, a3228) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3229 = v0;
                let a3230 = 12u64;
                if nat_lt(a3229, a3230) {
                    let a3231 = v0;
                    let a3232 = 10u64;
                    if nat_lt(a3231, a3232) {
                        let a3233 = v0;
                        let a3234 = 9u64;
                        if nat_lt(a3233, a3234) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3235 = v0;
                        let a3236 = 11u64;
                        if nat_lt(a3235, a3236) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3237 = v0;
                    let a3238 = 14u64;
                    if nat_lt(a3237, a3238) {
                        let a3239 = v0;
                        let a3240 = 13u64;
                        if nat_lt(a3239, a3240) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3241 = v0;
                        let a3242 = 15u64;
                        if nat_lt(a3241, a3242) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3243 = v0;
            let a3244 = 24u64;
            if nat_lt(a3243, a3244) {
                let a3245 = v0;
                let a3246 = 20u64;
                if nat_lt(a3245, a3246) {
                    let a3247 = v0;
                    let a3248 = 18u64;
                    if nat_lt(a3247, a3248) {
                        let a3249 = v0;
                        let a3250 = 17u64;
                        if nat_lt(a3249, a3250) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3251 = v0;
                        let a3252 = 19u64;
                        if nat_lt(a3251, a3252) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3253 = v0;
                    let a3254 = 22u64;
                    if nat_lt(a3253, a3254) {
                        let a3255 = v0;
                        let a3256 = 21u64;
                        if nat_lt(a3255, a3256) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3257 = v0;
                        let a3258 = 23u64;
                        if nat_lt(a3257, a3258) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3259 = v0;
                let a3260 = 28u64;
                if nat_lt(a3259, a3260) {
                    let a3261 = v0;
                    let a3262 = 26u64;
                    if nat_lt(a3261, a3262) {
                        let a3263 = v0;
                        let a3264 = 25u64;
                        if nat_lt(a3263, a3264) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3265 = v0;
                        let a3266 = 27u64;
                        if nat_lt(a3265, a3266) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3267 = v0;
                    let a3268 = 30u64;
                    if nat_lt(a3267, a3268) {
                        let a3269 = v0;
                        let a3270 = 29u64;
                        if nat_lt(a3269, a3270) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3271 = v0;
                        let a3272 = 31u64;
                        if nat_lt(a3271, a3272) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3273 = v0;
        let a3274 = 48u64;
        if nat_lt(a3273, a3274) {
            let a3275 = v0;
            let a3276 = 40u64;
            if nat_lt(a3275, a3276) {
                let a3277 = v0;
                let a3278 = 36u64;
                if nat_lt(a3277, a3278) {
                    let a3279 = v0;
                    let a3280 = 34u64;
                    if nat_lt(a3279, a3280) {
                        let a3281 = v0;
                        let a3282 = 33u64;
                        if nat_lt(a3281, a3282) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3283 = v0;
                        let a3284 = 35u64;
                        if nat_lt(a3283, a3284) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3285 = v0;
                    let a3286 = 38u64;
                    if nat_lt(a3285, a3286) {
                        let a3287 = v0;
                        let a3288 = 37u64;
                        if nat_lt(a3287, a3288) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3289 = v0;
                        let a3290 = 39u64;
                        if nat_lt(a3289, a3290) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3291 = v0;
                let a3292 = 44u64;
                if nat_lt(a3291, a3292) {
                    let a3293 = v0;
                    let a3294 = 42u64;
                    if nat_lt(a3293, a3294) {
                        let a3295 = v0;
                        let a3296 = 41u64;
                        if nat_lt(a3295, a3296) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3297 = v0;
                        let a3298 = 43u64;
                        if nat_lt(a3297, a3298) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3299 = v0;
                    let a3300 = 46u64;
                    if nat_lt(a3299, a3300) {
                        let a3301 = v0;
                        let a3302 = 45u64;
                        if nat_lt(a3301, a3302) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3303 = v0;
                        let a3304 = 47u64;
                        if nat_lt(a3303, a3304) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3305 = v0;
            let a3306 = 56u64;
            if nat_lt(a3305, a3306) {
                let a3307 = v0;
                let a3308 = 52u64;
                if nat_lt(a3307, a3308) {
                    let a3309 = v0;
                    let a3310 = 50u64;
                    if nat_lt(a3309, a3310) {
                        let a3311 = v0;
                        let a3312 = 49u64;
                        if nat_lt(a3311, a3312) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3313 = v0;
                        let a3314 = 51u64;
                        if nat_lt(a3313, a3314) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3315 = v0;
                    let a3316 = 54u64;
                    if nat_lt(a3315, a3316) {
                        let a3317 = v0;
                        let a3318 = 53u64;
                        if nat_lt(a3317, a3318) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3319 = v0;
                        let a3320 = 55u64;
                        if nat_lt(a3319, a3320) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3321 = v0;
                let a3322 = 60u64;
                if nat_lt(a3321, a3322) {
                    let a3323 = v0;
                    let a3324 = 58u64;
                    if nat_lt(a3323, a3324) {
                        let a3325 = v0;
                        let a3326 = 57u64;
                        if nat_lt(a3325, a3326) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3327 = v0;
                        let a3328 = 59u64;
                        if nat_lt(a3327, a3328) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3329 = v0;
                    let a3330 = 62u64;
                    if nat_lt(a3329, a3330) {
                        let a3331 = v0;
                        let a3332 = 61u64;
                        if nat_lt(a3331, a3332) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3333 = v0;
                        let a3334 = 63u64;
                        if nat_lt(a3333, a3334) {
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

pub fn f653(v0: u32) -> u32 {
    let a3337 = {
        let a3335 = f717(v0, 6u32, 26u32);
        let a3336 = f717(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3335, a3336)
    };
    let a3338 = f717(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3337, a3338)
}

pub fn f654(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3344 = {
        let a3339 = v0;
        let a3340 = v1;
        fixed_u32::bit_and(a3339, a3340)
    };
    let a3345 = {
        let a3342 = {
            let a3341 = v0;
            fixed_u32::bit_not(a3341)
        };
        let a3343 = v2;
        fixed_u32::bit_and(a3342, a3343)
    };
    fixed_u32::bit_xor(a3344, a3345)
}

pub fn f655(v0: u32) -> u32 {
    let a3348 = {
        let a3346 = f717(v0, 2u32, 30u32);
        let a3347 = f717(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3346, a3347)
    };
    let a3349 = f717(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3348, a3349)
}

pub fn f656(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3358 = {
        let a3354 = {
            let a3350 = v0;
            let a3351 = v1;
            fixed_u32::bit_and(a3350, a3351)
        };
        let a3355 = {
            let a3352 = v0;
            let a3353 = v2;
            fixed_u32::bit_and(a3352, a3353)
        };
        fixed_u32::bit_xor(a3354, a3355)
    };
    let a3359 = {
        let a3356 = v1;
        let a3357 = v2;
        fixed_u32::bit_and(a3356, a3357)
    };
    fixed_u32::bit_xor(a3358, a3359)
}

pub fn f657(v0: Adt33) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt33::C0(_, h3360, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3360
        }
    }, match v0.clone() {
        Adt33::C0(_, _, h3361, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3361
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, h3362, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3362
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, h3363, _, _, _, _, _, _, _, _, _, _, _) => {
            h3363
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, h3364, _, _, _, _, _, _, _, _, _, _) => {
            h3364
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, h3365, _, _, _, _, _, _, _, _, _) => {
            h3365
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, h3366, _, _, _, _, _, _, _, _) => {
            h3366
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, h3367, _, _, _, _, _, _, _) => {
            h3367
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, _, h3368, _, _, _, _, _, _) => {
            h3368
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, _, _, h3369, _, _, _, _, _) => {
            h3369
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, _, _, _, h3370, _, _, _, _) => {
            h3370
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3371, _, _, _) => {
            h3371
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3372, _, _) => {
            h3372
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3373, _) => {
            h3373
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3374) => {
            h3374
        }
    }, f536(f718(match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3375, _) => {
            h3375
        }
    }), f536(match v0.clone() {
        Adt33::C0(_, _, _, _, _, _, _, _, _, h3376, _, _, _, _, _, _) => {
            h3376
        }
    }, f536(f719(match v0.clone() {
        Adt33::C0(_, h3377, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3377
        }
    }), match v0 {
        Adt33::C0(h3378, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3378
        }
    }))))
}

pub fn f658(v0: Fn21, p3382: u64, p3383: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3381 = (p3382, p3383);
    {
        let mut m3381 = m3381;
        loop {
            let step = {
                let (v1, v2) = m3381;
                let m3379 = v1;
                if m3379 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3379 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3380: Fn21 = v0.clone();
                        c3380.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f659(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3384, _) = v0;
        h3384
    };
    let v2: u32 = {
        let (h3385, _) = {
            let (_, h3386) = v0;
            h3386
        };
        h3385
    };
    let v3: u32 = {
        let (_, h3387) = {
            let (_, h3388) = v0;
            h3388
        };
        h3387
    };
    ({
        let a3389 = v1;
        let a3390 = 2u64;
        let a3391 = 0u64;
        nat_quot(a3389, a3390, a3391)
    }, (f660(v2, 1u32), {
        let a3395 = {
            let a3392 = v1;
            let a3393 = 2u64;
            let a3394 = 0u64;
            nat_rem(a3392, a3393, a3394)
        };
        let a3396 = 1u64;
        if nat_eq(a3395, a3396) {
            let a3397 = v3;
            let a3398 = v2;
            fixed_u32::bit_or(a3397, a3398)
        } else {
            v3
        }
    }))
}

pub fn f660(v0: u32, v1: u32) -> u32 {
    let m3401 = {
        let a3399 = v0;
        let a3400 = v1;
        fixed_u32::shift_left(a3399, a3400)
    };
    match m3401 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f661(v0: Adt31, v1: Str, v2: Str) -> bool {
    let m3402 = f498(v0, v1);
    match m3402 {
        None => {
            false
        }
        Some(v3) => {
            f28(v3, f8(v2))
        }
    }
}

pub fn f662(v0: List<Adt30>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m3403 = v0;
    match m3403.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3404 = f662(v4, v1.clone(), v2)?;
            match m3404 {
                Ok(v6) => {
                    if f28(match v3.clone() {
                        Adt30::C0(h3405, _, _, _) => {
                            h3405
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m3406 = f720(v3.clone())?;
                        match m3406 {
                            Ok(v8) => {
                                let m3407 = v8;
                                match m3407 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f28({
                                            let (h3408, _) = {
                                                let (_, h3409) = v9.clone();
                                                h3409
                                            };
                                            h3408
                                        }, {
                                            let (h3410, _) = v1;
                                            h3410
                                        }) {
                                            if f28({
                                                let (_, h3411) = {
                                                    let (_, h3412) = v9;
                                                    h3412
                                                };
                                                h3411
                                            }, {
                                                let (_, h3413) = v1;
                                                h3413
                                            }) {
                                                if f496(v2) {
                                                    true
                                                } else {
                                                    f497({
                                                        let (h3414, _) = v9;
                                                        h3414
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt30::C0(_, _, h3415, _) => {
                                                    h3415
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

pub fn f663(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt32, Bytes>> {
    let m3416 = v1.clone();
    match m3416.uncons() {
        None => {
            Ok(Err::<Adt32, Bytes>(f605(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m3417 = f721(v1)?;
            match m3417 {
                Ok(v7) => {
                    let v8: List<Bytes> = f722(v7, v2);
                    Ok(Ok::<Adt32, Bytes>(Adt32::C0(v0.clone(), v3, f546(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt32, Bytes>(f605(v6, v0)))
                }
            }
        }
    }
}

pub fn f664(v0: List<Bytes>) -> R<List<Bytes>> {
    let m3418 = v0;
    match m3418.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f329(v1)?, f664(v2)?))
        }
    }
}

pub fn f665(v0: u64, v1: Adt29, v2: Bytes, v3: Bytes, v4: Adt31) -> R<Result<List<Adt32>, Bytes>> {
    let m3419 = v0;
    if m3419 == 0 {
        Ok(Err::<List<Adt32>, Bytes>(f605(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3419 - 1;
        let m3420 = f723(v5, v1, v2.clone(), v3, v4, f608())?;
        match m3420 {
            Ok(v7) => {
                let m3422 = f674(v2, match v7.clone() {
                    Adt38::C0(h3421, _, _, _, _) => {
                        h3421
                    }
                })?;
                match m3422 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt32>, Bytes>(f541(match v7 {
                            Adt38::C0(_, h3423, _, _, _) => {
                                h3423
                            }
                        }, List::cons(v9, List::<Adt32>::nil()))))
                    }
                    Err(v8) => {
                        Ok(Err::<List<Adt32>, Bytes>(v8))
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt32>, Bytes>(v6))
            }
        }
    }
}

pub fn f666(v0: Bytes) -> Bytes {
    let a3424 = v0;
    let a3425 = f8(Str::lit("Content"));
    append_bytes(a3424, a3425)
}

pub fn f667(v0: Bytes, v1: Adt26, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt37, Bytes>> {
    let v4: Bytes = f329(v0.clone())?;
    let m3426 = f724(v1, v2)?;
    match m3426 {
        Ok(v6) => {
            let m3427 = v6;
            match m3427 {
                None => {
                    Ok(f725(f726(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt41::C0(h3428, _, _, _) => {
                            h3428
                        }
                    };
                    let v9: Adt26 = match v7.clone() {
                        Adt41::C0(_, h3429, _, _) => {
                            h3429
                        }
                    };
                    let v10: Adt26 = match v7.clone() {
                        Adt41::C0(_, _, h3430, _) => {
                            h3430
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt41::C0(_, _, _, h3431) => {
                            h3431
                        }
                    };
                    if f28(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f725(f726(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f28(v8, f8(Str::lit("list"))) {
                            let m3432 = f494(v9, f8(Str::lit("itemType")));
                            match m3432 {
                                None => {
                                    Ok(f725(f726(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m3433 = f727(v11, v3, v12)?;
                                    match m3433 {
                                        Ok(v14) => {
                                            if f72(match v14.clone() {
                                                Adt37::C0(h3434, _) => {
                                                    h3434
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f725(f726(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt37, Bytes>(Adt37::C0(f728(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt37::C0(h3435, _) => {
                                                        h3435
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt37::C0(_, h3436) => {
                                                        h3436
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f725(f726(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m3437 = f494(v9, f8(Str::lit("base")));
                            match m3437 {
                                None => {
                                    Ok(f725(f726(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m3438 = f727(v11.clone(), v3, v15)?;
                                    match m3438 {
                                        Ok(v17) => {
                                            let m3439 = f729(v10, v11)?;
                                            match m3439 {
                                                Ok(v19) => {
                                                    let v20: List<Adt18> = f730(v19, List::<Bytes>::nil())?;
                                                    let m3440 = v20.clone();
                                                    match m3440.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt37, Bytes>(Adt37::C0(f728(v4, match v17.clone() {
                                                                Adt37::C0(h3441, _) => {
                                                                    h3441
                                                                }
                                                            }), match v17 {
                                                                Adt37::C0(_, h3442) => {
                                                                    h3442
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m3443 = f698(f699(f246(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m3443 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt37, Bytes>(Adt37::C0(f546(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f725(f726(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f725(f726(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f725(f726(v16, v0)))
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
            Ok(f725(f726(v5, v0)))
        }
    }
}

pub fn f668(v0: Adt29, v1: Adt31, v2: Bytes) -> R<Result<Adt37, Bytes>> {
    let m3445 = f492(match v1 {
        Adt31::C0(_, _, _, _, h3444) => {
            h3444
        }
    }, v2.clone())?;
    match m3445 {
        Ok(v4) => {
            if f28({
                let (h3446, _) = v4.clone();
                h3446
            }, f495()) {
                if f28({
                    let (_, h3447) = v4.clone();
                    h3447
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt37, Bytes>(Adt37::C0(f544(), List::cons(f544(), List::<Bytes>::nil()))))
                } else {
                    let m3449 = f731({
                        let (_, h3448) = v4;
                        h3448
                    }, 16u64)?;
                    match m3449 {
                        Ok(v6) => {
                            Ok(Ok::<Adt37, Bytes>(Adt37::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt37, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f41({
                    let (h3450, _) = v4.clone();
                    h3450
                }, match v0.clone() {
                    Adt29::C0(_, h3451) => {
                        h3451
                    }
                }) {
                    let v7: Bytes = f329({
                        let (_, h3452) = v4.clone();
                        h3452
                    })?;
                    let m3453 = f675(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m3453 {
                        None => {
                            let m3454 = f675(v0, Str::lit("simpleType"), v4);
                            match m3454 {
                                None => {
                                    Ok(Err::<Adt37, Bytes>(f422(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
                                }
                                Some(_) => {
                                    Ok(Ok::<Adt37, Bytes>(Adt37::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<Adt37, Bytes>(Adt37::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                        }
                    }
                } else {
                    Ok(Err::<Adt37, Bytes>(f422(f8(Str::lit("the type ")), v2, f422(f8(Str::lit(" is in the namespace ")), {
                        let (h3455, _) = v4;
                        h3455
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt37, Bytes>(v3))
        }
    }
}

pub fn f669(v0: Adt31, v1: Bytes) -> Bytes {
    if f661(v0, Str::lit("nillable"), Str::lit("true")) {
        f732(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f670(p3460: u64, v1: Adt29, v2: Bytes, v3: Bytes, v4: bool, p3461: List<Adt31>, p3462: Adt38) -> R<Result<Adt38, Bytes>> {
    let m3459 = (p3460, (p3461, p3462));
    {
        let mut m3459 = m3459;
        loop {
            let step = {
                let (v0, (v5, v6)) = m3459;
                let m3456 = v0;
                if m3456 == 0 {
                    Err::<(u64, (List<Adt31>, Adt38)), R<Result<Adt38, Bytes>>>(Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m3456 - 1;
                    let m3457 = v5;
                    match m3457.uncons() {
                        None => {
                            Err::<(u64, (List<Adt31>, Adt38)), R<Result<Adt38, Bytes>>>(Ok(Ok::<Adt38, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m3458 = f733(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m3458 {
                                Ok(v11) => {
                                    Ok::<(u64, (List<Adt31>, Adt38)), R<Result<Adt38, Bytes>>>((v7, (v9, v11)))
                                }
                                Err(v10) => {
                                    Err::<(u64, (List<Adt31>, Adt38)), R<Result<Adt38, Bytes>>>(Ok(Err::<Adt38, Bytes>(v10)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f671(v0: List<Adt39>) -> List<Bytes> {
    let m3463 = v0;
    match m3463.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt39::C0(h3464, _, _) => {
                    h3464
                }
            }, f671(v2))
        }
    }
}

pub fn f672(v0: List<Adt39>) -> List<Bytes> {
    let m3465 = v0;
    match m3465.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt39::C0(_, h3466, _) => {
                    h3466
                }
            }, f672(v2))
        }
    }
}

pub fn f673(v0: List<Adt39>) -> List<Bytes> {
    let m3467 = v0;
    match m3467.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f553(match v1 {
                Adt39::C0(_, _, h3468) => {
                    h3468
                }
            }, f673(v2))
        }
    }
}

pub fn f674(v0: Bytes, v1: List<Adt39>) -> R<Result<Adt32, Bytes>> {
    let m3469 = f721(f671(v1.clone()))?;
    match m3469 {
        Ok(v3) => {
            let v4: List<Bytes> = f672(v1.clone());
            let v7: Bytes = {
                let m3470 = v1.clone();
                match m3470.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f422(f8(Str::lit("\u{a}")), f342(f734(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt32, Bytes>(Adt32::C0(v0.clone(), f673(v1), f422(f8(Str::lit("structure{\u{a}  name: ")), v0, f422(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f422(f8(Str::lit("    {name: mk, fields: [")), f342(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt32, Bytes>(f605(v2, v0)))
        }
    }
}

pub fn f675(v0: Adt29, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt30> {
    f735(match v0 {
        Adt29::C0(h3471, _) => {
            h3471
        }
    }, f8(v1), {
        let (h3472, _) = v2;
        h3472
    }, {
        let (_, h3473) = v2;
        h3473
    })
}

pub fn f676(v0: Adt38, v1: Bytes, v2: Adt36, v3: bool) -> Adt38 {
    f736(f677(v0, Adt39::C0(v1, if v3 {
        let m3476 = match v2.clone() {
            Adt36::C0(h3475, _) => {
                h3475
            }
        };
        match m3476 {
            Adt37::C0(h3474, _) => {
                h3474
            }
        }
    } else {
        f732(Str::lit("Option"), {
            let m3479 = match v2.clone() {
                Adt36::C0(h3478, _) => {
                    h3478
                }
            };
            match m3479 {
                Adt37::C0(h3477, _) => {
                    h3477
                }
            }
        })
    }, {
        let m3482 = match v2.clone() {
            Adt36::C0(h3481, _) => {
                h3481
            }
        };
        match m3482 {
            Adt37::C0(_, h3480) => {
                h3480
            }
        }
    })), match v2 {
        Adt36::C0(_, h3483) => {
            h3483
        }
    })
}

pub fn f677(v0: Adt38, v1: Adt39) -> Adt38 {
    Adt38::C0(f737(match v0.clone() {
        Adt38::C0(h3484, _, _, _, _) => {
            h3484
        }
    }, List::cons(v1, List::<Adt39>::nil())), match v0.clone() {
        Adt38::C0(_, h3485, _, _, _) => {
            h3485
        }
    }, match v0.clone() {
        Adt38::C0(_, _, h3486, _, _) => {
            h3486
        }
    }, match v0.clone() {
        Adt38::C0(_, _, _, h3487, _) => {
            h3487
        }
    }, match v0 {
        Adt38::C0(_, _, _, _, h3488) => {
            h3488
        }
    })
}

pub fn f678() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f679(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f422(f8(Str::lit("inductive{\u{a}  name: ")), v0, f422(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f342(v1, f8(Str::lit(",\u{a}"))), f422(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f680(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h3489, _) = v1;
        h3489
    };
    let a3491 = {
        let a3490 = v0.clone();
        length_bytes(a3490)
    };
    let a3492 = v2;
    if nat_le(a3491, a3492) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a3493 = f563(v0.clone(), v2)?;
        let a3494 = 13u64;
        if nat_eq(a3493, a3494) {
            Ok(Some(({
                let a3497 = f563(v0, {
                    let a3495 = v2;
                    let a3496 = 1u64;
                    nat_add(a3495, a3496)?
                })?;
                let a3498 = 10u64;
                if nat_eq(a3497, a3498) {
                    let a3499 = v2;
                    let a3500 = 2u64;
                    nat_add(a3499, a3500)?
                } else {
                    let a3501 = v2;
                    let a3502 = 1u64;
                    nat_add(a3501, a3502)?
                }
            }, {
                let a3504 = {
                    let (_, h3503) = v1;
                    h3503
                };
                let a3505 = f8(Str::lit("\u{a}"));
                append_bytes(a3504, a3505)
            })))
        } else {
            let v4: u64 = {
                let m3506 = f571(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m3506 {
                    None => {
                        let a3507 = v0.clone();
                        length_bytes(a3507)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a3511 = {
                    let (_, h3508) = v1;
                    h3508
                };
                let a3512 = f35(v0, v2, {
                    let a3509 = v4;
                    let a3510 = v2;
                    nat_sub(a3509, a3510)
                });
                append_bytes(a3511, a3512)
            })))
        }
    }
}

pub fn f681(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3513 = f738(v0, v1)?;
    match m3513 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f739({
                let (h3514, _) = v2;
                h3514
            })? {
                Ok(Some({
                    let a3516 = v1;
                    let a3517 = {
                        let (_, h3515) = v2;
                        h3515
                    };
                    nat_add(a3516, a3517)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f682(v0: u64, v1: Bytes) -> R<bool> {
    f740(v0, v1.clone(), 0u64, {
        let a3519 = {
            let a3518 = v1;
            length_bytes(a3518)
        };
        let a3520 = 3u64;
        let a3521 = 0u64;
        nat_quot(a3519, a3520, a3521)
    }, true)
}

pub fn f683() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f684(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f741(v0, v1)
}

pub fn f685(v0: Bytes, p3524: List<Bytes>) -> bool {
    let m3523 = p3524;
    {
        let mut m3523 = m3523;
        loop {
            let step = {
                let v1 = m3523;
                let m3522 = v1;
                match m3522.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f28(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f742(v0, v1, v2)
}

pub fn f687(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f688(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f562(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a3527 = f563(v0.clone(), {
                let a3525 = v1;
                let a3526 = 2u64;
                nat_add(a3525, a3526)?
            })?;
            let a3528 = 120u64;
            nat_eq(a3527, a3528)
        };
        let v3: u64 = if v2 {
            let a3529 = v1;
            let a3530 = 3u64;
            nat_add(a3529, a3530)?
        } else {
            let a3531 = v1;
            let a3532 = 2u64;
            nat_add(a3531, a3532)?
        };
        let v4: (u64, u64) = f743(v0.clone(), v2, v3)?;
        let a3534 = {
            let (h3533, _) = v4;
            h3533
        };
        let a3535 = v3;
        if if nat_eq(a3534, a3535) {
            true
        } else {
            let a3537 = f563(v0, {
                let (h3536, _) = v4;
                h3536
            })?;
            let a3538 = 59u64;
            !nat_eq(a3537, a3538)
        } {
            Ok(f687(f559(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f739({
                let (_, h3539) = v4;
                h3539
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f744({
                    let (_, h3540) = v4;
                    h3540
                })?, {
                    let a3542 = {
                        let (h3541, _) = v4;
                        h3541
                    };
                    let a3543 = 1u64;
                    nat_add(a3542, a3543)?
                })))
            } else {
                Ok(f687(f559(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f562(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a3544 = v1;
                let a3545 = 4u64;
                nat_add(a3544, a3545)?
            })))
        } else {
            if f562(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a3546 = v1;
                    let a3547 = 4u64;
                    nat_add(a3546, a3547)?
                })))
            } else {
                if f562(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a3548 = v1;
                        let a3549 = 5u64;
                        nat_add(a3548, a3549)?
                    })))
                } else {
                    if f562(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a3550 = v1;
                            let a3551 = 6u64;
                            nat_add(a3550, a3551)?
                        })))
                    } else {
                        if f562(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a3552 = v1;
                                let a3553 = 6u64;
                                nat_add(a3552, a3553)?
                            })))
                        } else {
                            Ok(f687(f559(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f689(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h3554, _) = f287(Fn3::F745(v1, v2, v0.clone()), {
        let a3556 = {
            let a3555 = v0;
            length_bytes(a3555)
        };
        let a3557 = 1u64;
        nat_add(a3556, a3557)?
    }, v3)?;
    Ok(h3554)
}

pub fn f690(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f289(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a3559 = v2;
        let a3560 = {
            let a3558 = v0;
            length_bytes(a3558)
        };
        if nat_lt(a3559, a3560) {
            Ok(Some({
                let a3561 = v2;
                let a3562 = 1u64;
                nat_add(a3561, a3562)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f691(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3565 = f35(v0.clone(), {
        let a3563 = v1;
        let a3564 = 2u64;
        nat_add(a3563, a3564)?
    }, 2u64);
    let a3566 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3565, a3566) {
        let m3569 = f746(f35(v0, {
            let a3567 = v1;
            let a3568 = 4u64;
            nat_add(a3567, a3568)?
        }, 2u64))?;
        match m3569 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f351(v2.clone(), 0u64)?;
                let a3570 = v3;
                let a3571 = 32u64;
                if nat_lt(a3570, a3571) {
                    if f747(v3) {
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

pub fn f692(v0: u64) -> Bytes {
    let a3572 = v0;
    let a3573 = 34u64;
    if nat_eq(a3572, a3573) {
        Bytes::lit(&[0x22])
    } else {
        let a3574 = v0;
        let a3575 = 92u64;
        if nat_eq(a3574, a3575) {
            Bytes::lit(&[0x5c])
        } else {
            let a3576 = v0;
            let a3577 = 98u64;
            if nat_eq(a3576, a3577) {
                Bytes::lit(&[0x08])
            } else {
                let a3578 = v0;
                let a3579 = 102u64;
                if nat_eq(a3578, a3579) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3580 = v0;
                    let a3581 = 110u64;
                    if nat_eq(a3580, a3581) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3582 = v0;
                        let a3583 = 114u64;
                        if nat_eq(a3582, a3583) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3584 = v0;
                            let a3585 = 116u64;
                            if nat_eq(a3584, a3585) {
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

pub fn f693(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3630 = {
            let a3606 = {
                let a3594 = {
                    let a3588 = {
                        let a3586 = v1;
                        let a3587 = 128u8;
                        fixed_u8::bit_and(a3586, a3587)
                    };
                    let a3589 = 128u8;
                    if equal(a3588, a3589) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3595 = {
                    let a3592 = {
                        let a3590 = v1;
                        let a3591 = 64u8;
                        fixed_u8::bit_and(a3590, a3591)
                    };
                    let a3593 = 64u8;
                    if equal(a3592, a3593) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3594, a3595)?
            };
            let a3607 = {
                let a3604 = {
                    let a3598 = {
                        let a3596 = v1;
                        let a3597 = 32u8;
                        fixed_u8::bit_and(a3596, a3597)
                    };
                    let a3599 = 32u8;
                    if equal(a3598, a3599) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3605 = {
                    let a3602 = {
                        let a3600 = v1;
                        let a3601 = 16u8;
                        fixed_u8::bit_and(a3600, a3601)
                    };
                    let a3603 = 16u8;
                    if equal(a3602, a3603) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3604, a3605)?
            };
            nat_add(a3606, a3607)?
        };
        let a3631 = {
            let a3628 = {
                let a3616 = {
                    let a3610 = {
                        let a3608 = v1;
                        let a3609 = 8u8;
                        fixed_u8::bit_and(a3608, a3609)
                    };
                    let a3611 = 8u8;
                    if equal(a3610, a3611) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3617 = {
                    let a3614 = {
                        let a3612 = v1;
                        let a3613 = 4u8;
                        fixed_u8::bit_and(a3612, a3613)
                    };
                    let a3615 = 4u8;
                    if equal(a3614, a3615) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3616, a3617)?
            };
            let a3629 = {
                let a3626 = {
                    let a3620 = {
                        let a3618 = v1;
                        let a3619 = 2u8;
                        fixed_u8::bit_and(a3618, a3619)
                    };
                    let a3621 = 2u8;
                    if equal(a3620, a3621) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3627 = {
                    let a3624 = {
                        let a3622 = v1;
                        let a3623 = 1u8;
                        fixed_u8::bit_and(a3622, a3623)
                    };
                    let a3625 = 1u8;
                    if equal(a3624, a3625) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3626, a3627)?
            };
            nat_add(a3628, a3629)?
        };
        nat_add(a3630, a3631)?
    };
    let a3632 = 48u64;
    let a3633 = v2;
    if nat_le(a3632, a3633) {
        let a3634 = v2;
        let a3635 = 57u64;
        Ok(nat_le(a3634, a3635))
    } else {
        Ok(false)
    }
}

pub fn f694(v0: Fn22, p3640: u64, p3641: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3639 = (p3640, p3641);
    {
        let mut m3639 = m3639;
        loop {
            let step = {
                let (v1, v2) = m3639;
                let m3636 = v1;
                if m3636 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3636 - 1;
                    let m3638 = {
                        let c3637: Fn22 = v0.clone();
                        c3637.apply(v2.clone())?
                    };
                    match m3638 {
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
                Ok(next) => { m3639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f695(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f748(v0)
}

pub fn f696(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3646 = {
            let a3644 = {
                let a3642 = v0.clone();
                length_list(a3642)
            };
            let a3645 = {
                let a3643 = v1.clone();
                length_list(a3643)
            };
            nat_add(a3644, a3645)?
        };
        let a3647 = 1u64;
        nat_add(a3646, a3647)?
    };
    Ok(f392({
        let (_, h3648) = {
            let (_, h3649) = {
                let (h3650, _) = f749(Fn23::F750, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3650
            };
            h3649
        };
        h3648
    }))
}

pub fn f697(v0: u64) -> Bytes {
    f35({
        let a3651 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3651)
    }, v0, 1u64)
}

pub fn f698(v0: bool, v1: List<Adt18>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3652 = v1;
    match m3652.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3653 = f751(v0, v3.clone())?;
            match m3653 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f752(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f41(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3654 = f698(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3654 {
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

pub fn f699(p3658: List<Bytes>) -> R<bool> {
    let m3657 = p3658;
    {
        let mut m3657 = m3657;
        loop {
            let step = {
                let v0 = m3657;
                let m3655 = v0;
                match m3655.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3656 = f753(v1)?;
                        match m3656 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f41(v3, f754(false, v2.clone())?) {
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
                Ok(next) => { m3657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f700(v0: List<(Bytes, Adt18)>, v1: List<Adt18>) -> List<Bytes> {
    let m3659 = v0;
    match m3659.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f755(v1.clone(), {
                let (h3660, _) = v2.clone();
                h3660
            }) {
                List::cons({
                    let (h3661, _) = v2;
                    h3661
                }, f700(v3, v1))
            } else {
                f700(v3, v1)
            }
        }
    }
}

pub fn f701(v0: bool, v1: Bytes, v2: List<Adt18>, v3: List<Bytes>) -> R<Result<Adt40, Bytes>> {
    let m3662 = v2;
    match m3662.uncons() {
        None => {
            Ok(Ok::<Adt40, Bytes>(Adt40::C0(List::<Bytes>::nil(), List::<Adt28>::nil())))
        }
        Some((v4, v5)) => {
            let m3663 = f756(v4.clone())?;
            match m3663 {
                None => {
                    Ok(Err::<Adt40, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3664 = f757(v0, v6.clone())?;
                    match m3664 {
                        None => {
                            Ok(Err::<Adt40, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f41(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt40, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3667 = f758(v4, {
                                    let a3665 = v1.clone();
                                    let a3666 = f329(v6)?;
                                    append_bytes(a3665, a3666)
                                })?;
                                match m3667 {
                                    Ok(v9) => {
                                        let m3668 = f701(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3668 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt40, Bytes>(Adt40::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f533(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f641(v11.clone())), f467(f532(v9), f642(v11)))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<Adt40, Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<Adt40, Bytes>(v8))
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

pub fn f702(v0: List<Adt18>) -> R<List<Bytes>> {
    let m3669 = v0;
    match m3669.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3670 = f756(v1)?;
            match m3670 {
                None => {
                    f702(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f702(v2)?))
                }
            }
        }
    }
}

pub fn f703(v0: Bytes, v1: List<Adt18>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt40, Bytes>> {
    f759(f699(f760(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f704(v0: Adt18, v1: Bytes) -> R<bool> {
    let m3671 = f334(Some(v0));
    match m3671 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3672 = f633(v2)?;
            match m3672 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f28(v3, v1))
                }
            }
        }
    }
}

pub fn f705(v0: bool, v1: Bytes, v2: List<(Bytes, Adt18)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt40, Bytes>> {
    let m3673 = v2;
    match m3673.uncons() {
        None => {
            Ok(Ok::<Adt40, Bytes>(Adt40::C0(List::<Bytes>::nil(), List::<Adt28>::nil())))
        }
        Some((v5, v6)) => {
            let m3675 = f761(v0, {
                let (h3674, _) = v5.clone();
                h3674
            })?;
            match m3675 {
                None => {
                    Ok(Err::<Adt40, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3676, _) = v5;
                        h3676
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f41(v7.clone(), f762(v0, f706(v6.clone()))?) {
                        Ok(Err::<Adt40, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3681 = f645({
                            let (_, h3677) = v5.clone();
                            h3677
                        }, {
                            let a3679 = v1.clone();
                            let a3680 = f329({
                                let (h3678, _) = v5.clone();
                                h3678
                            })?;
                            append_bytes(a3679, a3680)
                        })?;
                        match m3681 {
                            Ok(v9) => {
                                let v10: Bytes = if f41({
                                    let (h3682, _) = v5;
                                    h3682
                                }, v3.clone()) {
                                    f533(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f533(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3683 = f705(v0, v1, v6, v3, v4)?;
                                match m3683 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt40, Bytes>(Adt40::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f641(v12.clone())), f467(f532(v9), f642(v12)))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt40, Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<Adt40, Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f706(v0: List<(Bytes, Adt18)>) -> List<Bytes> {
    let m3684 = v0;
    match m3684.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3685, _) = v1;
                h3685
            }, f706(v2))
        }
    }
}

pub fn f707(v0: Adt18) -> Option<Adt18> {
    let m3686 = f242(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3686 {
        None => {
            f763(v0)
        }
        Some(v1) => {
            if f708(v1.clone()) {
                let m3687 = f763(v0);
                match m3687 {
                    None => {
                        Some(v1)
                    }
                    Some(v2) => {
                        Some(v2)
                    }
                }
            } else {
                Some(v1)
            }
        }
    }
}

pub fn f708(v0: Adt18) -> bool {
    let m3688 = v0;
    match m3688 {
        Adt18::C0(v1) => {
            !v1
        }
        Adt18::C1(_, _) => {
            false
        }
        Adt18::C2(_, _, _) => {
            false
        }
        Adt18::C3(_) => {
            false
        }
        Adt18::C4(_) => {
            false
        }
        Adt18::C5(_) => {
            false
        }
        Adt18::C6 => {
            false
        }
        Adt18::C7(_, _) => {
            false
        }
        Adt18::C8(_, _, _) => {
            false
        }
    }
}

pub fn f709(v0: Adt18) -> Option<Adt18> {
    let m3689 = f247(f242(v0, f8(Str::lit("oneOf"))));
    match m3689.uncons() {
        None => {
            None::<Adt18>
        }
        Some((v1, v2)) => {
            let m3690 = v2;
            match m3690.uncons() {
                None => {
                    None::<Adt18>
                }
                Some((v3, v4)) => {
                    let m3691 = v4;
                    match m3691.uncons() {
                        None => {
                            if f764(v3.clone()) {
                                Some(v1)
                            } else {
                                if f764(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt18>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt18>
                        }
                    }
                }
            }
        }
    }
}

pub fn f710(v0: Bytes, v1: Result<Adt35, Bytes>) -> Result<Adt35, Bytes> {
    let m3692 = v1;
    match m3692 {
        Ok(v3) => {
            f637(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f533(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f532(v3))
        }
        Err(v2) => {
            Err::<Adt35, Bytes>(v2)
        }
    }
}

pub fn f711(v0: Adt18, v1: Bytes) -> R<Result<Adt35, Bytes>> {
    let m3693 = f709(v0.clone());
    match m3693 {
        None => {
            if f28(f245(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3694 = f242(v0, f8(Str::lit("items")));
                match m3694 {
                    None => {
                        Ok(f710(f8(Str::lit("List")), f712()))
                    }
                    Some(v3) => {
                        Ok(f710(f8(Str::lit("List")), f758(v3, {
                            let a3695 = v1;
                            let a3696 = f8(Str::lit("Item"));
                            append_bytes(a3695, a3696)
                        })?))
                    }
                }
            } else {
                f758(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f710(f8(Str::lit("Option")), f758(v2, v1)?))
        }
    }
}

pub fn f712() -> Result<Adt35, Bytes> {
    Ok::<Adt35, Bytes>(Adt35::C0(f529(), List::cons(Adt28::C2, List::<Adt28>::nil())))
}

pub fn f713(p3700: List<(Bytes, Adt18)>, v1: List<Bytes>) -> bool {
    let m3699 = p3700;
    {
        let mut m3699 = m3699;
        loop {
            let step = {
                let v0 = m3699;
                let m3697 = v0;
                match m3697.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h3698, _) = v2;
                            h3698
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt18)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt18)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f714(p3704: List<Adt18>, v1: Bytes, p3705: List<Bytes>) -> bool {
    let m3703 = (p3704, p3705);
    {
        let mut m3703 = m3703;
        loop {
            let step = {
                let (v0, v2) = m3703;
                let m3701 = v0;
                match m3701.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3702 = f765(v3, v1.clone());
                        match m3702 {
                            None => {
                                Err::<(List<Adt18>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f41(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt18>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt18>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f715(p3708: List<Adt18>, p3709: List<Bytes>) -> bool {
    let m3707 = (p3708, p3709);
    {
        let mut m3707 = m3707;
        loop {
            let step = {
                let (v0, v1) = m3707;
                let m3706 = v0;
                match m3706.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f245(v2, f8(Str::lit("type")));
                        if !f28(v4.clone(), Bytes::lit(&[])) {
                            if !f41(v4.clone(), v1.clone()) {
                                if !if if f28(v4.clone(), f8(Str::lit("integer"))) {
                                    f41(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f28(v4.clone(), f8(Str::lit("number"))) {
                                        f41(f8(Str::lit("integer")), v1.clone())
                                    } else {
                                        false
                                    }
                                } {
                                    Ok::<(List<Adt18>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt18>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt18>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt18>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f716(v0: Adt18) -> bool {
    let v2: Adt18 = {
        let m3710 = f242(v0.clone(), f8(Str::lit("anyOf")));
        match m3710 {
            None => {
                Adt18::C6
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f713(f525(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f646(f335(v2))
    } else {
        false
    }
}

pub fn f717(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3711 = f472(v0, v1);
    let a3712 = f660(v0, v2);
    fixed_u32::bit_or(a3711, a3712)
}

pub fn f718(v0: u32) -> u32 {
    let a3715 = {
        let a3713 = f717(v0, 17u32, 15u32);
        let a3714 = f717(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3713, a3714)
    };
    let a3716 = f472(v0, 10u32);
    fixed_u32::bit_xor(a3715, a3716)
}

pub fn f719(v0: u32) -> u32 {
    let a3719 = {
        let a3717 = f717(v0, 7u32, 25u32);
        let a3718 = f717(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3717, a3718)
    };
    let a3720 = f472(v0, 3u32);
    fixed_u32::bit_xor(a3719, a3720)
}

pub fn f720(v0: Adt30) -> R<Result<Option<(Adt31, (Bytes, Bytes))>, Bytes>> {
    let m3722 = f603(match v0 {
        Adt30::C0(_, _, _, h3721) => {
            h3721
        }
    })?;
    match m3722 {
        Ok(v2) => {
            let m3723 = f604(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m3723 {
                None => {
                    Ok(Ok::<Option<(Adt31, (Bytes, Bytes))>, Bytes>(None::<(Adt31, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m3724 = f603(v3)?;
                    match m3724 {
                        Ok(v5) => {
                            let m3725 = f604(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m3725 {
                                None => {
                                    Ok(Ok::<Option<(Adt31, (Bytes, Bytes))>, Bytes>(None::<(Adt31, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m3726 = f498(v6.clone(), Str::lit("base"));
                                    match m3726 {
                                        None => {
                                            Ok(Ok::<Option<(Adt31, (Bytes, Bytes))>, Bytes>(None::<(Adt31, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m3728 = f492(match v6.clone() {
                                                Adt31::C0(_, _, _, _, h3727) => {
                                                    h3727
                                                }
                                            }, v7)?;
                                            match m3728 {
                                                Ok(v9) => {
                                                    Ok(Ok::<Option<(Adt31, (Bytes, Bytes))>, Bytes>(Some((v6, v9))))
                                                }
                                                Err(v8) => {
                                                    Ok(Err::<Option<(Adt31, (Bytes, Bytes))>, Bytes>(v8))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(v4) => {
                            Ok(Err::<Option<(Adt31, (Bytes, Bytes))>, Bytes>(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Ok(Err::<Option<(Adt31, (Bytes, Bytes))>, Bytes>(v1))
        }
    }
}

pub fn f721(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f766(f699(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f722(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3729 = v0;
    match m3729.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3730 = v1;
            match m3730.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f422(f8(Str::lit("    {name: ")), v2, f422(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f722(v3, v5))
                }
            }
        }
    }
}

pub fn f723(v0: u64, v1: Adt29, v2: Bytes, v3: Bytes, v4: Adt31, v5: Adt38) -> R<Result<Adt38, Bytes>> {
    let m3731 = v0;
    if m3731 == 0 {
        Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m3731 - 1;
        if f661(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m3732 = f603(v4)?;
            match m3732 {
                Ok(v8) => {
                    let m3733 = f604(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m3733 {
                        None => {
                            let m3735 = f604(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m3735 {
                                None => {
                                    let m3736 = f670(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m3736 {
                                        Ok(v11) => {
                                            f607(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt38, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f661(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m3737 = f603(v12.clone())?;
                                        match m3737 {
                                            Ok(v14) => {
                                                let m3738 = f604(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m3738 {
                                                    None => {
                                                        Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m3739 = f498(v15.clone(), Str::lit("base"));
                                                        match m3739 {
                                                            None => {
                                                                Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m3741 = f492(match v15.clone() {
                                                                    Adt31::C0(_, _, _, _, h3740) => {
                                                                        h3740
                                                                    }
                                                                }, v16.clone())?;
                                                                match m3741 {
                                                                    Ok(v18) => {
                                                                        let m3742 = f603(v15.clone())?;
                                                                        match m3742 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f28({
                                                                                    let (h3743, _) = v18.clone();
                                                                                    h3743
                                                                                }, f495()) {
                                                                                    f28({
                                                                                        let (_, h3744) = v18.clone();
                                                                                        h3744
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f497(v15.clone(), Str::lit("restriction")) {
                                                                                    f496(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt38, Bytes>(f605(f422(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m3759 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f28(match v12 {
                                                                                            Adt31::C0(_, h3745, _, _, _) => {
                                                                                                h3745
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt38, Bytes>(v5)
                                                                                        } else {
                                                                                            let m3746 = f675(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m3746 {
                                                                                                None => {
                                                                                                    Err::<Adt38, Bytes>(f605(f422(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m3750 = f723(v6, v1.clone(), f329({
                                                                                                        let (_, h3747) = v18;
                                                                                                        h3747
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt30::C0(_, h3748, _, _) => {
                                                                                                            h3748
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt30::C0(_, _, _, h3749) => {
                                                                                                            h3749
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m3750 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt38, Bytes>(f767(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt38, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m3751 = f675(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m3751 {
                                                                                            None => {
                                                                                                let m3756 = f668(v1.clone(), v15, v16)?;
                                                                                                match m3756 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt38, Bytes>(f677(v5, Adt39::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt37::C0(h3757, _) => {
                                                                                                                h3757
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt37::C0(_, h3758) => {
                                                                                                                h3758
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt38, Bytes>(f605(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m3755 = f723(v6, v1.clone(), f329({
                                                                                                    let (_, h3752) = v18;
                                                                                                    h3752
                                                                                                })?, match v25.clone() {
                                                                                                    Adt30::C0(_, h3753, _, _) => {
                                                                                                        h3753
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt30::C0(_, _, _, h3754) => {
                                                                                                        h3754
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m3755 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt38, Bytes>(f767(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt38, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m3759 {
                                                                                        Ok(v31) => {
                                                                                            let m3760 = f670(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m3760 {
                                                                                                Ok(v33) => {
                                                                                                    f607(v6, v1, v2, v3, v20, v33)
                                                                                                }
                                                                                                Err(v32) => {
                                                                                                    Ok(Err::<Adt38, Bytes>(v32))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Err(v30) => {
                                                                                            Ok(Err::<Adt38, Bytes>(v30))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v19) => {
                                                                                Ok(Err::<Adt38, Bytes>(v19))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<Adt38, Bytes>(f605(v17, v2)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Err::<Adt38, Bytes>(v13))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v9) => {
                            Ok(Err::<Adt38, Bytes>(f605(f422(f8(Str::lit("xs:")), match v9 {
                                Adt31::C0(_, h3734, _, _, _) => {
                                    h3734
                                }
                            }, f8(Str::lit(" is not translated, because an open content wildcard or an XPath assertion constrains an instance beyond its fields (XSD 1.1 Part 1, sections 3.4.2.3 and 3.13)"))), v2)))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt38, Bytes>(v7))
                }
            }
        }
    }
}

pub fn f724(p3773: Adt26, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt41>, Bytes>> {
    let m3772 = p3773;
    {
        let mut m3772 = m3772;
        loop {
            let step = {
                let v0 = m3772;
                let m3761 = v0;
                match m3761 {
                    Adt26::C0(_, _, _) => {
                        Err::<Adt26, R<Result<Option<Adt41>, Bytes>>>(Ok(Ok::<Option<Adt41>, Bytes>(None::<Adt41>)))
                    }
                    Adt26::C1(_, _, _) => {
                        Err::<Adt26, R<Result<Option<Adt41>, Bytes>>>(Ok(Ok::<Option<Adt41>, Bytes>(None::<Adt41>)))
                    }
                    Adt26::C2(_) => {
                        Err::<Adt26, R<Result<Option<Adt41>, Bytes>>>(Ok(Ok::<Option<Adt41>, Bytes>(None::<Adt41>)))
                    }
                    Adt26::C3(r3770, r3771) => {
                        let v2 = (*r3770).clone();
                        let v3 = (*r3771).clone();
                        let m3762 = v2;
                        match m3762 {
                            Adt26::C0(v4, r3768, r3769) => {
                                let v5 = (*r3768).clone();
                                let v6 = (*r3769).clone();
                                let m3763 = f491(v5.clone(), v1.clone());
                                match m3763 {
                                    Ok(v8) => {
                                        let m3764 = f492(v8.clone(), v4)?;
                                        match m3764 {
                                            Ok(v10) => {
                                                if f685({
                                                    let (_, h3765) = v10.clone();
                                                    h3765
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f28({
                                                        let (h3766, _) = v10;
                                                        h3766
                                                    }, f495()) {
                                                        Err::<Adt26, R<Result<Option<Adt41>, Bytes>>>(Ok(Ok::<Option<Adt41>, Bytes>(Some(Adt41::C0({
                                                            let (_, h3767) = v10;
                                                            h3767
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt26, R<Result<Option<Adt41>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt26, R<Result<Option<Adt41>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt26, R<Result<Option<Adt41>, Bytes>>>(Ok(f768(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt26, R<Result<Option<Adt41>, Bytes>>>(Ok(f768(v7)))
                                    }
                                }
                            }
                            Adt26::C1(_, _, _) => {
                                Ok::<Adt26, R<Result<Option<Adt41>, Bytes>>>(v3)
                            }
                            Adt26::C2(_) => {
                                Ok::<Adt26, R<Result<Option<Adt41>, Bytes>>>(v3)
                            }
                            Adt26::C3(_, _) => {
                                Ok::<Adt26, R<Result<Option<Adt41>, Bytes>>>(v3)
                            }
                            Adt26::C4 => {
                                Ok::<Adt26, R<Result<Option<Adt41>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt26::C4 => {
                        Err::<Adt26, R<Result<Option<Adt41>, Bytes>>>(Ok(Ok::<Option<Adt41>, Bytes>(None::<Adt41>)))
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

pub fn f725(v0: Bytes) -> Result<Adt37, Bytes> {
    Err::<Adt37, Bytes>(v0)
}

pub fn f726(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f727(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt37, Bytes>> {
    let m3774 = f492(v0, v2.clone())?;
    match m3774 {
        Ok(v4) => {
            if f28({
                let (h3775, _) = v4.clone();
                h3775
            }, f495()) {
                let m3777 = f731({
                    let (_, h3776) = v4;
                    h3776
                }, 16u64)?;
                match m3777 {
                    Ok(v6) => {
                        Ok(Ok::<Adt37, Bytes>(Adt37::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f725(v5))
                    }
                }
            } else {
                if f28({
                    let (h3778, _) = v4.clone();
                    h3778
                }, v1) {
                    let v7: Bytes = f329({
                        let (_, h3779) = v4;
                        h3779
                    })?;
                    Ok(Ok::<Adt37, Bytes>(Adt37::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f725(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h3780, _) = v4;
                        h3780
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f725(v3))
        }
    }
}

pub fn f728(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f729(v0: Adt26, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt18>, Bytes>> {
    let m3781 = v0;
    match m3781 {
        Adt26::C0(_, _, _) => {
            Ok(Ok::<List<Adt18>, Bytes>(List::<Adt18>::nil()))
        }
        Adt26::C1(_, _, _) => {
            Ok(Ok::<List<Adt18>, Bytes>(List::<Adt18>::nil()))
        }
        Adt26::C2(_) => {
            Ok(Ok::<List<Adt18>, Bytes>(List::<Adt18>::nil()))
        }
        Adt26::C3(r3788, r3789) => {
            let v2 = (*r3788).clone();
            let v3 = (*r3789).clone();
            let m3782 = f729(v3, v1.clone())?;
            match m3782 {
                Ok(v5) => {
                    let m3783 = v2;
                    match m3783 {
                        Adt26::C0(v6, r3787, _) => {
                            let v7 = (*r3787).clone();
                            let m3784 = f491(v7.clone(), v1);
                            match m3784 {
                                Ok(v10) => {
                                    let m3785 = f492(v10, v6)?;
                                    match m3785 {
                                        Ok(v12) => {
                                            if f493(v12, Str::lit("enumeration")) {
                                                let m3786 = f494(v7, f8(Str::lit("value")));
                                                match m3786 {
                                                    None => {
                                                        Ok(f769(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt18>, Bytes>(List::cons(Adt18::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt18>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f769(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f769(v9))
                                }
                            }
                        }
                        Adt26::C1(_, _, _) => {
                            Ok(Ok::<List<Adt18>, Bytes>(v5))
                        }
                        Adt26::C2(_) => {
                            Ok(Ok::<List<Adt18>, Bytes>(v5))
                        }
                        Adt26::C3(_, _) => {
                            Ok(Ok::<List<Adt18>, Bytes>(v5))
                        }
                        Adt26::C4 => {
                            Ok(Ok::<List<Adt18>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f769(v4))
                }
            }
        }
        Adt26::C4 => {
            Ok(Ok::<List<Adt18>, Bytes>(List::<Adt18>::nil()))
        }
    }
}

pub fn f730(v0: List<Adt18>, v1: List<Bytes>) -> R<List<Adt18>> {
    let m3790 = v0;
    match m3790.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f752(v2.clone())?;
            if f41(v4.clone(), v1.clone()) {
                f730(v3, v1)
            } else {
                Ok(List::cons(v2, f730(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f731(p3794: Bytes, p3795: u64) -> R<Result<Bytes, Bytes>> {
    let m3793 = (p3794, p3795);
    {
        let mut m3793 = m3793;
        loop {
            let step = {
                let (v0, v1) = m3793;
                let m3791 = v1;
                if m3791 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f770(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m3791 - 1;
                    if f685(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f28(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f28(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f28(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f771())))
                                } else {
                                    if f28(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f685(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f770(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f685(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f770(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m3792 = f772(v0.clone())?;
                                                match m3792 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f770(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
                                                    }
                                                    Some(v3) => {
                                                        if f28(v3.clone(), f8(Str::lit("anyAtomicType"))) {
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
                Ok(next) => { m3793 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f732(v0: Str, v1: Bytes) -> Bytes {
    f422(f8(Str::lit("(")), f8(v0), f422(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f733(v0: u64, v1: Adt29, v2: Bytes, v3: Bytes, v4: bool, v5: Adt31, v6: Adt38) -> R<Result<Adt38, Bytes>> {
    let m3796 = v0;
    if m3796 == 0 {
        Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m3796 - 1;
        if f497(v5.clone(), Str::lit("element")) {
            let m3797 = f773(v7, v1, v2, v3, v5.clone())?;
            match m3797 {
                Ok(v9) => {
                    let m3803 = f774(v5, {
                        let m3802 = {
                            let m3801 = match v9.clone() {
                                Adt42::C0(_, h3800) => {
                                    h3800
                                }
                            };
                            match m3801 {
                                Adt36::C0(h3799, _) => {
                                    h3799
                                }
                            }
                        };
                        match m3802 {
                            Adt37::C0(h3798, _) => {
                                h3798
                            }
                        }
                    });
                    match m3803 {
                        None => {
                            Ok(Ok::<Adt38, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt38, Bytes>(f736(f677(v6, Adt39::C0(match v9.clone() {
                                Adt42::C0(h3804, _) => {
                                    h3804
                                }
                            }, v10, {
                                let m3809 = {
                                    let m3808 = match v9.clone() {
                                        Adt42::C0(_, h3807) => {
                                            h3807
                                        }
                                    };
                                    match m3808 {
                                        Adt36::C0(h3806, _) => {
                                            h3806
                                        }
                                    }
                                };
                                match m3809 {
                                    Adt37::C0(_, h3805) => {
                                        h3805
                                    }
                                }
                            })), {
                                let m3812 = match v9 {
                                    Adt42::C0(_, h3811) => {
                                        h3811
                                    }
                                };
                                match m3812 {
                                    Adt36::C0(_, h3810) => {
                                        h3810
                                    }
                                }
                            })))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<Adt38, Bytes>(v8))
                }
            }
        } else {
            if f497(v5.clone(), Str::lit("any")) {
                let m3813 = f774(v5, f544());
                match m3813 {
                    None => {
                        Ok(Ok::<Adt38, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt38, Bytes>(f677(v6, Adt39::C0(f8(Str::lit("any")), v11, List::cons(f544(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f497(v5.clone(), Str::lit("group")) {
                    let m3814 = f498(v5.clone(), Str::lit("ref"));
                    match m3814 {
                        None => {
                            Ok(Err::<Adt38, Bytes>(f605(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m3816 = f492(match v5.clone() {
                                Adt31::C0(_, _, _, _, h3815) => {
                                    h3815
                                }
                            }, v12.clone())?;
                            match m3816 {
                                Ok(v14) => {
                                    let m3817 = f675(v1, Str::lit("group"), v14.clone());
                                    match m3817 {
                                        None => {
                                            Ok(Err::<Adt38, Bytes>(f605(f422(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f329({
                                                let (_, h3818) = v14.clone();
                                                h3818
                                            })?;
                                            let m3819 = f774(v5, v16.clone());
                                            match m3819 {
                                                None => {
                                                    Ok(Ok::<Adt38, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt38, Bytes>(f677(v6, Adt39::C0({
                                                        let (_, h3820) = v14;
                                                        h3820
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt38, Bytes>(f605(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f497(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f497(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f497(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f775(v5.clone()) {
                                f496(f497(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m3821 = f603(v5)?;
                            match m3821 {
                                Ok(v19) => {
                                    f670(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt38, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt31::C0(_, h3822, _, _, _) => {
                                    h3822
                                }
                            };
                            let v21: u64 = f776(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a3823 = v21;
                                let a3824 = 1u64;
                                if nat_eq(a3823, a3824) {
                                    Bytes::lit(&[])
                                } else {
                                    f357(v21)?
                                }
                            };
                            let v23: Bytes = f422(v2, f329(v20.clone())?, v22.clone());
                            let m3825 = f606(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m3825 {
                                Ok(v25) => {
                                    let m3826 = f774(v5, v23.clone());
                                    match m3826 {
                                        None => {
                                            Ok(Ok::<Adt38, Bytes>(f777(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt38, Bytes>(f736(f677(f777(v6, v20.clone())?, Adt39::C0({
                                                let a3827 = v20;
                                                let a3828 = v22;
                                                append_bytes(a3827, a3828)
                                            }, v26, List::cons(v23, List::<Bytes>::nil()))), v25)))
                                        }
                                    }
                                }
                                Err(v24) => {
                                    Ok(Err::<Adt38, Bytes>(v24))
                                }
                            }
                        }
                    } else {
                        Ok(Ok::<Adt38, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f734(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3829 = v0;
    match m3829.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3830 = v1;
            match m3830.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f422(f8(Str::lit("    {name: ")), v2, f422(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f734(v3, v5))
                }
            }
        }
    }
}

pub fn f735(p3836: List<Adt30>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt30> {
    let m3835 = p3836;
    {
        let mut m3835 = m3835;
        loop {
            let step = {
                let v0 = m3835;
                let m3831 = v0;
                match m3831.uncons() {
                    None => {
                        Err::<List<Adt30>, Option<Adt30>>(None::<Adt30>)
                    }
                    Some((v4, v5)) => {
                        if if f28(match v4.clone() {
                            Adt30::C0(h3832, _, _, _) => {
                                h3832
                            }
                        }, v1.clone()) {
                            if f28(match v4.clone() {
                                Adt30::C0(_, h3833, _, _) => {
                                    h3833
                                }
                            }, v2.clone()) {
                                f28(match v4.clone() {
                                    Adt30::C0(_, _, h3834, _) => {
                                        h3834
                                    }
                                }, v3.clone())
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt30>, Option<Adt30>>(Some(v4))
                        } else {
                            Ok::<List<Adt30>, Option<Adt30>>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3835 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f736(v0: Adt38, v1: List<Adt32>) -> Adt38 {
    Adt38::C0(match v0.clone() {
        Adt38::C0(h3837, _, _, _, _) => {
            h3837
        }
    }, f541(match v0.clone() {
        Adt38::C0(_, h3838, _, _, _) => {
            h3838
        }
    }, v1), match v0.clone() {
        Adt38::C0(_, _, h3839, _, _) => {
            h3839
        }
    }, match v0.clone() {
        Adt38::C0(_, _, _, h3840, _) => {
            h3840
        }
    }, match v0 {
        Adt38::C0(_, _, _, _, h3841) => {
            h3841
        }
    })
}

pub fn f737(v0: List<Adt39>, v1: List<Adt39>) -> List<Adt39> {
    let m3842 = v0;
    match m3842.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f737(v3, v1))
        }
    }
}

pub fn f738(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3845 = {
        let a3843 = v0.clone();
        let a3844 = v1;
        index_bytes(a3843, a3844)
    };
    match m3845 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            let a3846 = v3;
            let a3847 = 128u64;
            if nat_lt(a3846, a3847) {
                Ok(Some((v3, 1u64)))
            } else {
                if f353({
                    let a3848 = 194u64;
                    let a3849 = v3;
                    nat_le(a3848, a3849)
                }, {
                    let a3850 = v3;
                    let a3851 = 223u64;
                    nat_le(a3850, a3851)
                }) {
                    let m3854 = f778(v0, {
                        let a3852 = v1;
                        let a3853 = 1u64;
                        nat_add(a3852, a3853)?
                    })?;
                    match m3854 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f779({
                                let a3859 = {
                                    let a3857 = {
                                        let a3855 = v3;
                                        let a3856 = 192u64;
                                        nat_sub(a3855, a3856)
                                    };
                                    let a3858 = 64u64;
                                    nat_mul(a3857, a3858)?
                                };
                                let a3860 = v4;
                                nat_add(a3859, a3860)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f353({
                        let a3861 = 224u64;
                        let a3862 = v3;
                        nat_le(a3861, a3862)
                    }, {
                        let a3863 = v3;
                        let a3864 = 239u64;
                        nat_le(a3863, a3864)
                    }) {
                        let m3867 = f778(v0.clone(), {
                            let a3865 = v1;
                            let a3866 = 1u64;
                            nat_add(a3865, a3866)?
                        })?;
                        match m3867 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3870 = f778(v0, {
                                    let a3868 = v1;
                                    let a3869 = 2u64;
                                    nat_add(a3868, a3869)?
                                })?;
                                match m3870 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f779({
                                            let a3879 = {
                                                let a3873 = {
                                                    let a3871 = v3;
                                                    let a3872 = 224u64;
                                                    nat_sub(a3871, a3872)
                                                };
                                                let a3874 = 4096u64;
                                                nat_mul(a3873, a3874)?
                                            };
                                            let a3880 = {
                                                let a3877 = {
                                                    let a3875 = v5;
                                                    let a3876 = 64u64;
                                                    nat_mul(a3875, a3876)?
                                                };
                                                let a3878 = v6;
                                                nat_add(a3877, a3878)?
                                            };
                                            nat_add(a3879, a3880)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f353({
                            let a3881 = 240u64;
                            let a3882 = v3;
                            nat_le(a3881, a3882)
                        }, {
                            let a3883 = v3;
                            let a3884 = 244u64;
                            nat_le(a3883, a3884)
                        }) {
                            let m3887 = f778(v0.clone(), {
                                let a3885 = v1;
                                let a3886 = 1u64;
                                nat_add(a3885, a3886)?
                            })?;
                            match m3887 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3890 = f778(v0.clone(), {
                                        let a3888 = v1;
                                        let a3889 = 2u64;
                                        nat_add(a3888, a3889)?
                                    })?;
                                    match m3890 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3893 = f778(v0, {
                                                let a3891 = v1;
                                                let a3892 = 3u64;
                                                nat_add(a3891, a3892)?
                                            })?;
                                            match m3893 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f779({
                                                        let a3906 = {
                                                            let a3896 = {
                                                                let a3894 = v3;
                                                                let a3895 = 240u64;
                                                                nat_sub(a3894, a3895)
                                                            };
                                                            let a3897 = 262144u64;
                                                            nat_mul(a3896, a3897)?
                                                        };
                                                        let a3907 = {
                                                            let a3904 = {
                                                                let a3898 = v7;
                                                                let a3899 = 4096u64;
                                                                nat_mul(a3898, a3899)?
                                                            };
                                                            let a3905 = {
                                                                let a3902 = {
                                                                    let a3900 = v8;
                                                                    let a3901 = 64u64;
                                                                    nat_mul(a3900, a3901)?
                                                                };
                                                                let a3903 = v9;
                                                                nat_add(a3902, a3903)?
                                                            };
                                                            nat_add(a3904, a3905)?
                                                        };
                                                        nat_add(a3906, a3907)?
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

pub fn f739(v0: u64) -> R<bool> {
    f682(v0, f780())
}

pub fn f740(v0: u64, v1: Bytes, p3918: u64, p3919: u64, p3920: bool) -> R<bool> {
    let m3917 = (p3918, (p3919, p3920));
    {
        let mut m3917 = m3917;
        loop {
            let step = {
                let (v2, (v3, v4)) = m3917;
                let m3908 = v3;
                if m3908 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m3908 - 1;
                    let v6: u64 = f781(v1.clone(), v2)?;
                    if v4 {
                        let a3909 = v0;
                        let a3910 = v6;
                        if nat_lt(a3909, a3910) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3911 = v2;
                                let a3912 = 1u64;
                                nat_add(a3911, a3912)?
                            }, (v5, false)))
                        }
                    } else {
                        let a3913 = v0;
                        let a3914 = v6;
                        if nat_le(a3913, a3914) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a3915 = v2;
                                let a3916 = 1u64;
                                nat_add(a3915, a3916)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f741(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f615(f563(v0, v1)?)? {
        Ok(Some({
            let a3921 = v1;
            let a3922 = 1u64;
            nat_add(a3921, a3922)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f742(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m3923 = f738(v0, v2)?;
    match m3923 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a3924 = v2;
            let a3925 = v1;
            if if nat_eq(a3924, a3925) {
                f782({
                    let (h3926, _) = v3;
                    h3926
                })?
            } else {
                f783({
                    let (h3927, _) = v3;
                    h3927
                })?
            } {
                Ok(Some({
                    let a3929 = v2;
                    let a3930 = {
                        let (_, h3928) = v3;
                        h3928
                    };
                    nat_add(a3929, a3930)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f743(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h3931, _) = f508(Fn15::F784(v1, v0.clone()), {
        let a3933 = {
            let a3932 = v0;
            length_bytes(a3932)
        };
        let a3934 = 1u64;
        nat_add(a3933, a3934)?
    }, (v2, 0u64))?;
    Ok(h3931)
}

pub fn f744(v0: u64) -> R<Bytes> {
    let a3935 = v0;
    let a3936 = 128u64;
    if nat_lt(a3935, a3936) {
        Ok(f785(f786(), v0))
    } else {
        let a3937 = v0;
        let a3938 = 2048u64;
        if nat_lt(a3937, a3938) {
            let a3946 = f785(f787(), {
                let a3944 = {
                    let a3942 = {
                        let a3939 = v0;
                        let a3940 = 64u64;
                        let a3941 = 0u64;
                        nat_quot(a3939, a3940, a3941)
                    };
                    let a3943 = 2u64;
                    nat_sub(a3942, a3943)
                };
                let a3945 = 2u64;
                nat_mul(a3944, a3945)?
            });
            let a3947 = f788(v0)?;
            Ok(append_bytes(a3946, a3947))
        } else {
            let a3948 = v0;
            let a3949 = 65536u64;
            if nat_lt(a3948, a3949) {
                let a3960 = f785(f789(), {
                    let a3953 = {
                        let a3950 = v0;
                        let a3951 = 4096u64;
                        let a3952 = 0u64;
                        nat_quot(a3950, a3951, a3952)
                    };
                    let a3954 = 3u64;
                    nat_mul(a3953, a3954)?
                });
                let a3961 = {
                    let a3958 = f788({
                        let a3955 = v0;
                        let a3956 = 64u64;
                        let a3957 = 0u64;
                        nat_quot(a3955, a3956, a3957)
                    })?;
                    let a3959 = f788(v0)?;
                    append_bytes(a3958, a3959)
                };
                Ok(append_bytes(a3960, a3961))
            } else {
                let a3975 = f785(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a3962 = v0;
                    let a3963 = 262144u64;
                    let a3964 = 0u64;
                    nat_quot(a3962, a3963, a3964)
                });
                let a3976 = {
                    let a3973 = f788({
                        let a3965 = v0;
                        let a3966 = 4096u64;
                        let a3967 = 0u64;
                        nat_quot(a3965, a3966, a3967)
                    })?;
                    let a3974 = {
                        let a3971 = f788({
                            let a3968 = v0;
                            let a3969 = 64u64;
                            let a3970 = 0u64;
                            nat_quot(a3968, a3969, a3970)
                        })?;
                        let a3972 = f788(v0)?;
                        append_bytes(a3971, a3972)
                    };
                    append_bytes(a3973, a3974)
                };
                Ok(append_bytes(a3975, a3976))
            }
        }
    }
}

pub fn f745(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f790(v2, v0, v1, v3)
}

pub fn f746(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3977 = v0.clone();
        length_bytes(a3977)
    };
    let a3981 = {
        let a3978 = v1;
        let a3979 = 2u64;
        let a3980 = 0u64;
        nat_rem(a3978, a3979, a3980)
    };
    let a3982 = 0u64;
    if nat_eq(a3981, a3982) {
        let (_, h3983) = {
            let (h3984, _) = f791(Fn24::F792(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3984
        };
        Ok(h3983)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f747(v0: u64) -> bool {
    let a3985 = v0;
    let a3986 = 8u64;
    if nat_eq(a3985, a3986) {
        true
    } else {
        let a3987 = v0;
        let a3988 = 9u64;
        if nat_eq(a3987, a3988) {
            true
        } else {
            let a3989 = v0;
            let a3990 = 10u64;
            if nat_eq(a3989, a3990) {
                true
            } else {
                let a3991 = v0;
                let a3992 = 12u64;
                if nat_eq(a3991, a3992) {
                    true
                } else {
                    let a3993 = v0;
                    let a3994 = 13u64;
                    nat_eq(a3993, a3994)
                }
            }
        }
    }
}

pub fn f748(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3995, _) = v0;
        h3995
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3996) = v0;
        h3996
    };
    let m3997 = v1;
    match m3997.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3998 = v4;
            match m3998.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f793(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f749(v0: Fn23, p4003: u64, p4004: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4002 = (p4003, p4004);
    {
        let mut m4002 = m4002;
        loop {
            let step = {
                let (v1, v2) = m4002;
                let m3999 = v1;
                if m3999 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3999 - 1;
                    let m4001 = {
                        let c4000: Fn23 = v0.clone();
                        c4000.apply(v2.clone())
                    };
                    match m4001 {
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
                Ok(next) => { m4002 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f750(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f794(v0)
}

pub fn f751(v0: bool, v1: Adt18) -> R<Option<Bytes>> {
    let m4005 = v1;
    match m4005 {
        Adt18::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C1(v7, v8) => {
            Ok(Some({
                let a4016 = if f353(v7, {
                    let a4014 = 0u64;
                    let a4015 = v8;
                    nat_lt(a4014, a4015)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a4017 = f267(false, v8)?;
                append_bytes(a4016, a4017)
            }))
        }
        Adt18::C2(v3, v4, v5) => {
            let m4008 = f795({
                let a4006 = f8(Str::lit("n"));
                let a4007 = f406(false, v4, v5)?;
                append_bytes(a4006, a4007)
            })?;
            match m4008 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a4012 = f8(Str::lit("nMinus"));
                        let a4013 = f35(v6.clone(), 1u64, {
                            let a4010 = {
                                let a4009 = v6;
                                length_bytes(a4009)
                            };
                            let a4011 = 1u64;
                            nat_sub(a4010, a4011)
                        });
                        append_bytes(a4012, a4013)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt18::C3(v2) => {
            f757(v0, v2)
        }
        Adt18::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C6 => {
            Ok(None::<Bytes>)
        }
        Adt18::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt18::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f752(v0: Adt18) -> R<Bytes> {
    f333(v0)
}

pub fn f753(v0: Bytes) -> R<Option<Bytes>> {
    let m4018 = f796(v0)?;
    match m4018 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f797(v1)?))
        }
    }
}

pub fn f754(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m4019 = v1;
    match m4019.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m4020 = f757(v0, v2)?;
            match m4020 {
                None => {
                    f754(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f754(v0, v3)?))
                }
            }
        }
    }
}

pub fn f755(p4024: List<Adt18>, v1: Bytes) -> bool {
    let m4023 = p4024;
    {
        let mut m4023 = m4023;
        loop {
            let step = {
                let v0 = m4023;
                let m4021 = v0;
                match m4021.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m4022 = f765(v2, v1.clone());
                        if match m4022 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt18>, bool>(v3)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4023 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f756(v0: Adt18) -> R<Option<Bytes>> {
    let m4025 = f334(f242(v0.clone(), f8(Str::lit("$ref"))));
    match m4025 {
        None => {
            Ok(f334(f242(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f633(v1)
        }
    }
}

pub fn f757(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f795(v1)
    } else {
        f753(v1)
    }
}

pub fn f758(v0: Adt18, v1: Bytes) -> R<Result<Adt35, Bytes>> {
    f798(f591(v0), v1)
}

pub fn f759(v0: bool, v1: Bytes, v2: List<Adt18>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt40, Bytes>> {
    let m4026 = v2;
    match m4026.uncons() {
        None => {
            Ok(Ok::<Adt40, Bytes>(Adt40::C0(List::<Bytes>::nil(), List::<Adt28>::nil())))
        }
        Some((v5, v6)) => {
            let m4027 = f765(v5.clone(), v3.clone());
            match m4027 {
                None => {
                    Ok(Err::<Adt40, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m4028 = f757(v0, v7.clone())?;
                    match m4028 {
                        None => {
                            Ok(Err::<Adt40, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f41(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt40, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4031 = f640({
                                    let a4029 = v1.clone();
                                    let a4030 = f329(v7)?;
                                    append_bytes(a4029, a4030)
                                }, f527(f463(f242(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f246(f247(f242(v5, f8(Str::lit("required"))))), true)?;
                                match m4031 {
                                    Ok(v10) => {
                                        let m4032 = f759(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m4032 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt40, Bytes>(Adt40::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f342(f641(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f641(v12.clone())), f467(f642(v10), f642(v12)))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt40, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt40, Bytes>(v9))
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

pub fn f760(v0: List<Adt18>, v1: Bytes) -> List<Bytes> {
    let m4033 = v0;
    match m4033.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4034 = f765(v2, v1.clone());
            match m4034 {
                None => {
                    f760(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f760(v3, v1))
                }
            }
        }
    }
}

pub fn f761(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m4035 = f757(v0, v1)?;
    match m4035 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f28(v2.clone(), f8(Str::lit("mk"))) {
                f589(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f762(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m4036 = v1;
    match m4036.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m4037 = f761(v0, v2)?;
            match m4037 {
                None => {
                    f762(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f762(v0, v3)?))
                }
            }
        }
    }
}

pub fn f763(v0: Adt18) -> Option<Adt18> {
    let m4038 = f463(f242(v0, f8(Str::lit("patternProperties"))));
    match m4038.uncons() {
        None => {
            None::<Adt18>
        }
        Some((v1, v2)) => {
            let m4039 = v2;
            match m4039.uncons() {
                None => {
                    Some({
                        let (_, h4040) = v1;
                        h4040
                    })
                }
                Some((_, _)) => {
                    None::<Adt18>
                }
            }
        }
    }
}

pub fn f764(v0: Adt18) -> bool {
    f28(f245(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f765(v0: Adt18, v1: Bytes) -> Option<Bytes> {
    if f41(v1.clone(), f246(f247(f242(v0.clone(), f8(Str::lit("required")))))) {
        let m4041 = f242(v0, f8(Str::lit("properties")));
        match m4041 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m4042 = f242(v2, v1);
                match m4042 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f334(f242(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f766(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m4043 = v1;
    match m4043.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m4044 = f757(v0, v3.clone())?;
            match m4044 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f422(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f41(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f422(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m4045 = f766(v0, v4, List::cons(v5.clone(), v2))?;
                        match m4045 {
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

pub fn f767(v0: Adt38) -> Adt38 {
    Adt38::C0(match v0.clone() {
        Adt38::C0(h4046, _, _, _, _) => {
            h4046
        }
    }, match v0 {
        Adt38::C0(_, h4047, _, _, _) => {
            h4047
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f768(v0: Bytes) -> Result<Option<Adt41>, Bytes> {
    Err::<Option<Adt41>, Bytes>(v0)
}

pub fn f769(v0: Bytes) -> Result<List<Adt18>, Bytes> {
    Err::<List<Adt18>, Bytes>(v0)
}

pub fn f770(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f771() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f772(v0: Bytes) -> R<Option<Bytes>> {
    let m4048 = f571(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m4048 {
        None => {
            let m4049 = f571(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m4049 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m4050 = f571(f799(), v3.clone(), 0u64)?;
                    match m4050 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a4052 = v4;
                                let a4053 = {
                                    let a4051 = v3;
                                    length_bytes(a4051)
                                };
                                nat_add(a4052, a4053)?
                            };
                            let m4054 = f571(f799(), f8(Str::lit(" ")), v5)?;
                            match m4054 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f35(f799(), v5, {
                                        let a4055 = v6;
                                        let a4056 = v5;
                                        nat_sub(a4055, a4056)
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

pub fn f773(v0: u64, v1: Adt29, v2: Bytes, v3: Bytes, v4: Adt31) -> R<Result<Adt42, Bytes>> {
    let m4057 = v0;
    if m4057 == 0 {
        Ok(Err::<Adt42, Bytes>(f605(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4057 - 1;
        let m4058 = f498(v4.clone(), Str::lit("ref"));
        match m4058 {
            None => {
                let m4069 = f498(v4.clone(), Str::lit("name"));
                match m4069 {
                    None => {
                        Ok(Err::<Adt42, Bytes>(f605(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m4072 = f601(v5, v1, {
                            let a4070 = v2;
                            let a4071 = f329(v12.clone())?;
                            append_bytes(a4070, a4071)
                        }, v3, v4)?;
                        match m4072 {
                            Ok(v14) => {
                                Ok(Ok::<Adt42, Bytes>(Adt42::C0(v12, v14)))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt42, Bytes>(v13))
                            }
                        }
                    }
                }
            }
            Some(v6) => {
                let m4060 = f492(match v4 {
                    Adt31::C0(_, _, _, _, h4059) => {
                        h4059
                    }
                }, v6.clone())?;
                match m4060 {
                    Ok(v8) => {
                        let m4061 = f675(v1.clone(), Str::lit("element"), v8.clone());
                        match m4061 {
                            None => {
                                Ok(Err::<Adt42, Bytes>(f605(f422(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f661(match v9.clone() {
                                    Adt30::C0(_, _, _, h4062) => {
                                        h4062
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f800(match v1.clone() {
                                        Adt29::C0(h4063, _) => {
                                            h4063
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt42, Bytes>(f605(f422(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m4067 = f601(v5, v1, f329({
                                        let (_, h4064) = v8.clone();
                                        h4064
                                    })?, match v9.clone() {
                                        Adt30::C0(_, h4065, _, _) => {
                                            h4065
                                        }
                                    }, match v9 {
                                        Adt30::C0(_, _, _, h4066) => {
                                            h4066
                                        }
                                    })?;
                                    match m4067 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt42, Bytes>(Adt42::C0({
                                                let (_, h4068) = v8;
                                                h4068
                                            }, v11)))
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt42, Bytes>(v10))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt42, Bytes>(f605(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f774(v0: Adt31, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f501(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f28(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f28(v2, f8(Str::lit("1"))) {
            if f661(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f732(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f732(Str::lit("List"), v1))
        }
    }
}

pub fn f775(v0: Adt31) -> bool {
    if f28(f501(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f28(f501(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f776(v0: Adt38, v1: Bytes) -> R<u64> {
    if f28(v1.clone(), f8(Str::lit("sequence"))) {
        let a4074 = match v0 {
            Adt38::C0(_, _, h4073, _, _) => {
                h4073
            }
        };
        let a4075 = 1u64;
        nat_add(a4074, a4075)
    } else {
        if f28(v1, f8(Str::lit("choice"))) {
            let a4077 = match v0 {
                Adt38::C0(_, _, _, h4076, _) => {
                    h4076
                }
            };
            let a4078 = 1u64;
            nat_add(a4077, a4078)
        } else {
            let a4080 = match v0 {
                Adt38::C0(_, _, _, _, h4079) => {
                    h4079
                }
            };
            let a4081 = 1u64;
            nat_add(a4080, a4081)
        }
    }
}

pub fn f777(v0: Adt38, v1: Bytes) -> R<Adt38> {
    if f28(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt38::C0(match v0.clone() {
            Adt38::C0(h4082, _, _, _, _) => {
                h4082
            }
        }, match v0.clone() {
            Adt38::C0(_, h4083, _, _, _) => {
                h4083
            }
        }, {
            let a4085 = match v0.clone() {
                Adt38::C0(_, _, h4084, _, _) => {
                    h4084
                }
            };
            let a4086 = 1u64;
            nat_add(a4085, a4086)?
        }, match v0.clone() {
            Adt38::C0(_, _, _, h4087, _) => {
                h4087
            }
        }, match v0 {
            Adt38::C0(_, _, _, _, h4088) => {
                h4088
            }
        }))
    } else {
        if f28(v1, f8(Str::lit("choice"))) {
            Ok(Adt38::C0(match v0.clone() {
                Adt38::C0(h4089, _, _, _, _) => {
                    h4089
                }
            }, match v0.clone() {
                Adt38::C0(_, h4090, _, _, _) => {
                    h4090
                }
            }, match v0.clone() {
                Adt38::C0(_, _, h4091, _, _) => {
                    h4091
                }
            }, {
                let a4093 = match v0.clone() {
                    Adt38::C0(_, _, _, h4092, _) => {
                        h4092
                    }
                };
                let a4094 = 1u64;
                nat_add(a4093, a4094)?
            }, match v0 {
                Adt38::C0(_, _, _, _, h4095) => {
                    h4095
                }
            }))
        } else {
            Ok(Adt38::C0(match v0.clone() {
                Adt38::C0(h4096, _, _, _, _) => {
                    h4096
                }
            }, match v0.clone() {
                Adt38::C0(_, h4097, _, _, _) => {
                    h4097
                }
            }, match v0.clone() {
                Adt38::C0(_, _, h4098, _, _) => {
                    h4098
                }
            }, match v0.clone() {
                Adt38::C0(_, _, _, h4099, _) => {
                    h4099
                }
            }, {
                let a4101 = match v0 {
                    Adt38::C0(_, _, _, _, h4100) => {
                        h4100
                    }
                };
                let a4102 = 1u64;
                nat_add(a4101, a4102)?
            }))
        }
    }
}

pub fn f778(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m4105 = {
        let a4103 = v0;
        let a4104 = v1;
        index_bytes(a4103, a4104)
    };
    match m4105 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f353({
                let a4106 = 128u64;
                let a4107 = v3;
                nat_le(a4106, a4107)
            }, {
                let a4108 = v3;
                let a4109 = 191u64;
                nat_le(a4108, a4109)
            }) {
                Ok(Some({
                    let a4110 = v3;
                    let a4111 = 128u64;
                    nat_sub(a4110, a4111)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f779(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f353({
        let a4112 = v1;
        let a4113 = v0;
        nat_le(a4112, a4113)
    }, f353({
        let a4114 = v0;
        let a4115 = v2;
        nat_le(a4114, a4115)
    }, f435({
        let a4116 = v0;
        let a4117 = 55296u64;
        nat_lt(a4116, a4117)
    }, {
        let a4118 = 57343u64;
        let a4119 = v0;
        nat_lt(a4118, a4119)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f780() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f781(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a4120 = v1;
        let a4121 = 3u64;
        nat_mul(a4120, a4121)?
    };
    let a4132 = {
        let a4128 = {
            let a4122 = f351(v0.clone(), v2)?;
            let a4123 = 16384u64;
            nat_mul(a4122, a4123)?
        };
        let a4129 = {
            let a4126 = f351(v0.clone(), {
                let a4124 = v2;
                let a4125 = 1u64;
                nat_add(a4124, a4125)?
            })?;
            let a4127 = 128u64;
            nat_mul(a4126, a4127)?
        };
        nat_add(a4128, a4129)?
    };
    let a4133 = f351(v0, {
        let a4130 = v2;
        let a4131 = 2u64;
        nat_add(a4130, a4131)?
    })?;
    nat_add(a4132, a4133)
}

pub fn f782(v0: u64) -> R<bool> {
    f682(v0, f801())
}

pub fn f783(v0: u64) -> R<bool> {
    f682(v0, f802())
}

pub fn f784(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f803(v1, v0, v2)
}

pub fn f785(v0: Bytes, v1: u64) -> Bytes {
    f35(v0, v1, 1u64)
}

pub fn f786() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f787() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f788(v0: u64) -> R<Bytes> {
    Ok(f785(f804(), {
        let a4139 = {
            let a4137 = {
                let a4134 = v0;
                let a4135 = 64u64;
                let a4136 = 0u64;
                nat_rem(a4134, a4135, a4136)
            };
            let a4138 = 2u64;
            nat_mul(a4137, a4138)?
        };
        let a4140 = 1u64;
        nat_add(a4139, a4140)?
    }))
}

pub fn f789() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f790(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f805(f563(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4141 = v3;
            let a4142 = 1u64;
            nat_add(a4141, a4142)?
        }))
    }
}

pub fn f791(v0: Fn24, p4147: u64, p4148: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4146 = (p4147, p4148);
    {
        let mut m4146 = m4146;
        loop {
            let step = {
                let (v1, v2) = m4146;
                let m4143 = v1;
                if m4143 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4143 - 1;
                    let m4145 = {
                        let c4144: Fn24 = v0.clone();
                        c4144.apply(v2.clone())?
                    };
                    match m4145 {
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
                Ok(next) => { m4146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f792(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f806(v0, v1)
}

pub fn f793(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4153 = {
            let a4151 = {
                let a4149 = v0.clone();
                length_list(a4149)
            };
            let a4152 = {
                let a4150 = v1.clone();
                length_list(a4150)
            };
            nat_add(a4151, a4152)?
        };
        let a4154 = 1u64;
        nat_add(a4153, a4154)?
    };
    Ok(f807({
        let (_, h4155) = {
            let (_, h4156) = {
                let (h4157, _) = f808(Fn25::F809, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4157
            };
            h4156
        };
        h4155
    }))
}

pub fn f794(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4158, _) = v0.clone();
        h4158
    };
    let v2: List<(Bytes, ())> = {
        let (h4159, _) = {
            let (_, h4160) = v0.clone();
            h4160
        };
        h4159
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4161) = {
            let (_, h4162) = v0;
            h4162
        };
        h4161
    };
    let m4163 = v1.clone();
    match m4163.uncons() {
        None => {
            let m4164 = v2;
            match m4164.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4165 = v2.clone();
            match m4165.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f810({
                        let (h4166, _) = v8.clone();
                        h4166
                    }, {
                        let (h4167, _) = v6.clone();
                        h4167
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

pub fn f795(v0: Bytes) -> R<Option<Bytes>> {
    let m4169 = f811({
        let a4168 = v0.clone();
        length_bytes(a4168)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m4169 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a4171 = {
                let a4170 = v1.clone();
                length_bytes(a4170)
            };
            let a4172 = 0u64;
            if nat_eq(a4171, a4172) {
                let a4174 = {
                    let a4173 = v0;
                    length_bytes(a4173)
                };
                let a4175 = 0u64;
                if nat_eq(a4174, a4175) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f797(v1)?))
            }
        }
    }
}

pub fn f796(v0: Bytes) -> R<Option<Bytes>> {
    if f812({
        let a4176 = v0.clone();
        length_bytes(a4176)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f813(v0)?;
        if f814(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m4179 = {
                let a4177 = v1.clone();
                let a4178 = 0u64;
                index_bytes(a4177, a4178)
            };
            match m4179 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f461(f62(v2)?) {
                        Ok(Some({
                            let a4180 = f8(Str::lit("n"));
                            let a4181 = v1;
                            append_bytes(a4180, a4181)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f815({
            let a4182 = v0.clone();
            length_bytes(a4182)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f797(v0: Bytes) -> R<Bytes> {
    if f41(v0.clone(), f816()) {
        f589(v0)
    } else {
        Ok(v0)
    }
}

pub fn f798(v0: Adt18, v1: Bytes) -> R<Result<Adt35, Bytes>> {
    let m4183 = v0.clone();
    match m4183 {
        Adt18::C0(v2) => {
            if v2 {
                Ok(f712())
            } else {
                Ok(f636(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt18::C1(_, _) => {
            Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C2(_, _, _) => {
            Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C3(_) => {
            Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C4(_) => {
            Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C5(_) => {
            let m4184 = f817(f525(v0.clone()));
            match m4184 {
                None => {
                    let m4185 = f334(f242(v0.clone(), f8(Str::lit("$ref"))));
                    match m4185 {
                        None => {
                            let m4187 = f242(v0.clone(), f8(Str::lit("allOf")));
                            match m4187 {
                                None => {
                                    let m4192 = f242(v0.clone(), f8(Str::lit("enum")));
                                    match m4192 {
                                        None => {
                                            let m4193 = f242(v0.clone(), f8(Str::lit("const")));
                                            match m4193 {
                                                None => {
                                                    let m4195 = f242(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4195 {
                                                        None => {
                                                            let v30: Bytes = f245(v0.clone(), f8(Str::lit("type")));
                                                            if if f28(v30.clone(), Bytes::lit(&[])) {
                                                                !f818(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f712())
                                                            } else {
                                                                if f28(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f637(f8(Str::lit("Bytes")), List::<Adt28>::nil()))
                                                                } else {
                                                                    if f28(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f637(f819(v0), List::<Adt28>::nil()))
                                                                    } else {
                                                                        if f28(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f637(f771(), List::<Adt28>::nil()))
                                                                        } else {
                                                                            if f28(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f637(f8(Str::lit("Bool")), List::<Adt28>::nil()))
                                                                            } else {
                                                                                if f28(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m4196 = f463(f242(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4196.uncons() {
                                                                                        None => {
                                                                                            if f643(v0.clone()) {
                                                                                                Ok(f637(v1.clone(), List::cons(Adt28::C0(v1, v0), List::<Adt28>::nil())))
                                                                                            } else {
                                                                                                let m4197 = f707(v0);
                                                                                                match m4197 {
                                                                                                    None => {
                                                                                                        Ok(f710(f8(Str::lit("List")), f820(f712())))
                                                                                                    }
                                                                                                    Some(v33) => {
                                                                                                        Ok(f710(f8(Str::lit("List")), f820(f821(v33, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f637(v1.clone(), List::cons(Adt28::C0(v1, v0), List::<Adt28>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f636(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f637(v1.clone(), List::cons(Adt28::C0(v1, v0), List::<Adt28>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m4194 = f334(Some(v27));
                                                    match m4194 {
                                                        None => {
                                                            Ok(f636(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f637(f8(Str::lit("Bytes")), List::<Adt28>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f637(v1.clone(), List::cons(Adt28::C0(v1, v0), List::<Adt28>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m4188 = f335(v19);
                                    match m4188.uncons() {
                                        None => {
                                            Ok(f636(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m4189 = v21;
                                            match m4189.uncons() {
                                                None => {
                                                    let m4190 = f334(f242(v20, f8(Str::lit("$ref"))));
                                                    match m4190 {
                                                        None => {
                                                            Ok(f636(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m4191 = f633(v24)?;
                                                            match m4191 {
                                                                None => {
                                                                    Ok(f636(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f637(v25, List::<Adt28>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f636(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m4186 = f633(v17)?;
                            match m4186 {
                                None => {
                                    Ok(f636(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f637(v18, List::<Adt28>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f636(if f28(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt18::C6 => {
            Ok(f712())
        }
        Adt18::C7(_, _) => {
            Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C8(_, _, _) => {
            Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f799() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f800(p4206: List<Adt30>, v1: (Bytes, Bytes)) -> R<bool> {
    let m4205 = p4206;
    {
        let mut m4205 = m4205;
        loop {
            let step = {
                let v0 = m4205;
                let m4198 = v0;
                match m4198.uncons() {
                    None => {
                        Err::<List<Adt30>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f28(match v2.clone() {
                            Adt30::C0(h4199, _, _, _) => {
                                h4199
                            }
                        }, f8(Str::lit("element"))) {
                            let m4201 = f498(match v2.clone() {
                                Adt30::C0(_, _, _, h4200) => {
                                    h4200
                                }
                            }, Str::lit("substitutionGroup"));
                            match m4201 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f822({
                                        let m4204 = match v2 {
                                            Adt30::C0(_, _, _, h4203) => {
                                                h4203
                                            }
                                        };
                                        match m4204 {
                                            Adt31::C0(_, _, _, _, h4202) => {
                                                h4202
                                            }
                                        }
                                    }, f823(v4)?, v1.clone())?
                                }
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt30>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt30>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4205 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f801() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f802() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f803(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f563(v0, {
        let (h4207, _) = v2;
        h4207
    })?;
    let a4209 = 1114111u64;
    let a4210 = {
        let (_, h4208) = v2;
        h4208
    };
    if nat_lt(a4209, a4210) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f824(v3, v1);
        let a4211 = v4;
        let a4212 = 16u64;
        if nat_eq(a4211, a4212) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a4214 = {
                    let (h4213, _) = v2;
                    h4213
                };
                let a4215 = 1u64;
                nat_add(a4214, a4215)?
            }, {
                let a4219 = {
                    let a4217 = {
                        let (_, h4216) = v2;
                        h4216
                    };
                    let a4218 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a4217, a4218)?
                };
                let a4220 = v4;
                nat_add(a4219, a4220)?
            })))
        }
    }
}

pub fn f804() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f805(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a4221 = v0;
        let a4222 = v2;
        if nat_eq(a4221, a4222) {
            Ok(true)
        } else {
            f682(v0, f825())
        }
    } else {
        f682(v0, f826())
    }
}

pub fn f806(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4223, _) = v1;
        h4223
    };
    let m4225 = {
        let (_, h4224) = v1;
        h4224
    };
    match m4225 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4228 = {
                let a4226 = v0.clone();
                let a4227 = v2;
                index_bytes(a4226, a4227)
            };
            match m4228 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4233 = {
                        let a4231 = v0;
                        let a4232 = {
                            let a4229 = v2;
                            let a4230 = 1u64;
                            nat_add(a4229, a4230)?
                        };
                        index_bytes(a4231, a4232)
                    };
                    match m4233 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f827(v4)? {
                                f827(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4234 = v2;
                                    let a4235 = 2u64;
                                    nat_add(a4234, a4235)?
                                }, Some({
                                    let a4240 = v3;
                                    let a4241 = f511(f828(), {
                                        let a4238 = {
                                            let a4236 = f829(v4)?;
                                            let a4237 = 16u64;
                                            nat_mul(a4236, a4237)?
                                        };
                                        let a4239 = f829(v5)?;
                                        nat_add(a4238, a4239)?
                                    });
                                    append_bytes(a4240, a4241)
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

pub fn f807(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f360(Fn5::F830, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f808(v0: Fn25, p4246: u64, p4247: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4245 = (p4246, p4247);
    {
        let mut m4245 = m4245;
        loop {
            let step = {
                let (v1, v2) = m4245;
                let m4242 = v1;
                if m4242 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4242 - 1;
                    let m4244 = {
                        let c4243: Fn25 = v0.clone();
                        c4243.apply(v2.clone())
                    };
                    match m4244 {
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
                Ok(next) => { m4245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f809(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f831(v0)
}

pub fn f810(v0: Bytes, v1: Bytes) -> bool {
    let a4252 = {
        let a4248 = v0;
        let a4249 = v1;
        compare_bytes(a4248, a4249)
    };
    let a4253 = {
        let a4250 = Bytes::lit(&[]);
        let a4251 = Bytes::lit(&[0x01]);
        compare_bytes(a4250, a4251)
    };
    equal(a4252, a4253)
}

pub fn f811(p4266: u64, v1: Bytes, p4267: u64, p4268: Bytes) -> R<Option<Bytes>> {
    let m4265 = (p4266, (p4267, p4268));
    {
        let mut m4265 = m4265;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4265;
                let m4254 = v0;
                if m4254 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4254 - 1;
                    let a4256 = {
                        let a4255 = v1.clone();
                        length_bytes(a4255)
                    };
                    let a4257 = v2;
                    if nat_le(a4256, a4257) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4258 = f738(v1.clone(), v2)?;
                        match m4258 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4260 = v2;
                                    let a4261 = {
                                        let (_, h4259) = v5;
                                        h4259
                                    };
                                    nat_add(a4260, a4261)?
                                }, {
                                    let a4263 = v3;
                                    let a4264 = f832(v1.clone(), v2, {
                                        let (h4262, _) = v5;
                                        h4262
                                    });
                                    append_bytes(a4263, a4264)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f812(p4278: u64, v1: Bytes, p4279: u64) -> R<bool> {
    let m4277 = (p4278, p4279);
    {
        let mut m4277 = m4277;
        loop {
            let step = {
                let (v0, v2) = m4277;
                let m4269 = v0;
                if m4269 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4269 - 1;
                    let m4272 = {
                        let a4270 = v1.clone();
                        let a4271 = v2;
                        index_bytes(a4270, a4271)
                    };
                    match m4272 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4273 = f62(v4)?;
                            let a4274 = 128u64;
                            if nat_lt(a4273, a4274) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4275 = v2;
                                    let a4276 = 1u64;
                                    nat_add(a4275, a4276)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f813(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f404(v0.clone(), 0u64, {
        let a4280 = v0;
        length_bytes(a4280)
    }, false, Bytes::lit(&[]))?;
    let m4283 = {
        let a4281 = v1.clone();
        let a4282 = 0u64;
        index_bytes(a4281, a4282)
    };
    match m4283 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f460(v3) {
                let a4290 = f35({
                    let a4284 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4284)
                }, {
                    let a4285 = v3;
                    let a4286 = 65u64;
                    nat_sub(a4285, a4286)
                }, 1u64);
                let a4291 = f35(v1.clone(), 1u64, {
                    let a4288 = {
                        let a4287 = v1;
                        length_bytes(a4287)
                    };
                    let a4289 = 1u64;
                    nat_sub(a4288, a4289)
                });
                Ok(append_bytes(a4290, a4291))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f814(v0: Bytes) -> R<bool> {
    let m4294 = {
        let a4292 = v0;
        let a4293 = 0u64;
        index_bytes(a4292, a4293)
    };
    match m4294 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f435(f459(f62(v1)?), f460(f62(v1)?)))
        }
    }
}

pub fn f815(p4307: u64, v1: Bytes, p4308: u64, p4309: Bytes) -> R<Option<Bytes>> {
    let m4306 = (p4307, (p4308, p4309));
    {
        let mut m4306 = m4306;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4306;
                let m4295 = v0;
                if m4295 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4295 - 1;
                    let a4297 = {
                        let a4296 = v1.clone();
                        length_bytes(a4296)
                    };
                    let a4298 = v2;
                    if nat_le(a4297, a4298) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4299 = f738(v1.clone(), v2)?;
                        match m4299 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4301 = v2;
                                    let a4302 = {
                                        let (_, h4300) = v5;
                                        h4300
                                    };
                                    nat_add(a4301, a4302)?
                                }, {
                                    let a4304 = v3;
                                    let a4305 = f833({
                                        let (h4303, _) = v5;
                                        h4303
                                    });
                                    append_bytes(a4304, a4305)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f816() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f817(p4314: List<(Bytes, Adt18)>) -> Option<Bytes> {
    let m4313 = p4314;
    {
        let mut m4313 = m4313;
        loop {
            let step = {
                let v0 = m4313;
                let m4310 = v0;
                match m4310.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h4311, _) = v1.clone();
                            h4311
                        }, f834()) {
                            Err::<List<(Bytes, Adt18)>, Option<Bytes>>(Some({
                                let (h4312, _) = v1;
                                h4312
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt18)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f818(v0: Adt18) -> bool {
    if f648(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f648(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f648(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f648(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f648(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f648(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f819(v0: Adt18) -> Bytes {
    let m4315 = f835(f242(v0, f8(Str::lit("minimum"))));
    match m4315 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f820(v0: Result<Adt35, Bytes>) -> Result<Adt35, Bytes> {
    let m4316 = v0;
    match m4316 {
        Ok(v2) => {
            f637(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f533(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f532(v2))
        }
        Err(v1) => {
            Err::<Adt35, Bytes>(v1)
        }
    }
}

pub fn f821(v0: Adt18, v1: Bytes) -> R<Result<Adt35, Bytes>> {
    f836(16u64, v0, v1)
}

pub fn f822(v0: List<(Bytes, Bytes)>, p4324: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m4323 = p4324;
    {
        let mut m4323 = m4323;
        loop {
            let step = {
                let v1 = m4323;
                let m4317 = v1;
                match m4317.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m4318 = f492(v0.clone(), v3)?;
                        match m4318 {
                            Ok(v6) => {
                                if if f28({
                                    let (h4319, _) = v6;
                                    h4319
                                }, {
                                    let (h4320, _) = v2.clone();
                                    h4320
                                }) {
                                    f28({
                                        let (_, h4321) = v6;
                                        h4321
                                    }, {
                                        let (_, h4322) = v2.clone();
                                        h4322
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
                Ok(next) => { m4323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f823(v0: Bytes) -> R<List<Bytes>> {
    f33(v0, 32u8)
}

pub fn f824(v0: u64, v1: bool) -> u64 {
    let a4325 = v0;
    let a4326 = 48u64;
    if nat_lt(a4325, a4326) {
        16u64
    } else {
        let a4327 = v0;
        let a4328 = 57u64;
        if nat_le(a4327, a4328) {
            let a4329 = v0;
            let a4330 = 48u64;
            nat_sub(a4329, a4330)
        } else {
            if v1 {
                let a4331 = v0;
                let a4332 = 65u64;
                if nat_lt(a4331, a4332) {
                    16u64
                } else {
                    let a4333 = v0;
                    let a4334 = 70u64;
                    if nat_le(a4333, a4334) {
                        let a4335 = v0;
                        let a4336 = 55u64;
                        nat_sub(a4335, a4336)
                    } else {
                        let a4337 = v0;
                        let a4338 = 97u64;
                        if nat_lt(a4337, a4338) {
                            16u64
                        } else {
                            let a4339 = v0;
                            let a4340 = 102u64;
                            if nat_le(a4339, a4340) {
                                let a4341 = v0;
                                let a4342 = 87u64;
                                nat_sub(a4341, a4342)
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

pub fn f825() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f826() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f827(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4387 = {
            let a4363 = {
                let a4351 = {
                    let a4345 = {
                        let a4343 = v1;
                        let a4344 = 128u8;
                        fixed_u8::bit_and(a4343, a4344)
                    };
                    let a4346 = 128u8;
                    if equal(a4345, a4346) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4352 = {
                    let a4349 = {
                        let a4347 = v1;
                        let a4348 = 64u8;
                        fixed_u8::bit_and(a4347, a4348)
                    };
                    let a4350 = 64u8;
                    if equal(a4349, a4350) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4351, a4352)?
            };
            let a4364 = {
                let a4361 = {
                    let a4355 = {
                        let a4353 = v1;
                        let a4354 = 32u8;
                        fixed_u8::bit_and(a4353, a4354)
                    };
                    let a4356 = 32u8;
                    if equal(a4355, a4356) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4362 = {
                    let a4359 = {
                        let a4357 = v1;
                        let a4358 = 16u8;
                        fixed_u8::bit_and(a4357, a4358)
                    };
                    let a4360 = 16u8;
                    if equal(a4359, a4360) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4361, a4362)?
            };
            nat_add(a4363, a4364)?
        };
        let a4388 = {
            let a4385 = {
                let a4373 = {
                    let a4367 = {
                        let a4365 = v1;
                        let a4366 = 8u8;
                        fixed_u8::bit_and(a4365, a4366)
                    };
                    let a4368 = 8u8;
                    if equal(a4367, a4368) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4374 = {
                    let a4371 = {
                        let a4369 = v1;
                        let a4370 = 4u8;
                        fixed_u8::bit_and(a4369, a4370)
                    };
                    let a4372 = 4u8;
                    if equal(a4371, a4372) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4373, a4374)?
            };
            let a4386 = {
                let a4383 = {
                    let a4377 = {
                        let a4375 = v1;
                        let a4376 = 2u8;
                        fixed_u8::bit_and(a4375, a4376)
                    };
                    let a4378 = 2u8;
                    if equal(a4377, a4378) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4384 = {
                    let a4381 = {
                        let a4379 = v1;
                        let a4380 = 1u8;
                        fixed_u8::bit_and(a4379, a4380)
                    };
                    let a4382 = 1u8;
                    if equal(a4381, a4382) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4383, a4384)?
            };
            nat_add(a4385, a4386)?
        };
        nat_add(a4387, a4388)?
    };
    let a4389 = 48u64;
    let a4390 = v2;
    if if nat_le(a4389, a4390) {
        let a4391 = v2;
        let a4392 = 57u64;
        nat_le(a4391, a4392)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4393 = 97u64;
        let a4394 = v2;
        if nat_le(a4393, a4394) {
            let a4395 = v2;
            let a4396 = 102u64;
            Ok(nat_le(a4395, a4396))
        } else {
            Ok(false)
        }
    }
}

pub fn f828() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f829(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4441 = {
            let a4417 = {
                let a4405 = {
                    let a4399 = {
                        let a4397 = v1;
                        let a4398 = 128u8;
                        fixed_u8::bit_and(a4397, a4398)
                    };
                    let a4400 = 128u8;
                    if equal(a4399, a4400) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4406 = {
                    let a4403 = {
                        let a4401 = v1;
                        let a4402 = 64u8;
                        fixed_u8::bit_and(a4401, a4402)
                    };
                    let a4404 = 64u8;
                    if equal(a4403, a4404) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4405, a4406)?
            };
            let a4418 = {
                let a4415 = {
                    let a4409 = {
                        let a4407 = v1;
                        let a4408 = 32u8;
                        fixed_u8::bit_and(a4407, a4408)
                    };
                    let a4410 = 32u8;
                    if equal(a4409, a4410) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4416 = {
                    let a4413 = {
                        let a4411 = v1;
                        let a4412 = 16u8;
                        fixed_u8::bit_and(a4411, a4412)
                    };
                    let a4414 = 16u8;
                    if equal(a4413, a4414) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4415, a4416)?
            };
            nat_add(a4417, a4418)?
        };
        let a4442 = {
            let a4439 = {
                let a4427 = {
                    let a4421 = {
                        let a4419 = v1;
                        let a4420 = 8u8;
                        fixed_u8::bit_and(a4419, a4420)
                    };
                    let a4422 = 8u8;
                    if equal(a4421, a4422) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4428 = {
                    let a4425 = {
                        let a4423 = v1;
                        let a4424 = 4u8;
                        fixed_u8::bit_and(a4423, a4424)
                    };
                    let a4426 = 4u8;
                    if equal(a4425, a4426) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4427, a4428)?
            };
            let a4440 = {
                let a4437 = {
                    let a4431 = {
                        let a4429 = v1;
                        let a4430 = 2u8;
                        fixed_u8::bit_and(a4429, a4430)
                    };
                    let a4432 = 2u8;
                    if equal(a4431, a4432) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4438 = {
                    let a4435 = {
                        let a4433 = v1;
                        let a4434 = 1u8;
                        fixed_u8::bit_and(a4433, a4434)
                    };
                    let a4436 = 1u8;
                    if equal(a4435, a4436) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4437, a4438)?
            };
            nat_add(a4439, a4440)?
        };
        nat_add(a4441, a4442)?
    };
    let a4443 = v2;
    let a4444 = 57u64;
    if nat_le(a4443, a4444) {
        let a4445 = v2;
        let a4446 = 48u64;
        Ok(nat_sub(a4445, a4446))
    } else {
        let a4447 = v2;
        let a4448 = 87u64;
        Ok(nat_sub(a4447, a4448))
    }
}

pub fn f830(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f831(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4449, _) = v0.clone();
        h4449
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4450, _) = {
            let (_, h4451) = v0.clone();
            h4451
        };
        h4450
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4452) = {
            let (_, h4453) = v0;
            h4453
        };
        h4452
    };
    let m4454 = v1.clone();
    match m4454.uncons() {
        None => {
            let m4455 = v2;
            match m4455.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4456 = v2.clone();
            match m4456.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f810({
                        let (h4457, _) = v8.clone();
                        h4457
                    }, {
                        let (h4458, _) = v6.clone();
                        h4458
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

pub fn f832(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f435(f353(f435(f459(v2), f460(v2)), f353({
        let a4459 = v2;
        let a4460 = 117u64;
        !nat_eq(a4459, a4460)
    }, {
        let a4461 = v2;
        let a4462 = 85u64;
        !nat_eq(a4461, a4462)
    })), f353(f461(v2), {
        let a4463 = 0u64;
        let a4464 = v1;
        nat_lt(a4463, a4464)
    })) {
        f35(v0, v1, 1u64)
    } else {
        let a4465 = v2;
        let a4466 = 65536u64;
        if nat_lt(a4465, a4466) {
            let a4467 = f8(Str::lit("u"));
            let a4468 = f634(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4467, a4468)
        } else {
            let a4469 = f8(Str::lit("U"));
            let a4470 = f634(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4469, a4470)
        }
    }
}

pub fn f833(v0: u64) -> Bytes {
    let a4471 = f8(Str::lit("u"));
    let a4472 = f634(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4471, a4472)
}

pub fn f834() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f835(v0: Option<Adt18>) -> Option<u64> {
    let m4473 = v0;
    match m4473 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m4474 = v1;
            match m4474 {
                Adt18::C0(_) => {
                    None::<u64>
                }
                Adt18::C1(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt18::C2(_, _, _) => {
                    None::<u64>
                }
                Adt18::C3(_) => {
                    None::<u64>
                }
                Adt18::C4(_) => {
                    None::<u64>
                }
                Adt18::C5(_) => {
                    None::<u64>
                }
                Adt18::C6 => {
                    None::<u64>
                }
                Adt18::C7(_, _) => {
                    None::<u64>
                }
                Adt18::C8(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f836(v0: u64, v1: Adt18, v2: Bytes) -> R<Result<Adt35, Bytes>> {
    let m4475 = v0;
    if m4475 == 0 {
        Ok(f636(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4475 - 1;
        let m4476 = v1.clone();
        match m4476 {
            Adt18::C0(v4) => {
                if v4 {
                    Ok(f712())
                } else {
                    Ok(f636(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt18::C1(_, _) => {
                Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C2(_, _, _) => {
                Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C3(_) => {
                Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C4(_) => {
                Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C5(_) => {
                let m4477 = f334(f242(v1.clone(), f8(Str::lit("$ref"))));
                match m4477 {
                    None => {
                        if if f648(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f648(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f648(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f637(v2.clone(), List::cons(Adt28::C0(v2, v1), List::<Adt28>::nil())))
                        } else {
                            let v20: Bytes = f245(v1.clone(), f8(Str::lit("type")));
                            if if f28(v20.clone(), Bytes::lit(&[])) {
                                !f818(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f712())
                            } else {
                                if f28(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f637(f8(Str::lit("Bytes")), List::<Adt28>::nil()))
                                } else {
                                    if f28(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f637(f819(v1), List::<Adt28>::nil()))
                                    } else {
                                        if f28(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f637(f771(), List::<Adt28>::nil()))
                                        } else {
                                            if f28(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f637(f8(Str::lit("Bool")), List::<Adt28>::nil()))
                                            } else {
                                                if f28(v20.clone(), f8(Str::lit("array"))) {
                                                    let m4479 = f242(v1, f8(Str::lit("items")));
                                                    match m4479 {
                                                        None => {
                                                            Ok(f710(f8(Str::lit("List")), f712()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f710(f8(Str::lit("List")), f836(v3, v21, {
                                                                let a4480 = v2;
                                                                let a4481 = f8(Str::lit("Item"));
                                                                append_bytes(a4480, a4481)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f28(v20.clone(), f8(Str::lit("object"))) {
                                                        if f643(v1.clone()) {
                                                            Ok(f637(v2.clone(), List::cons(Adt28::C0(v2, v1), List::<Adt28>::nil())))
                                                        } else {
                                                            let m4482 = f707(v1);
                                                            match m4482 {
                                                                None => {
                                                                    Ok(f710(f8(Str::lit("List")), f820(f712())))
                                                                }
                                                                Some(v22) => {
                                                                    Ok(f710(f8(Str::lit("List")), f820(f836(v3, v22, {
                                                                        let a4483 = v2;
                                                                        let a4484 = f8(Str::lit("Value"));
                                                                        append_bytes(a4483, a4484)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f636(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(v18) => {
                        let m4478 = f633(v18)?;
                        match m4478 {
                            None => {
                                Ok(f636(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f637(v19, List::<Adt28>::nil()))
                            }
                        }
                    }
                }
            }
            Adt18::C6 => {
                Ok(f712())
            }
            Adt18::C7(_, _) => {
                Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C8(_, _, _) => {
                Ok(f636(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}
