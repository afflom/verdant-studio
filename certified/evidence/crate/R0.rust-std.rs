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
    C0(List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Adt12, List<Adt14>, List<Adt14>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt14>),
    C4(std::rc::Rc<Adt14>),
    C5,
    C6(std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
    C7(Bytes, std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(List<Bytes>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(bool, bool, Bytes, List<Bytes>, List<List<Bytes>>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Bytes, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C1(Bytes, Bytes, std::rc::Rc<Adt18>),
    C2(Bytes),
    C3(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C4,
}

#[derive(Clone)]
pub enum Adt19 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt19>),
    C5(std::rc::Rc<Adt19>),
    C6,
    C7(std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
    C8(Bytes, std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
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
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Adt19),
    C1(Bytes),
    C2,
}

#[derive(Clone)]
pub enum Adt26 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt26, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F58(u8, Bytes),
    F188(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F58(k0, k1) => f58(*k0, k1.clone(), p0),
            Fn0::F188(k0) => f188(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F165,
    F184,
    F267,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F165 => f165(p0, p1),
            Fn1::F184 => f184(p0, p1),
            Fn1::F267 => f267(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F220(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F220(k0) => f220(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F257(Bytes, u64, Bytes, u64),
    F308(Bytes),
    F369(Bytes),
    F374(Bytes, Bytes),
    F375(Bytes),
    F376(Bytes, u64),
    F381(u64, Bytes),
    F427(bool, u64, Bytes),
    F468(Bytes),
    F469(Bytes),
    F503(Bytes),
    F545(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F257(k0, k1, k2, k3) => f257(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F308(k0) => f308(k0.clone(), p0),
            Fn3::F369(k0) => Ok(f369(k0.clone(), p0)),
            Fn3::F374(k0, k1) => f374(k0.clone(), k1.clone(), p0),
            Fn3::F375(k0) => f375(k0.clone(), p0),
            Fn3::F376(k0, k1) => f376(k0.clone(), *k1, p0),
            Fn3::F381(k0, k1) => f381(*k0, k1.clone(), p0),
            Fn3::F427(k0, k1, k2) => f427(*k0, *k1, k2.clone(), p0),
            Fn3::F468(k0) => f468(k0.clone(), p0),
            Fn3::F469(k0) => f469(k0.clone(), p0),
            Fn3::F503(k0) => f503(k0.clone(), p0),
            Fn3::F545(k0) => f545(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F300(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn4::F300(k0, k1, k2) => f300(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F307(Bytes),
    F320(u64, Bytes),
    F400(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F307(k0) => f307(k0.clone(), p0),
            Fn5::F320(k0, k1) => Ok(f320(*k0, k1.clone(), p0)),
            Fn5::F400(k0) => f400(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F349,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn6::F349 => f349(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F352,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn7::F352 => f352(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F354,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn8::F354 => f354(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F357,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn9::F357 => f357(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F402,
    F620,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F402 => f402(p0, p1),
            Fn10::F620 => f620(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F405,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F405 => f405(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F408,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F408 => f408(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F411,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F411 => f411(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F416,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn14::F416 => f416(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F429(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F429(k0) => f429(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F464(bool, Bytes),
    F471(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F464(k0, k1) => f464(*k0, k1.clone(), p0),
            Fn16::F471(k0) => f471(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F480,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F480 => f480(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F482,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F482 => f482(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F484,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F484 => f484(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F492,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F492 => f492(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F521,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F521 => f521(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F583,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F583 => f583(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F585,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F585 => f585(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F587(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F587(k0) => f587(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F618,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F618 => f618(p0),
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
    f27(f28(v0, v1)?)
}

pub fn f21(v0: Adt0, v1: List<Bytes>) -> Adt0 {
    Adt0::C0(v1, match v0.clone() {
        Adt0::C0(_, h136, _, _, _, _, _, _, _, _, _) => {
            h136
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h137, _, _, _, _, _, _, _, _) => {
            h137
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h138, _, _, _, _, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h139, _, _, _, _, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h140, _, _, _, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h141, _, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h142, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h143, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h144, _) => {
            h144
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h145) => {
            h145
        }
    })
}

pub fn f22(v0: Bytes, v1: u64, v2: u64) -> R<List<Str>> {
    let m146 = v2;
    if m146 == 0 {
        Ok(List::<Str>::nil())
    } else {
        let v3 = m146 - 1;
        let m149 = {
            let a147 = v0.clone();
            let a148 = v1;
            index_bytes(a147, a148)
        };
        match m149 {
            None => {
                Ok(List::<Str>::nil())
            }
            Some(v4) => {
                Ok(List::cons(f29(v4)?, f22(v0, {
                    let a150 = v1;
                    let a151 = 1u64;
                    nat_add(a150, a151)?
                }, v3)?))
            }
        }
    }
}

pub fn f23(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h152, _, _, _, _, _, _, _, _, _, _) => {
            h152
        }
    }, match v0.clone() {
        Adt0::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
            h153
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h154, _, _, _, _, _, _, _, _) => {
            h154
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h155, _, _, _, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h156, _, _, _, _) => {
            h156
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h157, _, _, _) => {
            h157
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h158, _, _) => {
            h158
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h159, _) => {
            h159
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h160) => {
            h160
        }
    })
}

pub fn f24(v0: List<Bytes>) -> R<List<Str>> {
    let m161 = v0;
    match m161.uncons() {
        None => {
            Ok(List::<Str>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f4(v1)?, f24(v2)?))
        }
    }
}

pub fn f25(v0: List<(Bytes, Bytes)>) -> R<List<(Str, Str)>> {
    let m162 = v0;
    match m162.uncons() {
        None => {
            Ok(List::<(Str, Str)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f4({
                let (h163, _) = v1;
                h163
            })?, f4({
                let (_, h164) = v1;
                h164
            })?), f25(v2)?))
        }
    }
}

pub fn f26(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f30(v0.clone(), 47u8)?;
    let v2: Bytes = f31(v1);
    let a167 = {
        let a165 = v0.clone();
        length_bytes(a165)
    };
    let a168 = {
        let a166 = v2.clone();
        length_bytes(a166)
    };
    if nat_eq(a167, a168) {
        Ok(f8(Str::lit(".")))
    } else {
        Ok(f32(v0.clone(), 0u64, {
            let a173 = {
                let a169 = v0;
                length_bytes(a169)
            };
            let a174 = {
                let a171 = {
                    let a170 = v2;
                    length_bytes(a170)
                };
                let a172 = 1u64;
                nat_add(a171, a172)?
            };
            nat_sub(a173, a174)
        }))
    }
}

pub fn f27(v0: Adt11) -> R<Adt11> {
    let m176 = {
        let m175 = v0.clone();
        match m175 {
            Adt11::C0(v1) => {
                f33(v1)?
            }
            Adt11::C1(v2) => {
                f33(v2)?
            }
            Adt11::C2(v3, v4, _) => {
                f33(List::cons(v3, v4))?
            }
            Adt11::C3(v6, _) => {
                f33(f34(v6))?
            }
            Adt11::C4(_) => {
                None::<Bytes>
            }
        }
    };
    match m176 {
        None => {
            Ok(v0)
        }
        Some(v9) => {
            Ok(Adt11::C4(f35(v9)))
        }
    }
}

pub fn f28(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m177 = v0;
    match m177.uncons() {
        None => {
            Ok(Adt11::C4(f36()))
        }
        Some((v2, v3)) => {
            if f37(v2.clone(), f8(Str::lit("check"))) {
                f38(v1)
            } else {
                if f37(v2.clone(), f8(Str::lit("next"))) {
                    f39(v1, v3)
                } else {
                    if f37(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f40(v1))
                    } else {
                        if f37(v2.clone(), f8(Str::lit("publish"))) {
                            f41(v1)
                        } else {
                            if f37(v2.clone(), f8(Str::lit("agenda"))) {
                                f42(v1)
                            } else {
                                let m178 = v3;
                                match m178.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f36()))
                                    }
                                    Some((v4, v5)) => {
                                        if f37(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f43(v1, v4)
                                        } else {
                                            if f37(v2.clone(), f8(Str::lit("read-xml"))) {
                                                f44(v1, v4)
                                            } else {
                                                if f37(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                    f45(v1.clone(), v4.clone(), f46(v1, v4)?)
                                                } else {
                                                    if f37(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                        f45(v1.clone(), v4.clone(), f47(v1, v4, false)?)
                                                    } else {
                                                        if f37(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                            f45(v1.clone(), v4.clone(), f47(v1, v4, true)?)
                                                        } else {
                                                            if f37(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                                f45(v1.clone(), v4.clone(), f48(v1, v4, false)?)
                                                            } else {
                                                                if f37(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                    f45(v1.clone(), v4.clone(), f48(v1, v4, true)?)
                                                                } else {
                                                                    if f37(v2.clone(), f8(Str::lit("admit"))) {
                                                                        f49(v1, v4)
                                                                    } else {
                                                                        if f37(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                            f45(v1.clone(), v4.clone(), f50(v1, v4)?)
                                                                        } else {
                                                                            if f37(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                                f51(v1, v4)
                                                                            } else {
                                                                                if f37(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                    f52(v1, v4)
                                                                                } else {
                                                                                    if f37(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                        f53(v1, v4)
                                                                                    } else {
                                                                                        if f37(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                            f54(v1, v4)
                                                                                        } else {
                                                                                            if f37(v2, f8(Str::lit("propose-encoding"))) {
                                                                                                let m179 = v5;
                                                                                                match m179.uncons() {
                                                                                                    None => {
                                                                                                        Ok(Adt11::C4(f36()))
                                                                                                    }
                                                                                                    Some((v6, v7)) => {
                                                                                                        f55(v1, v4, v6, v7)
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                Ok(Adt11::C4(f36()))
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
                    }
                }
            }
        }
    }
}

pub fn f29(v0: u8) -> R<Str> {
    let m228 = {
        let a226 = f56();
        let a227 = {
            let v1: u8 = v0;
            let a224 = {
                let a200 = {
                    let a188 = {
                        let a182 = {
                            let a180 = v1;
                            let a181 = 128u8;
                            fixed_u8::bit_and(a180, a181)
                        };
                        let a183 = 128u8;
                        if equal(a182, a183) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a189 = {
                        let a186 = {
                            let a184 = v1;
                            let a185 = 64u8;
                            fixed_u8::bit_and(a184, a185)
                        };
                        let a187 = 64u8;
                        if equal(a186, a187) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a188, a189)?
                };
                let a201 = {
                    let a198 = {
                        let a192 = {
                            let a190 = v1;
                            let a191 = 32u8;
                            fixed_u8::bit_and(a190, a191)
                        };
                        let a193 = 32u8;
                        if equal(a192, a193) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a199 = {
                        let a196 = {
                            let a194 = v1;
                            let a195 = 16u8;
                            fixed_u8::bit_and(a194, a195)
                        };
                        let a197 = 16u8;
                        if equal(a196, a197) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a198, a199)?
                };
                nat_add(a200, a201)?
            };
            let a225 = {
                let a222 = {
                    let a210 = {
                        let a204 = {
                            let a202 = v1;
                            let a203 = 8u8;
                            fixed_u8::bit_and(a202, a203)
                        };
                        let a205 = 8u8;
                        if equal(a204, a205) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a211 = {
                        let a208 = {
                            let a206 = v1;
                            let a207 = 4u8;
                            fixed_u8::bit_and(a206, a207)
                        };
                        let a209 = 4u8;
                        if equal(a208, a209) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a210, a211)?
                };
                let a223 = {
                    let a220 = {
                        let a214 = {
                            let a212 = v1;
                            let a213 = 2u8;
                            fixed_u8::bit_and(a212, a213)
                        };
                        let a215 = 2u8;
                        if equal(a214, a215) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a221 = {
                        let a218 = {
                            let a216 = v1;
                            let a217 = 1u8;
                            fixed_u8::bit_and(a216, a217)
                        };
                        let a219 = 1u8;
                        if equal(a218, a219) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a220, a221)?
                };
                nat_add(a222, a223)?
            };
            nat_add(a224, a225)?
        };
        index_list(a226, a227)
    };
    match m228 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f30(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a229 = v0.clone();
        length_bytes(a229)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h230, _) = f57(Fn0::F58(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f59(List::cons(f32(v0, v4, {
        let a233 = v2;
        let a234 = v4;
        nat_sub(a233, a234)
    }), {
        let (_, h235) = {
            let (_, h236) = v3;
            h236
        };
        h235
    })))
}

pub fn f31(p240: List<Bytes>) -> Bytes {
    let m239 = p240;
    {
        let mut m239 = m239;
        loop {
            let step = {
                let v0 = m239;
                let m237 = v0;
                match m237.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m238 = v2.clone();
                        match m238.uncons() {
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
                Ok(next) => { m239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m244 = {
        let a241 = v0;
        let a242 = v1;
        let a243 = v2;
        slice_bytes(a241, a242, a243)
    };
    match m244 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f33(p247: List<Bytes>) -> R<Option<Bytes>> {
    let m246 = p247;
    {
        let mut m246 = m246;
        loop {
            let step = {
                let v0 = m246;
                let m245 = v0;
                match m245.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f60(v1.clone())? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m248 = v0;
    match m248.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h249, _) = v1;
                h249
            }, f34(v2))
        }
    }
}

pub fn f35(v0: Bytes) -> Bytes {
    let a254 = {
        let a250 = Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ");
        utf8_encode(a250)
    };
    let a255 = {
        let a252 = v0;
        let a253 = {
            let a251 = Str::lit(" is not\u{a}");
            utf8_encode(a251)
        };
        append_bytes(a252, a253)
    };
    append_bytes(a254, a255)
}

pub fn f36() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | read-xml FILE | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f37(v0: Bytes, v1: Bytes) -> bool {
    let a256 = v0;
    let a257 = v1;
    equal(a256, a257)
}

pub fn f38(v0: Adt1) -> R<Adt11> {
    let m258 = f61(v0.clone(), f62());
    match m258.uncons() {
        None => {
            let m259 = f63(v0.clone(), f62())?;
            match m259.uncons() {
                None => {
                    let m262 = f64(match v0.clone() {
                        Adt1::C0(_, h260, _) => {
                            h260
                        }
                    }, f65(f66(), f67(match v0.clone() {
                        Adt1::C0(_, h261, _) => {
                            h261
                        }
                    }, false)));
                    match m262.uncons() {
                        None => {
                            let m264 = f61(v0.clone(), f67(match v0.clone() {
                                Adt1::C0(_, h263, _) => {
                                    h263
                                }
                            }, true));
                            match m264.uncons() {
                                None => {
                                    let m266 = f68(v0.clone(), f67(match v0.clone() {
                                        Adt1::C0(_, h265, _) => {
                                            h265
                                        }
                                    }, true))?;
                                    match m266 {
                                        None => {
                                            let v10: (bool, Bytes) = f69(Adt13::C0(Adt12::C0(f70(f71(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f72(v0.clone())?, f73(f74(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f70(f71(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f73(f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f74(v0.clone(), f8(Str::lit("model/strata.json")))?, f75(f74(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f67(match v0 {
                                                Adt1::C0(_, h267, _) => {
                                                    h267
                                                }
                                            }, true)))?;
                                            let (h268, _) = v10;
                                            if h268 {
                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                    let (_, h269) = v10;
                                                    h269
                                                }))
                                            } else {
                                                Ok(Adt11::C4({
                                                    let (_, h270) = v10;
                                                    h270
                                                }))
                                            }
                                        }
                                        Some(v9) => {
                                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: classical choice is an assumption, and ")), List::cons(v9, List::cons(f8(Str::lit(" depends on it\u{a}")), List::<Bytes>::nil()))))))
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
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("register is not JSON: ")), List::cons(v3, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f39(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f76(f77(match v0.clone() {
        Adt1::C0(_, h271, _) => {
            h271
        }
    }, f78())) {
        Ok(Adt11::C1(List::cons(f78(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f79(f80(f78(), f81(match v0.clone() {
            Adt1::C0(_, h272, _) => {
                h272
            }
        }, f78())))?;
        let m273 = f61(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m273.uncons() {
            None => {
                let m274 = f82(v0.clone(), f83(f84(f71(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f85(v2, v1.clone()))?;
                match m274 {
                    None => {
                        f86(v0, v1)
                    }
                    Some(v5) => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("ingest-authority ")), List::cons(v5, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f40(v0: Adt1) -> Adt11 {
    if f76(f87(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h275) => {
            h275
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f88(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f76(f87(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h276) => {
                h276
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f88(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f41(v0: Adt1) -> R<Adt11> {
    let m277 = f89(v0.clone());
    match m277.uncons() {
        None => {
            let m278 = f65(f61(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f90(), List::<Bytes>::nil())))), f91(v0.clone())?);
            match m278.uncons() {
                None => {
                    Ok(Adt11::C3(f92(f93(v0.clone())?, List::cons((f94(), f95()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f96()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f97()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f98()), List::cons((f8(Str::lit("docs/catalog.md")), f99(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f100(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f101(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f102()), List::<(Bytes, Bytes)>::nil()))))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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

pub fn f42(v0: Adt1) -> R<Adt11> {
    let m279 = f89(v0.clone());
    match m279.uncons() {
        None => {
            let m280 = f65(f61(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f91(v0.clone())?);
            match m280.uncons() {
                None => {
                    let v5: Adt15 = f103(v0.clone())?;
                    let v6: List<Adt14> = f74(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f65(f104(f105(match v5.clone() {
                        Adt15::C0(h281, _, _) => {
                            h281
                        }
                    }, f106(f74(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f107(v6.clone())), f108(f105(match v5.clone() {
                        Adt15::C0(h282, _, _) => {
                            h282
                        }
                    }, f106(f74(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f109(v5.clone(), v6, f110(v5, f111(v0.clone(), f112(v0)?)?))?)))))
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

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m283 = f61(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m283.uncons() {
        None => {
            let v4: Bytes = f71(v0.clone(), v1);
            let v5: List<List<Bytes>> = f113(v4.clone())?;
            if f76(f114(f115(v5.clone()), f116())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f117(Str::lit("id"), v6.clone())?, List::cons(f117(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f117(Str::lit("artifact"), f118())?, List::cons(f117(Str::lit("checksum"), f119(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f118(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f120(f71(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f121(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f122(f123(f124(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f44(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m284 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m284.uncons() {
        None => {
            let m285 = f125(f71(v0, v1))?;
            match m285 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f126(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f127(f128(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
                }
                Err(v4) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v4, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f45(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m286 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m286.uncons() {
        None => {
            if f129(f84(f71(v0, v1))?)? {
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

pub fn f46(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m287 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m287.uncons() {
        None => {
            let m288 = f130(f84(f71(v0, v1))?)?;
            match m288 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f47(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m289 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m289.uncons() {
        None => {
            let v5: Bytes = f131(if v2 {
                f132()
            } else {
                f133()
            }, f134(v1.clone())?);
            if f76(f135(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m290 = f136(f84(f71(v0.clone(), v1.clone()))?, v2)?;
                match m290 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a292 = {
                            let a291 = v6;
                            length_list(a291)
                        };
                        let a293 = 0u64;
                        if nat_eq(a292, a293) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f71(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f48(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m294 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m294.uncons() {
        None => {
            let v5: List<Bytes> = f84(f71(v0.clone(), v1.clone()))?;
            if f76(f135(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f137(v5)?
                } else {
                    f138(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f131(f139(), f134(v1.clone())?), f71(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f131(f139(), f134(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f140()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f49(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m295 = f61(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m295.uncons() {
        None => {
            let v4: List<Bytes> = f84(f71(v0.clone(), v1))?;
            let m296 = f141(v4.clone(), f8(Str::lit("requirement")))?;
            match m296 {
                None => {
                    Ok(Adt11::C4(f140()))
                }
                Some(v5) => {
                    let m297 = f141(v4, f8(Str::lit("admission")))?;
                    match m297 {
                        None => {
                            Ok(Adt11::C4(f140()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f74(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f74(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f65(f73(v5.clone(), f8(Str::lit("id"))), f73(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f79(f65(f73(v7.clone(), f8(Str::lit("id"))), f73(v8.clone(), f8(Str::lit("construct")))))?;
                            let m298 = f142(f107(v6.clone()), f106(f74(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m298 {
                                None => {
                                    let a300 = {
                                        let a299 = v9.clone();
                                        length_list(a299)
                                    };
                                    let a301 = 0u64;
                                    if nat_eq(a300, a301) {
                                        Ok(Adt11::C4(f140()))
                                    } else {
                                        if f76({
                                            let a304 = {
                                                let a302 = f79(f65(v9.clone(), v10.clone()))?;
                                                length_list(a302)
                                            };
                                            let a305 = {
                                                let a303 = f65(v9.clone(), v10);
                                                length_list(a303)
                                            };
                                            nat_eq(a304, a305)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f144(f8(Str::lit("model/requirements.json")), f145(v7, v5))?, List::cons(f144(f8(Str::lit("model/admissions.json")), f145(v8, v6))?, List::cons(f121(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f122(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f143(v11))
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

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m306 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m306.uncons() {
        None => {
            let m307 = f146(f84(f71(v0, v1))?)?;
            match m307 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f51(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m308 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m308.uncons() {
        None => {
            let m309 = f147(v0.clone(), v1.clone())?;
            match m309 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m310 = f148(v4.clone(), f8(Str::lit("schema")));
                    match m310 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f149(v0, f150(v5.clone())?, v5, f151(v4.clone(), f8(Str::lit("name"))), f152(f153(f148(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f52(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m311 = f61(v0.clone(), List::cons(v1.clone(), List::cons(f90(), List::<Bytes>::nil())));
    match m311.uncons() {
        None => {
            let m312 = f154(f84(f71(v0.clone(), v1))?)?;
            match m312 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f74(v0, f90())?;
                    let v6: List<Bytes> = f73(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f79(f73(v5.clone(), f8(Str::lit("at"))))?;
                    let a314 = {
                        let a313 = v6.clone();
                        length_list(a313)
                    };
                    let a315 = 0u64;
                    if if nat_eq(a314, a315) {
                        true
                    } else {
                        f76({
                            let a318 = {
                                let a316 = f79(f65(v6.clone(), v7.clone()))?;
                                length_list(a316)
                            };
                            let a319 = {
                                let a317 = f65(v6.clone(), v7);
                                length_list(a317)
                            };
                            nat_eq(a318, a319)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f144(f90(), f145(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f122(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f53(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m320 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m320.uncons() {
        None => {
            let m321 = f155(v0, v1)?;
            match m321 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f156(f157(Some(v4))) {
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

pub fn f54(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m322 = f61(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m322.uncons() {
        None => {
            let m323 = f155(v0.clone(), v1.clone())?;
            match m323 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f76(f37(f158(v4.clone(), f8(Str::lit("schema"))), f159())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f158(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m324 = f160(v4.clone(), f8(Str::lit("source")));
                            match m324 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f161(f160(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f131(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f131(v9.clone(), f8(Str::lit("binding.json"))), f71(v0.clone(), v1));
                        let m325 = v8;
                        match m325 {
                            None => {
                                f162(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f76(f37(f158(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f162(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f131(v9, f134(f158(v7.clone(), f8(Str::lit("url"))))?);
                                    if f76(f87(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h326) => {
                                            h326
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f158(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m327 = f61(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m327.uncons() {
                                            None => {
                                                if f37(f119(f71(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f162(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f55(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m328 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m328.uncons() {
        None => {
            let m329 = f147(v0.clone(), v1.clone())?;
            match m329 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f149(v0.clone(), f71(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f56() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f57(v0: Fn0, p334: u64, p335: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m333 = (p334, p335);
    {
        let mut m333 = m333;
        loop {
            let step = {
                let (v1, v2) = m333;
                let m330 = v1;
                if m330 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m330 - 1;
                    let m332 = {
                        let c331: Fn0 = v0.clone();
                        c331.apply(v2.clone())?
                    };
                    match m332 {
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
                Ok(next) => { m333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f163(v1, v0, v2)
}

pub fn f59(v0: List<Bytes>) -> List<Bytes> {
    f164(Fn1::F165, List::<Bytes>::nil(), v0)
}

pub fn f60(v0: Bytes) -> R<bool> {
    f166(v0.clone(), 0u64, {
        let a336 = v0;
        length_bytes(a336)
    })
}

pub fn f61(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m337 = v1;
    match m337.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m339 = f167(match v0.clone() {
                Adt1::C0(h338, _, _) => {
                    h338
                }
            }, v2.clone());
            match m339 {
                None => {
                    List::cons(v2, f61(v0, v3))
                }
                Some(_) => {
                    f61(v0, v3)
                }
            }
        }
    }
}

pub fn f62() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f118(), List::<Bytes>::nil())))))))
}

pub fn f63(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m340 = v1;
    match m340.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f168(v0.clone(), v2.clone()) {
                f135(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m341 = f155(v0.clone(), v2.clone())?;
                match m341 {
                    None => {
                        Ok(List::cons(v2, f63(v0, v3)?))
                    }
                    Some(_) => {
                        f63(v0, v3)
                    }
                }
            } else {
                f63(v0, v3)
            }
        }
    }
}

pub fn f64(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m342 = v1;
    match m342.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f77(v0.clone(), v2.clone()) {
                f64(v0, v3)
            } else {
                List::cons(v2, f64(v0, v3))
            }
        }
    }
}

pub fn f65(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m343 = v0;
    match m343.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f65(v3, v1))
        }
    }
}

pub fn f66() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f67(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m344 = v0;
    match m344.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f65(f169({
                let (h345, _) = v2;
                h345
            }, {
                let (_, h346) = v2;
                h346
            }, v1), f67(v3, v1))
        }
    }
}

pub fn f68(v0: Adt1, p350: List<Bytes>) -> R<Option<Bytes>> {
    let m349 = p350;
    {
        let mut m349 = m349;
        loop {
            let step = {
                let v1 = m349;
                let m347 = v1;
                match m347.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m348 = f170(f71(v0.clone(), v2.clone()), f171())?;
                        match m348 {
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
                Ok(next) => { m349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h351, _, _, _, _) => {
            h351
        }
    };
    let v2: Adt16 = f172(v1.clone())?;
    let v3: List<Bytes> = f173(v1.clone());
    let v4: List<Bytes> = f73(match v0.clone() {
        Adt13::C0(_, _, h352, _, _) => {
            h352
        }
    }, f8(Str::lit("path")));
    let m354 = f174(v2, List::<u64>::nil(), f175(v1.clone(), match v0.clone() {
        Adt13::C0(_, h353, _, _, _) => {
            h353
        }
    }, 0u64)?);
    match m354 {
        None => {
            if f177(f178(match v0.clone() {
                Adt13::C0(_, _, _, _, h356) => {
                    h356
                }
            }, f65(match v0.clone() {
                Adt13::C0(_, _, _, _, h357) => {
                    h357
                }
            }, v4.clone()))?, f178(v4.clone(), f65(match v0.clone() {
                Adt13::C0(_, _, _, _, h358) => {
                    h358
                }
            }, v4.clone()))?, f179(match v0.clone() {
                Adt13::C0(_, _, h359, _, _) => {
                    h359
                }
            })) {
                let m363 = f180(f178(match v0.clone() {
                    Adt13::C0(_, _, _, h360, _) => {
                        h360
                    }
                }, f65(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h361, _) => {
                        h361
                    }
                }))?, f178(v3.clone(), f65(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h362, _) => {
                        h362
                    }
                }))?);
                match m363 {
                    None => {
                        let m366 = f181(f73(match v0 {
                            Adt13::C0(_, h365, _, _, _) => {
                                h365
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m366 {
                            None => {
                                let m367 = f181(v4, List::<Bytes>::nil());
                                match m367 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f122(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f176(f65(v3, match v0 {
                            Adt13::C0(_, _, _, h364, _) => {
                                h364
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f176(match v1 {
                Adt12::C0(_, _, _, _, h355) => {
                    h355
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f70(v0: Bytes) -> R<List<Bytes>> {
    f182(f84(v0)?, f8(Str::lit("id = \"")))
}

pub fn f71(v0: Adt1, v1: Bytes) -> Bytes {
    let m369 = f167(match v0 {
        Adt1::C0(h368, _, _) => {
            h368
        }
    }, v1);
    match m369 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m370 = v2;
            match m370 {
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

pub fn f72(v0: Adt1) -> R<List<Bytes>> {
    let m371 = f113(f71(v0, f118()))?;
    match m371.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f123(v2))
        }
    }
}

pub fn f73(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m372 = v0;
    match m372.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f158(v2, v1.clone()), f73(v3, v1))
        }
    }
}

pub fn f74(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f157(f155(v0, v1)?))
}

pub fn f75(v0: List<Adt14>) -> List<Bytes> {
    let m373 = v0;
    match m373.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m374 = f161(Some(v1));
            match m374 {
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

pub fn f76(v0: bool) -> bool {
    !v0
}

pub fn f77(p378: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m377 = p378;
    {
        let mut m377 = m377;
        loop {
            let step = {
                let v0 = m377;
                let m375 = v0;
                match m375.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h376, _) = v2;
                            h376
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f79(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f183(f164(Fn1::F184, List::<Bytes>::nil(), f185(v0)?)))
}

pub fn f80(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m379 = v1;
    match m379.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f135(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f131(v0.clone(), v2), f80(v0, v3))
            } else {
                f80(v0, v3)
            }
        }
    }
}

pub fn f81(p384: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m383 = p384;
    {
        let mut m383 = m383;
        loop {
            let step = {
                let v0 = m383;
                let m380 = v0;
                match m380.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h381, _) = v2;
                            h381
                        }, v1.clone()) {
                            let (_, h382) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h382)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: Adt1, v1: List<List<Bytes>>, p387: List<Bytes>) -> R<Option<Bytes>> {
    let m386 = p387;
    {
        let mut m386 = m386;
        loop {
            let step = {
                let v2 = m386;
                let m385 = v2;
                match m385.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f186(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(p390: List<Bytes>, p391: List<Bytes>, p392: List<List<Bytes>>) -> List<List<Bytes>> {
    let m389 = (p390, (p391, p392));
    {
        let mut m389 = m389;
        loop {
            let step = {
                let (v0, (v1, v2)) = m389;
                let m388 = v0;
                match m388.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f187(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f37(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f187(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a393 = v0.clone();
        length_bytes(a393)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h394, _) = f57(Fn0::F188(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h394
    };
    let v3: u64 = {
        let (h395, _) = {
            let (_, h396) = v2.clone();
            h396
        };
        h395
    };
    let v4: List<Bytes> = {
        let (_, h397) = {
            let (_, h398) = v2;
            h398
        };
        h397
    };
    let a399 = v3;
    let a400 = v1;
    if nat_lt(a399, a400) {
        Ok(f59(List::cons(f32(v0, v3, {
            let a401 = v1;
            let a402 = v3;
            nat_sub(a401, a402)
        }), v4)))
    } else {
        Ok(f59(v4))
    }
}

pub fn f85(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m403 = v0;
    match m403.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f87(v2.clone(), v1.clone()) {
                f85(v3, v1)
            } else {
                List::cons(v2, f85(v3, v1))
            }
        }
    }
}

pub fn f86(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f76(f77(match v0.clone() {
        Adt1::C0(_, h404, _) => {
            h404
        }
    }, f189())) {
        Ok(Adt11::C1(List::cons(f189(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f85(f79(f80(f189(), f81(match v0.clone() {
            Adt1::C0(_, h405, _) => {
                h405
            }
        }, f189())))?, v1.clone());
        let m406 = f61(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m406.uncons() {
            None => {
                let v5: List<Bytes> = f73(f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m407 = f61(v0.clone(), f190(v0.clone(), v2.clone(), v5.clone())?);
                match m407.uncons() {
                    None => {
                        let m408 = f191(v0.clone(), v2, v5)?;
                        match m408 {
                            None => {
                                f192(v0, v1)
                            }
                            Some(v8) => {
                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("admit-encoding ")), List::cons(v8, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f87(v0: Bytes, p411: List<Bytes>) -> bool {
    let m410 = p411;
    {
        let mut m410 = m410;
        loop {
            let step = {
                let v1 = m410;
                let m409 = v1;
                match m409.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f37(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
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

pub fn f88(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f89(v0: Adt1) -> List<Bytes> {
    if f77(match v0.clone() {
        Adt1::C0(_, h412, _) => {
            h412
        }
    }, f133()) {
        if f77(match v0 {
            Adt1::C0(_, h413, _) => {
                h413
            }
        }, f132()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f132(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f133(), List::<Bytes>::nil())
    }
}

pub fn f90() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f91(v0: Adt1) -> R<List<Bytes>> {
    Ok(f61(v0.clone(), List::cons(f118(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f112(v0)?)))))
}

pub fn f92(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m414 = v0;
    match m414.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f92(v3, v1))
        }
    }
}

pub fn f93(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m415 = f74(v0.clone(), f90())?;
    match m415.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f193(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f94() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f95() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f96() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f194(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f97() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught when it opens, too.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  pull_request:\u{a}    types: [opened]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}              { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f194(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f98() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f99(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f103(v0.clone())?;
    let v2: List<Adt14> = f74(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f110(v1.clone(), f111(v0.clone(), f112(v0.clone())?)?);
    let v4: List<List<Bytes>> = f124(f113(f71(v0, f118()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f195(v4.clone(), f123(v4), f196(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f109(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f100(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f197(f103(v0.clone())?, f111(v0.clone(), f112(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f101(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f198(f74(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f199(f74(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f102() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f103(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f72(v0.clone())?, f70(f71(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f73(f74(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f104(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m416 = v0;
    match m416.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f87(v2.clone(), v1.clone()) {
                f104(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f104(v3, v1))
            }
        }
    }
}

pub fn f105(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m417 = v0;
    match m417.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f87(v2.clone(), v1.clone()) {
                f105(v3, v1)
            } else {
                List::cons(v2, f105(v3, v1))
            }
        }
    }
}

pub fn f106(v0: List<Adt14>) -> List<Bytes> {
    let m418 = v0;
    match m418.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f65(f75(f157(f160(v1, f8(Str::lit("defers"))))), f106(v2))
        }
    }
}

pub fn f107(v0: List<Adt14>) -> List<Bytes> {
    let m419 = v0;
    match m419.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m420 = f160(v1, f8(Str::lit("std")));
            match m420 {
                None => {
                    f107(v2)
                }
                Some(v3) => {
                    List::cons(f158(v3, f8(Str::lit("row"))), f107(v2))
                }
            }
        }
    }
}

pub fn f108(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m421 = v0;
    match m421.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f87(v2.clone(), v1.clone()) {
                f108(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f108(v3, v1))
            }
        }
    }
}

pub fn f109(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f92(f200(v2.clone()), f201(f196(v0, v1.clone(), v2)?));
    f202(f79(f203(v1))?, v3.clone(), {
        let a423 = {
            let a422 = v3;
            length_list(a422)
        };
        let a424 = 1u64;
        nat_add(a423, a424)?
    })
}

pub fn f110(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m425 = v1;
    match m425.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f204(v2.clone(), f8(Str::lit("subject"))) {
                f205(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f110(v0, v3))
            } else {
                f110(v0, v3)
            }
        }
    }
}

pub fn f111(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m426 = v1;
    match m426.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m427 = f136(f84(f71(v0.clone(), v2.clone()))?, f206(v2, f132()))?;
            match m427 {
                None => {
                    f111(v0, v3)
                }
                Some(v4) => {
                    Ok(f145(v4, f111(v0, v3)?))
                }
            }
        }
    }
}

pub fn f112(v0: Adt1) -> R<List<Bytes>> {
    Ok(f65(f79(f207(f133(), f81(match v0.clone() {
        Adt1::C0(_, h428, _) => {
            h428
        }
    }, f133())))?, f79(f207(f132(), f81(match v0 {
        Adt1::C0(_, h429, _) => {
            h429
        }
    }, f132())))?))
}

pub fn f113(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f208(v0.clone(), 0u64, {
        let a430 = v0;
        length_bytes(a430)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a433 = {
            let a432 = match v1.clone() {
                Adt17::C0(_, _, h431, _, _) => {
                    h431
                }
            };
            length_bytes(a432)
        };
        let a434 = 0u64;
        if if nat_eq(a433, a434) {
            let m436 = match v1.clone() {
                Adt17::C0(_, _, _, h435, _) => {
                    h435
                }
            };
            match m436.uncons() {
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
            f209(v1)
        }
    };
    Ok(f210(match v4 {
        Adt17::C0(_, _, _, _, h437) => {
            h437
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f114(p442: List<Bytes>, p443: List<Bytes>) -> bool {
    let m441 = (p442, p443);
    {
        let mut m441 = m441;
        loop {
            let step = {
                let (v0, v1) = m441;
                let m438 = v0;
                match m438.uncons() {
                    None => {
                        let m439 = v1;
                        match m439.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m440 = v1;
                        match m440.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f37(v4, v6) {
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
                Ok(next) => { m441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<List<Bytes>>) -> List<Bytes> {
    let m444 = v0;
    match m444.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f116() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f117(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f211(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f118() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f119(v0: Bytes) -> R<Bytes> {
    Ok(f212(f213(v0)?))
}

pub fn f120(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f214(f83(f84(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f215(f216(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f121(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f217(Adt14::C3(std::rc::Rc::new(f218(f79(f65(v1, f75(f74(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f122(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m445 = f219(Fn2::F220(v1), None::<Bytes>, v0);
    match m445 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f123(v0: List<List<Bytes>>) -> List<Bytes> {
    let m446 = v0;
    match m446.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m447 = v1;
            match m447.uncons() {
                None => {
                    f123(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f123(v2))
                }
            }
        }
    }
}

pub fn f124(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m448 = v0;
    match m448.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f125(v0: Bytes) -> R<Result<Adt18, Bytes>> {
    let v1: Bytes = if f221(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
            let a450 = {
                let a449 = v0;
                length_bytes(a449)
            };
            let a451 = 3u64;
            nat_sub(a450, a451)
        })
    } else {
        v0
    };
    let v2: Bytes = f222(v1)?;
    let v3: u64 = {
        let a452 = v2.clone();
        length_bytes(a452)
    };
    let v4: u64 = f223(v2.clone())?;
    let a453 = v4;
    let a454 = v3;
    if nat_lt(a453, a454) {
        Ok(Err::<Adt18, Bytes>(f224(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m455 = f225(v2.clone())?;
        match m455 {
            Ok(v6) => {
                let m458 = f226(v2.clone(), v6, {
                    let a456 = v3;
                    let a457 = 2u64;
                    nat_add(a456, a457)?
                })?;
                match m458 {
                    Ok(v8) => {
                        if f227(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(Err::<Adt18, Bytes>(f224(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a459 = f228(v2.clone(), v8)?;
                            let a460 = 60u64;
                            if !nat_eq(a459, a460) {
                                Ok(Err::<Adt18, Bytes>(f224(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            } else {
                                let m463 = f229(v2.clone(), v8, {
                                    let a461 = v3;
                                    let a462 = 2u64;
                                    nat_add(a461, a462)?
                                }, 0u64)?;
                                match m463 {
                                    Ok(v10) => {
                                        let m467 = f226(v2, {
                                            let (_, h464) = v10;
                                            h464
                                        }, {
                                            let a465 = v3;
                                            let a466 = 2u64;
                                            nat_add(a465, a466)?
                                        })?;
                                        match m467 {
                                            Ok(v12) => {
                                                let a468 = v12;
                                                let a469 = v3;
                                                if nat_eq(a468, a469) {
                                                    Ok(Ok::<Adt18, Bytes>({
                                                        let (h470, _) = v10;
                                                        h470
                                                    }))
                                                } else {
                                                    Ok(Err::<Adt18, Bytes>(f224(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt18, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt18, Bytes>(v9))
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt18, Bytes>(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(Err::<Adt18, Bytes>(v5))
            }
        }
    }
}

pub fn f126(v0: Adt18) -> Bytes {
    let m471 = v0;
    match m471 {
        Adt18::C0(v1, _, _) => {
            v1
        }
        Adt18::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt18::C2(_) => {
            Bytes::lit(&[])
        }
        Adt18::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt18::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f127(v0: u64) -> R<Bytes> {
    f230(f231(), 10u64, v0)
}

pub fn f128(v0: Adt18) -> R<u64> {
    let m472 = v0;
    match m472 {
        Adt18::C0(_, _, r477) => {
            let v3 = (*r477).clone();
            let a473 = 1u64;
            let a474 = f128(v3)?;
            nat_add(a473, a474)
        }
        Adt18::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt18::C2(_) => {
            Ok(0u64)
        }
        Adt18::C3(r478, r479) => {
            let v8 = (*r478).clone();
            let v9 = (*r479).clone();
            let a475 = f128(v8)?;
            let a476 = f128(v9)?;
            nat_add(a475, a476)
        }
        Adt18::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f129(p485: List<Bytes>) -> R<bool> {
    let m484 = p485;
    {
        let mut m484 = m484;
        loop {
            let step = {
                let v0 = m484;
                let m480 = v0;
                match m480.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a482 = {
                            let a481 = f232(v1)?;
                            length_bytes(a481)
                        };
                        let a483 = 0u64;
                        if nat_eq(a482, a483) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
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

pub fn f130(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m486 = v0;
    match m486.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f232(v1)?;
            let a488 = {
                let a487 = v3.clone();
                length_bytes(a487)
            };
            let a489 = 0u64;
            if nat_eq(a488, a489) {
                f130(v2)
            } else {
                let m490 = f233(v3.clone())?;
                match m490 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f158(v4.clone(), f8(Str::lit("id")));
                        if if f37(f158(v4, f8(Str::lit("schema"))), f159()) {
                            f76({
                                let a492 = {
                                    let a491 = v5.clone();
                                    length_bytes(a491)
                                };
                                let a493 = 0u64;
                                nat_eq(a492, a493)
                            })
                        } else {
                            false
                        } {
                            let m494 = f130(v2)?;
                            match m494 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f234(v5), {
                                        let a495 = v3;
                                        let a496 = Bytes::lit(&[0x0a]);
                                        append_bytes(a495, a496)
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

pub fn f131(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f132() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f133() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f134(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f135(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a497 = v0.clone();
        length_bytes(a497)
    };
    let v3: u64 = {
        let a498 = v1.clone();
        length_bytes(a498)
    };
    let a499 = v3;
    let a500 = v2;
    if nat_le(a499, a500) {
        let m506 = {
            let a503 = v0;
            let a504 = {
                let a501 = v2;
                let a502 = v3;
                nat_sub(a501, a502)
            };
            let a505 = v3;
            slice_bytes(a503, a504, a505)
        };
        match m506 {
            None => {
                false
            }
            Some(v4) => {
                let a507 = v4;
                let a508 = v1;
                equal(a507, a508)
            }
        }
    } else {
        false
    }
}

pub fn f136(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m509 = v0;
    match m509.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f232(v2)?;
            let a511 = {
                let a510 = v4.clone();
                length_bytes(a510)
            };
            let a512 = 0u64;
            if nat_eq(a511, a512) {
                f136(v3, v1)
            } else {
                let m513 = f233(v4)?;
                match m513 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f235(v5.clone())
                        } else {
                            f236(v5.clone())
                        } {
                            let m514 = f136(v3, v1)?;
                            match m514 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons(v5, v6)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt14>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f137(v0: List<Bytes>) -> R<bool> {
    if f237(f154(v0.clone())?) {
        Ok(f76({
            let a516 = {
                let a515 = f238(f154(v0)?);
                length_list(a515)
            };
            let a517 = 0u64;
            nat_eq(a516, a517)
        }))
    } else {
        Ok(false)
    }
}

pub fn f138(v0: List<Bytes>) -> R<bool> {
    if f237(f141(v0.clone(), f8(Str::lit("requirement")))?) {
        if f237(f141(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f76({
                let a519 = {
                    let a518 = f145(f238(f141(v0.clone(), f8(Str::lit("requirement")))?), f238(f141(v0, f8(Str::lit("admission")))?));
                    length_list(a518)
                };
                let a520 = 0u64;
                nat_eq(a519, a520)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f139() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f140() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f141(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m521 = v0;
    match m521.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f232(v2)?;
            let a523 = {
                let a522 = v4.clone();
                length_bytes(a522)
            };
            let a524 = 0u64;
            if nat_eq(a523, a524) {
                f141(v3, v1)
            } else {
                let m525 = f233(v4)?;
                match m525 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f239(v5.clone());
                        let a527 = {
                            let a526 = v6.clone();
                            length_bytes(a526)
                        };
                        let a528 = 0u64;
                        if nat_eq(a527, a528) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m529 = f141(v3, v1.clone())?;
                            match m529 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f37(v6, v1) {
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

pub fn f142(p532: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m531 = p532;
    {
        let mut m531 = m531;
        loop {
            let step = {
                let v0 = m531;
                let m530 = v0;
                match m530.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f87(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f144(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f217(Adt14::C3(std::rc::Rc::new(f240(v1))))?))
}

pub fn f145(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m533 = v0;
    match m533.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f145(v3, v1))
        }
    }
}

pub fn f146(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m534 = v0;
    match m534.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f232(v1)?;
            let a536 = {
                let a535 = v3.clone();
                length_bytes(a535)
            };
            let a537 = 0u64;
            if nat_eq(a536, a537) {
                f146(v2)
            } else {
                let m538 = f241(v3.clone())?;
                match m538 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f242(v4.clone()) {
                            let m539 = f146(v2)?;
                            match m539 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f189(), List::cons(f8(Str::lit("/")), List::cons(f243(f151(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a540 = v3;
                                        let a541 = Bytes::lit(&[0x0a]);
                                        append_bytes(a540, a541)
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

pub fn f147(v0: Adt1, v1: Bytes) -> R<Option<Adt19>> {
    f241(f71(v0, v1))
}

pub fn f148(p547: Adt19, v1: Bytes) -> Option<Adt19> {
    let m546 = p547;
    {
        let mut m546 = m546;
        loop {
            let step = {
                let v0 = m546;
                let m542 = v0;
                match m542 {
                    Adt19::C0(_) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C1(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C2(_, _, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C3(_) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C4(_) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C5(r543) => {
                        let v5 = (*r543).clone();
                        Ok::<Adt19, Option<Adt19>>(v5)
                    }
                    Adt19::C6 => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C7(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C8(v2, r544, r545) => {
                        let v3 = (*r544).clone();
                        let v4 = (*r545).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt19, Option<Adt19>>(Some(v3))
                        } else {
                            Ok::<Adt19, Option<Adt19>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m546 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m548 = f61(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m548.uncons() {
        None => {
            let m550 = f142(f107(List::cons(Adt14::C4(std::rc::Rc::new({
                let m549 = f244(v4.clone());
                match m549 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f106(f74(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m550 {
                None => {
                    f245(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f143(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f150(v0: Adt19) -> R<Bytes> {
    let a551 = f246(v0)?;
    let a552 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a551, a552))
}

pub fn f151(v0: Adt19, v1: Bytes) -> Bytes {
    let m553 = f247(f148(v0, v1));
    match m553 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f152(v0: List<Adt19>) -> List<Bytes> {
    let m554 = v0;
    match m554.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m555 = f247(Some(v1));
            match m555 {
                None => {
                    f152(v2)
                }
                Some(v3) => {
                    List::cons(v3, f152(v2))
                }
            }
        }
    }
}

pub fn f153(v0: Option<Adt19>) -> List<Adt19> {
    let m556 = v0;
    match m556 {
        None => {
            List::<Adt19>::nil()
        }
        Some(v1) => {
            f248(v1)
        }
    }
}

pub fn f154(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m557 = v0;
    match m557.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f232(v1)?;
            let a559 = {
                let a558 = v3.clone();
                length_bytes(a558)
            };
            let a560 = 0u64;
            if nat_eq(a559, a560) {
                f154(v2)
            } else {
                let m561 = f233(v3)?;
                match m561 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f204(v4.clone(), f8(Str::lit("agent"))) {
                            if f204(v4.clone(), f8(Str::lit("at"))) {
                                if f204(v4.clone(), f8(Str::lit("quote"))) {
                                    f249(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m562 = f154(v2)?;
                            match m562 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons(v4, v5)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt14>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f155(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f233(f71(v0, v1))
}

pub fn f156(p565: List<Adt14>) -> bool {
    let m564 = p565;
    {
        let mut m564 = m564;
        loop {
            let step = {
                let v0 = m564;
                let m563 = v0;
                match m563.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f37(f158(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f87(f8(Str::lit("studio")), f73(f250(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
                        } else {
                            false
                        } {
                            Err::<List<Adt14>, bool>(true)
                        } else {
                            Ok::<List<Adt14>, bool>(v2)
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

pub fn f157(v0: Option<Adt14>) -> List<Adt14> {
    let m566 = v0;
    match m566 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f251(v1)
        }
    }
}

pub fn f158(v0: Adt14, v1: Bytes) -> Bytes {
    let m567 = f161(f160(v0, v1));
    match m567 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f159() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f160(p573: Adt14, v1: Bytes) -> Option<Adt14> {
    let m572 = p573;
    {
        let mut m572 = m572;
        loop {
            let step = {
                let v0 = m572;
                let m568 = v0;
                match m568 {
                    Adt14::C0(_) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C1(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C2(_) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C3(_) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C4(r569) => {
                        let v5 = (*r569).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r570, r571) => {
                        let v3 = (*r570).clone();
                        let v4 = (*r571).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Option<Adt14>) -> Option<Bytes> {
    let m574 = v0;
    match m574 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m575 = v1;
            match m575 {
                Adt14::C0(_) => {
                    None::<Bytes>
                }
                Adt14::C1(_, _) => {
                    None::<Bytes>
                }
                Adt14::C2(v2) => {
                    Some(v2)
                }
                Adt14::C3(_) => {
                    None::<Bytes>
                }
                Adt14::C4(_) => {
                    None::<Bytes>
                }
                Adt14::C5 => {
                    None::<Bytes>
                }
                Adt14::C6(_, _) => {
                    None::<Bytes>
                }
                Adt14::C7(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f162(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f120(f71(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f252(v1, v4)?)?), List::cons(f121(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f163(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h576, _) = v2.clone();
        h576
    };
    let v4: u64 = {
        let (h577, _) = {
            let (_, h578) = v2.clone();
            h578
        };
        h577
    };
    let v5: List<Bytes> = {
        let (_, h579) = {
            let (_, h580) = v2;
            h580
        };
        h579
    };
    let m581 = f253(v0.clone(), v3);
    match m581 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a582 = v6;
            let a583 = v1;
            if equal(a582, a583) {
                Ok(Some(({
                    let a584 = v3;
                    let a585 = 1u64;
                    nat_add(a584, a585)?
                }, ({
                    let a586 = v3;
                    let a587 = 1u64;
                    nat_add(a586, a587)?
                }, List::cons(f32(v0, v4, {
                    let a588 = v3;
                    let a589 = v4;
                    nat_sub(a588, a589)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a590 = v3;
                    let a591 = 1u64;
                    nat_add(a590, a591)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f164(v0: Fn1, p595: List<Bytes>, p596: List<Bytes>) -> List<Bytes> {
    let m594 = (p595, p596);
    {
        let mut m594 = m594;
        loop {
            let step = {
                let (v1, v2) = m594;
                let m592 = v2;
                match m592.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c593: Fn1 = v0.clone();
                            c593.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f166(v0: Bytes, p650: u64, p651: u64) -> R<bool> {
    let m649 = (p650, p651);
    {
        let mut m649 = m649;
        loop {
            let step = {
                let (v1, v2) = m649;
                let m597 = v2;
                if m597 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m597 - 1;
                    let m600 = {
                        let a598 = v0.clone();
                        let a599 = v1;
                        index_bytes(a598, a599)
                    };
                    match m600 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f254({
                                let v5: u8 = v4;
                                let a645 = {
                                    let a621 = {
                                        let a609 = {
                                            let a603 = {
                                                let a601 = v5;
                                                let a602 = 128u8;
                                                fixed_u8::bit_and(a601, a602)
                                            };
                                            let a604 = 128u8;
                                            if equal(a603, a604) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a610 = {
                                            let a607 = {
                                                let a605 = v5;
                                                let a606 = 64u8;
                                                fixed_u8::bit_and(a605, a606)
                                            };
                                            let a608 = 64u8;
                                            if equal(a607, a608) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a609, a610)?
                                    };
                                    let a622 = {
                                        let a619 = {
                                            let a613 = {
                                                let a611 = v5;
                                                let a612 = 32u8;
                                                fixed_u8::bit_and(a611, a612)
                                            };
                                            let a614 = 32u8;
                                            if equal(a613, a614) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a620 = {
                                            let a617 = {
                                                let a615 = v5;
                                                let a616 = 16u8;
                                                fixed_u8::bit_and(a615, a616)
                                            };
                                            let a618 = 16u8;
                                            if equal(a617, a618) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a619, a620)?
                                    };
                                    nat_add(a621, a622)?
                                };
                                let a646 = {
                                    let a643 = {
                                        let a631 = {
                                            let a625 = {
                                                let a623 = v5;
                                                let a624 = 8u8;
                                                fixed_u8::bit_and(a623, a624)
                                            };
                                            let a626 = 8u8;
                                            if equal(a625, a626) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a632 = {
                                            let a629 = {
                                                let a627 = v5;
                                                let a628 = 4u8;
                                                fixed_u8::bit_and(a627, a628)
                                            };
                                            let a630 = 4u8;
                                            if equal(a629, a630) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a631, a632)?
                                    };
                                    let a644 = {
                                        let a641 = {
                                            let a635 = {
                                                let a633 = v5;
                                                let a634 = 2u8;
                                                fixed_u8::bit_and(a633, a634)
                                            };
                                            let a636 = 2u8;
                                            if equal(a635, a636) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a642 = {
                                            let a639 = {
                                                let a637 = v5;
                                                let a638 = 1u8;
                                                fixed_u8::bit_and(a637, a638)
                                            };
                                            let a640 = 1u8;
                                            if equal(a639, a640) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a641, a642)?
                                    };
                                    nat_add(a643, a644)?
                                };
                                nat_add(a645, a646)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a647 = v1;
                                    let a648 = 1u64;
                                    nat_add(a647, a648)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(p656: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m655 = p656;
    {
        let mut m655 = m655;
        loop {
            let step = {
                let v0 = m655;
                let m652 = v0;
                match m652.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h653, _) = v2;
                            h653
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h654) = v2;
                                h654
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f168(v0: Adt1, v1: Bytes) -> bool {
    let m658 = f167(match v0 {
        Adt1::C0(h657, _, _) => {
            h657
        }
    }, v1);
    match m658 {
        None => {
            false
        }
        Some(v2) => {
            let m659 = v2;
            match m659 {
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

pub fn f169(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m660 = v1;
    match m660.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f255(v3.clone())
            } else {
                f76(f255(v3.clone()))
            } {
                List::cons(f131(v0.clone(), v3), f169(v0, v4, v2))
            } else {
                f169(v0, v4, v2)
            }
        }
    }
}

pub fn f170(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a661 = v0.clone();
        length_bytes(a661)
    };
    let v3: u64 = {
        let a662 = v1.clone();
        length_bytes(a662)
    };
    let v4: u64 = {
        let (h663, _) = f256(Fn3::F257(v1.clone(), v3, v0.clone(), v2), {
            let a664 = v2;
            let a665 = 1u64;
            nat_add(a664, a665)?
        }, 0u64)?;
        h663
    };
    if f221(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f171() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f172(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f258(match v0.clone() {
        Adt12::C0(h666, _, _, _, _) => {
            h666
        }
    }, 0u64)?, f258(match v0.clone() {
        Adt12::C0(_, h667, _, _, _) => {
            h667
        }
    }, 0u64)?, f258(match v0.clone() {
        Adt12::C0(_, _, h668, _, _) => {
            h668
        }
    }, 0u64)?, f258(match v0 {
        Adt12::C0(_, _, _, h669, _) => {
            h669
        }
    }, 0u64)?))
}

pub fn f173(v0: Adt12) -> List<Bytes> {
    f65(match v0.clone() {
        Adt12::C0(h670, _, _, _, _) => {
            h670
        }
    }, f65(match v0.clone() {
        Adt12::C0(_, _, h671, _, _) => {
            h671
        }
    }, f65(match v0.clone() {
        Adt12::C0(_, _, _, h672, _) => {
            h672
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h673) => {
            h673
        }
    })))
}

pub fn f174(v0: Adt16, p679: List<u64>, p680: List<Adt20>) -> Option<u64> {
    let m678 = (p679, p680);
    {
        let mut m678 = m678;
        loop {
            let step = {
                let (v1, v2) = m678;
                let m674 = v2;
                match m674.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f259(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h675) => {
                                h675
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h676, _) => {
                                    h676
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h677, _) => {
                                    h677
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m681 = v1;
    match m681.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f260(v0.clone(), v3)?), f175(v0, v4, {
                let a682 = v2;
                let a683 = 1u64;
                nat_add(a682, a683)?
            })?))
        }
    }
}

pub fn f176(p687: List<Bytes>, p688: u64) -> Bytes {
    let m686 = (p687, p688);
    {
        let mut m686 = m686;
        loop {
            let step = {
                let (v0, v1) = m686;
                let m684 = v0;
                match m684.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m685 = v1;
                        if m685 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m685 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f261(v0.clone(), v1.clone()) {
        if f261(v1, v0) {
            f262(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f178(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m689 = v0;
    match m689.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f263(v2, v1.clone())?, f178(v3, v1)?))
        }
    }
}

pub fn f179(v0: List<Adt14>) -> List<u64> {
    let m690 = v0;
    match m690.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f264(v1), f179(v2))
        }
    }
}

pub fn f180(p693: List<u64>, v1: List<u64>) -> Option<u64> {
    let m692 = p693;
    {
        let mut m692 = m692;
        loop {
            let step = {
                let v0 = m692;
                let m691 = v0;
                match m691.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f265(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m692 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(p696: List<Bytes>, p697: List<Bytes>) -> Option<Bytes> {
    let m695 = (p696, p697);
    {
        let mut m695 = m695;
        loop {
            let step = {
                let (v0, v1) = m695;
                let m694 = v0;
                match m694.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f87(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m695 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m698 = v0;
    match m698.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m699 = f266(v2, v1.clone())?;
            match m699 {
                None => {
                    f182(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f182(v3, v1)?))
                }
            }
        }
    }
}

pub fn f183(v0: List<Bytes>) -> List<Bytes> {
    f164(Fn1::F267, List::<Bytes>::nil(), v0)
}

pub fn f184(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f268(v0, v1)
}

pub fn f185(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f269(f270(f271(v0))?))
}

pub fn f186(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m700 = f155(v0, v2)?;
    match m700 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f272(v1, f158(v3.clone(), f8(Str::lit("id"))))?;
            let m701 = v4.clone();
            match m701.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m703 = f161(f160({
                        let m702 = f160(v3.clone(), f8(Str::lit("source")));
                        match m702 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m703 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f76(f87(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f211(f273(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f187(p706: List<Bytes>, p707: List<Bytes>) -> List<Bytes> {
    let m705 = (p706, p707);
    {
        let mut m705 = m705;
        loop {
            let step = {
                let (v0, v1) = m705;
                let m704 = v0;
                match m704.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f188(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f274(v0, v1)
}

pub fn f189() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f190(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m708 = v1;
    match m708.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt19 = f275(v0.clone(), v3)?;
            if f87(f276(f151(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f277(v5)?, f190(v0, v4, v2)?))
            } else {
                f190(v0, v4, v2)
            }
        }
    }
}

pub fn f191(v0: Adt1, p712: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m711 = p712;
    {
        let mut m711 = m711;
        loop {
            let step = {
                let v1 = m711;
                let m709 = v1;
                match m709.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt19 = f275(v0.clone(), v3.clone())?;
                        if if f87(f276(f151(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f37(f150({
                                let m710 = f148(v5.clone(), f8(Str::lit("schema")));
                                match m710 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f71(v0.clone(), f277(v5)?))
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
                Ok(next) => { m711 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f76(f77(match v0.clone() {
        Adt1::C0(_, h713, _) => {
            h713
        }
    }, f139())) {
        Ok(Adt11::C1(List::cons(f139(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f85(f79(f278(f81(match v0.clone() {
            Adt1::C0(_, h714, _) => {
                h714
            }
        }, f139())))?, v1);
        let m715 = f61(v0.clone(), List::cons(f90(), List::cons(f118(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m715.uncons() {
            None => {
                let v5: (bool, Bytes) = f279(v0.clone(), v2)?;
                let (h716, _) = v5;
                if h716 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h717) = v5;
                        h717
                    }))
                } else {
                    let m718 = f89(v0.clone());
                    match m718.uncons() {
                        None => {
                            let m719 = f91(v0.clone())?;
                            match m719.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h720) = v5;
                                        h720
                                    }, List::cons(f7(f79(f280(f103(v0.clone())?, f111(v0.clone(), f112(v0.clone())?)?))?), List::cons(f281(v0)?, List::<Bytes>::nil()))))))
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

pub fn f193(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f282(f74(v0, f90())?)?)))
}

pub fn f194() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f195(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt23>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m721 = v1;
    match m721.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f65(f283(v5.clone(), v2.clone()), f284(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f285(v0.clone(), v5.clone()), List::cons(if f87(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a723 = {
                    let a722 = v7.clone();
                    length_list(a722)
                };
                let a724 = 0u64;
                if nat_eq(a723, a724) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f122(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f195(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f196(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt23>> {
    let v3: List<(Bytes, Bytes)> = f286(v1);
    let v4: List<(Bytes, Bytes)> = f287(v0, v2);
    f288(v3.clone(), f92(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f197(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m725 = v1;
    match m725.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f204(v2.clone(), f8(Str::lit("domain"))) {
                f289(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f158(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f158(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m726 = f160(v2.clone(), f8(Str::lit("role")));
                    match m726 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f158(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f158(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f158(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f290(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m727 = f160(v2.clone(), f8(Str::lit("origin")));
                    match m727 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f158(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m728 = f160(v2, f8(Str::lit("evidence")));
                    match m728 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f158(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f197(v0, v3))
            } else {
                f197(v0, v3)
            }
        }
    }
}

pub fn f198(v0: List<Adt14>) -> List<Bytes> {
    let m729 = v0;
    match m729.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f158(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f158(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f158(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f198(v2))
        }
    }
}

pub fn f199(v0: List<Adt14>) -> List<Bytes> {
    let m730 = v0;
    match m730.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f158(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f291(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f199(v2))
        }
    }
}

pub fn f200(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m731 = v0;
    match m731.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f158(v1.clone(), f8(Str::lit("subject"))), f158(v1, f8(Str::lit("object")))), f200(v2))
        }
    }
}

pub fn f201(v0: List<Adt23>) -> List<(Bytes, Bytes)> {
    let m732 = v0;
    match m732.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt23::C0(h733, _, _) => {
                    h733
                }
            }, match v1 {
                Adt23::C0(_, _, h734) => {
                    h734
                }
            }), f201(v2))
        }
    }
}

pub fn f202(p741: List<Bytes>, v1: List<(Bytes, Bytes)>, p742: u64) -> R<List<Bytes>> {
    let m740 = (p741, p742);
    {
        let mut m740 = m740;
        loop {
            let step = {
                let (v0, v2) = m740;
                let m735 = v2;
                if m735 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m735 - 1;
                    let v4: List<Bytes> = f79(f65(v0.clone(), f292(v0.clone(), v1.clone())))?;
                    let a738 = {
                        let a736 = v4.clone();
                        length_list(a736)
                    };
                    let a739 = {
                        let a737 = v0.clone();
                        length_list(a737)
                    };
                    if nat_eq(a738, a739) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m740 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: List<Adt14>) -> List<Bytes> {
    let m743 = v0;
    match m743.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m744 = f160(v1, f8(Str::lit("std")));
            match m744 {
                None => {
                    f203(v2)
                }
                Some(v3) => {
                    List::cons(f158(v3.clone(), f8(Str::lit("authority"))), if f204(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f158(v3, f8(Str::lit("row"))), f203(v2))
                    } else {
                        f203(v2)
                    })
                }
            }
        }
    }
}

pub fn f204(v0: Adt14, v1: Bytes) -> bool {
    f76({
        let a746 = {
            let a745 = f158(v0, v1);
            length_bytes(a745)
        };
        let a747 = 0u64;
        nat_eq(a746, a747)
    })
}

pub fn f205(v0: Adt15, v1: Adt14) -> bool {
    if f293(v0.clone(), v1.clone(), f158(v1.clone(), f8(Str::lit("subject")))) {
        f293(v0, v1.clone(), f158(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f206(v0: Bytes, v1: Bytes) -> bool {
    let m752 = {
        let a749 = v0;
        let a750 = 0u64;
        let a751 = {
            let a748 = v1.clone();
            length_bytes(a748)
        };
        slice_bytes(a749, a750, a751)
    };
    match m752 {
        None => {
            false
        }
        Some(v2) => {
            let a753 = v2;
            let a754 = v1;
            equal(a753, a754)
        }
    }
}

pub fn f207(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m755 = v1;
    match m755.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f135(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f131(v0.clone(), v2), f207(v0, v3))
            } else {
                f207(v0, v3)
            }
        }
    }
}

pub fn f208(v0: Bytes, p813: u64, p814: u64, p815: Adt17) -> R<Adt17> {
    let m812 = (p813, (p814, p815));
    {
        let mut m812 = m812;
        loop {
            let step = {
                let (v1, (v2, v3)) = m812;
                let m756 = v2;
                if m756 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m756 - 1;
                    let v5: u64 = f294(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h757, _, _, _) => {
                            h757
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a758 = v1;
                            let a759 = 1u64;
                            nat_add(a758, a759)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h760, _, _) => {
                                h760
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h761, _) => {
                                h761
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h762) => {
                                h762
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h763, _, _, _, _) => {
                                h763
                            }
                        } {
                            let a764 = v5;
                            let a765 = 34u64;
                            if nat_eq(a764, a765) {
                                let a768 = f294(v0.clone(), {
                                    let a766 = v1;
                                    let a767 = 1u64;
                                    nat_add(a766, a767)?
                                })?;
                                let a769 = 34u64;
                                if nat_eq(a768, a769) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a770 = v1;
                                        let a771 = 1u64;
                                        nat_add(a770, a771)?
                                    }, (v4, Adt17::C0(true, true, f295(match v3.clone() {
                                        Adt17::C0(_, _, h772, _, _) => {
                                            h772
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h773, _) => {
                                            h773
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h774) => {
                                            h774
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a775 = v1;
                                        let a776 = 1u64;
                                        nat_add(a775, a776)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h777, _, _) => {
                                            h777
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h778, _) => {
                                            h778
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h779) => {
                                            h779
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a780 = v1;
                                    let a781 = 1u64;
                                    nat_add(a780, a781)?
                                }, (v4, Adt17::C0(true, false, f295(match v3.clone() {
                                    Adt17::C0(_, _, h782, _, _) => {
                                        h782
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h783, _) => {
                                        h783
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h784) => {
                                        h784
                                    }
                                }))))
                            }
                        } else {
                            let a785 = v5;
                            let a786 = 34u64;
                            if nat_eq(a785, a786) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a787 = v1;
                                    let a788 = 1u64;
                                    nat_add(a787, a788)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h789, _, _) => {
                                        h789
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h790, _) => {
                                        h790
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h791) => {
                                        h791
                                    }
                                }))))
                            } else {
                                let a792 = v5;
                                let a793 = 44u64;
                                if nat_eq(a792, a793) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a794 = v1;
                                        let a795 = 1u64;
                                        nat_add(a794, a795)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h796, _, _) => {
                                            h796
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h797, _) => {
                                            h797
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h798) => {
                                            h798
                                        }
                                    }))))
                                } else {
                                    let a799 = v5;
                                    let a800 = 13u64;
                                    if nat_eq(a799, a800) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a801 = v1;
                                            let a802 = 1u64;
                                            nat_add(a801, a802)?
                                        }, (v4, v3)))
                                    } else {
                                        let a803 = v5;
                                        let a804 = 10u64;
                                        if nat_eq(a803, a804) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a805 = v1;
                                                let a806 = 1u64;
                                                nat_add(a805, a806)?
                                            }, (v4, f209(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a807 = v1;
                                                let a808 = 1u64;
                                                nat_add(a807, a808)?
                                            }, (v4, Adt17::C0(false, false, f295(match v3.clone() {
                                                Adt17::C0(_, _, h809, _, _) => {
                                                    h809
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h810, _) => {
                                                    h810
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h811) => {
                                                    h811
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
                Ok(next) => { m812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f296(List::cons(match v0.clone() {
        Adt17::C0(_, _, h816, _, _) => {
            h816
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h817, _) => {
            h817
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h818) => {
            h818
        }
    }))
}

pub fn f210(p821: List<List<Bytes>>, p822: List<List<Bytes>>) -> List<List<Bytes>> {
    let m820 = (p821, p822);
    {
        let mut m820 = m820;
        loop {
            let step = {
                let (v0, v1) = m820;
                let m819 = v0;
                match m819.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f297(v0.clone(), 0u64, {
        let a823 = v0;
        length_bytes(a823)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f212(v0: Adt24) -> Bytes {
    let a844 = f298(match v0.clone() {
        Adt24::C0(h824, _, _, _, _, _, _, _) => {
            h824
        }
    });
    let a845 = {
        let a842 = f298(match v0.clone() {
            Adt24::C0(_, h825, _, _, _, _, _, _) => {
                h825
            }
        });
        let a843 = {
            let a840 = f298(match v0.clone() {
                Adt24::C0(_, _, h826, _, _, _, _, _) => {
                    h826
                }
            });
            let a841 = {
                let a838 = f298(match v0.clone() {
                    Adt24::C0(_, _, _, h827, _, _, _, _) => {
                        h827
                    }
                });
                let a839 = {
                    let a836 = f298(match v0.clone() {
                        Adt24::C0(_, _, _, _, h828, _, _, _) => {
                            h828
                        }
                    });
                    let a837 = {
                        let a834 = f298(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h829, _, _) => {
                                h829
                            }
                        });
                        let a835 = {
                            let a832 = f298(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h830, _) => {
                                    h830
                                }
                            });
                            let a833 = f298(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h831) => {
                                    h831
                                }
                            });
                            append_bytes(a832, a833)
                        };
                        append_bytes(a834, a835)
                    };
                    append_bytes(a836, a837)
                };
                append_bytes(a838, a839)
            };
            append_bytes(a840, a841)
        };
        append_bytes(a842, a843)
    };
    append_bytes(a844, a845)
}

pub fn f213(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a846 = v0.clone();
        length_bytes(a846)
    };
    let v2: u64 = {
        let a852 = {
            let a849 = {
                let a847 = v1;
                let a848 = 72u64;
                nat_add(a847, a848)?
            };
            let a850 = 64u64;
            let a851 = 0u64;
            nat_quot(a849, a850, a851)
        };
        let a853 = 64u64;
        nat_mul(a852, a853)?
    };
    let (_, h854) = f299(Fn4::F300(v0, v1, v2), {
        let a855 = v2;
        let a856 = 64u64;
        let a857 = 0u64;
        nat_quot(a855, a856, a857)
    }, (0u64, f301()))?;
    Ok(h854)
}

pub fn f214(p860: List<List<Bytes>>, v1: Bytes, p861: List<Bytes>) -> R<List<Bytes>> {
    let m859 = (p860, p861);
    {
        let mut m859 = m859;
        loop {
            let step = {
                let (v0, v2) = m859;
                let m858 = v0;
                match m858.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f302(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f216(f303(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f304(v0)?))
}

pub fn f216(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m862 = v0;
    match m862.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m863 = v3.clone();
            match m863.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a866 = v2;
                    let a867 = {
                        let a864 = v1.clone();
                        let a865 = f216(v3, v1);
                        append_bytes(a864, a865)
                    };
                    append_bytes(a866, a867)
                }
            }
        }
    }
}

pub fn f217(v0: Adt14) -> R<Bytes> {
    let a868 = f305(v0)?;
    let a869 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a868, a869))
}

pub fn f218(v0: List<Bytes>) -> Adt14 {
    let m870 = v0;
    match m870.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f218(v2)))
        }
    }
}

pub fn f219(v0: Fn2, p874: Option<Bytes>, p875: List<Bytes>) -> Option<Bytes> {
    let m873 = (p874, p875);
    {
        let mut m873 = m873;
        loop {
            let step = {
                let (v1, v2) = m873;
                let m871 = v2;
                match m871.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c872: Fn2 = v0.clone();
                            c872.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m876 = v1;
    match m876 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a879 = {
                    let a877 = v3;
                    let a878 = v0;
                    append_bytes(a877, a878)
                };
                let a880 = v2;
                append_bytes(a879, a880)
            })
        }
    }
}

pub fn f221(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m885 = {
        let a882 = v0;
        let a883 = v2;
        let a884 = {
            let a881 = v1.clone();
            length_bytes(a881)
        };
        slice_bytes(a882, a883, a884)
    };
    match m885 {
        None => {
            false
        }
        Some(v3) => {
            let a886 = v3;
            let a887 = v1;
            equal(a886, a887)
        }
    }
}

pub fn f222(v0: Bytes) -> R<Bytes> {
    let (_, h888) = {
        let (h889, _) = f306(Fn5::F307(v0.clone()), {
            let a891 = {
                let a890 = v0;
                length_bytes(a890)
            };
            let a892 = 1u64;
            nat_add(a891, a892)?
        }, (0u64, Bytes::lit(&[])))?;
        h889
    };
    Ok(h888)
}

pub fn f223(v0: Bytes) -> R<u64> {
    let (h893, _) = f256(Fn3::F308(v0.clone()), {
        let a895 = {
            let a894 = v0;
            length_bytes(a894)
        };
        let a896 = 1u64;
        nat_add(a895, a896)?
    }, 0u64)?;
    Ok(h893)
}

pub fn f224(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a897 = v0;
        utf8_encode(a897)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f127(v1)?, List::<Bytes>::nil())))))
}

pub fn f225(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f227(v0.clone(), 0u64, Str::lit("<?xml")) {
        f309(f228(v0.clone(), 5u64)?)
    } else {
        false
    } {
        let m899 = f310(v0.clone(), {
            let a898 = Str::lit("?>");
            utf8_encode(a898)
        }, 5u64)?;
        match m899 {
            None => {
                Ok(Err::<u64, Bytes>(f224(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m901 = f310(f32(v0.clone(), 0u64, v1), {
                    let a900 = Str::lit("encoding");
                    utf8_encode(a900)
                }, 5u64)?;
                match m901 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a902 = v1;
                            let a903 = 2u64;
                            nat_add(a902, a903)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f311(v0.clone(), {
                            let a904 = v2;
                            let a905 = 8u64;
                            nat_add(a904, a905)?
                        })?;
                        let v4: u64 = f311(v0.clone(), {
                            let a906 = v3;
                            let a907 = 1u64;
                            nat_add(a906, a907)?
                        })?;
                        let _: u64 = f228(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m910 = f310(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a908 = v4;
                                let a909 = 1u64;
                                nat_add(a908, a909)?
                            })?;
                            match m910 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f312(f32(v0, {
                            let a911 = v4;
                            let a912 = 1u64;
                            nat_add(a911, a912)?
                        }, {
                            let a915 = v7;
                            let a916 = {
                                let a913 = v4;
                                let a914 = 1u64;
                                nat_add(a913, a914)?
                            };
                            nat_sub(a915, a916)
                        }))? {
                            Ok(Ok::<u64, Bytes>({
                                let a917 = v1;
                                let a918 = 2u64;
                                nat_add(a917, a918)?
                            }))
                        } else {
                            Ok(Err::<u64, Bytes>(f224(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f226(v0: Bytes, p923: u64, p924: u64) -> R<Result<u64, Bytes>> {
    let m922 = (p923, p924);
    {
        let mut m922 = m922;
        loop {
            let step = {
                let (v1, v2) = m922;
                let m919 = v2;
                if m919 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(f224(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m919 - 1;
                    let v4: u64 = f311(v0.clone(), v1)?;
                    if f227(v0.clone(), v4, Str::lit("<!--")) {
                        let m920 = f313(v0.clone(), v4)?;
                        match m920 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v5)))
                            }
                        }
                    } else {
                        if f227(v0.clone(), v4, Str::lit("<?")) {
                            let m921 = f314(v0.clone(), v4)?;
                            match m921 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m922 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: Bytes, v1: u64, v2: Str) -> bool {
    f221(v0, {
        let a925 = v2;
        utf8_encode(a925)
    }, v1)
}

pub fn f228(v0: Bytes, v1: u64) -> R<u64> {
    f315(v0, v1)
}

pub fn f229(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt18, u64), Bytes>> {
    let m926 = v2;
    if m926 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m926 - 1;
        let a927 = v3;
        let a928 = 0u64;
        if nat_eq(a927, a928) {
            let v5: u64 = f316(v0.clone(), {
                let a929 = v1;
                let a930 = 1u64;
                nat_add(a929, a930)?
            })?;
            let a933 = v5;
            let a934 = {
                let a931 = v1;
                let a932 = 1u64;
                nat_add(a931, a932)?
            };
            if nat_eq(a933, a934) {
                Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a935 = v1;
                    let a936 = 1u64;
                    nat_add(a935, a936)?
                }, {
                    let a939 = v5;
                    let a940 = {
                        let a937 = v1;
                        let a938 = 1u64;
                        nat_add(a937, a938)?
                    };
                    nat_sub(a939, a940)
                });
                let m941 = f317(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m941 {
                    Ok(v8) => {
                        if f227(v0.clone(), {
                            let (_, h942) = v8.clone();
                            h942
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                let (h943, _) = v8;
                                h943
                            }), std::rc::Rc::new(Adt18::C4)), {
                                let a945 = {
                                    let (_, h944) = v8;
                                    h944
                                };
                                let a946 = 2u64;
                                nat_add(a945, a946)?
                            })))
                        } else {
                            let m950 = f229(v0.clone(), {
                                let a948 = {
                                    let (_, h947) = v8;
                                    h947
                                };
                                let a949 = 1u64;
                                nat_add(a948, a949)?
                            }, v4, 1u64)?;
                            match m950 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a952 = {
                                            let (_, h951) = v10.clone();
                                            h951
                                        };
                                        let a953 = 2u64;
                                        nat_add(a952, a953)?
                                    };
                                    let v12: u64 = f316(v0.clone(), v11)?;
                                    if !f37(f32(v0.clone(), v11, {
                                        let a954 = v12;
                                        let a955 = v11;
                                        nat_sub(a954, a955)
                                    }), v6.clone()) {
                                        Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h956) = v10;
                                            h956
                                        })?))
                                    } else {
                                        let v13: u64 = f311(v0.clone(), v12)?;
                                        let a957 = f228(v0, v13)?;
                                        let a958 = 62u64;
                                        if nat_eq(a957, a958) {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                                let (h959, _) = v8;
                                                h959
                                            }), std::rc::Rc::new({
                                                let (h960, _) = v10;
                                                h960
                                            })), {
                                                let a961 = v13;
                                                let a962 = 1u64;
                                                nat_add(a961, a962)?
                                            })))
                                        } else {
                                            Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<(Adt18, u64), Bytes>(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<(Adt18, u64), Bytes>(v7))
                    }
                }
            }
        } else {
            let a963 = f228(v0.clone(), v1)?;
            let a964 = 256u64;
            if nat_eq(a963, a964) {
                Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f227(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v1)))
                } else {
                    if f227(v0.clone(), v1, Str::lit("<!--")) {
                        let m965 = f313(v0.clone(), v1)?;
                        match m965 {
                            Ok(v15) => {
                                f229(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(Err::<(Adt18, u64), Bytes>(v14))
                            }
                        }
                    } else {
                        if f227(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m969 = f310(v0.clone(), {
                                let a966 = Str::lit("]]>");
                                utf8_encode(a966)
                            }, {
                                let a967 = v1;
                                let a968 = 9u64;
                                nat_add(a967, a968)?
                            })?;
                            match m969 {
                                None => {
                                    Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m972 = f229(v0.clone(), {
                                        let a970 = v16;
                                        let a971 = 3u64;
                                        nat_add(a970, a971)?
                                    }, v4, 1u64)?;
                                    match m972 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2(f32(v0, {
                                                let a973 = v1;
                                                let a974 = 9u64;
                                                nat_add(a973, a974)?
                                            }, {
                                                let a977 = v16;
                                                let a978 = {
                                                    let a975 = v1;
                                                    let a976 = 9u64;
                                                    nat_add(a975, a976)?
                                                };
                                                nat_sub(a977, a978)
                                            }))), std::rc::Rc::new({
                                                let (h979, _) = v18;
                                                h979
                                            })), {
                                                let (_, h980) = v18;
                                                h980
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(Err::<(Adt18, u64), Bytes>(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f227(v0.clone(), v1, Str::lit("<?")) {
                                let m981 = f314(v0.clone(), v1)?;
                                match m981 {
                                    Ok(v20) => {
                                        f229(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v19))
                                    }
                                }
                            } else {
                                if f227(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a982 = f228(v0.clone(), v1)?;
                                    let a983 = 60u64;
                                    if nat_eq(a982, a983) {
                                        let m984 = f229(v0.clone(), v1, v4, 0u64)?;
                                        match m984 {
                                            Ok(v22) => {
                                                let m986 = f229(v0, {
                                                    let (_, h985) = v22;
                                                    h985
                                                }, v4, 1u64)?;
                                                match m986 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new({
                                                            let (h987, _) = v22;
                                                            h987
                                                        }), std::rc::Rc::new({
                                                            let (h988, _) = v24;
                                                            h988
                                                        })), {
                                                            let (_, h989) = v24;
                                                            h989
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<(Adt18, u64), Bytes>(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(Err::<(Adt18, u64), Bytes>(v21))
                                            }
                                        }
                                    } else {
                                        let m990 = f318(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m990 {
                                            Ok(v26) => {
                                                let m992 = f229(v0, {
                                                    let (_, h991) = v26;
                                                    h991
                                                }, v4, 1u64)?;
                                                match m992 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2({
                                                            let (h993, _) = v26;
                                                            h993
                                                        })), std::rc::Rc::new({
                                                            let (h994, _) = v28;
                                                            h994
                                                        })), {
                                                            let (_, h995) = v28;
                                                            h995
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(Err::<(Adt18, u64), Bytes>(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(Err::<(Adt18, u64), Bytes>(v25))
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

pub fn f230(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a996 = v2;
    let a997 = 0u64;
    if nat_eq(a996, a997) {
        Ok(f319(v0, 0u64))
    } else {
        let (_, h998) = {
            let (h999, _) = f306(Fn5::F320(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h999
        };
        Ok(h998)
    }
}

pub fn f231() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f232(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f304(v0.clone())?;
    let v2: u64 = f321(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a1000 = v1;
        let a1001 = v2;
        nat_sub(a1000, a1001)
    }))
}

pub fn f233(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f322(v0)?;
    let m1005 = f323(v1.clone(), 0u64, {
        let a1003 = {
            let a1002 = v1.clone();
            length_bytes(a1002)
        };
        let a1004 = 2u64;
        nat_add(a1003, a1004)?
    }, 0u64)?;
    match m1005 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a1008 = {
                let (_, h1006) = v2;
                h1006
            };
            let a1009 = {
                let a1007 = v1;
                length_bytes(a1007)
            };
            if nat_eq(a1008, a1009) {
                Ok(Some({
                    let (h1010, _) = v2;
                    h1010
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f234(v0: Bytes) -> Bytes {
    f7(List::cons(f78(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f235(v0: Adt14) -> bool {
    if f204(v0.clone(), f8(Str::lit("domain"))) {
        if f204(v0.clone(), f8(Str::lit("row"))) {
            if f87(f158(v0.clone(), f8(Str::lit("relation"))), f324()) {
                if f325(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f325(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f325(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f325(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f236(v0: Adt14) -> bool {
    if f204(v0.clone(), f8(Str::lit("subject"))) {
        if f204(v0.clone(), f8(Str::lit("object"))) {
            if f204(v0.clone(), f8(Str::lit("evidence"))) {
                f87(f158(v0, f8(Str::lit("relation"))), f326())
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

pub fn f237(v0: Option<List<Adt14>>) -> bool {
    let m1011 = v0;
    match m1011 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f238(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1012 = v0;
    match m1012 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f239(v0: Adt14) -> Bytes {
    if f204(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f204(v0.clone(), f8(Str::lit("id"))) {
            if f204(v0.clone(), f8(Str::lit("statement"))) {
                if f204(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f249(v0, f8(Str::lit("evidence")))
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

pub fn f240(v0: List<Adt14>) -> Adt14 {
    let m1013 = v0;
    match m1013.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f240(v2)))
        }
    }
}

pub fn f241(v0: Bytes) -> R<Option<Adt19>> {
    let v1: Bytes = f322(v0)?;
    let m1017 = f327(v1.clone(), 0u64, {
        let a1015 = {
            let a1014 = v1.clone();
            length_bytes(a1014)
        };
        let a1016 = 2u64;
        nat_add(a1015, a1016)?
    }, 0u64)?;
    match m1017 {
        None => {
            Ok(None::<Adt19>)
        }
        Some(v2) => {
            let a1020 = {
                let (_, h1018) = v2;
                h1018
            };
            let a1021 = {
                let a1019 = v1;
                length_bytes(a1019)
            };
            if nat_eq(a1020, a1021) {
                Ok(Some({
                    let (h1022, _) = v2;
                    h1022
                }))
            } else {
                Ok(None::<Adt19>)
            }
        }
    }
}

pub fn f242(v0: Adt19) -> bool {
    if f328(v0.clone(), f8(Str::lit("name"))) {
        if f329(v0.clone(), f8(Str::lit("schema"))) {
            let m1023 = f244(f152(f153(f148(v0, f8(Str::lit("citation"))))));
            match m1023 {
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

pub fn f243(v0: Bytes) -> R<Bytes> {
    f330(v0.clone(), 0u64, {
        let a1024 = v0;
        length_bytes(a1024)
    }, true, Bytes::lit(&[]))
}

pub fn f244(v0: List<Bytes>) -> Option<Adt14> {
    let m1025 = v0;
    match m1025.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f37(v1.clone(), f8(Str::lit("req"))) {
                let m1026 = v2;
                match m1026.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f37(v1.clone(), f8(Str::lit("std"))) {
                    let m1027 = v2;
                    match m1027.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1028 = v6;
                                match m1028.uncons() {
                                    None => {
                                        Adt14::C5
                                    }
                                    Some((v7, _)) => {
                                        Adt14::C7(f8(Str::lit("row")), std::rc::Rc::new(Adt14::C2(v7)), std::rc::Rc::new(Adt14::C5))
                                    }
                                }
                            }))))), std::rc::Rc::new(Adt14::C5)))
                        }
                    }
                } else {
                    if f37(v1, f8(Str::lit("der"))) {
                        let m1029 = v2;
                        match m1029.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1030 = v10;
                                match m1030.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f218(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
                                    }
                                }
                            }
                        }
                    } else {
                        None::<Adt14>
                    }
                }
            }
        }
    }
}

pub fn f245(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1031 = f61(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1031.uncons() {
        None => {
            let m1032 = f244(v4);
            match m1032 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1033 = f8(Str::lit("Catalog."));
                        let a1034 = f243(v3.clone())?;
                        append_bytes(a1033, a1034)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f243(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1035 = f331(v2, v9.clone(), f243(v3.clone())?)?;
                    match m1035 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f243(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f217(Adt14::C3(std::rc::Rc::new(f240(f332(f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f217(Adt14::C3(std::rc::Rc::new(f240(f332(f74(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1039 = f333(f71(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a1037 = {
                                        let a1036 = v10;
                                        length_bytes(a1036)
                                    };
                                    let a1038 = 4u64;
                                    nat_sub(a1037, a1038)
                                }))?;
                                let a1040 = Bytes::lit(&[0x0a]);
                                append_bytes(a1039, a1040)
                            }), List::cons(f121(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                        }
                        Err(v11) => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v11, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f246(v0: Adt19) -> R<Bytes> {
    let m1041 = v0;
    match m1041 {
        Adt19::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C1(v2, v3) => {
            f334(v2, v3)
        }
        Adt19::C2(v4, v5, v6) => {
            f335(v4, v5, v6)
        }
        Adt19::C3(v7) => {
            f336(v7)
        }
        Adt19::C4(r1042) => {
            let v8 = (*r1042).clone();
            Ok(f337(f338(v8)?))
        }
        Adt19::C5(r1043) => {
            let v9 = (*r1043).clone();
            f339(f340(v9)?)
        }
        Adt19::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f247(v0: Option<Adt19>) -> Option<Bytes> {
    let m1044 = v0;
    match m1044 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1045 = v1;
            match m1045 {
                Adt19::C0(_) => {
                    None::<Bytes>
                }
                Adt19::C1(_, _) => {
                    None::<Bytes>
                }
                Adt19::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt19::C3(v2) => {
                    Some(v2)
                }
                Adt19::C4(_) => {
                    None::<Bytes>
                }
                Adt19::C5(_) => {
                    None::<Bytes>
                }
                Adt19::C6 => {
                    None::<Bytes>
                }
                Adt19::C7(_, _) => {
                    None::<Bytes>
                }
                Adt19::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f248(v0: Adt19) -> List<Adt19> {
    let m1046 = v0;
    match m1046 {
        Adt19::C0(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C1(_, _) => {
            List::<Adt19>::nil()
        }
        Adt19::C2(_, _, _) => {
            List::<Adt19>::nil()
        }
        Adt19::C3(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C4(r1047) => {
            let v3 = (*r1047).clone();
            f248(v3)
        }
        Adt19::C5(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C6 => {
            List::<Adt19>::nil()
        }
        Adt19::C7(r1048, r1049) => {
            let v1 = (*r1048).clone();
            let v2 = (*r1049).clone();
            List::cons(v1, f248(v2))
        }
        Adt19::C8(_, _, _) => {
            List::<Adt19>::nil()
        }
    }
}

pub fn f249(v0: Adt14, v1: Bytes) -> bool {
    let m1050 = f160(v0, v1);
    match m1050 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f250(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1051 = f160(v0, v1);
    match m1051 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f157(f160(v3, v2))
        }
    }
}

pub fn f251(v0: Adt14) -> List<Adt14> {
    let m1052 = v0;
    match m1052 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1053) => {
            let v3 = (*r1053).clone();
            f251(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1054, r1055) => {
            let v1 = (*r1054).clone();
            let v2 = (*r1055).clone();
            List::cons(v1, f251(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f252(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f160(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f117(Str::lit("id"), f158(v0.clone(), f8(Str::lit("id"))))?, List::cons(f117(Str::lit("name"), f158(v0.clone(), f8(Str::lit("name"))))?, List::cons(f117(Str::lit("issuer"), f158(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f117(Str::lit("edition"), f158(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f117(Str::lit("canonical_id"), f158(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f117(Str::lit("source_role"), f158(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f117(Str::lit("retrieval_date"), f158(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f117(Str::lit("url"), f341(v2.clone(), f8(Str::lit("url"))))?, List::cons(f117(Str::lit("media_type"), f341(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f117(Str::lit("license"), f341(v2.clone(), f8(Str::lit("license"))))?, List::cons(f117(Str::lit("redistribution"), f341(v2, f8(Str::lit("redistribution"))))?, List::cons(f117(Str::lit("statement"), f158(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1056 = v1;
        match m1056 {
            None => {
                f7(List::cons(f117(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f117(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f117(Str::lit("artifact"), {
                    let (h1057, _) = v3;
                    h1057
                })?, List::cons(f117(Str::lit("checksum"), {
                    let (_, h1058) = v3;
                    h1058
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f342(f75(f157(f160(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f253(v0: Bytes, v1: u64) -> Option<u8> {
    let a1059 = v0;
    let a1060 = v1;
    index_bytes(a1059, a1060)
}

pub fn f254(v0: u64) -> bool {
    let a1061 = v0;
    let a1062 = 127u64;
    nat_le(a1061, a1062)
}

pub fn f255(v0: Bytes) -> bool {
    f135(v0, f8(Str::lit(".lex.tex")))
}

pub fn f256(v0: Fn3, p1067: u64, p1068: u64) -> R<(u64, bool)> {
    let m1066 = (p1067, p1068);
    {
        let mut m1066 = m1066;
        loop {
            let step = {
                let (v1, v2) = m1066;
                let m1063 = v1;
                if m1063 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1063 - 1;
                    let m1065 = {
                        let c1064: Fn3 = v0.clone();
                        c1064.apply(v2)?
                    };
                    match m1065 {
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
                Ok(next) => { m1066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f257(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f221(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1071 = {
            let a1069 = v4;
            let a1070 = v1;
            nat_add(a1069, a1070)?
        };
        let a1072 = v3;
        if nat_lt(a1071, a1072) {
            Ok(Some({
                let a1073 = v4;
                let a1074 = 1u64;
                nat_add(a1073, a1074)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f258(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1075 = v0;
    match m1075.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f258(v3, {
                let a1076 = v1;
                let a1077 = 1u64;
                nat_add(a1076, a1077)?
            })?))
        }
    }
}

pub fn f259(v0: Adt16, v1: List<u64>, v2: Adt21) -> bool {
    let m1078 = v2;
    match m1078 {
        Adt21::C0(v3, v4) => {
            if f265(v3, match v0.clone() {
                Adt16::C0(h1079, _, _, _) => {
                    h1079
                }
            }) {
                f343(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f265(v5, match v0 {
                Adt16::C0(_, _, h1080, _) => {
                    h1080
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f261(v7, v1) {
                f265(v8, match v0 {
                    Adt16::C0(_, _, _, h1081) => {
                        h1081
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f260(v0: Adt12, v1: Adt14) -> R<Adt21> {
    let m1082 = f160(v1.clone(), f8(Str::lit("std")));
    match m1082 {
        None => {
            let m1087 = f161(f160(v1.clone(), f8(Str::lit("req"))));
            match m1087 {
                None => {
                    let m1089 = f160(v1, f8(Str::lit("der")));
                    match m1089 {
                        None => {
                            Ok(f344(v0))
                        }
                        Some(v5) => {
                            let m1090 = f345(f158(v5.clone(), f8(Str::lit("rule"))));
                            match m1090 {
                                None => {
                                    Ok(f344(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f178(f75(f157(f160(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h1091) => {
                                            h1091
                                        }
                                    })?, f263(f158(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h1092, _) => {
                                            h1092
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f263(v4, match v0 {
                        Adt12::C0(_, _, h1088, _, _) => {
                            h1088
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1083 = f161(f160(v2.clone(), f8(Str::lit("row"))));
            match m1083 {
                None => {
                    Ok(Adt21::C0(f263(f158(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h1084, _, _, _, _) => {
                            h1084
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f263(f158(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h1085, _, _, _, _) => {
                            h1085
                        }
                    })?, Some(f263(v3, match v0 {
                        Adt12::C0(_, h1086, _, _, _) => {
                            h1086
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f261(p1095: List<u64>, v1: List<u64>) -> bool {
    let m1094 = p1095;
    {
        let mut m1094 = m1094;
        loop {
            let step = {
                let v0 = m1094;
                let m1093 = v0;
                match m1093.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f265(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f262(p1100: List<u64>) -> bool {
    let m1099 = p1100;
    {
        let mut m1099 = m1099;
        loop {
            let step = {
                let v0 = m1099;
                let m1096 = v0;
                match m1096.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1097 = v1;
                        let a1098 = 7u64;
                        if nat_le(a1097, a1098) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1099 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1101 = f346(v0, v1.clone(), 0u64)?;
    match m1101 {
        None => {
            let a1102 = v1;
            Ok(length_list(a1102))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f264(v0: Adt14) -> u64 {
    let m1103 = f347(f160(v0, f8(Str::lit("stratum"))));
    match m1103 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f265(v0: u64, p1108: List<u64>) -> bool {
    let m1107 = p1108;
    {
        let mut m1107 = m1107;
        loop {
            let step = {
                let v1 = m1107;
                let m1104 = v1;
                match m1104.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1105 = v2;
                        let a1106 = v0;
                        if nat_eq(a1105, a1106) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f206(v0.clone(), v1) {
        let m1109 = f30(v0, 34u8)?;
        match m1109.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1110 = v3;
                match m1110.uncons() {
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

pub fn f267(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f268(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1111 = v0.clone();
    match m1111.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1112 = v2;
            let a1113 = v1.clone();
            if equal(a1112, a1113) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f269(v0: List<(Bytes, ())>) -> List<Bytes> {
    f183(f348(Fn6::F349, List::<Bytes>::nil(), v0))
}

pub fn f270(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f350(f351(Fn7::F352, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1118 = {
        let (h1114, _) = f353(Fn8::F354, {
            let a1116 = {
                let a1115 = v0;
                length_list(a1115)
            };
            let a1117 = 1u64;
            nat_add(a1116, a1117)?
        }, v1)?;
        h1114
    };
    match m1118.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f271(v0: List<Bytes>) -> List<(Bytes, ())> {
    f355(f356(Fn9::F357, List::<(Bytes, ())>::nil(), v0))
}

pub fn f272(p1121: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1120 = p1121;
    {
        let mut m1120 = m1120;
        loop {
            let step = {
                let v0 = m1120;
                let m1119 = v0;
                match m1119.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f302(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: Adt14, v1: Bytes) -> Bytes {
    if f37(f341(f160(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f274(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1122, _) = v1.clone();
        h1122
    };
    let v3: u64 = {
        let (h1123, _) = {
            let (_, h1124) = v1.clone();
            h1124
        };
        h1123
    };
    let v4: List<Bytes> = {
        let (_, h1125) = {
            let (_, h1126) = v1;
            h1126
        };
        h1125
    };
    let m1127 = f253(v0.clone(), v2);
    match m1127 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1128 = v5;
            let a1129 = 10u8;
            if equal(a1128, a1129) {
                Ok(Some(({
                    let a1130 = v2;
                    let a1131 = 1u64;
                    nat_add(a1130, a1131)?
                }, ({
                    let a1132 = v2;
                    let a1133 = 1u64;
                    nat_add(a1132, a1133)?
                }, List::cons(f358(f32(v0, v3, {
                    let a1134 = v2;
                    let a1135 = v3;
                    nat_sub(a1134, a1135)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1136 = v2;
                    let a1137 = 1u64;
                    nat_add(a1136, a1137)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f275(v0: Adt1, v1: Bytes) -> R<Adt19> {
    let m1138 = f147(v0, v1)?;
    match m1138 {
        None => {
            Ok(Adt19::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f276(v0: Bytes) -> R<Bytes> {
    let a1139 = f8(Str::lit("Catalog."));
    let a1140 = f243(v0)?;
    Ok(append_bytes(a1139, a1140))
}

pub fn f277(v0: Adt19) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f243(f151(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f278(v0: List<Bytes>) -> List<Bytes> {
    let m1141 = v0;
    match m1141.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f135(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f131(f139(), v1), f278(v2))
            } else {
                f278(v2)
            }
        }
    }
}

pub fn f279(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1142 = v1;
    match m1142.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f359(v0.clone(), v2)?;
            let (h1143, _) = v4.clone();
            if h1143 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f279(v0, v3)?;
                let (h1144, _) = v5.clone();
                if h1144 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1147 = {
                            let (_, h1145) = v4;
                            h1145
                        };
                        let a1148 = {
                            let (_, h1146) = v5;
                            h1146
                        };
                        append_bytes(a1147, a1148)
                    }))
                }
            }
        }
    }
}

pub fn f280(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1149 = v1;
    match m1149.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f204(v2.clone(), f8(Str::lit("subject"))) {
                if f205(v0.clone(), v2.clone()) {
                    f280(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f158(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f158(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f158(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f280(v0, v3))
                }
            } else {
                if f289(v0.clone(), v2.clone()) {
                    f280(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f158(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f290(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f280(v0, v3))
                }
            }
        }
    }
}

pub fn f281(v0: Adt1) -> R<Bytes> {
    let a1151 = {
        let a1150 = f104(f105(f123(f124(f113(f71(v0.clone(), f118()))?)), f106(f74(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f107(f74(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1150)
    };
    let a1152 = 0u64;
    if nat_eq(a1151, a1152) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f282(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1153 = v0;
    match m1153.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f122(f75(f157(f160(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f158(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f158(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f360(f84(f158(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f282(v2)?))
        }
    }
}

pub fn f283(v0: Bytes, v1: List<Adt23>) -> List<Bytes> {
    let m1154 = v1;
    match m1154.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(match v2.clone() {
                Adt23::C0(h1155, _, _) => {
                    h1155
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt23::C0(_, h1156, _) => {
                        h1156
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt23::C0(_, _, h1157) => {
                        h1157
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f283(v0, v3))
            } else {
                f283(v0, v3)
            }
        }
    }
}

pub fn f284(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1158 = v1;
    match m1158.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(f158(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f158(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f158(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f284(v0, v3))
            } else {
                if f37(f158(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f158(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f158(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f284(v0, v3))
                } else {
                    f284(v0, v3)
                }
            }
        }
    }
}

pub fn f285(p1163: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1162 = p1163;
    {
        let mut m1162 = m1162;
        loop {
            let step = {
                let v0 = m1162;
                let m1159 = v0;
                match m1159.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1160 = v2;
                        match m1160.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f37(v4, v1.clone()) {
                                    let m1161 = v5;
                                    match m1161.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f361(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f286(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1164 = v0;
    match m1164.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1165 = f160(v1, f8(Str::lit("std")));
            match m1165 {
                None => {
                    f286(v2)
                }
                Some(v3) => {
                    if f204(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f158(v3.clone(), f8(Str::lit("row"))), f158(v3, f8(Str::lit("authority")))), f286(v2))
                    } else {
                        f286(v2)
                    }
                }
            }
        }
    }
}

pub fn f287(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1166 = v1;
    match m1166.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f158(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f158(v2, f8(Str::lit("object")));
            if if f87(v4.clone(), match v0.clone() {
                Adt15::C0(h1167, _, _) => {
                    h1167
                }
            }) {
                f87(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1168, _) => {
                        h1168
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f287(v0, v3))
            } else {
                if if f87(v5.clone(), match v0.clone() {
                    Adt15::C0(h1169, _, _) => {
                        h1169
                    }
                }) {
                    f87(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1170, _) => {
                            h1170
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f287(v0, v3))
                } else {
                    f287(v0, v3)
                }
            }
        }
    }
}

pub fn f288(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt23>> {
    let m1171 = v0;
    match m1171.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v4, v5)) => {
            if if f362(v4.clone(), v2.clone()) {
                true
            } else {
                f362(v4.clone(), v3.clone())
            } {
                f288(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt23::C0({
                    let (h1172, _) = v4.clone();
                    h1172
                }, {
                    let a1175 = {
                        let a1174 = f79(f363({
                            let (h1173, _) = v4.clone();
                            h1173
                        }, v1.clone()))?;
                        length_list(a1174)
                    };
                    let a1176 = 1u64;
                    if nat_eq(a1175, a1176) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1177) = v4.clone();
                    h1177
                }), f288(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f289(v0: Adt15, v1: Adt14) -> bool {
    if f87(f158(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1178, _, _) => {
            h1178
        }
    }) {
        f87(f290(v1), match v0 {
            Adt15::C0(_, h1179, _) => {
                h1179
            }
        })
    } else {
        false
    }
}

pub fn f290(v0: Adt14) -> Bytes {
    let m1180 = f160(v0, f8(Str::lit("evidence")));
    match m1180 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f158(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f291(v0: Adt14) -> Bytes {
    if f204(v0.clone(), f8(Str::lit("req"))) {
        let a1181 = f8(Str::lit("req "));
        let a1182 = f158(v0, f8(Str::lit("req")));
        append_bytes(a1181, a1182)
    } else {
        let m1183 = f160(v0.clone(), f8(Str::lit("std")));
        match m1183 {
            None => {
                let m1186 = f160(v0, f8(Str::lit("der")));
                match m1186 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f158(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f122(f75(f157(f160(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f158(v1.clone(), f8(Str::lit("authority"))), List::cons(if f204(v1.clone(), f8(Str::lit("row"))) {
                    let a1184 = f8(Str::lit(" row "));
                    let a1185 = f158(v1, f8(Str::lit("row")));
                    append_bytes(a1184, a1185)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f292(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1187 = v1;
    match m1187.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f87({
                let (h1188, _) = v2.clone();
                h1188
            }, v0.clone()) {
                List::cons({
                    let (_, h1189) = v2;
                    h1189
                }, f292(v0, v3))
            } else {
                if f87({
                    let (_, h1190) = v2;
                    h1190
                }, v0.clone()) {
                    List::cons({
                        let (h1191, _) = v2;
                        h1191
                    }, f292(v0, v3))
                } else {
                    f292(v0, v3)
                }
            }
        }
    }
}

pub fn f293(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f87(v2.clone(), match v0.clone() {
        Adt15::C0(h1192, _, _) => {
            h1192
        }
    }) {
        true
    } else {
        if f87(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1193, _) => {
                h1193
            }
        }) {
            true
        } else {
            if f87(v2, match v0.clone() {
                Adt15::C0(_, _, h1194) => {
                    h1194
                }
            }) {
                true
            } else {
                if f204(v1.clone(), f8(Str::lit("scheme"))) {
                    f87(f158(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1195, _) => {
                            h1195
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f294(v0: Bytes, v1: u64) -> R<u64> {
    let m1198 = {
        let a1196 = v0;
        let a1197 = v1;
        index_bytes(a1196, a1197)
    };
    match m1198 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1243 = {
                let a1219 = {
                    let a1207 = {
                        let a1201 = {
                            let a1199 = v3;
                            let a1200 = 128u8;
                            fixed_u8::bit_and(a1199, a1200)
                        };
                        let a1202 = 128u8;
                        if equal(a1201, a1202) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1208 = {
                        let a1205 = {
                            let a1203 = v3;
                            let a1204 = 64u8;
                            fixed_u8::bit_and(a1203, a1204)
                        };
                        let a1206 = 64u8;
                        if equal(a1205, a1206) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1207, a1208)?
                };
                let a1220 = {
                    let a1217 = {
                        let a1211 = {
                            let a1209 = v3;
                            let a1210 = 32u8;
                            fixed_u8::bit_and(a1209, a1210)
                        };
                        let a1212 = 32u8;
                        if equal(a1211, a1212) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1218 = {
                        let a1215 = {
                            let a1213 = v3;
                            let a1214 = 16u8;
                            fixed_u8::bit_and(a1213, a1214)
                        };
                        let a1216 = 16u8;
                        if equal(a1215, a1216) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1217, a1218)?
                };
                nat_add(a1219, a1220)?
            };
            let a1244 = {
                let a1241 = {
                    let a1229 = {
                        let a1223 = {
                            let a1221 = v3;
                            let a1222 = 8u8;
                            fixed_u8::bit_and(a1221, a1222)
                        };
                        let a1224 = 8u8;
                        if equal(a1223, a1224) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1230 = {
                        let a1227 = {
                            let a1225 = v3;
                            let a1226 = 4u8;
                            fixed_u8::bit_and(a1225, a1226)
                        };
                        let a1228 = 4u8;
                        if equal(a1227, a1228) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1229, a1230)?
                };
                let a1242 = {
                    let a1239 = {
                        let a1233 = {
                            let a1231 = v3;
                            let a1232 = 2u8;
                            fixed_u8::bit_and(a1231, a1232)
                        };
                        let a1234 = 2u8;
                        if equal(a1233, a1234) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1240 = {
                        let a1237 = {
                            let a1235 = v3;
                            let a1236 = 1u8;
                            fixed_u8::bit_and(a1235, a1236)
                        };
                        let a1238 = 1u8;
                        if equal(a1237, a1238) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1239, a1240)?
                };
                nat_add(a1241, a1242)?
            };
            nat_add(a1243, a1244)
        }
    }
}

pub fn f295(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1248 = {
        let a1245 = v1;
        let a1246 = v2;
        let a1247 = 1u64;
        slice_bytes(a1245, a1246, a1247)
    };
    match m1248 {
        None => {
            v0
        }
        Some(v3) => {
            let a1249 = v0;
            let a1250 = v3;
            append_bytes(a1249, a1250)
        }
    }
}

pub fn f296(p1253: List<Bytes>, p1254: List<Bytes>) -> List<Bytes> {
    let m1252 = (p1253, p1254);
    {
        let mut m1252 = m1252;
        loop {
            let step = {
                let (v0, v1) = m1252;
                let m1251 = v0;
                match m1251.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: Bytes, p1326: u64, p1327: u64, p1328: Bytes) -> R<Bytes> {
    let m1325 = (p1326, (p1327, p1328));
    {
        let mut m1325 = m1325;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1325;
                let m1255 = v2;
                if m1255 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1255 - 1;
                    let m1258 = {
                        let a1256 = v0.clone();
                        let a1257 = v1;
                        index_bytes(a1256, a1257)
                    };
                    match m1258 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1303 = {
                                    let a1279 = {
                                        let a1267 = {
                                            let a1261 = {
                                                let a1259 = v6;
                                                let a1260 = 128u8;
                                                fixed_u8::bit_and(a1259, a1260)
                                            };
                                            let a1262 = 128u8;
                                            if equal(a1261, a1262) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1268 = {
                                            let a1265 = {
                                                let a1263 = v6;
                                                let a1264 = 64u8;
                                                fixed_u8::bit_and(a1263, a1264)
                                            };
                                            let a1266 = 64u8;
                                            if equal(a1265, a1266) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1267, a1268)?
                                    };
                                    let a1280 = {
                                        let a1277 = {
                                            let a1271 = {
                                                let a1269 = v6;
                                                let a1270 = 32u8;
                                                fixed_u8::bit_and(a1269, a1270)
                                            };
                                            let a1272 = 32u8;
                                            if equal(a1271, a1272) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1278 = {
                                            let a1275 = {
                                                let a1273 = v6;
                                                let a1274 = 16u8;
                                                fixed_u8::bit_and(a1273, a1274)
                                            };
                                            let a1276 = 16u8;
                                            if equal(a1275, a1276) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1277, a1278)?
                                    };
                                    nat_add(a1279, a1280)?
                                };
                                let a1304 = {
                                    let a1301 = {
                                        let a1289 = {
                                            let a1283 = {
                                                let a1281 = v6;
                                                let a1282 = 8u8;
                                                fixed_u8::bit_and(a1281, a1282)
                                            };
                                            let a1284 = 8u8;
                                            if equal(a1283, a1284) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1290 = {
                                            let a1287 = {
                                                let a1285 = v6;
                                                let a1286 = 4u8;
                                                fixed_u8::bit_and(a1285, a1286)
                                            };
                                            let a1288 = 4u8;
                                            if equal(a1287, a1288) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1289, a1290)?
                                    };
                                    let a1302 = {
                                        let a1299 = {
                                            let a1293 = {
                                                let a1291 = v6;
                                                let a1292 = 2u8;
                                                fixed_u8::bit_and(a1291, a1292)
                                            };
                                            let a1294 = 2u8;
                                            if equal(a1293, a1294) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1300 = {
                                            let a1297 = {
                                                let a1295 = v6;
                                                let a1296 = 1u8;
                                                fixed_u8::bit_and(a1295, a1296)
                                            };
                                            let a1298 = 1u8;
                                            if equal(a1297, a1298) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1299, a1300)?
                                    };
                                    nat_add(a1301, a1302)?
                                };
                                nat_add(a1303, a1304)?
                            };
                            let a1305 = v7;
                            let a1306 = 34u64;
                            if if nat_eq(a1305, a1306) {
                                true
                            } else {
                                let a1307 = v7;
                                let a1308 = 92u64;
                                nat_eq(a1307, a1308)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1309 = v1;
                                    let a1310 = 1u64;
                                    nat_add(a1309, a1310)?
                                }, (v4, {
                                    let a1313 = v3;
                                    let a1314 = {
                                        let a1311 = Bytes::lit(&[0x5c]);
                                        let a1312 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1311, a1312)
                                    };
                                    append_bytes(a1313, a1314)
                                })))
                            } else {
                                let a1315 = v7;
                                let a1316 = 10u64;
                                if nat_eq(a1315, a1316) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1317 = v1;
                                        let a1318 = 1u64;
                                        nat_add(a1317, a1318)?
                                    }, (v4, {
                                        let a1319 = v3;
                                        let a1320 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1319, a1320)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1321 = v1;
                                        let a1322 = 1u64;
                                        nat_add(a1321, a1322)?
                                    }, (v4, {
                                        let a1323 = v3;
                                        let a1324 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1323, a1324)
                                    })))
                                }
                            }
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

pub fn f298(v0: u32) -> Bytes {
    let a1357 = f364({
        let a1329 = f365(v0, 28u32);
        let a1330 = 15u32;
        fixed_u32::bit_and(a1329, a1330)
    });
    let a1358 = {
        let a1355 = f364({
            let a1331 = f365(v0, 24u32);
            let a1332 = 15u32;
            fixed_u32::bit_and(a1331, a1332)
        });
        let a1356 = {
            let a1353 = f364({
                let a1333 = f365(v0, 20u32);
                let a1334 = 15u32;
                fixed_u32::bit_and(a1333, a1334)
            });
            let a1354 = {
                let a1351 = f364({
                    let a1335 = f365(v0, 16u32);
                    let a1336 = 15u32;
                    fixed_u32::bit_and(a1335, a1336)
                });
                let a1352 = {
                    let a1349 = f364({
                        let a1337 = f365(v0, 12u32);
                        let a1338 = 15u32;
                        fixed_u32::bit_and(a1337, a1338)
                    });
                    let a1350 = {
                        let a1347 = f364({
                            let a1339 = f365(v0, 8u32);
                            let a1340 = 15u32;
                            fixed_u32::bit_and(a1339, a1340)
                        });
                        let a1348 = {
                            let a1345 = f364({
                                let a1341 = f365(v0, 4u32);
                                let a1342 = 15u32;
                                fixed_u32::bit_and(a1341, a1342)
                            });
                            let a1346 = f364({
                                let a1343 = v0;
                                let a1344 = 15u32;
                                fixed_u32::bit_and(a1343, a1344)
                            });
                            append_bytes(a1345, a1346)
                        };
                        append_bytes(a1347, a1348)
                    };
                    append_bytes(a1349, a1350)
                };
                append_bytes(a1351, a1352)
            };
            append_bytes(a1353, a1354)
        };
        append_bytes(a1355, a1356)
    };
    append_bytes(a1357, a1358)
}

pub fn f299(v0: Fn4, p1362: u64, p1363: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1361 = (p1362, p1363);
    {
        let mut m1361 = m1361;
        loop {
            let step = {
                let (v1, v2) = m1361;
                let m1359 = v1;
                if m1359 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1359 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1360: Fn4 = v0.clone();
                        c1360.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1361 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1365 = {
            let (h1364, _) = v3.clone();
            h1364
        };
        let a1366 = 64u64;
        nat_add(a1365, a1366)?
    }, f366({
        let (_, h1367) = v3;
        h1367
    }, f367(v0, v1, v2, {
        let (h1368, _) = v3;
        h1368
    })?)?))
}

pub fn f301() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f302(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1369 = f30(v1, 34u8)?;
    match m1369.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1370 = v3;
            match m1370.uncons() {
                None => {
                    Ok(f368(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f303(v0: List<Bytes>) -> List<Bytes> {
    let m1371 = v0;
    match m1371.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f37(v1.clone(), f8(Str::lit("authority = []"))) {
                f303(v2)
            } else {
                List::cons(v1, f303(v2))
            }
        }
    }
}

pub fn f304(v0: Bytes) -> R<u64> {
    let (h1372, _) = f256(Fn3::F369(v0.clone()), {
        let a1373 = v0.clone();
        length_bytes(a1373)
    }, {
        let a1374 = v0;
        length_bytes(a1374)
    })?;
    Ok(h1372)
}

pub fn f305(v0: Adt14) -> R<Bytes> {
    let m1375 = v0;
    match m1375 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f334(v2, v3)
        }
        Adt14::C2(v4) => {
            f336(v4)
        }
        Adt14::C3(r1376) => {
            let v5 = (*r1376).clone();
            Ok(f337(f370(v5)?))
        }
        Adt14::C4(r1377) => {
            let v6 = (*r1377).clone();
            f339(f371(v6)?)
        }
        Adt14::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt14::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt14::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f306(v0: Fn5, p1382: u64, p1383: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1381 = (p1382, p1383);
    {
        let mut m1381 = m1381;
        loop {
            let step = {
                let (v1, v2) = m1381;
                let m1378 = v1;
                if m1378 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1378 - 1;
                    let m1380 = {
                        let c1379: Fn5 = v0.clone();
                        c1379.apply(v2.clone())?
                    };
                    match m1380 {
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
                Ok(next) => { m1381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f307(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f372(v0, v1)
}

pub fn f308(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f373(v0, v1)
}

pub fn f309(v0: u64) -> bool {
    let a1384 = v0;
    let a1385 = 32u64;
    if if nat_eq(a1384, a1385) {
        true
    } else {
        let a1386 = v0;
        let a1387 = 9u64;
        nat_eq(a1386, a1387)
    } {
        true
    } else {
        let a1388 = v0;
        let a1389 = 10u64;
        if nat_eq(a1388, a1389) {
            true
        } else {
            let a1390 = v0;
            let a1391 = 13u64;
            nat_eq(a1390, a1391)
        }
    }
}

pub fn f310(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1392, _) = f256(Fn3::F374(v1.clone(), v0.clone()), {
            let a1394 = {
                let a1393 = v0.clone();
                length_bytes(a1393)
            };
            let a1395 = 1u64;
            nat_add(a1394, a1395)?
        }, v2)?;
        h1392
    };
    if f221(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f311(v0: Bytes, v1: u64) -> R<u64> {
    let (h1396, _) = f256(Fn3::F375(v0.clone()), {
        let a1398 = {
            let a1397 = v0;
            length_bytes(a1397)
        };
        let a1399 = 1u64;
        nat_add(a1398, a1399)?
    }, v1)?;
    Ok(h1396)
}

pub fn f312(v0: Bytes) -> R<bool> {
    let a1401 = {
        let a1400 = v0.clone();
        length_bytes(a1400)
    };
    let a1402 = 5u64;
    if nat_eq(a1401, a1402) {
        let a1403 = f315(v0.clone(), 0u64)?;
        let a1404 = 85u64;
        if if nat_eq(a1403, a1404) {
            true
        } else {
            let a1405 = f315(v0.clone(), 0u64)?;
            let a1406 = 117u64;
            nat_eq(a1405, a1406)
        } {
            let a1407 = f315(v0.clone(), 1u64)?;
            let a1408 = 84u64;
            if if nat_eq(a1407, a1408) {
                true
            } else {
                let a1409 = f315(v0.clone(), 1u64)?;
                let a1410 = 116u64;
                nat_eq(a1409, a1410)
            } {
                let a1411 = f315(v0.clone(), 2u64)?;
                let a1412 = 70u64;
                if if nat_eq(a1411, a1412) {
                    true
                } else {
                    let a1413 = f315(v0.clone(), 2u64)?;
                    let a1414 = 102u64;
                    nat_eq(a1413, a1414)
                } {
                    let a1415 = f315(v0.clone(), 3u64)?;
                    let a1416 = 45u64;
                    if nat_eq(a1415, a1416) {
                        let a1417 = f315(v0, 4u64)?;
                        let a1418 = 56u64;
                        Ok(nat_eq(a1417, a1418))
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f313(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1422 = f310(v0.clone(), {
        let a1419 = Str::lit("--");
        utf8_encode(a1419)
    }, {
        let a1420 = v1;
        let a1421 = 4u64;
        nat_add(a1420, a1421)?
    })?;
    match m1422 {
        None => {
            Ok(Err::<u64, Bytes>(f224(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1425 = f228(v0, {
                let a1423 = v2;
                let a1424 = 2u64;
                nat_add(a1423, a1424)?
            })?;
            let a1426 = 62u64;
            if nat_eq(a1425, a1426) {
                Ok(Ok::<u64, Bytes>({
                    let a1427 = v2;
                    let a1428 = 3u64;
                    nat_add(a1427, a1428)?
                }))
            } else {
                Ok(Err::<u64, Bytes>(f224(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f314(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f316(v0.clone(), {
        let a1429 = v1;
        let a1430 = 2u64;
        nat_add(a1429, a1430)?
    })?;
    let a1433 = v2;
    let a1434 = {
        let a1431 = v1;
        let a1432 = 2u64;
        nat_add(a1431, a1432)?
    };
    if nat_eq(a1433, a1434) {
        Ok(Err::<u64, Bytes>(f224(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1437 = v2;
        let a1438 = {
            let a1435 = v1;
            let a1436 = 5u64;
            nat_add(a1435, a1436)?
        };
        if if nat_eq(a1437, a1438) {
            let a1441 = f228(v0.clone(), {
                let a1439 = v1;
                let a1440 = 2u64;
                nat_add(a1439, a1440)?
            })?;
            let a1442 = 120u64;
            if if nat_eq(a1441, a1442) {
                true
            } else {
                let a1445 = f228(v0.clone(), {
                    let a1443 = v1;
                    let a1444 = 2u64;
                    nat_add(a1443, a1444)?
                })?;
                let a1446 = 88u64;
                nat_eq(a1445, a1446)
            } {
                let a1449 = f228(v0.clone(), {
                    let a1447 = v1;
                    let a1448 = 3u64;
                    nat_add(a1447, a1448)?
                })?;
                let a1450 = 109u64;
                if if nat_eq(a1449, a1450) {
                    true
                } else {
                    let a1453 = f228(v0.clone(), {
                        let a1451 = v1;
                        let a1452 = 3u64;
                        nat_add(a1451, a1452)?
                    })?;
                    let a1454 = 77u64;
                    nat_eq(a1453, a1454)
                } {
                    let a1457 = f228(v0.clone(), {
                        let a1455 = v1;
                        let a1456 = 4u64;
                        nat_add(a1455, a1456)?
                    })?;
                    let a1458 = 108u64;
                    if nat_eq(a1457, a1458) {
                        true
                    } else {
                        let a1461 = f228(v0.clone(), {
                            let a1459 = v1;
                            let a1460 = 4u64;
                            nat_add(a1459, a1460)?
                        })?;
                        let a1462 = 76u64;
                        nat_eq(a1461, a1462)
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f224(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1464 = f310(v0, {
                let a1463 = Str::lit("?>");
                utf8_encode(a1463)
            }, v2)?;
            match m1464 {
                None => {
                    Ok(Err::<u64, Bytes>(f224(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1465 = v3;
                        let a1466 = 2u64;
                        nat_add(a1465, a1466)?
                    }))
                }
            }
        }
    }
}

pub fn f315(v0: Bytes, v1: u64) -> R<u64> {
    let m1469 = {
        let a1467 = v0;
        let a1468 = v1;
        index_bytes(a1467, a1468)
    };
    match m1469 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1514 = {
                let a1490 = {
                    let a1478 = {
                        let a1472 = {
                            let a1470 = v3;
                            let a1471 = 128u8;
                            fixed_u8::bit_and(a1470, a1471)
                        };
                        let a1473 = 128u8;
                        if equal(a1472, a1473) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1479 = {
                        let a1476 = {
                            let a1474 = v3;
                            let a1475 = 64u8;
                            fixed_u8::bit_and(a1474, a1475)
                        };
                        let a1477 = 64u8;
                        if equal(a1476, a1477) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1478, a1479)?
                };
                let a1491 = {
                    let a1488 = {
                        let a1482 = {
                            let a1480 = v3;
                            let a1481 = 32u8;
                            fixed_u8::bit_and(a1480, a1481)
                        };
                        let a1483 = 32u8;
                        if equal(a1482, a1483) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1489 = {
                        let a1486 = {
                            let a1484 = v3;
                            let a1485 = 16u8;
                            fixed_u8::bit_and(a1484, a1485)
                        };
                        let a1487 = 16u8;
                        if equal(a1486, a1487) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1488, a1489)?
                };
                nat_add(a1490, a1491)?
            };
            let a1515 = {
                let a1512 = {
                    let a1500 = {
                        let a1494 = {
                            let a1492 = v3;
                            let a1493 = 8u8;
                            fixed_u8::bit_and(a1492, a1493)
                        };
                        let a1495 = 8u8;
                        if equal(a1494, a1495) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1501 = {
                        let a1498 = {
                            let a1496 = v3;
                            let a1497 = 4u8;
                            fixed_u8::bit_and(a1496, a1497)
                        };
                        let a1499 = 4u8;
                        if equal(a1498, a1499) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1500, a1501)?
                };
                let a1513 = {
                    let a1510 = {
                        let a1504 = {
                            let a1502 = v3;
                            let a1503 = 2u8;
                            fixed_u8::bit_and(a1502, a1503)
                        };
                        let a1505 = 2u8;
                        if equal(a1504, a1505) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1511 = {
                        let a1508 = {
                            let a1506 = v3;
                            let a1507 = 1u8;
                            fixed_u8::bit_and(a1506, a1507)
                        };
                        let a1509 = 1u8;
                        if equal(a1508, a1509) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1510, a1511)?
                };
                nat_add(a1512, a1513)?
            };
            nat_add(a1514, a1515)
        }
    }
}

pub fn f316(v0: Bytes, v1: u64) -> R<u64> {
    let (h1516, _) = f256(Fn3::F376(v0.clone(), v1), {
        let a1518 = {
            let a1517 = v0;
            length_bytes(a1517)
        };
        let a1519 = 1u64;
        nat_add(a1518, a1519)?
    }, v1)?;
    Ok(h1516)
}

pub fn f317(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt18, u64), Bytes>> {
    let m1520 = v2;
    if m1520 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1520 - 1;
        let v5: u64 = f311(v0.clone(), v1)?;
        let a1521 = f228(v0.clone(), v5)?;
        let a1522 = 62u64;
        if if nat_eq(a1521, a1522) {
            true
        } else {
            f227(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v5)))
        } else {
            let a1523 = v5;
            let a1524 = v1;
            if nat_eq(a1523, a1524) {
                Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f316(v0.clone(), v5)?;
                let a1525 = v6;
                let a1526 = v5;
                if nat_eq(a1525, a1526) {
                    Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a1527 = v6;
                        let a1528 = v5;
                        nat_sub(a1527, a1528)
                    });
                    if f377(v7.clone(), v3.clone()) {
                        Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f311(v0.clone(), v6)?;
                        let a1529 = f228(v0.clone(), v8)?;
                        let a1530 = 61u64;
                        if !nat_eq(a1529, a1530) {
                            Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        } else {
                            let v9: u64 = f311(v0.clone(), {
                                let a1531 = v8;
                                let a1532 = 1u64;
                                nat_add(a1531, a1532)?
                            })?;
                            let v10: u64 = f228(v0.clone(), v9)?;
                            let a1533 = v10;
                            let a1534 = 34u64;
                            if !if nat_eq(a1533, a1534) {
                                true
                            } else {
                                let a1535 = v10;
                                let a1536 = 39u64;
                                nat_eq(a1535, a1536)
                            } {
                                Ok(Err::<(Adt18, u64), Bytes>(f224(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            } else {
                                let m1539 = f318(v0.clone(), {
                                    let a1537 = v9;
                                    let a1538 = 1u64;
                                    nat_add(a1537, a1538)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1539 {
                                    Ok(v12) => {
                                        let m1541 = f317(v0, {
                                            let (_, h1540) = v12;
                                            h1540
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1541 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C1(v7, {
                                                    let (h1542, _) = v12;
                                                    h1542
                                                }, std::rc::Rc::new({
                                                    let (h1543, _) = v14;
                                                    h1543
                                                })), {
                                                    let (_, h1544) = v14;
                                                    h1544
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt18, u64), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v11))
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

pub fn f318(v0: Bytes, p1584: u64, p1585: u64, p1586: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1583 = (p1584, (p1585, p1586));
    {
        let mut m1583 = m1583;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1583;
                let m1545 = v2;
                if m1545 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f224(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1545 - 1;
                    let v7: u64 = f228(v0.clone(), v1)?;
                    if if v4 {
                        let a1546 = v7;
                        let a1547 = v5;
                        nat_eq(a1546, a1547)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1548 = v1;
                            let a1549 = 1u64;
                            nat_add(a1548, a1549)?
                        }))))
                    } else {
                        let a1550 = v7;
                        let a1551 = 256u64;
                        if nat_eq(a1550, a1551) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f224(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1552 = v7;
                            let a1553 = 60u64;
                            if nat_eq(a1552, a1553) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f224(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1554 = v7;
                                let a1555 = 38u64;
                                if nat_eq(a1554, a1555) {
                                    let m1556 = f378(v0.clone(), v1)?;
                                    match m1556 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1557) = v9;
                                                h1557
                                            }, (v6, {
                                                let a1559 = v3;
                                                let a1560 = {
                                                    let (h1558, _) = v9;
                                                    h1558
                                                };
                                                append_bytes(a1559, a1560)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1561 = v7;
                                        let a1562 = 9u64;
                                        if if nat_eq(a1561, a1562) {
                                            true
                                        } else {
                                            let a1563 = v7;
                                            let a1564 = 10u64;
                                            nat_eq(a1563, a1564)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1565 = v1;
                                                let a1566 = 1u64;
                                                nat_add(a1565, a1566)?
                                            }, (v6, {
                                                let a1567 = v3;
                                                let a1568 = Bytes::lit(&[0x20]);
                                                append_bytes(a1567, a1568)
                                            })))
                                        } else {
                                            let v10: u64 = f379(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1571 = v3;
                                                let a1572 = f32(v0.clone(), v1, {
                                                    let a1569 = v10;
                                                    let a1570 = v1;
                                                    nat_sub(a1569, a1570)
                                                });
                                                append_bytes(a1571, a1572)
                                            })))
                                        }
                                    } else {
                                        let a1573 = v7;
                                        let a1574 = 93u64;
                                        if nat_eq(a1573, a1574) {
                                            if f227(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f224(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1575 = v1;
                                                    let a1576 = 1u64;
                                                    nat_add(a1575, a1576)?
                                                }, (v6, {
                                                    let a1577 = v3;
                                                    let a1578 = Bytes::lit(&[0x5d]);
                                                    append_bytes(a1577, a1578)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f379(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1581 = v3;
                                                let a1582 = f32(v0.clone(), v1, {
                                                    let a1579 = v11;
                                                    let a1580 = v1;
                                                    nat_sub(a1579, a1580)
                                                });
                                                append_bytes(a1581, a1582)
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
                Ok(next) => { m1583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: Bytes, v1: u64) -> Bytes {
    let m1590 = {
        let a1587 = v0;
        let a1588 = v1;
        let a1589 = 1u64;
        slice_bytes(a1587, a1588, a1589)
    };
    match m1590 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f320(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f380(v1, v0, v2)
}

pub fn f321(v0: Bytes, v1: u64) -> R<u64> {
    let (h1591, _) = f256(Fn3::F381(v1, v0), v1, 0u64)?;
    Ok(h1591)
}

pub fn f322(v0: Bytes) -> R<Bytes> {
    f382(v0.clone(), 0u64, {
        let a1592 = v0;
        length_bytes(a1592)
    }, false, false, Bytes::lit(&[]))
}

pub fn f323(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1593 = v2;
    if m1593 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1593 - 1;
        let v5: u64 = f315(v0.clone(), v1)?;
        let a1594 = v3;
        let a1595 = 1u64;
        if nat_eq(a1594, a1595) {
            let m1596 = f323(v0.clone(), v1, v4, 0u64)?;
            match m1596 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f315(v0.clone(), {
                        let (_, h1597) = v6.clone();
                        h1597
                    })?;
                    let a1598 = v7;
                    let a1599 = 44u64;
                    if nat_eq(a1598, a1599) {
                        let m1603 = f323(v0, {
                            let a1601 = {
                                let (_, h1600) = v6;
                                h1600
                            };
                            let a1602 = 1u64;
                            nat_add(a1601, a1602)?
                        }, v4, 1u64)?;
                        match m1603 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1604, _) = v6;
                                    h1604
                                }), std::rc::Rc::new({
                                    let (h1605, _) = v8;
                                    h1605
                                })), {
                                    let (_, h1606) = v8;
                                    h1606
                                })))
                            }
                        }
                    } else {
                        let a1607 = v7;
                        let a1608 = 93u64;
                        if nat_eq(a1607, a1608) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1609, _) = v6;
                                h1609
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1611 = {
                                    let (_, h1610) = v6;
                                    h1610
                                };
                                let a1612 = 1u64;
                                nat_add(a1611, a1612)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1613 = v3;
            let a1614 = 2u64;
            if nat_eq(a1613, a1614) {
                let m1617 = f383(v0.clone(), {
                    let a1615 = v1;
                    let a1616 = 1u64;
                    nat_add(a1615, a1616)?
                })?;
                match m1617 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1618 = v5;
                        let a1619 = 34u64;
                        if if nat_eq(a1618, a1619) {
                            let a1621 = f315(v0.clone(), {
                                let (h1620, _) = v9.clone();
                                h1620
                            })?;
                            let a1622 = 58u64;
                            nat_eq(a1621, a1622)
                        } else {
                            false
                        } {
                            let m1626 = f323(v0.clone(), {
                                let a1624 = {
                                    let (h1623, _) = v9;
                                    h1623
                                };
                                let a1625 = 1u64;
                                nat_add(a1624, a1625)?
                            }, v4, 0u64)?;
                            match m1626 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f315(v0.clone(), {
                                        let (_, h1627) = v10.clone();
                                        h1627
                                    })?;
                                    let a1628 = v11;
                                    let a1629 = 44u64;
                                    if nat_eq(a1628, a1629) {
                                        let m1633 = f323(v0, {
                                            let a1631 = {
                                                let (_, h1630) = v10;
                                                h1630
                                            };
                                            let a1632 = 1u64;
                                            nat_add(a1631, a1632)?
                                        }, v4, 2u64)?;
                                        match m1633 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1634) = v9;
                                                    h1634
                                                }, std::rc::Rc::new({
                                                    let (h1635, _) = v10;
                                                    h1635
                                                }), std::rc::Rc::new({
                                                    let (h1636, _) = v12;
                                                    h1636
                                                })), {
                                                    let (_, h1637) = v12;
                                                    h1637
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1638 = v11;
                                        let a1639 = 125u64;
                                        if nat_eq(a1638, a1639) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1640) = v9;
                                                h1640
                                            }, std::rc::Rc::new({
                                                let (h1641, _) = v10;
                                                h1641
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1643 = {
                                                    let (_, h1642) = v10;
                                                    h1642
                                                };
                                                let a1644 = 1u64;
                                                nat_add(a1643, a1644)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            } else {
                let a1645 = v5;
                let a1646 = 34u64;
                if nat_eq(a1645, a1646) {
                    let m1649 = f383(v0, {
                        let a1647 = v1;
                        let a1648 = 1u64;
                        nat_add(a1647, a1648)?
                    })?;
                    match m1649 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1650) = v13;
                                h1650
                            }), {
                                let (h1651, _) = v13;
                                h1651
                            })))
                        }
                    }
                } else {
                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1652 = v1;
                            let a1653 = 4u64;
                            nat_add(a1652, a1653)?
                        })))
                    } else {
                        if f37(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1654 = v1;
                                let a1655 = 5u64;
                                nat_add(a1654, a1655)?
                            })))
                        } else {
                            let a1656 = v5;
                            let a1657 = 91u64;
                            if nat_eq(a1656, a1657) {
                                let a1660 = f315(v0.clone(), {
                                    let a1658 = v1;
                                    let a1659 = 1u64;
                                    nat_add(a1658, a1659)?
                                })?;
                                let a1661 = 93u64;
                                if nat_eq(a1660, a1661) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1662 = v1;
                                        let a1663 = 2u64;
                                        nat_add(a1662, a1663)?
                                    })))
                                } else {
                                    let m1666 = f323(v0, {
                                        let a1664 = v1;
                                        let a1665 = 1u64;
                                        nat_add(a1664, a1665)?
                                    }, v4, 1u64)?;
                                    match m1666 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1667, _) = v14;
                                                h1667
                                            })), {
                                                let (_, h1668) = v14;
                                                h1668
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1669 = v5;
                                let a1670 = 123u64;
                                if nat_eq(a1669, a1670) {
                                    let a1673 = f315(v0.clone(), {
                                        let a1671 = v1;
                                        let a1672 = 1u64;
                                        nat_add(a1671, a1672)?
                                    })?;
                                    let a1674 = 125u64;
                                    if nat_eq(a1673, a1674) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1675 = v1;
                                            let a1676 = 2u64;
                                            nat_add(a1675, a1676)?
                                        })))
                                    } else {
                                        let m1679 = f323(v0, {
                                            let a1677 = v1;
                                            let a1678 = 1u64;
                                            nat_add(a1677, a1678)?
                                        }, v4, 2u64)?;
                                        match m1679 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1680, _) = v15;
                                                    h1680
                                                })), {
                                                    let (_, h1681) = v15;
                                                    h1681
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1682 = f384(v0, v1)?;
                                    match m1682 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1683) = v16;
                                                h1683
                                            }, {
                                                let (h1684, _) = v16;
                                                h1684
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

pub fn f324() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f325(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1685 = f160(v0, v1);
    match m1685 {
        None => {
            false
        }
        Some(v3) => {
            f204(v3, v2)
        }
    }
}

pub fn f326() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f327(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt19, u64)>> {
    let m1686 = v2;
    if m1686 == 0 {
        Ok(None::<(Adt19, u64)>)
    } else {
        let v4 = m1686 - 1;
        let v5: u64 = f315(v0.clone(), v1)?;
        let a1687 = v3;
        let a1688 = 1u64;
        if nat_eq(a1687, a1688) {
            let m1689 = f327(v0.clone(), v1, v4, 0u64)?;
            match m1689 {
                None => {
                    Ok(None::<(Adt19, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f315(v0.clone(), {
                        let (_, h1690) = v6.clone();
                        h1690
                    })?;
                    let a1691 = v7;
                    let a1692 = 44u64;
                    if nat_eq(a1691, a1692) {
                        let m1696 = f327(v0, {
                            let a1694 = {
                                let (_, h1693) = v6;
                                h1693
                            };
                            let a1695 = 1u64;
                            nat_add(a1694, a1695)?
                        }, v4, 1u64)?;
                        match m1696 {
                            None => {
                                Ok(None::<(Adt19, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt19::C7(std::rc::Rc::new({
                                    let (h1697, _) = v6;
                                    h1697
                                }), std::rc::Rc::new({
                                    let (h1698, _) = v8;
                                    h1698
                                })), {
                                    let (_, h1699) = v8;
                                    h1699
                                })))
                            }
                        }
                    } else {
                        let a1700 = v7;
                        let a1701 = 93u64;
                        if nat_eq(a1700, a1701) {
                            Ok(Some((Adt19::C7(std::rc::Rc::new({
                                let (h1702, _) = v6;
                                h1702
                            }), std::rc::Rc::new(Adt19::C6)), {
                                let a1704 = {
                                    let (_, h1703) = v6;
                                    h1703
                                };
                                let a1705 = 1u64;
                                nat_add(a1704, a1705)?
                            })))
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1706 = v3;
            let a1707 = 2u64;
            if nat_eq(a1706, a1707) {
                let m1710 = f383(v0.clone(), {
                    let a1708 = v1;
                    let a1709 = 1u64;
                    nat_add(a1708, a1709)?
                })?;
                match m1710 {
                    None => {
                        Ok(None::<(Adt19, u64)>)
                    }
                    Some(v9) => {
                        let a1711 = v5;
                        let a1712 = 34u64;
                        if if nat_eq(a1711, a1712) {
                            let a1714 = f315(v0.clone(), {
                                let (h1713, _) = v9.clone();
                                h1713
                            })?;
                            let a1715 = 58u64;
                            nat_eq(a1714, a1715)
                        } else {
                            false
                        } {
                            let m1719 = f327(v0.clone(), {
                                let a1717 = {
                                    let (h1716, _) = v9;
                                    h1716
                                };
                                let a1718 = 1u64;
                                nat_add(a1717, a1718)?
                            }, v4, 0u64)?;
                            match m1719 {
                                None => {
                                    Ok(None::<(Adt19, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f315(v0.clone(), {
                                        let (_, h1720) = v10.clone();
                                        h1720
                                    })?;
                                    let a1721 = v11;
                                    let a1722 = 44u64;
                                    if nat_eq(a1721, a1722) {
                                        let m1726 = f327(v0, {
                                            let a1724 = {
                                                let (_, h1723) = v10;
                                                h1723
                                            };
                                            let a1725 = 1u64;
                                            nat_add(a1724, a1725)?
                                        }, v4, 2u64)?;
                                        match m1726 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt19::C8({
                                                    let (_, h1727) = v9;
                                                    h1727
                                                }, std::rc::Rc::new({
                                                    let (h1728, _) = v10;
                                                    h1728
                                                }), std::rc::Rc::new({
                                                    let (h1729, _) = v12;
                                                    h1729
                                                })), {
                                                    let (_, h1730) = v12;
                                                    h1730
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1731 = v11;
                                        let a1732 = 125u64;
                                        if nat_eq(a1731, a1732) {
                                            Ok(Some((Adt19::C8({
                                                let (_, h1733) = v9;
                                                h1733
                                            }, std::rc::Rc::new({
                                                let (h1734, _) = v10;
                                                h1734
                                            }), std::rc::Rc::new(Adt19::C6)), {
                                                let a1736 = {
                                                    let (_, h1735) = v10;
                                                    h1735
                                                };
                                                let a1737 = 1u64;
                                                nat_add(a1736, a1737)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            } else {
                let a1738 = v5;
                let a1739 = 34u64;
                if nat_eq(a1738, a1739) {
                    let m1742 = f383(v0, {
                        let a1740 = v1;
                        let a1741 = 1u64;
                        nat_add(a1740, a1741)?
                    })?;
                    match m1742 {
                        None => {
                            Ok(None::<(Adt19, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt19::C3({
                                let (_, h1743) = v13;
                                h1743
                            }), {
                                let (h1744, _) = v13;
                                h1744
                            })))
                        }
                    }
                } else {
                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt19::C0(true), {
                            let a1745 = v1;
                            let a1746 = 4u64;
                            nat_add(a1745, a1746)?
                        })))
                    } else {
                        if f37(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt19::C0(false), {
                                let a1747 = v1;
                                let a1748 = 5u64;
                                nat_add(a1747, a1748)?
                            })))
                        } else {
                            let a1749 = v5;
                            let a1750 = 91u64;
                            if nat_eq(a1749, a1750) {
                                let a1753 = f315(v0.clone(), {
                                    let a1751 = v1;
                                    let a1752 = 1u64;
                                    nat_add(a1751, a1752)?
                                })?;
                                let a1754 = 93u64;
                                if nat_eq(a1753, a1754) {
                                    Ok(Some((Adt19::C4(std::rc::Rc::new(Adt19::C6)), {
                                        let a1755 = v1;
                                        let a1756 = 2u64;
                                        nat_add(a1755, a1756)?
                                    })))
                                } else {
                                    let m1759 = f327(v0, {
                                        let a1757 = v1;
                                        let a1758 = 1u64;
                                        nat_add(a1757, a1758)?
                                    }, v4, 1u64)?;
                                    match m1759 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt19::C4(std::rc::Rc::new({
                                                let (h1760, _) = v14;
                                                h1760
                                            })), {
                                                let (_, h1761) = v14;
                                                h1761
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1762 = v5;
                                let a1763 = 123u64;
                                if nat_eq(a1762, a1763) {
                                    let a1766 = f315(v0.clone(), {
                                        let a1764 = v1;
                                        let a1765 = 1u64;
                                        nat_add(a1764, a1765)?
                                    })?;
                                    let a1767 = 125u64;
                                    if nat_eq(a1766, a1767) {
                                        Ok(Some((Adt19::C5(std::rc::Rc::new(Adt19::C6)), {
                                            let a1768 = v1;
                                            let a1769 = 2u64;
                                            nat_add(a1768, a1769)?
                                        })))
                                    } else {
                                        let m1772 = f327(v0, {
                                            let a1770 = v1;
                                            let a1771 = 1u64;
                                            nat_add(a1770, a1771)?
                                        }, v4, 2u64)?;
                                        match m1772 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt19::C5(std::rc::Rc::new({
                                                    let (h1773, _) = v15;
                                                    h1773
                                                })), {
                                                    let (_, h1774) = v15;
                                                    h1774
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1775 = f385(v0, v1)?;
                                    match m1775 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1776) = v16;
                                                h1776
                                            }, {
                                                let (h1777, _) = v16;
                                                h1777
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

pub fn f328(v0: Adt19, v1: Bytes) -> bool {
    let a1779 = {
        let a1778 = f151(v0, v1);
        length_bytes(a1778)
    };
    let a1780 = 0u64;
    !nat_eq(a1779, a1780)
}

pub fn f329(v0: Adt19, v1: Bytes) -> bool {
    let m1781 = f148(v0, v1);
    match m1781 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f330(v0: Bytes, p1848: u64, p1849: u64, p1850: bool, p1851: Bytes) -> R<Bytes> {
    let m1847 = (p1848, (p1849, (p1850, p1851)));
    {
        let mut m1847 = m1847;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1847;
                let m1782 = v2;
                if m1782 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1782 - 1;
                    let m1785 = {
                        let a1783 = v0.clone();
                        let a1784 = v1;
                        index_bytes(a1783, a1784)
                    };
                    match m1785 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1830 = {
                                    let a1806 = {
                                        let a1794 = {
                                            let a1788 = {
                                                let a1786 = v7;
                                                let a1787 = 128u8;
                                                fixed_u8::bit_and(a1786, a1787)
                                            };
                                            let a1789 = 128u8;
                                            if equal(a1788, a1789) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1795 = {
                                            let a1792 = {
                                                let a1790 = v7;
                                                let a1791 = 64u8;
                                                fixed_u8::bit_and(a1790, a1791)
                                            };
                                            let a1793 = 64u8;
                                            if equal(a1792, a1793) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1794, a1795)?
                                    };
                                    let a1807 = {
                                        let a1804 = {
                                            let a1798 = {
                                                let a1796 = v7;
                                                let a1797 = 32u8;
                                                fixed_u8::bit_and(a1796, a1797)
                                            };
                                            let a1799 = 32u8;
                                            if equal(a1798, a1799) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1805 = {
                                            let a1802 = {
                                                let a1800 = v7;
                                                let a1801 = 16u8;
                                                fixed_u8::bit_and(a1800, a1801)
                                            };
                                            let a1803 = 16u8;
                                            if equal(a1802, a1803) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1804, a1805)?
                                    };
                                    nat_add(a1806, a1807)?
                                };
                                let a1831 = {
                                    let a1828 = {
                                        let a1816 = {
                                            let a1810 = {
                                                let a1808 = v7;
                                                let a1809 = 8u8;
                                                fixed_u8::bit_and(a1808, a1809)
                                            };
                                            let a1811 = 8u8;
                                            if equal(a1810, a1811) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1817 = {
                                            let a1814 = {
                                                let a1812 = v7;
                                                let a1813 = 4u8;
                                                fixed_u8::bit_and(a1812, a1813)
                                            };
                                            let a1815 = 4u8;
                                            if equal(a1814, a1815) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1816, a1817)?
                                    };
                                    let a1829 = {
                                        let a1826 = {
                                            let a1820 = {
                                                let a1818 = v7;
                                                let a1819 = 2u8;
                                                fixed_u8::bit_and(a1818, a1819)
                                            };
                                            let a1821 = 2u8;
                                            if equal(a1820, a1821) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1827 = {
                                            let a1824 = {
                                                let a1822 = v7;
                                                let a1823 = 1u8;
                                                fixed_u8::bit_and(a1822, a1823)
                                            };
                                            let a1825 = 1u8;
                                            if equal(a1824, a1825) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1826, a1827)?
                                    };
                                    nat_add(a1828, a1829)?
                                };
                                nat_add(a1830, a1831)?
                            };
                            if f386(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1832 = v1;
                                    let a1833 = 1u64;
                                    nat_add(a1832, a1833)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f387(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1834 = v1;
                                        let a1835 = 1u64;
                                        nat_add(a1834, a1835)?
                                    }, (v5, (false, {
                                        let a1839 = v4;
                                        let a1840 = f32({
                                            let a1836 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1836)
                                        }, {
                                            let a1837 = v8;
                                            let a1838 = 97u64;
                                            nat_sub(a1837, a1838)
                                        }, 1u64);
                                        append_bytes(a1839, a1840)
                                    }))))
                                } else {
                                    if if f387(v8) {
                                        true
                                    } else {
                                        if f388(v8) {
                                            true
                                        } else {
                                            f389(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1841 = v1;
                                            let a1842 = 1u64;
                                            nat_add(a1841, a1842)?
                                        }, (v5, (false, {
                                            let a1843 = v4;
                                            let a1844 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1843, a1844)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1845 = v1;
                                            let a1846 = 1u64;
                                            nat_add(a1845, a1846)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: Adt19, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt19)> = f390(f391(f148(v0.clone(), f8(Str::lit("$defs")))), f391(f148(v0.clone(), f8(Str::lit("definitions")))));
    let m1852 = f392(4096u64, v3.clone(), List::<Bytes>::nil(), f393(v3)?)?;
    match m1852 {
        Ok(v5) => {
            let m1853 = f394(4096u64, f395(f396(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1853 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f216(f187(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f332(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1854 = v0;
    match m1854.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f37(f158(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f397(v5, v1, v2))
            } else {
                List::cons(v4, f332(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f333(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f216(f398(f84(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f334(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1855 = 0u64;
        let a1856 = v1;
        nat_lt(a1855, a1856)
    } else {
        false
    } {
        let a1857 = Bytes::lit(&[0x2d]);
        let a1858 = f127(v1)?;
        Ok(append_bytes(a1857, a1858))
    } else {
        f127(v1)
    }
}

pub fn f335(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f127(v1)?;
    let v4: u64 = {
        let a1859 = v3.clone();
        length_bytes(a1859)
    };
    let v5: Bytes = {
        let a1860 = v2;
        let a1861 = v4;
        if nat_lt(a1860, a1861) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1862 = v4;
                let a1863 = v2;
                nat_sub(a1862, a1863)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1864 = v4;
                let a1865 = v2;
                nat_sub(a1864, a1865)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f399({
                let a1866 = v2;
                let a1867 = v4;
                nat_sub(a1866, a1867)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1868 = f8(Str::lit("-"));
        let a1869 = v5;
        Ok(append_bytes(a1868, a1869))
    } else {
        Ok(v5)
    }
}

pub fn f336(v0: Bytes) -> R<Bytes> {
    let a1875 = {
        let a1873 = Bytes::lit(&[0x22]);
        let a1874 = {
            let (_, h1870) = {
                let (h1871, _) = f306(Fn5::F400(v0.clone()), {
                    let a1872 = v0;
                    length_bytes(a1872)
                }, (0u64, Bytes::lit(&[])))?;
                h1871
            };
            h1870
        };
        append_bytes(a1873, a1874)
    };
    let a1876 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1875, a1876))
}

pub fn f337(v0: List<Bytes>) -> Bytes {
    let a1879 = {
        let a1877 = Bytes::lit(&[0x5b]);
        let a1878 = f122(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1877, a1878)
    };
    let a1880 = Bytes::lit(&[0x5d]);
    append_bytes(a1879, a1880)
}

pub fn f338(v0: Adt19) -> R<List<Bytes>> {
    let m1881 = v0;
    match m1881 {
        Adt19::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C7(r1882, r1883) => {
            let v10 = (*r1882).clone();
            let v11 = (*r1883).clone();
            Ok(List::cons(f246(v10)?, f338(v11)?))
        }
        Adt19::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f339(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f401(Fn10::F402, List::<(Bytes, Bytes)>::nil(), f403(v0)?);
    let v2: List<Bytes> = f404(Fn11::F405, List::<Bytes>::nil(), v1)?;
    let a1886 = {
        let a1884 = Bytes::lit(&[0x7b]);
        let a1885 = f122(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1884, a1885)
    };
    let a1887 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1886, a1887))
}

pub fn f340(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m1888 = v0;
    match m1888 {
        Adt19::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C8(v12, r1889, r1890) => {
            let v13 = (*r1889).clone();
            let v14 = (*r1890).clone();
            Ok(List::cons((v12, f246(v13)?), f340(v14)?))
        }
    }
}

pub fn f341(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1891 = v0;
    match m1891 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f158(v2, v1)
        }
    }
}

pub fn f342(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f216(f406(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f343(v0: Adt16, v1: Option<u64>) -> bool {
    let m1892 = v1;
    match m1892 {
        None => {
            true
        }
        Some(v2) => {
            f265(v2, match v0 {
                Adt16::C0(_, h1893, _, _) => {
                    h1893
                }
            })
        }
    }
}

pub fn f344(v0: Adt12) -> Adt21 {
    Adt21::C1({
        let a1895 = match v0 {
            Adt12::C0(_, _, h1894, _, _) => {
                h1894
            }
        };
        length_list(a1895)
    })
}

pub fn f345(v0: Bytes) -> Option<Adt22> {
    if f37(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt22::C0)
    } else {
        if f37(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt22::C1)
        } else {
            if f37(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt22::C2)
            } else {
                None::<Adt22>
            }
        }
    }
}

pub fn f346(v0: Bytes, p1900: List<Bytes>, p1901: u64) -> R<Option<u64>> {
    let m1899 = (p1900, p1901);
    {
        let mut m1899 = m1899;
        loop {
            let step = {
                let (v1, v2) = m1899;
                let m1896 = v1;
                match m1896.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f37(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1897 = v2;
                                let a1898 = 1u64;
                                nat_add(a1897, a1898)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1899 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: Option<Adt14>) -> Option<u64> {
    let m1902 = v0;
    match m1902 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1903 = v1;
            match m1903 {
                Adt14::C0(_) => {
                    None::<u64>
                }
                Adt14::C1(v2, v3) => {
                    if v2 {
                        None::<u64>
                    } else {
                        Some(v3)
                    }
                }
                Adt14::C2(_) => {
                    None::<u64>
                }
                Adt14::C3(_) => {
                    None::<u64>
                }
                Adt14::C4(_) => {
                    None::<u64>
                }
                Adt14::C5 => {
                    None::<u64>
                }
                Adt14::C6(_, _) => {
                    None::<u64>
                }
                Adt14::C7(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f348(v0: Fn6, p1907: List<Bytes>, p1908: List<(Bytes, ())>) -> List<Bytes> {
    let m1906 = (p1907, p1908);
    {
        let mut m1906 = m1906;
        loop {
            let step = {
                let (v1, v2) = m1906;
                let m1904 = v2;
                match m1904.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1905: Fn6 = v0.clone();
                            c1905.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1909, _) = v1;
        h1909
    }, v0)
}

pub fn f350(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f407(Fn12::F408, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f351(v0: Fn7, p1913: List<List<(Bytes, ())>>, p1914: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1912 = (p1913, p1914);
    {
        let mut m1912 = m1912;
        loop {
            let step = {
                let (v1, v2) = m1912;
                let m1910 = v2;
                match m1910.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1911: Fn7 = v0.clone();
                            c1911.apply(v1, v3)
                        }, v4))
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

pub fn f352(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f353(v0: Fn8, p1919: u64, p1920: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1918 = (p1919, p1920);
    {
        let mut m1918 = m1918;
        loop {
            let step = {
                let (v1, v2) = m1918;
                let m1915 = v1;
                if m1915 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1915 - 1;
                    let m1917 = {
                        let c1916: Fn8 = v0.clone();
                        c1916.apply(v2.clone())?
                    };
                    match m1917 {
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
                Ok(next) => { m1918 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f409(v0)
}

pub fn f355(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f410(Fn13::F411, List::<(Bytes, ())>::nil(), v0)
}

pub fn f356(v0: Fn9, p1924: List<(Bytes, ())>, p1925: List<Bytes>) -> List<(Bytes, ())> {
    let m1923 = (p1924, p1925);
    {
        let mut m1923 = m1923;
        loop {
            let step = {
                let (v1, v2) = m1923;
                let m1921 = v2;
                match m1921.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1922: Fn9 = v0.clone();
                            c1922.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1923 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f358(v0: Bytes) -> Bytes {
    if f135(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1927 = {
                let a1926 = v0;
                length_bytes(a1926)
            };
            let a1928 = 1u64;
            nat_sub(a1927, a1928)
        })
    } else {
        v0
    }
}

pub fn f359(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f84(f71(v0.clone(), v1.clone()))?;
    let m1929 = f154(v2.clone())?;
    match m1929 {
        None => {
            f413(v0, v1, v2)
        }
        Some(v3) => {
            f412(v0, v1, v3)
        }
    }
}

pub fn f360(v0: List<Bytes>) -> List<Bytes> {
    let m1930 = v0;
    match m1930.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1932 = {
                    let a1931 = v1.clone();
                    length_bytes(a1931)
                };
                let a1933 = 0u64;
                if nat_eq(a1932, a1933) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f360(v2))
        }
    }
}

pub fn f361(v0: List<Bytes>) -> Bytes {
    let m1934 = v0;
    match m1934.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f362(v0: (Bytes, Bytes), p1937: List<(Bytes, Bytes)>) -> bool {
    let m1936 = p1937;
    {
        let mut m1936 = m1936;
        loop {
            let step = {
                let v1 = m1936;
                let m1935 = v1;
                match m1935.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f414(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1938 = v1;
    match m1938.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37({
                let (h1939, _) = v2;
                h1939
            }, v0.clone()) {
                List::cons({
                    let (_, h1940) = v2;
                    h1940
                }, f363(v0, v3))
            } else {
                f363(v0, v3)
            }
        }
    }
}

pub fn f364(v0: u32) -> Bytes {
    let a1943 = {
        let a1941 = v0;
        let a1942 = 8u32;
        fixed_u32::bit_and(a1941, a1942)
    };
    let a1944 = 0u32;
    if equal(a1943, a1944) {
        let a1947 = {
            let a1945 = v0;
            let a1946 = 4u32;
            fixed_u32::bit_and(a1945, a1946)
        };
        let a1948 = 0u32;
        if equal(a1947, a1948) {
            let a1951 = {
                let a1949 = v0;
                let a1950 = 2u32;
                fixed_u32::bit_and(a1949, a1950)
            };
            let a1952 = 0u32;
            if equal(a1951, a1952) {
                let a1955 = {
                    let a1953 = v0;
                    let a1954 = 1u32;
                    fixed_u32::bit_and(a1953, a1954)
                };
                let a1956 = 0u32;
                if equal(a1955, a1956) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1959 = {
                    let a1957 = v0;
                    let a1958 = 1u32;
                    fixed_u32::bit_and(a1957, a1958)
                };
                let a1960 = 0u32;
                if equal(a1959, a1960) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1963 = {
                let a1961 = v0;
                let a1962 = 2u32;
                fixed_u32::bit_and(a1961, a1962)
            };
            let a1964 = 0u32;
            if equal(a1963, a1964) {
                let a1967 = {
                    let a1965 = v0;
                    let a1966 = 1u32;
                    fixed_u32::bit_and(a1965, a1966)
                };
                let a1968 = 0u32;
                if equal(a1967, a1968) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1971 = {
                    let a1969 = v0;
                    let a1970 = 1u32;
                    fixed_u32::bit_and(a1969, a1970)
                };
                let a1972 = 0u32;
                if equal(a1971, a1972) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1975 = {
            let a1973 = v0;
            let a1974 = 4u32;
            fixed_u32::bit_and(a1973, a1974)
        };
        let a1976 = 0u32;
        if equal(a1975, a1976) {
            let a1979 = {
                let a1977 = v0;
                let a1978 = 2u32;
                fixed_u32::bit_and(a1977, a1978)
            };
            let a1980 = 0u32;
            if equal(a1979, a1980) {
                let a1983 = {
                    let a1981 = v0;
                    let a1982 = 1u32;
                    fixed_u32::bit_and(a1981, a1982)
                };
                let a1984 = 0u32;
                if equal(a1983, a1984) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1987 = {
                    let a1985 = v0;
                    let a1986 = 1u32;
                    fixed_u32::bit_and(a1985, a1986)
                };
                let a1988 = 0u32;
                if equal(a1987, a1988) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1991 = {
                let a1989 = v0;
                let a1990 = 2u32;
                fixed_u32::bit_and(a1989, a1990)
            };
            let a1992 = 0u32;
            if equal(a1991, a1992) {
                let a1995 = {
                    let a1993 = v0;
                    let a1994 = 1u32;
                    fixed_u32::bit_and(a1993, a1994)
                };
                let a1996 = 0u32;
                if equal(a1995, a1996) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1999 = {
                    let a1997 = v0;
                    let a1998 = 1u32;
                    fixed_u32::bit_and(a1997, a1998)
                };
                let a2000 = 0u32;
                if equal(a1999, a2000) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f365(v0: u32, v1: u32) -> u32 {
    let m2003 = {
        let a2001 = v0;
        let a2002 = v1;
        fixed_u32::shift_right(a2001, a2002)
    };
    match m2003 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f366(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f415(Fn14::F416, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2004, _, _, _, _, _, _, _) => {
            h2004
        }
    }, match v0.clone() {
        Adt24::C0(_, h2005, _, _, _, _, _, _) => {
            h2005
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2006, _, _, _, _, _) => {
            h2006
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2007, _, _, _, _) => {
            h2007
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2008, _, _, _) => {
            h2008
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2009, _, _) => {
            h2009
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2010, _) => {
            h2010
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2011) => {
            h2011
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f417(match v0.clone() {
        Adt24::C0(h2012, _, _, _, _, _, _, _) => {
            h2012
        }
    }, match v2.clone() {
        Adt27::C0(h2013, _, _, _, _, _, _, _, _, _) => {
            h2013
        }
    }), f417(match v0.clone() {
        Adt24::C0(_, h2014, _, _, _, _, _, _) => {
            h2014
        }
    }, match v2.clone() {
        Adt27::C0(_, h2015, _, _, _, _, _, _, _, _) => {
            h2015
        }
    }), f417(match v0.clone() {
        Adt24::C0(_, _, h2016, _, _, _, _, _) => {
            h2016
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2017, _, _, _, _, _, _, _) => {
            h2017
        }
    }), f417(match v0.clone() {
        Adt24::C0(_, _, _, h2018, _, _, _, _) => {
            h2018
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2019, _, _, _, _, _, _) => {
            h2019
        }
    }), f417(match v0.clone() {
        Adt24::C0(_, _, _, _, h2020, _, _, _) => {
            h2020
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2021, _, _, _, _, _) => {
            h2021
        }
    }), f417(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2022, _, _) => {
            h2022
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2023, _, _, _, _) => {
            h2023
        }
    }), f417(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2024, _) => {
            h2024
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2025, _, _, _) => {
            h2025
        }
    }), f417(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2026) => {
            h2026
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2027, _, _) => {
            h2027
        }
    })))
}

pub fn f367(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f418(v0.clone(), v1, v2, v3)?, f418(v0.clone(), v1, v2, {
        let a2028 = v3;
        let a2029 = 4u64;
        nat_add(a2028, a2029)?
    })?, f418(v0.clone(), v1, v2, {
        let a2030 = v3;
        let a2031 = 8u64;
        nat_add(a2030, a2031)?
    })?, f418(v0.clone(), v1, v2, {
        let a2032 = v3;
        let a2033 = 12u64;
        nat_add(a2032, a2033)?
    })?, f418(v0.clone(), v1, v2, {
        let a2034 = v3;
        let a2035 = 16u64;
        nat_add(a2034, a2035)?
    })?, f418(v0.clone(), v1, v2, {
        let a2036 = v3;
        let a2037 = 20u64;
        nat_add(a2036, a2037)?
    })?, f418(v0.clone(), v1, v2, {
        let a2038 = v3;
        let a2039 = 24u64;
        nat_add(a2038, a2039)?
    })?, f418(v0.clone(), v1, v2, {
        let a2040 = v3;
        let a2041 = 28u64;
        nat_add(a2040, a2041)?
    })?, f418(v0.clone(), v1, v2, {
        let a2042 = v3;
        let a2043 = 32u64;
        nat_add(a2042, a2043)?
    })?, f418(v0.clone(), v1, v2, {
        let a2044 = v3;
        let a2045 = 36u64;
        nat_add(a2044, a2045)?
    })?, f418(v0.clone(), v1, v2, {
        let a2046 = v3;
        let a2047 = 40u64;
        nat_add(a2046, a2047)?
    })?, f418(v0.clone(), v1, v2, {
        let a2048 = v3;
        let a2049 = 44u64;
        nat_add(a2048, a2049)?
    })?, f418(v0.clone(), v1, v2, {
        let a2050 = v3;
        let a2051 = 48u64;
        nat_add(a2050, a2051)?
    })?, f418(v0.clone(), v1, v2, {
        let a2052 = v3;
        let a2053 = 52u64;
        nat_add(a2052, a2053)?
    })?, f418(v0.clone(), v1, v2, {
        let a2054 = v3;
        let a2055 = 56u64;
        nat_add(a2054, a2055)?
    })?, f418(v0, v1, v2, {
        let a2056 = v3;
        let a2057 = 60u64;
        nat_add(a2056, a2057)?
    })?))
}

pub fn f368(p2060: List<Bytes>, v1: Bytes) -> bool {
    let m2059 = p2060;
    {
        let mut m2059 = m2059;
        loop {
            let step = {
                let v0 = m2059;
                let m2058 = v0;
                match m2058.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f206(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2059 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: Bytes, v1: u64) -> Option<u64> {
    let a2061 = 0u64;
    let a2062 = v1;
    if nat_lt(a2061, a2062) {
        if f419(v0, {
            let a2063 = v1;
            let a2064 = 1u64;
            nat_sub(a2063, a2064)
        }) {
            Some({
                let a2065 = v1;
                let a2066 = 1u64;
                nat_sub(a2065, a2066)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f370(v0: Adt14) -> R<List<Bytes>> {
    let m2067 = v0;
    match m2067 {
        Adt14::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C6(r2068, r2069) => {
            let v7 = (*r2068).clone();
            let v8 = (*r2069).clone();
            Ok(List::cons(f305(v7)?, f370(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f371(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m2070 = v0;
    match m2070 {
        Adt14::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C7(v9, r2071, r2072) => {
            let v10 = (*r2071).clone();
            let v11 = (*r2072).clone();
            Ok(List::cons((v9, f305(v10)?), f371(v11)?))
        }
    }
}

pub fn f372(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2073, _) = v1;
        h2073
    };
    let a2075 = {
        let a2074 = v0.clone();
        length_bytes(a2074)
    };
    let a2076 = v2;
    if nat_le(a2075, a2076) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2077 = f228(v0.clone(), v2)?;
        let a2078 = 13u64;
        if nat_eq(a2077, a2078) {
            Ok(Some(({
                let a2081 = f228(v0, {
                    let a2079 = v2;
                    let a2080 = 1u64;
                    nat_add(a2079, a2080)?
                })?;
                let a2082 = 10u64;
                if nat_eq(a2081, a2082) {
                    let a2083 = v2;
                    let a2084 = 2u64;
                    nat_add(a2083, a2084)?
                } else {
                    let a2085 = v2;
                    let a2086 = 1u64;
                    nat_add(a2085, a2086)?
                }
            }, {
                let a2088 = {
                    let (_, h2087) = v1;
                    h2087
                };
                let a2089 = Bytes::lit(&[0x0a]);
                append_bytes(a2088, a2089)
            })))
        } else {
            let v4: u64 = {
                let m2090 = f310(v0.clone(), Bytes::lit(&[0x0d]), v2)?;
                match m2090 {
                    None => {
                        let a2091 = v0.clone();
                        length_bytes(a2091)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2095 = {
                    let (_, h2092) = v1;
                    h2092
                };
                let a2096 = f32(v0, v2, {
                    let a2093 = v4;
                    let a2094 = v2;
                    nat_sub(a2093, a2094)
                });
                append_bytes(a2095, a2096)
            })))
        }
    }
}

pub fn f373(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2097 = f420(v0, v1)?;
    match m2097 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f421({
                let (h2098, _) = v2;
                h2098
            }) {
                Ok(Some({
                    let a2100 = v1;
                    let a2101 = {
                        let (_, h2099) = v2;
                        h2099
                    };
                    nat_add(a2100, a2101)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f374(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f422(v1, v0, v2)
}

pub fn f375(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f423(v0, v1)
}

pub fn f376(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f424(v0, v1, v2)
}

pub fn f377(v0: Bytes, p2104: List<Bytes>) -> bool {
    let m2103 = p2104;
    {
        let mut m2103 = m2103;
        loop {
            let step = {
                let v1 = m2103;
                let m2102 = v1;
                match m2102.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f37(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f227(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2107 = f228(v0.clone(), {
                let a2105 = v1;
                let a2106 = 2u64;
                nat_add(a2105, a2106)?
            })?;
            let a2108 = 120u64;
            nat_eq(a2107, a2108)
        };
        let v3: u64 = if v2 {
            let a2109 = v1;
            let a2110 = 3u64;
            nat_add(a2109, a2110)?
        } else {
            let a2111 = v1;
            let a2112 = 2u64;
            nat_add(a2111, a2112)?
        };
        let v4: (u64, u64) = f425(v0.clone(), v2, v3)?;
        let a2114 = {
            let (h2113, _) = v4;
            h2113
        };
        let a2115 = v3;
        if if nat_eq(a2114, a2115) {
            true
        } else {
            let a2117 = f228(v0, {
                let (h2116, _) = v4;
                h2116
            })?;
            let a2118 = 59u64;
            !nat_eq(a2117, a2118)
        } {
            Ok(Err::<(Bytes, u64), Bytes>(f224(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f421({
                let (_, h2119) = v4;
                h2119
            }) {
                Ok(Ok::<(Bytes, u64), Bytes>((f426({
                    let (_, h2120) = v4;
                    h2120
                })?, {
                    let a2122 = {
                        let (h2121, _) = v4;
                        h2121
                    };
                    let a2123 = 1u64;
                    nat_add(a2122, a2123)?
                })))
            } else {
                Ok(Err::<(Bytes, u64), Bytes>(f224(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f227(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3c]), {
                let a2124 = v1;
                let a2125 = 4u64;
                nat_add(a2124, a2125)?
            })))
        } else {
            if f227(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3e]), {
                    let a2126 = v1;
                    let a2127 = 4u64;
                    nat_add(a2126, a2127)?
                })))
            } else {
                if f227(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x26]), {
                        let a2128 = v1;
                        let a2129 = 5u64;
                        nat_add(a2128, a2129)?
                    })))
                } else {
                    if f227(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x27]), {
                            let a2130 = v1;
                            let a2131 = 6u64;
                            nat_add(a2130, a2131)?
                        })))
                    } else {
                        if f227(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x22]), {
                                let a2132 = v1;
                                let a2133 = 6u64;
                                nat_add(a2132, a2133)?
                            })))
                        } else {
                            Ok(Err::<(Bytes, u64), Bytes>(f224(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f379(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2134, _) = f256(Fn3::F427(v1, v2, v0.clone()), {
        let a2136 = {
            let a2135 = v0;
            length_bytes(a2135)
        };
        let a2137 = 1u64;
        nat_add(a2136, a2137)?
    }, v3)?;
    Ok(h2134)
}

pub fn f380(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2138, _) = v2;
        h2138
    };
    let a2139 = v3;
    let a2140 = 0u64;
    if nat_eq(a2139, a2140) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2141 = v3;
            let a2142 = v1;
            let a2143 = 0u64;
            nat_quot(a2141, a2142, a2143)
        }, {
            let a2148 = f319(v0, {
                let a2144 = v3;
                let a2145 = v1;
                let a2146 = 0u64;
                nat_rem(a2144, a2145, a2146)
            });
            let a2149 = {
                let (_, h2147) = v2;
                h2147
            };
            append_bytes(a2148, a2149)
        }))
    }
}

pub fn f381(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a2150 = v2;
    let a2151 = v0;
    if nat_lt(a2150, a2151) {
        if f419(v1, v2) {
            Ok(Some({
                let a2152 = v2;
                let a2153 = 1u64;
                nat_add(a2152, a2153)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f382(v0: Bytes, p2233: u64, p2234: u64, p2235: bool, p2236: bool, p2237: Bytes) -> R<Bytes> {
    let m2232 = (p2233, (p2234, (p2235, (p2236, p2237))));
    {
        let mut m2232 = m2232;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m2232;
                let m2154 = v2;
                if m2154 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m2154 - 1;
                    let m2157 = {
                        let a2155 = v0.clone();
                        let a2156 = v1;
                        index_bytes(a2155, a2156)
                    };
                    match m2157 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a2202 = {
                                    let a2178 = {
                                        let a2166 = {
                                            let a2160 = {
                                                let a2158 = v8;
                                                let a2159 = 128u8;
                                                fixed_u8::bit_and(a2158, a2159)
                                            };
                                            let a2161 = 128u8;
                                            if equal(a2160, a2161) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2167 = {
                                            let a2164 = {
                                                let a2162 = v8;
                                                let a2163 = 64u8;
                                                fixed_u8::bit_and(a2162, a2163)
                                            };
                                            let a2165 = 64u8;
                                            if equal(a2164, a2165) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2166, a2167)?
                                    };
                                    let a2179 = {
                                        let a2176 = {
                                            let a2170 = {
                                                let a2168 = v8;
                                                let a2169 = 32u8;
                                                fixed_u8::bit_and(a2168, a2169)
                                            };
                                            let a2171 = 32u8;
                                            if equal(a2170, a2171) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2177 = {
                                            let a2174 = {
                                                let a2172 = v8;
                                                let a2173 = 16u8;
                                                fixed_u8::bit_and(a2172, a2173)
                                            };
                                            let a2175 = 16u8;
                                            if equal(a2174, a2175) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2176, a2177)?
                                    };
                                    nat_add(a2178, a2179)?
                                };
                                let a2203 = {
                                    let a2200 = {
                                        let a2188 = {
                                            let a2182 = {
                                                let a2180 = v8;
                                                let a2181 = 8u8;
                                                fixed_u8::bit_and(a2180, a2181)
                                            };
                                            let a2183 = 8u8;
                                            if equal(a2182, a2183) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2189 = {
                                            let a2186 = {
                                                let a2184 = v8;
                                                let a2185 = 4u8;
                                                fixed_u8::bit_and(a2184, a2185)
                                            };
                                            let a2187 = 4u8;
                                            if equal(a2186, a2187) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2188, a2189)?
                                    };
                                    let a2201 = {
                                        let a2198 = {
                                            let a2192 = {
                                                let a2190 = v8;
                                                let a2191 = 2u8;
                                                fixed_u8::bit_and(a2190, a2191)
                                            };
                                            let a2193 = 2u8;
                                            if equal(a2192, a2193) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2199 = {
                                            let a2196 = {
                                                let a2194 = v8;
                                                let a2195 = 1u8;
                                                fixed_u8::bit_and(a2194, a2195)
                                            };
                                            let a2197 = 1u8;
                                            if equal(a2196, a2197) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2198, a2199)?
                                    };
                                    nat_add(a2200, a2201)?
                                };
                                nat_add(a2202, a2203)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a2204 = v1;
                                    let a2205 = 1u64;
                                    nat_add(a2204, a2205)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a2206 = v9;
                                    let a2207 = 34u64;
                                    !nat_eq(a2206, a2207)
                                }, (if v4 {
                                    false
                                } else {
                                    let a2208 = v9;
                                    let a2209 = 92u64;
                                    nat_eq(a2208, a2209)
                                }, {
                                    let a2210 = v5;
                                    let a2211 = v10;
                                    append_bytes(a2210, a2211)
                                })))))
                            } else {
                                let a2212 = v9;
                                let a2213 = 32u64;
                                if if if if nat_eq(a2212, a2213) {
                                    true
                                } else {
                                    let a2214 = v9;
                                    let a2215 = 9u64;
                                    nat_eq(a2214, a2215)
                                } {
                                    true
                                } else {
                                    let a2216 = v9;
                                    let a2217 = 10u64;
                                    nat_eq(a2216, a2217)
                                } {
                                    true
                                } else {
                                    let a2218 = v9;
                                    let a2219 = 13u64;
                                    nat_eq(a2218, a2219)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a2220 = v1;
                                        let a2221 = 1u64;
                                        nat_add(a2220, a2221)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2222 = v1;
                                            let a2223 = 4u64;
                                            nat_add(a2222, a2223)?
                                        }, (v6, (false, (false, {
                                            let a2224 = v5;
                                            let a2225 = f8(Str::lit("[]"));
                                            append_bytes(a2224, a2225)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2226 = v1;
                                            let a2227 = 1u64;
                                            nat_add(a2226, a2227)?
                                        }, (v6, ({
                                            let a2228 = v9;
                                            let a2229 = 34u64;
                                            nat_eq(a2228, a2229)
                                        }, (false, {
                                            let a2230 = v5;
                                            let a2231 = v10;
                                            append_bytes(a2230, a2231)
                                        })))))
                                    }
                                }
                            }
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

pub fn f383(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f428(Fn15::F429(v0.clone()), {
        let a2239 = {
            let a2238 = v0;
            length_bytes(a2238)
        };
        let a2240 = 1u64;
        nat_add(a2239, a2240)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a2244 = {
        let (_, h2241) = {
            let (_, h2242) = {
                let (h2243, _) = v2.clone();
                h2243
            };
            h2242
        };
        h2241
    };
    let a2245 = 1u64;
    if nat_eq(a2244, a2245) {
        Ok(Some(({
            let (h2246, _) = {
                let (h2247, _) = v2.clone();
                h2247
            };
            h2246
        }, {
            let (h2248, _) = {
                let (_, h2249) = {
                    let (h2250, _) = v2;
                    h2250
                };
                h2249
            };
            h2248
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f384(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2251 = f315(v0.clone(), v1)?;
        let a2252 = 45u64;
        nat_eq(a2251, a2252)
    };
    let v3: u64 = if v2 {
        let a2253 = v1;
        let a2254 = 1u64;
        nat_add(a2253, a2254)?
    } else {
        v1
    };
    let v4: u64 = f430(v0.clone(), v3)?;
    let v5: u64 = {
        let a2255 = v4;
        let a2256 = v3;
        nat_sub(a2255, a2256)
    };
    let v6: bool = {
        let a2257 = f315(v0.clone(), v4)?;
        let a2258 = 46u64;
        nat_eq(a2257, a2258)
    };
    let v7: u64 = if v6 {
        let a2259 = v4;
        let a2260 = 1u64;
        nat_add(a2259, a2260)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f430(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f315(v0.clone(), v8)?;
    let v10: bool = {
        let a2261 = v9;
        let a2262 = 101u64;
        if nat_eq(a2261, a2262) {
            true
        } else {
            let a2263 = v9;
            let a2264 = 69u64;
            nat_eq(a2263, a2264)
        }
    };
    let v11: u64 = f315(v0.clone(), {
        let a2265 = v8;
        let a2266 = 1u64;
        nat_add(a2265, a2266)?
    })?;
    let v12: bool = if v10 {
        let a2267 = v11;
        let a2268 = 43u64;
        if nat_eq(a2267, a2268) {
            true
        } else {
            let a2269 = v11;
            let a2270 = 45u64;
            nat_eq(a2269, a2270)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2271 = v8;
            let a2272 = 2u64;
            nat_add(a2271, a2272)?
        } else {
            let a2273 = v8;
            let a2274 = 1u64;
            nat_add(a2273, a2274)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f430(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2275 = v14;
        let a2276 = v13;
        nat_sub(a2275, a2276)
    });
    let v16: u64 = f431(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2278 = {
            let a2277 = v15;
            length_bytes(a2277)
        };
        let a2279 = v16;
        nat_sub(a2278, a2279)
    });
    let a2280 = v5;
    let a2281 = 0u64;
    if if nat_eq(a2280, a2281) {
        true
    } else {
        let a2282 = 1u64;
        let a2283 = v5;
        if if nat_lt(a2282, a2283) {
            let a2284 = f315(v0.clone(), v3)?;
            let a2285 = 48u64;
            nat_eq(a2284, a2285)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2286 = v8;
                let a2287 = v7;
                nat_eq(a2286, a2287)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2288 = v14;
                    let a2289 = v13;
                    nat_eq(a2288, a2289)
                } else {
                    false
                } {
                    true
                } else {
                    let a2291 = 18u64;
                    let a2292 = {
                        let a2290 = v17.clone();
                        length_bytes(a2290)
                    };
                    nat_lt(a2291, a2292)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2296 = {
            let a2294 = {
                let a2293 = v17.clone();
                length_bytes(a2293)
            };
            let a2295 = 0u64;
            if nat_eq(a2294, a2295) {
                Some(0u64)
            } else {
                f432(v17)?
            }
        };
        match m2296 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2301 = f433(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2297 = v8;
                    let a2298 = v7;
                    nat_sub(a2297, a2298)
                }), if v12 {
                    let a2299 = v11;
                    let a2300 = 45u64;
                    nat_eq(a2299, a2300)
                } else {
                    false
                }, v18)?;
                match m2301 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2303 = 19u64;
                        let a2304 = {
                            let a2302 = v19.clone();
                            length_bytes(a2302)
                        };
                        if nat_lt(a2303, a2304) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2305 = f432(v19)?;
                            match m2305 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2306 = 0u64;
                                        let a2307 = v20;
                                        nat_lt(a2306, a2307)
                                    } else {
                                        false
                                    };
                                    if f434(v21, v20) {
                                        Ok(Some((v14, Adt14::C1(v21, v20))))
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

pub fn f385(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2308 = f315(v0.clone(), v1)?;
        let a2309 = 45u64;
        nat_eq(a2308, a2309)
    };
    let v3: u64 = if v2 {
        let a2310 = v1;
        let a2311 = 1u64;
        nat_add(a2310, a2311)?
    } else {
        v1
    };
    let v4: u64 = f430(v0.clone(), v3)?;
    let v5: u64 = {
        let a2312 = v4;
        let a2313 = v3;
        nat_sub(a2312, a2313)
    };
    let v6: bool = {
        let a2314 = f315(v0.clone(), v4)?;
        let a2315 = 46u64;
        nat_eq(a2314, a2315)
    };
    let v7: u64 = if v6 {
        let a2316 = v4;
        let a2317 = 1u64;
        nat_add(a2316, a2317)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f430(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f315(v0.clone(), v8)?;
    let v10: bool = {
        let a2318 = v9;
        let a2319 = 101u64;
        if nat_eq(a2318, a2319) {
            true
        } else {
            let a2320 = v9;
            let a2321 = 69u64;
            nat_eq(a2320, a2321)
        }
    };
    let v11: u64 = f315(v0.clone(), {
        let a2322 = v8;
        let a2323 = 1u64;
        nat_add(a2322, a2323)?
    })?;
    let v12: bool = if v10 {
        let a2324 = v11;
        let a2325 = 43u64;
        if nat_eq(a2324, a2325) {
            true
        } else {
            let a2326 = v11;
            let a2327 = 45u64;
            nat_eq(a2326, a2327)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2328 = v8;
            let a2329 = 2u64;
            nat_add(a2328, a2329)?
        } else {
            let a2330 = v8;
            let a2331 = 1u64;
            nat_add(a2330, a2331)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f430(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2332 = v14;
        let a2333 = v13;
        nat_sub(a2332, a2333)
    });
    let v16: u64 = f431(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2335 = {
            let a2334 = v15;
            length_bytes(a2334)
        };
        let a2336 = v16;
        nat_sub(a2335, a2336)
    });
    let a2337 = v5;
    let a2338 = 0u64;
    if if nat_eq(a2337, a2338) {
        true
    } else {
        let a2339 = 1u64;
        let a2340 = v5;
        if if nat_lt(a2339, a2340) {
            let a2341 = f315(v0.clone(), v3)?;
            let a2342 = 48u64;
            nat_eq(a2341, a2342)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2343 = v8;
                let a2344 = v7;
                nat_eq(a2343, a2344)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2345 = v14;
                    let a2346 = v13;
                    nat_eq(a2345, a2346)
                } else {
                    false
                } {
                    true
                } else {
                    let a2348 = 18u64;
                    let a2349 = {
                        let a2347 = v17.clone();
                        length_bytes(a2347)
                    };
                    nat_lt(a2348, a2349)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2353 = {
            let a2351 = {
                let a2350 = v17.clone();
                length_bytes(a2350)
            };
            let a2352 = 0u64;
            if nat_eq(a2351, a2352) {
                Some(0u64)
            } else {
                f432(v17)?
            }
        };
        match m2353 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2358 = f435(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2354 = v8;
                    let a2355 = v7;
                    nat_sub(a2354, a2355)
                }), if v12 {
                    let a2356 = v11;
                    let a2357 = 45u64;
                    nat_eq(a2356, a2357)
                } else {
                    false
                }, v18)?;
                match m2358 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2361 = 19u64;
                        let a2362 = {
                            let a2360 = {
                                let (h2359, _) = v19.clone();
                                h2359
                            };
                            length_bytes(a2360)
                        };
                        if nat_lt(a2361, a2362) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2364 = f432({
                                let (h2363, _) = v19.clone();
                                h2363
                            })?;
                            match m2364 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2365 = 0u64;
                                        let a2366 = v20;
                                        nat_lt(a2365, a2366)
                                    } else {
                                        false
                                    };
                                    if f434(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2368 = {
                                                let (_, h2367) = v19.clone();
                                                h2367
                                            };
                                            let a2369 = 0u64;
                                            if nat_eq(a2368, a2369) {
                                                Adt19::C1(v21, v20)
                                            } else {
                                                Adt19::C2(v21, v20, {
                                                    let (_, h2370) = v19;
                                                    h2370
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

pub fn f386(v0: u64) -> bool {
    let a2371 = v0;
    let a2372 = 95u64;
    if nat_eq(a2371, a2372) {
        true
    } else {
        let a2373 = v0;
        let a2374 = 45u64;
        if nat_eq(a2373, a2374) {
            true
        } else {
            let a2375 = v0;
            let a2376 = 32u64;
            if nat_eq(a2375, a2376) {
                true
            } else {
                let a2377 = v0;
                let a2378 = 46u64;
                if nat_eq(a2377, a2378) {
                    true
                } else {
                    let a2379 = v0;
                    let a2380 = 47u64;
                    nat_eq(a2379, a2380)
                }
            }
        }
    }
}

pub fn f387(v0: u64) -> bool {
    let a2381 = 97u64;
    let a2382 = v0;
    if nat_le(a2381, a2382) {
        let a2383 = v0;
        let a2384 = 122u64;
        nat_le(a2383, a2384)
    } else {
        false
    }
}

pub fn f388(v0: u64) -> bool {
    let a2385 = 65u64;
    let a2386 = v0;
    if nat_le(a2385, a2386) {
        let a2387 = v0;
        let a2388 = 90u64;
        nat_le(a2387, a2388)
    } else {
        false
    }
}

pub fn f389(v0: u64) -> bool {
    let a2389 = 48u64;
    let a2390 = v0;
    if nat_le(a2389, a2390) {
        let a2391 = v0;
        let a2392 = 57u64;
        nat_le(a2391, a2392)
    } else {
        false
    }
}

pub fn f390(v0: List<(Bytes, Adt19)>, v1: List<(Bytes, Adt19)>) -> List<(Bytes, Adt19)> {
    let m2393 = v0;
    match m2393.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f390(v3, v1))
        }
    }
}

pub fn f391(v0: Option<Adt19>) -> List<(Bytes, Adt19)> {
    let m2394 = v0;
    match m2394 {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some(v1) => {
            f436(v1)
        }
    }
}

pub fn f392(v0: u64, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt19)>, Bytes>> {
    let m2395 = v0;
    if m2395 == 0 {
        let m2396 = v1;
        match m2396.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2395 - 1;
        let m2397 = v1.clone();
        match m2397.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                let m2398 = f437(v1.clone(), v2.clone(), v3.clone())?;
                match m2398 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f216(f393(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2401 = f392(v6, f438(v1, {
                            let (h2399, _) = v9.clone();
                            h2399
                        }), List::cons(f243({
                            let (h2400, _) = v9.clone();
                            h2400
                        })?, v2), v3)?;
                        match m2401 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f393(v0: List<(Bytes, Adt19)>) -> R<List<Bytes>> {
    let m2402 = v0;
    match m2402.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f243({
                let (h2403, _) = v1;
                h2403
            })?, f393(v2)?))
        }
    }
}

pub fn f394(p2411: u64, p2412: List<Adt25>, p2413: List<Bytes>, p2414: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2410 = (p2411, (p2412, (p2413, p2414)));
    {
        let mut m2410 = m2410;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2410;
                let m2404 = v0;
                if m2404 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2404 - 1;
                    let m2405 = v1;
                    match m2405.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2406 = v5;
                            match m2406 {
                                Adt25::C0(v8, v9) => {
                                    if f37(v8.clone(), f439()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f87(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2407 = f441(v8.clone(), v9)?;
                                            match m2407 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f395({
                                                        let (_, h2408) = v11;
                                                        h2408
                                                    }, List::cons(Adt25::C1({
                                                        let (h2409, _) = v11;
                                                        h2409
                                                    }), v6)), (List::cons(v8, v2), v3))))
                                                }
                                                Err(v10) => {
                                                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt25::C1(v7) => {
                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                                Adt25::C2 => {
                                    if f87(f439(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f439(), v2), List::cons(f440(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2415 = v0;
    match m2415.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f395(v3, v1))
        }
    }
}

pub fn f396(v0: List<(Bytes, Adt19)>) -> R<List<Adt25>> {
    let m2416 = v0;
    match m2416.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f243({
                let (h2417, _) = v1;
                h2417
            })?, {
                let (_, h2418) = v1;
                h2418
            }), f396(v2)?))
        }
    }
}

pub fn f397(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2419 = v0;
    match m2419.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f37(f158(v3.clone(), v1.clone()), v2.clone()) {
                f397(v4, v1, v2)
            } else {
                List::cons(v3, f397(v4, v1, v2))
            }
        }
    }
}

pub fn f398(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2420 = v0;
    match m2420.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f206(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f216(f406(f79(List::cons(v1, f442(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f398(v3, v1)?))
            }
        }
    }
}

pub fn f399(v0: u64) -> Bytes {
    let m2421 = v0;
    if m2421 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2421 - 1;
        let a2422 = f8(Str::lit("0"));
        let a2423 = f399(v1);
        append_bytes(a2422, a2423)
    }
}

pub fn f400(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f443(v0, v1)
}

pub fn f401(v0: Fn10, p2427: List<(Bytes, Bytes)>, p2428: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2426 = (p2427, p2428);
    {
        let mut m2426 = m2426;
        loop {
            let step = {
                let (v1, v2) = m2426;
                let m2424 = v2;
                match m2424.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2425: Fn10 = v0.clone();
                            c2425.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f444(v0, v1)
}

pub fn f403(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f445(v0)
}

pub fn f404(v0: Fn11, p2432: List<Bytes>, p2433: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2431 = (p2432, p2433);
    {
        let mut m2431 = m2431;
        loop {
            let step = {
                let (v1, v2) = m2431;
                let m2429 = v2;
                match m2429.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2430: Fn11 = v0.clone();
                            c2430.apply(v1, v3)?
                        }, v4))
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

pub fn f405(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f446(v1)?, v0))
}

pub fn f406(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2434 = v0;
    match m2434.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f211(v1)?, f406(v2)?))
        }
    }
}

pub fn f407(v0: Fn12, p2438: List<List<(Bytes, ())>>, p2439: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2437 = (p2438, p2439);
    {
        let mut m2437 = m2437;
        loop {
            let step = {
                let (v1, v2) = m2437;
                let m2435 = v2;
                match m2435.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2436: Fn12 = v0.clone();
                            c2436.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f409(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2441 = {
        let a2440 = v0.clone();
        length_list(a2440)
    };
    let a2442 = 1u64;
    if nat_le(a2441, a2442) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f447(v0)?))
    }
}

pub fn f410(v0: Fn13, p2446: List<(Bytes, ())>, p2447: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2445 = (p2446, p2447);
    {
        let mut m2445 = m2445;
        loop {
            let step = {
                let (v1, v2) = m2445;
                let m2443 = v2;
                match m2443.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2444: Fn13 = v0.clone();
                            c2444.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f412(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f73(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f448(v3.clone(), f73(f74(v0, f90())?, f8(Str::lit("at"))));
    let a2449 = {
        let a2448 = v4.clone();
        length_list(a2448)
    };
    let a2450 = 0u64;
    if nat_eq(a2449, a2450) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2453 = {
            let a2451 = v4;
            length_list(a2451)
        };
        let a2454 = {
            let a2452 = v3;
            length_list(a2452)
        };
        if nat_eq(a2453, a2454) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f449(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f413(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f238(f141(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f238(f141(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f65(f73(v3.clone(), f8(Str::lit("id"))), f73(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f73(f74(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f448(v5.clone(), f65(v6.clone(), f73(f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2456 = {
        let a2455 = v7.clone();
        length_list(a2455)
    };
    let a2457 = 0u64;
    if nat_eq(a2456, a2457) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f76({
            let a2460 = {
                let a2458 = v7;
                length_list(a2458)
            };
            let a2461 = {
                let a2459 = v5;
                length_list(a2459)
            };
            nat_eq(a2460, a2461)
        }) {
            Ok((false, f449(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f79(f65(f448(f450(v4.clone()), f70(f71(v0, f8(Str::lit("model/authorities.toml"))))?), f448(f451(v4), f65(v6, f73(v3, f8(Str::lit("id")))))))?;
            let a2463 = {
                let a2462 = v8.clone();
                length_list(a2462)
            };
            let a2464 = 0u64;
            if nat_eq(a2463, a2464) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f449(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f122(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f414(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f37({
        let (h2465, _) = v0;
        h2465
    }, {
        let (h2466, _) = v1;
        h2466
    }) {
        f37({
            let (_, h2467) = v0;
            h2467
        }, {
            let (_, h2468) = v1;
            h2468
        })
    } else {
        false
    }
}

pub fn f415(v0: Fn14, p2472: u64, p2473: Adt27) -> R<Adt27> {
    let m2471 = (p2472, p2473);
    {
        let mut m2471 = m2471;
        loop {
            let step = {
                let (v1, v2) = m2471;
                let m2469 = v1;
                if m2469 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2469 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2470: Fn14 = v0.clone();
                        c2470.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2471 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(v0: Adt27) -> R<Adt27> {
    f452(v0)
}

pub fn f417(v0: u32, v1: u32) -> u32 {
    let m2476 = {
        let a2474 = f453(v0);
        let a2475 = f453(v1);
        fixed_u64::checked_add(a2474, a2475)
    };
    match m2476 {
        None => {
            0u32
        }
        Some(v2) => {
            f454(v2)
        }
    }
}

pub fn f418(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2479 = {
        let a2477 = v3;
        let a2478 = 8u64;
        nat_add(a2477, a2478)?
    };
    let a2480 = v2;
    if nat_eq(a2479, a2480) {
        Ok(f455({
            let a2483 = {
                let a2481 = v1;
                let a2482 = 8u64;
                nat_mul(a2481, a2482)?
            };
            let a2484 = 4294967296u64;
            let a2485 = 0u64;
            nat_quot(a2483, a2484, a2485)
        }))
    } else {
        let a2488 = {
            let a2486 = v3;
            let a2487 = 4u64;
            nat_add(a2486, a2487)?
        };
        let a2489 = v2;
        if nat_eq(a2488, a2489) {
            Ok(f455({
                let a2492 = {
                    let a2490 = v1;
                    let a2491 = 8u64;
                    nat_mul(a2490, a2491)?
                };
                let a2493 = 4294967296u64;
                let a2494 = 0u64;
                nat_rem(a2492, a2493, a2494)
            }))
        } else {
            Ok(f456(f457(v0.clone(), v1, v3), f457(v0.clone(), v1, {
                let a2495 = v3;
                let a2496 = 1u64;
                nat_add(a2495, a2496)?
            }), f457(v0.clone(), v1, {
                let a2497 = v3;
                let a2498 = 2u64;
                nat_add(a2497, a2498)?
            }), f457(v0, v1, {
                let a2499 = v3;
                let a2500 = 3u64;
                nat_add(a2499, a2500)?
            })))
        }
    }
}

pub fn f419(v0: Bytes, v1: u64) -> bool {
    let m2501 = f253(v0, v1);
    match m2501 {
        None => {
            false
        }
        Some(v2) => {
            f458(v2)
        }
    }
}

pub fn f420(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2504 = {
        let a2502 = v0.clone();
        let a2503 = v1;
        index_bytes(a2502, a2503)
    };
    match m2504 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2549 = {
                    let a2525 = {
                        let a2513 = {
                            let a2507 = {
                                let a2505 = v3;
                                let a2506 = 128u8;
                                fixed_u8::bit_and(a2505, a2506)
                            };
                            let a2508 = 128u8;
                            if equal(a2507, a2508) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2514 = {
                            let a2511 = {
                                let a2509 = v3;
                                let a2510 = 64u8;
                                fixed_u8::bit_and(a2509, a2510)
                            };
                            let a2512 = 64u8;
                            if equal(a2511, a2512) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2513, a2514)?
                    };
                    let a2526 = {
                        let a2523 = {
                            let a2517 = {
                                let a2515 = v3;
                                let a2516 = 32u8;
                                fixed_u8::bit_and(a2515, a2516)
                            };
                            let a2518 = 32u8;
                            if equal(a2517, a2518) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2524 = {
                            let a2521 = {
                                let a2519 = v3;
                                let a2520 = 16u8;
                                fixed_u8::bit_and(a2519, a2520)
                            };
                            let a2522 = 16u8;
                            if equal(a2521, a2522) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2523, a2524)?
                    };
                    nat_add(a2525, a2526)?
                };
                let a2550 = {
                    let a2547 = {
                        let a2535 = {
                            let a2529 = {
                                let a2527 = v3;
                                let a2528 = 8u8;
                                fixed_u8::bit_and(a2527, a2528)
                            };
                            let a2530 = 8u8;
                            if equal(a2529, a2530) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2536 = {
                            let a2533 = {
                                let a2531 = v3;
                                let a2532 = 4u8;
                                fixed_u8::bit_and(a2531, a2532)
                            };
                            let a2534 = 4u8;
                            if equal(a2533, a2534) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2535, a2536)?
                    };
                    let a2548 = {
                        let a2545 = {
                            let a2539 = {
                                let a2537 = v3;
                                let a2538 = 2u8;
                                fixed_u8::bit_and(a2537, a2538)
                            };
                            let a2540 = 2u8;
                            if equal(a2539, a2540) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2546 = {
                            let a2543 = {
                                let a2541 = v3;
                                let a2542 = 1u8;
                                fixed_u8::bit_and(a2541, a2542)
                            };
                            let a2544 = 1u8;
                            if equal(a2543, a2544) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2545, a2546)?
                    };
                    nat_add(a2547, a2548)?
                };
                nat_add(a2549, a2550)?
            };
            let a2551 = v4;
            let a2552 = 128u64;
            if nat_lt(a2551, a2552) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2553 = 194u64;
                let a2554 = v4;
                if if nat_le(a2553, a2554) {
                    let a2555 = v4;
                    let a2556 = 223u64;
                    nat_le(a2555, a2556)
                } else {
                    false
                } {
                    let m2559 = f459(v0, {
                        let a2557 = v1;
                        let a2558 = 1u64;
                        nat_add(a2557, a2558)?
                    })?;
                    match m2559 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f460({
                                let a2564 = {
                                    let a2562 = {
                                        let a2560 = v4;
                                        let a2561 = 192u64;
                                        nat_sub(a2560, a2561)
                                    };
                                    let a2563 = 64u64;
                                    nat_mul(a2562, a2563)?
                                };
                                let a2565 = v5;
                                nat_add(a2564, a2565)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2566 = 224u64;
                    let a2567 = v4;
                    if if nat_le(a2566, a2567) {
                        let a2568 = v4;
                        let a2569 = 239u64;
                        nat_le(a2568, a2569)
                    } else {
                        false
                    } {
                        let m2572 = f459(v0.clone(), {
                            let a2570 = v1;
                            let a2571 = 1u64;
                            nat_add(a2570, a2571)?
                        })?;
                        match m2572 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2575 = f459(v0, {
                                    let a2573 = v1;
                                    let a2574 = 2u64;
                                    nat_add(a2573, a2574)?
                                })?;
                                match m2575 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f460({
                                            let a2584 = {
                                                let a2578 = {
                                                    let a2576 = v4;
                                                    let a2577 = 224u64;
                                                    nat_sub(a2576, a2577)
                                                };
                                                let a2579 = 4096u64;
                                                nat_mul(a2578, a2579)?
                                            };
                                            let a2585 = {
                                                let a2582 = {
                                                    let a2580 = v6;
                                                    let a2581 = 64u64;
                                                    nat_mul(a2580, a2581)?
                                                };
                                                let a2583 = v7;
                                                nat_add(a2582, a2583)?
                                            };
                                            nat_add(a2584, a2585)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2586 = 240u64;
                        let a2587 = v4;
                        if if nat_le(a2586, a2587) {
                            let a2588 = v4;
                            let a2589 = 244u64;
                            nat_le(a2588, a2589)
                        } else {
                            false
                        } {
                            let m2592 = f459(v0.clone(), {
                                let a2590 = v1;
                                let a2591 = 1u64;
                                nat_add(a2590, a2591)?
                            })?;
                            match m2592 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2595 = f459(v0.clone(), {
                                        let a2593 = v1;
                                        let a2594 = 2u64;
                                        nat_add(a2593, a2594)?
                                    })?;
                                    match m2595 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2598 = f459(v0, {
                                                let a2596 = v1;
                                                let a2597 = 3u64;
                                                nat_add(a2596, a2597)?
                                            })?;
                                            match m2598 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f460({
                                                        let a2611 = {
                                                            let a2601 = {
                                                                let a2599 = v4;
                                                                let a2600 = 240u64;
                                                                nat_sub(a2599, a2600)
                                                            };
                                                            let a2602 = 262144u64;
                                                            nat_mul(a2601, a2602)?
                                                        };
                                                        let a2612 = {
                                                            let a2609 = {
                                                                let a2603 = v8;
                                                                let a2604 = 4096u64;
                                                                nat_mul(a2603, a2604)?
                                                            };
                                                            let a2610 = {
                                                                let a2607 = {
                                                                    let a2605 = v9;
                                                                    let a2606 = 64u64;
                                                                    nat_mul(a2605, a2606)?
                                                                };
                                                                let a2608 = v10;
                                                                nat_add(a2607, a2608)?
                                                            };
                                                            nat_add(a2609, a2610)?
                                                        };
                                                        nat_add(a2611, a2612)?
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

pub fn f421(v0: u64) -> bool {
    let a2613 = v0;
    let a2614 = 9u64;
    if if if nat_eq(a2613, a2614) {
        true
    } else {
        let a2615 = v0;
        let a2616 = 10u64;
        nat_eq(a2615, a2616)
    } {
        true
    } else {
        let a2617 = v0;
        let a2618 = 13u64;
        nat_eq(a2617, a2618)
    } {
        true
    } else {
        let a2619 = 32u64;
        let a2620 = v0;
        if if if nat_le(a2619, a2620) {
            let a2621 = v0;
            let a2622 = 55295u64;
            nat_le(a2621, a2622)
        } else {
            false
        } {
            true
        } else {
            let a2623 = 57344u64;
            let a2624 = v0;
            if nat_le(a2623, a2624) {
                let a2625 = v0;
                let a2626 = 65533u64;
                nat_le(a2625, a2626)
            } else {
                false
            }
        } {
            true
        } else {
            let a2627 = 65536u64;
            let a2628 = v0;
            if nat_le(a2627, a2628) {
                let a2629 = v0;
                let a2630 = 1114111u64;
                nat_le(a2629, a2630)
            } else {
                false
            }
        }
    }
}

pub fn f422(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f221(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2632 = v2;
        let a2633 = {
            let a2631 = v0;
            length_bytes(a2631)
        };
        if nat_lt(a2632, a2633) {
            Ok(Some({
                let a2634 = v2;
                let a2635 = 1u64;
                nat_add(a2634, a2635)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f423(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f309(f228(v0, v1)?) {
        Ok(Some({
            let a2636 = v1;
            let a2637 = 1u64;
            nat_add(a2636, a2637)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f424(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2638 = f420(v0, v2)?;
    match m2638 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2639 = v2;
            let a2640 = v1;
            if if nat_eq(a2639, a2640) {
                f461({
                    let (h2641, _) = v3;
                    h2641
                })
            } else {
                f462({
                    let (h2642, _) = v3;
                    h2642
                })
            } {
                Ok(Some({
                    let a2644 = v2;
                    let a2645 = {
                        let (_, h2643) = v3;
                        h2643
                    };
                    nat_add(a2644, a2645)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f425(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2646, _) = f463(Fn16::F464(v1, v0.clone()), {
        let a2648 = {
            let a2647 = v0;
            length_bytes(a2647)
        };
        let a2649 = 1u64;
        nat_add(a2648, a2649)?
    }, (v2, 0u64))?;
    Ok(h2646)
}

pub fn f426(v0: u64) -> R<Bytes> {
    let a2650 = v0;
    let a2651 = 128u64;
    if nat_lt(a2650, a2651) {
        Ok(f465(v0))
    } else {
        let a2652 = v0;
        let a2653 = 2048u64;
        if nat_lt(a2652, a2653) {
            let a2664 = f465({
                let a2657 = 192u64;
                let a2658 = {
                    let a2654 = v0;
                    let a2655 = 64u64;
                    let a2656 = 0u64;
                    nat_quot(a2654, a2655, a2656)
                };
                nat_add(a2657, a2658)?
            });
            let a2665 = f465({
                let a2662 = 128u64;
                let a2663 = {
                    let a2659 = v0;
                    let a2660 = 64u64;
                    let a2661 = 0u64;
                    nat_rem(a2659, a2660, a2661)
                };
                nat_add(a2662, a2663)?
            });
            Ok(append_bytes(a2664, a2665))
        } else {
            let a2666 = v0;
            let a2667 = 65536u64;
            if nat_lt(a2666, a2667) {
                let a2688 = f465({
                    let a2671 = 224u64;
                    let a2672 = {
                        let a2668 = v0;
                        let a2669 = 4096u64;
                        let a2670 = 0u64;
                        nat_quot(a2668, a2669, a2670)
                    };
                    nat_add(a2671, a2672)?
                });
                let a2689 = {
                    let a2686 = f465({
                        let a2679 = 128u64;
                        let a2680 = {
                            let a2676 = {
                                let a2673 = v0;
                                let a2674 = 64u64;
                                let a2675 = 0u64;
                                nat_quot(a2673, a2674, a2675)
                            };
                            let a2677 = 64u64;
                            let a2678 = 0u64;
                            nat_rem(a2676, a2677, a2678)
                        };
                        nat_add(a2679, a2680)?
                    });
                    let a2687 = f465({
                        let a2684 = 128u64;
                        let a2685 = {
                            let a2681 = v0;
                            let a2682 = 64u64;
                            let a2683 = 0u64;
                            nat_rem(a2681, a2682, a2683)
                        };
                        nat_add(a2684, a2685)?
                    });
                    append_bytes(a2686, a2687)
                };
                Ok(append_bytes(a2688, a2689))
            } else {
                let a2720 = f465({
                    let a2693 = 240u64;
                    let a2694 = {
                        let a2690 = v0;
                        let a2691 = 262144u64;
                        let a2692 = 0u64;
                        nat_quot(a2690, a2691, a2692)
                    };
                    nat_add(a2693, a2694)?
                });
                let a2721 = {
                    let a2718 = f465({
                        let a2701 = 128u64;
                        let a2702 = {
                            let a2698 = {
                                let a2695 = v0;
                                let a2696 = 4096u64;
                                let a2697 = 0u64;
                                nat_quot(a2695, a2696, a2697)
                            };
                            let a2699 = 64u64;
                            let a2700 = 0u64;
                            nat_rem(a2698, a2699, a2700)
                        };
                        nat_add(a2701, a2702)?
                    });
                    let a2719 = {
                        let a2716 = f465({
                            let a2709 = 128u64;
                            let a2710 = {
                                let a2706 = {
                                    let a2703 = v0;
                                    let a2704 = 64u64;
                                    let a2705 = 0u64;
                                    nat_quot(a2703, a2704, a2705)
                                };
                                let a2707 = 64u64;
                                let a2708 = 0u64;
                                nat_rem(a2706, a2707, a2708)
                            };
                            nat_add(a2709, a2710)?
                        });
                        let a2717 = f465({
                            let a2714 = 128u64;
                            let a2715 = {
                                let a2711 = v0;
                                let a2712 = 64u64;
                                let a2713 = 0u64;
                                nat_rem(a2711, a2712, a2713)
                            };
                            nat_add(a2714, a2715)?
                        });
                        append_bytes(a2716, a2717)
                    };
                    append_bytes(a2718, a2719)
                };
                Ok(append_bytes(a2720, a2721))
            }
        }
    }
}

pub fn f427(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f466(v2, v0, v1, v3)
}

pub fn f428(v0: Fn15, p2726: u64, p2727: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2725 = (p2726, p2727);
    {
        let mut m2725 = m2725;
        loop {
            let step = {
                let (v1, v2) = m2725;
                let m2722 = v1;
                if m2722 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2722 - 1;
                    let m2724 = {
                        let c2723: Fn15 = v0.clone();
                        c2723.apply(v2.clone())?
                    };
                    match m2724 {
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
                Ok(next) => { m2725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f467(v0, v1)
}

pub fn f430(v0: Bytes, v1: u64) -> R<u64> {
    let (h2728, _) = f256(Fn3::F468(v0.clone()), {
        let a2730 = {
            let a2729 = v0;
            length_bytes(a2729)
        };
        let a2731 = 1u64;
        nat_add(a2730, a2731)?
    }, v1)?;
    Ok(h2728)
}

pub fn f431(v0: Bytes) -> R<u64> {
    let (h2732, _) = f256(Fn3::F469(v0.clone()), {
        let a2734 = {
            let a2733 = v0;
            length_bytes(a2733)
        };
        let a2735 = 1u64;
        nat_add(a2734, a2735)?
    }, 0u64)?;
    Ok(h2732)
}

pub fn f432(v0: Bytes) -> R<Option<u64>> {
    if f470(v0.clone())? {
        Ok(Some({
            let (_, h2736) = {
                let (h2737, _) = f463(Fn16::F471(v0.clone()), {
                    let a2738 = v0;
                    length_bytes(a2738)
                }, (0u64, 0u64))?;
                h2737
            };
            h2736
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f433(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2740 = {
            let a2739 = v1.clone();
            length_bytes(a2739)
        };
        let a2741 = f472(v1)?;
        nat_sub(a2740, a2741)
    });
    let v5: Bytes = {
        let a2742 = v0;
        let a2743 = v4.clone();
        append_bytes(a2742, a2743)
    };
    let v6: u64 = f431(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2745 = {
            let a2744 = v5;
            length_bytes(a2744)
        };
        let a2746 = v6;
        nat_sub(a2745, a2746)
    });
    let v8: u64 = {
        let a2747 = v4;
        length_bytes(a2747)
    };
    let v9: u64 = {
        let a2748 = v7.clone();
        length_bytes(a2748)
    };
    let a2749 = v9;
    let a2750 = 0u64;
    if nat_eq(a2749, a2750) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a2751 = v3;
            let a2752 = v8;
            nat_lt(a2751, a2752)
        } {
            let v10: u64 = if v2 {
                let a2753 = v3;
                let a2754 = v8;
                nat_add(a2753, a2754)?
            } else {
                let a2755 = v8;
                let a2756 = v3;
                nat_sub(a2755, a2756)
            };
            let a2757 = v10;
            let a2758 = f472(v7.clone())?;
            if nat_le(a2757, a2758) {
                Ok(Some(f32(v7, 0u64, {
                    let a2759 = v9;
                    let a2760 = v10;
                    nat_sub(a2759, a2760)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a2761 = v3;
                let a2762 = v8;
                nat_sub(a2761, a2762)
            };
            let a2765 = {
                let a2763 = v9;
                let a2764 = v11;
                nat_add(a2763, a2764)?
            };
            let a2766 = 19u64;
            if nat_le(a2765, a2766) {
                Ok(Some({
                    let a2767 = v7;
                    let a2768 = f399(v11);
                    append_bytes(a2767, a2768)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f434(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2769 = 0u64;
        let a2770 = v1;
        if nat_lt(a2769, a2770) {
            let a2771 = v1;
            let a2772 = 9223372036854775808u64;
            nat_le(a2771, a2772)
        } else {
            false
        }
    } else {
        let a2773 = v1;
        let a2774 = 9223372036854775807u64;
        nat_le(a2773, a2774)
    }
}

pub fn f435(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2776 = {
            let a2775 = v1.clone();
            length_bytes(a2775)
        };
        let a2777 = f472(v1)?;
        nat_sub(a2776, a2777)
    });
    let v5: Bytes = {
        let a2778 = v0;
        let a2779 = v4.clone();
        append_bytes(a2778, a2779)
    };
    let v6: u64 = f431(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2781 = {
            let a2780 = v5;
            length_bytes(a2780)
        };
        let a2782 = v6;
        nat_sub(a2781, a2782)
    });
    let v8: u64 = {
        let a2783 = v4;
        length_bytes(a2783)
    };
    let v9: u64 = {
        let a2784 = v7.clone();
        length_bytes(a2784)
    };
    let a2785 = v9;
    let a2786 = 0u64;
    if nat_eq(a2785, a2786) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if if v2 {
            true
        } else {
            let a2787 = v3;
            let a2788 = v8;
            nat_lt(a2787, a2788)
        } {
            let v10: u64 = if v2 {
                let a2789 = v3;
                let a2790 = v8;
                nat_add(a2789, a2790)?
            } else {
                let a2791 = v8;
                let a2792 = v3;
                nat_sub(a2791, a2792)
            };
            let v11: u64 = f472(v7.clone())?;
            let a2793 = v10;
            let a2794 = v11;
            if nat_le(a2793, a2794) {
                Ok(Some((f32(v7, 0u64, {
                    let a2795 = v9;
                    let a2796 = v10;
                    nat_sub(a2795, a2796)
                }), 0u64)))
            } else {
                let a2799 = {
                    let a2797 = v10;
                    let a2798 = v11;
                    nat_sub(a2797, a2798)
                };
                let a2800 = 19u64;
                if nat_le(a2799, a2800) {
                    Ok(Some((f32(v7, 0u64, {
                        let a2801 = v9;
                        let a2802 = v11;
                        nat_sub(a2801, a2802)
                    }), {
                        let a2803 = v10;
                        let a2804 = v11;
                        nat_sub(a2803, a2804)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a2805 = v3;
                let a2806 = v8;
                nat_sub(a2805, a2806)
            };
            let a2809 = {
                let a2807 = v9;
                let a2808 = v12;
                nat_add(a2807, a2808)?
            };
            let a2810 = 19u64;
            if nat_le(a2809, a2810) {
                Ok(Some(({
                    let a2811 = v7;
                    let a2812 = f399(v12);
                    append_bytes(a2811, a2812)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f436(v0: Adt19) -> List<(Bytes, Adt19)> {
    let m2813 = v0;
    match m2813 {
        Adt19::C0(_) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C1(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C2(_, _, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C3(_) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C4(_) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C5(r2814) => {
            let v4 = (*r2814).clone();
            f436(v4)
        }
        Adt19::C6 => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C7(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C8(v1, r2815, r2816) => {
            let v2 = (*r2815).clone();
            let v3 = (*r2816).clone();
            List::cons((v1, v2), f436(v3))
        }
    }
}

pub fn f437(p2821: List<(Bytes, Adt19)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt19)>> {
    let m2820 = p2821;
    {
        let mut m2820 = m2820;
        loop {
            let step = {
                let v0 = m2820;
                let m2817 = v0;
                match m2817.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(None::<(Bytes, Adt19)>))
                    }
                    Some((v3, v4)) => {
                        if f473(f474({
                            let (_, h2818) = v3.clone();
                            h2818
                        }, List::<Bytes>::nil())?, f243({
                            let (h2819, _) = v3.clone();
                            h2819
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: List<(Bytes, Adt19)>, v1: Bytes) -> List<(Bytes, Adt19)> {
    let m2822 = v0;
    match m2822.uncons() {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some((v2, v3)) => {
            if f37({
                let (h2823, _) = v2.clone();
                h2823
            }, v1.clone()) {
                f438(v3, v1)
            } else {
                List::cons(v2, f438(v3, v1))
            }
        }
    }
}

pub fn f439() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f440() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f441(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f475(v0, f476(v1))
}

pub fn f442(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2824 = v0;
    match m2824.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f442(v3, false))
            } else {
                f442(v3, true)
            }
        }
    }
}

pub fn f443(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2828 = {
        let a2826 = v0.clone();
        let a2827 = {
            let (h2825, _) = v1.clone();
            h2825
        };
        index_bytes(a2826, a2827)
    };
    match m2828 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2830 = {
                    let (h2829, _) = v1.clone();
                    h2829
                };
                let a2831 = 1u64;
                nat_add(a2830, a2831)?
            }, {
                let a2834 = {
                    let (_, h2832) = v1;
                    h2832
                };
                let a2835 = f477(v0, {
                    let (h2833, _) = v1;
                    h2833
                }, v2)?;
                append_bytes(a2834, a2835)
            })))
        }
    }
}

pub fn f444(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2836 = v0.clone();
    match m2836.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2839 = {
                let (h2837, _) = v2;
                h2837
            };
            let a2840 = {
                let (h2838, _) = v1.clone();
                h2838
            };
            if equal(a2839, a2840) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f445(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f478(f479(Fn17::F480, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2845 = {
        let (h2841, _) = f481(Fn18::F482, {
            let a2843 = {
                let a2842 = v0;
                length_list(a2842)
            };
            let a2844 = 1u64;
            nat_add(a2843, a2844)?
        }, v1)?;
        h2841
    };
    match m2845.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f446(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2850 = {
        let a2847 = f336({
            let (h2846, _) = v0;
            h2846
        })?;
        let a2848 = Bytes::lit(&[0x3a]);
        append_bytes(a2847, a2848)
    };
    let a2851 = {
        let (_, h2849) = v0;
        h2849
    };
    Ok(append_bytes(a2850, a2851))
}

pub fn f447(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f350({
        let (_, h2852) = {
            let (h2853, _) = f483(Fn19::F484, {
                let a2855 = {
                    let a2854 = v0.clone();
                    length_list(a2854)
                };
                let a2856 = 1u64;
                nat_add(a2855, a2856)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2853
        };
        h2852
    }))
}

pub fn f448(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2857 = v0;
    match m2857.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f87(v2.clone(), v1.clone()) {
                f448(v3, v1)
            } else {
                List::cons(v2, f448(v3, v1))
            }
        }
    }
}

pub fn f449(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f450(v0: List<Adt14>) -> List<Bytes> {
    let m2858 = v0;
    match m2858.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2859 = f160(v1, f8(Str::lit("std")));
            match m2859 {
                None => {
                    f450(v2)
                }
                Some(v3) => {
                    List::cons(f158(v3, f8(Str::lit("authority"))), f450(v2))
                }
            }
        }
    }
}

pub fn f451(v0: List<Adt14>) -> List<Bytes> {
    let m2860 = v0;
    match m2860.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2861 = f161(f160(v1, f8(Str::lit("req"))));
            match m2861 {
                None => {
                    f451(v2)
                }
                Some(v3) => {
                    List::cons(v3, f451(v2))
                }
            }
        }
    }
}

pub fn f452(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f485(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2862) => {
            h2862
        }
    });
    let v2: u32 = f417(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2863, _, _) => {
            h2863
        }
    }, f417(f486(match v0.clone() {
        Adt27::C0(_, _, _, _, h2864, _, _, _, _, _) => {
            h2864
        }
    }), f417(f487(match v0.clone() {
        Adt27::C0(_, _, _, _, h2865, _, _, _, _, _) => {
            h2865
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2866, _, _, _, _) => {
            h2866
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2867, _, _, _) => {
            h2867
        }
    }), f417(v1, {
        let m2870 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2869, _) => {
                h2869
            }
        };
        match m2870 {
            Adt26::C0(h2868, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2868
            }
        }
    }))));
    let v3: u32 = f417(f488(match v0.clone() {
        Adt27::C0(h2871, _, _, _, _, _, _, _, _, _) => {
            h2871
        }
    }), f489(match v0.clone() {
        Adt27::C0(h2872, _, _, _, _, _, _, _, _, _) => {
            h2872
        }
    }, match v0.clone() {
        Adt27::C0(_, h2873, _, _, _, _, _, _, _, _) => {
            h2873
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2874, _, _, _, _, _, _, _) => {
            h2874
        }
    }));
    Ok(Adt27::C0(f417(v2, v3), match v0.clone() {
        Adt27::C0(h2875, _, _, _, _, _, _, _, _, _) => {
            h2875
        }
    }, match v0.clone() {
        Adt27::C0(_, h2876, _, _, _, _, _, _, _, _) => {
            h2876
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2877, _, _, _, _, _, _, _) => {
            h2877
        }
    }, f417(match v0.clone() {
        Adt27::C0(_, _, _, h2878, _, _, _, _, _, _) => {
            h2878
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2879, _, _, _, _, _) => {
            h2879
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2880, _, _, _, _) => {
            h2880
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2881, _, _, _) => {
            h2881
        }
    }, f490(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2882, _) => {
            h2882
        }
    }), {
        let a2884 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2883) => {
                h2883
            }
        };
        let a2885 = 1u64;
        nat_add(a2884, a2885)?
    }))
}

pub fn f453(v0: u32) -> u64 {
    let m2887 = {
        let a2886 = v0;
        fixed_u64::convert(i128::from(a2886))
    };
    match m2887 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f454(v0: u64) -> u32 {
    let m2891 = {
        let a2890 = {
            let a2888 = v0;
            let a2889 = 4294967295u64;
            fixed_u64::bit_and(a2888, a2889)
        };
        fixed_u32::convert(i128::from(a2890))
    };
    match m2891 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f455(v0: u64) -> u32 {
    let (_, h2892) = {
        let (_, h2893) = f491(Fn20::F492, 32u64, (v0, (1u32, 0u32)));
        h2893
    };
    h2892
}

pub fn f456(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2898 = f493(v0, 24u32);
    let a2899 = {
        let a2896 = f493(v1, 16u32);
        let a2897 = {
            let a2894 = f493(v2, 8u32);
            let a2895 = v3;
            fixed_u32::bit_or(a2894, a2895)
        };
        fixed_u32::bit_or(a2896, a2897)
    };
    fixed_u32::bit_or(a2898, a2899)
}

pub fn f457(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2900 = v2;
    let a2901 = v1;
    if nat_lt(a2900, a2901) {
        let m2904 = {
            let a2902 = v0;
            let a2903 = v2;
            index_bytes(a2902, a2903)
        };
        match m2904 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2906 = {
                    let a2905 = v3;
                    fixed_u32::convert(i128::from(a2905))
                };
                match m2906 {
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
        let a2907 = v2;
        let a2908 = v1;
        if nat_eq(a2907, a2908) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f458(v0: u8) -> bool {
    let a2909 = v0;
    let a2910 = 32u8;
    if if if equal(a2909, a2910) {
        true
    } else {
        let a2911 = v0;
        let a2912 = 9u8;
        equal(a2911, a2912)
    } {
        true
    } else {
        let a2913 = v0;
        let a2914 = 10u8;
        equal(a2913, a2914)
    } {
        true
    } else {
        let a2915 = v0;
        let a2916 = 12u8;
        if equal(a2915, a2916) {
            true
        } else {
            let a2917 = v0;
            let a2918 = 13u8;
            equal(a2917, a2918)
        }
    }
}

pub fn f459(v0: Bytes, v1: u64) -> R<Option<u64>> {
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
            let v4: u64 = {
                let v3: u8 = v2;
                let a2966 = {
                    let a2942 = {
                        let a2930 = {
                            let a2924 = {
                                let a2922 = v3;
                                let a2923 = 128u8;
                                fixed_u8::bit_and(a2922, a2923)
                            };
                            let a2925 = 128u8;
                            if equal(a2924, a2925) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2931 = {
                            let a2928 = {
                                let a2926 = v3;
                                let a2927 = 64u8;
                                fixed_u8::bit_and(a2926, a2927)
                            };
                            let a2929 = 64u8;
                            if equal(a2928, a2929) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2930, a2931)?
                    };
                    let a2943 = {
                        let a2940 = {
                            let a2934 = {
                                let a2932 = v3;
                                let a2933 = 32u8;
                                fixed_u8::bit_and(a2932, a2933)
                            };
                            let a2935 = 32u8;
                            if equal(a2934, a2935) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2941 = {
                            let a2938 = {
                                let a2936 = v3;
                                let a2937 = 16u8;
                                fixed_u8::bit_and(a2936, a2937)
                            };
                            let a2939 = 16u8;
                            if equal(a2938, a2939) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2940, a2941)?
                    };
                    nat_add(a2942, a2943)?
                };
                let a2967 = {
                    let a2964 = {
                        let a2952 = {
                            let a2946 = {
                                let a2944 = v3;
                                let a2945 = 8u8;
                                fixed_u8::bit_and(a2944, a2945)
                            };
                            let a2947 = 8u8;
                            if equal(a2946, a2947) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2953 = {
                            let a2950 = {
                                let a2948 = v3;
                                let a2949 = 4u8;
                                fixed_u8::bit_and(a2948, a2949)
                            };
                            let a2951 = 4u8;
                            if equal(a2950, a2951) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2952, a2953)?
                    };
                    let a2965 = {
                        let a2962 = {
                            let a2956 = {
                                let a2954 = v3;
                                let a2955 = 2u8;
                                fixed_u8::bit_and(a2954, a2955)
                            };
                            let a2957 = 2u8;
                            if equal(a2956, a2957) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2963 = {
                            let a2960 = {
                                let a2958 = v3;
                                let a2959 = 1u8;
                                fixed_u8::bit_and(a2958, a2959)
                            };
                            let a2961 = 1u8;
                            if equal(a2960, a2961) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2962, a2963)?
                    };
                    nat_add(a2964, a2965)?
                };
                nat_add(a2966, a2967)?
            };
            let a2968 = 128u64;
            let a2969 = v4;
            if if nat_le(a2968, a2969) {
                let a2970 = v4;
                let a2971 = 191u64;
                nat_le(a2970, a2971)
            } else {
                false
            } {
                Ok(Some({
                    let a2972 = v4;
                    let a2973 = 128u64;
                    nat_sub(a2972, a2973)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f460(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a2974 = v1;
    let a2975 = v0;
    if if nat_le(a2974, a2975) {
        let a2976 = v0;
        let a2977 = v2;
        if nat_le(a2976, a2977) {
            let a2978 = v0;
            let a2979 = 55296u64;
            if nat_lt(a2978, a2979) {
                true
            } else {
                let a2980 = 57343u64;
                let a2981 = v0;
                nat_lt(a2980, a2981)
            }
        } else {
            false
        }
    } else {
        false
    } {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f461(v0: u64) -> bool {
    let a2982 = v0;
    let a2983 = 58u64;
    if if if nat_eq(a2982, a2983) {
        true
    } else {
        let a2984 = v0;
        let a2985 = 95u64;
        nat_eq(a2984, a2985)
    } {
        true
    } else {
        let a2986 = 65u64;
        let a2987 = v0;
        if if nat_le(a2986, a2987) {
            let a2988 = v0;
            let a2989 = 90u64;
            nat_le(a2988, a2989)
        } else {
            false
        } {
            true
        } else {
            let a2990 = 97u64;
            let a2991 = v0;
            if nat_le(a2990, a2991) {
                let a2992 = v0;
                let a2993 = 122u64;
                nat_le(a2992, a2993)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2994 = 192u64;
        let a2995 = v0;
        if if if if nat_le(a2994, a2995) {
            let a2996 = v0;
            let a2997 = 214u64;
            nat_le(a2996, a2997)
        } else {
            false
        } {
            true
        } else {
            let a2998 = 216u64;
            let a2999 = v0;
            if nat_le(a2998, a2999) {
                let a3000 = v0;
                let a3001 = 246u64;
                nat_le(a3000, a3001)
            } else {
                false
            }
        } {
            true
        } else {
            let a3002 = 248u64;
            let a3003 = v0;
            if if nat_le(a3002, a3003) {
                let a3004 = v0;
                let a3005 = 767u64;
                nat_le(a3004, a3005)
            } else {
                false
            } {
                true
            } else {
                let a3006 = 880u64;
                let a3007 = v0;
                if nat_le(a3006, a3007) {
                    let a3008 = v0;
                    let a3009 = 893u64;
                    nat_le(a3008, a3009)
                } else {
                    false
                }
            }
        } {
            true
        } else {
            let a3010 = 895u64;
            let a3011 = v0;
            if if if if nat_le(a3010, a3011) {
                let a3012 = v0;
                let a3013 = 8191u64;
                nat_le(a3012, a3013)
            } else {
                false
            } {
                true
            } else {
                let a3014 = 8204u64;
                let a3015 = v0;
                if nat_le(a3014, a3015) {
                    let a3016 = v0;
                    let a3017 = 8205u64;
                    nat_le(a3016, a3017)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3018 = 8304u64;
                let a3019 = v0;
                if if nat_le(a3018, a3019) {
                    let a3020 = v0;
                    let a3021 = 8591u64;
                    nat_le(a3020, a3021)
                } else {
                    false
                } {
                    true
                } else {
                    let a3022 = 11264u64;
                    let a3023 = v0;
                    if nat_le(a3022, a3023) {
                        let a3024 = v0;
                        let a3025 = 12271u64;
                        nat_le(a3024, a3025)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a3026 = 12289u64;
                let a3027 = v0;
                if if if nat_le(a3026, a3027) {
                    let a3028 = v0;
                    let a3029 = 55295u64;
                    nat_le(a3028, a3029)
                } else {
                    false
                } {
                    true
                } else {
                    let a3030 = 63744u64;
                    let a3031 = v0;
                    if nat_le(a3030, a3031) {
                        let a3032 = v0;
                        let a3033 = 64975u64;
                        nat_le(a3032, a3033)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a3034 = 65008u64;
                    let a3035 = v0;
                    if if nat_le(a3034, a3035) {
                        let a3036 = v0;
                        let a3037 = 65533u64;
                        nat_le(a3036, a3037)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a3038 = 65536u64;
                        let a3039 = v0;
                        if nat_le(a3038, a3039) {
                            let a3040 = v0;
                            let a3041 = 983039u64;
                            nat_le(a3040, a3041)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f462(v0: u64) -> bool {
    if f461(v0) {
        true
    } else {
        let a3042 = v0;
        let a3043 = 45u64;
        if if if nat_eq(a3042, a3043) {
            true
        } else {
            let a3044 = v0;
            let a3045 = 46u64;
            nat_eq(a3044, a3045)
        } {
            true
        } else {
            let a3046 = 48u64;
            let a3047 = v0;
            if nat_le(a3046, a3047) {
                let a3048 = v0;
                let a3049 = 57u64;
                nat_le(a3048, a3049)
            } else {
                false
            }
        } {
            true
        } else {
            let a3050 = v0;
            let a3051 = 183u64;
            if if nat_eq(a3050, a3051) {
                true
            } else {
                let a3052 = 768u64;
                let a3053 = v0;
                if nat_le(a3052, a3053) {
                    let a3054 = v0;
                    let a3055 = 879u64;
                    nat_le(a3054, a3055)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3056 = 8255u64;
                let a3057 = v0;
                if nat_le(a3056, a3057) {
                    let a3058 = v0;
                    let a3059 = 8256u64;
                    nat_le(a3058, a3059)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f463(v0: Fn16, p3064: u64, p3065: (u64, u64)) -> R<((u64, u64), bool)> {
    let m3063 = (p3064, p3065);
    {
        let mut m3063 = m3063;
        loop {
            let step = {
                let (v1, v2) = m3063;
                let m3060 = v1;
                if m3060 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3060 - 1;
                    let m3062 = {
                        let c3061: Fn16 = v0.clone();
                        c3061.apply(v2)?
                    };
                    match m3062 {
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
                Ok(next) => { m3063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f464(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f494(v1, v0, v2)
}

pub fn f465(v0: u64) -> Bytes {
    f495(f496({
        let a3066 = v0;
        let a3067 = 16u64;
        let a3068 = 0u64;
        nat_quot(a3066, a3067, a3068)
    }), {
        let a3069 = v0;
        let a3070 = 16u64;
        let a3071 = 0u64;
        nat_rem(a3069, a3070, a3071)
    })
}

pub fn f466(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f497(f228(v0, v3)?, v1, v2) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3072 = v3;
            let a3073 = 1u64;
            nat_add(a3072, a3073)?
        }))
    }
}

pub fn f467(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h3074, _) = v1.clone();
        h3074
    };
    let v3: Bytes = {
        let (h3075, _) = {
            let (_, h3076) = v1.clone();
            h3076
        };
        h3075
    };
    let a3079 = {
        let (_, h3077) = {
            let (_, h3078) = v1;
            h3078
        };
        h3077
    };
    let a3080 = 0u64;
    if nat_eq(a3079, a3080) {
        let v4: u64 = f315(v0.clone(), v2)?;
        let a3081 = v4;
        let a3082 = 34u64;
        if nat_eq(a3081, a3082) {
            Ok(Some(({
                let a3083 = v2;
                let a3084 = 1u64;
                nat_add(a3083, a3084)?
            }, (v3, 1u64))))
        } else {
            let a3085 = v4;
            let a3086 = 92u64;
            if nat_eq(a3085, a3086) {
                f498(v0, v2, v3)
            } else {
                let a3087 = v4;
                let a3088 = 32u64;
                if if nat_lt(a3087, a3088) {
                    true
                } else {
                    let a3089 = v4;
                    let a3090 = 256u64;
                    nat_eq(a3089, a3090)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a3091 = v2;
                        let a3092 = 1u64;
                        nat_add(a3091, a3092)?
                    }, ({
                        let a3093 = v3;
                        let a3094 = f32(v0, v2, 1u64);
                        append_bytes(a3093, a3094)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f468(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f499(v0, v1)
}

pub fn f469(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f500(v0, v1)
}

pub fn f470(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3095 = v0.clone();
        length_bytes(a3095)
    };
    let a3096 = 0u64;
    let a3097 = v1;
    if nat_lt(a3096, a3097) {
        let a3098 = f501(v0.clone())?;
        let a3099 = v1;
        if nat_eq(a3098, a3099) {
            let a3100 = 1u64;
            let a3101 = v1;
            if nat_lt(a3100, a3101) {
                let m3104 = {
                    let a3102 = v0;
                    let a3103 = 0u64;
                    index_bytes(a3102, a3103)
                };
                match m3104 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3105 = v2;
                        let a3106 = 48u8;
                        Ok(!equal(a3105, a3106))
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

pub fn f471(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f502(v0, v1)
}

pub fn f472(v0: Bytes) -> R<u64> {
    let (h3107, _) = f256(Fn3::F503(v0.clone()), {
        let a3109 = {
            let a3108 = v0;
            length_bytes(a3108)
        };
        let a3110 = 1u64;
        nat_add(a3109, a3110)?
    }, 0u64)?;
    Ok(h3107)
}

pub fn f473(p3113: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m3112 = p3113;
    {
        let mut m3112 = m3112;
        loop {
            let step = {
                let v0 = m3112;
                let m3111 = v0;
                match m3111.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f37(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f87(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f87(v4, v3.clone())
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
                Ok(next) => { m3112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f474(v0: Adt19, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3114 = v0;
    match m3114 {
        Adt19::C0(_) => {
            Ok(v1)
        }
        Adt19::C1(_, _) => {
            Ok(v1)
        }
        Adt19::C2(_, _, _) => {
            Ok(v1)
        }
        Adt19::C3(_) => {
            Ok(v1)
        }
        Adt19::C4(r3117) => {
            let v2 = (*r3117).clone();
            f474(v2, v1)
        }
        Adt19::C5(r3118) => {
            let v3 = (*r3118).clone();
            f474(v3, v1)
        }
        Adt19::C6 => {
            Ok(v1)
        }
        Adt19::C7(r3119, r3120) => {
            let v4 = (*r3119).clone();
            let v5 = (*r3120).clone();
            f474(v4, f474(v5, v1)?)
        }
        Adt19::C8(v6, r3121, r3122) => {
            let v7 = (*r3121).clone();
            let v8 = (*r3122).clone();
            let v9: List<Bytes> = f474(v7.clone(), f474(v8, v1)?)?;
            if f37(v6, f8(Str::lit("$ref"))) {
                let m3115 = f247(Some(v7));
                match m3115 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m3116 = f504(v10)?;
                        match m3116 {
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

pub fn f475(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3123 = f148(v1.clone(), f8(Str::lit("enum")));
    match m3123 {
        None => {
            let m3125 = f148(v1.clone(), f8(Str::lit("oneOf")));
            match m3125 {
                None => {
                    if f509(v1.clone(), v0.clone())? {
                        let m3126 = f510(v0.clone(), f391(f148(v1.clone(), f8(Str::lit("properties")))), f152(f153(f148(v1, f8(Str::lit("required"))))), true)?;
                        match m3126 {
                            Ok(v7) => {
                                Ok(f507(f511(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f216({
                                    let (h3127, _) = v7;
                                    h3127
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h3128) = v7;
                                    h3128
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt25>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m3129 = f391(f148(v1.clone(), f8(Str::lit("properties"))));
                        match m3129.uncons() {
                            None => {
                                let m3132 = f512(v1, {
                                    let a3130 = v0.clone();
                                    let a3131 = f8(Str::lit("Value"));
                                    append_bytes(a3130, a3131)
                                })?;
                                match m3132 {
                                    Ok(v9) => {
                                        Ok(f507(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h3133, _) = v9;
                                            h3133
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3134) = v9;
                                            h3134
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m3135 = f510(v0.clone(), f391(f148(v1.clone(), f8(Str::lit("properties")))), f152(f153(f148(v1, f8(Str::lit("required"))))), false)?;
                                match m3135 {
                                    Ok(v13) => {
                                        Ok(f507(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m3137 = {
                                                let (h3136, _) = v13.clone();
                                                h3136
                                            };
                                            match m3137.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f216({
                                                        let (h3138, _) = v13;
                                                        h3138
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3139) = v13;
                                            h3139
                                        }))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f508(v0, f248(v5))
                }
            }
        }
        Some(v2) => {
            let m3124 = f505(f248(v2), List::<Bytes>::nil())?;
            match m3124 {
                Ok(v4) => {
                    Ok(f507(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f216(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f506(v3, v0))
                }
            }
        }
    }
}

pub fn f476(v0: Adt19) -> Adt19 {
    let v2: Adt19 = {
        let m3140 = f148(v0.clone(), f8(Str::lit("anyOf")));
        match m3140 {
            None => {
                v0
            }
            Some(v1) => {
                if f513(f248(v1.clone())) {
                    f514(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f515(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f516(f248(v1.clone()))
                    } {
                        v0
                    } else {
                        f517(f514(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt19 = {
        let m3141 = f148(v2.clone(), f8(Str::lit("oneOf")));
        match m3141 {
            None => {
                v2
            }
            Some(v3) => {
                if f513(f248(v3)) {
                    f514(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m3142 = f148(v4.clone(), f8(Str::lit("allOf")));
    match m3142 {
        None => {
            v4
        }
        Some(v5) => {
            if f518(f248(v5)) {
                f514(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f477(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3187 = {
            let a3163 = {
                let a3151 = {
                    let a3145 = {
                        let a3143 = v3;
                        let a3144 = 128u8;
                        fixed_u8::bit_and(a3143, a3144)
                    };
                    let a3146 = 128u8;
                    if equal(a3145, a3146) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3152 = {
                    let a3149 = {
                        let a3147 = v3;
                        let a3148 = 64u8;
                        fixed_u8::bit_and(a3147, a3148)
                    };
                    let a3150 = 64u8;
                    if equal(a3149, a3150) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3151, a3152)?
            };
            let a3164 = {
                let a3161 = {
                    let a3155 = {
                        let a3153 = v3;
                        let a3154 = 32u8;
                        fixed_u8::bit_and(a3153, a3154)
                    };
                    let a3156 = 32u8;
                    if equal(a3155, a3156) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3162 = {
                    let a3159 = {
                        let a3157 = v3;
                        let a3158 = 16u8;
                        fixed_u8::bit_and(a3157, a3158)
                    };
                    let a3160 = 16u8;
                    if equal(a3159, a3160) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3161, a3162)?
            };
            nat_add(a3163, a3164)?
        };
        let a3188 = {
            let a3185 = {
                let a3173 = {
                    let a3167 = {
                        let a3165 = v3;
                        let a3166 = 8u8;
                        fixed_u8::bit_and(a3165, a3166)
                    };
                    let a3168 = 8u8;
                    if equal(a3167, a3168) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3174 = {
                    let a3171 = {
                        let a3169 = v3;
                        let a3170 = 4u8;
                        fixed_u8::bit_and(a3169, a3170)
                    };
                    let a3172 = 4u8;
                    if equal(a3171, a3172) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3173, a3174)?
            };
            let a3186 = {
                let a3183 = {
                    let a3177 = {
                        let a3175 = v3;
                        let a3176 = 2u8;
                        fixed_u8::bit_and(a3175, a3176)
                    };
                    let a3178 = 2u8;
                    if equal(a3177, a3178) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3184 = {
                    let a3181 = {
                        let a3179 = v3;
                        let a3180 = 1u8;
                        fixed_u8::bit_and(a3179, a3180)
                    };
                    let a3182 = 1u8;
                    if equal(a3181, a3182) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3183, a3184)?
            };
            nat_add(a3185, a3186)?
        };
        nat_add(a3187, a3188)?
    };
    let a3189 = v4;
    let a3190 = 34u64;
    if nat_eq(a3189, a3190) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3191 = v4;
        let a3192 = 92u64;
        if nat_eq(a3191, a3192) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3193 = v4;
            let a3194 = 8u64;
            if nat_eq(a3193, a3194) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3195 = v4;
                let a3196 = 12u64;
                if nat_eq(a3195, a3196) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3197 = v4;
                    let a3198 = 10u64;
                    if nat_eq(a3197, a3198) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3199 = v4;
                        let a3200 = 13u64;
                        if nat_eq(a3199, a3200) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3201 = v4;
                            let a3202 = 9u64;
                            if nat_eq(a3201, a3202) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3203 = v4;
                                let a3204 = 32u64;
                                if nat_lt(a3203, a3204) {
                                    let a3205 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3206 = f519(v2)?;
                                    Ok(append_bytes(a3205, a3206))
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

pub fn f478(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f520(Fn21::F521, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f479(v0: Fn17, p3210: List<List<(Bytes, Bytes)>>, p3211: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3209 = (p3210, p3211);
    {
        let mut m3209 = m3209;
        loop {
            let step = {
                let (v1, v2) = m3209;
                let m3207 = v2;
                match m3207.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3208: Fn17 = v0.clone();
                            c3208.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f481(v0: Fn18, p3216: u64, p3217: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3215 = (p3216, p3217);
    {
        let mut m3215 = m3215;
        loop {
            let step = {
                let (v1, v2) = m3215;
                let m3212 = v1;
                if m3212 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3212 - 1;
                    let m3214 = {
                        let c3213: Fn18 = v0.clone();
                        c3213.apply(v2.clone())?
                    };
                    match m3214 {
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
                Ok(next) => { m3215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f482(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f522(v0)
}

pub fn f483(v0: Fn19, p3222: u64, p3223: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m3221 = (p3222, p3223);
    {
        let mut m3221 = m3221;
        loop {
            let step = {
                let (v1, v2) = m3221;
                let m3218 = v1;
                if m3218 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3218 - 1;
                    let m3220 = {
                        let c3219: Fn19 = v0.clone();
                        c3219.apply(v2.clone())?
                    };
                    match m3220 {
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
                Ok(next) => { m3221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f523(v0)
}

pub fn f485(v0: u64) -> u32 {
    let a3224 = v0;
    let a3225 = 32u64;
    if nat_lt(a3224, a3225) {
        let a3226 = v0;
        let a3227 = 16u64;
        if nat_lt(a3226, a3227) {
            let a3228 = v0;
            let a3229 = 8u64;
            if nat_lt(a3228, a3229) {
                let a3230 = v0;
                let a3231 = 4u64;
                if nat_lt(a3230, a3231) {
                    let a3232 = v0;
                    let a3233 = 2u64;
                    if nat_lt(a3232, a3233) {
                        let a3234 = v0;
                        let a3235 = 1u64;
                        if nat_lt(a3234, a3235) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3236 = v0;
                        let a3237 = 3u64;
                        if nat_lt(a3236, a3237) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3238 = v0;
                    let a3239 = 6u64;
                    if nat_lt(a3238, a3239) {
                        let a3240 = v0;
                        let a3241 = 5u64;
                        if nat_lt(a3240, a3241) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3242 = v0;
                        let a3243 = 7u64;
                        if nat_lt(a3242, a3243) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3244 = v0;
                let a3245 = 12u64;
                if nat_lt(a3244, a3245) {
                    let a3246 = v0;
                    let a3247 = 10u64;
                    if nat_lt(a3246, a3247) {
                        let a3248 = v0;
                        let a3249 = 9u64;
                        if nat_lt(a3248, a3249) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3250 = v0;
                        let a3251 = 11u64;
                        if nat_lt(a3250, a3251) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3252 = v0;
                    let a3253 = 14u64;
                    if nat_lt(a3252, a3253) {
                        let a3254 = v0;
                        let a3255 = 13u64;
                        if nat_lt(a3254, a3255) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3256 = v0;
                        let a3257 = 15u64;
                        if nat_lt(a3256, a3257) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3258 = v0;
            let a3259 = 24u64;
            if nat_lt(a3258, a3259) {
                let a3260 = v0;
                let a3261 = 20u64;
                if nat_lt(a3260, a3261) {
                    let a3262 = v0;
                    let a3263 = 18u64;
                    if nat_lt(a3262, a3263) {
                        let a3264 = v0;
                        let a3265 = 17u64;
                        if nat_lt(a3264, a3265) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3266 = v0;
                        let a3267 = 19u64;
                        if nat_lt(a3266, a3267) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3268 = v0;
                    let a3269 = 22u64;
                    if nat_lt(a3268, a3269) {
                        let a3270 = v0;
                        let a3271 = 21u64;
                        if nat_lt(a3270, a3271) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3272 = v0;
                        let a3273 = 23u64;
                        if nat_lt(a3272, a3273) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3274 = v0;
                let a3275 = 28u64;
                if nat_lt(a3274, a3275) {
                    let a3276 = v0;
                    let a3277 = 26u64;
                    if nat_lt(a3276, a3277) {
                        let a3278 = v0;
                        let a3279 = 25u64;
                        if nat_lt(a3278, a3279) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3280 = v0;
                        let a3281 = 27u64;
                        if nat_lt(a3280, a3281) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3282 = v0;
                    let a3283 = 30u64;
                    if nat_lt(a3282, a3283) {
                        let a3284 = v0;
                        let a3285 = 29u64;
                        if nat_lt(a3284, a3285) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3286 = v0;
                        let a3287 = 31u64;
                        if nat_lt(a3286, a3287) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3288 = v0;
        let a3289 = 48u64;
        if nat_lt(a3288, a3289) {
            let a3290 = v0;
            let a3291 = 40u64;
            if nat_lt(a3290, a3291) {
                let a3292 = v0;
                let a3293 = 36u64;
                if nat_lt(a3292, a3293) {
                    let a3294 = v0;
                    let a3295 = 34u64;
                    if nat_lt(a3294, a3295) {
                        let a3296 = v0;
                        let a3297 = 33u64;
                        if nat_lt(a3296, a3297) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3298 = v0;
                        let a3299 = 35u64;
                        if nat_lt(a3298, a3299) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3300 = v0;
                    let a3301 = 38u64;
                    if nat_lt(a3300, a3301) {
                        let a3302 = v0;
                        let a3303 = 37u64;
                        if nat_lt(a3302, a3303) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3304 = v0;
                        let a3305 = 39u64;
                        if nat_lt(a3304, a3305) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3306 = v0;
                let a3307 = 44u64;
                if nat_lt(a3306, a3307) {
                    let a3308 = v0;
                    let a3309 = 42u64;
                    if nat_lt(a3308, a3309) {
                        let a3310 = v0;
                        let a3311 = 41u64;
                        if nat_lt(a3310, a3311) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3312 = v0;
                        let a3313 = 43u64;
                        if nat_lt(a3312, a3313) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3314 = v0;
                    let a3315 = 46u64;
                    if nat_lt(a3314, a3315) {
                        let a3316 = v0;
                        let a3317 = 45u64;
                        if nat_lt(a3316, a3317) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3318 = v0;
                        let a3319 = 47u64;
                        if nat_lt(a3318, a3319) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3320 = v0;
            let a3321 = 56u64;
            if nat_lt(a3320, a3321) {
                let a3322 = v0;
                let a3323 = 52u64;
                if nat_lt(a3322, a3323) {
                    let a3324 = v0;
                    let a3325 = 50u64;
                    if nat_lt(a3324, a3325) {
                        let a3326 = v0;
                        let a3327 = 49u64;
                        if nat_lt(a3326, a3327) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3328 = v0;
                        let a3329 = 51u64;
                        if nat_lt(a3328, a3329) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3330 = v0;
                    let a3331 = 54u64;
                    if nat_lt(a3330, a3331) {
                        let a3332 = v0;
                        let a3333 = 53u64;
                        if nat_lt(a3332, a3333) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3334 = v0;
                        let a3335 = 55u64;
                        if nat_lt(a3334, a3335) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3336 = v0;
                let a3337 = 60u64;
                if nat_lt(a3336, a3337) {
                    let a3338 = v0;
                    let a3339 = 58u64;
                    if nat_lt(a3338, a3339) {
                        let a3340 = v0;
                        let a3341 = 57u64;
                        if nat_lt(a3340, a3341) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3342 = v0;
                        let a3343 = 59u64;
                        if nat_lt(a3342, a3343) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3344 = v0;
                    let a3345 = 62u64;
                    if nat_lt(a3344, a3345) {
                        let a3346 = v0;
                        let a3347 = 61u64;
                        if nat_lt(a3346, a3347) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3348 = v0;
                        let a3349 = 63u64;
                        if nat_lt(a3348, a3349) {
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

pub fn f486(v0: u32) -> u32 {
    let a3352 = {
        let a3350 = f524(v0, 6u32, 26u32);
        let a3351 = f524(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3350, a3351)
    };
    let a3353 = f524(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3352, a3353)
}

pub fn f487(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3359 = {
        let a3354 = v0;
        let a3355 = v1;
        fixed_u32::bit_and(a3354, a3355)
    };
    let a3360 = {
        let a3357 = {
            let a3356 = v0;
            fixed_u32::bit_not(a3356)
        };
        let a3358 = v2;
        fixed_u32::bit_and(a3357, a3358)
    };
    fixed_u32::bit_xor(a3359, a3360)
}

pub fn f488(v0: u32) -> u32 {
    let a3363 = {
        let a3361 = f524(v0, 2u32, 30u32);
        let a3362 = f524(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3361, a3362)
    };
    let a3364 = f524(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3363, a3364)
}

pub fn f489(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3373 = {
        let a3369 = {
            let a3365 = v0;
            let a3366 = v1;
            fixed_u32::bit_and(a3365, a3366)
        };
        let a3370 = {
            let a3367 = v0;
            let a3368 = v2;
            fixed_u32::bit_and(a3367, a3368)
        };
        fixed_u32::bit_xor(a3369, a3370)
    };
    let a3374 = {
        let a3371 = v1;
        let a3372 = v2;
        fixed_u32::bit_and(a3371, a3372)
    };
    fixed_u32::bit_xor(a3373, a3374)
}

pub fn f490(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3375, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3375
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3376, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3376
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3377, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3377
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3378, _, _, _, _, _, _, _, _, _, _, _) => {
            h3378
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3379, _, _, _, _, _, _, _, _, _, _) => {
            h3379
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3380, _, _, _, _, _, _, _, _, _) => {
            h3380
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3381, _, _, _, _, _, _, _, _) => {
            h3381
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3382, _, _, _, _, _, _, _) => {
            h3382
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3383, _, _, _, _, _, _) => {
            h3383
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3384, _, _, _, _, _) => {
            h3384
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3385, _, _, _, _) => {
            h3385
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3386, _, _, _) => {
            h3386
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3387, _, _) => {
            h3387
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3388, _) => {
            h3388
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3389) => {
            h3389
        }
    }, f417(f525(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3390, _) => {
            h3390
        }
    }), f417(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3391, _, _, _, _, _, _) => {
            h3391
        }
    }, f417(f526(match v0.clone() {
        Adt26::C0(_, h3392, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3392
        }
    }), match v0 {
        Adt26::C0(h3393, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3393
        }
    }))))
}

pub fn f491(v0: Fn20, p3397: u64, p3398: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3396 = (p3397, p3398);
    {
        let mut m3396 = m3396;
        loop {
            let step = {
                let (v1, v2) = m3396;
                let m3394 = v1;
                if m3394 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3394 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3395: Fn20 = v0.clone();
                        c3395.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3399, _) = v0;
        h3399
    };
    let v2: u32 = {
        let (h3400, _) = {
            let (_, h3401) = v0;
            h3401
        };
        h3400
    };
    let v3: u32 = {
        let (_, h3402) = {
            let (_, h3403) = v0;
            h3403
        };
        h3402
    };
    ({
        let a3404 = v1;
        let a3405 = 2u64;
        let a3406 = 0u64;
        nat_quot(a3404, a3405, a3406)
    }, (f493(v2, 1u32), {
        let a3410 = {
            let a3407 = v1;
            let a3408 = 2u64;
            let a3409 = 0u64;
            nat_rem(a3407, a3408, a3409)
        };
        let a3411 = 1u64;
        if nat_eq(a3410, a3411) {
            let a3412 = v3;
            let a3413 = v2;
            fixed_u32::bit_or(a3412, a3413)
        } else {
            v3
        }
    }))
}

pub fn f493(v0: u32, v1: u32) -> u32 {
    let m3416 = {
        let a3414 = v0;
        let a3415 = v1;
        fixed_u32::shift_left(a3414, a3415)
    };
    match m3416 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f494(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f228(v0, {
        let (h3417, _) = v2;
        h3417
    })?;
    let a3419 = 1114111u64;
    let a3420 = {
        let (_, h3418) = v2;
        h3418
    };
    if nat_lt(a3419, a3420) {
        Ok(None::<(u64, u64)>)
    } else {
        let a3421 = 48u64;
        let a3422 = v3;
        if if nat_le(a3421, a3422) {
            let a3423 = v3;
            let a3424 = 57u64;
            nat_le(a3423, a3424)
        } else {
            false
        } {
            Ok(Some(({
                let a3426 = {
                    let (h3425, _) = v2;
                    h3425
                };
                let a3427 = 1u64;
                nat_add(a3426, a3427)?
            }, {
                let a3433 = {
                    let a3429 = {
                        let (_, h3428) = v2;
                        h3428
                    };
                    let a3430 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3429, a3430)?
                };
                let a3434 = {
                    let a3431 = v3;
                    let a3432 = 48u64;
                    nat_sub(a3431, a3432)
                };
                nat_add(a3433, a3434)?
            })))
        } else {
            if if v1 {
                let a3435 = 97u64;
                let a3436 = v3;
                if nat_le(a3435, a3436) {
                    let a3437 = v3;
                    let a3438 = 102u64;
                    nat_le(a3437, a3438)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some(({
                    let a3440 = {
                        let (h3439, _) = v2;
                        h3439
                    };
                    let a3441 = 1u64;
                    nat_add(a3440, a3441)?
                }, {
                    let a3447 = {
                        let a3443 = {
                            let (_, h3442) = v2;
                            h3442
                        };
                        let a3444 = 16u64;
                        nat_mul(a3443, a3444)?
                    };
                    let a3448 = {
                        let a3445 = v3;
                        let a3446 = 87u64;
                        nat_sub(a3445, a3446)
                    };
                    nat_add(a3447, a3448)?
                })))
            } else {
                if if v1 {
                    let a3449 = 65u64;
                    let a3450 = v3;
                    if nat_le(a3449, a3450) {
                        let a3451 = v3;
                        let a3452 = 70u64;
                        nat_le(a3451, a3452)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(({
                        let a3454 = {
                            let (h3453, _) = v2;
                            h3453
                        };
                        let a3455 = 1u64;
                        nat_add(a3454, a3455)?
                    }, {
                        let a3461 = {
                            let a3457 = {
                                let (_, h3456) = v2;
                                h3456
                            };
                            let a3458 = 16u64;
                            nat_mul(a3457, a3458)?
                        };
                        let a3462 = {
                            let a3459 = v3;
                            let a3460 = 55u64;
                            nat_sub(a3459, a3460)
                        };
                        nat_add(a3461, a3462)?
                    })))
                } else {
                    Ok(None::<(u64, u64)>)
                }
            }
        }
    }
}

pub fn f495(p3469: List<Bytes>, p3470: u64) -> Bytes {
    let m3468 = (p3469, p3470);
    {
        let mut m3468 = m3468;
        loop {
            let step = {
                let (v0, v1) = m3468;
                let m3463 = v0;
                match m3463.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a3464 = v1;
                        let a3465 = 0u64;
                        if nat_eq(a3464, a3465) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a3466 = v1;
                                let a3467 = 1u64;
                                nat_sub(a3466, a3467)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f496(v0: u64) -> List<Bytes> {
    let a3471 = v0;
    let a3472 = 0u64;
    if nat_eq(a3471, a3472) {
        f527()
    } else {
        let a3473 = v0;
        let a3474 = 1u64;
        if nat_eq(a3473, a3474) {
            f528()
        } else {
            let a3475 = v0;
            let a3476 = 2u64;
            if nat_eq(a3475, a3476) {
                f529()
            } else {
                let a3477 = v0;
                let a3478 = 3u64;
                if nat_eq(a3477, a3478) {
                    f530()
                } else {
                    let a3479 = v0;
                    let a3480 = 4u64;
                    if nat_eq(a3479, a3480) {
                        f531()
                    } else {
                        let a3481 = v0;
                        let a3482 = 5u64;
                        if nat_eq(a3481, a3482) {
                            f532()
                        } else {
                            let a3483 = v0;
                            let a3484 = 6u64;
                            if nat_eq(a3483, a3484) {
                                f533()
                            } else {
                                let a3485 = v0;
                                let a3486 = 7u64;
                                if nat_eq(a3485, a3486) {
                                    f534()
                                } else {
                                    let a3487 = v0;
                                    let a3488 = 8u64;
                                    if nat_eq(a3487, a3488) {
                                        f535()
                                    } else {
                                        let a3489 = v0;
                                        let a3490 = 9u64;
                                        if nat_eq(a3489, a3490) {
                                            f536()
                                        } else {
                                            let a3491 = v0;
                                            let a3492 = 10u64;
                                            if nat_eq(a3491, a3492) {
                                                f537()
                                            } else {
                                                let a3493 = v0;
                                                let a3494 = 11u64;
                                                if nat_eq(a3493, a3494) {
                                                    f538()
                                                } else {
                                                    let a3495 = v0;
                                                    let a3496 = 12u64;
                                                    if nat_eq(a3495, a3496) {
                                                        f539()
                                                    } else {
                                                        let a3497 = v0;
                                                        let a3498 = 13u64;
                                                        if nat_eq(a3497, a3498) {
                                                            f540()
                                                        } else {
                                                            let a3499 = v0;
                                                            let a3500 = 14u64;
                                                            if nat_eq(a3499, a3500) {
                                                                f541()
                                                            } else {
                                                                f542()
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

pub fn f497(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        let a3501 = v0;
        let a3502 = v2;
        if if if nat_eq(a3501, a3502) {
            true
        } else {
            let a3503 = v0;
            let a3504 = 60u64;
            nat_eq(a3503, a3504)
        } {
            true
        } else {
            let a3505 = v0;
            let a3506 = 38u64;
            if nat_eq(a3505, a3506) {
                true
            } else {
                let a3507 = v0;
                let a3508 = 256u64;
                nat_eq(a3507, a3508)
            }
        } {
            true
        } else {
            let a3509 = v0;
            let a3510 = 9u64;
            if nat_eq(a3509, a3510) {
                true
            } else {
                let a3511 = v0;
                let a3512 = 10u64;
                nat_eq(a3511, a3512)
            }
        }
    } else {
        let a3513 = v0;
        let a3514 = 60u64;
        if if nat_eq(a3513, a3514) {
            true
        } else {
            let a3515 = v0;
            let a3516 = 38u64;
            nat_eq(a3515, a3516)
        } {
            true
        } else {
            let a3517 = v0;
            let a3518 = 93u64;
            if nat_eq(a3517, a3518) {
                true
            } else {
                let a3519 = v0;
                let a3520 = 256u64;
                nat_eq(a3519, a3520)
            }
        }
    }
}

pub fn f498(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f315(v0.clone(), {
        let a3521 = v1;
        let a3522 = 1u64;
        nat_add(a3521, a3522)?
    })?;
    let v4: Bytes = {
        let a3523 = v3;
        let a3524 = 117u64;
        if nat_eq(a3523, a3524) {
            f543(v0, v1)?
        } else {
            f544(v3)
        }
    };
    let a3525 = v4.clone();
    let a3526 = Bytes::lit(&[]);
    if equal(a3525, a3526) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a3527 = v3;
            let a3528 = 117u64;
            if nat_eq(a3527, a3528) {
                let a3529 = v1;
                let a3530 = 6u64;
                nat_add(a3529, a3530)?
            } else {
                let a3531 = v1;
                let a3532 = 2u64;
                nat_add(a3531, a3532)?
            }
        }, ({
            let a3533 = v2;
            let a3534 = v4;
            append_bytes(a3533, a3534)
        }, 0u64))))
    }
}

pub fn f499(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f315(v0, v1)?;
    let a3535 = 47u64;
    let a3536 = v2;
    if if nat_lt(a3535, a3536) {
        let a3537 = v2;
        let a3538 = 58u64;
        nat_lt(a3537, a3538)
    } else {
        false
    } {
        Ok(Some({
            let a3539 = v1;
            let a3540 = 1u64;
            nat_add(a3539, a3540)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f500(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3541 = f315(v0, v1)?;
    let a3542 = 48u64;
    if nat_eq(a3541, a3542) {
        Ok(Some({
            let a3543 = v1;
            let a3544 = 1u64;
            nat_add(a3543, a3544)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f501(v0: Bytes) -> R<u64> {
    let (h3545, _) = f256(Fn3::F545(v0.clone()), {
        let a3546 = v0;
        length_bytes(a3546)
    }, 0u64)?;
    Ok(h3545)
}

pub fn f502(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3550 = {
        let a3548 = v0;
        let a3549 = {
            let (h3547, _) = v1;
            h3547
        };
        index_bytes(a3548, a3549)
    };
    match m3550 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3552 = {
                    let (h3551, _) = v1;
                    h3551
                };
                let a3553 = 1u64;
                nat_add(a3552, a3553)?
            }, {
                let a3557 = {
                    let a3555 = {
                        let (_, h3554) = v1;
                        h3554
                    };
                    let a3556 = 10u64;
                    nat_mul(a3555, a3556)?
                };
                let a3558 = f546(v2)?;
                nat_add(a3557, a3558)?
            })))
        }
    }
}

pub fn f503(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f547(v0, v1)
}

pub fn f504(v0: Bytes) -> R<Option<Bytes>> {
    if f206(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f243(f32(v0.clone(), 8u64, {
            let a3560 = {
                let a3559 = v0;
                length_bytes(a3559)
            };
            let a3561 = 8u64;
            nat_sub(a3560, a3561)
        }))?))
    } else {
        if f206(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f243(f32(v0.clone(), 14u64, {
                let a3563 = {
                    let a3562 = v0;
                    length_bytes(a3562)
                };
                let a3564 = 14u64;
                nat_sub(a3563, a3564)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f505(v0: List<Adt19>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f548(f549(f152(v0.clone()))?, v0, v1)
}

pub fn f506(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt25>), Bytes> {
    Err::<(Bytes, List<Adt25>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f507(v0: Bytes, v1: List<Adt25>) -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((v0, v1))
}

pub fn f508(v0: Bytes, v1: List<Adt19>) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3565 = v1.clone();
    match m3565.uncons() {
        None => {
            Ok(f506(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3566 = f550(f391(f148(v2, f8(Str::lit("properties")))), v1.clone());
            match m3566.uncons() {
                None => {
                    let m3567 = f551(f549(f552(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3567 {
                        Ok(v5) => {
                            Ok(f507(f511(v0, {
                                let (h3568, _) = v5;
                                h3568
                            }), {
                                let (_, h3569) = v5;
                                h3569
                            }))
                        }
                        Err(v4) => {
                            Ok(f506(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3570 = f553(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3570 {
                        Ok(v9) => {
                            Ok(f507(f511(v0, {
                                let (h3571, _) = v9;
                                h3571
                            }), {
                                let (_, h3572) = v9;
                                h3572
                            }))
                        }
                        Err(v8) => {
                            Ok(f506(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f509(v0: Adt19, v1: Bytes) -> R<bool> {
    let m3573 = v0;
    match m3573 {
        Adt19::C0(_) => {
            Ok(false)
        }
        Adt19::C1(_, _) => {
            Ok(false)
        }
        Adt19::C2(_, _, _) => {
            Ok(false)
        }
        Adt19::C3(_) => {
            Ok(false)
        }
        Adt19::C4(r3574) => {
            let v2 = (*r3574).clone();
            f509(v2, v1)
        }
        Adt19::C5(r3575) => {
            let v3 = (*r3575).clone();
            f509(v3, v1)
        }
        Adt19::C6 => {
            Ok(false)
        }
        Adt19::C7(r3576, r3577) => {
            let v4 = (*r3576).clone();
            let v5 = (*r3577).clone();
            if f509(v4, v1.clone())? {
                Ok(true)
            } else {
                f509(v5, v1)
            }
        }
        Adt19::C8(v6, r3578, r3579) => {
            let v7 = (*r3578).clone();
            let v8 = (*r3579).clone();
            if if f37(v6, f8(Str::lit("$ref"))) {
                f554(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f509(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f509(v8, v1)
                }
            }
        }
    }
}

pub fn f510(v0: Bytes, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f555(f549(f556(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f511(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f216(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f512(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3580 = f557(v0.clone());
    match m3580 {
        None => {
            if f37(f151(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3581 = f148(v0, f8(Str::lit("items")));
                match m3581 {
                    None => {
                        Ok(f558(f8(Str::lit("List")), f560()))
                    }
                    Some(v3) => {
                        Ok(f558(f8(Str::lit("List")), f559(v3, {
                            let a3582 = v1;
                            let a3583 = f8(Str::lit("Item"));
                            append_bytes(a3582, a3583)
                        })?))
                    }
                }
            } else {
                f559(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f558(f8(Str::lit("Option")), f559(v2, v1)?))
        }
    }
}

pub fn f513(p3586: List<Adt19>) -> bool {
    let m3585 = p3586;
    {
        let mut m3585 = m3585;
        loop {
            let step = {
                let v0 = m3585;
                let m3584 = v0;
                match m3584.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f561(f436(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt19>, bool>(v2)
                        } else {
                            Err::<List<Adt19>, bool>(false)
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

pub fn f514(v0: Adt19, v1: Bytes) -> Adt19 {
    let m3587 = v0;
    match m3587 {
        Adt19::C0(v6) => {
            Adt19::C0(v6)
        }
        Adt19::C1(v10, v11) => {
            Adt19::C1(v10, v11)
        }
        Adt19::C2(v7, v8, v9) => {
            Adt19::C2(v7, v8, v9)
        }
        Adt19::C3(v12) => {
            Adt19::C3(v12)
        }
        Adt19::C4(r3588) => {
            let v13 = (*r3588).clone();
            Adt19::C4(std::rc::Rc::new(v13))
        }
        Adt19::C5(r3589) => {
            let v5 = (*r3589).clone();
            Adt19::C5(std::rc::Rc::new(f514(v5, v1)))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r3590, r3591) => {
            let v14 = (*r3590).clone();
            let v15 = (*r3591).clone();
            Adt19::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt19::C8(v2, r3592, r3593) => {
            let v3 = (*r3592).clone();
            let v4 = (*r3593).clone();
            if f37(v2.clone(), v1.clone()) {
                f514(v4, v1)
            } else {
                Adt19::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f514(v4, v1)))
            }
        }
    }
}

pub fn f515(v0: Adt19, v1: Bytes) -> bool {
    let m3594 = f148(v0, v1);
    match m3594 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f516(v0: List<Adt19>) -> bool {
    let m3595 = v0.clone();
    match m3595.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3596 = f550(f391(f148(v1, f8(Str::lit("properties")))), v0.clone());
            match m3596.uncons() {
                None => {
                    f563(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f562(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f517(v0: Adt19, v1: Bytes, v2: Adt19) -> Adt19 {
    let m3597 = v0;
    match m3597 {
        Adt19::C0(v7) => {
            Adt19::C0(v7)
        }
        Adt19::C1(v11, v12) => {
            Adt19::C1(v11, v12)
        }
        Adt19::C2(v8, v9, v10) => {
            Adt19::C2(v8, v9, v10)
        }
        Adt19::C3(v13) => {
            Adt19::C3(v13)
        }
        Adt19::C4(r3598) => {
            let v14 = (*r3598).clone();
            Adt19::C4(std::rc::Rc::new(v14))
        }
        Adt19::C5(r3599) => {
            let v3 = (*r3599).clone();
            Adt19::C5(std::rc::Rc::new(Adt19::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r3600, r3601) => {
            let v15 = (*r3600).clone();
            let v16 = (*r3601).clone();
            Adt19::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt19::C8(v4, r3602, r3603) => {
            let v5 = (*r3602).clone();
            let v6 = (*r3603).clone();
            Adt19::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f518(p3607: List<Adt19>) -> bool {
    let m3606 = p3607;
    {
        let mut m3606 = m3606;
        loop {
            let step = {
                let v0 = m3606;
                let m3604 = v0;
                match m3604.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f561(f436(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f513(f248({
                                let m3605 = f148(v1, f8(Str::lit("anyOf")));
                                match m3605 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v3) => {
                                        v3
                                    }
                                }
                            })) {
                                Ok::<List<Adt19>, bool>(v2)
                            } else {
                                Err::<List<Adt19>, bool>(false)
                            }
                        } else {
                            Err::<List<Adt19>, bool>(false)
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

pub fn f519(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3652 = {
            let a3628 = {
                let a3616 = {
                    let a3610 = {
                        let a3608 = v1;
                        let a3609 = 128u8;
                        fixed_u8::bit_and(a3608, a3609)
                    };
                    let a3611 = 128u8;
                    if equal(a3610, a3611) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3617 = {
                    let a3614 = {
                        let a3612 = v1;
                        let a3613 = 64u8;
                        fixed_u8::bit_and(a3612, a3613)
                    };
                    let a3615 = 64u8;
                    if equal(a3614, a3615) {
                        64u64
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
                        let a3619 = 32u8;
                        fixed_u8::bit_and(a3618, a3619)
                    };
                    let a3621 = 32u8;
                    if equal(a3620, a3621) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3627 = {
                    let a3624 = {
                        let a3622 = v1;
                        let a3623 = 16u8;
                        fixed_u8::bit_and(a3622, a3623)
                    };
                    let a3625 = 16u8;
                    if equal(a3624, a3625) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3626, a3627)?
            };
            nat_add(a3628, a3629)?
        };
        let a3653 = {
            let a3650 = {
                let a3638 = {
                    let a3632 = {
                        let a3630 = v1;
                        let a3631 = 8u8;
                        fixed_u8::bit_and(a3630, a3631)
                    };
                    let a3633 = 8u8;
                    if equal(a3632, a3633) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3639 = {
                    let a3636 = {
                        let a3634 = v1;
                        let a3635 = 4u8;
                        fixed_u8::bit_and(a3634, a3635)
                    };
                    let a3637 = 4u8;
                    if equal(a3636, a3637) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3638, a3639)?
            };
            let a3651 = {
                let a3648 = {
                    let a3642 = {
                        let a3640 = v1;
                        let a3641 = 2u8;
                        fixed_u8::bit_and(a3640, a3641)
                    };
                    let a3643 = 2u8;
                    if equal(a3642, a3643) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3649 = {
                    let a3646 = {
                        let a3644 = v1;
                        let a3645 = 1u8;
                        fixed_u8::bit_and(a3644, a3645)
                    };
                    let a3647 = 1u8;
                    if equal(a3646, a3647) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3648, a3649)?
            };
            nat_add(a3650, a3651)?
        };
        nat_add(a3652, a3653)?
    };
    let a3660 = f319(f564(), {
        let a3654 = v2;
        let a3655 = 16u64;
        let a3656 = 0u64;
        nat_quot(a3654, a3655, a3656)
    });
    let a3661 = f319(f564(), {
        let a3657 = v2;
        let a3658 = 16u64;
        let a3659 = 0u64;
        nat_rem(a3657, a3658, a3659)
    });
    Ok(append_bytes(a3660, a3661))
}

pub fn f520(v0: Fn21, p3665: List<List<(Bytes, Bytes)>>, p3666: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3664 = (p3665, p3666);
    {
        let mut m3664 = m3664;
        loop {
            let step = {
                let (v1, v2) = m3664;
                let m3662 = v2;
                match m3662.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3663: Fn21 = v0.clone();
                            c3663.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f522(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3668 = {
        let a3667 = v0.clone();
        length_list(a3667)
    };
    let a3669 = 1u64;
    if nat_le(a3668, a3669) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f565(v0)?))
    }
}

pub fn f523(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3670, _) = v0;
        h3670
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3671) = v0;
        h3671
    };
    let m3672 = v1;
    match m3672.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3673 = v4;
            match m3673.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f566(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f524(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3674 = f365(v0, v1);
    let a3675 = f493(v0, v2);
    fixed_u32::bit_or(a3674, a3675)
}

pub fn f525(v0: u32) -> u32 {
    let a3678 = {
        let a3676 = f524(v0, 17u32, 15u32);
        let a3677 = f524(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3676, a3677)
    };
    let a3679 = f365(v0, 10u32);
    fixed_u32::bit_xor(a3678, a3679)
}

pub fn f526(v0: u32) -> u32 {
    let a3682 = {
        let a3680 = f524(v0, 7u32, 25u32);
        let a3681 = f524(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3680, a3681)
    };
    let a3683 = f365(v0, 3u32);
    fixed_u32::bit_xor(a3682, a3683)
}

pub fn f527() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x00]), List::cons(Bytes::lit(&[0x01]), List::cons(Bytes::lit(&[0x02]), List::cons(Bytes::lit(&[0x03]), List::cons(Bytes::lit(&[0x04]), List::cons(Bytes::lit(&[0x05]), List::cons(Bytes::lit(&[0x06]), List::cons(Bytes::lit(&[0x07]), List::cons(Bytes::lit(&[0x08]), List::cons(Bytes::lit(&[0x09]), List::cons(Bytes::lit(&[0x0a]), List::cons(Bytes::lit(&[0x0b]), List::cons(Bytes::lit(&[0x0c]), List::cons(Bytes::lit(&[0x0d]), List::cons(Bytes::lit(&[0x0e]), List::cons(Bytes::lit(&[0x0f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f528() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x10]), List::cons(Bytes::lit(&[0x11]), List::cons(Bytes::lit(&[0x12]), List::cons(Bytes::lit(&[0x13]), List::cons(Bytes::lit(&[0x14]), List::cons(Bytes::lit(&[0x15]), List::cons(Bytes::lit(&[0x16]), List::cons(Bytes::lit(&[0x17]), List::cons(Bytes::lit(&[0x18]), List::cons(Bytes::lit(&[0x19]), List::cons(Bytes::lit(&[0x1a]), List::cons(Bytes::lit(&[0x1b]), List::cons(Bytes::lit(&[0x1c]), List::cons(Bytes::lit(&[0x1d]), List::cons(Bytes::lit(&[0x1e]), List::cons(Bytes::lit(&[0x1f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f529() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x20]), List::cons(Bytes::lit(&[0x21]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x27]), List::cons(Bytes::lit(&[0x28]), List::cons(Bytes::lit(&[0x29]), List::cons(Bytes::lit(&[0x2a]), List::cons(Bytes::lit(&[0x2b]), List::cons(Bytes::lit(&[0x2c]), List::cons(Bytes::lit(&[0x2d]), List::cons(Bytes::lit(&[0x2e]), List::cons(Bytes::lit(&[0x2f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f530() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x30]), List::cons(Bytes::lit(&[0x31]), List::cons(Bytes::lit(&[0x32]), List::cons(Bytes::lit(&[0x33]), List::cons(Bytes::lit(&[0x34]), List::cons(Bytes::lit(&[0x35]), List::cons(Bytes::lit(&[0x36]), List::cons(Bytes::lit(&[0x37]), List::cons(Bytes::lit(&[0x38]), List::cons(Bytes::lit(&[0x39]), List::cons(Bytes::lit(&[0x3a]), List::cons(Bytes::lit(&[0x3b]), List::cons(Bytes::lit(&[0x3c]), List::cons(Bytes::lit(&[0x3d]), List::cons(Bytes::lit(&[0x3e]), List::cons(Bytes::lit(&[0x3f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f531() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x40]), List::cons(Bytes::lit(&[0x41]), List::cons(Bytes::lit(&[0x42]), List::cons(Bytes::lit(&[0x43]), List::cons(Bytes::lit(&[0x44]), List::cons(Bytes::lit(&[0x45]), List::cons(Bytes::lit(&[0x46]), List::cons(Bytes::lit(&[0x47]), List::cons(Bytes::lit(&[0x48]), List::cons(Bytes::lit(&[0x49]), List::cons(Bytes::lit(&[0x4a]), List::cons(Bytes::lit(&[0x4b]), List::cons(Bytes::lit(&[0x4c]), List::cons(Bytes::lit(&[0x4d]), List::cons(Bytes::lit(&[0x4e]), List::cons(Bytes::lit(&[0x4f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f532() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x50]), List::cons(Bytes::lit(&[0x51]), List::cons(Bytes::lit(&[0x52]), List::cons(Bytes::lit(&[0x53]), List::cons(Bytes::lit(&[0x54]), List::cons(Bytes::lit(&[0x55]), List::cons(Bytes::lit(&[0x56]), List::cons(Bytes::lit(&[0x57]), List::cons(Bytes::lit(&[0x58]), List::cons(Bytes::lit(&[0x59]), List::cons(Bytes::lit(&[0x5a]), List::cons(Bytes::lit(&[0x5b]), List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x5d]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f533() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x60]), List::cons(Bytes::lit(&[0x61]), List::cons(Bytes::lit(&[0x62]), List::cons(Bytes::lit(&[0x63]), List::cons(Bytes::lit(&[0x64]), List::cons(Bytes::lit(&[0x65]), List::cons(Bytes::lit(&[0x66]), List::cons(Bytes::lit(&[0x67]), List::cons(Bytes::lit(&[0x68]), List::cons(Bytes::lit(&[0x69]), List::cons(Bytes::lit(&[0x6a]), List::cons(Bytes::lit(&[0x6b]), List::cons(Bytes::lit(&[0x6c]), List::cons(Bytes::lit(&[0x6d]), List::cons(Bytes::lit(&[0x6e]), List::cons(Bytes::lit(&[0x6f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f534() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x70]), List::cons(Bytes::lit(&[0x71]), List::cons(Bytes::lit(&[0x72]), List::cons(Bytes::lit(&[0x73]), List::cons(Bytes::lit(&[0x74]), List::cons(Bytes::lit(&[0x75]), List::cons(Bytes::lit(&[0x76]), List::cons(Bytes::lit(&[0x77]), List::cons(Bytes::lit(&[0x78]), List::cons(Bytes::lit(&[0x79]), List::cons(Bytes::lit(&[0x7a]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7c]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x7f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f535() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x80]), List::cons(Bytes::lit(&[0x81]), List::cons(Bytes::lit(&[0x82]), List::cons(Bytes::lit(&[0x83]), List::cons(Bytes::lit(&[0x84]), List::cons(Bytes::lit(&[0x85]), List::cons(Bytes::lit(&[0x86]), List::cons(Bytes::lit(&[0x87]), List::cons(Bytes::lit(&[0x88]), List::cons(Bytes::lit(&[0x89]), List::cons(Bytes::lit(&[0x8a]), List::cons(Bytes::lit(&[0x8b]), List::cons(Bytes::lit(&[0x8c]), List::cons(Bytes::lit(&[0x8d]), List::cons(Bytes::lit(&[0x8e]), List::cons(Bytes::lit(&[0x8f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f536() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x90]), List::cons(Bytes::lit(&[0x91]), List::cons(Bytes::lit(&[0x92]), List::cons(Bytes::lit(&[0x93]), List::cons(Bytes::lit(&[0x94]), List::cons(Bytes::lit(&[0x95]), List::cons(Bytes::lit(&[0x96]), List::cons(Bytes::lit(&[0x97]), List::cons(Bytes::lit(&[0x98]), List::cons(Bytes::lit(&[0x99]), List::cons(Bytes::lit(&[0x9a]), List::cons(Bytes::lit(&[0x9b]), List::cons(Bytes::lit(&[0x9c]), List::cons(Bytes::lit(&[0x9d]), List::cons(Bytes::lit(&[0x9e]), List::cons(Bytes::lit(&[0x9f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f537() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xa0]), List::cons(Bytes::lit(&[0xa1]), List::cons(Bytes::lit(&[0xa2]), List::cons(Bytes::lit(&[0xa3]), List::cons(Bytes::lit(&[0xa4]), List::cons(Bytes::lit(&[0xa5]), List::cons(Bytes::lit(&[0xa6]), List::cons(Bytes::lit(&[0xa7]), List::cons(Bytes::lit(&[0xa8]), List::cons(Bytes::lit(&[0xa9]), List::cons(Bytes::lit(&[0xaa]), List::cons(Bytes::lit(&[0xab]), List::cons(Bytes::lit(&[0xac]), List::cons(Bytes::lit(&[0xad]), List::cons(Bytes::lit(&[0xae]), List::cons(Bytes::lit(&[0xaf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f538() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xb0]), List::cons(Bytes::lit(&[0xb1]), List::cons(Bytes::lit(&[0xb2]), List::cons(Bytes::lit(&[0xb3]), List::cons(Bytes::lit(&[0xb4]), List::cons(Bytes::lit(&[0xb5]), List::cons(Bytes::lit(&[0xb6]), List::cons(Bytes::lit(&[0xb7]), List::cons(Bytes::lit(&[0xb8]), List::cons(Bytes::lit(&[0xb9]), List::cons(Bytes::lit(&[0xba]), List::cons(Bytes::lit(&[0xbb]), List::cons(Bytes::lit(&[0xbc]), List::cons(Bytes::lit(&[0xbd]), List::cons(Bytes::lit(&[0xbe]), List::cons(Bytes::lit(&[0xbf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f539() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xc0]), List::cons(Bytes::lit(&[0xc1]), List::cons(Bytes::lit(&[0xc2]), List::cons(Bytes::lit(&[0xc3]), List::cons(Bytes::lit(&[0xc4]), List::cons(Bytes::lit(&[0xc5]), List::cons(Bytes::lit(&[0xc6]), List::cons(Bytes::lit(&[0xc7]), List::cons(Bytes::lit(&[0xc8]), List::cons(Bytes::lit(&[0xc9]), List::cons(Bytes::lit(&[0xca]), List::cons(Bytes::lit(&[0xcb]), List::cons(Bytes::lit(&[0xcc]), List::cons(Bytes::lit(&[0xcd]), List::cons(Bytes::lit(&[0xce]), List::cons(Bytes::lit(&[0xcf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f540() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xd0]), List::cons(Bytes::lit(&[0xd1]), List::cons(Bytes::lit(&[0xd2]), List::cons(Bytes::lit(&[0xd3]), List::cons(Bytes::lit(&[0xd4]), List::cons(Bytes::lit(&[0xd5]), List::cons(Bytes::lit(&[0xd6]), List::cons(Bytes::lit(&[0xd7]), List::cons(Bytes::lit(&[0xd8]), List::cons(Bytes::lit(&[0xd9]), List::cons(Bytes::lit(&[0xda]), List::cons(Bytes::lit(&[0xdb]), List::cons(Bytes::lit(&[0xdc]), List::cons(Bytes::lit(&[0xdd]), List::cons(Bytes::lit(&[0xde]), List::cons(Bytes::lit(&[0xdf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f541() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xe0]), List::cons(Bytes::lit(&[0xe1]), List::cons(Bytes::lit(&[0xe2]), List::cons(Bytes::lit(&[0xe3]), List::cons(Bytes::lit(&[0xe4]), List::cons(Bytes::lit(&[0xe5]), List::cons(Bytes::lit(&[0xe6]), List::cons(Bytes::lit(&[0xe7]), List::cons(Bytes::lit(&[0xe8]), List::cons(Bytes::lit(&[0xe9]), List::cons(Bytes::lit(&[0xea]), List::cons(Bytes::lit(&[0xeb]), List::cons(Bytes::lit(&[0xec]), List::cons(Bytes::lit(&[0xed]), List::cons(Bytes::lit(&[0xee]), List::cons(Bytes::lit(&[0xef]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f542() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xf0]), List::cons(Bytes::lit(&[0xf1]), List::cons(Bytes::lit(&[0xf2]), List::cons(Bytes::lit(&[0xf3]), List::cons(Bytes::lit(&[0xf4]), List::cons(Bytes::lit(&[0xf5]), List::cons(Bytes::lit(&[0xf6]), List::cons(Bytes::lit(&[0xf7]), List::cons(Bytes::lit(&[0xf8]), List::cons(Bytes::lit(&[0xf9]), List::cons(Bytes::lit(&[0xfa]), List::cons(Bytes::lit(&[0xfb]), List::cons(Bytes::lit(&[0xfc]), List::cons(Bytes::lit(&[0xfd]), List::cons(Bytes::lit(&[0xfe]), List::cons(Bytes::lit(&[0xff]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f543(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3686 = f32(v0.clone(), {
        let a3684 = v1;
        let a3685 = 2u64;
        nat_add(a3684, a3685)?
    }, 2u64);
    let a3687 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3686, a3687) {
        let m3690 = f567(f32(v0, {
            let a3688 = v1;
            let a3689 = 4u64;
            nat_add(a3688, a3689)?
        }, 2u64))?;
        match m3690 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f315(v2.clone(), 0u64)?;
                let a3691 = v3;
                let a3692 = 32u64;
                if nat_lt(a3691, a3692) {
                    if f568(v3) {
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

pub fn f544(v0: u64) -> Bytes {
    let a3693 = v0;
    let a3694 = 34u64;
    if nat_eq(a3693, a3694) {
        Bytes::lit(&[0x22])
    } else {
        let a3695 = v0;
        let a3696 = 92u64;
        if nat_eq(a3695, a3696) {
            Bytes::lit(&[0x5c])
        } else {
            let a3697 = v0;
            let a3698 = 98u64;
            if nat_eq(a3697, a3698) {
                Bytes::lit(&[0x08])
            } else {
                let a3699 = v0;
                let a3700 = 102u64;
                if nat_eq(a3699, a3700) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3701 = v0;
                    let a3702 = 110u64;
                    if nat_eq(a3701, a3702) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3703 = v0;
                        let a3704 = 114u64;
                        if nat_eq(a3703, a3704) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3705 = v0;
                            let a3706 = 116u64;
                            if nat_eq(a3705, a3706) {
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

pub fn f545(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3709 = {
        let a3707 = v0;
        let a3708 = v1;
        index_bytes(a3707, a3708)
    };
    match m3709 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f569(v2)? {
                Ok(Some({
                    let a3710 = v1;
                    let a3711 = 1u64;
                    nat_add(a3710, a3711)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f546(v0: u8) -> R<u64> {
    let a3758 = {
        let v1: u8 = v0;
        let a3756 = {
            let a3732 = {
                let a3720 = {
                    let a3714 = {
                        let a3712 = v1;
                        let a3713 = 128u8;
                        fixed_u8::bit_and(a3712, a3713)
                    };
                    let a3715 = 128u8;
                    if equal(a3714, a3715) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3721 = {
                    let a3718 = {
                        let a3716 = v1;
                        let a3717 = 64u8;
                        fixed_u8::bit_and(a3716, a3717)
                    };
                    let a3719 = 64u8;
                    if equal(a3718, a3719) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3720, a3721)?
            };
            let a3733 = {
                let a3730 = {
                    let a3724 = {
                        let a3722 = v1;
                        let a3723 = 32u8;
                        fixed_u8::bit_and(a3722, a3723)
                    };
                    let a3725 = 32u8;
                    if equal(a3724, a3725) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3731 = {
                    let a3728 = {
                        let a3726 = v1;
                        let a3727 = 16u8;
                        fixed_u8::bit_and(a3726, a3727)
                    };
                    let a3729 = 16u8;
                    if equal(a3728, a3729) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3730, a3731)?
            };
            nat_add(a3732, a3733)?
        };
        let a3757 = {
            let a3754 = {
                let a3742 = {
                    let a3736 = {
                        let a3734 = v1;
                        let a3735 = 8u8;
                        fixed_u8::bit_and(a3734, a3735)
                    };
                    let a3737 = 8u8;
                    if equal(a3736, a3737) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3743 = {
                    let a3740 = {
                        let a3738 = v1;
                        let a3739 = 4u8;
                        fixed_u8::bit_and(a3738, a3739)
                    };
                    let a3741 = 4u8;
                    if equal(a3740, a3741) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3742, a3743)?
            };
            let a3755 = {
                let a3752 = {
                    let a3746 = {
                        let a3744 = v1;
                        let a3745 = 2u8;
                        fixed_u8::bit_and(a3744, a3745)
                    };
                    let a3747 = 2u8;
                    if equal(a3746, a3747) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3753 = {
                    let a3750 = {
                        let a3748 = v1;
                        let a3749 = 1u8;
                        fixed_u8::bit_and(a3748, a3749)
                    };
                    let a3751 = 1u8;
                    if equal(a3750, a3751) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3752, a3753)?
            };
            nat_add(a3754, a3755)?
        };
        nat_add(a3756, a3757)?
    };
    let a3759 = 48u64;
    Ok(nat_sub(a3758, a3759))
}

pub fn f547(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3760 = v0.clone();
        length_bytes(a3760)
    };
    let a3761 = v1;
    let a3762 = v2;
    if if nat_lt(a3761, a3762) {
        let a3767 = f315(v0, {
            let a3765 = v2;
            let a3766 = {
                let a3763 = v1;
                let a3764 = 1u64;
                nat_add(a3763, a3764)?
            };
            nat_sub(a3765, a3766)
        })?;
        let a3768 = 48u64;
        nat_eq(a3767, a3768)
    } else {
        false
    } {
        Ok(Some({
            let a3769 = v1;
            let a3770 = 1u64;
            nat_add(a3769, a3770)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f548(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3771 = v1;
    match m3771.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3772 = f570(v0, v3.clone())?;
            match m3772 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f571(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f87(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3773 = f548(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3773 {
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

pub fn f549(p3777: List<Bytes>) -> R<bool> {
    let m3776 = p3777;
    {
        let mut m3776 = m3776;
        loop {
            let step = {
                let v0 = m3776;
                let m3774 = v0;
                match m3774.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3775 = f572(v1)?;
                        match m3775 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f87(v3, f573(false, v2.clone())?) {
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
                Ok(next) => { m3776 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: List<(Bytes, Adt19)>, v1: List<Adt19>) -> List<Bytes> {
    let m3778 = v0;
    match m3778.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f574(v1.clone(), {
                let (h3779, _) = v2.clone();
                h3779
            }) {
                List::cons({
                    let (h3780, _) = v2;
                    h3780
                }, f550(v3, v1))
            } else {
                f550(v3, v1)
            }
        }
    }
}

pub fn f551(v0: bool, v1: Bytes, v2: List<Adt19>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m3781 = v2;
    match m3781.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3782 = f575(v4.clone())?;
            match m3782 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3783 = f576(v0, v6.clone())?;
                    match m3783 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f87(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3786 = f577(v4, {
                                    let a3784 = v1.clone();
                                    let a3785 = f243(v6)?;
                                    append_bytes(a3784, a3785)
                                })?;
                                match m3786 {
                                    Ok(v9) => {
                                        let m3787 = f551(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3787 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h3788, _) = v9;
                                                    h3788
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h3789, _) = v11;
                                                    h3789
                                                }), f395({
                                                    let (_, h3790) = v9;
                                                    h3790
                                                }, {
                                                    let (_, h3791) = v11;
                                                    h3791
                                                }))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v8))
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

pub fn f552(v0: List<Adt19>) -> R<List<Bytes>> {
    let m3792 = v0;
    match m3792.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3793 = f575(v1)?;
            match m3793 {
                None => {
                    f552(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f552(v2)?))
                }
            }
        }
    }
}

pub fn f553(v0: Bytes, v1: List<Adt19>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f578(f549(f579(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f554(v0: Adt19, v1: Bytes) -> R<bool> {
    let m3794 = f247(Some(v0));
    match m3794 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3795 = f504(v2)?;
            match m3795 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f37(v3, v1))
                }
            }
        }
    }
}

pub fn f555(v0: bool, v1: Bytes, v2: List<(Bytes, Adt19)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m3796 = v2;
    match m3796.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3798 = f576(v0, {
                let (h3797, _) = v5.clone();
                h3797
            })?;
            match m3798 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3799, _) = v5;
                        h3799
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f87(v7.clone(), f573(v0, f556(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3804 = f512({
                            let (_, h3800) = v5.clone();
                            h3800
                        }, {
                            let a3802 = v1.clone();
                            let a3803 = f243({
                                let (h3801, _) = v5.clone();
                                h3801
                            })?;
                            append_bytes(a3802, a3803)
                        })?;
                        match m3804 {
                            Ok(v9) => {
                                let v10: Bytes = if f87({
                                    let (h3805, _) = v5;
                                    h3805
                                }, v3.clone()) {
                                    let (h3806, _) = v9;
                                    h3806
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h3807, _) = v9;
                                        h3807
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3808 = f555(v0, v1, v6, v3, v4)?;
                                match m3808 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h3809, _) = v12;
                                            h3809
                                        }), f395({
                                            let (_, h3810) = v9;
                                            h3810
                                        }, {
                                            let (_, h3811) = v12;
                                            h3811
                                        }))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f556(v0: List<(Bytes, Adt19)>) -> List<Bytes> {
    let m3812 = v0;
    match m3812.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3813, _) = v1;
                h3813
            }, f556(v2))
        }
    }
}

pub fn f557(v0: Adt19) -> Option<Adt19> {
    let m3814 = f153(f148(v0, f8(Str::lit("oneOf"))));
    match m3814.uncons() {
        None => {
            None::<Adt19>
        }
        Some((v1, v2)) => {
            let m3815 = v2;
            match m3815.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v3, v4)) => {
                    let m3816 = v4;
                    match m3816.uncons() {
                        None => {
                            if f580(v3.clone()) {
                                Some(v1)
                            } else {
                                if f580(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt19>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt19>
                        }
                    }
                }
            }
        }
    }
}

pub fn f558(v0: Bytes, v1: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m3817 = v1;
    match m3817 {
        Ok(v3) => {
            f507(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h3818, _) = v3;
                h3818
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h3819) = v3;
                h3819
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v2)
        }
    }
}

pub fn f559(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3820 = f557(v0.clone());
    match m3820 {
        None => {
            if f37(f151(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3821 = f148(v0, f8(Str::lit("items")));
                match m3821 {
                    None => {
                        Ok(f558(f8(Str::lit("List")), f560()))
                    }
                    Some(v3) => {
                        Ok(f558(f8(Str::lit("List")), f577(v3, {
                            let a3822 = v1;
                            let a3823 = f8(Str::lit("Item"));
                            append_bytes(a3822, a3823)
                        })?))
                    }
                }
            } else {
                f577(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f558(f8(Str::lit("Option")), f577(v2, v1)?))
        }
    }
}

pub fn f560() -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((f439(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f561(p3827: List<(Bytes, Adt19)>, v1: List<Bytes>) -> bool {
    let m3826 = p3827;
    {
        let mut m3826 = m3826;
        loop {
            let step = {
                let v0 = m3826;
                let m3824 = v0;
                match m3824.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f87({
                            let (h3825, _) = v2;
                            h3825
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt19)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt19)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3826 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f562(p3831: List<Adt19>, v1: Bytes, p3832: List<Bytes>) -> bool {
    let m3830 = (p3831, p3832);
    {
        let mut m3830 = m3830;
        loop {
            let step = {
                let (v0, v2) = m3830;
                let m3828 = v0;
                match m3828.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3829 = f581(v3, v1.clone());
                        match m3829 {
                            None => {
                                Err::<(List<Adt19>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f87(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt19>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt19>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3830 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f563(p3835: List<Adt19>, p3836: List<Bytes>) -> bool {
    let m3834 = (p3835, p3836);
    {
        let mut m3834 = m3834;
        loop {
            let step = {
                let (v0, v1) = m3834;
                let m3833 = v0;
                match m3833.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f151(v2, f8(Str::lit("type")));
                        if !f37(v4.clone(), Bytes::lit(&[])) {
                            if !f87(v4.clone(), v1.clone()) {
                                if !if if f37(v4.clone(), f8(Str::lit("integer"))) {
                                    f87(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f37(v4.clone(), f8(Str::lit("number"))) {
                                        f87(f8(Str::lit("integer")), v1.clone())
                                    } else {
                                        false
                                    }
                                } {
                                    Ok::<(List<Adt19>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt19>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt19>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt19>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3834 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f564() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f565(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f478({
        let (_, h3837) = {
            let (h3838, _) = f582(Fn22::F583, {
                let a3840 = {
                    let a3839 = v0.clone();
                    length_list(a3839)
                };
                let a3841 = 1u64;
                nat_add(a3840, a3841)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3838
        };
        h3837
    }))
}

pub fn f566(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3846 = {
            let a3844 = {
                let a3842 = v0.clone();
                length_list(a3842)
            };
            let a3845 = {
                let a3843 = v1.clone();
                length_list(a3843)
            };
            nat_add(a3844, a3845)?
        };
        let a3847 = 1u64;
        nat_add(a3846, a3847)?
    };
    Ok(f355({
        let (_, h3848) = {
            let (_, h3849) = {
                let (h3850, _) = f584(Fn23::F585, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3850
            };
            h3849
        };
        h3848
    }))
}

pub fn f567(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3851 = v0.clone();
        length_bytes(a3851)
    };
    let a3855 = {
        let a3852 = v1;
        let a3853 = 2u64;
        let a3854 = 0u64;
        nat_rem(a3852, a3853, a3854)
    };
    let a3856 = 0u64;
    if nat_eq(a3855, a3856) {
        let (_, h3857) = {
            let (h3858, _) = f586(Fn24::F587(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3858
        };
        Ok(h3857)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f568(v0: u64) -> bool {
    let a3859 = v0;
    let a3860 = 8u64;
    if nat_eq(a3859, a3860) {
        true
    } else {
        let a3861 = v0;
        let a3862 = 9u64;
        if nat_eq(a3861, a3862) {
            true
        } else {
            let a3863 = v0;
            let a3864 = 10u64;
            if nat_eq(a3863, a3864) {
                true
            } else {
                let a3865 = v0;
                let a3866 = 12u64;
                if nat_eq(a3865, a3866) {
                    true
                } else {
                    let a3867 = v0;
                    let a3868 = 13u64;
                    nat_eq(a3867, a3868)
                }
            }
        }
    }
}

pub fn f569(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3913 = {
            let a3889 = {
                let a3877 = {
                    let a3871 = {
                        let a3869 = v1;
                        let a3870 = 128u8;
                        fixed_u8::bit_and(a3869, a3870)
                    };
                    let a3872 = 128u8;
                    if equal(a3871, a3872) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3878 = {
                    let a3875 = {
                        let a3873 = v1;
                        let a3874 = 64u8;
                        fixed_u8::bit_and(a3873, a3874)
                    };
                    let a3876 = 64u8;
                    if equal(a3875, a3876) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3877, a3878)?
            };
            let a3890 = {
                let a3887 = {
                    let a3881 = {
                        let a3879 = v1;
                        let a3880 = 32u8;
                        fixed_u8::bit_and(a3879, a3880)
                    };
                    let a3882 = 32u8;
                    if equal(a3881, a3882) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3888 = {
                    let a3885 = {
                        let a3883 = v1;
                        let a3884 = 16u8;
                        fixed_u8::bit_and(a3883, a3884)
                    };
                    let a3886 = 16u8;
                    if equal(a3885, a3886) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3887, a3888)?
            };
            nat_add(a3889, a3890)?
        };
        let a3914 = {
            let a3911 = {
                let a3899 = {
                    let a3893 = {
                        let a3891 = v1;
                        let a3892 = 8u8;
                        fixed_u8::bit_and(a3891, a3892)
                    };
                    let a3894 = 8u8;
                    if equal(a3893, a3894) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3900 = {
                    let a3897 = {
                        let a3895 = v1;
                        let a3896 = 4u8;
                        fixed_u8::bit_and(a3895, a3896)
                    };
                    let a3898 = 4u8;
                    if equal(a3897, a3898) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3899, a3900)?
            };
            let a3912 = {
                let a3909 = {
                    let a3903 = {
                        let a3901 = v1;
                        let a3902 = 2u8;
                        fixed_u8::bit_and(a3901, a3902)
                    };
                    let a3904 = 2u8;
                    if equal(a3903, a3904) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3910 = {
                    let a3907 = {
                        let a3905 = v1;
                        let a3906 = 1u8;
                        fixed_u8::bit_and(a3905, a3906)
                    };
                    let a3908 = 1u8;
                    if equal(a3907, a3908) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3909, a3910)?
            };
            nat_add(a3911, a3912)?
        };
        nat_add(a3913, a3914)?
    };
    let a3915 = 48u64;
    let a3916 = v2;
    if nat_le(a3915, a3916) {
        let a3917 = v2;
        let a3918 = 57u64;
        Ok(nat_le(a3917, a3918))
    } else {
        Ok(false)
    }
}

pub fn f570(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m3919 = v1;
    match m3919 {
        Adt19::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C1(v7, v8) => {
            Ok(Some({
                let a3930 = if if v7 {
                    let a3928 = 0u64;
                    let a3929 = v8;
                    nat_lt(a3928, a3929)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3931 = f334(false, v8)?;
                append_bytes(a3930, a3931)
            }))
        }
        Adt19::C2(v3, v4, v5) => {
            let m3922 = f588({
                let a3920 = f8(Str::lit("n"));
                let a3921 = f335(false, v4, v5)?;
                append_bytes(a3920, a3921)
            })?;
            match m3922 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3926 = f8(Str::lit("nMinus"));
                        let a3927 = f32(v6.clone(), 1u64, {
                            let a3924 = {
                                let a3923 = v6;
                                length_bytes(a3923)
                            };
                            let a3925 = 1u64;
                            nat_sub(a3924, a3925)
                        });
                        append_bytes(a3926, a3927)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C3(v2) => {
            f576(v0, v2)
        }
        Adt19::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C6 => {
            Ok(None::<Bytes>)
        }
        Adt19::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt19::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f571(v0: Adt19) -> R<Bytes> {
    f246(v0)
}

pub fn f572(v0: Bytes) -> R<Option<Bytes>> {
    if f589({
        let a3932 = v0.clone();
        length_bytes(a3932)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f590(v0)?;
        if f591(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3935 = {
                let a3933 = v1.clone();
                let a3934 = 0u64;
                index_bytes(a3933, a3934)
            };
            match m3935 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f389({
                        let v3: u8 = v2;
                        let a3980 = {
                            let a3956 = {
                                let a3944 = {
                                    let a3938 = {
                                        let a3936 = v3;
                                        let a3937 = 128u8;
                                        fixed_u8::bit_and(a3936, a3937)
                                    };
                                    let a3939 = 128u8;
                                    if equal(a3938, a3939) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3945 = {
                                    let a3942 = {
                                        let a3940 = v3;
                                        let a3941 = 64u8;
                                        fixed_u8::bit_and(a3940, a3941)
                                    };
                                    let a3943 = 64u8;
                                    if equal(a3942, a3943) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3944, a3945)?
                            };
                            let a3957 = {
                                let a3954 = {
                                    let a3948 = {
                                        let a3946 = v3;
                                        let a3947 = 32u8;
                                        fixed_u8::bit_and(a3946, a3947)
                                    };
                                    let a3949 = 32u8;
                                    if equal(a3948, a3949) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3955 = {
                                    let a3952 = {
                                        let a3950 = v3;
                                        let a3951 = 16u8;
                                        fixed_u8::bit_and(a3950, a3951)
                                    };
                                    let a3953 = 16u8;
                                    if equal(a3952, a3953) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3954, a3955)?
                            };
                            nat_add(a3956, a3957)?
                        };
                        let a3981 = {
                            let a3978 = {
                                let a3966 = {
                                    let a3960 = {
                                        let a3958 = v3;
                                        let a3959 = 8u8;
                                        fixed_u8::bit_and(a3958, a3959)
                                    };
                                    let a3961 = 8u8;
                                    if equal(a3960, a3961) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3967 = {
                                    let a3964 = {
                                        let a3962 = v3;
                                        let a3963 = 4u8;
                                        fixed_u8::bit_and(a3962, a3963)
                                    };
                                    let a3965 = 4u8;
                                    if equal(a3964, a3965) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3966, a3967)?
                            };
                            let a3979 = {
                                let a3976 = {
                                    let a3970 = {
                                        let a3968 = v3;
                                        let a3969 = 2u8;
                                        fixed_u8::bit_and(a3968, a3969)
                                    };
                                    let a3971 = 2u8;
                                    if equal(a3970, a3971) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3977 = {
                                    let a3974 = {
                                        let a3972 = v3;
                                        let a3973 = 1u8;
                                        fixed_u8::bit_and(a3972, a3973)
                                    };
                                    let a3975 = 1u8;
                                    if equal(a3974, a3975) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3976, a3977)?
                            };
                            nat_add(a3978, a3979)?
                        };
                        nat_add(a3980, a3981)?
                    }) {
                        Ok(Some({
                            let a3982 = f8(Str::lit("n"));
                            let a3983 = v1;
                            append_bytes(a3982, a3983)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f592({
            let a3984 = v0.clone();
            length_bytes(a3984)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f573(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3985 = v1;
    match m3985.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3986 = f576(v0, v2)?;
            match m3986 {
                None => {
                    f573(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f573(v0, v3)?))
                }
            }
        }
    }
}

pub fn f574(p3990: List<Adt19>, v1: Bytes) -> bool {
    let m3989 = p3990;
    {
        let mut m3989 = m3989;
        loop {
            let step = {
                let v0 = m3989;
                let m3987 = v0;
                match m3987.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3988 = f581(v2, v1.clone());
                        if match m3988 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt19>, bool>(v3)
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3989 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f575(v0: Adt19) -> R<Option<Bytes>> {
    let m3991 = f247(f148(v0.clone(), f8(Str::lit("$ref"))));
    match m3991 {
        None => {
            Ok(f247(f148(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f504(v1)
        }
    }
}

pub fn f576(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f588(v1)
    } else {
        f572(v1)
    }
}

pub fn f577(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f593(f476(v0), v1)
}

pub fn f578(v0: bool, v1: Bytes, v2: List<Adt19>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m3992 = v2;
    match m3992.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3993 = f581(v5.clone(), v3.clone());
            match m3993 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3994 = f576(v0, v7.clone())?;
                    match m3994 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f87(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3997 = f510({
                                    let a3995 = v1.clone();
                                    let a3996 = f243(v7)?;
                                    append_bytes(a3995, a3996)
                                }, f438(f391(f148(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f152(f153(f148(v5, f8(Str::lit("required"))))), true)?;
                                match m3997 {
                                    Ok(v10) => {
                                        let m3998 = f578(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3998 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f216({
                                                    let (h3999, _) = v10;
                                                    h3999
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4000, _) = v12;
                                                    h4000
                                                }), f395({
                                                    let (_, h4001) = v10;
                                                    h4001
                                                }, {
                                                    let (_, h4002) = v12;
                                                    h4002
                                                }))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v9))
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

pub fn f579(v0: List<Adt19>, v1: Bytes) -> List<Bytes> {
    let m4003 = v0;
    match m4003.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4004 = f581(v2, v1.clone());
            match m4004 {
                None => {
                    f579(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f579(v3, v1))
                }
            }
        }
    }
}

pub fn f580(v0: Adt19) -> bool {
    f37(f151(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f581(v0: Adt19, v1: Bytes) -> Option<Bytes> {
    if f87(v1.clone(), f152(f153(f148(v0.clone(), f8(Str::lit("required")))))) {
        let m4005 = f148(v0, f8(Str::lit("properties")));
        match m4005 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m4006 = f148(v2, v1);
                match m4006 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f247(f148(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f582(v0: Fn22, p4011: u64, p4012: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4010 = (p4011, p4012);
    {
        let mut m4010 = m4010;
        loop {
            let step = {
                let (v1, v2) = m4010;
                let m4007 = v1;
                if m4007 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4007 - 1;
                    let m4009 = {
                        let c4008: Fn22 = v0.clone();
                        c4008.apply(v2.clone())?
                    };
                    match m4009 {
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
                Ok(next) => { m4010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f594(v0)
}

pub fn f584(v0: Fn23, p4017: u64, p4018: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4016 = (p4017, p4018);
    {
        let mut m4016 = m4016;
        loop {
            let step = {
                let (v1, v2) = m4016;
                let m4013 = v1;
                if m4013 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4013 - 1;
                    let m4015 = {
                        let c4014: Fn23 = v0.clone();
                        c4014.apply(v2.clone())
                    };
                    match m4015 {
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
                Ok(next) => { m4016 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f585(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f595(v0)
}

pub fn f586(v0: Fn24, p4023: u64, p4024: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4022 = (p4023, p4024);
    {
        let mut m4022 = m4022;
        loop {
            let step = {
                let (v1, v2) = m4022;
                let m4019 = v1;
                if m4019 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4019 - 1;
                    let m4021 = {
                        let c4020: Fn24 = v0.clone();
                        c4020.apply(v2.clone())?
                    };
                    match m4021 {
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
                Ok(next) => { m4022 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f596(v0, v1)
}

pub fn f588(v0: Bytes) -> R<Option<Bytes>> {
    let m4026 = f597({
        let a4025 = v0.clone();
        length_bytes(a4025)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m4026 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a4028 = {
                let a4027 = v1.clone();
                length_bytes(a4027)
            };
            let a4029 = 0u64;
            if nat_eq(a4028, a4029) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f589(p4085: u64, v1: Bytes, p4086: u64) -> R<bool> {
    let m4084 = (p4085, p4086);
    {
        let mut m4084 = m4084;
        loop {
            let step = {
                let (v0, v2) = m4084;
                let m4030 = v0;
                if m4030 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4030 - 1;
                    let m4033 = {
                        let a4031 = v1.clone();
                        let a4032 = v2;
                        index_bytes(a4031, a4032)
                    };
                    match m4033 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4080 = {
                                let v5: u8 = v4;
                                let a4078 = {
                                    let a4054 = {
                                        let a4042 = {
                                            let a4036 = {
                                                let a4034 = v5;
                                                let a4035 = 128u8;
                                                fixed_u8::bit_and(a4034, a4035)
                                            };
                                            let a4037 = 128u8;
                                            if equal(a4036, a4037) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4043 = {
                                            let a4040 = {
                                                let a4038 = v5;
                                                let a4039 = 64u8;
                                                fixed_u8::bit_and(a4038, a4039)
                                            };
                                            let a4041 = 64u8;
                                            if equal(a4040, a4041) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4042, a4043)?
                                    };
                                    let a4055 = {
                                        let a4052 = {
                                            let a4046 = {
                                                let a4044 = v5;
                                                let a4045 = 32u8;
                                                fixed_u8::bit_and(a4044, a4045)
                                            };
                                            let a4047 = 32u8;
                                            if equal(a4046, a4047) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4053 = {
                                            let a4050 = {
                                                let a4048 = v5;
                                                let a4049 = 16u8;
                                                fixed_u8::bit_and(a4048, a4049)
                                            };
                                            let a4051 = 16u8;
                                            if equal(a4050, a4051) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4052, a4053)?
                                    };
                                    nat_add(a4054, a4055)?
                                };
                                let a4079 = {
                                    let a4076 = {
                                        let a4064 = {
                                            let a4058 = {
                                                let a4056 = v5;
                                                let a4057 = 8u8;
                                                fixed_u8::bit_and(a4056, a4057)
                                            };
                                            let a4059 = 8u8;
                                            if equal(a4058, a4059) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4065 = {
                                            let a4062 = {
                                                let a4060 = v5;
                                                let a4061 = 4u8;
                                                fixed_u8::bit_and(a4060, a4061)
                                            };
                                            let a4063 = 4u8;
                                            if equal(a4062, a4063) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4064, a4065)?
                                    };
                                    let a4077 = {
                                        let a4074 = {
                                            let a4068 = {
                                                let a4066 = v5;
                                                let a4067 = 2u8;
                                                fixed_u8::bit_and(a4066, a4067)
                                            };
                                            let a4069 = 2u8;
                                            if equal(a4068, a4069) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4075 = {
                                            let a4072 = {
                                                let a4070 = v5;
                                                let a4071 = 1u8;
                                                fixed_u8::bit_and(a4070, a4071)
                                            };
                                            let a4073 = 1u8;
                                            if equal(a4072, a4073) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4074, a4075)?
                                    };
                                    nat_add(a4076, a4077)?
                                };
                                nat_add(a4078, a4079)?
                            };
                            let a4081 = 128u64;
                            if nat_lt(a4080, a4081) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4082 = v2;
                                    let a4083 = 1u64;
                                    nat_add(a4082, a4083)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4084 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f330(v0.clone(), 0u64, {
        let a4087 = v0;
        length_bytes(a4087)
    }, false, Bytes::lit(&[]))?;
    let m4090 = {
        let a4088 = v1.clone();
        let a4089 = 0u64;
        index_bytes(a4088, a4089)
    };
    match m4090 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a4135 = {
                    let a4111 = {
                        let a4099 = {
                            let a4093 = {
                                let a4091 = v3;
                                let a4092 = 128u8;
                                fixed_u8::bit_and(a4091, a4092)
                            };
                            let a4094 = 128u8;
                            if equal(a4093, a4094) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4100 = {
                            let a4097 = {
                                let a4095 = v3;
                                let a4096 = 64u8;
                                fixed_u8::bit_and(a4095, a4096)
                            };
                            let a4098 = 64u8;
                            if equal(a4097, a4098) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4099, a4100)?
                    };
                    let a4112 = {
                        let a4109 = {
                            let a4103 = {
                                let a4101 = v3;
                                let a4102 = 32u8;
                                fixed_u8::bit_and(a4101, a4102)
                            };
                            let a4104 = 32u8;
                            if equal(a4103, a4104) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4110 = {
                            let a4107 = {
                                let a4105 = v3;
                                let a4106 = 16u8;
                                fixed_u8::bit_and(a4105, a4106)
                            };
                            let a4108 = 16u8;
                            if equal(a4107, a4108) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4109, a4110)?
                    };
                    nat_add(a4111, a4112)?
                };
                let a4136 = {
                    let a4133 = {
                        let a4121 = {
                            let a4115 = {
                                let a4113 = v3;
                                let a4114 = 8u8;
                                fixed_u8::bit_and(a4113, a4114)
                            };
                            let a4116 = 8u8;
                            if equal(a4115, a4116) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4122 = {
                            let a4119 = {
                                let a4117 = v3;
                                let a4118 = 4u8;
                                fixed_u8::bit_and(a4117, a4118)
                            };
                            let a4120 = 4u8;
                            if equal(a4119, a4120) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4121, a4122)?
                    };
                    let a4134 = {
                        let a4131 = {
                            let a4125 = {
                                let a4123 = v3;
                                let a4124 = 2u8;
                                fixed_u8::bit_and(a4123, a4124)
                            };
                            let a4126 = 2u8;
                            if equal(a4125, a4126) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4132 = {
                            let a4129 = {
                                let a4127 = v3;
                                let a4128 = 1u8;
                                fixed_u8::bit_and(a4127, a4128)
                            };
                            let a4130 = 1u8;
                            if equal(a4129, a4130) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4131, a4132)?
                    };
                    nat_add(a4133, a4134)?
                };
                nat_add(a4135, a4136)?
            };
            if f388(v4) {
                let a4143 = f32({
                    let a4137 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4137)
                }, {
                    let a4138 = v4;
                    let a4139 = 65u64;
                    nat_sub(a4138, a4139)
                }, 1u64);
                let a4144 = f32(v1.clone(), 1u64, {
                    let a4141 = {
                        let a4140 = v1;
                        length_bytes(a4140)
                    };
                    let a4142 = 1u64;
                    nat_sub(a4141, a4142)
                });
                Ok(append_bytes(a4143, a4144))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f591(v0: Bytes) -> R<bool> {
    let m4147 = {
        let a4145 = v0;
        let a4146 = 0u64;
        index_bytes(a4145, a4146)
    };
    match m4147 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f387({
                let v2: u8 = v1;
                let a4192 = {
                    let a4168 = {
                        let a4156 = {
                            let a4150 = {
                                let a4148 = v2;
                                let a4149 = 128u8;
                                fixed_u8::bit_and(a4148, a4149)
                            };
                            let a4151 = 128u8;
                            if equal(a4150, a4151) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4157 = {
                            let a4154 = {
                                let a4152 = v2;
                                let a4153 = 64u8;
                                fixed_u8::bit_and(a4152, a4153)
                            };
                            let a4155 = 64u8;
                            if equal(a4154, a4155) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4156, a4157)?
                    };
                    let a4169 = {
                        let a4166 = {
                            let a4160 = {
                                let a4158 = v2;
                                let a4159 = 32u8;
                                fixed_u8::bit_and(a4158, a4159)
                            };
                            let a4161 = 32u8;
                            if equal(a4160, a4161) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4167 = {
                            let a4164 = {
                                let a4162 = v2;
                                let a4163 = 16u8;
                                fixed_u8::bit_and(a4162, a4163)
                            };
                            let a4165 = 16u8;
                            if equal(a4164, a4165) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4166, a4167)?
                    };
                    nat_add(a4168, a4169)?
                };
                let a4193 = {
                    let a4190 = {
                        let a4178 = {
                            let a4172 = {
                                let a4170 = v2;
                                let a4171 = 8u8;
                                fixed_u8::bit_and(a4170, a4171)
                            };
                            let a4173 = 8u8;
                            if equal(a4172, a4173) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4179 = {
                            let a4176 = {
                                let a4174 = v2;
                                let a4175 = 4u8;
                                fixed_u8::bit_and(a4174, a4175)
                            };
                            let a4177 = 4u8;
                            if equal(a4176, a4177) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4178, a4179)?
                    };
                    let a4191 = {
                        let a4188 = {
                            let a4182 = {
                                let a4180 = v2;
                                let a4181 = 2u8;
                                fixed_u8::bit_and(a4180, a4181)
                            };
                            let a4183 = 2u8;
                            if equal(a4182, a4183) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4189 = {
                            let a4186 = {
                                let a4184 = v2;
                                let a4185 = 1u8;
                                fixed_u8::bit_and(a4184, a4185)
                            };
                            let a4187 = 1u8;
                            if equal(a4186, a4187) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4188, a4189)?
                    };
                    nat_add(a4190, a4191)?
                };
                nat_add(a4192, a4193)?
            }) {
                Ok(true)
            } else {
                Ok(f388({
                    let v3: u8 = v1;
                    let a4238 = {
                        let a4214 = {
                            let a4202 = {
                                let a4196 = {
                                    let a4194 = v3;
                                    let a4195 = 128u8;
                                    fixed_u8::bit_and(a4194, a4195)
                                };
                                let a4197 = 128u8;
                                if equal(a4196, a4197) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a4203 = {
                                let a4200 = {
                                    let a4198 = v3;
                                    let a4199 = 64u8;
                                    fixed_u8::bit_and(a4198, a4199)
                                };
                                let a4201 = 64u8;
                                if equal(a4200, a4201) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4202, a4203)?
                        };
                        let a4215 = {
                            let a4212 = {
                                let a4206 = {
                                    let a4204 = v3;
                                    let a4205 = 32u8;
                                    fixed_u8::bit_and(a4204, a4205)
                                };
                                let a4207 = 32u8;
                                if equal(a4206, a4207) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a4213 = {
                                let a4210 = {
                                    let a4208 = v3;
                                    let a4209 = 16u8;
                                    fixed_u8::bit_and(a4208, a4209)
                                };
                                let a4211 = 16u8;
                                if equal(a4210, a4211) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4212, a4213)?
                        };
                        nat_add(a4214, a4215)?
                    };
                    let a4239 = {
                        let a4236 = {
                            let a4224 = {
                                let a4218 = {
                                    let a4216 = v3;
                                    let a4217 = 8u8;
                                    fixed_u8::bit_and(a4216, a4217)
                                };
                                let a4219 = 8u8;
                                if equal(a4218, a4219) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a4225 = {
                                let a4222 = {
                                    let a4220 = v3;
                                    let a4221 = 4u8;
                                    fixed_u8::bit_and(a4220, a4221)
                                };
                                let a4223 = 4u8;
                                if equal(a4222, a4223) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4224, a4225)?
                        };
                        let a4237 = {
                            let a4234 = {
                                let a4228 = {
                                    let a4226 = v3;
                                    let a4227 = 2u8;
                                    fixed_u8::bit_and(a4226, a4227)
                                };
                                let a4229 = 2u8;
                                if equal(a4228, a4229) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a4235 = {
                                let a4232 = {
                                    let a4230 = v3;
                                    let a4231 = 1u8;
                                    fixed_u8::bit_and(a4230, a4231)
                                };
                                let a4233 = 1u8;
                                if equal(a4232, a4233) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4234, a4235)?
                        };
                        nat_add(a4236, a4237)?
                    };
                    nat_add(a4238, a4239)?
                }))
            }
        }
    }
}

pub fn f592(p4252: u64, v1: Bytes, p4253: u64, p4254: Bytes) -> R<Option<Bytes>> {
    let m4251 = (p4252, (p4253, p4254));
    {
        let mut m4251 = m4251;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4251;
                let m4240 = v0;
                if m4240 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4240 - 1;
                    let a4242 = {
                        let a4241 = v1.clone();
                        length_bytes(a4241)
                    };
                    let a4243 = v2;
                    if nat_le(a4242, a4243) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4244 = f420(v1.clone(), v2)?;
                        match m4244 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4246 = v2;
                                    let a4247 = {
                                        let (_, h4245) = v5;
                                        h4245
                                    };
                                    nat_add(a4246, a4247)?
                                }, {
                                    let a4249 = v3;
                                    let a4250 = f598({
                                        let (h4248, _) = v5;
                                        h4248
                                    });
                                    append_bytes(a4249, a4250)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4255 = v0.clone();
    match m4255 {
        Adt19::C0(v2) => {
            if v2 {
                Ok(f560())
            } else {
                Ok(f506(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt19::C1(_, _) => {
            Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C2(_, _, _) => {
            Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C3(_) => {
            Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C4(_) => {
            Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C5(_) => {
            let m4256 = f599(f436(v0.clone()));
            match m4256 {
                None => {
                    let m4257 = f247(f148(v0.clone(), f8(Str::lit("$ref"))));
                    match m4257 {
                        None => {
                            let m4259 = f148(v0.clone(), f8(Str::lit("allOf")));
                            match m4259 {
                                None => {
                                    let m4264 = f148(v0.clone(), f8(Str::lit("enum")));
                                    match m4264 {
                                        None => {
                                            let m4265 = f148(v0.clone(), f8(Str::lit("const")));
                                            match m4265 {
                                                None => {
                                                    let m4267 = f148(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4267 {
                                                        None => {
                                                            let v30: Bytes = f151(v0.clone(), f8(Str::lit("type")));
                                                            if if f37(v30.clone(), Bytes::lit(&[])) {
                                                                !f600(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f560())
                                                            } else {
                                                                if f37(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f507(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f37(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f507(f601(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f37(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f507(f602(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f37(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f507(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f37(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m4268 = f391(f148(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4268.uncons() {
                                                                                        None => {
                                                                                            let m4269 = f603(v0);
                                                                                            match m4269 {
                                                                                                None => {
                                                                                                    Ok(f558(f8(Str::lit("List")), f604(f560())))
                                                                                                }
                                                                                                Some(v33) => {
                                                                                                    Ok(f558(f8(Str::lit("List")), f604(f605(v33, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f507(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f506(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f507(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m4266 = f247(Some(v27));
                                                    match m4266 {
                                                        None => {
                                                            Ok(f506(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f507(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f507(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m4260 = f248(v19);
                                    match m4260.uncons() {
                                        None => {
                                            Ok(f506(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m4261 = v21;
                                            match m4261.uncons() {
                                                None => {
                                                    let m4262 = f247(f148(v20, f8(Str::lit("$ref"))));
                                                    match m4262 {
                                                        None => {
                                                            Ok(f506(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m4263 = f504(v24)?;
                                                            match m4263 {
                                                                None => {
                                                                    Ok(f506(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f507(v25, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f506(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m4258 = f504(v17)?;
                            match m4258 {
                                None => {
                                    Ok(f506(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f507(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f506(if f37(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt19::C6 => {
            Ok(f560())
        }
        Adt19::C7(_, _) => {
            Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C8(_, _, _) => {
            Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f594(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4270, _) = v0;
        h4270
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4271) = v0;
        h4271
    };
    let m4272 = v1;
    match m4272.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4273 = v4;
            match m4273.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f606(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f595(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4274, _) = v0.clone();
        h4274
    };
    let v2: List<(Bytes, ())> = {
        let (h4275, _) = {
            let (_, h4276) = v0.clone();
            h4276
        };
        h4275
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4277) = {
            let (_, h4278) = v0;
            h4278
        };
        h4277
    };
    let m4279 = v1.clone();
    match m4279.uncons() {
        None => {
            let m4280 = v2;
            match m4280.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4281 = v2.clone();
            match m4281.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f607({
                        let (h4282, _) = v8.clone();
                        h4282
                    }, {
                        let (h4283, _) = v6.clone();
                        h4283
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

pub fn f596(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4284, _) = v1;
        h4284
    };
    let m4286 = {
        let (_, h4285) = v1;
        h4285
    };
    match m4286 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4289 = {
                let a4287 = v0.clone();
                let a4288 = v2;
                index_bytes(a4287, a4288)
            };
            match m4289 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4294 = {
                        let a4292 = v0;
                        let a4293 = {
                            let a4290 = v2;
                            let a4291 = 1u64;
                            nat_add(a4290, a4291)?
                        };
                        index_bytes(a4292, a4293)
                    };
                    match m4294 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f608(v4)? {
                                f608(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4295 = v2;
                                    let a4296 = 2u64;
                                    nat_add(a4295, a4296)?
                                }, Some({
                                    let a4301 = v3;
                                    let a4302 = f319(f609(), {
                                        let a4299 = {
                                            let a4297 = f610(v4)?;
                                            let a4298 = 16u64;
                                            nat_mul(a4297, a4298)?
                                        };
                                        let a4300 = f610(v5)?;
                                        nat_add(a4299, a4300)?
                                    });
                                    append_bytes(a4301, a4302)
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

pub fn f597(p4315: u64, v1: Bytes, p4316: u64, p4317: Bytes) -> R<Option<Bytes>> {
    let m4314 = (p4315, (p4316, p4317));
    {
        let mut m4314 = m4314;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4314;
                let m4303 = v0;
                if m4303 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4303 - 1;
                    let a4305 = {
                        let a4304 = v1.clone();
                        length_bytes(a4304)
                    };
                    let a4306 = v2;
                    if nat_le(a4305, a4306) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4307 = f420(v1.clone(), v2)?;
                        match m4307 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4309 = v2;
                                    let a4310 = {
                                        let (_, h4308) = v5;
                                        h4308
                                    };
                                    nat_add(a4309, a4310)?
                                }, {
                                    let a4312 = v3;
                                    let a4313 = f611(v1.clone(), v2, {
                                        let (h4311, _) = v5;
                                        h4311
                                    });
                                    append_bytes(a4312, a4313)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(v0: u64) -> Bytes {
    let a4318 = f8(Str::lit("u"));
    let a4319 = f612(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4318, a4319)
}

pub fn f599(p4324: List<(Bytes, Adt19)>) -> Option<Bytes> {
    let m4323 = p4324;
    {
        let mut m4323 = m4323;
        loop {
            let step = {
                let v0 = m4323;
                let m4320 = v0;
                match m4320.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f87({
                            let (h4321, _) = v1.clone();
                            h4321
                        }, f613()) {
                            Err::<List<(Bytes, Adt19)>, Option<Bytes>>(Some({
                                let (h4322, _) = v1;
                                h4322
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, Option<Bytes>>(v2)
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

pub fn f600(v0: Adt19) -> bool {
    if f515(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f515(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f515(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f515(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f515(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f515(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f601(v0: Adt19) -> Bytes {
    let m4325 = f614(f148(v0, f8(Str::lit("minimum"))));
    match m4325 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f602() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f603(v0: Adt19) -> Option<Adt19> {
    let m4326 = f148(v0.clone(), f8(Str::lit("additionalProperties")));
    match m4326 {
        None => {
            let m4327 = f391(f148(v0, f8(Str::lit("patternProperties"))));
            match m4327.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v2, v3)) => {
                    let m4328 = v3;
                    match m4328.uncons() {
                        None => {
                            Some({
                                let (_, h4329) = v2;
                                h4329
                            })
                        }
                        Some((_, _)) => {
                            None::<Adt19>
                        }
                    }
                }
            }
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f604(v0: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4330 = v0;
    match m4330 {
        Ok(v2) => {
            f507(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h4331, _) = v2;
                h4331
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h4332) = v2;
                h4332
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v1)
        }
    }
}

pub fn f605(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f615(16u64, v0, v1)
}

pub fn f606(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4337 = {
            let a4335 = {
                let a4333 = v0.clone();
                length_list(a4333)
            };
            let a4336 = {
                let a4334 = v1.clone();
                length_list(a4334)
            };
            nat_add(a4335, a4336)?
        };
        let a4338 = 1u64;
        nat_add(a4337, a4338)?
    };
    Ok(f616({
        let (_, h4339) = {
            let (_, h4340) = {
                let (h4341, _) = f617(Fn25::F618, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4341
            };
            h4340
        };
        h4339
    }))
}

pub fn f607(v0: Bytes, v1: Bytes) -> bool {
    let a4346 = {
        let a4342 = v0;
        let a4343 = v1;
        compare_bytes(a4342, a4343)
    };
    let a4347 = {
        let a4344 = Bytes::lit(&[]);
        let a4345 = Bytes::lit(&[0x01]);
        compare_bytes(a4344, a4345)
    };
    equal(a4346, a4347)
}

pub fn f608(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4392 = {
            let a4368 = {
                let a4356 = {
                    let a4350 = {
                        let a4348 = v1;
                        let a4349 = 128u8;
                        fixed_u8::bit_and(a4348, a4349)
                    };
                    let a4351 = 128u8;
                    if equal(a4350, a4351) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4357 = {
                    let a4354 = {
                        let a4352 = v1;
                        let a4353 = 64u8;
                        fixed_u8::bit_and(a4352, a4353)
                    };
                    let a4355 = 64u8;
                    if equal(a4354, a4355) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4356, a4357)?
            };
            let a4369 = {
                let a4366 = {
                    let a4360 = {
                        let a4358 = v1;
                        let a4359 = 32u8;
                        fixed_u8::bit_and(a4358, a4359)
                    };
                    let a4361 = 32u8;
                    if equal(a4360, a4361) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4367 = {
                    let a4364 = {
                        let a4362 = v1;
                        let a4363 = 16u8;
                        fixed_u8::bit_and(a4362, a4363)
                    };
                    let a4365 = 16u8;
                    if equal(a4364, a4365) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4366, a4367)?
            };
            nat_add(a4368, a4369)?
        };
        let a4393 = {
            let a4390 = {
                let a4378 = {
                    let a4372 = {
                        let a4370 = v1;
                        let a4371 = 8u8;
                        fixed_u8::bit_and(a4370, a4371)
                    };
                    let a4373 = 8u8;
                    if equal(a4372, a4373) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4379 = {
                    let a4376 = {
                        let a4374 = v1;
                        let a4375 = 4u8;
                        fixed_u8::bit_and(a4374, a4375)
                    };
                    let a4377 = 4u8;
                    if equal(a4376, a4377) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4378, a4379)?
            };
            let a4391 = {
                let a4388 = {
                    let a4382 = {
                        let a4380 = v1;
                        let a4381 = 2u8;
                        fixed_u8::bit_and(a4380, a4381)
                    };
                    let a4383 = 2u8;
                    if equal(a4382, a4383) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4389 = {
                    let a4386 = {
                        let a4384 = v1;
                        let a4385 = 1u8;
                        fixed_u8::bit_and(a4384, a4385)
                    };
                    let a4387 = 1u8;
                    if equal(a4386, a4387) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4388, a4389)?
            };
            nat_add(a4390, a4391)?
        };
        nat_add(a4392, a4393)?
    };
    let a4394 = 48u64;
    let a4395 = v2;
    if if nat_le(a4394, a4395) {
        let a4396 = v2;
        let a4397 = 57u64;
        nat_le(a4396, a4397)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4398 = 97u64;
        let a4399 = v2;
        if nat_le(a4398, a4399) {
            let a4400 = v2;
            let a4401 = 102u64;
            Ok(nat_le(a4400, a4401))
        } else {
            Ok(false)
        }
    }
}

pub fn f609() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f610(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4446 = {
            let a4422 = {
                let a4410 = {
                    let a4404 = {
                        let a4402 = v1;
                        let a4403 = 128u8;
                        fixed_u8::bit_and(a4402, a4403)
                    };
                    let a4405 = 128u8;
                    if equal(a4404, a4405) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4411 = {
                    let a4408 = {
                        let a4406 = v1;
                        let a4407 = 64u8;
                        fixed_u8::bit_and(a4406, a4407)
                    };
                    let a4409 = 64u8;
                    if equal(a4408, a4409) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4410, a4411)?
            };
            let a4423 = {
                let a4420 = {
                    let a4414 = {
                        let a4412 = v1;
                        let a4413 = 32u8;
                        fixed_u8::bit_and(a4412, a4413)
                    };
                    let a4415 = 32u8;
                    if equal(a4414, a4415) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4421 = {
                    let a4418 = {
                        let a4416 = v1;
                        let a4417 = 16u8;
                        fixed_u8::bit_and(a4416, a4417)
                    };
                    let a4419 = 16u8;
                    if equal(a4418, a4419) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4420, a4421)?
            };
            nat_add(a4422, a4423)?
        };
        let a4447 = {
            let a4444 = {
                let a4432 = {
                    let a4426 = {
                        let a4424 = v1;
                        let a4425 = 8u8;
                        fixed_u8::bit_and(a4424, a4425)
                    };
                    let a4427 = 8u8;
                    if equal(a4426, a4427) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4433 = {
                    let a4430 = {
                        let a4428 = v1;
                        let a4429 = 4u8;
                        fixed_u8::bit_and(a4428, a4429)
                    };
                    let a4431 = 4u8;
                    if equal(a4430, a4431) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4432, a4433)?
            };
            let a4445 = {
                let a4442 = {
                    let a4436 = {
                        let a4434 = v1;
                        let a4435 = 2u8;
                        fixed_u8::bit_and(a4434, a4435)
                    };
                    let a4437 = 2u8;
                    if equal(a4436, a4437) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4443 = {
                    let a4440 = {
                        let a4438 = v1;
                        let a4439 = 1u8;
                        fixed_u8::bit_and(a4438, a4439)
                    };
                    let a4441 = 1u8;
                    if equal(a4440, a4441) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4442, a4443)?
            };
            nat_add(a4444, a4445)?
        };
        nat_add(a4446, a4447)?
    };
    let a4448 = v2;
    let a4449 = 57u64;
    if nat_le(a4448, a4449) {
        let a4450 = v2;
        let a4451 = 48u64;
        Ok(nat_sub(a4450, a4451))
    } else {
        let a4452 = v2;
        let a4453 = 87u64;
        Ok(nat_sub(a4452, a4453))
    }
}

pub fn f611(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f387(v2) {
        true
    } else {
        f388(v2)
    } {
        let a4454 = v2;
        let a4455 = 117u64;
        if !nat_eq(a4454, a4455) {
            let a4456 = v2;
            let a4457 = 85u64;
            !nat_eq(a4456, a4457)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f389(v2) {
            let a4458 = 0u64;
            let a4459 = v1;
            nat_lt(a4458, a4459)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a4460 = v2;
        let a4461 = 65536u64;
        if nat_lt(a4460, a4461) {
            let a4462 = f8(Str::lit("u"));
            let a4463 = f612(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4462, a4463)
        } else {
            let a4464 = f8(Str::lit("U"));
            let a4465 = f612(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4464, a4465)
        }
    }
}

pub fn f612(p4482: u64, p4483: u64, p4484: u64, p4485: Bytes) -> Bytes {
    let m4481 = (p4482, (p4483, (p4484, p4485)));
    {
        let mut m4481 = m4481;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m4481;
                let m4466 = v0;
                if m4466 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m4466 - 1;
                    let a4467 = v1;
                    let a4468 = 0u64;
                    if if nat_eq(a4467, a4468) {
                        let a4469 = v2;
                        let a4470 = 0u64;
                        nat_eq(a4469, a4470)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a4471 = v1;
                            let a4472 = 16u64;
                            let a4473 = 0u64;
                            nat_quot(a4471, a4472, a4473)
                        }, ({
                            let a4474 = v2;
                            let a4475 = 1u64;
                            nat_sub(a4474, a4475)
                        }, {
                            let a4479 = f619({
                                let a4476 = v1;
                                let a4477 = 16u64;
                                let a4478 = 0u64;
                                nat_rem(a4476, a4477, a4478)
                            });
                            let a4480 = v3;
                            append_bytes(a4479, a4480)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m4481 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f613() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f614(v0: Option<Adt19>) -> Option<u64> {
    let m4486 = v0;
    match m4486 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m4487 = v1;
            match m4487 {
                Adt19::C0(_) => {
                    None::<u64>
                }
                Adt19::C1(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt19::C2(_, _, _) => {
                    None::<u64>
                }
                Adt19::C3(_) => {
                    None::<u64>
                }
                Adt19::C4(_) => {
                    None::<u64>
                }
                Adt19::C5(_) => {
                    None::<u64>
                }
                Adt19::C6 => {
                    None::<u64>
                }
                Adt19::C7(_, _) => {
                    None::<u64>
                }
                Adt19::C8(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f615(v0: u64, v1: Adt19, v2: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4488 = v0;
    if m4488 == 0 {
        Ok(f506(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4488 - 1;
        let m4489 = v1.clone();
        match m4489 {
            Adt19::C0(v4) => {
                if v4 {
                    Ok(f560())
                } else {
                    Ok(f506(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt19::C1(_, _) => {
                Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C2(_, _, _) => {
                Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C3(_) => {
                Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C4(_) => {
                Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C5(_) => {
                let m4490 = f247(f148(v1.clone(), f8(Str::lit("$ref"))));
                match m4490 {
                    None => {
                        if if f515(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f515(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f515(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f507(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f151(v1.clone(), f8(Str::lit("type")));
                            if if f37(v20.clone(), Bytes::lit(&[])) {
                                !f600(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f560())
                            } else {
                                if f37(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f507(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f37(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f507(f601(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f37(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f507(f602(), List::<Adt25>::nil()))
                                        } else {
                                            if f37(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f507(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f37(v20.clone(), f8(Str::lit("array"))) {
                                                    let m4492 = f148(v1, f8(Str::lit("items")));
                                                    match m4492 {
                                                        None => {
                                                            Ok(f558(f8(Str::lit("List")), f560()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f558(f8(Str::lit("List")), f615(v3, v21, {
                                                                let a4493 = v2;
                                                                let a4494 = f8(Str::lit("Item"));
                                                                append_bytes(a4493, a4494)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f37(v20.clone(), f8(Str::lit("object"))) {
                                                        let m4495 = f603(v1);
                                                        match m4495 {
                                                            None => {
                                                                Ok(f558(f8(Str::lit("List")), f604(f560())))
                                                            }
                                                            Some(v22) => {
                                                                Ok(f558(f8(Str::lit("List")), f604(f615(v3, v22, {
                                                                    let a4496 = v2;
                                                                    let a4497 = f8(Str::lit("Value"));
                                                                    append_bytes(a4496, a4497)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f506(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m4491 = f504(v18)?;
                        match m4491 {
                            None => {
                                Ok(f506(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f507(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt19::C6 => {
                Ok(f560())
            }
            Adt19::C7(_, _) => {
                Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C8(_, _, _) => {
                Ok(f506(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f616(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f401(Fn10::F620, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f617(v0: Fn25, p4502: u64, p4503: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4501 = (p4502, p4503);
    {
        let mut m4501 = m4501;
        loop {
            let step = {
                let (v1, v2) = m4501;
                let m4498 = v1;
                if m4498 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4498 - 1;
                    let m4500 = {
                        let c4499: Fn25 = v0.clone();
                        c4499.apply(v2.clone())
                    };
                    match m4500 {
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
                Ok(next) => { m4501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f621(v0)
}

pub fn f619(v0: u64) -> Bytes {
    f32({
        let a4504 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4504)
    }, v0, 1u64)
}

pub fn f620(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f621(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4505, _) = v0.clone();
        h4505
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4506, _) = {
            let (_, h4507) = v0.clone();
            h4507
        };
        h4506
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4508) = {
            let (_, h4509) = v0;
            h4509
        };
        h4508
    };
    let m4510 = v1.clone();
    match m4510.uncons() {
        None => {
            let m4511 = v2;
            match m4511.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4512 = v2.clone();
            match m4512.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f607({
                        let (h4513, _) = v8.clone();
                        h4513
                    }, {
                        let (h4514, _) = v6.clone();
                        h4514
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
