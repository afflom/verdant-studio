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
    C0(List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Adt14, List<Adt13>, List<Adt13>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(List<Bytes>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(bool, bool, Bytes, List<Bytes>, List<List<Bytes>>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Bytes, std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
    C1(Bytes, Bytes, std::rc::Rc<Adt19>),
    C2(Bytes),
    C3(std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
    C4,
}

#[derive(Clone)]
pub enum Adt20 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt20>),
    C5(std::rc::Rc<Adt20>),
    C6,
    C7(std::rc::Rc<Adt20>, std::rc::Rc<Adt20>),
    C8(Bytes, std::rc::Rc<Adt20>, std::rc::Rc<Adt20>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, Adt22),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Option<u64>),
    C1(u64),
    C2(Adt23, List<u64>, u64),
}

#[derive(Clone)]
pub enum Adt23 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Bytes, Adt20),
    C1(Bytes),
    C2,
}

#[derive(Clone)]
pub enum Adt27 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt27, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F63(u8, Bytes),
    F207(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F63(k0, k1) => f63(*k0, k1.clone(), p0),
            Fn0::F207(k0) => f207(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F105,
    F235,
    F341,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F105 => f105(p0, p1),
            Fn1::F235 => f235(p0, p1),
            Fn1::F341 => f341(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F268(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F268(k0) => f268(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F332(Bytes, u64, Bytes, u64),
    F379(Bytes),
    F435(Bytes),
    F436(u64, Bytes),
    F465(Bytes, Bytes),
    F466(Bytes),
    F467(Bytes, u64),
    F499(Bytes),
    F500(Bytes),
    F530(bool, u64, Bytes),
    F549(Bytes),
    F588(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F332(k0, k1, k2, k3) => f332(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F379(k0) => f379(k0.clone(), p0),
            Fn3::F435(k0) => Ok(f435(k0.clone(), p0)),
            Fn3::F436(k0, k1) => f436(*k0, k1.clone(), p0),
            Fn3::F465(k0, k1) => f465(k0.clone(), k1.clone(), p0),
            Fn3::F466(k0) => f466(k0.clone(), p0),
            Fn3::F467(k0, k1) => f467(k0.clone(), *k1, p0),
            Fn3::F499(k0) => f499(k0.clone(), p0),
            Fn3::F500(k0) => f500(k0.clone(), p0),
            Fn3::F530(k0, k1, k2) => f530(*k0, *k1, k2.clone(), p0),
            Fn3::F549(k0) => f549(k0.clone(), p0),
            Fn3::F588(k0) => f588(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F373(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt25)) -> R<(u64, Adt25)> {
        match self {
            Fn4::F373(k0, k1, k2) => f373(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F378(Bytes),
    F390(u64, Bytes),
    F425(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F378(k0) => f378(k0.clone(), p0),
            Fn5::F390(k0, k1) => Ok(f390(*k0, k1.clone(), p0)),
            Fn5::F425(k0) => f425(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F417(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn6::F417(k0) => f417(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F427,
    F703,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn7::F427 => f427(p0, p1),
            Fn7::F703 => f703(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F430,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn8::F430 => f430(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F444,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn9::F444 => f444(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F447,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn10::F447 => f447(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F449,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn11::F449 => f449(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F452,
}

impl Fn12 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn12::F452 => f452(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F503(Bytes),
    F571(bool, Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn13::F503(k0) => f503(k0.clone(), p0),
            Fn13::F571(k0, k1) => f571(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F512,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn14::F512 => f512(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F515,
}

impl Fn15 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn15::F515 => f515(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F520,
}

impl Fn16 {
    pub fn apply(&self, p0: Adt28) -> R<Adt28> {
        match self {
            Fn16::F520 => f520(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F553,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F553 => f553(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F555,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F555 => f555(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F593,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F593 => f593(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F596,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F596 => f596(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F604,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F604 => f604(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F668(Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn22::F668(k0) => f668(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F670,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn23::F670 => f670(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F683,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn24::F683 => f683(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F698,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F698 => f698(p0),
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
    let m239 = {
        let a237 = f61();
        let a238 = {
            let v1: u8 = v0;
            let a235 = {
                let a211 = {
                    let a199 = {
                        let a193 = {
                            let a191 = v1;
                            let a192 = 128u8;
                            fixed_u8::bit_and(a191, a192)
                        };
                        let a194 = 128u8;
                        if equal(a193, a194) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a200 = {
                        let a197 = {
                            let a195 = v1;
                            let a196 = 64u8;
                            fixed_u8::bit_and(a195, a196)
                        };
                        let a198 = 64u8;
                        if equal(a197, a198) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a199, a200)?
                };
                let a212 = {
                    let a209 = {
                        let a203 = {
                            let a201 = v1;
                            let a202 = 32u8;
                            fixed_u8::bit_and(a201, a202)
                        };
                        let a204 = 32u8;
                        if equal(a203, a204) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a210 = {
                        let a207 = {
                            let a205 = v1;
                            let a206 = 16u8;
                            fixed_u8::bit_and(a205, a206)
                        };
                        let a208 = 16u8;
                        if equal(a207, a208) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a209, a210)?
                };
                nat_add(a211, a212)?
            };
            let a236 = {
                let a233 = {
                    let a221 = {
                        let a215 = {
                            let a213 = v1;
                            let a214 = 8u8;
                            fixed_u8::bit_and(a213, a214)
                        };
                        let a216 = 8u8;
                        if equal(a215, a216) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a222 = {
                        let a219 = {
                            let a217 = v1;
                            let a218 = 4u8;
                            fixed_u8::bit_and(a217, a218)
                        };
                        let a220 = 4u8;
                        if equal(a219, a220) {
                            4u64
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
                            let a224 = 2u8;
                            fixed_u8::bit_and(a223, a224)
                        };
                        let a226 = 2u8;
                        if equal(a225, a226) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a232 = {
                        let a229 = {
                            let a227 = v1;
                            let a228 = 1u8;
                            fixed_u8::bit_and(a227, a228)
                        };
                        let a230 = 1u8;
                        if equal(a229, a230) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a231, a232)?
                };
                nat_add(a233, a234)?
            };
            nat_add(a235, a236)?
        };
        index_list(a237, a238)
    };
    match m239 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f33(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a240 = v0.clone();
        length_bytes(a240)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h241, _) = f62(Fn0::F63(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h241
    };
    let v4: u64 = {
        let (h242, _) = {
            let (_, h243) = v3.clone();
            h243
        };
        h242
    };
    Ok(f64(List::cons(f35(v0, v4, {
        let a244 = v2;
        let a245 = v4;
        nat_sub(a244, a245)
    }), {
        let (_, h246) = {
            let (_, h247) = v3;
            h247
        };
        h246
    })))
}

pub fn f34(p251: List<Bytes>) -> Bytes {
    let m250 = p251;
    {
        let mut m250 = m250;
        loop {
            let step = {
                let v0 = m250;
                let m248 = v0;
                match m248.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m249 = v2.clone();
                        match m249.uncons() {
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
                Ok(next) => { m250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m255 = {
        let a252 = v0;
        let a253 = v1;
        let a254 = v2;
        slice_bytes(a252, a253, a254)
    };
    match m255 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f36(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m256 = v0;
    match m256.uncons() {
        None => {
            Ok(Adt11::C4(f65()))
        }
        Some((v2, v3)) => {
            if f28(v2.clone(), f8(Str::lit("check"))) {
                f66(v1)
            } else {
                if f28(v2.clone(), f8(Str::lit("next"))) {
                    f67(v1, v3)
                } else {
                    if f28(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f68(v1))
                    } else {
                        if f28(v2.clone(), f8(Str::lit("publish"))) {
                            f69(v1)
                        } else {
                            if f28(v2.clone(), f8(Str::lit("agenda"))) {
                                f70(v1)
                            } else {
                                let m257 = v3;
                                match m257.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f65()))
                                    }
                                    Some((v4, v5)) => {
                                        if f28(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f71(v1, v4)
                                        } else {
                                            if f28(v2.clone(), f8(Str::lit("read-xml"))) {
                                                f72(v1, v4)
                                            } else {
                                                if f28(v2.clone(), f8(Str::lit("read-xsd"))) {
                                                    f73(v1, v4)
                                                } else {
                                                    if f28(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                        f74(v1.clone(), v4.clone(), f75(v1, v4)?)
                                                    } else {
                                                        if f28(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                            f74(v1.clone(), v4.clone(), f76(v1, v4, false)?)
                                                        } else {
                                                            if f28(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                                f74(v1.clone(), v4.clone(), f76(v1, v4, true)?)
                                                            } else {
                                                                if f28(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                                    f74(v1.clone(), v4.clone(), f77(v1, v4, false)?)
                                                                } else {
                                                                    if f28(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                        f74(v1.clone(), v4.clone(), f77(v1, v4, true)?)
                                                                    } else {
                                                                        if f28(v2.clone(), f8(Str::lit("admit"))) {
                                                                            f78(v1, v4)
                                                                        } else {
                                                                            if f28(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                                f74(v1.clone(), v4.clone(), f79(v1, v4)?)
                                                                            } else {
                                                                                if f28(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                                    f80(v1, v4)
                                                                                } else {
                                                                                    if f28(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                        f81(v1, v4)
                                                                                    } else {
                                                                                        if f28(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                            f82(v1, v4)
                                                                                        } else {
                                                                                            if f28(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                                f83(v1, v4)
                                                                                            } else {
                                                                                                if f28(v2, f8(Str::lit("propose-encoding"))) {
                                                                                                    let m258 = v5;
                                                                                                    match m258.uncons() {
                                                                                                        None => {
                                                                                                            Ok(Adt11::C4(f65()))
                                                                                                        }
                                                                                                        Some((v6, v7)) => {
                                                                                                            f84(v1, v4, v6, v7)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    Ok(Adt11::C4(f65()))
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
}

pub fn f37(p261: List<Bytes>) -> R<Option<Bytes>> {
    let m260 = p261;
    {
        let mut m260 = m260;
        loop {
            let step = {
                let v0 = m260;
                let m259 = v0;
                match m259.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f85(v1.clone())? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m260 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m262 = v0;
    match m262.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h263, _) = v1;
                h263
            }, f38(v2))
        }
    }
}

pub fn f39(v0: Bytes) -> Bytes {
    let a268 = {
        let a264 = Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ");
        utf8_encode(a264)
    };
    let a269 = {
        let a266 = v0;
        let a267 = {
            let a265 = Str::lit(" is not\u{a}");
            utf8_encode(a265)
        };
        append_bytes(a266, a267)
    };
    append_bytes(a268, a269)
}

pub fn f40(v0: bool) -> bool {
    !v0
}

pub fn f41(v0: Bytes, p272: List<Bytes>) -> bool {
    let m271 = p272;
    {
        let mut m271 = m271;
        loop {
            let step = {
                let v1 = m271;
                let m270 = v1;
                match m270.uncons() {
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
                Ok(next) => { m271 = next; }
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
    let m273 = v1;
    match m273.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m275 = f86(match v0.clone() {
                Adt1::C0(h274, _, _) => {
                    h274
                }
            }, v2.clone());
            match m275 {
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
    f87(f53(v0, v1))
}

pub fn f47(v0: Adt13, v1: Bytes) -> Bytes {
    let m276 = f88(f89(v0, v1));
    match m276 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f48(v0: List<Adt12>, v1: Bytes) -> List<Bytes> {
    let m277 = v0;
    match m277.uncons() {
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
    if f90(v0.clone(), f8(Str::lit("v"))) {
        f35(v0.clone(), 1u64, {
            let a279 = {
                let a278 = v0;
                length_bytes(a278)
            };
            let a280 = 1u64;
            nat_sub(a279, a280)
        })
    } else {
        v0
    }
}

pub fn f51(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m281 = v0;
    match m281.uncons() {
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
    f91(f92(v0)?, f8(Str::lit("id = \"")))
}

pub fn f53(v0: Adt1, v1: Bytes) -> Bytes {
    let m283 = f86(match v0 {
        Adt1::C0(h282, _, _) => {
            h282
        }
    }, v1);
    match m283 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m284 = v2;
            match m284 {
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
    let v2: List<Adt13> = f93(v0, f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f94(v2.clone(), f50(v1.clone()));
    if f95(f96(v2), f96(v3.clone())) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" is pinned, and every admission of its registers cites it\u{a}")), List::<Bytes>::nil()))))))
    } else {
        Ok(Adt11::C3(List::cons((f8(Str::lit("model/admissions.json")), f97(Adt13::C3(std::rc::Rc::new(f98(v3))))?), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("re-cited the admissions of the verdant-engine registers at ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
    }
}

pub fn f55(v0: List<Bytes>) -> List<Bytes> {
    let m285 = v0;
    match m285.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f57(v1), f55(v2))
        }
    }
}

pub fn f56(v0: Adt1, v1: Bytes) -> bool {
    let m287 = f86(match v0 {
        Adt1::C0(h286, _, _) => {
            h286
        }
    }, v1);
    match m287 {
        None => {
            false
        }
        Some(v2) => {
            let m288 = v2;
            match m288 {
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
    f7(List::cons(f99(v0), List::cons(v1, List::<Bytes>::nil())))
}

pub fn f59(v0: Adt1, v1: Bytes) -> R<Adt11> {
    if f40(f41(f100(), match v0.clone() {
        Adt1::C0(_, _, h289) => {
            h289
        }
    })) {
        Ok(f43(f100(), f44(f7(List::cons(f8(Str::lit("git/ref/tags/")), List::cons(v1, List::<Bytes>::nil()))))))
    } else {
        let m290 = f45(v0.clone(), List::cons(f100(), List::<Bytes>::nil()));
        match m290.uncons() {
            None => {
                let v6: Adt13 = {
                    let m292 = f89({
                        let m291 = f46(v0.clone(), f100())?;
                        match m291 {
                            None => {
                                Adt13::C5
                            }
                            Some(v4) => {
                                v4
                            }
                        }
                    }, f8(Str::lit("object")));
                    match m292 {
                        None => {
                            Adt13::C5
                        }
                        Some(v5) => {
                            v5
                        }
                    }
                };
                if f28(f47(v6.clone(), f8(Str::lit("type"))), f8(Str::lit("tag"))) {
                    if f40(f41(f101(), match v0.clone() {
                        Adt1::C0(_, _, h293) => {
                            h293
                        }
                    })) {
                        Ok(f43(f101(), f44(f7(List::cons(f8(Str::lit("git/tags/")), List::cons(f47(v6, f8(Str::lit("sha"))), List::<Bytes>::nil()))))))
                    } else {
                        let m294 = f45(v0.clone(), List::cons(f101(), List::<Bytes>::nil()));
                        match m294.uncons() {
                            None => {
                                f102(v0.clone(), v1, {
                                    let m296 = f89({
                                        let m295 = f46(v0, f101())?;
                                        match m295 {
                                            None => {
                                                Adt13::C5
                                            }
                                            Some(v9) => {
                                                v9
                                            }
                                        }
                                    }, f8(Str::lit("object")));
                                    match m296 {
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
                    f102(v0, v1, f47(v6, f8(Str::lit("sha"))))
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

pub fn f62(v0: Fn0, p301: u64, p302: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m300 = (p301, p302);
    {
        let mut m300 = m300;
        loop {
            let step = {
                let (v1, v2) = m300;
                let m297 = v1;
                if m297 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m297 - 1;
                    let m299 = {
                        let c298: Fn0 = v0.clone();
                        c298.apply(v2.clone())?
                    };
                    match m299 {
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
                Ok(next) => { m300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f103(v1, v0, v2)
}

pub fn f64(v0: List<Bytes>) -> List<Bytes> {
    f104(Fn1::F105, List::<Bytes>::nil(), v0)
}

pub fn f65() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-engine | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | read-xml FILE | read-xsd FILE | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f66(v0: Adt1) -> R<Adt11> {
    let m303 = f45(v0.clone(), f106());
    match m303.uncons() {
        None => {
            let m304 = f107(v0.clone(), f106())?;
            match m304.uncons() {
                None => {
                    let m307 = f108(match v0.clone() {
                        Adt1::C0(_, h305, _) => {
                            h305
                        }
                    }, f109(f110(), f111(match v0.clone() {
                        Adt1::C0(_, h306, _) => {
                            h306
                        }
                    }, false)));
                    match m307.uncons() {
                        None => {
                            let m309 = f45(v0.clone(), f111(match v0.clone() {
                                Adt1::C0(_, h308, _) => {
                                    h308
                                }
                            }, true));
                            match m309.uncons() {
                                None => {
                                    let m310 = f45(v0.clone(), f112());
                                    match m310.uncons() {
                                        None => {
                                            let m311 = f113(v0.clone())?;
                                            match m311 {
                                                None => {
                                                    let m313 = f114(v0.clone(), f111(match v0.clone() {
                                                        Adt1::C0(_, h312, _) => {
                                                            h312
                                                        }
                                                    }, true))?;
                                                    match m313 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f115(Adt15::C0(Adt14::C0(f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f116(v0.clone())?, f117(f93(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f52(f53(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f117(f93(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f93(v0.clone(), f8(Str::lit("model/admissions.json")))?, f93(v0.clone(), f8(Str::lit("model/strata.json")))?, f118(f93(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f111(match v0 {
                                                                Adt1::C0(_, h314, _) => {
                                                                    h314
                                                                }
                                                            }, true)))?;
                                                            let (h315, _) = v13;
                                                            if h315 {
                                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h316) = v13;
                                                                    h316
                                                                }))
                                                            } else {
                                                                Ok(Adt11::C4({
                                                                    let (_, h317) = v13;
                                                                    h317
                                                                }))
                                                            }
                                                        }
                                                        Some(v12) => {
                                                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: classical choice is an assumption, and ")), List::cons(v12, List::cons(f8(Str::lit(" depends on it\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                    }
                                                }
                                                Some(v11) => {
                                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: a negated guard in a generated workflow is one command, because `if !` negates only the first command of a chain: ")), List::cons(v11, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("register is not JSON: ")), List::cons(v3, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f67(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f119(match v0.clone() {
        Adt1::C0(_, h318, _) => {
            h318
        }
    }, f120())) {
        Ok(Adt11::C1(List::cons(f120(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f121(f122(f120(), f123(match v0.clone() {
            Adt1::C0(_, h319, _) => {
                h319
            }
        }, f120())))?;
        let m320 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m320.uncons() {
            None => {
                let m321 = f124(v0.clone(), f125(f92(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f126(v2, v1.clone()))?;
                match m321 {
                    None => {
                        f127(v0, v1)
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

pub fn f68(v0: Adt1) -> Adt11 {
    if f40(f41(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h322) => {
            h322
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f128(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f40(f41(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h323) => {
                h323
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f128(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f69(v0: Adt1) -> R<Adt11> {
    let m324 = f129(v0.clone());
    match m324.uncons() {
        None => {
            let m325 = f109(f45(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f130(), List::<Bytes>::nil())))), f131(v0.clone())?);
            match m325.uncons() {
                None => {
                    Ok(Adt11::C3(f132(f133(v0.clone())?, List::cons((f134(), f135()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f136()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f137()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f138()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f139()), List::cons((f8(Str::lit("docs/catalog.md")), f140(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f141(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f142(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f143()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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

pub fn f70(v0: Adt1) -> R<Adt11> {
    let m326 = f129(v0.clone());
    match m326.uncons() {
        None => {
            let m327 = f109(f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f131(v0.clone())?);
            match m327.uncons() {
                None => {
                    let v5: Adt16 = f144(v0.clone())?;
                    let v6: List<Adt13> = f93(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f109(f145(f146(match v5.clone() {
                        Adt16::C0(h328, _, _) => {
                            h328
                        }
                    }, f147(f93(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f148(v6.clone())), f149(f146(match v5.clone() {
                        Adt16::C0(h329, _, _) => {
                            h329
                        }
                    }, f147(f93(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f150(v5.clone(), v6, f151(v5, f152(v0.clone(), f153(v0)?)?))?)))))
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

pub fn f71(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m330 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m330.uncons() {
        None => {
            let v4: Bytes = f53(v0.clone(), v1);
            let v5: List<List<Bytes>> = f154(v4.clone())?;
            if f40(f95(f155(v5.clone()), f156())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f157(Str::lit("id"), v6.clone())?, List::cons(f157(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f157(Str::lit("artifact"), f158())?, List::cons(f157(Str::lit("checksum"), f159(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f158(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f160(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f161(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f162(f163(f164(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f72(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m331 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m331.uncons() {
        None => {
            let m332 = f165(f53(v0, v1))?;
            match m332 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f166(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f167(f168(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f73(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m333 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m333.uncons() {
        None => {
            let m334 = f169(f53(v0, v1))?;
            match m334 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f167({
                        let (h335, _) = v5;
                        h335
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h336) = v5;
                        h336
                    }, List::<Bytes>::nil())))))))
                }
                Err(v4) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v4, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f74(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m337 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m337.uncons() {
        None => {
            if f170(f92(f53(v0, v1))?)? {
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

pub fn f75(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m338 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m338.uncons() {
        None => {
            let m339 = f171(f92(f53(v0.clone(), v1.clone()))?)?;
            match m339 {
                None => {
                    Ok(Adt11::C4(f172()))
                }
                Some(v4) => {
                    let m340 = f173(f92(f53(v0, v1))?)?;
                    match m340 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f162(f38(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f174(v5)))
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

pub fn f76(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m341 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m341.uncons() {
        None => {
            let v5: Bytes = f175(if v2 {
                f176()
            } else {
                f177()
            }, f178(v1.clone())?);
            if f40(f179(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m342 = f180(f92(f53(v0.clone(), v1.clone()))?, v2)?;
                match m342 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a344 = {
                            let a343 = v6;
                            length_list(a343)
                        };
                        let a345 = 0u64;
                        if nat_eq(a344, a345) {
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

pub fn f77(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m346 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m346.uncons() {
        None => {
            let v5: List<Bytes> = f92(f53(v0.clone(), v1.clone()))?;
            if f40(f179(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f181(v5)?
                } else {
                    f182(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f175(f183(), f178(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f175(f183(), f178(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f184()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f78(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m347 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m347.uncons() {
        None => {
            let v4: List<Bytes> = f92(f53(v0.clone(), v1))?;
            let m348 = f185(v4.clone(), f8(Str::lit("requirement")))?;
            match m348 {
                None => {
                    Ok(Adt11::C4(f184()))
                }
                Some(v5) => {
                    let m349 = f185(v4, f8(Str::lit("admission")))?;
                    match m349 {
                        None => {
                            Ok(Adt11::C4(f184()))
                        }
                        Some(v6) => {
                            let v7: List<Adt13> = f93(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt13> = f93(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f109(f117(v5.clone(), f8(Str::lit("id"))), f117(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f121(f109(f117(v7.clone(), f8(Str::lit("id"))), f117(v8.clone(), f8(Str::lit("construct")))))?;
                            let m350 = f186(f148(v6.clone()), f147(f93(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m350 {
                                None => {
                                    let a352 = {
                                        let a351 = v9.clone();
                                        length_list(a351)
                                    };
                                    let a353 = 0u64;
                                    if nat_eq(a352, a353) {
                                        Ok(Adt11::C4(f184()))
                                    } else {
                                        if f40({
                                            let a356 = {
                                                let a354 = f121(f109(v9.clone(), v10.clone()))?;
                                                length_list(a354)
                                            };
                                            let a357 = {
                                                let a355 = f109(v9.clone(), v10);
                                                length_list(a355)
                                            };
                                            nat_eq(a356, a357)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f188(f8(Str::lit("model/requirements.json")), f189(v7, v5))?, List::cons(f188(f8(Str::lit("model/admissions.json")), f189(v8, v6))?, List::cons(f161(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f162(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f187(v11))
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

pub fn f79(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m358 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m358.uncons() {
        None => {
            let m359 = f190(f92(f53(v0, v1))?)?;
            match m359 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f162(f38(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f80(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m360 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m360.uncons() {
        None => {
            let m361 = f191(v0.clone(), v1.clone())?;
            match m361 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m362 = f192(v4.clone(), f8(Str::lit("schema")));
                    match m362 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f193(v0, f194(v5.clone())?, v5, f195(v4.clone(), f8(Str::lit("name"))), f196(f197(f192(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f81(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m363 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f130(), List::<Bytes>::nil())));
    match m363.uncons() {
        None => {
            let m364 = f198(f92(f53(v0.clone(), v1))?)?;
            match m364 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt13> = f93(v0, f130())?;
                    let v6: List<Bytes> = f117(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f121(f117(v5.clone(), f8(Str::lit("at"))))?;
                    let a366 = {
                        let a365 = v6.clone();
                        length_list(a365)
                    };
                    let a367 = 0u64;
                    if if nat_eq(a366, a367) {
                        true
                    } else {
                        f40({
                            let a370 = {
                                let a368 = f121(f109(v6.clone(), v7.clone()))?;
                                length_list(a368)
                            };
                            let a371 = {
                                let a369 = f109(v6.clone(), v7);
                                length_list(a369)
                            };
                            nat_eq(a370, a371)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f188(f130(), f189(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f162(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f82(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m372 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m372.uncons() {
        None => {
            let m373 = f46(v0, v1)?;
            match m373 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f199(f200(Some(v4))) {
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

pub fn f83(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m374 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m374.uncons() {
        None => {
            let m375 = f46(v0.clone(), v1.clone())?;
            match m375 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f40(f28(f47(v4.clone(), f8(Str::lit("schema"))), f201())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt13 = {
                            let m376 = f89(v4.clone(), f8(Str::lit("source")));
                            match m376 {
                                None => {
                                    Adt13::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f88(f89(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f175(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f175(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m377 = v8;
                        match m377 {
                            None => {
                                f202(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f40(f28(f47(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f202(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f175(v9, f178(f47(v7.clone(), f8(Str::lit("url"))))?);
                                    if f40(f41(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h378) => {
                                            h378
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f47(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m379 = f45(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m379.uncons() {
                                            None => {
                                                if f28(f159(f53(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f202(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f84(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m380 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m380.uncons() {
        None => {
            let m381 = f191(v0.clone(), v1.clone())?;
            match m381 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f193(v0.clone(), f53(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f85(v0: Bytes) -> R<bool> {
    f203(v0.clone(), 0u64, {
        let a382 = v0;
        length_bytes(a382)
    })
}

pub fn f86(p387: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m386 = p387;
    {
        let mut m386 = m386;
        loop {
            let step = {
                let v0 = m386;
                let m383 = v0;
                match m383.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h384, _) = v2;
                            h384
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h385) = v2;
                                h385
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f87(v0: Bytes) -> R<Option<Adt13>> {
    let v1: Bytes = f204(v0)?;
    let m391 = f205(v1.clone(), 0u64, {
        let a389 = {
            let a388 = v1.clone();
            length_bytes(a388)
        };
        let a390 = 2u64;
        nat_add(a389, a390)?
    }, 0u64)?;
    match m391 {
        None => {
            Ok(None::<Adt13>)
        }
        Some(v2) => {
            let a394 = {
                let (_, h392) = v2;
                h392
            };
            let a395 = {
                let a393 = v1;
                length_bytes(a393)
            };
            if nat_eq(a394, a395) {
                Ok(Some({
                    let (h396, _) = v2;
                    h396
                }))
            } else {
                Ok(None::<Adt13>)
            }
        }
    }
}

pub fn f88(v0: Option<Adt13>) -> Option<Bytes> {
    let m397 = v0;
    match m397 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m398 = v1;
            match m398 {
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

pub fn f89(p404: Adt13, v1: Bytes) -> Option<Adt13> {
    let m403 = p404;
    {
        let mut m403 = m403;
        loop {
            let step = {
                let v0 = m403;
                let m399 = v0;
                match m399 {
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
                    Adt13::C4(r400) => {
                        let v5 = (*r400).clone();
                        Ok::<Adt13, Option<Adt13>>(v5)
                    }
                    Adt13::C5 => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C6(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C7(v2, r401, r402) => {
                        let v3 = (*r401).clone();
                        let v4 = (*r402).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt13, Option<Adt13>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Adt13>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Bytes, v1: Bytes) -> bool {
    let m409 = {
        let a406 = v0;
        let a407 = 0u64;
        let a408 = {
            let a405 = v1.clone();
            length_bytes(a405)
        };
        slice_bytes(a406, a407, a408)
    };
    match m409 {
        None => {
            false
        }
        Some(v2) => {
            let a410 = v2;
            let a411 = v1;
            equal(a410, a411)
        }
    }
}

pub fn f91(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m412 = v0;
    match m412.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m413 = f206(v2, v1.clone())?;
            match m413 {
                None => {
                    f91(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f91(v3, v1)?))
                }
            }
        }
    }
}

pub fn f92(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a414 = v0.clone();
        length_bytes(a414)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h415, _) = f62(Fn0::F207(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h415
    };
    let v3: u64 = {
        let (h416, _) = {
            let (_, h417) = v2.clone();
            h417
        };
        h416
    };
    let v4: List<Bytes> = {
        let (_, h418) = {
            let (_, h419) = v2;
            h419
        };
        h418
    };
    let a420 = v3;
    let a421 = v1;
    if nat_lt(a420, a421) {
        Ok(f64(List::cons(f35(v0, v3, {
            let a422 = v1;
            let a423 = v3;
            nat_sub(a422, a423)
        }), v4)))
    } else {
        Ok(f64(v4))
    }
}

pub fn f93(v0: Adt1, v1: Bytes) -> R<List<Adt13>> {
    Ok(f200(f46(v0, v1)?))
}

pub fn f94(v0: List<Adt13>, v1: Bytes) -> List<Adt13> {
    let m424 = v0;
    match m424.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f208(v2, v1.clone()), f94(v3, v1))
        }
    }
}

pub fn f95(p429: List<Bytes>, p430: List<Bytes>) -> bool {
    let m428 = (p429, p430);
    {
        let mut m428 = m428;
        loop {
            let step = {
                let (v0, v1) = m428;
                let m425 = v0;
                match m425.uncons() {
                    None => {
                        let m426 = v1;
                        match m426.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m427 = v1;
                        match m427.uncons() {
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
                Ok(next) => { m428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: List<Adt13>) -> List<Bytes> {
    let m431 = v0;
    match m431.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m432 = f89(v1, f8(Str::lit("std")));
            match m432 {
                None => {
                    f96(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f96(v2))
                }
            }
        }
    }
}

pub fn f97(v0: Adt13) -> R<Bytes> {
    let a433 = f209(v0)?;
    let a434 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a433, a434))
}

pub fn f98(v0: List<Adt13>) -> Adt13 {
    let m435 = v0;
    match m435.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f98(v2)))
        }
    }
}

pub fn f99(v0: Adt12) -> Bytes {
    let m436 = v0;
    match m436 {
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

pub fn f100() -> Bytes {
    f8(Str::lit("target/engine/ref.json"))
}

pub fn f101() -> Bytes {
    f8(Str::lit("target/engine/tag.json"))
}

pub fn f102(v0: Adt1, v1: Bytes, v2: Bytes) -> R<Adt11> {
    let a438 = {
        let a437 = v2.clone();
        length_bytes(a437)
    };
    let a439 = 0u64;
    if nat_eq(a438, a439) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m440 = f210(v0.clone(), f49())?;
        match m440 {
            None => {
                if f40(f41(f213(), match v0.clone() {
                    Adt1::C0(_, _, h441) => {
                        h441
                    }
                })) {
                    Ok(Adt11::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f213()))
                } else {
                    let m442 = f45(v0.clone(), List::cons(f214(), f215(f49())?));
                    match m442.uncons() {
                        None => {
                            Ok(Adt11::C3(f216(v0.clone(), f49(), v1.clone(), v2.clone(), f217(f53(v0, f214()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt11::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f43(f211(v3.clone())?, f212(v3, v2)))
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h443, _) = v2.clone();
        h443
    };
    let v4: u64 = {
        let (h444, _) = {
            let (_, h445) = v2.clone();
            h445
        };
        h444
    };
    let v5: List<Bytes> = {
        let (_, h446) = {
            let (_, h447) = v2;
            h447
        };
        h446
    };
    let m448 = f218(v0.clone(), v3);
    match m448 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a449 = v6;
            let a450 = v1;
            if equal(a449, a450) {
                Ok(Some(({
                    let a451 = v3;
                    let a452 = 1u64;
                    nat_add(a451, a452)?
                }, ({
                    let a453 = v3;
                    let a454 = 1u64;
                    nat_add(a453, a454)?
                }, List::cons(f35(v0, v4, {
                    let a455 = v3;
                    let a456 = v4;
                    nat_sub(a455, a456)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a457 = v3;
                    let a458 = 1u64;
                    nat_add(a457, a458)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f104(v0: Fn1, p462: List<Bytes>, p463: List<Bytes>) -> List<Bytes> {
    let m461 = (p462, p463);
    {
        let mut m461 = m461;
        loop {
            let step = {
                let (v1, v2) = m461;
                let m459 = v2;
                match m459.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c460: Fn1 = v0.clone();
                            c460.apply(v1, v3)
                        }, v4))
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

pub fn f105(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f106() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f158(), List::<Bytes>::nil())))))))
}

pub fn f107(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m464 = v1;
    match m464.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f56(v0.clone(), v2.clone()) {
                f179(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m465 = f46(v0.clone(), v2.clone())?;
                match m465 {
                    None => {
                        Ok(List::cons(v2, f107(v0, v3)?))
                    }
                    Some(_) => {
                        f107(v0, v3)
                    }
                }
            } else {
                f107(v0, v3)
            }
        }
    }
}

pub fn f108(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m466 = v1;
    match m466.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f119(v0.clone(), v2.clone()) {
                f108(v0, v3)
            } else {
                List::cons(v2, f108(v0, v3))
            }
        }
    }
}

pub fn f109(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m467 = v0;
    match m467.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f109(v3, v1))
        }
    }
}

pub fn f110() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f111(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m468 = v0;
    match m468.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f109(f219({
                let (h469, _) = v2;
                h469
            }, {
                let (_, h470) = v2;
                h470
            }, v1), f111(v3, v1))
        }
    }
}

pub fn f112() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f113(v0: Adt1) -> R<Option<Bytes>> {
    f220(f221(v0, f112())?)
}

pub fn f114(v0: Adt1, p474: List<Bytes>) -> R<Option<Bytes>> {
    let m473 = p474;
    {
        let mut m473 = m473;
        loop {
            let step = {
                let v1 = m473;
                let m471 = v1;
                match m471.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m472 = f222(f53(v0.clone(), v2.clone()), f223())?;
                        match m472 {
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
                Ok(next) => { m473 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: Adt15) -> R<(bool, Bytes)> {
    let v1: Adt14 = match v0.clone() {
        Adt15::C0(h475, _, _, _, _) => {
            h475
        }
    };
    let v2: Adt17 = f224(v1.clone())?;
    let v3: List<Bytes> = f225(v1.clone());
    let v4: List<Bytes> = f117(match v0.clone() {
        Adt15::C0(_, _, h476, _, _) => {
            h476
        }
    }, f8(Str::lit("path")));
    let m478 = f226(v2, List::<u64>::nil(), f227(v1.clone(), match v0.clone() {
        Adt15::C0(_, h477, _, _, _) => {
            h477
        }
    }, 0u64)?);
    match m478 {
        None => {
            if f229(f230(match v0.clone() {
                Adt15::C0(_, _, _, _, h480) => {
                    h480
                }
            }, f109(match v0.clone() {
                Adt15::C0(_, _, _, _, h481) => {
                    h481
                }
            }, v4.clone()))?, f230(v4.clone(), f109(match v0.clone() {
                Adt15::C0(_, _, _, _, h482) => {
                    h482
                }
            }, v4.clone()))?, f231(match v0.clone() {
                Adt15::C0(_, _, h483, _, _) => {
                    h483
                }
            })) {
                let m487 = f232(f230(match v0.clone() {
                    Adt15::C0(_, _, _, h484, _) => {
                        h484
                    }
                }, f109(v3.clone(), match v0.clone() {
                    Adt15::C0(_, _, _, h485, _) => {
                        h485
                    }
                }))?, f230(v3.clone(), f109(v3.clone(), match v0.clone() {
                    Adt15::C0(_, _, _, h486, _) => {
                        h486
                    }
                }))?);
                match m487 {
                    None => {
                        let m490 = f233(f117(match v0 {
                            Adt15::C0(_, h489, _, _, _) => {
                                h489
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m490 {
                            None => {
                                let m491 = f233(v4, List::<Bytes>::nil());
                                match m491 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f162(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f228(f109(v3, match v0 {
                            Adt15::C0(_, _, _, h488, _) => {
                                h488
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f228(match v1 {
                Adt14::C0(_, _, _, _, h479) => {
                    h479
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f116(v0: Adt1) -> R<List<Bytes>> {
    let m492 = f154(f53(v0, f158()))?;
    match m492.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f163(v2))
        }
    }
}

pub fn f117(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m493 = v0;
    match m493.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f47(v2, v1.clone()), f117(v3, v1))
        }
    }
}

pub fn f118(v0: List<Adt13>) -> List<Bytes> {
    let m494 = v0;
    match m494.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m495 = f88(Some(v1));
            match m495 {
                None => {
                    f118(v2)
                }
                Some(v3) => {
                    List::cons(v3, f118(v2))
                }
            }
        }
    }
}

pub fn f119(p499: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m498 = p499;
    {
        let mut m498 = m498;
        loop {
            let step = {
                let v0 = m498;
                let m496 = v0;
                match m496.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h497, _) = v2;
                            h497
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f121(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f234(f104(Fn1::F235, List::<Bytes>::nil(), f236(v0)?)))
}

pub fn f122(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m500 = v1;
    match m500.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f179(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f175(v0.clone(), v2), f122(v0, v3))
            } else {
                f122(v0, v3)
            }
        }
    }
}

pub fn f123(p505: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m504 = p505;
    {
        let mut m504 = m504;
        loop {
            let step = {
                let v0 = m504;
                let m501 = v0;
                match m501.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h502, _) = v2;
                            h502
                        }, v1.clone()) {
                            let (_, h503) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h503)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m504 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: Adt1, v1: List<List<Bytes>>, p508: List<Bytes>) -> R<Option<Bytes>> {
    let m507 = p508;
    {
        let mut m507 = m507;
        loop {
            let step = {
                let v2 = m507;
                let m506 = v2;
                match m506.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f237(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(p511: List<Bytes>, p512: List<Bytes>, p513: List<List<Bytes>>) -> List<List<Bytes>> {
    let m510 = (p511, (p512, p513));
    {
        let mut m510 = m510;
        loop {
            let step = {
                let (v0, (v1, v2)) = m510;
                let m509 = v0;
                match m509.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f238(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f28(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f238(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m510 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m514 = v0;
    match m514.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f126(v3, v1)
            } else {
                List::cons(v2, f126(v3, v1))
            }
        }
    }
}

pub fn f127(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f119(match v0.clone() {
        Adt1::C0(_, h515, _) => {
            h515
        }
    }, f239())) {
        Ok(Adt11::C1(List::cons(f239(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f126(f121(f122(f239(), f123(match v0.clone() {
            Adt1::C0(_, h516, _) => {
                h516
            }
        }, f239())))?, v1.clone());
        let m517 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m517.uncons() {
            None => {
                let v5: List<Bytes> = f117(f93(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m518 = f45(v0.clone(), f240(v0.clone(), v2.clone(), v5.clone())?);
                match m518.uncons() {
                    None => {
                        let m519 = f241(v0.clone(), v2, v5)?;
                        match m519 {
                            None => {
                                f242(v0, v1)
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

pub fn f128(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f129(v0: Adt1) -> List<Bytes> {
    if f119(match v0.clone() {
        Adt1::C0(_, h520, _) => {
            h520
        }
    }, f177()) {
        if f119(match v0 {
            Adt1::C0(_, h521, _) => {
                h521
            }
        }, f176()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f176(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f177(), List::<Bytes>::nil())
    }
}

pub fn f130() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f131(v0: Adt1) -> R<List<Bytes>> {
    Ok(f45(v0.clone(), List::cons(f158(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f153(v0)?)))))
}

pub fn f132(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m522 = v0;
    match m522.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f132(v3, v1))
        }
    }
}

pub fn f133(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m523 = f93(v0.clone(), f130())?;
    match m523.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f243(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f134() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f135() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f136() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f244(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # main may have moved while this ran, so refresh checks the new pull request at once.\u{a}            gh workflow run refresh.yml --repo \"$GITHUB_REPOSITORY\" --ref main || echo \"::warning::refresh could not be started; it runs on the next push to main\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f137() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: operate starts this workflow\u{a}# right after it opens one, so every regeneration takes this one path.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f244(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f138() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}(cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log\" 2>&1\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}"))
}

pub fn f139() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f140(v0: Adt1) -> R<Bytes> {
    let v1: Adt16 = f144(v0.clone())?;
    let v2: List<Adt13> = f93(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f151(v1.clone(), f152(v0.clone(), f153(v0.clone())?)?);
    let v4: List<List<Bytes>> = f164(f154(f53(v0, f158()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f245(v4.clone(), f163(v4), f246(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f150(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f141(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f247(f144(v0.clone())?, f152(v0.clone(), f153(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f142(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f248(f93(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f249(f93(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f143() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f144(v0: Adt1) -> R<Adt16> {
    Ok(Adt16::C0(f116(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f117(f93(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f145(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m524 = v0;
    match m524.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f145(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f145(v3, v1))
            }
        }
    }
}

pub fn f146(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m525 = v0;
    match m525.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f146(v3, v1)
            } else {
                List::cons(v2, f146(v3, v1))
            }
        }
    }
}

pub fn f147(v0: List<Adt13>) -> List<Bytes> {
    let m526 = v0;
    match m526.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f109(f118(f200(f89(v1, f8(Str::lit("defers"))))), f147(v2))
        }
    }
}

pub fn f148(v0: List<Adt13>) -> List<Bytes> {
    let m527 = v0;
    match m527.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m528 = f89(v1, f8(Str::lit("std")));
            match m528 {
                None => {
                    f148(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("row"))), f148(v2))
                }
            }
        }
    }
}

pub fn f149(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m529 = v0;
    match m529.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f149(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f149(v3, v1))
            }
        }
    }
}

pub fn f150(v0: Adt16, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f132(f250(v2.clone()), f251(f246(v0, v1.clone(), v2)?));
    f252(f121(f253(v1))?, v3.clone(), {
        let a531 = {
            let a530 = v3;
            length_list(a530)
        };
        let a532 = 1u64;
        nat_add(a531, a532)?
    })
}

pub fn f151(v0: Adt16, v1: List<Adt13>) -> List<Adt13> {
    let m533 = v1;
    match m533.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            if if f254(v2.clone(), f8(Str::lit("subject"))) {
                f255(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f151(v0, v3))
            } else {
                f151(v0, v3)
            }
        }
    }
}

pub fn f152(v0: Adt1, v1: List<Bytes>) -> R<List<Adt13>> {
    let m534 = v1;
    match m534.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v2, v3)) => {
            let m535 = f180(f92(f53(v0.clone(), v2.clone()))?, f90(v2, f176()))?;
            match m535 {
                None => {
                    f152(v0, v3)
                }
                Some(v4) => {
                    Ok(f189(v4, f152(v0, v3)?))
                }
            }
        }
    }
}

pub fn f153(v0: Adt1) -> R<List<Bytes>> {
    Ok(f109(f121(f256(f177(), f123(match v0.clone() {
        Adt1::C0(_, h536, _) => {
            h536
        }
    }, f177())))?, f121(f256(f176(), f123(match v0 {
        Adt1::C0(_, h537, _) => {
            h537
        }
    }, f176())))?))
}

pub fn f154(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt18 = f257(v0.clone(), 0u64, {
        let a538 = v0;
        length_bytes(a538)
    }, Adt18::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt18 = {
        let a541 = {
            let a540 = match v1.clone() {
                Adt18::C0(_, _, h539, _, _) => {
                    h539
                }
            };
            length_bytes(a540)
        };
        let a542 = 0u64;
        if if nat_eq(a541, a542) {
            let m544 = match v1.clone() {
                Adt18::C0(_, _, _, h543, _) => {
                    h543
                }
            };
            match m544.uncons() {
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
            f258(v1)
        }
    };
    Ok(f259(match v4 {
        Adt18::C0(_, _, _, _, h545) => {
            h545
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f155(v0: List<List<Bytes>>) -> List<Bytes> {
    let m546 = v0;
    match m546.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f156() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f157(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f260(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f158() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f159(v0: Bytes) -> R<Bytes> {
    Ok(f261(f262(v0)?))
}

pub fn f160(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f263(f125(f92(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f264(f265(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f161(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f97(Adt13::C3(std::rc::Rc::new(f266(f121(f109(v1, f118(f93(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f162(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m547 = f267(Fn2::F268(v1), None::<Bytes>, v0);
    match m547 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f163(v0: List<List<Bytes>>) -> List<Bytes> {
    let m548 = v0;
    match m548.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m549 = v1;
            match m549.uncons() {
                None => {
                    f163(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f163(v2))
                }
            }
        }
    }
}

pub fn f164(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m550 = v0;
    match m550.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f165(v0: Bytes) -> R<Result<Adt19, Bytes>> {
    let v1: Bytes = if f269(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f35(v0.clone(), 3u64, {
            let a552 = {
                let a551 = v0;
                length_bytes(a551)
            };
            let a553 = 3u64;
            nat_sub(a552, a553)
        })
    } else {
        v0
    };
    let v2: Bytes = f270(v1)?;
    let v3: u64 = {
        let a554 = v2.clone();
        length_bytes(a554)
    };
    let v4: u64 = f271(v2.clone())?;
    let a555 = v4;
    let a556 = v3;
    if nat_lt(a555, a556) {
        Ok(Err::<Adt19, Bytes>(f272(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m557 = f273(v2.clone())?;
        match m557 {
            Ok(v6) => {
                let m560 = f274(v2.clone(), v6, {
                    let a558 = v3;
                    let a559 = 2u64;
                    nat_add(a558, a559)?
                })?;
                match m560 {
                    Ok(v8) => {
                        if f275(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(Err::<Adt19, Bytes>(f272(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a561 = f276(v2.clone(), v8)?;
                            let a562 = 60u64;
                            if !nat_eq(a561, a562) {
                                Ok(Err::<Adt19, Bytes>(f272(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            } else {
                                let m565 = f277(v2.clone(), v8, {
                                    let a563 = v3;
                                    let a564 = 2u64;
                                    nat_add(a563, a564)?
                                }, 0u64)?;
                                match m565 {
                                    Ok(v10) => {
                                        let m569 = f274(v2, {
                                            let (_, h566) = v10;
                                            h566
                                        }, {
                                            let a567 = v3;
                                            let a568 = 2u64;
                                            nat_add(a567, a568)?
                                        })?;
                                        match m569 {
                                            Ok(v12) => {
                                                let a570 = v12;
                                                let a571 = v3;
                                                if nat_eq(a570, a571) {
                                                    Ok(Ok::<Adt19, Bytes>({
                                                        let (h572, _) = v10;
                                                        h572
                                                    }))
                                                } else {
                                                    Ok(Err::<Adt19, Bytes>(f272(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt19, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt19, Bytes>(v9))
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt19, Bytes>(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(Err::<Adt19, Bytes>(v5))
            }
        }
    }
}

pub fn f166(v0: Adt19) -> Bytes {
    let m573 = v0;
    match m573 {
        Adt19::C0(v1, _, _) => {
            v1
        }
        Adt19::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt19::C2(_) => {
            Bytes::lit(&[])
        }
        Adt19::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt19::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f167(v0: u64) -> R<Bytes> {
    f278(f279(), 10u64, v0)
}

pub fn f168(v0: Adt19) -> R<u64> {
    let m574 = v0;
    match m574 {
        Adt19::C0(_, _, r579) => {
            let v3 = (*r579).clone();
            let a575 = 1u64;
            let a576 = f168(v3)?;
            nat_add(a575, a576)
        }
        Adt19::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt19::C2(_) => {
            Ok(0u64)
        }
        Adt19::C3(r580, r581) => {
            let v8 = (*r580).clone();
            let v9 = (*r581).clone();
            let a577 = f168(v8)?;
            let a578 = f168(v9)?;
            nat_add(a577, a578)
        }
        Adt19::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f169(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m582 = f165(v0)?;
    match m582 {
        Ok(v2) => {
            let m583 = v2;
            match m583 {
                Adt19::C0(v3, r595, r596) => {
                    let v4 = (*r595).clone();
                    let v5 = (*r596).clone();
                    let m584 = f280(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m584 {
                        Ok(v7) => {
                            let m585 = f281(v7.clone(), v3)?;
                            match m585 {
                                Ok(v9) => {
                                    if f282(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m586 = f283(v4, f8(Str::lit("targetNamespace")));
                                            match m586 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m587 = f284(v5, v7, v11)?;
                                        match m587 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f285(v13.clone());
                                                let m588 = f286(v14.clone());
                                                match m588 {
                                                    None => {
                                                        let m589 = f287(v13.clone(), v14.clone());
                                                        match m589 {
                                                            None => {
                                                                let m593 = f288({
                                                                    let a591 = {
                                                                        let a590 = v14.clone();
                                                                        length_list(a590)
                                                                    };
                                                                    let a592 = 1u64;
                                                                    nat_add(a591, a592)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m593 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a594 = v14;
                                                                            length_list(a594)
                                                                        }, f265(f289(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<(u64, Bytes), Bytes>(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(Err::<(u64, Bytes), Bytes>(v12))
                                            }
                                        }
                                    } else {
                                        Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(Err::<(u64, Bytes), Bytes>(v6))
                        }
                    }
                }
                Adt19::C1(_, _, _) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt19::C2(_) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt19::C3(_, _) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt19::C4 => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f170(p602: List<Bytes>) -> R<bool> {
    let m601 = p602;
    {
        let mut m601 = m601;
        loop {
            let step = {
                let v0 = m601;
                let m597 = v0;
                match m597.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a599 = {
                            let a598 = f217(v1)?;
                            length_bytes(a598)
                        };
                        let a600 = 0u64;
                        if nat_eq(a599, a600) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m603 = v0;
    match m603.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f217(v1)?;
            let a605 = {
                let a604 = v3.clone();
                length_bytes(a604)
            };
            let a606 = 0u64;
            if nat_eq(a605, a606) {
                f171(v2)
            } else {
                let m607 = f87(v3.clone())?;
                match m607 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        if f290(f47(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m608 = f171(v2)?;
                            match m608 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f291(v5), {
                                        let a609 = v3;
                                        let a610 = Bytes::lit(&[0x0a]);
                                        append_bytes(a609, a610)
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

pub fn f172() -> Bytes {
    let a611 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a611)
}

pub fn f173(p616: List<Bytes>) -> R<Option<Bytes>> {
    let m615 = p616;
    {
        let mut m615 = m615;
        loop {
            let step = {
                let v0 = m615;
                let m612 = v0;
                match m612.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m613 = f87(f217(v1)?)?;
                        match m613 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m614 = f89(v3.clone(), f8(Str::lit("source")));
                                match m614 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f292(f47(v4.clone(), f8(Str::lit("redistribution"))), f47(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m615 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(v0: Bytes) -> Bytes {
    let a621 = {
        let a617 = Str::lit("binding ");
        utf8_encode(a617)
    };
    let a622 = {
        let a619 = v0;
        let a620 = {
            let a618 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a618)
        };
        append_bytes(a619, a620)
    };
    append_bytes(a621, a622)
}

pub fn f175(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f176() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f177() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f178(v0: Bytes) -> R<Bytes> {
    Ok(f34(f33(v0, 47u8)?))
}

pub fn f179(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a623 = v0.clone();
        length_bytes(a623)
    };
    let v3: u64 = {
        let a624 = v1.clone();
        length_bytes(a624)
    };
    let a625 = v3;
    let a626 = v2;
    if nat_le(a625, a626) {
        let m632 = {
            let a629 = v0;
            let a630 = {
                let a627 = v2;
                let a628 = v3;
                nat_sub(a627, a628)
            };
            let a631 = v3;
            slice_bytes(a629, a630, a631)
        };
        match m632 {
            None => {
                false
            }
            Some(v4) => {
                let a633 = v4;
                let a634 = v1;
                equal(a633, a634)
            }
        }
    } else {
        false
    }
}

pub fn f180(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt13>>> {
    let m635 = v0;
    match m635.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f217(v2)?;
            let a637 = {
                let a636 = v4.clone();
                length_bytes(a636)
            };
            let a638 = 0u64;
            if nat_eq(a637, a638) {
                f180(v3, v1)
            } else {
                let m639 = f87(v4)?;
                match m639 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f293(v5.clone())
                        } else {
                            f294(v5.clone())
                        } {
                            let m640 = f180(v3, v1)?;
                            match m640 {
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

pub fn f181(v0: List<Bytes>) -> R<bool> {
    if f295(f198(v0.clone())?) {
        Ok(f40({
            let a642 = {
                let a641 = f296(f198(v0)?);
                length_list(a641)
            };
            let a643 = 0u64;
            nat_eq(a642, a643)
        }))
    } else {
        Ok(false)
    }
}

pub fn f182(v0: List<Bytes>) -> R<bool> {
    if f295(f185(v0.clone(), f8(Str::lit("requirement")))?) {
        if f295(f185(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f40({
                let a645 = {
                    let a644 = f189(f296(f185(v0.clone(), f8(Str::lit("requirement")))?), f296(f185(v0, f8(Str::lit("admission")))?));
                    length_list(a644)
                };
                let a646 = 0u64;
                nat_eq(a645, a646)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f183() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f184() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f185(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt13>>> {
    let m647 = v0;
    match m647.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f217(v2)?;
            let a649 = {
                let a648 = v4.clone();
                length_bytes(a648)
            };
            let a650 = 0u64;
            if nat_eq(a649, a650) {
                f185(v3, v1)
            } else {
                let m651 = f87(v4)?;
                match m651 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f297(v5.clone());
                        let a653 = {
                            let a652 = v6.clone();
                            length_bytes(a652)
                        };
                        let a654 = 0u64;
                        if nat_eq(a653, a654) {
                            Ok(None::<List<Adt13>>)
                        } else {
                            let m655 = f185(v3, v1.clone())?;
                            match m655 {
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

pub fn f186(p658: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m657 = p658;
    {
        let mut m657 = m657;
        loop {
            let step = {
                let v0 = m657;
                let m656 = v0;
                match m656.uncons() {
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
                Ok(next) => { m657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f188(v0: Bytes, v1: List<Adt13>) -> R<(Bytes, Bytes)> {
    Ok((v0, f97(Adt13::C3(std::rc::Rc::new(f98(v1))))?))
}

pub fn f189(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m659 = v0;
    match m659.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f189(v3, v1))
        }
    }
}

pub fn f190(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m660 = v0;
    match m660.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f217(v1)?;
            let a662 = {
                let a661 = v3.clone();
                length_bytes(a661)
            };
            let a663 = 0u64;
            if nat_eq(a662, a663) {
                f190(v2)
            } else {
                let m664 = f298(v3.clone())?;
                match m664 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f299(v4.clone()) {
                            let m665 = f190(v2)?;
                            match m665 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f239(), List::cons(f8(Str::lit("/")), List::cons(f300(f195(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a666 = v3;
                                        let a667 = Bytes::lit(&[0x0a]);
                                        append_bytes(a666, a667)
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

pub fn f191(v0: Adt1, v1: Bytes) -> R<Option<Adt20>> {
    f298(f53(v0, v1))
}

pub fn f192(p673: Adt20, v1: Bytes) -> Option<Adt20> {
    let m672 = p673;
    {
        let mut m672 = m672;
        loop {
            let step = {
                let v0 = m672;
                let m668 = v0;
                match m668 {
                    Adt20::C0(_) => {
                        Err::<Adt20, Option<Adt20>>(None::<Adt20>)
                    }
                    Adt20::C1(_, _) => {
                        Err::<Adt20, Option<Adt20>>(None::<Adt20>)
                    }
                    Adt20::C2(_, _, _) => {
                        Err::<Adt20, Option<Adt20>>(None::<Adt20>)
                    }
                    Adt20::C3(_) => {
                        Err::<Adt20, Option<Adt20>>(None::<Adt20>)
                    }
                    Adt20::C4(_) => {
                        Err::<Adt20, Option<Adt20>>(None::<Adt20>)
                    }
                    Adt20::C5(r669) => {
                        let v5 = (*r669).clone();
                        Ok::<Adt20, Option<Adt20>>(v5)
                    }
                    Adt20::C6 => {
                        Err::<Adt20, Option<Adt20>>(None::<Adt20>)
                    }
                    Adt20::C7(_, _) => {
                        Err::<Adt20, Option<Adt20>>(None::<Adt20>)
                    }
                    Adt20::C8(v2, r670, r671) => {
                        let v3 = (*r670).clone();
                        let v4 = (*r671).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt20, Option<Adt20>>(Some(v3))
                        } else {
                            Ok::<Adt20, Option<Adt20>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m672 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Adt1, v1: Bytes, v2: Adt20, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m674 = f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m674.uncons() {
        None => {
            let m676 = f186(f148(List::cons(Adt13::C4(std::rc::Rc::new({
                let m675 = f301(v4.clone());
                match m675 {
                    None => {
                        Adt13::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt13>::nil())), f147(f93(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m676 {
                None => {
                    f302(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f187(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f194(v0: Adt20) -> R<Bytes> {
    let a677 = f303(v0)?;
    let a678 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a677, a678))
}

pub fn f195(v0: Adt20, v1: Bytes) -> Bytes {
    let m679 = f304(f192(v0, v1));
    match m679 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f196(v0: List<Adt20>) -> List<Bytes> {
    let m680 = v0;
    match m680.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m681 = f304(Some(v1));
            match m681 {
                None => {
                    f196(v2)
                }
                Some(v3) => {
                    List::cons(v3, f196(v2))
                }
            }
        }
    }
}

pub fn f197(v0: Option<Adt20>) -> List<Adt20> {
    let m682 = v0;
    match m682 {
        None => {
            List::<Adt20>::nil()
        }
        Some(v1) => {
            f305(v1)
        }
    }
}

pub fn f198(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m683 = v0;
    match m683.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f217(v1)?;
            let a685 = {
                let a684 = v3.clone();
                length_bytes(a684)
            };
            let a686 = 0u64;
            if nat_eq(a685, a686) {
                f198(v2)
            } else {
                let m687 = f87(v3)?;
                match m687 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if if f254(v4.clone(), f8(Str::lit("agent"))) {
                            if f254(v4.clone(), f8(Str::lit("at"))) {
                                if f254(v4.clone(), f8(Str::lit("quote"))) {
                                    f306(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m688 = f198(v2)?;
                            match m688 {
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

pub fn f199(p691: List<Adt13>) -> bool {
    let m690 = p691;
    {
        let mut m690 = m690;
        loop {
            let step = {
                let v0 = m690;
                let m689 = v0;
                match m689.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f28(f47(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f41(f8(Str::lit("studio")), f117(f307(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Option<Adt13>) -> List<Adt13> {
    let m692 = v0;
    match m692 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            f308(v1)
        }
    }
}

pub fn f201() -> Bytes {
    f309()
}

pub fn f202(v0: Adt1, v1: Adt13, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f160(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f310(v1, v4)?)?), List::cons(f161(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f203(v0: Bytes, p746: u64, p747: u64) -> R<bool> {
    let m745 = (p746, p747);
    {
        let mut m745 = m745;
        loop {
            let step = {
                let (v1, v2) = m745;
                let m693 = v2;
                if m693 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m693 - 1;
                    let m696 = {
                        let a694 = v0.clone();
                        let a695 = v1;
                        index_bytes(a694, a695)
                    };
                    match m696 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f311({
                                let v5: u8 = v4;
                                let a741 = {
                                    let a717 = {
                                        let a705 = {
                                            let a699 = {
                                                let a697 = v5;
                                                let a698 = 128u8;
                                                fixed_u8::bit_and(a697, a698)
                                            };
                                            let a700 = 128u8;
                                            if equal(a699, a700) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a706 = {
                                            let a703 = {
                                                let a701 = v5;
                                                let a702 = 64u8;
                                                fixed_u8::bit_and(a701, a702)
                                            };
                                            let a704 = 64u8;
                                            if equal(a703, a704) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a705, a706)?
                                    };
                                    let a718 = {
                                        let a715 = {
                                            let a709 = {
                                                let a707 = v5;
                                                let a708 = 32u8;
                                                fixed_u8::bit_and(a707, a708)
                                            };
                                            let a710 = 32u8;
                                            if equal(a709, a710) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a716 = {
                                            let a713 = {
                                                let a711 = v5;
                                                let a712 = 16u8;
                                                fixed_u8::bit_and(a711, a712)
                                            };
                                            let a714 = 16u8;
                                            if equal(a713, a714) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a715, a716)?
                                    };
                                    nat_add(a717, a718)?
                                };
                                let a742 = {
                                    let a739 = {
                                        let a727 = {
                                            let a721 = {
                                                let a719 = v5;
                                                let a720 = 8u8;
                                                fixed_u8::bit_and(a719, a720)
                                            };
                                            let a722 = 8u8;
                                            if equal(a721, a722) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a728 = {
                                            let a725 = {
                                                let a723 = v5;
                                                let a724 = 4u8;
                                                fixed_u8::bit_and(a723, a724)
                                            };
                                            let a726 = 4u8;
                                            if equal(a725, a726) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a727, a728)?
                                    };
                                    let a740 = {
                                        let a737 = {
                                            let a731 = {
                                                let a729 = v5;
                                                let a730 = 2u8;
                                                fixed_u8::bit_and(a729, a730)
                                            };
                                            let a732 = 2u8;
                                            if equal(a731, a732) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a738 = {
                                            let a735 = {
                                                let a733 = v5;
                                                let a734 = 1u8;
                                                fixed_u8::bit_and(a733, a734)
                                            };
                                            let a736 = 1u8;
                                            if equal(a735, a736) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a737, a738)?
                                    };
                                    nat_add(a739, a740)?
                                };
                                nat_add(a741, a742)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a743 = v1;
                                    let a744 = 1u64;
                                    nat_add(a743, a744)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m745 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Bytes) -> R<Bytes> {
    f312(v0.clone(), 0u64, {
        let a748 = v0;
        length_bytes(a748)
    }, false, false, Bytes::lit(&[]))
}

pub fn f205(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt13, u64)>> {
    let m749 = v2;
    if m749 == 0 {
        Ok(None::<(Adt13, u64)>)
    } else {
        let v4 = m749 - 1;
        let v5: u64 = f313(v0.clone(), v1)?;
        let a750 = v3;
        let a751 = 1u64;
        if nat_eq(a750, a751) {
            let m752 = f205(v0.clone(), v1, v4, 0u64)?;
            match m752 {
                None => {
                    Ok(None::<(Adt13, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f313(v0.clone(), {
                        let (_, h753) = v6.clone();
                        h753
                    })?;
                    let a754 = v7;
                    let a755 = 44u64;
                    if nat_eq(a754, a755) {
                        let m759 = f205(v0, {
                            let a757 = {
                                let (_, h756) = v6;
                                h756
                            };
                            let a758 = 1u64;
                            nat_add(a757, a758)?
                        }, v4, 1u64)?;
                        match m759 {
                            None => {
                                Ok(None::<(Adt13, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt13::C6(std::rc::Rc::new({
                                    let (h760, _) = v6;
                                    h760
                                }), std::rc::Rc::new({
                                    let (h761, _) = v8;
                                    h761
                                })), {
                                    let (_, h762) = v8;
                                    h762
                                })))
                            }
                        }
                    } else {
                        let a763 = v7;
                        let a764 = 93u64;
                        if nat_eq(a763, a764) {
                            Ok(Some((Adt13::C6(std::rc::Rc::new({
                                let (h765, _) = v6;
                                h765
                            }), std::rc::Rc::new(Adt13::C5)), {
                                let a767 = {
                                    let (_, h766) = v6;
                                    h766
                                };
                                let a768 = 1u64;
                                nat_add(a767, a768)?
                            })))
                        } else {
                            Ok(None::<(Adt13, u64)>)
                        }
                    }
                }
            }
        } else {
            let a769 = v3;
            let a770 = 2u64;
            if nat_eq(a769, a770) {
                let m773 = f314(v0.clone(), {
                    let a771 = v1;
                    let a772 = 1u64;
                    nat_add(a771, a772)?
                })?;
                match m773 {
                    None => {
                        Ok(None::<(Adt13, u64)>)
                    }
                    Some(v9) => {
                        let a774 = v5;
                        let a775 = 34u64;
                        if if nat_eq(a774, a775) {
                            let a777 = f313(v0.clone(), {
                                let (h776, _) = v9.clone();
                                h776
                            })?;
                            let a778 = 58u64;
                            nat_eq(a777, a778)
                        } else {
                            false
                        } {
                            let m782 = f205(v0.clone(), {
                                let a780 = {
                                    let (h779, _) = v9;
                                    h779
                                };
                                let a781 = 1u64;
                                nat_add(a780, a781)?
                            }, v4, 0u64)?;
                            match m782 {
                                None => {
                                    Ok(None::<(Adt13, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f313(v0.clone(), {
                                        let (_, h783) = v10.clone();
                                        h783
                                    })?;
                                    let a784 = v11;
                                    let a785 = 44u64;
                                    if nat_eq(a784, a785) {
                                        let m789 = f205(v0, {
                                            let a787 = {
                                                let (_, h786) = v10;
                                                h786
                                            };
                                            let a788 = 1u64;
                                            nat_add(a787, a788)?
                                        }, v4, 2u64)?;
                                        match m789 {
                                            None => {
                                                Ok(None::<(Adt13, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt13::C7({
                                                    let (_, h790) = v9;
                                                    h790
                                                }, std::rc::Rc::new({
                                                    let (h791, _) = v10;
                                                    h791
                                                }), std::rc::Rc::new({
                                                    let (h792, _) = v12;
                                                    h792
                                                })), {
                                                    let (_, h793) = v12;
                                                    h793
                                                })))
                                            }
                                        }
                                    } else {
                                        let a794 = v11;
                                        let a795 = 125u64;
                                        if nat_eq(a794, a795) {
                                            Ok(Some((Adt13::C7({
                                                let (_, h796) = v9;
                                                h796
                                            }, std::rc::Rc::new({
                                                let (h797, _) = v10;
                                                h797
                                            }), std::rc::Rc::new(Adt13::C5)), {
                                                let a799 = {
                                                    let (_, h798) = v10;
                                                    h798
                                                };
                                                let a800 = 1u64;
                                                nat_add(a799, a800)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt13, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt13, u64)>)
                        }
                    }
                }
            } else {
                let a801 = v5;
                let a802 = 34u64;
                if nat_eq(a801, a802) {
                    let m805 = f314(v0, {
                        let a803 = v1;
                        let a804 = 1u64;
                        nat_add(a803, a804)?
                    })?;
                    match m805 {
                        None => {
                            Ok(None::<(Adt13, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt13::C2({
                                let (_, h806) = v13;
                                h806
                            }), {
                                let (h807, _) = v13;
                                h807
                            })))
                        }
                    }
                } else {
                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt13::C0(true), {
                            let a808 = v1;
                            let a809 = 4u64;
                            nat_add(a808, a809)?
                        })))
                    } else {
                        if f28(f35(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt13::C0(false), {
                                let a810 = v1;
                                let a811 = 5u64;
                                nat_add(a810, a811)?
                            })))
                        } else {
                            let a812 = v5;
                            let a813 = 91u64;
                            if nat_eq(a812, a813) {
                                let a816 = f313(v0.clone(), {
                                    let a814 = v1;
                                    let a815 = 1u64;
                                    nat_add(a814, a815)?
                                })?;
                                let a817 = 93u64;
                                if nat_eq(a816, a817) {
                                    Ok(Some((Adt13::C3(std::rc::Rc::new(Adt13::C5)), {
                                        let a818 = v1;
                                        let a819 = 2u64;
                                        nat_add(a818, a819)?
                                    })))
                                } else {
                                    let m822 = f205(v0, {
                                        let a820 = v1;
                                        let a821 = 1u64;
                                        nat_add(a820, a821)?
                                    }, v4, 1u64)?;
                                    match m822 {
                                        None => {
                                            Ok(None::<(Adt13, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt13::C3(std::rc::Rc::new({
                                                let (h823, _) = v14;
                                                h823
                                            })), {
                                                let (_, h824) = v14;
                                                h824
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a825 = v5;
                                let a826 = 123u64;
                                if nat_eq(a825, a826) {
                                    let a829 = f313(v0.clone(), {
                                        let a827 = v1;
                                        let a828 = 1u64;
                                        nat_add(a827, a828)?
                                    })?;
                                    let a830 = 125u64;
                                    if nat_eq(a829, a830) {
                                        Ok(Some((Adt13::C4(std::rc::Rc::new(Adt13::C5)), {
                                            let a831 = v1;
                                            let a832 = 2u64;
                                            nat_add(a831, a832)?
                                        })))
                                    } else {
                                        let m835 = f205(v0, {
                                            let a833 = v1;
                                            let a834 = 1u64;
                                            nat_add(a833, a834)?
                                        }, v4, 2u64)?;
                                        match m835 {
                                            None => {
                                                Ok(None::<(Adt13, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt13::C4(std::rc::Rc::new({
                                                    let (h836, _) = v15;
                                                    h836
                                                })), {
                                                    let (_, h837) = v15;
                                                    h837
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m838 = f315(v0, v1)?;
                                    match m838 {
                                        None => {
                                            Ok(None::<(Adt13, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h839) = v16;
                                                h839
                                            }, {
                                                let (h840, _) = v16;
                                                h840
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

pub fn f206(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f90(v0.clone(), v1) {
        let m841 = f33(v0, 34u8)?;
        match m841.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m842 = v3;
                match m842.uncons() {
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

pub fn f207(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f316(v0, v1)
}

pub fn f208(v0: Adt13, v1: Bytes) -> Adt13 {
    let m843 = v0;
    match m843 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r844) => {
            let v10 = (*r844).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r845) => {
            let v5 = (*r845).clone();
            Adt13::C4(std::rc::Rc::new(f208(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r846, r847) => {
            let v11 = (*r846).clone();
            let v12 = (*r847).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r848, r849) => {
            let v3 = (*r848).clone();
            let v4 = (*r849).clone();
            if f28(v2.clone(), f8(Str::lit("std"))) {
                Adt13::C7(v2, std::rc::Rc::new(f317(v3, v1.clone())), std::rc::Rc::new(f208(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f208(v4, v1)))
            }
        }
    }
}

pub fn f209(v0: Adt13) -> R<Bytes> {
    let m850 = v0;
    match m850 {
        Adt13::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt13::C1(v2, v3) => {
            f318(v2, v3)
        }
        Adt13::C2(v4) => {
            f319(v4)
        }
        Adt13::C3(r851) => {
            let v5 = (*r851).clone();
            Ok(f320(f321(v5)?))
        }
        Adt13::C4(r852) => {
            let v6 = (*r852).clone();
            f322(f323(v6)?)
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

pub fn f210(v0: Adt1, p856: List<Adt12>) -> R<Option<Adt12>> {
    let m855 = p856;
    {
        let mut m855 = m855;
        loop {
            let step = {
                let v1 = m855;
                let m853 = v1;
                match m853.uncons() {
                    None => {
                        Err::<List<Adt12>, R<Option<Adt12>>>(Ok(None::<Adt12>))
                    }
                    Some((v2, v3)) => {
                        if f41(f211(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h854) => {
                                h854
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
                Ok(next) => { m855 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: Adt12) -> R<Bytes> {
    Ok(f175(f324(), f178(f325(v0))?))
}

pub fn f212(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f325(v0), List::<Bytes>::nil())))))
}

pub fn f213() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f214() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f215(v0: List<Adt12>) -> R<List<Bytes>> {
    let m857 = v0;
    match m857.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f211(v1)?, f215(v2)?))
        }
    }
}

pub fn f216(v0: Adt1, v1: List<Adt12>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m858 = v1;
    match m858.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f57(f58(v5.clone(), f50(v2.clone()))), f326(v5.clone(), v2.clone(), v3.clone(), f159(f53(v0.clone(), f211(v5)?))?, v4.clone())?), f216(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f217(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f327(v0.clone())?;
    let v2: u64 = f328(v0.clone(), v1)?;
    Ok(f35(v0, v2, {
        let a859 = v1;
        let a860 = v2;
        nat_sub(a859, a860)
    }))
}

pub fn f218(v0: Bytes, v1: u64) -> Option<u8> {
    let a861 = v0;
    let a862 = v1;
    index_bytes(a861, a862)
}

pub fn f219(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m863 = v1;
    match m863.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f329(v3.clone())
            } else {
                f40(f329(v3.clone()))
            } {
                List::cons(f175(v0.clone(), v3), f219(v0, v4, v2))
            } else {
                f219(v0, v4, v2)
            }
        }
    }
}

pub fn f220(p866: List<Bytes>) -> R<Option<Bytes>> {
    let m865 = p866;
    {
        let mut m865 = m865;
        loop {
            let step = {
                let v0 = m865;
                let m864 = v0;
                match m864.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f330(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f217(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m867 = v1;
    match m867.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f109(f92(f53(v0.clone(), v2))?, f221(v0, v3)?))
        }
    }
}

pub fn f222(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a868 = v0.clone();
        length_bytes(a868)
    };
    let v3: u64 = {
        let a869 = v1.clone();
        length_bytes(a869)
    };
    let v4: u64 = {
        let (h870, _) = f331(Fn3::F332(v1.clone(), v3, v0.clone(), v2), {
            let a871 = v2;
            let a872 = 1u64;
            nat_add(a871, a872)?
        }, 0u64)?;
        h870
    };
    if f269(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f223() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f224(v0: Adt14) -> R<Adt17> {
    Ok(Adt17::C0(f333(match v0.clone() {
        Adt14::C0(h873, _, _, _, _) => {
            h873
        }
    }, 0u64)?, f333(match v0.clone() {
        Adt14::C0(_, h874, _, _, _) => {
            h874
        }
    }, 0u64)?, f333(match v0.clone() {
        Adt14::C0(_, _, h875, _, _) => {
            h875
        }
    }, 0u64)?, f333(match v0 {
        Adt14::C0(_, _, _, h876, _) => {
            h876
        }
    }, 0u64)?))
}

pub fn f225(v0: Adt14) -> List<Bytes> {
    f109(match v0.clone() {
        Adt14::C0(h877, _, _, _, _) => {
            h877
        }
    }, f109(match v0.clone() {
        Adt14::C0(_, _, h878, _, _) => {
            h878
        }
    }, f109(match v0.clone() {
        Adt14::C0(_, _, _, h879, _) => {
            h879
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, h880) => {
            h880
        }
    })))
}

pub fn f226(v0: Adt17, p886: List<u64>, p887: List<Adt21>) -> Option<u64> {
    let m885 = (p886, p887);
    {
        let mut m885 = m885;
        loop {
            let step = {
                let (v1, v2) = m885;
                let m881 = v2;
                match m881.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt21>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f334(v0.clone(), v1.clone(), match v3.clone() {
                            Adt21::C0(_, h882) => {
                                h882
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt21>), Option<u64>>((List::cons(match v3 {
                                Adt21::C0(h883, _) => {
                                    h883
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt21>), Option<u64>>(Some(match v3 {
                                Adt21::C0(h884, _) => {
                                    h884
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: Adt14, v1: List<Adt13>, v2: u64) -> R<List<Adt21>> {
    let m888 = v1;
    match m888.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt21::C0(v2, f335(v0.clone(), v3)?), f227(v0, v4, {
                let a889 = v2;
                let a890 = 1u64;
                nat_add(a889, a890)?
            })?))
        }
    }
}

pub fn f228(p894: List<Bytes>, p895: u64) -> Bytes {
    let m893 = (p894, p895);
    {
        let mut m893 = m893;
        loop {
            let step = {
                let (v0, v1) = m893;
                let m891 = v0;
                match m891.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m892 = v1;
                        if m892 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m892 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f336(v0.clone(), v1.clone()) {
        if f336(v1, v0) {
            f337(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f230(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m896 = v0;
    match m896.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f338(v2, v1.clone())?, f230(v3, v1)?))
        }
    }
}

pub fn f231(v0: List<Adt13>) -> List<u64> {
    let m897 = v0;
    match m897.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f339(v1), f231(v2))
        }
    }
}

pub fn f232(p900: List<u64>, v1: List<u64>) -> Option<u64> {
    let m899 = p900;
    {
        let mut m899 = m899;
        loop {
            let step = {
                let v0 = m899;
                let m898 = v0;
                match m898.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f340(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m899 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233(p903: List<Bytes>, p904: List<Bytes>) -> Option<Bytes> {
    let m902 = (p903, p904);
    {
        let mut m902 = m902;
        loop {
            let step = {
                let (v0, v1) = m902;
                let m901 = v0;
                match m901.uncons() {
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
                Ok(next) => { m902 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: List<Bytes>) -> List<Bytes> {
    f104(Fn1::F341, List::<Bytes>::nil(), v0)
}

pub fn f235(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f342(v0, v1)
}

pub fn f236(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f343(f344(f345(v0))?))
}

pub fn f237(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m905 = f46(v0, v2)?;
    match m905 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f346(v1, f47(v3.clone(), f8(Str::lit("id"))))?;
            let m906 = v4.clone();
            match m906.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m908 = f88(f89({
                        let m907 = f89(v3.clone(), f8(Str::lit("source")));
                        match m907 {
                            None => {
                                Adt13::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m908 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f40(f41(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f260(f347(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f238(p911: List<Bytes>, p912: List<Bytes>) -> List<Bytes> {
    let m910 = (p911, p912);
    {
        let mut m910 = m910;
        loop {
            let step = {
                let (v0, v1) = m910;
                let m909 = v0;
                match m909.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m910 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f240(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m913 = v1;
    match m913.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt20 = f348(v0.clone(), v3)?;
            if f41(f349(f195(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f350(v5)?, f240(v0, v4, v2)?))
            } else {
                f240(v0, v4, v2)
            }
        }
    }
}

pub fn f241(v0: Adt1, p917: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m916 = p917;
    {
        let mut m916 = m916;
        loop {
            let step = {
                let v1 = m916;
                let m914 = v1;
                match m914.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt20 = f348(v0.clone(), v3.clone())?;
                        if if f41(f349(f195(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f28(f194({
                                let m915 = f192(v5.clone(), f8(Str::lit("schema")));
                                match m915 {
                                    None => {
                                        Adt20::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f53(v0.clone(), f350(v5)?))
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
                Ok(next) => { m916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f119(match v0.clone() {
        Adt1::C0(_, h918, _) => {
            h918
        }
    }, f183())) {
        Ok(Adt11::C1(List::cons(f183(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f126(f121(f351(f123(match v0.clone() {
            Adt1::C0(_, h919, _) => {
                h919
            }
        }, f183())))?, v1);
        let m920 = f45(v0.clone(), List::cons(f130(), List::cons(f158(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m920.uncons() {
            None => {
                let v5: (bool, Bytes) = f352(v0.clone(), v2)?;
                let (h921, _) = v5;
                if h921 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h922) = v5;
                        h922
                    }))
                } else {
                    let m923 = f129(v0.clone());
                    match m923.uncons() {
                        None => {
                            let m924 = f131(v0.clone())?;
                            match m924.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h925) = v5;
                                        h925
                                    }, List::cons(f7(f121(f353(f144(v0.clone())?, f152(v0.clone(), f153(v0.clone())?)?))?), List::cons(f354(v0)?, List::<Bytes>::nil()))))))
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

pub fn f243(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f355(f93(v0, f130())?)?)))
}

pub fn f244() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f245(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt24>, v3: List<Adt13>, v4: List<Bytes>) -> List<Bytes> {
    let m926 = v1;
    match m926.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f109(f356(v5.clone(), v2.clone()), f357(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f358(v0.clone(), v5.clone()), List::cons(if f41(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a928 = {
                    let a927 = v7.clone();
                    length_list(a927)
                };
                let a929 = 0u64;
                if nat_eq(a928, a929) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f162(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f245(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f246(v0: Adt16, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Adt24>> {
    let v3: List<(Bytes, Bytes)> = f359(v1);
    let v4: List<(Bytes, Bytes)> = f360(v0, v2);
    f361(v3.clone(), f132(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f247(v0: Adt16, v1: List<Adt13>) -> List<Bytes> {
    let m930 = v1;
    match m930.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f254(v2.clone(), f8(Str::lit("domain"))) {
                f362(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f47(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m931 = f89(v2.clone(), f8(Str::lit("role")));
                    match m931 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f47(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f47(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f363(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m932 = f89(v2.clone(), f8(Str::lit("origin")));
                    match m932 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f47(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m933 = f89(v2, f8(Str::lit("evidence")));
                    match m933 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f47(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f247(v0, v3))
            } else {
                f247(v0, v3)
            }
        }
    }
}

pub fn f248(v0: List<Adt13>) -> List<Bytes> {
    let m934 = v0;
    match m934.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f47(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f47(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f47(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f248(v2))
        }
    }
}

pub fn f249(v0: List<Adt13>) -> List<Bytes> {
    let m935 = v0;
    match m935.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f47(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f364(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f249(v2))
        }
    }
}

pub fn f250(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m936 = v0;
    match m936.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f47(v1.clone(), f8(Str::lit("subject"))), f47(v1, f8(Str::lit("object")))), f250(v2))
        }
    }
}

pub fn f251(v0: List<Adt24>) -> List<(Bytes, Bytes)> {
    let m937 = v0;
    match m937.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt24::C0(h938, _, _) => {
                    h938
                }
            }, match v1 {
                Adt24::C0(_, _, h939) => {
                    h939
                }
            }), f251(v2))
        }
    }
}

pub fn f252(p946: List<Bytes>, v1: List<(Bytes, Bytes)>, p947: u64) -> R<List<Bytes>> {
    let m945 = (p946, p947);
    {
        let mut m945 = m945;
        loop {
            let step = {
                let (v0, v2) = m945;
                let m940 = v2;
                if m940 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m940 - 1;
                    let v4: List<Bytes> = f121(f109(v0.clone(), f365(v0.clone(), v1.clone())))?;
                    let a943 = {
                        let a941 = v4.clone();
                        length_list(a941)
                    };
                    let a944 = {
                        let a942 = v0.clone();
                        length_list(a942)
                    };
                    if nat_eq(a943, a944) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m945 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: List<Adt13>) -> List<Bytes> {
    let m948 = v0;
    match m948.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m949 = f89(v1, f8(Str::lit("std")));
            match m949 {
                None => {
                    f253(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3.clone(), f8(Str::lit("authority"))), if f254(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f47(v3, f8(Str::lit("row"))), f253(v2))
                    } else {
                        f253(v2)
                    })
                }
            }
        }
    }
}

pub fn f254(v0: Adt13, v1: Bytes) -> bool {
    f40({
        let a951 = {
            let a950 = f47(v0, v1);
            length_bytes(a950)
        };
        let a952 = 0u64;
        nat_eq(a951, a952)
    })
}

pub fn f255(v0: Adt16, v1: Adt13) -> bool {
    if f366(v0.clone(), v1.clone(), f47(v1.clone(), f8(Str::lit("subject")))) {
        f366(v0, v1.clone(), f47(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f256(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m953 = v1;
    match m953.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f179(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f175(v0.clone(), v2), f256(v0, v3))
            } else {
                f256(v0, v3)
            }
        }
    }
}

pub fn f257(v0: Bytes, p1011: u64, p1012: u64, p1013: Adt18) -> R<Adt18> {
    let m1010 = (p1011, (p1012, p1013));
    {
        let mut m1010 = m1010;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1010;
                let m954 = v2;
                if m954 == 0 {
                    Err::<(u64, (u64, Adt18)), R<Adt18>>(Ok(v3))
                } else {
                    let v4 = m954 - 1;
                    let v5: u64 = f367(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt18::C0(_, h955, _, _, _) => {
                            h955
                        }
                    } {
                        Ok::<(u64, (u64, Adt18)), R<Adt18>>(({
                            let a956 = v1;
                            let a957 = 1u64;
                            nat_add(a956, a957)?
                        }, (v4, Adt18::C0(true, false, match v3.clone() {
                            Adt18::C0(_, _, h958, _, _) => {
                                h958
                            }
                        }, match v3.clone() {
                            Adt18::C0(_, _, _, h959, _) => {
                                h959
                            }
                        }, match v3 {
                            Adt18::C0(_, _, _, _, h960) => {
                                h960
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt18::C0(h961, _, _, _, _) => {
                                h961
                            }
                        } {
                            let a962 = v5;
                            let a963 = 34u64;
                            if nat_eq(a962, a963) {
                                let a966 = f367(v0.clone(), {
                                    let a964 = v1;
                                    let a965 = 1u64;
                                    nat_add(a964, a965)?
                                })?;
                                let a967 = 34u64;
                                if nat_eq(a966, a967) {
                                    Ok::<(u64, (u64, Adt18)), R<Adt18>>(({
                                        let a968 = v1;
                                        let a969 = 1u64;
                                        nat_add(a968, a969)?
                                    }, (v4, Adt18::C0(true, true, f368(match v3.clone() {
                                        Adt18::C0(_, _, h970, _, _) => {
                                            h970
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt18::C0(_, _, _, h971, _) => {
                                            h971
                                        }
                                    }, match v3 {
                                        Adt18::C0(_, _, _, _, h972) => {
                                            h972
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt18)), R<Adt18>>(({
                                        let a973 = v1;
                                        let a974 = 1u64;
                                        nat_add(a973, a974)?
                                    }, (v4, Adt18::C0(false, false, match v3.clone() {
                                        Adt18::C0(_, _, h975, _, _) => {
                                            h975
                                        }
                                    }, match v3.clone() {
                                        Adt18::C0(_, _, _, h976, _) => {
                                            h976
                                        }
                                    }, match v3 {
                                        Adt18::C0(_, _, _, _, h977) => {
                                            h977
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt18)), R<Adt18>>(({
                                    let a978 = v1;
                                    let a979 = 1u64;
                                    nat_add(a978, a979)?
                                }, (v4, Adt18::C0(true, false, f368(match v3.clone() {
                                    Adt18::C0(_, _, h980, _, _) => {
                                        h980
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt18::C0(_, _, _, h981, _) => {
                                        h981
                                    }
                                }, match v3 {
                                    Adt18::C0(_, _, _, _, h982) => {
                                        h982
                                    }
                                }))))
                            }
                        } else {
                            let a983 = v5;
                            let a984 = 34u64;
                            if nat_eq(a983, a984) {
                                Ok::<(u64, (u64, Adt18)), R<Adt18>>(({
                                    let a985 = v1;
                                    let a986 = 1u64;
                                    nat_add(a985, a986)?
                                }, (v4, Adt18::C0(true, false, match v3.clone() {
                                    Adt18::C0(_, _, h987, _, _) => {
                                        h987
                                    }
                                }, match v3.clone() {
                                    Adt18::C0(_, _, _, h988, _) => {
                                        h988
                                    }
                                }, match v3 {
                                    Adt18::C0(_, _, _, _, h989) => {
                                        h989
                                    }
                                }))))
                            } else {
                                let a990 = v5;
                                let a991 = 44u64;
                                if nat_eq(a990, a991) {
                                    Ok::<(u64, (u64, Adt18)), R<Adt18>>(({
                                        let a992 = v1;
                                        let a993 = 1u64;
                                        nat_add(a992, a993)?
                                    }, (v4, Adt18::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt18::C0(_, _, h994, _, _) => {
                                            h994
                                        }
                                    }, match v3.clone() {
                                        Adt18::C0(_, _, _, h995, _) => {
                                            h995
                                        }
                                    }), match v3 {
                                        Adt18::C0(_, _, _, _, h996) => {
                                            h996
                                        }
                                    }))))
                                } else {
                                    let a997 = v5;
                                    let a998 = 13u64;
                                    if nat_eq(a997, a998) {
                                        Ok::<(u64, (u64, Adt18)), R<Adt18>>(({
                                            let a999 = v1;
                                            let a1000 = 1u64;
                                            nat_add(a999, a1000)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1001 = v5;
                                        let a1002 = 10u64;
                                        if nat_eq(a1001, a1002) {
                                            Ok::<(u64, (u64, Adt18)), R<Adt18>>(({
                                                let a1003 = v1;
                                                let a1004 = 1u64;
                                                nat_add(a1003, a1004)?
                                            }, (v4, f258(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt18)), R<Adt18>>(({
                                                let a1005 = v1;
                                                let a1006 = 1u64;
                                                nat_add(a1005, a1006)?
                                            }, (v4, Adt18::C0(false, false, f368(match v3.clone() {
                                                Adt18::C0(_, _, h1007, _, _) => {
                                                    h1007
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt18::C0(_, _, _, h1008, _) => {
                                                    h1008
                                                }
                                            }, match v3 {
                                                Adt18::C0(_, _, _, _, h1009) => {
                                                    h1009
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
                Ok(next) => { m1010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f258(v0: Adt18) -> Adt18 {
    Adt18::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f369(List::cons(match v0.clone() {
        Adt18::C0(_, _, h1014, _, _) => {
            h1014
        }
    }, match v0.clone() {
        Adt18::C0(_, _, _, h1015, _) => {
            h1015
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt18::C0(_, _, _, _, h1016) => {
            h1016
        }
    }))
}

pub fn f259(p1019: List<List<Bytes>>, p1020: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1018 = (p1019, p1020);
    {
        let mut m1018 = m1018;
        loop {
            let step = {
                let (v0, v1) = m1018;
                let m1017 = v0;
                match m1017.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
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

pub fn f260(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f370(v0.clone(), 0u64, {
        let a1021 = v0;
        length_bytes(a1021)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f261(v0: Adt25) -> Bytes {
    let a1042 = f371(match v0.clone() {
        Adt25::C0(h1022, _, _, _, _, _, _, _) => {
            h1022
        }
    });
    let a1043 = {
        let a1040 = f371(match v0.clone() {
            Adt25::C0(_, h1023, _, _, _, _, _, _) => {
                h1023
            }
        });
        let a1041 = {
            let a1038 = f371(match v0.clone() {
                Adt25::C0(_, _, h1024, _, _, _, _, _) => {
                    h1024
                }
            });
            let a1039 = {
                let a1036 = f371(match v0.clone() {
                    Adt25::C0(_, _, _, h1025, _, _, _, _) => {
                        h1025
                    }
                });
                let a1037 = {
                    let a1034 = f371(match v0.clone() {
                        Adt25::C0(_, _, _, _, h1026, _, _, _) => {
                            h1026
                        }
                    });
                    let a1035 = {
                        let a1032 = f371(match v0.clone() {
                            Adt25::C0(_, _, _, _, _, h1027, _, _) => {
                                h1027
                            }
                        });
                        let a1033 = {
                            let a1030 = f371(match v0.clone() {
                                Adt25::C0(_, _, _, _, _, _, h1028, _) => {
                                    h1028
                                }
                            });
                            let a1031 = f371(match v0 {
                                Adt25::C0(_, _, _, _, _, _, _, h1029) => {
                                    h1029
                                }
                            });
                            append_bytes(a1030, a1031)
                        };
                        append_bytes(a1032, a1033)
                    };
                    append_bytes(a1034, a1035)
                };
                append_bytes(a1036, a1037)
            };
            append_bytes(a1038, a1039)
        };
        append_bytes(a1040, a1041)
    };
    append_bytes(a1042, a1043)
}

pub fn f262(v0: Bytes) -> R<Adt25> {
    let v1: u64 = {
        let a1044 = v0.clone();
        length_bytes(a1044)
    };
    let v2: u64 = {
        let a1050 = {
            let a1047 = {
                let a1045 = v1;
                let a1046 = 72u64;
                nat_add(a1045, a1046)?
            };
            let a1048 = 64u64;
            let a1049 = 0u64;
            nat_quot(a1047, a1048, a1049)
        };
        let a1051 = 64u64;
        nat_mul(a1050, a1051)?
    };
    let (_, h1052) = f372(Fn4::F373(v0, v1, v2), {
        let a1053 = v2;
        let a1054 = 64u64;
        let a1055 = 0u64;
        nat_quot(a1053, a1054, a1055)
    }, (0u64, f374()))?;
    Ok(h1052)
}

pub fn f263(p1058: List<List<Bytes>>, v1: Bytes, p1059: List<Bytes>) -> R<List<Bytes>> {
    let m1057 = (p1058, p1059);
    {
        let mut m1057 = m1057;
        loop {
            let step = {
                let (v0, v2) = m1057;
                let m1056 = v0;
                match m1056.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f375(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f265(f376(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1057 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: Bytes) -> R<Bytes> {
    Ok(f35(v0.clone(), 0u64, f327(v0)?))
}

pub fn f265(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1060 = v0;
    match m1060.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1061 = v3.clone();
            match m1061.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1064 = v2;
                    let a1065 = {
                        let a1062 = v1.clone();
                        let a1063 = f265(v3, v1);
                        append_bytes(a1062, a1063)
                    };
                    append_bytes(a1064, a1065)
                }
            }
        }
    }
}

pub fn f266(v0: List<Bytes>) -> Adt13 {
    let m1066 = v0;
    match m1066.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(f266(v2)))
        }
    }
}

pub fn f267(v0: Fn2, p1070: Option<Bytes>, p1071: List<Bytes>) -> Option<Bytes> {
    let m1069 = (p1070, p1071);
    {
        let mut m1069 = m1069;
        loop {
            let step = {
                let (v1, v2) = m1069;
                let m1067 = v2;
                match m1067.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1068: Fn2 = v0.clone();
                            c1068.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f268(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1072 = v1;
    match m1072 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1075 = {
                    let a1073 = v3;
                    let a1074 = v0;
                    append_bytes(a1073, a1074)
                };
                let a1076 = v2;
                append_bytes(a1075, a1076)
            })
        }
    }
}

pub fn f269(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1081 = {
        let a1078 = v0;
        let a1079 = v2;
        let a1080 = {
            let a1077 = v1.clone();
            length_bytes(a1077)
        };
        slice_bytes(a1078, a1079, a1080)
    };
    match m1081 {
        None => {
            false
        }
        Some(v3) => {
            let a1082 = v3;
            let a1083 = v1;
            equal(a1082, a1083)
        }
    }
}

pub fn f270(v0: Bytes) -> R<Bytes> {
    let (_, h1084) = {
        let (h1085, _) = f377(Fn5::F378(v0.clone()), {
            let a1087 = {
                let a1086 = v0;
                length_bytes(a1086)
            };
            let a1088 = 1u64;
            nat_add(a1087, a1088)?
        }, (0u64, Bytes::lit(&[])))?;
        h1085
    };
    Ok(h1084)
}

pub fn f271(v0: Bytes) -> R<u64> {
    let (h1089, _) = f331(Fn3::F379(v0.clone()), {
        let a1091 = {
            let a1090 = v0;
            length_bytes(a1090)
        };
        let a1092 = 1u64;
        nat_add(a1091, a1092)?
    }, 0u64)?;
    Ok(h1089)
}

pub fn f272(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1093 = v0;
        utf8_encode(a1093)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f167(v1)?, List::<Bytes>::nil())))))
}

pub fn f273(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f275(v0.clone(), 0u64, Str::lit("<?xml")) {
        f380(f276(v0.clone(), 5u64)?)
    } else {
        false
    } {
        let m1095 = f381(v0.clone(), {
            let a1094 = Str::lit("?>");
            utf8_encode(a1094)
        }, 5u64)?;
        match m1095 {
            None => {
                Ok(Err::<u64, Bytes>(f272(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m1097 = f381(f35(v0.clone(), 0u64, v1), {
                    let a1096 = Str::lit("encoding");
                    utf8_encode(a1096)
                }, 5u64)?;
                match m1097 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a1098 = v1;
                            let a1099 = 2u64;
                            nat_add(a1098, a1099)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f382(v0.clone(), {
                            let a1100 = v2;
                            let a1101 = 8u64;
                            nat_add(a1100, a1101)?
                        })?;
                        let v4: u64 = f382(v0.clone(), {
                            let a1102 = v3;
                            let a1103 = 1u64;
                            nat_add(a1102, a1103)?
                        })?;
                        let _: u64 = f276(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m1106 = f381(v0.clone(), f35(v0.clone(), v4, 1u64), {
                                let a1104 = v4;
                                let a1105 = 1u64;
                                nat_add(a1104, a1105)?
                            })?;
                            match m1106 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f383(f35(v0, {
                            let a1107 = v4;
                            let a1108 = 1u64;
                            nat_add(a1107, a1108)?
                        }, {
                            let a1111 = v7;
                            let a1112 = {
                                let a1109 = v4;
                                let a1110 = 1u64;
                                nat_add(a1109, a1110)?
                            };
                            nat_sub(a1111, a1112)
                        }))? {
                            Ok(Ok::<u64, Bytes>({
                                let a1113 = v1;
                                let a1114 = 2u64;
                                nat_add(a1113, a1114)?
                            }))
                        } else {
                            Ok(Err::<u64, Bytes>(f272(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f274(v0: Bytes, p1119: u64, p1120: u64) -> R<Result<u64, Bytes>> {
    let m1118 = (p1119, p1120);
    {
        let mut m1118 = m1118;
        loop {
            let step = {
                let (v1, v2) = m1118;
                let m1115 = v2;
                if m1115 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(f272(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m1115 - 1;
                    let v4: u64 = f382(v0.clone(), v1)?;
                    if f275(v0.clone(), v4, Str::lit("<!--")) {
                        let m1116 = f384(v0.clone(), v4)?;
                        match m1116 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v5)))
                            }
                        }
                    } else {
                        if f275(v0.clone(), v4, Str::lit("<?")) {
                            let m1117 = f385(v0.clone(), v4)?;
                            match m1117 {
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
                Ok(next) => { m1118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes, v1: u64, v2: Str) -> bool {
    f269(v0, {
        let a1121 = v2;
        utf8_encode(a1121)
    }, v1)
}

pub fn f276(v0: Bytes, v1: u64) -> R<u64> {
    f313(v0, v1)
}

pub fn f277(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt19, u64), Bytes>> {
    let m1122 = v2;
    if m1122 == 0 {
        Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m1122 - 1;
        let a1123 = v3;
        let a1124 = 0u64;
        if nat_eq(a1123, a1124) {
            let v5: u64 = f386(v0.clone(), {
                let a1125 = v1;
                let a1126 = 1u64;
                nat_add(a1125, a1126)?
            })?;
            let a1129 = v5;
            let a1130 = {
                let a1127 = v1;
                let a1128 = 1u64;
                nat_add(a1127, a1128)?
            };
            if nat_eq(a1129, a1130) {
                Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f35(v0.clone(), {
                    let a1131 = v1;
                    let a1132 = 1u64;
                    nat_add(a1131, a1132)?
                }, {
                    let a1135 = v5;
                    let a1136 = {
                        let a1133 = v1;
                        let a1134 = 1u64;
                        nat_add(a1133, a1134)?
                    };
                    nat_sub(a1135, a1136)
                });
                let m1137 = f387(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m1137 {
                    Ok(v8) => {
                        if f275(v0.clone(), {
                            let (_, h1138) = v8.clone();
                            h1138
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt19, u64), Bytes>((Adt19::C0(v6, std::rc::Rc::new({
                                let (h1139, _) = v8;
                                h1139
                            }), std::rc::Rc::new(Adt19::C4)), {
                                let a1141 = {
                                    let (_, h1140) = v8;
                                    h1140
                                };
                                let a1142 = 2u64;
                                nat_add(a1141, a1142)?
                            })))
                        } else {
                            let m1146 = f277(v0.clone(), {
                                let a1144 = {
                                    let (_, h1143) = v8;
                                    h1143
                                };
                                let a1145 = 1u64;
                                nat_add(a1144, a1145)?
                            }, v4, 1u64)?;
                            match m1146 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a1148 = {
                                            let (_, h1147) = v10.clone();
                                            h1147
                                        };
                                        let a1149 = 2u64;
                                        nat_add(a1148, a1149)?
                                    };
                                    let v12: u64 = f386(v0.clone(), v11)?;
                                    if !f28(f35(v0.clone(), v11, {
                                        let a1150 = v12;
                                        let a1151 = v11;
                                        nat_sub(a1150, a1151)
                                    }), v6.clone()) {
                                        Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h1152) = v10;
                                            h1152
                                        })?))
                                    } else {
                                        let v13: u64 = f382(v0.clone(), v12)?;
                                        let a1153 = f276(v0, v13)?;
                                        let a1154 = 62u64;
                                        if nat_eq(a1153, a1154) {
                                            Ok(Ok::<(Adt19, u64), Bytes>((Adt19::C0(v6, std::rc::Rc::new({
                                                let (h1155, _) = v8;
                                                h1155
                                            }), std::rc::Rc::new({
                                                let (h1156, _) = v10;
                                                h1156
                                            })), {
                                                let a1157 = v13;
                                                let a1158 = 1u64;
                                                nat_add(a1157, a1158)?
                                            })))
                                        } else {
                                            Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<(Adt19, u64), Bytes>(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<(Adt19, u64), Bytes>(v7))
                    }
                }
            }
        } else {
            let a1159 = f276(v0.clone(), v1)?;
            let a1160 = 256u64;
            if nat_eq(a1159, a1160) {
                Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f275(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt19, u64), Bytes>((Adt19::C4, v1)))
                } else {
                    if f275(v0.clone(), v1, Str::lit("<!--")) {
                        let m1161 = f384(v0.clone(), v1)?;
                        match m1161 {
                            Ok(v15) => {
                                f277(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(Err::<(Adt19, u64), Bytes>(v14))
                            }
                        }
                    } else {
                        if f275(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m1165 = f381(v0.clone(), {
                                let a1162 = Str::lit("]]>");
                                utf8_encode(a1162)
                            }, {
                                let a1163 = v1;
                                let a1164 = 9u64;
                                nat_add(a1163, a1164)?
                            })?;
                            match m1165 {
                                None => {
                                    Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m1168 = f277(v0.clone(), {
                                        let a1166 = v16;
                                        let a1167 = 3u64;
                                        nat_add(a1166, a1167)?
                                    }, v4, 1u64)?;
                                    match m1168 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt19, u64), Bytes>((Adt19::C3(std::rc::Rc::new(Adt19::C2(f35(v0, {
                                                let a1169 = v1;
                                                let a1170 = 9u64;
                                                nat_add(a1169, a1170)?
                                            }, {
                                                let a1173 = v16;
                                                let a1174 = {
                                                    let a1171 = v1;
                                                    let a1172 = 9u64;
                                                    nat_add(a1171, a1172)?
                                                };
                                                nat_sub(a1173, a1174)
                                            }))), std::rc::Rc::new({
                                                let (h1175, _) = v18;
                                                h1175
                                            })), {
                                                let (_, h1176) = v18;
                                                h1176
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(Err::<(Adt19, u64), Bytes>(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f275(v0.clone(), v1, Str::lit("<?")) {
                                let m1177 = f385(v0.clone(), v1)?;
                                match m1177 {
                                    Ok(v20) => {
                                        f277(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(Err::<(Adt19, u64), Bytes>(v19))
                                    }
                                }
                            } else {
                                if f275(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a1178 = f276(v0.clone(), v1)?;
                                    let a1179 = 60u64;
                                    if nat_eq(a1178, a1179) {
                                        let m1180 = f277(v0.clone(), v1, v4, 0u64)?;
                                        match m1180 {
                                            Ok(v22) => {
                                                let m1182 = f277(v0, {
                                                    let (_, h1181) = v22;
                                                    h1181
                                                }, v4, 1u64)?;
                                                match m1182 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt19, u64), Bytes>((Adt19::C3(std::rc::Rc::new({
                                                            let (h1183, _) = v22;
                                                            h1183
                                                        }), std::rc::Rc::new({
                                                            let (h1184, _) = v24;
                                                            h1184
                                                        })), {
                                                            let (_, h1185) = v24;
                                                            h1185
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<(Adt19, u64), Bytes>(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(Err::<(Adt19, u64), Bytes>(v21))
                                            }
                                        }
                                    } else {
                                        let m1186 = f388(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m1186 {
                                            Ok(v26) => {
                                                let m1188 = f277(v0, {
                                                    let (_, h1187) = v26;
                                                    h1187
                                                }, v4, 1u64)?;
                                                match m1188 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt19, u64), Bytes>((Adt19::C3(std::rc::Rc::new(Adt19::C2({
                                                            let (h1189, _) = v26;
                                                            h1189
                                                        })), std::rc::Rc::new({
                                                            let (h1190, _) = v28;
                                                            h1190
                                                        })), {
                                                            let (_, h1191) = v28;
                                                            h1191
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(Err::<(Adt19, u64), Bytes>(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(Err::<(Adt19, u64), Bytes>(v25))
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

pub fn f278(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1192 = v2;
    let a1193 = 0u64;
    if nat_eq(a1192, a1193) {
        Ok(f389(v0, 0u64))
    } else {
        let (_, h1194) = {
            let (h1195, _) = f377(Fn5::F390(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1195
        };
        Ok(h1194)
    }
}

pub fn f279() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f280(v0: Adt19, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m1196 = v0;
    match m1196 {
        Adt19::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt19::C1(v2, v3, r1201) => {
            let v4 = (*r1201).clone();
            let m1197 = f280(v4, v1);
            match m1197 {
                Ok(v6) => {
                    if f28(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f28(v3.clone(), f391()) {
                            true
                        } else {
                            f28(v3.clone(), f392())
                        } {
                            Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f90(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f35(v2.clone(), 6u64, {
                                let a1199 = {
                                    let a1198 = v2;
                                    length_bytes(a1198)
                                };
                                let a1200 = 6u64;
                                nat_sub(a1199, a1200)
                            });
                            if f28(v7.clone(), f8(Str::lit("xmlns"))) {
                                Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f28(v7.clone(), f8(Str::lit("xml"))) {
                                    !f28(v3.clone(), f391())
                                } else {
                                    false
                                } {
                                    Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f28(v3.clone(), f391()) {
                                        true
                                    } else {
                                        f28(v3.clone(), f392())
                                    } {
                                        !f28(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f28(v3.clone(), Bytes::lit(&[])) {
                                            Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    Err::<List<(Bytes, Bytes)>, Bytes>(v5)
                }
            }
        }
        Adt19::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt19::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt19::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f281(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f393(v1.clone())?;
    if f28(v2.clone(), Bytes::lit(&[])) {
        let m1202 = f394(v0, Bytes::lit(&[]));
        match m1202 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f28(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f391(), f395(v1)?)))
        } else {
            let m1203 = f394(v0, v2.clone());
            match m1203 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f395(v1)?)))
                }
            }
        }
    }
}

pub fn f282(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f28({
        let (h1204, _) = v0;
        h1204
    }, f396()) {
        f28({
            let (_, h1205) = v0;
            h1205
        }, {
            let a1206 = v1;
            utf8_encode(a1206)
        })
    } else {
        false
    }
}

pub fn f283(p1210: Adt19, v1: Bytes) -> Option<Bytes> {
    let m1209 = p1210;
    {
        let mut m1209 = m1209;
        loop {
            let step = {
                let v0 = m1209;
                let m1207 = v0;
                match m1207 {
                    Adt19::C0(_, _, _) => {
                        Err::<Adt19, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt19::C1(v2, v3, r1208) => {
                        let v4 = (*r1208).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt19, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt19, Option<Bytes>>(v4)
                        }
                    }
                    Adt19::C2(_) => {
                        Err::<Adt19, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt19::C3(_, _) => {
                        Err::<Adt19, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt19::C4 => {
                        Err::<Adt19, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m1209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284(v0: Adt19, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m1211 = v0;
    match m1211 {
        Adt19::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt19::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt19::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt19::C3(r1222, r1223) => {
            let v3 = (*r1222).clone();
            let v4 = (*r1223).clone();
            let m1212 = f284(v4, v1.clone(), v2.clone())?;
            match m1212 {
                Ok(v6) => {
                    let m1213 = v3;
                    match m1213 {
                        Adt19::C0(v7, r1220, r1221) => {
                            let v8 = (*r1220).clone();
                            let v9 = (*r1221).clone();
                            let m1214 = f280(v8.clone(), v1);
                            match m1214 {
                                Ok(v11) => {
                                    let m1215 = f281(v11.clone(), v7)?;
                                    match m1215 {
                                        Ok(v13) => {
                                            if f282(v13, Str::lit("simpleType")) {
                                                let m1216 = f283(v8, f8(Str::lit("name")));
                                                match m1216 {
                                                    None => {
                                                        Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m1217 = f397(v14.clone(), v9, v11, v2)?;
                                                        match m1217 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f300(v14)?, ({
                                                                    let (_, h1218) = v16;
                                                                    h1218
                                                                }, {
                                                                    let (h1219, _) = v16;
                                                                    h1219
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v10))
                                }
                            }
                        }
                        Adt19::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt19::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt19::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt19::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v5))
                }
            }
        }
        Adt19::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f285(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m1224 = v0;
    match m1224.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1225, _) = v1;
                h1225
            }, f285(v2))
        }
    }
}

pub fn f286(p1228: List<Bytes>) -> Option<Bytes> {
    let m1227 = p1228;
    {
        let mut m1227 = m1227;
        loop {
            let step = {
                let v0 = m1227;
                let m1226 = v0;
                match m1226.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287(p1234: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m1233 = p1234;
    {
        let mut m1233 = m1233;
        loop {
            let step = {
                let v0 = m1233;
                let m1229 = v0;
                match m1229.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1232 = f398({
                            let (h1230, _) = {
                                let (_, h1231) = v2;
                                h1231
                            };
                            h1230
                        }, v1.clone());
                        match m1232 {
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
                Ok(next) => { m1233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(p1245: u64, p1246: List<(Bytes, (List<Bytes>, Bytes))>, p1247: List<Bytes>, p1248: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1244 = (p1245, (p1246, (p1247, p1248)));
    {
        let mut m1244 = m1244;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1244;
                let m1235 = v0;
                if m1235 == 0 {
                    let m1236 = v1;
                    match m1236.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m1235 - 1;
                    let m1237 = v1.clone();
                    match m1237.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m1238 = f399(v1, v2.clone());
                            match m1238 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h1239) = v9.clone();
                                        h1239
                                    }, (List::cons({
                                        let (h1240, _) = {
                                            let (h1241, _) = v9.clone();
                                            h1241
                                        };
                                        h1240
                                    }, v2), List::cons({
                                        let (_, h1242) = {
                                            let (h1243, _) = v9;
                                            h1243
                                        };
                                        h1242
                                    }, v3)))))
                                }
                            }
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

pub fn f289(p1251: List<Bytes>, p1252: List<Bytes>) -> List<Bytes> {
    let m1250 = (p1251, p1252);
    {
        let mut m1250 = m1250;
        loop {
            let step = {
                let (v0, v1) = m1250;
                let m1249 = v0;
                match m1249.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(v0: Bytes, v1: Bytes) -> bool {
    let a1253 = v0;
    let a1254 = f309();
    if equal(a1253, a1254) {
        let a1258 = {
            let a1256 = {
                let a1255 = v1;
                length_bytes(a1255)
            };
            let a1257 = 0u64;
            nat_eq(a1256, a1257)
        };
        bool_not(a1258)
    } else {
        false
    }
}

pub fn f291(v0: Bytes) -> Bytes {
    f7(List::cons(f120(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f292(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1260 = v0;
    let a1261 = {
        let a1259 = Str::lit("redistributable");
        utf8_encode(a1259)
    };
    if equal(a1260, a1261) {
        f400(v1)
    } else {
        Ok(false)
    }
}

pub fn f293(v0: Adt13) -> bool {
    if f254(v0.clone(), f8(Str::lit("domain"))) {
        if f254(v0.clone(), f8(Str::lit("row"))) {
            if f41(f47(v0.clone(), f8(Str::lit("relation"))), f401()) {
                if f402(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f402(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f402(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f402(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f294(v0: Adt13) -> bool {
    if f254(v0.clone(), f8(Str::lit("subject"))) {
        if f254(v0.clone(), f8(Str::lit("object"))) {
            if f254(v0.clone(), f8(Str::lit("evidence"))) {
                f41(f47(v0, f8(Str::lit("relation"))), f403())
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

pub fn f295(v0: Option<List<Adt13>>) -> bool {
    let m1262 = v0;
    match m1262 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f296(v0: Option<List<Adt13>>) -> List<Adt13> {
    let m1263 = v0;
    match m1263 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f297(v0: Adt13) -> Bytes {
    if f254(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f254(v0.clone(), f8(Str::lit("id"))) {
            if f254(v0.clone(), f8(Str::lit("statement"))) {
                if f254(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f306(v0, f8(Str::lit("evidence")))
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

pub fn f298(v0: Bytes) -> R<Option<Adt20>> {
    let v1: Bytes = f204(v0)?;
    let m1267 = f404(v1.clone(), 0u64, {
        let a1265 = {
            let a1264 = v1.clone();
            length_bytes(a1264)
        };
        let a1266 = 2u64;
        nat_add(a1265, a1266)?
    }, 0u64)?;
    match m1267 {
        None => {
            Ok(None::<Adt20>)
        }
        Some(v2) => {
            let a1270 = {
                let (_, h1268) = v2;
                h1268
            };
            let a1271 = {
                let a1269 = v1;
                length_bytes(a1269)
            };
            if nat_eq(a1270, a1271) {
                Ok(Some({
                    let (h1272, _) = v2;
                    h1272
                }))
            } else {
                Ok(None::<Adt20>)
            }
        }
    }
}

pub fn f299(v0: Adt20) -> bool {
    if f405(v0.clone(), f8(Str::lit("name"))) {
        if f406(v0.clone(), f8(Str::lit("schema"))) {
            let m1273 = f301(f196(f197(f192(v0, f8(Str::lit("citation"))))));
            match m1273 {
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

pub fn f300(v0: Bytes) -> R<Bytes> {
    f407(v0.clone(), 0u64, {
        let a1274 = v0;
        length_bytes(a1274)
    }, true, Bytes::lit(&[]))
}

pub fn f301(v0: List<Bytes>) -> Option<Adt13> {
    let m1275 = v0;
    match m1275.uncons() {
        None => {
            None::<Adt13>
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("req"))) {
                let m1276 = v2;
                match m1276.uncons() {
                    None => {
                        None::<Adt13>
                    }
                    Some((v3, _)) => {
                        Some(Adt13::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5)))
                    }
                }
            } else {
                if f28(v1.clone(), f8(Str::lit("std"))) {
                    let m1277 = v2;
                    match m1277.uncons() {
                        None => {
                            None::<Adt13>
                        }
                        Some((v5, v6)) => {
                            Some(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v5)), std::rc::Rc::new({
                                let m1278 = v6;
                                match m1278.uncons() {
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
                        let m1279 = v2;
                        match m1279.uncons() {
                            None => {
                                None::<Adt13>
                            }
                            Some((v9, v10)) => {
                                let m1280 = v10;
                                match m1280.uncons() {
                                    None => {
                                        None::<Adt13>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt13::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt13::C2(v11)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f266(v12)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt13::C2(v9)), std::rc::Rc::new(Adt13::C5))))))))), std::rc::Rc::new(Adt13::C5)))
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

pub fn f302(v0: Adt1, v1: Bytes, v2: Adt20, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1281 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1281.uncons() {
        None => {
            let m1282 = f301(v4);
            match m1282 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1283 = f8(Str::lit("Catalog."));
                        let a1284 = f300(v3.clone())?;
                        append_bytes(a1283, a1284)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f300(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1285 = f408(v2, v9.clone(), f300(v3.clone())?)?;
                    match m1285 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f300(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f97(Adt13::C3(std::rc::Rc::new(f98(f409(f93(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f97(Adt13::C3(std::rc::Rc::new(f98(f409(f93(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt13::C2(v10.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt13::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt13::C1(false, 0u64)), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1289 = f410(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f35(v10.clone(), 4u64, {
                                    let a1287 = {
                                        let a1286 = v10;
                                        length_bytes(a1286)
                                    };
                                    let a1288 = 4u64;
                                    nat_sub(a1287, a1288)
                                }))?;
                                let a1290 = Bytes::lit(&[0x0a]);
                                append_bytes(a1289, a1290)
                            }), List::cons(f161(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f303(v0: Adt20) -> R<Bytes> {
    let m1291 = v0;
    match m1291 {
        Adt20::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt20::C1(v2, v3) => {
            f318(v2, v3)
        }
        Adt20::C2(v4, v5, v6) => {
            f411(v4, v5, v6)
        }
        Adt20::C3(v7) => {
            f319(v7)
        }
        Adt20::C4(r1292) => {
            let v8 = (*r1292).clone();
            Ok(f320(f412(v8)?))
        }
        Adt20::C5(r1293) => {
            let v9 = (*r1293).clone();
            f322(f413(v9)?)
        }
        Adt20::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt20::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt20::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f304(v0: Option<Adt20>) -> Option<Bytes> {
    let m1294 = v0;
    match m1294 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1295 = v1;
            match m1295 {
                Adt20::C0(_) => {
                    None::<Bytes>
                }
                Adt20::C1(_, _) => {
                    None::<Bytes>
                }
                Adt20::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt20::C3(v2) => {
                    Some(v2)
                }
                Adt20::C4(_) => {
                    None::<Bytes>
                }
                Adt20::C5(_) => {
                    None::<Bytes>
                }
                Adt20::C6 => {
                    None::<Bytes>
                }
                Adt20::C7(_, _) => {
                    None::<Bytes>
                }
                Adt20::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f305(v0: Adt20) -> List<Adt20> {
    let m1296 = v0;
    match m1296 {
        Adt20::C0(_) => {
            List::<Adt20>::nil()
        }
        Adt20::C1(_, _) => {
            List::<Adt20>::nil()
        }
        Adt20::C2(_, _, _) => {
            List::<Adt20>::nil()
        }
        Adt20::C3(_) => {
            List::<Adt20>::nil()
        }
        Adt20::C4(r1297) => {
            let v3 = (*r1297).clone();
            f305(v3)
        }
        Adt20::C5(_) => {
            List::<Adt20>::nil()
        }
        Adt20::C6 => {
            List::<Adt20>::nil()
        }
        Adt20::C7(r1298, r1299) => {
            let v1 = (*r1298).clone();
            let v2 = (*r1299).clone();
            List::cons(v1, f305(v2))
        }
        Adt20::C8(_, _, _) => {
            List::<Adt20>::nil()
        }
    }
}

pub fn f306(v0: Adt13, v1: Bytes) -> bool {
    let m1300 = f89(v0, v1);
    match m1300 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f307(v0: Adt13, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1301 = f89(v0, v1);
    match m1301 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v3) => {
            f200(f89(v3, v2))
        }
    }
}

pub fn f308(v0: Adt13) -> List<Adt13> {
    let m1302 = v0;
    match m1302 {
        Adt13::C0(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C1(_, _) => {
            List::<Adt13>::nil()
        }
        Adt13::C2(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C3(r1303) => {
            let v3 = (*r1303).clone();
            f308(v3)
        }
        Adt13::C4(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C5 => {
            List::<Adt13>::nil()
        }
        Adt13::C6(r1304, r1305) => {
            let v1 = (*r1304).clone();
            let v2 = (*r1305).clone();
            List::cons(v1, f308(v2))
        }
        Adt13::C7(_, _, _) => {
            List::<Adt13>::nil()
        }
    }
}

pub fn f309() -> Bytes {
    let a1306 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1306)
}

pub fn f310(v0: Adt13, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt13> = f89(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f157(Str::lit("id"), f47(v0.clone(), f8(Str::lit("id"))))?, List::cons(f157(Str::lit("name"), f47(v0.clone(), f8(Str::lit("name"))))?, List::cons(f157(Str::lit("issuer"), f47(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f157(Str::lit("edition"), f47(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f157(Str::lit("canonical_id"), f47(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f157(Str::lit("source_role"), f47(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f157(Str::lit("retrieval_date"), f47(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f157(Str::lit("url"), f414(v2.clone(), f8(Str::lit("url"))))?, List::cons(f157(Str::lit("media_type"), f414(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f157(Str::lit("license"), f414(v2.clone(), f8(Str::lit("license"))))?, List::cons(f157(Str::lit("redistribution"), f414(v2, f8(Str::lit("redistribution"))))?, List::cons(f157(Str::lit("statement"), f47(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1307 = v1;
        match m1307 {
            None => {
                f7(List::cons(f157(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f157(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f157(Str::lit("artifact"), {
                    let (h1308, _) = v3;
                    h1308
                })?, List::cons(f157(Str::lit("checksum"), {
                    let (_, h1309) = v3;
                    h1309
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f415(f118(f200(f89(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f311(v0: u64) -> bool {
    let a1310 = v0;
    let a1311 = 127u64;
    nat_le(a1310, a1311)
}

pub fn f312(v0: Bytes, p1391: u64, p1392: u64, p1393: bool, p1394: bool, p1395: Bytes) -> R<Bytes> {
    let m1390 = (p1391, (p1392, (p1393, (p1394, p1395))));
    {
        let mut m1390 = m1390;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1390;
                let m1312 = v2;
                if m1312 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1312 - 1;
                    let m1315 = {
                        let a1313 = v0.clone();
                        let a1314 = v1;
                        index_bytes(a1313, a1314)
                    };
                    match m1315 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1360 = {
                                    let a1336 = {
                                        let a1324 = {
                                            let a1318 = {
                                                let a1316 = v8;
                                                let a1317 = 128u8;
                                                fixed_u8::bit_and(a1316, a1317)
                                            };
                                            let a1319 = 128u8;
                                            if equal(a1318, a1319) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1325 = {
                                            let a1322 = {
                                                let a1320 = v8;
                                                let a1321 = 64u8;
                                                fixed_u8::bit_and(a1320, a1321)
                                            };
                                            let a1323 = 64u8;
                                            if equal(a1322, a1323) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1324, a1325)?
                                    };
                                    let a1337 = {
                                        let a1334 = {
                                            let a1328 = {
                                                let a1326 = v8;
                                                let a1327 = 32u8;
                                                fixed_u8::bit_and(a1326, a1327)
                                            };
                                            let a1329 = 32u8;
                                            if equal(a1328, a1329) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1335 = {
                                            let a1332 = {
                                                let a1330 = v8;
                                                let a1331 = 16u8;
                                                fixed_u8::bit_and(a1330, a1331)
                                            };
                                            let a1333 = 16u8;
                                            if equal(a1332, a1333) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1334, a1335)?
                                    };
                                    nat_add(a1336, a1337)?
                                };
                                let a1361 = {
                                    let a1358 = {
                                        let a1346 = {
                                            let a1340 = {
                                                let a1338 = v8;
                                                let a1339 = 8u8;
                                                fixed_u8::bit_and(a1338, a1339)
                                            };
                                            let a1341 = 8u8;
                                            if equal(a1340, a1341) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1347 = {
                                            let a1344 = {
                                                let a1342 = v8;
                                                let a1343 = 4u8;
                                                fixed_u8::bit_and(a1342, a1343)
                                            };
                                            let a1345 = 4u8;
                                            if equal(a1344, a1345) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1346, a1347)?
                                    };
                                    let a1359 = {
                                        let a1356 = {
                                            let a1350 = {
                                                let a1348 = v8;
                                                let a1349 = 2u8;
                                                fixed_u8::bit_and(a1348, a1349)
                                            };
                                            let a1351 = 2u8;
                                            if equal(a1350, a1351) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1357 = {
                                            let a1354 = {
                                                let a1352 = v8;
                                                let a1353 = 1u8;
                                                fixed_u8::bit_and(a1352, a1353)
                                            };
                                            let a1355 = 1u8;
                                            if equal(a1354, a1355) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1356, a1357)?
                                    };
                                    nat_add(a1358, a1359)?
                                };
                                nat_add(a1360, a1361)?
                            };
                            let v10: Bytes = f35(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1362 = v1;
                                    let a1363 = 1u64;
                                    nat_add(a1362, a1363)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1364 = v9;
                                    let a1365 = 34u64;
                                    !nat_eq(a1364, a1365)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1366 = v9;
                                    let a1367 = 92u64;
                                    nat_eq(a1366, a1367)
                                }, {
                                    let a1368 = v5;
                                    let a1369 = v10;
                                    append_bytes(a1368, a1369)
                                })))))
                            } else {
                                let a1370 = v9;
                                let a1371 = 32u64;
                                if if if if nat_eq(a1370, a1371) {
                                    true
                                } else {
                                    let a1372 = v9;
                                    let a1373 = 9u64;
                                    nat_eq(a1372, a1373)
                                } {
                                    true
                                } else {
                                    let a1374 = v9;
                                    let a1375 = 10u64;
                                    nat_eq(a1374, a1375)
                                } {
                                    true
                                } else {
                                    let a1376 = v9;
                                    let a1377 = 13u64;
                                    nat_eq(a1376, a1377)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1378 = v1;
                                        let a1379 = 1u64;
                                        nat_add(a1378, a1379)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1380 = v1;
                                            let a1381 = 4u64;
                                            nat_add(a1380, a1381)?
                                        }, (v6, (false, (false, {
                                            let a1382 = v5;
                                            let a1383 = f8(Str::lit("[]"));
                                            append_bytes(a1382, a1383)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1384 = v1;
                                            let a1385 = 1u64;
                                            nat_add(a1384, a1385)?
                                        }, (v6, ({
                                            let a1386 = v9;
                                            let a1387 = 34u64;
                                            nat_eq(a1386, a1387)
                                        }, (false, {
                                            let a1388 = v5;
                                            let a1389 = v10;
                                            append_bytes(a1388, a1389)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: u64) -> R<u64> {
    let m1398 = {
        let a1396 = v0;
        let a1397 = v1;
        index_bytes(a1396, a1397)
    };
    match m1398 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1443 = {
                let a1419 = {
                    let a1407 = {
                        let a1401 = {
                            let a1399 = v3;
                            let a1400 = 128u8;
                            fixed_u8::bit_and(a1399, a1400)
                        };
                        let a1402 = 128u8;
                        if equal(a1401, a1402) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1408 = {
                        let a1405 = {
                            let a1403 = v3;
                            let a1404 = 64u8;
                            fixed_u8::bit_and(a1403, a1404)
                        };
                        let a1406 = 64u8;
                        if equal(a1405, a1406) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1407, a1408)?
                };
                let a1420 = {
                    let a1417 = {
                        let a1411 = {
                            let a1409 = v3;
                            let a1410 = 32u8;
                            fixed_u8::bit_and(a1409, a1410)
                        };
                        let a1412 = 32u8;
                        if equal(a1411, a1412) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1418 = {
                        let a1415 = {
                            let a1413 = v3;
                            let a1414 = 16u8;
                            fixed_u8::bit_and(a1413, a1414)
                        };
                        let a1416 = 16u8;
                        if equal(a1415, a1416) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1417, a1418)?
                };
                nat_add(a1419, a1420)?
            };
            let a1444 = {
                let a1441 = {
                    let a1429 = {
                        let a1423 = {
                            let a1421 = v3;
                            let a1422 = 8u8;
                            fixed_u8::bit_and(a1421, a1422)
                        };
                        let a1424 = 8u8;
                        if equal(a1423, a1424) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1430 = {
                        let a1427 = {
                            let a1425 = v3;
                            let a1426 = 4u8;
                            fixed_u8::bit_and(a1425, a1426)
                        };
                        let a1428 = 4u8;
                        if equal(a1427, a1428) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1429, a1430)?
                };
                let a1442 = {
                    let a1439 = {
                        let a1433 = {
                            let a1431 = v3;
                            let a1432 = 2u8;
                            fixed_u8::bit_and(a1431, a1432)
                        };
                        let a1434 = 2u8;
                        if equal(a1433, a1434) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1440 = {
                        let a1437 = {
                            let a1435 = v3;
                            let a1436 = 1u8;
                            fixed_u8::bit_and(a1435, a1436)
                        };
                        let a1438 = 1u8;
                        if equal(a1437, a1438) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1439, a1440)?
                };
                nat_add(a1441, a1442)?
            };
            nat_add(a1443, a1444)
        }
    }
}

pub fn f314(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f416(Fn6::F417(v0.clone()), {
        let a1446 = {
            let a1445 = v0;
            length_bytes(a1445)
        };
        let a1447 = 1u64;
        nat_add(a1446, a1447)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1451 = {
        let (_, h1448) = {
            let (_, h1449) = {
                let (h1450, _) = v2.clone();
                h1450
            };
            h1449
        };
        h1448
    };
    let a1452 = 1u64;
    if nat_eq(a1451, a1452) {
        Ok(Some(({
            let (h1453, _) = {
                let (h1454, _) = v2.clone();
                h1454
            };
            h1453
        }, {
            let (h1455, _) = {
                let (_, h1456) = {
                    let (h1457, _) = v2;
                    h1457
                };
                h1456
            };
            h1455
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f315(v0: Bytes, v1: u64) -> R<Option<(u64, Adt13)>> {
    let v2: bool = {
        let a1458 = f313(v0.clone(), v1)?;
        let a1459 = 45u64;
        nat_eq(a1458, a1459)
    };
    let v3: u64 = if v2 {
        let a1460 = v1;
        let a1461 = 1u64;
        nat_add(a1460, a1461)?
    } else {
        v1
    };
    let v4: u64 = f418(v0.clone(), v3)?;
    let v5: u64 = {
        let a1462 = v4;
        let a1463 = v3;
        nat_sub(a1462, a1463)
    };
    let v6: bool = {
        let a1464 = f313(v0.clone(), v4)?;
        let a1465 = 46u64;
        nat_eq(a1464, a1465)
    };
    let v7: u64 = if v6 {
        let a1466 = v4;
        let a1467 = 1u64;
        nat_add(a1466, a1467)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f418(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f313(v0.clone(), v8)?;
    let v10: bool = {
        let a1468 = v9;
        let a1469 = 101u64;
        if nat_eq(a1468, a1469) {
            true
        } else {
            let a1470 = v9;
            let a1471 = 69u64;
            nat_eq(a1470, a1471)
        }
    };
    let v11: u64 = f313(v0.clone(), {
        let a1472 = v8;
        let a1473 = 1u64;
        nat_add(a1472, a1473)?
    })?;
    let v12: bool = if v10 {
        let a1474 = v11;
        let a1475 = 43u64;
        if nat_eq(a1474, a1475) {
            true
        } else {
            let a1476 = v11;
            let a1477 = 45u64;
            nat_eq(a1476, a1477)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a1478 = v8;
            let a1479 = 2u64;
            nat_add(a1478, a1479)?
        } else {
            let a1480 = v8;
            let a1481 = 1u64;
            nat_add(a1480, a1481)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f418(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f35(v0.clone(), v13, {
        let a1482 = v14;
        let a1483 = v13;
        nat_sub(a1482, a1483)
    });
    let v16: u64 = f419(v15.clone())?;
    let v17: Bytes = f35(v15.clone(), v16, {
        let a1485 = {
            let a1484 = v15;
            length_bytes(a1484)
        };
        let a1486 = v16;
        nat_sub(a1485, a1486)
    });
    let a1487 = v5;
    let a1488 = 0u64;
    if if nat_eq(a1487, a1488) {
        true
    } else {
        let a1489 = 1u64;
        let a1490 = v5;
        if if nat_lt(a1489, a1490) {
            let a1491 = f313(v0.clone(), v3)?;
            let a1492 = 48u64;
            nat_eq(a1491, a1492)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a1493 = v8;
                let a1494 = v7;
                nat_eq(a1493, a1494)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a1495 = v14;
                    let a1496 = v13;
                    nat_eq(a1495, a1496)
                } else {
                    false
                } {
                    true
                } else {
                    let a1498 = 18u64;
                    let a1499 = {
                        let a1497 = v17.clone();
                        length_bytes(a1497)
                    };
                    nat_lt(a1498, a1499)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt13)>)
    } else {
        let m1503 = {
            let a1501 = {
                let a1500 = v17.clone();
                length_bytes(a1500)
            };
            let a1502 = 0u64;
            if nat_eq(a1501, a1502) {
                Some(0u64)
            } else {
                f420(v17)?
            }
        };
        match m1503 {
            None => {
                Ok(None::<(u64, Adt13)>)
            }
            Some(v18) => {
                let m1508 = f421(f35(v0.clone(), v3, v5), f35(v0, v7, {
                    let a1504 = v8;
                    let a1505 = v7;
                    nat_sub(a1504, a1505)
                }), if v12 {
                    let a1506 = v11;
                    let a1507 = 45u64;
                    nat_eq(a1506, a1507)
                } else {
                    false
                }, v18)?;
                match m1508 {
                    None => {
                        Ok(None::<(u64, Adt13)>)
                    }
                    Some(v19) => {
                        let a1510 = 19u64;
                        let a1511 = {
                            let a1509 = v19.clone();
                            length_bytes(a1509)
                        };
                        if nat_lt(a1510, a1511) {
                            Ok(None::<(u64, Adt13)>)
                        } else {
                            let m1512 = f420(v19)?;
                            match m1512 {
                                None => {
                                    Ok(None::<(u64, Adt13)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a1513 = 0u64;
                                        let a1514 = v20;
                                        nat_lt(a1513, a1514)
                                    } else {
                                        false
                                    };
                                    if f422(v21, v20) {
                                        Ok(Some((v14, Adt13::C1(v21, v20))))
                                    } else {
                                        Ok(None::<(u64, Adt13)>)
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

pub fn f316(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1515, _) = v1.clone();
        h1515
    };
    let v3: u64 = {
        let (h1516, _) = {
            let (_, h1517) = v1.clone();
            h1517
        };
        h1516
    };
    let v4: List<Bytes> = {
        let (_, h1518) = {
            let (_, h1519) = v1;
            h1519
        };
        h1518
    };
    let m1520 = f218(v0.clone(), v2);
    match m1520 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1521 = v5;
            let a1522 = 10u8;
            if equal(a1521, a1522) {
                Ok(Some(({
                    let a1523 = v2;
                    let a1524 = 1u64;
                    nat_add(a1523, a1524)?
                }, ({
                    let a1525 = v2;
                    let a1526 = 1u64;
                    nat_add(a1525, a1526)?
                }, List::cons(f423(f35(v0, v3, {
                    let a1527 = v2;
                    let a1528 = v3;
                    nat_sub(a1527, a1528)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1529 = v2;
                    let a1530 = 1u64;
                    nat_add(a1529, a1530)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f317(v0: Adt13, v1: Bytes) -> Adt13 {
    let m1531 = v0;
    match m1531 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r1532) => {
            let v10 = (*r1532).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r1533) => {
            let v5 = (*r1533).clone();
            Adt13::C4(std::rc::Rc::new(f317(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r1534, r1535) => {
            let v11 = (*r1534).clone();
            let v12 = (*r1535).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r1536, r1537) => {
            let v3 = (*r1536).clone();
            let v4 = (*r1537).clone();
            if f28(v2.clone(), f8(Str::lit("authority"))) {
                Adt13::C7(v2, std::rc::Rc::new(f424(v3, v1.clone())), std::rc::Rc::new(f317(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f317(v4, v1)))
            }
        }
    }
}

pub fn f318(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1538 = 0u64;
        let a1539 = v1;
        nat_lt(a1538, a1539)
    } else {
        false
    } {
        let a1540 = Bytes::lit(&[0x2d]);
        let a1541 = f167(v1)?;
        Ok(append_bytes(a1540, a1541))
    } else {
        f167(v1)
    }
}

pub fn f319(v0: Bytes) -> R<Bytes> {
    let a1547 = {
        let a1545 = Bytes::lit(&[0x22]);
        let a1546 = {
            let (_, h1542) = {
                let (h1543, _) = f377(Fn5::F425(v0.clone()), {
                    let a1544 = v0;
                    length_bytes(a1544)
                }, (0u64, Bytes::lit(&[])))?;
                h1543
            };
            h1542
        };
        append_bytes(a1545, a1546)
    };
    let a1548 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1547, a1548))
}

pub fn f320(v0: List<Bytes>) -> Bytes {
    let a1551 = {
        let a1549 = Bytes::lit(&[0x5b]);
        let a1550 = f162(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1549, a1550)
    };
    let a1552 = Bytes::lit(&[0x5d]);
    append_bytes(a1551, a1552)
}

pub fn f321(v0: Adt13) -> R<List<Bytes>> {
    let m1553 = v0;
    match m1553 {
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
        Adt13::C6(r1554, r1555) => {
            let v7 = (*r1554).clone();
            let v8 = (*r1555).clone();
            Ok(List::cons(f209(v7)?, f321(v8)?))
        }
        Adt13::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f322(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f426(Fn7::F427, List::<(Bytes, Bytes)>::nil(), f428(v0)?);
    let v2: List<Bytes> = f429(Fn8::F430, List::<Bytes>::nil(), v1)?;
    let a1558 = {
        let a1556 = Bytes::lit(&[0x7b]);
        let a1557 = f162(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1556, a1557)
    };
    let a1559 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1558, a1559))
}

pub fn f323(v0: Adt13) -> R<List<(Bytes, Bytes)>> {
    let m1560 = v0;
    match m1560 {
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
        Adt13::C7(v9, r1561, r1562) => {
            let v10 = (*r1561).clone();
            let v11 = (*r1562).clone();
            Ok(List::cons((v9, f209(v10)?), f323(v11)?))
        }
    }
}

pub fn f324() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f325(v0: Adt12) -> Bytes {
    let m1563 = v0;
    match m1563 {
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

pub fn f326(v0: Adt12, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f431(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f431(f8(Str::lit("id")), f58(v0.clone(), f50(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f431(f8(Str::lit("name")), f432(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f431(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f431(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f431(f8(Str::lit("statement")), f433(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f431(f8(Str::lit("media_type")), f434(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f431(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f431(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f431(f8(Str::lit("url")), f212(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f327(v0: Bytes) -> R<u64> {
    let (h1564, _) = f331(Fn3::F435(v0.clone()), {
        let a1565 = v0.clone();
        length_bytes(a1565)
    }, {
        let a1566 = v0;
        length_bytes(a1566)
    })?;
    Ok(h1564)
}

pub fn f328(v0: Bytes, v1: u64) -> R<u64> {
    let (h1567, _) = f331(Fn3::F436(v1, v0), v1, 0u64)?;
    Ok(h1567)
}

pub fn f329(v0: Bytes) -> bool {
    f179(v0, f8(Str::lit(".lex.tex")))
}

pub fn f330(v0: Bytes) -> R<bool> {
    let v1: Bytes = f217(v0)?;
    let v2: u64 = if f90(v1.clone(), f8(Str::lit("if ! "))) {
        5u64
    } else {
        if f90(v1.clone(), f8(Str::lit("elif ! "))) {
            7u64
        } else {
            0u64
        }
    };
    let v3: Bytes = f35(v1.clone(), v2, {
        let a1569 = {
            let a1568 = v1;
            length_bytes(a1568)
        };
        let a1570 = v2;
        nat_sub(a1569, a1570)
    });
    let v5: Bytes = if f90(v3.clone(), f8(Str::lit("("))) {
        let m1571 = f222(v3.clone(), f8(Str::lit(")")))?;
        match m1571 {
            None => {
                v3
            }
            Some(v4) => {
                f35(v3.clone(), {
                    let a1572 = v4;
                    let a1573 = 1u64;
                    nat_add(a1572, a1573)?
                }, {
                    let a1577 = {
                        let a1574 = v3;
                        length_bytes(a1574)
                    };
                    let a1578 = {
                        let a1575 = v4;
                        let a1576 = 1u64;
                        nat_add(a1575, a1576)?
                    };
                    nat_sub(a1577, a1578)
                })
            }
        }
    } else {
        v3
    };
    let a1579 = 0u64;
    let a1580 = v2;
    if nat_lt(a1579, a1580) {
        f437(v5)
    } else {
        Ok(false)
    }
}

pub fn f331(v0: Fn3, p1585: u64, p1586: u64) -> R<(u64, bool)> {
    let m1584 = (p1585, p1586);
    {
        let mut m1584 = m1584;
        loop {
            let step = {
                let (v1, v2) = m1584;
                let m1581 = v1;
                if m1581 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1581 - 1;
                    let m1583 = {
                        let c1582: Fn3 = v0.clone();
                        c1582.apply(v2)?
                    };
                    match m1583 {
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
                Ok(next) => { m1584 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f269(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1589 = {
            let a1587 = v4;
            let a1588 = v1;
            nat_add(a1587, a1588)?
        };
        let a1590 = v3;
        if nat_lt(a1589, a1590) {
            Ok(Some({
                let a1591 = v4;
                let a1592 = 1u64;
                nat_add(a1591, a1592)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f333(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1593 = v0;
    match m1593.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f333(v3, {
                let a1594 = v1;
                let a1595 = 1u64;
                nat_add(a1594, a1595)?
            })?))
        }
    }
}

pub fn f334(v0: Adt17, v1: List<u64>, v2: Adt22) -> bool {
    let m1596 = v2;
    match m1596 {
        Adt22::C0(v3, v4) => {
            if f340(v3, match v0.clone() {
                Adt17::C0(h1597, _, _, _) => {
                    h1597
                }
            }) {
                f438(v0, v4)
            } else {
                false
            }
        }
        Adt22::C1(v5) => {
            f340(v5, match v0 {
                Adt17::C0(_, _, h1598, _) => {
                    h1598
                }
            })
        }
        Adt22::C2(_, v7, v8) => {
            if f336(v7, v1) {
                f340(v8, match v0 {
                    Adt17::C0(_, _, _, h1599) => {
                        h1599
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f335(v0: Adt14, v1: Adt13) -> R<Adt22> {
    let m1600 = f89(v1.clone(), f8(Str::lit("std")));
    match m1600 {
        None => {
            let m1605 = f88(f89(v1.clone(), f8(Str::lit("req"))));
            match m1605 {
                None => {
                    let m1607 = f89(v1, f8(Str::lit("der")));
                    match m1607 {
                        None => {
                            Ok(f439(v0))
                        }
                        Some(v5) => {
                            let m1608 = f440(f47(v5.clone(), f8(Str::lit("rule"))));
                            match m1608 {
                                None => {
                                    Ok(f439(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt22::C2(v6, f230(f118(f200(f89(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt14::C0(_, _, _, _, h1609) => {
                                            h1609
                                        }
                                    })?, f338(f47(v5, f8(Str::lit("check"))), match v0 {
                                        Adt14::C0(_, _, _, h1610, _) => {
                                            h1610
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt22::C1(f338(v4, match v0 {
                        Adt14::C0(_, _, h1606, _, _) => {
                            h1606
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1601 = f88(f89(v2.clone(), f8(Str::lit("row"))));
            match m1601 {
                None => {
                    Ok(Adt22::C0(f338(f47(v2, f8(Str::lit("authority"))), match v0 {
                        Adt14::C0(h1602, _, _, _, _) => {
                            h1602
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt22::C0(f338(f47(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt14::C0(h1603, _, _, _, _) => {
                            h1603
                        }
                    })?, Some(f338(v3, match v0 {
                        Adt14::C0(_, h1604, _, _, _) => {
                            h1604
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f336(p1613: List<u64>, v1: List<u64>) -> bool {
    let m1612 = p1613;
    {
        let mut m1612 = m1612;
        loop {
            let step = {
                let v0 = m1612;
                let m1611 = v0;
                match m1611.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f340(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1612 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(p1618: List<u64>) -> bool {
    let m1617 = p1618;
    {
        let mut m1617 = m1617;
        loop {
            let step = {
                let v0 = m1617;
                let m1614 = v0;
                match m1614.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1615 = v1;
                        let a1616 = 7u64;
                        if nat_le(a1615, a1616) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1619 = f441(v0, v1.clone(), 0u64)?;
    match m1619 {
        None => {
            let a1620 = v1;
            Ok(length_list(a1620))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f339(v0: Adt13) -> u64 {
    let m1621 = f442(f89(v0, f8(Str::lit("stratum"))));
    match m1621 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f340(v0: u64, p1626: List<u64>) -> bool {
    let m1625 = p1626;
    {
        let mut m1625 = m1625;
        loop {
            let step = {
                let v1 = m1625;
                let m1622 = v1;
                match m1622.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1623 = v2;
                        let a1624 = v0;
                        if nat_eq(a1623, a1624) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f342(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1627 = v0.clone();
    match m1627.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1628 = v2;
            let a1629 = v1.clone();
            if equal(a1628, a1629) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f343(v0: List<(Bytes, ())>) -> List<Bytes> {
    f234(f443(Fn9::F444, List::<Bytes>::nil(), v0))
}

pub fn f344(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f445(f446(Fn10::F447, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1634 = {
        let (h1630, _) = f448(Fn11::F449, {
            let a1632 = {
                let a1631 = v0;
                length_list(a1631)
            };
            let a1633 = 1u64;
            nat_add(a1632, a1633)?
        }, v1)?;
        h1630
    };
    match m1634.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f345(v0: List<Bytes>) -> List<(Bytes, ())> {
    f450(f451(Fn12::F452, List::<(Bytes, ())>::nil(), v0))
}

pub fn f346(p1637: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1636 = p1637;
    {
        let mut m1636 = m1636;
        loop {
            let step = {
                let v0 = m1636;
                let m1635 = v0;
                match m1635.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f375(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1636 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: Adt13, v1: Bytes) -> Bytes {
    if f28(f414(f89(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f348(v0: Adt1, v1: Bytes) -> R<Adt20> {
    let m1638 = f191(v0, v1)?;
    match m1638 {
        None => {
            Ok(Adt20::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f349(v0: Bytes) -> R<Bytes> {
    let a1639 = f8(Str::lit("Catalog."));
    let a1640 = f300(v0)?;
    Ok(append_bytes(a1639, a1640))
}

pub fn f350(v0: Adt20) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f300(f195(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f351(v0: List<Bytes>) -> List<Bytes> {
    let m1641 = v0;
    match m1641.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f179(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f175(f183(), v1), f351(v2))
            } else {
                f351(v2)
            }
        }
    }
}

pub fn f352(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1642 = v1;
    match m1642.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f453(v0.clone(), v2)?;
            let (h1643, _) = v4.clone();
            if h1643 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f352(v0, v3)?;
                let (h1644, _) = v5.clone();
                if h1644 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1647 = {
                            let (_, h1645) = v4;
                            h1645
                        };
                        let a1648 = {
                            let (_, h1646) = v5;
                            h1646
                        };
                        append_bytes(a1647, a1648)
                    }))
                }
            }
        }
    }
}

pub fn f353(v0: Adt16, v1: List<Adt13>) -> List<Bytes> {
    let m1649 = v1;
    match m1649.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f254(v2.clone(), f8(Str::lit("subject"))) {
                if f255(v0.clone(), v2.clone()) {
                    f353(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f47(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f353(v0, v3))
                }
            } else {
                if f362(v0.clone(), v2.clone()) {
                    f353(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f363(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f353(v0, v3))
                }
            }
        }
    }
}

pub fn f354(v0: Adt1) -> R<Bytes> {
    let a1651 = {
        let a1650 = f145(f146(f163(f164(f154(f53(v0.clone(), f158()))?)), f147(f93(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f148(f93(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1650)
    };
    let a1652 = 0u64;
    if nat_eq(a1651, a1652) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f355(v0: List<Adt13>) -> R<List<Bytes>> {
    let m1653 = v0;
    match m1653.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f162(f118(f200(f89(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f47(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f47(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f454(f92(f47(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f355(v2)?))
        }
    }
}

pub fn f356(v0: Bytes, v1: List<Adt24>) -> List<Bytes> {
    let m1654 = v1;
    match m1654.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(match v2.clone() {
                Adt24::C0(h1655, _, _) => {
                    h1655
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt24::C0(_, h1656, _) => {
                        h1656
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt24::C0(_, _, h1657) => {
                        h1657
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f356(v0, v3))
            } else {
                f356(v0, v3)
            }
        }
    }
}

pub fn f357(v0: Bytes, v1: List<Adt13>) -> List<Bytes> {
    let m1658 = v1;
    match m1658.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(f47(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f357(v0, v3))
            } else {
                if f28(f47(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f47(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f357(v0, v3))
                } else {
                    f357(v0, v3)
                }
            }
        }
    }
}

pub fn f358(p1663: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1662 = p1663;
    {
        let mut m1662 = m1662;
        loop {
            let step = {
                let v0 = m1662;
                let m1659 = v0;
                match m1659.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1660 = v2;
                        match m1660.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f28(v4, v1.clone()) {
                                    let m1661 = v5;
                                    match m1661.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f455(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m1664 = v0;
    match m1664.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1665 = f89(v1, f8(Str::lit("std")));
            match m1665 {
                None => {
                    f359(v2)
                }
                Some(v3) => {
                    if f254(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f47(v3.clone(), f8(Str::lit("row"))), f47(v3, f8(Str::lit("authority")))), f359(v2))
                    } else {
                        f359(v2)
                    }
                }
            }
        }
    }
}

pub fn f360(v0: Adt16, v1: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m1666 = v1;
    match m1666.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f47(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f47(v2, f8(Str::lit("object")));
            if if f41(v4.clone(), match v0.clone() {
                Adt16::C0(h1667, _, _) => {
                    h1667
                }
            }) {
                f41(v5.clone(), match v0.clone() {
                    Adt16::C0(_, h1668, _) => {
                        h1668
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f360(v0, v3))
            } else {
                if if f41(v5.clone(), match v0.clone() {
                    Adt16::C0(h1669, _, _) => {
                        h1669
                    }
                }) {
                    f41(v4.clone(), match v0.clone() {
                        Adt16::C0(_, h1670, _) => {
                            h1670
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f360(v0, v3))
                } else {
                    f360(v0, v3)
                }
            }
        }
    }
}

pub fn f361(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt24>> {
    let m1671 = v0;
    match m1671.uncons() {
        None => {
            Ok(List::<Adt24>::nil())
        }
        Some((v4, v5)) => {
            if if f456(v4.clone(), v2.clone()) {
                true
            } else {
                f456(v4.clone(), v3.clone())
            } {
                f361(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt24::C0({
                    let (h1672, _) = v4.clone();
                    h1672
                }, {
                    let a1675 = {
                        let a1674 = f121(f457({
                            let (h1673, _) = v4.clone();
                            h1673
                        }, v1.clone()))?;
                        length_list(a1674)
                    };
                    let a1676 = 1u64;
                    if nat_eq(a1675, a1676) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1677) = v4.clone();
                    h1677
                }), f361(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f362(v0: Adt16, v1: Adt13) -> bool {
    if f41(f47(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt16::C0(h1678, _, _) => {
            h1678
        }
    }) {
        f41(f363(v1), match v0 {
            Adt16::C0(_, h1679, _) => {
                h1679
            }
        })
    } else {
        false
    }
}

pub fn f363(v0: Adt13) -> Bytes {
    let m1680 = f89(v0, f8(Str::lit("evidence")));
    match m1680 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f47(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f364(v0: Adt13) -> Bytes {
    if f254(v0.clone(), f8(Str::lit("req"))) {
        let a1681 = f8(Str::lit("req "));
        let a1682 = f47(v0, f8(Str::lit("req")));
        append_bytes(a1681, a1682)
    } else {
        let m1683 = f89(v0.clone(), f8(Str::lit("std")));
        match m1683 {
            None => {
                let m1686 = f89(v0, f8(Str::lit("der")));
                match m1686 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f47(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f162(f118(f200(f89(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f47(v1.clone(), f8(Str::lit("authority"))), List::cons(if f254(v1.clone(), f8(Str::lit("row"))) {
                    let a1684 = f8(Str::lit(" row "));
                    let a1685 = f47(v1, f8(Str::lit("row")));
                    append_bytes(a1684, a1685)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f365(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1687 = v1;
    match m1687.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41({
                let (h1688, _) = v2.clone();
                h1688
            }, v0.clone()) {
                List::cons({
                    let (_, h1689) = v2;
                    h1689
                }, f365(v0, v3))
            } else {
                if f41({
                    let (_, h1690) = v2;
                    h1690
                }, v0.clone()) {
                    List::cons({
                        let (h1691, _) = v2;
                        h1691
                    }, f365(v0, v3))
                } else {
                    f365(v0, v3)
                }
            }
        }
    }
}

pub fn f366(v0: Adt16, v1: Adt13, v2: Bytes) -> bool {
    if f41(v2.clone(), match v0.clone() {
        Adt16::C0(h1692, _, _) => {
            h1692
        }
    }) {
        true
    } else {
        if f41(v2.clone(), match v0.clone() {
            Adt16::C0(_, h1693, _) => {
                h1693
            }
        }) {
            true
        } else {
            if f41(v2, match v0.clone() {
                Adt16::C0(_, _, h1694) => {
                    h1694
                }
            }) {
                true
            } else {
                if f254(v1.clone(), f8(Str::lit("scheme"))) {
                    f41(f47(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt16::C0(_, h1695, _) => {
                            h1695
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f367(v0: Bytes, v1: u64) -> R<u64> {
    let m1698 = {
        let a1696 = v0;
        let a1697 = v1;
        index_bytes(a1696, a1697)
    };
    match m1698 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1743 = {
                let a1719 = {
                    let a1707 = {
                        let a1701 = {
                            let a1699 = v3;
                            let a1700 = 128u8;
                            fixed_u8::bit_and(a1699, a1700)
                        };
                        let a1702 = 128u8;
                        if equal(a1701, a1702) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1708 = {
                        let a1705 = {
                            let a1703 = v3;
                            let a1704 = 64u8;
                            fixed_u8::bit_and(a1703, a1704)
                        };
                        let a1706 = 64u8;
                        if equal(a1705, a1706) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1707, a1708)?
                };
                let a1720 = {
                    let a1717 = {
                        let a1711 = {
                            let a1709 = v3;
                            let a1710 = 32u8;
                            fixed_u8::bit_and(a1709, a1710)
                        };
                        let a1712 = 32u8;
                        if equal(a1711, a1712) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1718 = {
                        let a1715 = {
                            let a1713 = v3;
                            let a1714 = 16u8;
                            fixed_u8::bit_and(a1713, a1714)
                        };
                        let a1716 = 16u8;
                        if equal(a1715, a1716) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1717, a1718)?
                };
                nat_add(a1719, a1720)?
            };
            let a1744 = {
                let a1741 = {
                    let a1729 = {
                        let a1723 = {
                            let a1721 = v3;
                            let a1722 = 8u8;
                            fixed_u8::bit_and(a1721, a1722)
                        };
                        let a1724 = 8u8;
                        if equal(a1723, a1724) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1730 = {
                        let a1727 = {
                            let a1725 = v3;
                            let a1726 = 4u8;
                            fixed_u8::bit_and(a1725, a1726)
                        };
                        let a1728 = 4u8;
                        if equal(a1727, a1728) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1729, a1730)?
                };
                let a1742 = {
                    let a1739 = {
                        let a1733 = {
                            let a1731 = v3;
                            let a1732 = 2u8;
                            fixed_u8::bit_and(a1731, a1732)
                        };
                        let a1734 = 2u8;
                        if equal(a1733, a1734) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1740 = {
                        let a1737 = {
                            let a1735 = v3;
                            let a1736 = 1u8;
                            fixed_u8::bit_and(a1735, a1736)
                        };
                        let a1738 = 1u8;
                        if equal(a1737, a1738) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1739, a1740)?
                };
                nat_add(a1741, a1742)?
            };
            nat_add(a1743, a1744)
        }
    }
}

pub fn f368(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1748 = {
        let a1745 = v1;
        let a1746 = v2;
        let a1747 = 1u64;
        slice_bytes(a1745, a1746, a1747)
    };
    match m1748 {
        None => {
            v0
        }
        Some(v3) => {
            let a1749 = v0;
            let a1750 = v3;
            append_bytes(a1749, a1750)
        }
    }
}

pub fn f369(p1753: List<Bytes>, p1754: List<Bytes>) -> List<Bytes> {
    let m1752 = (p1753, p1754);
    {
        let mut m1752 = m1752;
        loop {
            let step = {
                let (v0, v1) = m1752;
                let m1751 = v0;
                match m1751.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f370(v0: Bytes, p1826: u64, p1827: u64, p1828: Bytes) -> R<Bytes> {
    let m1825 = (p1826, (p1827, p1828));
    {
        let mut m1825 = m1825;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1825;
                let m1755 = v2;
                if m1755 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1755 - 1;
                    let m1758 = {
                        let a1756 = v0.clone();
                        let a1757 = v1;
                        index_bytes(a1756, a1757)
                    };
                    match m1758 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1803 = {
                                    let a1779 = {
                                        let a1767 = {
                                            let a1761 = {
                                                let a1759 = v6;
                                                let a1760 = 128u8;
                                                fixed_u8::bit_and(a1759, a1760)
                                            };
                                            let a1762 = 128u8;
                                            if equal(a1761, a1762) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1768 = {
                                            let a1765 = {
                                                let a1763 = v6;
                                                let a1764 = 64u8;
                                                fixed_u8::bit_and(a1763, a1764)
                                            };
                                            let a1766 = 64u8;
                                            if equal(a1765, a1766) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1767, a1768)?
                                    };
                                    let a1780 = {
                                        let a1777 = {
                                            let a1771 = {
                                                let a1769 = v6;
                                                let a1770 = 32u8;
                                                fixed_u8::bit_and(a1769, a1770)
                                            };
                                            let a1772 = 32u8;
                                            if equal(a1771, a1772) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1778 = {
                                            let a1775 = {
                                                let a1773 = v6;
                                                let a1774 = 16u8;
                                                fixed_u8::bit_and(a1773, a1774)
                                            };
                                            let a1776 = 16u8;
                                            if equal(a1775, a1776) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1777, a1778)?
                                    };
                                    nat_add(a1779, a1780)?
                                };
                                let a1804 = {
                                    let a1801 = {
                                        let a1789 = {
                                            let a1783 = {
                                                let a1781 = v6;
                                                let a1782 = 8u8;
                                                fixed_u8::bit_and(a1781, a1782)
                                            };
                                            let a1784 = 8u8;
                                            if equal(a1783, a1784) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1790 = {
                                            let a1787 = {
                                                let a1785 = v6;
                                                let a1786 = 4u8;
                                                fixed_u8::bit_and(a1785, a1786)
                                            };
                                            let a1788 = 4u8;
                                            if equal(a1787, a1788) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1789, a1790)?
                                    };
                                    let a1802 = {
                                        let a1799 = {
                                            let a1793 = {
                                                let a1791 = v6;
                                                let a1792 = 2u8;
                                                fixed_u8::bit_and(a1791, a1792)
                                            };
                                            let a1794 = 2u8;
                                            if equal(a1793, a1794) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1800 = {
                                            let a1797 = {
                                                let a1795 = v6;
                                                let a1796 = 1u8;
                                                fixed_u8::bit_and(a1795, a1796)
                                            };
                                            let a1798 = 1u8;
                                            if equal(a1797, a1798) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1799, a1800)?
                                    };
                                    nat_add(a1801, a1802)?
                                };
                                nat_add(a1803, a1804)?
                            };
                            let a1805 = v7;
                            let a1806 = 34u64;
                            if if nat_eq(a1805, a1806) {
                                true
                            } else {
                                let a1807 = v7;
                                let a1808 = 92u64;
                                nat_eq(a1807, a1808)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1809 = v1;
                                    let a1810 = 1u64;
                                    nat_add(a1809, a1810)?
                                }, (v4, {
                                    let a1813 = v3;
                                    let a1814 = {
                                        let a1811 = Bytes::lit(&[0x5c]);
                                        let a1812 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1811, a1812)
                                    };
                                    append_bytes(a1813, a1814)
                                })))
                            } else {
                                let a1815 = v7;
                                let a1816 = 10u64;
                                if nat_eq(a1815, a1816) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1817 = v1;
                                        let a1818 = 1u64;
                                        nat_add(a1817, a1818)?
                                    }, (v4, {
                                        let a1819 = v3;
                                        let a1820 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1819, a1820)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1821 = v1;
                                        let a1822 = 1u64;
                                        nat_add(a1821, a1822)?
                                    }, (v4, {
                                        let a1823 = v3;
                                        let a1824 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1823, a1824)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1825 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: u32) -> Bytes {
    let a1857 = f458({
        let a1829 = f459(v0, 28u32);
        let a1830 = 15u32;
        fixed_u32::bit_and(a1829, a1830)
    });
    let a1858 = {
        let a1855 = f458({
            let a1831 = f459(v0, 24u32);
            let a1832 = 15u32;
            fixed_u32::bit_and(a1831, a1832)
        });
        let a1856 = {
            let a1853 = f458({
                let a1833 = f459(v0, 20u32);
                let a1834 = 15u32;
                fixed_u32::bit_and(a1833, a1834)
            });
            let a1854 = {
                let a1851 = f458({
                    let a1835 = f459(v0, 16u32);
                    let a1836 = 15u32;
                    fixed_u32::bit_and(a1835, a1836)
                });
                let a1852 = {
                    let a1849 = f458({
                        let a1837 = f459(v0, 12u32);
                        let a1838 = 15u32;
                        fixed_u32::bit_and(a1837, a1838)
                    });
                    let a1850 = {
                        let a1847 = f458({
                            let a1839 = f459(v0, 8u32);
                            let a1840 = 15u32;
                            fixed_u32::bit_and(a1839, a1840)
                        });
                        let a1848 = {
                            let a1845 = f458({
                                let a1841 = f459(v0, 4u32);
                                let a1842 = 15u32;
                                fixed_u32::bit_and(a1841, a1842)
                            });
                            let a1846 = f458({
                                let a1843 = v0;
                                let a1844 = 15u32;
                                fixed_u32::bit_and(a1843, a1844)
                            });
                            append_bytes(a1845, a1846)
                        };
                        append_bytes(a1847, a1848)
                    };
                    append_bytes(a1849, a1850)
                };
                append_bytes(a1851, a1852)
            };
            append_bytes(a1853, a1854)
        };
        append_bytes(a1855, a1856)
    };
    append_bytes(a1857, a1858)
}

pub fn f372(v0: Fn4, p1862: u64, p1863: (u64, Adt25)) -> R<(u64, Adt25)> {
    let m1861 = (p1862, p1863);
    {
        let mut m1861 = m1861;
        loop {
            let step = {
                let (v1, v2) = m1861;
                let m1859 = v1;
                if m1859 == 0 {
                    Err::<(u64, (u64, Adt25)), R<(u64, Adt25)>>(Ok(v2))
                } else {
                    let v3 = m1859 - 1;
                    Ok::<(u64, (u64, Adt25)), R<(u64, Adt25)>>((v3, {
                        let c1860: Fn4 = v0.clone();
                        c1860.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1861 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt25)) -> R<(u64, Adt25)> {
    Ok(({
        let a1865 = {
            let (h1864, _) = v3.clone();
            h1864
        };
        let a1866 = 64u64;
        nat_add(a1865, a1866)?
    }, f460({
        let (_, h1867) = v3;
        h1867
    }, f461(v0, v1, v2, {
        let (h1868, _) = v3;
        h1868
    })?)?))
}

pub fn f374() -> Adt25 {
    Adt25::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f375(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1869 = f33(v1, 34u8)?;
    match m1869.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1870 = v3;
            match m1870.uncons() {
                None => {
                    Ok(f462(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f376(v0: List<Bytes>) -> List<Bytes> {
    let m1871 = v0;
    match m1871.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("authority = []"))) {
                f376(v2)
            } else {
                List::cons(v1, f376(v2))
            }
        }
    }
}

pub fn f377(v0: Fn5, p1876: u64, p1877: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1875 = (p1876, p1877);
    {
        let mut m1875 = m1875;
        loop {
            let step = {
                let (v1, v2) = m1875;
                let m1872 = v1;
                if m1872 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1872 - 1;
                    let m1874 = {
                        let c1873: Fn5 = v0.clone();
                        c1873.apply(v2.clone())?
                    };
                    match m1874 {
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
                Ok(next) => { m1875 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f463(v0, v1)
}

pub fn f379(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f464(v0, v1)
}

pub fn f380(v0: u64) -> bool {
    let a1878 = v0;
    let a1879 = 32u64;
    if if nat_eq(a1878, a1879) {
        true
    } else {
        let a1880 = v0;
        let a1881 = 9u64;
        nat_eq(a1880, a1881)
    } {
        true
    } else {
        let a1882 = v0;
        let a1883 = 10u64;
        if nat_eq(a1882, a1883) {
            true
        } else {
            let a1884 = v0;
            let a1885 = 13u64;
            nat_eq(a1884, a1885)
        }
    }
}

pub fn f381(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1886, _) = f331(Fn3::F465(v1.clone(), v0.clone()), {
            let a1888 = {
                let a1887 = v0.clone();
                length_bytes(a1887)
            };
            let a1889 = 1u64;
            nat_add(a1888, a1889)?
        }, v2)?;
        h1886
    };
    if f269(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f382(v0: Bytes, v1: u64) -> R<u64> {
    let (h1890, _) = f331(Fn3::F466(v0.clone()), {
        let a1892 = {
            let a1891 = v0;
            length_bytes(a1891)
        };
        let a1893 = 1u64;
        nat_add(a1892, a1893)?
    }, v1)?;
    Ok(h1890)
}

pub fn f383(v0: Bytes) -> R<bool> {
    let a1895 = {
        let a1894 = v0.clone();
        length_bytes(a1894)
    };
    let a1896 = 5u64;
    if nat_eq(a1895, a1896) {
        let a1897 = f313(v0.clone(), 0u64)?;
        let a1898 = 85u64;
        if if nat_eq(a1897, a1898) {
            true
        } else {
            let a1899 = f313(v0.clone(), 0u64)?;
            let a1900 = 117u64;
            nat_eq(a1899, a1900)
        } {
            let a1901 = f313(v0.clone(), 1u64)?;
            let a1902 = 84u64;
            if if nat_eq(a1901, a1902) {
                true
            } else {
                let a1903 = f313(v0.clone(), 1u64)?;
                let a1904 = 116u64;
                nat_eq(a1903, a1904)
            } {
                let a1905 = f313(v0.clone(), 2u64)?;
                let a1906 = 70u64;
                if if nat_eq(a1905, a1906) {
                    true
                } else {
                    let a1907 = f313(v0.clone(), 2u64)?;
                    let a1908 = 102u64;
                    nat_eq(a1907, a1908)
                } {
                    let a1909 = f313(v0.clone(), 3u64)?;
                    let a1910 = 45u64;
                    if nat_eq(a1909, a1910) {
                        let a1911 = f313(v0, 4u64)?;
                        let a1912 = 56u64;
                        Ok(nat_eq(a1911, a1912))
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

pub fn f384(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1916 = f381(v0.clone(), {
        let a1913 = Str::lit("--");
        utf8_encode(a1913)
    }, {
        let a1914 = v1;
        let a1915 = 4u64;
        nat_add(a1914, a1915)?
    })?;
    match m1916 {
        None => {
            Ok(Err::<u64, Bytes>(f272(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1919 = f276(v0, {
                let a1917 = v2;
                let a1918 = 2u64;
                nat_add(a1917, a1918)?
            })?;
            let a1920 = 62u64;
            if nat_eq(a1919, a1920) {
                Ok(Ok::<u64, Bytes>({
                    let a1921 = v2;
                    let a1922 = 3u64;
                    nat_add(a1921, a1922)?
                }))
            } else {
                Ok(Err::<u64, Bytes>(f272(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f385(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f386(v0.clone(), {
        let a1923 = v1;
        let a1924 = 2u64;
        nat_add(a1923, a1924)?
    })?;
    let a1927 = v2;
    let a1928 = {
        let a1925 = v1;
        let a1926 = 2u64;
        nat_add(a1925, a1926)?
    };
    if nat_eq(a1927, a1928) {
        Ok(Err::<u64, Bytes>(f272(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1931 = v2;
        let a1932 = {
            let a1929 = v1;
            let a1930 = 5u64;
            nat_add(a1929, a1930)?
        };
        if if nat_eq(a1931, a1932) {
            let a1935 = f276(v0.clone(), {
                let a1933 = v1;
                let a1934 = 2u64;
                nat_add(a1933, a1934)?
            })?;
            let a1936 = 120u64;
            if if nat_eq(a1935, a1936) {
                true
            } else {
                let a1939 = f276(v0.clone(), {
                    let a1937 = v1;
                    let a1938 = 2u64;
                    nat_add(a1937, a1938)?
                })?;
                let a1940 = 88u64;
                nat_eq(a1939, a1940)
            } {
                let a1943 = f276(v0.clone(), {
                    let a1941 = v1;
                    let a1942 = 3u64;
                    nat_add(a1941, a1942)?
                })?;
                let a1944 = 109u64;
                if if nat_eq(a1943, a1944) {
                    true
                } else {
                    let a1947 = f276(v0.clone(), {
                        let a1945 = v1;
                        let a1946 = 3u64;
                        nat_add(a1945, a1946)?
                    })?;
                    let a1948 = 77u64;
                    nat_eq(a1947, a1948)
                } {
                    let a1951 = f276(v0.clone(), {
                        let a1949 = v1;
                        let a1950 = 4u64;
                        nat_add(a1949, a1950)?
                    })?;
                    let a1952 = 108u64;
                    if nat_eq(a1951, a1952) {
                        true
                    } else {
                        let a1955 = f276(v0.clone(), {
                            let a1953 = v1;
                            let a1954 = 4u64;
                            nat_add(a1953, a1954)?
                        })?;
                        let a1956 = 76u64;
                        nat_eq(a1955, a1956)
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
            Ok(Err::<u64, Bytes>(f272(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1958 = f381(v0, {
                let a1957 = Str::lit("?>");
                utf8_encode(a1957)
            }, v2)?;
            match m1958 {
                None => {
                    Ok(Err::<u64, Bytes>(f272(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1959 = v3;
                        let a1960 = 2u64;
                        nat_add(a1959, a1960)?
                    }))
                }
            }
        }
    }
}

pub fn f386(v0: Bytes, v1: u64) -> R<u64> {
    let (h1961, _) = f331(Fn3::F467(v0.clone(), v1), {
        let a1963 = {
            let a1962 = v0;
            length_bytes(a1962)
        };
        let a1964 = 1u64;
        nat_add(a1963, a1964)?
    }, v1)?;
    Ok(h1961)
}

pub fn f387(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt19, u64), Bytes>> {
    let m1965 = v2;
    if m1965 == 0 {
        Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1965 - 1;
        let v5: u64 = f382(v0.clone(), v1)?;
        let a1966 = f276(v0.clone(), v5)?;
        let a1967 = 62u64;
        if if nat_eq(a1966, a1967) {
            true
        } else {
            f275(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt19, u64), Bytes>((Adt19::C4, v5)))
        } else {
            let a1968 = v5;
            let a1969 = v1;
            if nat_eq(a1968, a1969) {
                Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f386(v0.clone(), v5)?;
                let a1970 = v6;
                let a1971 = v5;
                if nat_eq(a1970, a1971) {
                    Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f35(v0.clone(), v5, {
                        let a1972 = v6;
                        let a1973 = v5;
                        nat_sub(a1972, a1973)
                    });
                    if f468(v7.clone(), v3.clone()) {
                        Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f382(v0.clone(), v6)?;
                        let a1974 = f276(v0.clone(), v8)?;
                        let a1975 = 61u64;
                        if !nat_eq(a1974, a1975) {
                            Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        } else {
                            let v9: u64 = f382(v0.clone(), {
                                let a1976 = v8;
                                let a1977 = 1u64;
                                nat_add(a1976, a1977)?
                            })?;
                            let v10: u64 = f276(v0.clone(), v9)?;
                            let a1978 = v10;
                            let a1979 = 34u64;
                            if !if nat_eq(a1978, a1979) {
                                true
                            } else {
                                let a1980 = v10;
                                let a1981 = 39u64;
                                nat_eq(a1980, a1981)
                            } {
                                Ok(Err::<(Adt19, u64), Bytes>(f272(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            } else {
                                let m1984 = f388(v0.clone(), {
                                    let a1982 = v9;
                                    let a1983 = 1u64;
                                    nat_add(a1982, a1983)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1984 {
                                    Ok(v12) => {
                                        let m1986 = f387(v0, {
                                            let (_, h1985) = v12;
                                            h1985
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1986 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt19, u64), Bytes>((Adt19::C1(v7, {
                                                    let (h1987, _) = v12;
                                                    h1987
                                                }, std::rc::Rc::new({
                                                    let (h1988, _) = v14;
                                                    h1988
                                                })), {
                                                    let (_, h1989) = v14;
                                                    h1989
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt19, u64), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt19, u64), Bytes>(v11))
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

pub fn f388(v0: Bytes, p2029: u64, p2030: u64, p2031: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m2028 = (p2029, (p2030, p2031));
    {
        let mut m2028 = m2028;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2028;
                let m1990 = v2;
                if m1990 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f272(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1990 - 1;
                    let v7: u64 = f276(v0.clone(), v1)?;
                    if if v4 {
                        let a1991 = v7;
                        let a1992 = v5;
                        nat_eq(a1991, a1992)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1993 = v1;
                            let a1994 = 1u64;
                            nat_add(a1993, a1994)?
                        }))))
                    } else {
                        let a1995 = v7;
                        let a1996 = 256u64;
                        if nat_eq(a1995, a1996) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f272(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1997 = v7;
                            let a1998 = 60u64;
                            if nat_eq(a1997, a1998) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f272(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1999 = v7;
                                let a2000 = 38u64;
                                if nat_eq(a1999, a2000) {
                                    let m2001 = f469(v0.clone(), v1)?;
                                    match m2001 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h2002) = v9;
                                                h2002
                                            }, (v6, {
                                                let a2004 = v3;
                                                let a2005 = {
                                                    let (h2003, _) = v9;
                                                    h2003
                                                };
                                                append_bytes(a2004, a2005)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a2006 = v7;
                                        let a2007 = 9u64;
                                        if if nat_eq(a2006, a2007) {
                                            true
                                        } else {
                                            let a2008 = v7;
                                            let a2009 = 10u64;
                                            nat_eq(a2008, a2009)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a2010 = v1;
                                                let a2011 = 1u64;
                                                nat_add(a2010, a2011)?
                                            }, (v6, {
                                                let a2012 = v3;
                                                let a2013 = Bytes::lit(&[0x20]);
                                                append_bytes(a2012, a2013)
                                            })))
                                        } else {
                                            let v10: u64 = f470(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a2016 = v3;
                                                let a2017 = f35(v0.clone(), v1, {
                                                    let a2014 = v10;
                                                    let a2015 = v1;
                                                    nat_sub(a2014, a2015)
                                                });
                                                append_bytes(a2016, a2017)
                                            })))
                                        }
                                    } else {
                                        let a2018 = v7;
                                        let a2019 = 93u64;
                                        if nat_eq(a2018, a2019) {
                                            if f275(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f272(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a2020 = v1;
                                                    let a2021 = 1u64;
                                                    nat_add(a2020, a2021)?
                                                }, (v6, {
                                                    let a2022 = v3;
                                                    let a2023 = Bytes::lit(&[0x5d]);
                                                    append_bytes(a2022, a2023)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f470(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a2026 = v3;
                                                let a2027 = f35(v0.clone(), v1, {
                                                    let a2024 = v11;
                                                    let a2025 = v1;
                                                    nat_sub(a2024, a2025)
                                                });
                                                append_bytes(a2026, a2027)
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
                Ok(next) => { m2028 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f389(v0: Bytes, v1: u64) -> Bytes {
    let m2035 = {
        let a2032 = v0;
        let a2033 = v1;
        let a2034 = 1u64;
        slice_bytes(a2032, a2033, a2034)
    };
    match m2035 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f390(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f471(v1, v0, v2)
}

pub fn f391() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f392() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f393(v0: Bytes) -> R<Bytes> {
    let m2036 = f381(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m2036 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f35(v0, 0u64, v1))
        }
    }
}

pub fn f394(p2041: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m2040 = p2041;
    {
        let mut m2040 = m2040;
        loop {
            let step = {
                let v0 = m2040;
                let m2037 = v0;
                match m2037.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h2038, _) = v2;
                            h2038
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h2039) = v2;
                                h2039
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2040 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: Bytes) -> R<Bytes> {
    let m2042 = f381(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m2042 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f35(v0.clone(), {
                let a2043 = v1;
                let a2044 = 1u64;
                nat_add(a2043, a2044)?
            }, {
                let a2048 = {
                    let a2045 = v0;
                    length_bytes(a2045)
                };
                let a2049 = {
                    let a2046 = v1;
                    let a2047 = 1u64;
                    nat_add(a2046, a2047)?
                };
                nat_sub(a2048, a2049)
            }))
        }
    }
}

pub fn f396() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f397(v0: Bytes, v1: Adt19, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v4: Bytes = f300(v0.clone())?;
    let m2050 = f472(v1, v2)?;
    match m2050 {
        Ok(v6) => {
            let m2051 = v6;
            match m2051 {
                None => {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = {
                        let (h2052, _) = v7.clone();
                        h2052
                    };
                    let v9: Adt19 = {
                        let (h2053, _) = {
                            let (_, h2054) = v7.clone();
                            h2054
                        };
                        h2053
                    };
                    let v10: Adt19 = {
                        let (h2055, _) = {
                            let (_, h2056) = {
                                let (_, h2057) = v7.clone();
                                h2057
                            };
                            h2056
                        };
                        h2055
                    };
                    let v11: List<(Bytes, Bytes)> = {
                        let (_, h2058) = {
                            let (_, h2059) = {
                                let (_, h2060) = v7;
                                h2060
                            };
                            h2059
                        };
                        h2058
                    };
                    if f28(v8.clone(), f8(Str::lit("union"))) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f28(v8, f8(Str::lit("list"))) {
                            let m2061 = f283(v9, f8(Str::lit("itemType")));
                            match m2061 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m2062 = f474(v11, v3, v12)?;
                                    match m2062 {
                                        Ok(v14) => {
                                            if f90({
                                                let (h2063, _) = v14.clone();
                                                h2063
                                            }, f8(Str::lit("(List "))) {
                                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f475(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons({
                                                    let (h2064, _) = v14;
                                                    h2064
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), {
                                                    let (_, h2065) = v14;
                                                    h2065
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m2066 = f283(v9, f8(Str::lit("base")));
                            match m2066 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m2067 = f474(v11.clone(), v3, v15)?;
                                    match m2067 {
                                        Ok(v17) => {
                                            let m2068 = f476(v10, v11)?;
                                            match m2068 {
                                                Ok(v19) => {
                                                    let v20: List<Adt20> = f477(v19, List::<Bytes>::nil())?;
                                                    let m2069 = v20.clone();
                                                    match m2069.uncons() {
                                                        None => {
                                                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f475(v4, {
                                                                let (h2070, _) = v17;
                                                                h2070
                                                            }), {
                                                                let (_, h2071) = v17;
                                                                h2071
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m2072 = f478(f479(f196(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m2072 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f480(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(v16, v0)))
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
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f473(v5, v0)))
        }
    }
}

pub fn f398(p2075: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m2074 = p2075;
    {
        let mut m2074 = m2074;
        loop {
            let step = {
                let v0 = m2074;
                let m2073 = v0;
                match m2073.uncons() {
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
                Ok(next) => { m2074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m2076 = v0;
    match m2076.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m2079 = f398({
                let (h2077, _) = {
                    let (_, h2078) = v2.clone();
                    h2078
                };
                h2077
            }, v1.clone());
            match m2079 {
                None => {
                    Some((({
                        let (h2080, _) = v2;
                        h2080
                    }, {
                        let (_, h2081) = {
                            let (_, h2082) = v2;
                            h2082
                        };
                        h2081
                    }), v3))
                }
                Some(_) => {
                    let m2083 = f399(v3, v1);
                    match m2083 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2084, _) = v5;
                                h2084
                            }, List::cons(v2, {
                                let (_, h2085) = v5;
                                h2085
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f400(v0: Bytes) -> R<bool> {
    let m2087 = {
        let a2086 = v0.clone();
        length_bytes(a2086)
    };
    if m2087 == 0 {
        Ok(true)
    } else {
        let v1 = m2087 - 1;
        let m2090 = {
            let a2088 = v0;
            let a2089 = v1;
            index_bytes(a2088, a2089)
        };
        match m2090 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f481({
                    let v3: u8 = v2;
                    let a2135 = {
                        let a2111 = {
                            let a2099 = {
                                let a2093 = {
                                    let a2091 = v3;
                                    let a2092 = 128u8;
                                    fixed_u8::bit_and(a2091, a2092)
                                };
                                let a2094 = 128u8;
                                if equal(a2093, a2094) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a2100 = {
                                let a2097 = {
                                    let a2095 = v3;
                                    let a2096 = 64u8;
                                    fixed_u8::bit_and(a2095, a2096)
                                };
                                let a2098 = 64u8;
                                if equal(a2097, a2098) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2099, a2100)?
                        };
                        let a2112 = {
                            let a2109 = {
                                let a2103 = {
                                    let a2101 = v3;
                                    let a2102 = 32u8;
                                    fixed_u8::bit_and(a2101, a2102)
                                };
                                let a2104 = 32u8;
                                if equal(a2103, a2104) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a2110 = {
                                let a2107 = {
                                    let a2105 = v3;
                                    let a2106 = 16u8;
                                    fixed_u8::bit_and(a2105, a2106)
                                };
                                let a2108 = 16u8;
                                if equal(a2107, a2108) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2109, a2110)?
                        };
                        nat_add(a2111, a2112)?
                    };
                    let a2136 = {
                        let a2133 = {
                            let a2121 = {
                                let a2115 = {
                                    let a2113 = v3;
                                    let a2114 = 8u8;
                                    fixed_u8::bit_and(a2113, a2114)
                                };
                                let a2116 = 8u8;
                                if equal(a2115, a2116) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a2122 = {
                                let a2119 = {
                                    let a2117 = v3;
                                    let a2118 = 4u8;
                                    fixed_u8::bit_and(a2117, a2118)
                                };
                                let a2120 = 4u8;
                                if equal(a2119, a2120) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2121, a2122)?
                        };
                        let a2134 = {
                            let a2131 = {
                                let a2125 = {
                                    let a2123 = v3;
                                    let a2124 = 2u8;
                                    fixed_u8::bit_and(a2123, a2124)
                                };
                                let a2126 = 2u8;
                                if equal(a2125, a2126) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2132 = {
                                let a2129 = {
                                    let a2127 = v3;
                                    let a2128 = 1u8;
                                    fixed_u8::bit_and(a2127, a2128)
                                };
                                let a2130 = 1u8;
                                if equal(a2129, a2130) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2131, a2132)?
                        };
                        nat_add(a2133, a2134)?
                    };
                    nat_add(a2135, a2136)?
                }))
            }
        }
    }
}

pub fn f401() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f402(v0: Adt13, v1: Bytes, v2: Bytes) -> bool {
    let m2137 = f89(v0, v1);
    match m2137 {
        None => {
            false
        }
        Some(v3) => {
            f254(v3, v2)
        }
    }
}

pub fn f403() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f404(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt20, u64)>> {
    let m2138 = v2;
    if m2138 == 0 {
        Ok(None::<(Adt20, u64)>)
    } else {
        let v4 = m2138 - 1;
        let v5: u64 = f313(v0.clone(), v1)?;
        let a2139 = v3;
        let a2140 = 1u64;
        if nat_eq(a2139, a2140) {
            let m2141 = f404(v0.clone(), v1, v4, 0u64)?;
            match m2141 {
                None => {
                    Ok(None::<(Adt20, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f313(v0.clone(), {
                        let (_, h2142) = v6.clone();
                        h2142
                    })?;
                    let a2143 = v7;
                    let a2144 = 44u64;
                    if nat_eq(a2143, a2144) {
                        let m2148 = f404(v0, {
                            let a2146 = {
                                let (_, h2145) = v6;
                                h2145
                            };
                            let a2147 = 1u64;
                            nat_add(a2146, a2147)?
                        }, v4, 1u64)?;
                        match m2148 {
                            None => {
                                Ok(None::<(Adt20, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt20::C7(std::rc::Rc::new({
                                    let (h2149, _) = v6;
                                    h2149
                                }), std::rc::Rc::new({
                                    let (h2150, _) = v8;
                                    h2150
                                })), {
                                    let (_, h2151) = v8;
                                    h2151
                                })))
                            }
                        }
                    } else {
                        let a2152 = v7;
                        let a2153 = 93u64;
                        if nat_eq(a2152, a2153) {
                            Ok(Some((Adt20::C7(std::rc::Rc::new({
                                let (h2154, _) = v6;
                                h2154
                            }), std::rc::Rc::new(Adt20::C6)), {
                                let a2156 = {
                                    let (_, h2155) = v6;
                                    h2155
                                };
                                let a2157 = 1u64;
                                nat_add(a2156, a2157)?
                            })))
                        } else {
                            Ok(None::<(Adt20, u64)>)
                        }
                    }
                }
            }
        } else {
            let a2158 = v3;
            let a2159 = 2u64;
            if nat_eq(a2158, a2159) {
                let m2162 = f314(v0.clone(), {
                    let a2160 = v1;
                    let a2161 = 1u64;
                    nat_add(a2160, a2161)?
                })?;
                match m2162 {
                    None => {
                        Ok(None::<(Adt20, u64)>)
                    }
                    Some(v9) => {
                        let a2163 = v5;
                        let a2164 = 34u64;
                        if if nat_eq(a2163, a2164) {
                            let a2166 = f313(v0.clone(), {
                                let (h2165, _) = v9.clone();
                                h2165
                            })?;
                            let a2167 = 58u64;
                            nat_eq(a2166, a2167)
                        } else {
                            false
                        } {
                            let m2171 = f404(v0.clone(), {
                                let a2169 = {
                                    let (h2168, _) = v9;
                                    h2168
                                };
                                let a2170 = 1u64;
                                nat_add(a2169, a2170)?
                            }, v4, 0u64)?;
                            match m2171 {
                                None => {
                                    Ok(None::<(Adt20, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f313(v0.clone(), {
                                        let (_, h2172) = v10.clone();
                                        h2172
                                    })?;
                                    let a2173 = v11;
                                    let a2174 = 44u64;
                                    if nat_eq(a2173, a2174) {
                                        let m2178 = f404(v0, {
                                            let a2176 = {
                                                let (_, h2175) = v10;
                                                h2175
                                            };
                                            let a2177 = 1u64;
                                            nat_add(a2176, a2177)?
                                        }, v4, 2u64)?;
                                        match m2178 {
                                            None => {
                                                Ok(None::<(Adt20, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt20::C8({
                                                    let (_, h2179) = v9;
                                                    h2179
                                                }, std::rc::Rc::new({
                                                    let (h2180, _) = v10;
                                                    h2180
                                                }), std::rc::Rc::new({
                                                    let (h2181, _) = v12;
                                                    h2181
                                                })), {
                                                    let (_, h2182) = v12;
                                                    h2182
                                                })))
                                            }
                                        }
                                    } else {
                                        let a2183 = v11;
                                        let a2184 = 125u64;
                                        if nat_eq(a2183, a2184) {
                                            Ok(Some((Adt20::C8({
                                                let (_, h2185) = v9;
                                                h2185
                                            }, std::rc::Rc::new({
                                                let (h2186, _) = v10;
                                                h2186
                                            }), std::rc::Rc::new(Adt20::C6)), {
                                                let a2188 = {
                                                    let (_, h2187) = v10;
                                                    h2187
                                                };
                                                let a2189 = 1u64;
                                                nat_add(a2188, a2189)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt20, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt20, u64)>)
                        }
                    }
                }
            } else {
                let a2190 = v5;
                let a2191 = 34u64;
                if nat_eq(a2190, a2191) {
                    let m2194 = f314(v0, {
                        let a2192 = v1;
                        let a2193 = 1u64;
                        nat_add(a2192, a2193)?
                    })?;
                    match m2194 {
                        None => {
                            Ok(None::<(Adt20, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt20::C3({
                                let (_, h2195) = v13;
                                h2195
                            }), {
                                let (h2196, _) = v13;
                                h2196
                            })))
                        }
                    }
                } else {
                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt20::C0(true), {
                            let a2197 = v1;
                            let a2198 = 4u64;
                            nat_add(a2197, a2198)?
                        })))
                    } else {
                        if f28(f35(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt20::C0(false), {
                                let a2199 = v1;
                                let a2200 = 5u64;
                                nat_add(a2199, a2200)?
                            })))
                        } else {
                            let a2201 = v5;
                            let a2202 = 91u64;
                            if nat_eq(a2201, a2202) {
                                let a2205 = f313(v0.clone(), {
                                    let a2203 = v1;
                                    let a2204 = 1u64;
                                    nat_add(a2203, a2204)?
                                })?;
                                let a2206 = 93u64;
                                if nat_eq(a2205, a2206) {
                                    Ok(Some((Adt20::C4(std::rc::Rc::new(Adt20::C6)), {
                                        let a2207 = v1;
                                        let a2208 = 2u64;
                                        nat_add(a2207, a2208)?
                                    })))
                                } else {
                                    let m2211 = f404(v0, {
                                        let a2209 = v1;
                                        let a2210 = 1u64;
                                        nat_add(a2209, a2210)?
                                    }, v4, 1u64)?;
                                    match m2211 {
                                        None => {
                                            Ok(None::<(Adt20, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt20::C4(std::rc::Rc::new({
                                                let (h2212, _) = v14;
                                                h2212
                                            })), {
                                                let (_, h2213) = v14;
                                                h2213
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a2214 = v5;
                                let a2215 = 123u64;
                                if nat_eq(a2214, a2215) {
                                    let a2218 = f313(v0.clone(), {
                                        let a2216 = v1;
                                        let a2217 = 1u64;
                                        nat_add(a2216, a2217)?
                                    })?;
                                    let a2219 = 125u64;
                                    if nat_eq(a2218, a2219) {
                                        Ok(Some((Adt20::C5(std::rc::Rc::new(Adt20::C6)), {
                                            let a2220 = v1;
                                            let a2221 = 2u64;
                                            nat_add(a2220, a2221)?
                                        })))
                                    } else {
                                        let m2224 = f404(v0, {
                                            let a2222 = v1;
                                            let a2223 = 1u64;
                                            nat_add(a2222, a2223)?
                                        }, v4, 2u64)?;
                                        match m2224 {
                                            None => {
                                                Ok(None::<(Adt20, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt20::C5(std::rc::Rc::new({
                                                    let (h2225, _) = v15;
                                                    h2225
                                                })), {
                                                    let (_, h2226) = v15;
                                                    h2226
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m2227 = f482(v0, v1)?;
                                    match m2227 {
                                        None => {
                                            Ok(None::<(Adt20, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h2228) = v16;
                                                h2228
                                            }, {
                                                let (h2229, _) = v16;
                                                h2229
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

pub fn f405(v0: Adt20, v1: Bytes) -> bool {
    let a2231 = {
        let a2230 = f195(v0, v1);
        length_bytes(a2230)
    };
    let a2232 = 0u64;
    !nat_eq(a2231, a2232)
}

pub fn f406(v0: Adt20, v1: Bytes) -> bool {
    let m2233 = f192(v0, v1);
    match m2233 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f407(v0: Bytes, p2300: u64, p2301: u64, p2302: bool, p2303: Bytes) -> R<Bytes> {
    let m2299 = (p2300, (p2301, (p2302, p2303)));
    {
        let mut m2299 = m2299;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2299;
                let m2234 = v2;
                if m2234 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m2234 - 1;
                    let m2237 = {
                        let a2235 = v0.clone();
                        let a2236 = v1;
                        index_bytes(a2235, a2236)
                    };
                    match m2237 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a2282 = {
                                    let a2258 = {
                                        let a2246 = {
                                            let a2240 = {
                                                let a2238 = v7;
                                                let a2239 = 128u8;
                                                fixed_u8::bit_and(a2238, a2239)
                                            };
                                            let a2241 = 128u8;
                                            if equal(a2240, a2241) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2247 = {
                                            let a2244 = {
                                                let a2242 = v7;
                                                let a2243 = 64u8;
                                                fixed_u8::bit_and(a2242, a2243)
                                            };
                                            let a2245 = 64u8;
                                            if equal(a2244, a2245) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2246, a2247)?
                                    };
                                    let a2259 = {
                                        let a2256 = {
                                            let a2250 = {
                                                let a2248 = v7;
                                                let a2249 = 32u8;
                                                fixed_u8::bit_and(a2248, a2249)
                                            };
                                            let a2251 = 32u8;
                                            if equal(a2250, a2251) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2257 = {
                                            let a2254 = {
                                                let a2252 = v7;
                                                let a2253 = 16u8;
                                                fixed_u8::bit_and(a2252, a2253)
                                            };
                                            let a2255 = 16u8;
                                            if equal(a2254, a2255) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2256, a2257)?
                                    };
                                    nat_add(a2258, a2259)?
                                };
                                let a2283 = {
                                    let a2280 = {
                                        let a2268 = {
                                            let a2262 = {
                                                let a2260 = v7;
                                                let a2261 = 8u8;
                                                fixed_u8::bit_and(a2260, a2261)
                                            };
                                            let a2263 = 8u8;
                                            if equal(a2262, a2263) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2269 = {
                                            let a2266 = {
                                                let a2264 = v7;
                                                let a2265 = 4u8;
                                                fixed_u8::bit_and(a2264, a2265)
                                            };
                                            let a2267 = 4u8;
                                            if equal(a2266, a2267) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2268, a2269)?
                                    };
                                    let a2281 = {
                                        let a2278 = {
                                            let a2272 = {
                                                let a2270 = v7;
                                                let a2271 = 2u8;
                                                fixed_u8::bit_and(a2270, a2271)
                                            };
                                            let a2273 = 2u8;
                                            if equal(a2272, a2273) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2279 = {
                                            let a2276 = {
                                                let a2274 = v7;
                                                let a2275 = 1u8;
                                                fixed_u8::bit_and(a2274, a2275)
                                            };
                                            let a2277 = 1u8;
                                            if equal(a2276, a2277) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2278, a2279)?
                                    };
                                    nat_add(a2280, a2281)?
                                };
                                nat_add(a2282, a2283)?
                            };
                            if f483(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2284 = v1;
                                    let a2285 = 1u64;
                                    nat_add(a2284, a2285)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f484(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2286 = v1;
                                        let a2287 = 1u64;
                                        nat_add(a2286, a2287)?
                                    }, (v5, (false, {
                                        let a2291 = v4;
                                        let a2292 = f35({
                                            let a2288 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2288)
                                        }, {
                                            let a2289 = v8;
                                            let a2290 = 97u64;
                                            nat_sub(a2289, a2290)
                                        }, 1u64);
                                        append_bytes(a2291, a2292)
                                    }))))
                                } else {
                                    if if f484(v8) {
                                        true
                                    } else {
                                        if f485(v8) {
                                            true
                                        } else {
                                            f486(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2293 = v1;
                                            let a2294 = 1u64;
                                            nat_add(a2293, a2294)?
                                        }, (v5, (false, {
                                            let a2295 = v4;
                                            let a2296 = f35(v0.clone(), v1, 1u64);
                                            append_bytes(a2295, a2296)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2297 = v1;
                                            let a2298 = 1u64;
                                            nat_add(a2297, a2298)?
                                        }, (v5, (v3, v4))))
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

pub fn f408(v0: Adt20, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt20)> = f487(f488(f192(v0.clone(), f8(Str::lit("$defs")))), f488(f192(v0.clone(), f8(Str::lit("definitions")))));
    let m2304 = f489(4096u64, v3.clone(), List::<Bytes>::nil(), f490(v3)?)?;
    match m2304 {
        Ok(v5) => {
            let m2305 = f491(4096u64, f492(f493(v5)?, List::cons(Adt26::C0(v2, v0), List::<Adt26>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m2305 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f265(f238(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f409(v0: List<Adt13>, v1: Bytes, v2: Bytes, v3: Adt13) -> List<Adt13> {
    let m2306 = v0;
    match m2306.uncons() {
        None => {
            List::cons(v3, List::<Adt13>::nil())
        }
        Some((v4, v5)) => {
            if f28(f47(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f494(v5, v1, v2))
            } else {
                List::cons(v4, f409(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f410(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f265(f495(f92(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f411(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f167(v1)?;
    let v4: u64 = {
        let a2307 = v3.clone();
        length_bytes(a2307)
    };
    let v5: Bytes = {
        let a2308 = v2;
        let a2309 = v4;
        if nat_lt(a2308, a2309) {
            f7(List::cons(f35(v3.clone(), 0u64, {
                let a2310 = v4;
                let a2311 = v2;
                nat_sub(a2310, a2311)
            }), List::cons(f8(Str::lit(".")), List::cons(f35(v3, {
                let a2312 = v4;
                let a2313 = v2;
                nat_sub(a2312, a2313)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f496({
                let a2314 = v2;
                let a2315 = v4;
                nat_sub(a2314, a2315)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a2316 = f8(Str::lit("-"));
        let a2317 = v5;
        Ok(append_bytes(a2316, a2317))
    } else {
        Ok(v5)
    }
}

pub fn f412(v0: Adt20) -> R<List<Bytes>> {
    let m2318 = v0;
    match m2318 {
        Adt20::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt20::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt20::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt20::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt20::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt20::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt20::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt20::C7(r2319, r2320) => {
            let v10 = (*r2319).clone();
            let v11 = (*r2320).clone();
            Ok(List::cons(f303(v10)?, f412(v11)?))
        }
        Adt20::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f413(v0: Adt20) -> R<List<(Bytes, Bytes)>> {
    let m2321 = v0;
    match m2321 {
        Adt20::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt20::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt20::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt20::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt20::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt20::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt20::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt20::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt20::C8(v12, r2322, r2323) => {
            let v13 = (*r2322).clone();
            let v14 = (*r2323).clone();
            Ok(List::cons((v12, f303(v13)?), f413(v14)?))
        }
    }
}

pub fn f414(v0: Option<Adt13>, v1: Bytes) -> Bytes {
    let m2324 = v0;
    match m2324 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f47(v2, v1)
        }
    }
}

pub fn f415(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f265(f497(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f416(v0: Fn6, p2329: u64, p2330: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2328 = (p2329, p2330);
    {
        let mut m2328 = m2328;
        loop {
            let step = {
                let (v1, v2) = m2328;
                let m2325 = v1;
                if m2325 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2325 - 1;
                    let m2327 = {
                        let c2326: Fn6 = v0.clone();
                        c2326.apply(v2.clone())?
                    };
                    match m2327 {
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
                Ok(next) => { m2328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f498(v0, v1)
}

pub fn f418(v0: Bytes, v1: u64) -> R<u64> {
    let (h2331, _) = f331(Fn3::F499(v0.clone()), {
        let a2333 = {
            let a2332 = v0;
            length_bytes(a2332)
        };
        let a2334 = 1u64;
        nat_add(a2333, a2334)?
    }, v1)?;
    Ok(h2331)
}

pub fn f419(v0: Bytes) -> R<u64> {
    let (h2335, _) = f331(Fn3::F500(v0.clone()), {
        let a2337 = {
            let a2336 = v0;
            length_bytes(a2336)
        };
        let a2338 = 1u64;
        nat_add(a2337, a2338)?
    }, 0u64)?;
    Ok(h2335)
}

pub fn f420(v0: Bytes) -> R<Option<u64>> {
    if f501(v0.clone())? {
        Ok(Some({
            let (_, h2339) = {
                let (h2340, _) = f502(Fn13::F503(v0.clone()), {
                    let a2341 = v0;
                    length_bytes(a2341)
                }, (0u64, 0u64))?;
                h2340
            };
            h2339
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f421(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f35(v1.clone(), 0u64, {
        let a2343 = {
            let a2342 = v1.clone();
            length_bytes(a2342)
        };
        let a2344 = f504(v1)?;
        nat_sub(a2343, a2344)
    });
    let v5: Bytes = {
        let a2345 = v0;
        let a2346 = v4.clone();
        append_bytes(a2345, a2346)
    };
    let v6: u64 = f419(v5.clone())?;
    let v7: Bytes = f35(v5.clone(), v6, {
        let a2348 = {
            let a2347 = v5;
            length_bytes(a2347)
        };
        let a2349 = v6;
        nat_sub(a2348, a2349)
    });
    let v8: u64 = {
        let a2350 = v4;
        length_bytes(a2350)
    };
    let v9: u64 = {
        let a2351 = v7.clone();
        length_bytes(a2351)
    };
    let a2352 = v9;
    let a2353 = 0u64;
    if nat_eq(a2352, a2353) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a2354 = v3;
            let a2355 = v8;
            nat_lt(a2354, a2355)
        } {
            let v10: u64 = if v2 {
                let a2356 = v3;
                let a2357 = v8;
                nat_add(a2356, a2357)?
            } else {
                let a2358 = v8;
                let a2359 = v3;
                nat_sub(a2358, a2359)
            };
            let a2360 = v10;
            let a2361 = f504(v7.clone())?;
            if nat_le(a2360, a2361) {
                Ok(Some(f35(v7, 0u64, {
                    let a2362 = v9;
                    let a2363 = v10;
                    nat_sub(a2362, a2363)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a2364 = v3;
                let a2365 = v8;
                nat_sub(a2364, a2365)
            };
            let a2368 = {
                let a2366 = v9;
                let a2367 = v11;
                nat_add(a2366, a2367)?
            };
            let a2369 = 19u64;
            if nat_le(a2368, a2369) {
                Ok(Some({
                    let a2370 = v7;
                    let a2371 = f496(v11);
                    append_bytes(a2370, a2371)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f422(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2372 = 0u64;
        let a2373 = v1;
        if nat_lt(a2372, a2373) {
            let a2374 = v1;
            let a2375 = 9223372036854775808u64;
            nat_le(a2374, a2375)
        } else {
            false
        }
    } else {
        let a2376 = v1;
        let a2377 = 9223372036854775807u64;
        nat_le(a2376, a2377)
    }
}

pub fn f423(v0: Bytes) -> Bytes {
    if f179(v0.clone(), Bytes::lit(&[0x0d])) {
        f35(v0.clone(), 0u64, {
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

pub fn f424(v0: Adt13, v1: Bytes) -> Adt13 {
    let m2381 = v0;
    match m2381 {
        Adt13::C0(v3) => {
            Adt13::C0(v3)
        }
        Adt13::C1(v4, v5) => {
            Adt13::C1(v4, v5)
        }
        Adt13::C2(v2) => {
            Adt13::C2(f505(f49(), v2, v1))
        }
        Adt13::C3(r2382) => {
            let v6 = (*r2382).clone();
            Adt13::C3(std::rc::Rc::new(v6))
        }
        Adt13::C4(r2383) => {
            let v7 = (*r2383).clone();
            Adt13::C4(std::rc::Rc::new(v7))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r2384, r2385) => {
            let v8 = (*r2384).clone();
            let v9 = (*r2385).clone();
            Adt13::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt13::C7(v10, r2386, r2387) => {
            let v11 = (*r2386).clone();
            let v12 = (*r2387).clone();
            Adt13::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f425(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f506(v0, v1)
}

pub fn f426(v0: Fn7, p2391: List<(Bytes, Bytes)>, p2392: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2390 = (p2391, p2392);
    {
        let mut m2390 = m2390;
        loop {
            let step = {
                let (v1, v2) = m2390;
                let m2388 = v2;
                match m2388.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2389: Fn7 = v0.clone();
                            c2389.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f427(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f507(v0, v1)
}

pub fn f428(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f508(v0)
}

pub fn f429(v0: Fn8, p2396: List<Bytes>, p2397: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2395 = (p2396, p2397);
    {
        let mut m2395 = m2395;
        loop {
            let step = {
                let (v1, v2) = m2395;
                let m2393 = v2;
                match m2393.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2394: Fn8 = v0.clone();
                            c2394.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f430(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f509(v1)?, v0))
}

pub fn f431(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f319(v0)?, List::cons(f8(Str::lit(":")), List::cons(f319(v1)?, List::<Bytes>::nil())))))
}

pub fn f432(v0: Adt12) -> Bytes {
    let m2398 = v0;
    match m2398 {
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

pub fn f433(v0: Adt12) -> Bytes {
    let m2399 = v0;
    match m2399 {
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

pub fn f434(v0: Adt12) -> Bytes {
    let m2400 = v0;
    match m2400 {
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

pub fn f435(v0: Bytes, v1: u64) -> Option<u64> {
    let a2401 = 0u64;
    let a2402 = v1;
    if nat_lt(a2401, a2402) {
        if f510(v0, {
            let a2403 = v1;
            let a2404 = 1u64;
            nat_sub(a2403, a2404)
        }) {
            Some({
                let a2405 = v1;
                let a2406 = 1u64;
                nat_sub(a2405, a2406)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f436(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a2407 = v2;
    let a2408 = v0;
    if nat_lt(a2407, a2408) {
        if f510(v1, v2) {
            Ok(Some({
                let a2409 = v2;
                let a2410 = 1u64;
                nat_add(a2409, a2410)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f437(v0: Bytes) -> R<bool> {
    let m2411 = f222(v0.clone(), f8(Str::lit("&&")))?;
    if match m2411 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2412 = f222(v0, f8(Str::lit("||")))?;
        match m2412 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f438(v0: Adt17, v1: Option<u64>) -> bool {
    let m2413 = v1;
    match m2413 {
        None => {
            true
        }
        Some(v2) => {
            f340(v2, match v0 {
                Adt17::C0(_, h2414, _, _) => {
                    h2414
                }
            })
        }
    }
}

pub fn f439(v0: Adt14) -> Adt22 {
    Adt22::C1({
        let a2416 = match v0 {
            Adt14::C0(_, _, h2415, _, _) => {
                h2415
            }
        };
        length_list(a2416)
    })
}

pub fn f440(v0: Bytes) -> Option<Adt23> {
    if f28(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt23::C0)
    } else {
        if f28(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt23::C1)
        } else {
            if f28(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt23::C2)
            } else {
                None::<Adt23>
            }
        }
    }
}

pub fn f441(v0: Bytes, p2421: List<Bytes>, p2422: u64) -> R<Option<u64>> {
    let m2420 = (p2421, p2422);
    {
        let mut m2420 = m2420;
        loop {
            let step = {
                let (v1, v2) = m2420;
                let m2417 = v1;
                match m2417.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f28(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2418 = v2;
                                let a2419 = 1u64;
                                nat_add(a2418, a2419)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2420 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: Option<Adt13>) -> Option<u64> {
    let m2423 = v0;
    match m2423 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2424 = v1;
            match m2424 {
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

pub fn f443(v0: Fn9, p2428: List<Bytes>, p2429: List<(Bytes, ())>) -> List<Bytes> {
    let m2427 = (p2428, p2429);
    {
        let mut m2427 = m2427;
        loop {
            let step = {
                let (v1, v2) = m2427;
                let m2425 = v2;
                match m2425.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c2426: Fn9 = v0.clone();
                            c2426.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f444(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h2430, _) = v1;
        h2430
    }, v0)
}

pub fn f445(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f511(Fn14::F512, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f446(v0: Fn10, p2434: List<List<(Bytes, ())>>, p2435: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m2433 = (p2434, p2435);
    {
        let mut m2433 = m2433;
        loop {
            let step = {
                let (v1, v2) = m2433;
                let m2431 = v2;
                match m2431.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c2432: Fn10 = v0.clone();
                            c2432.apply(v1, v3)
                        }, v4))
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

pub fn f447(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f448(v0: Fn11, p2440: u64, p2441: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m2439 = (p2440, p2441);
    {
        let mut m2439 = m2439;
        loop {
            let step = {
                let (v1, v2) = m2439;
                let m2436 = v1;
                if m2436 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2436 - 1;
                    let m2438 = {
                        let c2437: Fn11 = v0.clone();
                        c2437.apply(v2.clone())?
                    };
                    match m2438 {
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
                Ok(next) => { m2439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f513(v0)
}

pub fn f450(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f514(Fn15::F515, List::<(Bytes, ())>::nil(), v0)
}

pub fn f451(v0: Fn12, p2445: List<(Bytes, ())>, p2446: List<Bytes>) -> List<(Bytes, ())> {
    let m2444 = (p2445, p2446);
    {
        let mut m2444 = m2444;
        loop {
            let step = {
                let (v1, v2) = m2444;
                let m2442 = v2;
                match m2442.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c2443: Fn12 = v0.clone();
                            c2443.apply(v1, v3)
                        }, v4))
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

pub fn f452(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f453(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f92(f53(v0.clone(), v1.clone()))?;
    let m2447 = f198(v2.clone())?;
    match m2447 {
        None => {
            f517(v0, v1, v2)
        }
        Some(v3) => {
            f516(v0, v1, v3)
        }
    }
}

pub fn f454(v0: List<Bytes>) -> List<Bytes> {
    let m2448 = v0;
    match m2448.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a2450 = {
                    let a2449 = v1.clone();
                    length_bytes(a2449)
                };
                let a2451 = 0u64;
                if nat_eq(a2450, a2451) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f454(v2))
        }
    }
}

pub fn f455(v0: List<Bytes>) -> Bytes {
    let m2452 = v0;
    match m2452.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f456(v0: (Bytes, Bytes), p2455: List<(Bytes, Bytes)>) -> bool {
    let m2454 = p2455;
    {
        let mut m2454 = m2454;
        loop {
            let step = {
                let v1 = m2454;
                let m2453 = v1;
                match m2453.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f518(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m2456 = v1;
    match m2456.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h2457, _) = v2;
                h2457
            }, v0.clone()) {
                List::cons({
                    let (_, h2458) = v2;
                    h2458
                }, f457(v0, v3))
            } else {
                f457(v0, v3)
            }
        }
    }
}

pub fn f458(v0: u32) -> Bytes {
    let a2461 = {
        let a2459 = v0;
        let a2460 = 8u32;
        fixed_u32::bit_and(a2459, a2460)
    };
    let a2462 = 0u32;
    if equal(a2461, a2462) {
        let a2465 = {
            let a2463 = v0;
            let a2464 = 4u32;
            fixed_u32::bit_and(a2463, a2464)
        };
        let a2466 = 0u32;
        if equal(a2465, a2466) {
            let a2469 = {
                let a2467 = v0;
                let a2468 = 2u32;
                fixed_u32::bit_and(a2467, a2468)
            };
            let a2470 = 0u32;
            if equal(a2469, a2470) {
                let a2473 = {
                    let a2471 = v0;
                    let a2472 = 1u32;
                    fixed_u32::bit_and(a2471, a2472)
                };
                let a2474 = 0u32;
                if equal(a2473, a2474) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2477 = {
                    let a2475 = v0;
                    let a2476 = 1u32;
                    fixed_u32::bit_and(a2475, a2476)
                };
                let a2478 = 0u32;
                if equal(a2477, a2478) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2481 = {
                let a2479 = v0;
                let a2480 = 2u32;
                fixed_u32::bit_and(a2479, a2480)
            };
            let a2482 = 0u32;
            if equal(a2481, a2482) {
                let a2485 = {
                    let a2483 = v0;
                    let a2484 = 1u32;
                    fixed_u32::bit_and(a2483, a2484)
                };
                let a2486 = 0u32;
                if equal(a2485, a2486) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2489 = {
                    let a2487 = v0;
                    let a2488 = 1u32;
                    fixed_u32::bit_and(a2487, a2488)
                };
                let a2490 = 0u32;
                if equal(a2489, a2490) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2493 = {
            let a2491 = v0;
            let a2492 = 4u32;
            fixed_u32::bit_and(a2491, a2492)
        };
        let a2494 = 0u32;
        if equal(a2493, a2494) {
            let a2497 = {
                let a2495 = v0;
                let a2496 = 2u32;
                fixed_u32::bit_and(a2495, a2496)
            };
            let a2498 = 0u32;
            if equal(a2497, a2498) {
                let a2501 = {
                    let a2499 = v0;
                    let a2500 = 1u32;
                    fixed_u32::bit_and(a2499, a2500)
                };
                let a2502 = 0u32;
                if equal(a2501, a2502) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2505 = {
                    let a2503 = v0;
                    let a2504 = 1u32;
                    fixed_u32::bit_and(a2503, a2504)
                };
                let a2506 = 0u32;
                if equal(a2505, a2506) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2509 = {
                let a2507 = v0;
                let a2508 = 2u32;
                fixed_u32::bit_and(a2507, a2508)
            };
            let a2510 = 0u32;
            if equal(a2509, a2510) {
                let a2513 = {
                    let a2511 = v0;
                    let a2512 = 1u32;
                    fixed_u32::bit_and(a2511, a2512)
                };
                let a2514 = 0u32;
                if equal(a2513, a2514) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2517 = {
                    let a2515 = v0;
                    let a2516 = 1u32;
                    fixed_u32::bit_and(a2515, a2516)
                };
                let a2518 = 0u32;
                if equal(a2517, a2518) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f459(v0: u32, v1: u32) -> u32 {
    let m2521 = {
        let a2519 = v0;
        let a2520 = v1;
        fixed_u32::shift_right(a2519, a2520)
    };
    match m2521 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f460(v0: Adt25, v1: Adt27) -> R<Adt25> {
    let v2: Adt28 = f519(Fn16::F520, 64u64, Adt28::C0(match v0.clone() {
        Adt25::C0(h2522, _, _, _, _, _, _, _) => {
            h2522
        }
    }, match v0.clone() {
        Adt25::C0(_, h2523, _, _, _, _, _, _) => {
            h2523
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2524, _, _, _, _, _) => {
            h2524
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, h2525, _, _, _, _) => {
            h2525
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, h2526, _, _, _) => {
            h2526
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2527, _, _) => {
            h2527
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2528, _) => {
            h2528
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h2529) => {
            h2529
        }
    }, v1, 0u64))?;
    Ok(Adt25::C0(f521(match v0.clone() {
        Adt25::C0(h2530, _, _, _, _, _, _, _) => {
            h2530
        }
    }, match v2.clone() {
        Adt28::C0(h2531, _, _, _, _, _, _, _, _, _) => {
            h2531
        }
    }), f521(match v0.clone() {
        Adt25::C0(_, h2532, _, _, _, _, _, _) => {
            h2532
        }
    }, match v2.clone() {
        Adt28::C0(_, h2533, _, _, _, _, _, _, _, _) => {
            h2533
        }
    }), f521(match v0.clone() {
        Adt25::C0(_, _, h2534, _, _, _, _, _) => {
            h2534
        }
    }, match v2.clone() {
        Adt28::C0(_, _, h2535, _, _, _, _, _, _, _) => {
            h2535
        }
    }), f521(match v0.clone() {
        Adt25::C0(_, _, _, h2536, _, _, _, _) => {
            h2536
        }
    }, match v2.clone() {
        Adt28::C0(_, _, _, h2537, _, _, _, _, _, _) => {
            h2537
        }
    }), f521(match v0.clone() {
        Adt25::C0(_, _, _, _, h2538, _, _, _) => {
            h2538
        }
    }, match v2.clone() {
        Adt28::C0(_, _, _, _, h2539, _, _, _, _, _) => {
            h2539
        }
    }), f521(match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2540, _, _) => {
            h2540
        }
    }, match v2.clone() {
        Adt28::C0(_, _, _, _, _, h2541, _, _, _, _) => {
            h2541
        }
    }), f521(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2542, _) => {
            h2542
        }
    }, match v2.clone() {
        Adt28::C0(_, _, _, _, _, _, h2543, _, _, _) => {
            h2543
        }
    }), f521(match v0 {
        Adt25::C0(_, _, _, _, _, _, _, h2544) => {
            h2544
        }
    }, match v2 {
        Adt28::C0(_, _, _, _, _, _, _, h2545, _, _) => {
            h2545
        }
    })))
}

pub fn f461(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt27> {
    Ok(Adt27::C0(f522(v0.clone(), v1, v2, v3)?, f522(v0.clone(), v1, v2, {
        let a2546 = v3;
        let a2547 = 4u64;
        nat_add(a2546, a2547)?
    })?, f522(v0.clone(), v1, v2, {
        let a2548 = v3;
        let a2549 = 8u64;
        nat_add(a2548, a2549)?
    })?, f522(v0.clone(), v1, v2, {
        let a2550 = v3;
        let a2551 = 12u64;
        nat_add(a2550, a2551)?
    })?, f522(v0.clone(), v1, v2, {
        let a2552 = v3;
        let a2553 = 16u64;
        nat_add(a2552, a2553)?
    })?, f522(v0.clone(), v1, v2, {
        let a2554 = v3;
        let a2555 = 20u64;
        nat_add(a2554, a2555)?
    })?, f522(v0.clone(), v1, v2, {
        let a2556 = v3;
        let a2557 = 24u64;
        nat_add(a2556, a2557)?
    })?, f522(v0.clone(), v1, v2, {
        let a2558 = v3;
        let a2559 = 28u64;
        nat_add(a2558, a2559)?
    })?, f522(v0.clone(), v1, v2, {
        let a2560 = v3;
        let a2561 = 32u64;
        nat_add(a2560, a2561)?
    })?, f522(v0.clone(), v1, v2, {
        let a2562 = v3;
        let a2563 = 36u64;
        nat_add(a2562, a2563)?
    })?, f522(v0.clone(), v1, v2, {
        let a2564 = v3;
        let a2565 = 40u64;
        nat_add(a2564, a2565)?
    })?, f522(v0.clone(), v1, v2, {
        let a2566 = v3;
        let a2567 = 44u64;
        nat_add(a2566, a2567)?
    })?, f522(v0.clone(), v1, v2, {
        let a2568 = v3;
        let a2569 = 48u64;
        nat_add(a2568, a2569)?
    })?, f522(v0.clone(), v1, v2, {
        let a2570 = v3;
        let a2571 = 52u64;
        nat_add(a2570, a2571)?
    })?, f522(v0.clone(), v1, v2, {
        let a2572 = v3;
        let a2573 = 56u64;
        nat_add(a2572, a2573)?
    })?, f522(v0, v1, v2, {
        let a2574 = v3;
        let a2575 = 60u64;
        nat_add(a2574, a2575)?
    })?))
}

pub fn f462(p2578: List<Bytes>, v1: Bytes) -> bool {
    let m2577 = p2578;
    {
        let mut m2577 = m2577;
        loop {
            let step = {
                let v0 = m2577;
                let m2576 = v0;
                match m2576.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f90(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2579, _) = v1;
        h2579
    };
    let a2581 = {
        let a2580 = v0.clone();
        length_bytes(a2580)
    };
    let a2582 = v2;
    if nat_le(a2581, a2582) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2583 = f276(v0.clone(), v2)?;
        let a2584 = 13u64;
        if nat_eq(a2583, a2584) {
            Ok(Some(({
                let a2587 = f276(v0, {
                    let a2585 = v2;
                    let a2586 = 1u64;
                    nat_add(a2585, a2586)?
                })?;
                let a2588 = 10u64;
                if nat_eq(a2587, a2588) {
                    let a2589 = v2;
                    let a2590 = 2u64;
                    nat_add(a2589, a2590)?
                } else {
                    let a2591 = v2;
                    let a2592 = 1u64;
                    nat_add(a2591, a2592)?
                }
            }, {
                let a2594 = {
                    let (_, h2593) = v1;
                    h2593
                };
                let a2595 = Bytes::lit(&[0x0a]);
                append_bytes(a2594, a2595)
            })))
        } else {
            let v4: u64 = {
                let m2596 = f381(v0.clone(), Bytes::lit(&[0x0d]), v2)?;
                match m2596 {
                    None => {
                        let a2597 = v0.clone();
                        length_bytes(a2597)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2601 = {
                    let (_, h2598) = v1;
                    h2598
                };
                let a2602 = f35(v0, v2, {
                    let a2599 = v4;
                    let a2600 = v2;
                    nat_sub(a2599, a2600)
                });
                append_bytes(a2601, a2602)
            })))
        }
    }
}

pub fn f464(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2603 = f523(v0, v1)?;
    match m2603 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f524({
                let (h2604, _) = v2;
                h2604
            }) {
                Ok(Some({
                    let a2606 = v1;
                    let a2607 = {
                        let (_, h2605) = v2;
                        h2605
                    };
                    nat_add(a2606, a2607)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f465(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f525(v1, v0, v2)
}

pub fn f466(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f526(v0, v1)
}

pub fn f467(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f527(v0, v1, v2)
}

pub fn f468(v0: Bytes, p2610: List<Bytes>) -> bool {
    let m2609 = p2610;
    {
        let mut m2609 = m2609;
        loop {
            let step = {
                let v1 = m2609;
                let m2608 = v1;
                match m2608.uncons() {
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
                Ok(next) => { m2609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f275(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2613 = f276(v0.clone(), {
                let a2611 = v1;
                let a2612 = 2u64;
                nat_add(a2611, a2612)?
            })?;
            let a2614 = 120u64;
            nat_eq(a2613, a2614)
        };
        let v3: u64 = if v2 {
            let a2615 = v1;
            let a2616 = 3u64;
            nat_add(a2615, a2616)?
        } else {
            let a2617 = v1;
            let a2618 = 2u64;
            nat_add(a2617, a2618)?
        };
        let v4: (u64, u64) = f528(v0.clone(), v2, v3)?;
        let a2620 = {
            let (h2619, _) = v4;
            h2619
        };
        let a2621 = v3;
        if if nat_eq(a2620, a2621) {
            true
        } else {
            let a2623 = f276(v0, {
                let (h2622, _) = v4;
                h2622
            })?;
            let a2624 = 59u64;
            !nat_eq(a2623, a2624)
        } {
            Ok(Err::<(Bytes, u64), Bytes>(f272(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f524({
                let (_, h2625) = v4;
                h2625
            }) {
                Ok(Ok::<(Bytes, u64), Bytes>((f529({
                    let (_, h2626) = v4;
                    h2626
                })?, {
                    let a2628 = {
                        let (h2627, _) = v4;
                        h2627
                    };
                    let a2629 = 1u64;
                    nat_add(a2628, a2629)?
                })))
            } else {
                Ok(Err::<(Bytes, u64), Bytes>(f272(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f275(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3c]), {
                let a2630 = v1;
                let a2631 = 4u64;
                nat_add(a2630, a2631)?
            })))
        } else {
            if f275(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3e]), {
                    let a2632 = v1;
                    let a2633 = 4u64;
                    nat_add(a2632, a2633)?
                })))
            } else {
                if f275(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x26]), {
                        let a2634 = v1;
                        let a2635 = 5u64;
                        nat_add(a2634, a2635)?
                    })))
                } else {
                    if f275(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x27]), {
                            let a2636 = v1;
                            let a2637 = 6u64;
                            nat_add(a2636, a2637)?
                        })))
                    } else {
                        if f275(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x22]), {
                                let a2638 = v1;
                                let a2639 = 6u64;
                                nat_add(a2638, a2639)?
                            })))
                        } else {
                            Ok(Err::<(Bytes, u64), Bytes>(f272(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f470(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2640, _) = f331(Fn3::F530(v1, v2, v0.clone()), {
        let a2642 = {
            let a2641 = v0;
            length_bytes(a2641)
        };
        let a2643 = 1u64;
        nat_add(a2642, a2643)?
    }, v3)?;
    Ok(h2640)
}

pub fn f471(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2644, _) = v2;
        h2644
    };
    let a2645 = v3;
    let a2646 = 0u64;
    if nat_eq(a2645, a2646) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2647 = v3;
            let a2648 = v1;
            let a2649 = 0u64;
            nat_quot(a2647, a2648, a2649)
        }, {
            let a2654 = f389(v0, {
                let a2650 = v3;
                let a2651 = v1;
                let a2652 = 0u64;
                nat_rem(a2650, a2651, a2652)
            });
            let a2655 = {
                let (_, h2653) = v2;
                h2653
            };
            append_bytes(a2654, a2655)
        }))
    }
}

pub fn f472(p2666: Adt19, v1: List<(Bytes, Bytes)>) -> R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>> {
    let m2665 = p2666;
    {
        let mut m2665 = m2665;
        loop {
            let step = {
                let v0 = m2665;
                let m2656 = v0;
                match m2656 {
                    Adt19::C0(_, _, _) => {
                        Err::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt19::C1(_, _, _) => {
                        Err::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt19::C2(_) => {
                        Err::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt19::C3(r2663, r2664) => {
                        let v2 = (*r2663).clone();
                        let v3 = (*r2664).clone();
                        let m2657 = v2;
                        match m2657 {
                            Adt19::C0(v4, r2661, r2662) => {
                                let v5 = (*r2661).clone();
                                let v6 = (*r2662).clone();
                                let m2658 = f280(v5.clone(), v1.clone());
                                match m2658 {
                                    Ok(v8) => {
                                        let m2659 = f281(v8.clone(), v4)?;
                                        match m2659 {
                                            Ok(v10) => {
                                                if if f282(v10.clone(), Str::lit("restriction")) {
                                                    true
                                                } else {
                                                    if f282(v10.clone(), Str::lit("list")) {
                                                        true
                                                    } else {
                                                        f282(v10.clone(), Str::lit("union"))
                                                    }
                                                } {
                                                    Err::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>(Some(({
                                                        let (_, h2660) = v10;
                                                        h2660
                                                    }, (v5, (v6, v8)))))))
                                                } else {
                                                    Ok::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Err::<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Err::<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>(v7)))
                                    }
                                }
                            }
                            Adt19::C1(_, _, _) => {
                                Ok::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt19::C2(_) => {
                                Ok::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt19::C3(_, _) => {
                                Ok::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt19::C4 => {
                                Ok::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt19::C4 => {
                        Err::<Adt19, R<Result<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt19, (Adt19, List<(Bytes, Bytes)>)))>)))
                    }
                }
            };
            match step {
                Ok(next) => { m2665 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f474(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m2667 = f281(v0, v2.clone())?;
    match m2667 {
        Ok(v4) => {
            if f28({
                let (h2668, _) = v4.clone();
                h2668
            }, f396()) {
                let m2670 = f531({
                    let (_, h2669) = v4;
                    h2669
                }, 16u64);
                match m2670 {
                    Ok(v6) => {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v5))
                    }
                }
            } else {
                if f28({
                    let (h2671, _) = v4.clone();
                    h2671
                }, v1) {
                    let v7: Bytes = f300({
                        let (_, h2672) = v4;
                        h2672
                    })?;
                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h2673, _) = v4;
                        h2673
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(v3))
        }
    }
}

pub fn f475(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f476(v0: Adt19, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt20>, Bytes>> {
    let m2674 = v0;
    match m2674 {
        Adt19::C0(_, _, _) => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
        Adt19::C1(_, _, _) => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
        Adt19::C2(_) => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
        Adt19::C3(r2681, r2682) => {
            let v2 = (*r2681).clone();
            let v3 = (*r2682).clone();
            let m2675 = f476(v3, v1.clone())?;
            match m2675 {
                Ok(v5) => {
                    let m2676 = v2;
                    match m2676 {
                        Adt19::C0(v6, r2680, _) => {
                            let v7 = (*r2680).clone();
                            let m2677 = f280(v7.clone(), v1);
                            match m2677 {
                                Ok(v10) => {
                                    let m2678 = f281(v10, v6)?;
                                    match m2678 {
                                        Ok(v12) => {
                                            if f282(v12, Str::lit("enumeration")) {
                                                let m2679 = f283(v7, f8(Str::lit("value")));
                                                match m2679 {
                                                    None => {
                                                        Ok(Err::<List<Adt20>, Bytes>(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt20>, Bytes>(List::cons(Adt20::C3(v13), v5)))
                                                    }
                                                }
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
                        Adt19::C1(_, _, _) => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                        Adt19::C2(_) => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                        Adt19::C3(_, _) => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                        Adt19::C4 => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt20>, Bytes>(v4))
                }
            }
        }
        Adt19::C4 => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
    }
}

pub fn f477(v0: List<Adt20>, v1: List<Bytes>) -> R<List<Adt20>> {
    let m2683 = v0;
    match m2683.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f532(v2.clone())?;
            if f41(v4.clone(), v1.clone()) {
                f477(v3, v1)
            } else {
                Ok(List::cons(v2, f477(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f478(v0: bool, v1: List<Adt20>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2684 = v1;
    match m2684.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2685 = f533(v0, v3.clone())?;
            match m2685 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f532(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f41(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2686 = f478(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2686 {
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

pub fn f479(p2690: List<Bytes>) -> R<bool> {
    let m2689 = p2690;
    {
        let mut m2689 = m2689;
        loop {
            let step = {
                let v0 = m2689;
                let m2687 = v0;
                match m2687.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2688 = f534(v1)?;
                        match m2688 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f41(v3, f535(false, v2.clone())?) {
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
                Ok(next) => { m2689 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f265(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f481(v0: u64) -> bool {
    let a2691 = v0;
    let a2692 = 47u64;
    nat_eq(a2691, a2692)
}

pub fn f482(v0: Bytes, v1: u64) -> R<Option<(u64, Adt20)>> {
    let v2: bool = {
        let a2693 = f313(v0.clone(), v1)?;
        let a2694 = 45u64;
        nat_eq(a2693, a2694)
    };
    let v3: u64 = if v2 {
        let a2695 = v1;
        let a2696 = 1u64;
        nat_add(a2695, a2696)?
    } else {
        v1
    };
    let v4: u64 = f418(v0.clone(), v3)?;
    let v5: u64 = {
        let a2697 = v4;
        let a2698 = v3;
        nat_sub(a2697, a2698)
    };
    let v6: bool = {
        let a2699 = f313(v0.clone(), v4)?;
        let a2700 = 46u64;
        nat_eq(a2699, a2700)
    };
    let v7: u64 = if v6 {
        let a2701 = v4;
        let a2702 = 1u64;
        nat_add(a2701, a2702)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f418(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f313(v0.clone(), v8)?;
    let v10: bool = {
        let a2703 = v9;
        let a2704 = 101u64;
        if nat_eq(a2703, a2704) {
            true
        } else {
            let a2705 = v9;
            let a2706 = 69u64;
            nat_eq(a2705, a2706)
        }
    };
    let v11: u64 = f313(v0.clone(), {
        let a2707 = v8;
        let a2708 = 1u64;
        nat_add(a2707, a2708)?
    })?;
    let v12: bool = if v10 {
        let a2709 = v11;
        let a2710 = 43u64;
        if nat_eq(a2709, a2710) {
            true
        } else {
            let a2711 = v11;
            let a2712 = 45u64;
            nat_eq(a2711, a2712)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2713 = v8;
            let a2714 = 2u64;
            nat_add(a2713, a2714)?
        } else {
            let a2715 = v8;
            let a2716 = 1u64;
            nat_add(a2715, a2716)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f418(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f35(v0.clone(), v13, {
        let a2717 = v14;
        let a2718 = v13;
        nat_sub(a2717, a2718)
    });
    let v16: u64 = f419(v15.clone())?;
    let v17: Bytes = f35(v15.clone(), v16, {
        let a2720 = {
            let a2719 = v15;
            length_bytes(a2719)
        };
        let a2721 = v16;
        nat_sub(a2720, a2721)
    });
    let a2722 = v5;
    let a2723 = 0u64;
    if if nat_eq(a2722, a2723) {
        true
    } else {
        let a2724 = 1u64;
        let a2725 = v5;
        if if nat_lt(a2724, a2725) {
            let a2726 = f313(v0.clone(), v3)?;
            let a2727 = 48u64;
            nat_eq(a2726, a2727)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2728 = v8;
                let a2729 = v7;
                nat_eq(a2728, a2729)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2730 = v14;
                    let a2731 = v13;
                    nat_eq(a2730, a2731)
                } else {
                    false
                } {
                    true
                } else {
                    let a2733 = 18u64;
                    let a2734 = {
                        let a2732 = v17.clone();
                        length_bytes(a2732)
                    };
                    nat_lt(a2733, a2734)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt20)>)
    } else {
        let m2738 = {
            let a2736 = {
                let a2735 = v17.clone();
                length_bytes(a2735)
            };
            let a2737 = 0u64;
            if nat_eq(a2736, a2737) {
                Some(0u64)
            } else {
                f420(v17)?
            }
        };
        match m2738 {
            None => {
                Ok(None::<(u64, Adt20)>)
            }
            Some(v18) => {
                let m2743 = f536(f35(v0.clone(), v3, v5), f35(v0, v7, {
                    let a2739 = v8;
                    let a2740 = v7;
                    nat_sub(a2739, a2740)
                }), if v12 {
                    let a2741 = v11;
                    let a2742 = 45u64;
                    nat_eq(a2741, a2742)
                } else {
                    false
                }, v18)?;
                match m2743 {
                    None => {
                        Ok(None::<(u64, Adt20)>)
                    }
                    Some(v19) => {
                        let a2746 = 19u64;
                        let a2747 = {
                            let a2745 = {
                                let (h2744, _) = v19.clone();
                                h2744
                            };
                            length_bytes(a2745)
                        };
                        if nat_lt(a2746, a2747) {
                            Ok(None::<(u64, Adt20)>)
                        } else {
                            let m2749 = f420({
                                let (h2748, _) = v19.clone();
                                h2748
                            })?;
                            match m2749 {
                                None => {
                                    Ok(None::<(u64, Adt20)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2750 = 0u64;
                                        let a2751 = v20;
                                        nat_lt(a2750, a2751)
                                    } else {
                                        false
                                    };
                                    if f422(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2753 = {
                                                let (_, h2752) = v19.clone();
                                                h2752
                                            };
                                            let a2754 = 0u64;
                                            if nat_eq(a2753, a2754) {
                                                Adt20::C1(v21, v20)
                                            } else {
                                                Adt20::C2(v21, v20, {
                                                    let (_, h2755) = v19;
                                                    h2755
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt20)>)
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

pub fn f483(v0: u64) -> bool {
    let a2756 = v0;
    let a2757 = 95u64;
    if nat_eq(a2756, a2757) {
        true
    } else {
        let a2758 = v0;
        let a2759 = 45u64;
        if nat_eq(a2758, a2759) {
            true
        } else {
            let a2760 = v0;
            let a2761 = 32u64;
            if nat_eq(a2760, a2761) {
                true
            } else {
                let a2762 = v0;
                let a2763 = 46u64;
                if nat_eq(a2762, a2763) {
                    true
                } else {
                    let a2764 = v0;
                    let a2765 = 47u64;
                    nat_eq(a2764, a2765)
                }
            }
        }
    }
}

pub fn f484(v0: u64) -> bool {
    let a2766 = 97u64;
    let a2767 = v0;
    if nat_le(a2766, a2767) {
        let a2768 = v0;
        let a2769 = 122u64;
        nat_le(a2768, a2769)
    } else {
        false
    }
}

pub fn f485(v0: u64) -> bool {
    let a2770 = 65u64;
    let a2771 = v0;
    if nat_le(a2770, a2771) {
        let a2772 = v0;
        let a2773 = 90u64;
        nat_le(a2772, a2773)
    } else {
        false
    }
}

pub fn f486(v0: u64) -> bool {
    let a2774 = 48u64;
    let a2775 = v0;
    if nat_le(a2774, a2775) {
        let a2776 = v0;
        let a2777 = 57u64;
        nat_le(a2776, a2777)
    } else {
        false
    }
}

pub fn f487(v0: List<(Bytes, Adt20)>, v1: List<(Bytes, Adt20)>) -> List<(Bytes, Adt20)> {
    let m2778 = v0;
    match m2778.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f487(v3, v1))
        }
    }
}

pub fn f488(v0: Option<Adt20>) -> List<(Bytes, Adt20)> {
    let m2779 = v0;
    match m2779 {
        None => {
            List::<(Bytes, Adt20)>::nil()
        }
        Some(v1) => {
            f537(v1)
        }
    }
}

pub fn f489(v0: u64, v1: List<(Bytes, Adt20)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt20)>, Bytes>> {
    let m2780 = v0;
    if m2780 == 0 {
        let m2781 = v1;
        match m2781.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt20)>, Bytes>(List::<(Bytes, Adt20)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt20)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2780 - 1;
        let m2782 = v1.clone();
        match m2782.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt20)>, Bytes>(List::<(Bytes, Adt20)>::nil()))
            }
            Some((_, _)) => {
                let m2783 = f538(v1.clone(), v2.clone(), v3.clone())?;
                match m2783 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt20)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f265(f490(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2786 = f489(v6, f539(v1, {
                            let (h2784, _) = v9.clone();
                            h2784
                        }), List::cons(f300({
                            let (h2785, _) = v9.clone();
                            h2785
                        })?, v2), v3)?;
                        match m2786 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt20)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt20)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f490(v0: List<(Bytes, Adt20)>) -> R<List<Bytes>> {
    let m2787 = v0;
    match m2787.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f300({
                let (h2788, _) = v1;
                h2788
            })?, f490(v2)?))
        }
    }
}

pub fn f491(p2796: u64, p2797: List<Adt26>, p2798: List<Bytes>, p2799: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2795 = (p2796, (p2797, (p2798, p2799)));
    {
        let mut m2795 = m2795;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2795;
                let m2789 = v0;
                if m2789 == 0 {
                    Err::<(u64, (List<Adt26>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2789 - 1;
                    let m2790 = v1;
                    match m2790.uncons() {
                        None => {
                            Err::<(u64, (List<Adt26>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2791 = v5;
                            match m2791 {
                                Adt26::C0(v8, v9) => {
                                    if f28(v8.clone(), f540()) {
                                        Err::<(u64, (List<Adt26>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f41(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt26>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2792 = f542(v8.clone(), v9)?;
                                            match m2792 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt26>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f492({
                                                        let (_, h2793) = v11;
                                                        h2793
                                                    }, List::cons(Adt26::C1({
                                                        let (h2794, _) = v11;
                                                        h2794
                                                    }), v6)), (List::cons(v8, v2), v3))))
                                                }
                                                Err(v10) => {
                                                    Err::<(u64, (List<Adt26>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt26::C1(v7) => {
                                    Ok::<(u64, (List<Adt26>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                                Adt26::C2 => {
                                    if f41(f540(), v2.clone()) {
                                        Ok::<(u64, (List<Adt26>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt26>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f540(), v2), List::cons(f541(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m2800 = v0;
    match m2800.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f492(v3, v1))
        }
    }
}

pub fn f493(v0: List<(Bytes, Adt20)>) -> R<List<Adt26>> {
    let m2801 = v0;
    match m2801.uncons() {
        None => {
            Ok(List::<Adt26>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt26::C0(f300({
                let (h2802, _) = v1;
                h2802
            })?, {
                let (_, h2803) = v1;
                h2803
            }), f493(v2)?))
        }
    }
}

pub fn f494(v0: List<Adt13>, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m2804 = v0;
    match m2804.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            if f28(f47(v3.clone(), v1.clone()), v2.clone()) {
                f494(v4, v1, v2)
            } else {
                List::cons(v3, f494(v4, v1, v2))
            }
        }
    }
}

pub fn f495(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2805 = v0;
    match m2805.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f90(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f265(f497(f121(List::cons(v1, f543(f33(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f495(v3, v1)?))
            }
        }
    }
}

pub fn f496(v0: u64) -> Bytes {
    let m2806 = v0;
    if m2806 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2806 - 1;
        let a2807 = f8(Str::lit("0"));
        let a2808 = f496(v1);
        append_bytes(a2807, a2808)
    }
}

pub fn f497(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2809 = v0;
    match m2809.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f260(v1)?, f497(v2)?))
        }
    }
}

pub fn f498(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2810, _) = v1.clone();
        h2810
    };
    let v3: Bytes = {
        let (h2811, _) = {
            let (_, h2812) = v1.clone();
            h2812
        };
        h2811
    };
    let a2815 = {
        let (_, h2813) = {
            let (_, h2814) = v1;
            h2814
        };
        h2813
    };
    let a2816 = 0u64;
    if nat_eq(a2815, a2816) {
        let v4: u64 = f313(v0.clone(), v2)?;
        let a2817 = v4;
        let a2818 = 34u64;
        if nat_eq(a2817, a2818) {
            Ok(Some(({
                let a2819 = v2;
                let a2820 = 1u64;
                nat_add(a2819, a2820)?
            }, (v3, 1u64))))
        } else {
            let a2821 = v4;
            let a2822 = 92u64;
            if nat_eq(a2821, a2822) {
                f544(v0, v2, v3)
            } else {
                let a2823 = v4;
                let a2824 = 32u64;
                if if nat_lt(a2823, a2824) {
                    true
                } else {
                    let a2825 = v4;
                    let a2826 = 256u64;
                    nat_eq(a2825, a2826)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2827 = v2;
                        let a2828 = 1u64;
                        nat_add(a2827, a2828)?
                    }, ({
                        let a2829 = v3;
                        let a2830 = f35(v0, v2, 1u64);
                        append_bytes(a2829, a2830)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f499(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f545(v0, v1)
}

pub fn f500(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f546(v0, v1)
}

pub fn f501(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2831 = v0.clone();
        length_bytes(a2831)
    };
    let a2832 = 0u64;
    let a2833 = v1;
    if nat_lt(a2832, a2833) {
        let a2834 = f547(v0.clone())?;
        let a2835 = v1;
        if nat_eq(a2834, a2835) {
            let a2836 = 1u64;
            let a2837 = v1;
            if nat_lt(a2836, a2837) {
                let m2840 = {
                    let a2838 = v0;
                    let a2839 = 0u64;
                    index_bytes(a2838, a2839)
                };
                match m2840 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2841 = v2;
                        let a2842 = 48u8;
                        Ok(!equal(a2841, a2842))
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

pub fn f502(v0: Fn13, p2847: u64, p2848: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2846 = (p2847, p2848);
    {
        let mut m2846 = m2846;
        loop {
            let step = {
                let (v1, v2) = m2846;
                let m2843 = v1;
                if m2843 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2843 - 1;
                    let m2845 = {
                        let c2844: Fn13 = v0.clone();
                        c2844.apply(v2)?
                    };
                    match m2845 {
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
                Ok(next) => { m2846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f548(v0, v1)
}

pub fn f504(v0: Bytes) -> R<u64> {
    let (h2849, _) = f331(Fn3::F549(v0.clone()), {
        let a2851 = {
            let a2850 = v0;
            length_bytes(a2850)
        };
        let a2852 = 1u64;
        nat_add(a2851, a2852)?
    }, 0u64)?;
    Ok(h2849)
}

pub fn f505(p2855: List<Adt12>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2854 = p2855;
    {
        let mut m2854 = m2854;
        loop {
            let step = {
                let v0 = m2854;
                let m2853 = v0;
                match m2853.uncons() {
                    None => {
                        Err::<List<Adt12>, Bytes>(v1.clone())
                    }
                    Some((v3, v4)) => {
                        if f90(v1.clone(), f99(v3.clone())) {
                            Err::<List<Adt12>, Bytes>(f58(v3, v2.clone()))
                        } else {
                            Ok::<List<Adt12>, Bytes>(v4)
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

pub fn f506(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2859 = {
        let a2857 = v0.clone();
        let a2858 = {
            let (h2856, _) = v1.clone();
            h2856
        };
        index_bytes(a2857, a2858)
    };
    match m2859 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2861 = {
                    let (h2860, _) = v1.clone();
                    h2860
                };
                let a2862 = 1u64;
                nat_add(a2861, a2862)?
            }, {
                let a2865 = {
                    let (_, h2863) = v1;
                    h2863
                };
                let a2866 = f550(v0, {
                    let (h2864, _) = v1;
                    h2864
                }, v2)?;
                append_bytes(a2865, a2866)
            })))
        }
    }
}

pub fn f507(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2867 = v0.clone();
    match m2867.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2870 = {
                let (h2868, _) = v2;
                h2868
            };
            let a2871 = {
                let (h2869, _) = v1.clone();
                h2869
            };
            if equal(a2870, a2871) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f508(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f551(f552(Fn17::F553, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2876 = {
        let (h2872, _) = f554(Fn18::F555, {
            let a2874 = {
                let a2873 = v0;
                length_list(a2873)
            };
            let a2875 = 1u64;
            nat_add(a2874, a2875)?
        }, v1)?;
        h2872
    };
    match m2876.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f509(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2881 = {
        let a2878 = f319({
            let (h2877, _) = v0;
            h2877
        })?;
        let a2879 = Bytes::lit(&[0x3a]);
        append_bytes(a2878, a2879)
    };
    let a2882 = {
        let (_, h2880) = v0;
        h2880
    };
    Ok(append_bytes(a2881, a2882))
}

pub fn f510(v0: Bytes, v1: u64) -> bool {
    let m2883 = f218(v0, v1);
    match m2883 {
        None => {
            false
        }
        Some(v2) => {
            f556(v2)
        }
    }
}

pub fn f511(v0: Fn14, p2887: List<List<(Bytes, ())>>, p2888: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2886 = (p2887, p2888);
    {
        let mut m2886 = m2886;
        loop {
            let step = {
                let (v1, v2) = m2886;
                let m2884 = v2;
                match m2884.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2885: Fn14 = v0.clone();
                            c2885.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f513(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2890 = {
        let a2889 = v0.clone();
        length_list(a2889)
    };
    let a2891 = 1u64;
    if nat_le(a2890, a2891) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f557(v0)?))
    }
}

pub fn f514(v0: Fn15, p2895: List<(Bytes, ())>, p2896: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2894 = (p2895, p2896);
    {
        let mut m2894 = m2894;
        loop {
            let step = {
                let (v1, v2) = m2894;
                let m2892 = v2;
                match m2892.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2893: Fn15 = v0.clone();
                            c2893.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f516(v0: Adt1, v1: Bytes, v2: List<Adt13>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f117(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f51(v3.clone(), f117(f93(v0, f130())?, f8(Str::lit("at"))));
    let a2898 = {
        let a2897 = v4.clone();
        length_list(a2897)
    };
    let a2899 = 0u64;
    if nat_eq(a2898, a2899) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2902 = {
            let a2900 = v4;
            length_list(a2900)
        };
        let a2903 = {
            let a2901 = v3;
            length_list(a2901)
        };
        if nat_eq(a2902, a2903) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f558(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f517(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt13> = f296(f185(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt13> = f296(f185(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f109(f117(v3.clone(), f8(Str::lit("id"))), f117(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f117(f93(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f51(v5.clone(), f109(v6.clone(), f117(f93(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2905 = {
        let a2904 = v7.clone();
        length_list(a2904)
    };
    let a2906 = 0u64;
    if nat_eq(a2905, a2906) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f40({
            let a2909 = {
                let a2907 = v7;
                length_list(a2907)
            };
            let a2910 = {
                let a2908 = v5;
                length_list(a2908)
            };
            nat_eq(a2909, a2910)
        }) {
            Ok((false, f558(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f121(f109(f51(f559(v4.clone()), f52(f53(v0, f8(Str::lit("model/authorities.toml"))))?), f51(f560(v4), f109(v6, f117(v3, f8(Str::lit("id")))))))?;
            let a2912 = {
                let a2911 = v8.clone();
                length_list(a2911)
            };
            let a2913 = 0u64;
            if nat_eq(a2912, a2913) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f558(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f162(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f518(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f28({
        let (h2914, _) = v0;
        h2914
    }, {
        let (h2915, _) = v1;
        h2915
    }) {
        f28({
            let (_, h2916) = v0;
            h2916
        }, {
            let (_, h2917) = v1;
            h2917
        })
    } else {
        false
    }
}

pub fn f519(v0: Fn16, p2921: u64, p2922: Adt28) -> R<Adt28> {
    let m2920 = (p2921, p2922);
    {
        let mut m2920 = m2920;
        loop {
            let step = {
                let (v1, v2) = m2920;
                let m2918 = v1;
                if m2918 == 0 {
                    Err::<(u64, Adt28), R<Adt28>>(Ok(v2))
                } else {
                    let v3 = m2918 - 1;
                    Ok::<(u64, Adt28), R<Adt28>>((v3, {
                        let c2919: Fn16 = v0.clone();
                        c2919.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f520(v0: Adt28) -> R<Adt28> {
    f561(v0)
}

pub fn f521(v0: u32, v1: u32) -> u32 {
    let m2925 = {
        let a2923 = f562(v0);
        let a2924 = f562(v1);
        fixed_u64::checked_add(a2923, a2924)
    };
    match m2925 {
        None => {
            0u32
        }
        Some(v2) => {
            f563(v2)
        }
    }
}

pub fn f522(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2928 = {
        let a2926 = v3;
        let a2927 = 8u64;
        nat_add(a2926, a2927)?
    };
    let a2929 = v2;
    if nat_eq(a2928, a2929) {
        Ok(f564({
            let a2932 = {
                let a2930 = v1;
                let a2931 = 8u64;
                nat_mul(a2930, a2931)?
            };
            let a2933 = 4294967296u64;
            let a2934 = 0u64;
            nat_quot(a2932, a2933, a2934)
        }))
    } else {
        let a2937 = {
            let a2935 = v3;
            let a2936 = 4u64;
            nat_add(a2935, a2936)?
        };
        let a2938 = v2;
        if nat_eq(a2937, a2938) {
            Ok(f564({
                let a2941 = {
                    let a2939 = v1;
                    let a2940 = 8u64;
                    nat_mul(a2939, a2940)?
                };
                let a2942 = 4294967296u64;
                let a2943 = 0u64;
                nat_rem(a2941, a2942, a2943)
            }))
        } else {
            Ok(f565(f566(v0.clone(), v1, v3), f566(v0.clone(), v1, {
                let a2944 = v3;
                let a2945 = 1u64;
                nat_add(a2944, a2945)?
            }), f566(v0.clone(), v1, {
                let a2946 = v3;
                let a2947 = 2u64;
                nat_add(a2946, a2947)?
            }), f566(v0, v1, {
                let a2948 = v3;
                let a2949 = 3u64;
                nat_add(a2948, a2949)?
            })))
        }
    }
}

pub fn f523(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2952 = {
        let a2950 = v0.clone();
        let a2951 = v1;
        index_bytes(a2950, a2951)
    };
    match m2952 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2997 = {
                    let a2973 = {
                        let a2961 = {
                            let a2955 = {
                                let a2953 = v3;
                                let a2954 = 128u8;
                                fixed_u8::bit_and(a2953, a2954)
                            };
                            let a2956 = 128u8;
                            if equal(a2955, a2956) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2962 = {
                            let a2959 = {
                                let a2957 = v3;
                                let a2958 = 64u8;
                                fixed_u8::bit_and(a2957, a2958)
                            };
                            let a2960 = 64u8;
                            if equal(a2959, a2960) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2961, a2962)?
                    };
                    let a2974 = {
                        let a2971 = {
                            let a2965 = {
                                let a2963 = v3;
                                let a2964 = 32u8;
                                fixed_u8::bit_and(a2963, a2964)
                            };
                            let a2966 = 32u8;
                            if equal(a2965, a2966) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2972 = {
                            let a2969 = {
                                let a2967 = v3;
                                let a2968 = 16u8;
                                fixed_u8::bit_and(a2967, a2968)
                            };
                            let a2970 = 16u8;
                            if equal(a2969, a2970) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2971, a2972)?
                    };
                    nat_add(a2973, a2974)?
                };
                let a2998 = {
                    let a2995 = {
                        let a2983 = {
                            let a2977 = {
                                let a2975 = v3;
                                let a2976 = 8u8;
                                fixed_u8::bit_and(a2975, a2976)
                            };
                            let a2978 = 8u8;
                            if equal(a2977, a2978) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2984 = {
                            let a2981 = {
                                let a2979 = v3;
                                let a2980 = 4u8;
                                fixed_u8::bit_and(a2979, a2980)
                            };
                            let a2982 = 4u8;
                            if equal(a2981, a2982) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2983, a2984)?
                    };
                    let a2996 = {
                        let a2993 = {
                            let a2987 = {
                                let a2985 = v3;
                                let a2986 = 2u8;
                                fixed_u8::bit_and(a2985, a2986)
                            };
                            let a2988 = 2u8;
                            if equal(a2987, a2988) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2994 = {
                            let a2991 = {
                                let a2989 = v3;
                                let a2990 = 1u8;
                                fixed_u8::bit_and(a2989, a2990)
                            };
                            let a2992 = 1u8;
                            if equal(a2991, a2992) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2993, a2994)?
                    };
                    nat_add(a2995, a2996)?
                };
                nat_add(a2997, a2998)?
            };
            let a2999 = v4;
            let a3000 = 128u64;
            if nat_lt(a2999, a3000) {
                Ok(Some((v4, 1u64)))
            } else {
                let a3001 = 194u64;
                let a3002 = v4;
                if if nat_le(a3001, a3002) {
                    let a3003 = v4;
                    let a3004 = 223u64;
                    nat_le(a3003, a3004)
                } else {
                    false
                } {
                    let m3007 = f567(v0, {
                        let a3005 = v1;
                        let a3006 = 1u64;
                        nat_add(a3005, a3006)?
                    })?;
                    match m3007 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f568({
                                let a3012 = {
                                    let a3010 = {
                                        let a3008 = v4;
                                        let a3009 = 192u64;
                                        nat_sub(a3008, a3009)
                                    };
                                    let a3011 = 64u64;
                                    nat_mul(a3010, a3011)?
                                };
                                let a3013 = v5;
                                nat_add(a3012, a3013)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a3014 = 224u64;
                    let a3015 = v4;
                    if if nat_le(a3014, a3015) {
                        let a3016 = v4;
                        let a3017 = 239u64;
                        nat_le(a3016, a3017)
                    } else {
                        false
                    } {
                        let m3020 = f567(v0.clone(), {
                            let a3018 = v1;
                            let a3019 = 1u64;
                            nat_add(a3018, a3019)?
                        })?;
                        match m3020 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m3023 = f567(v0, {
                                    let a3021 = v1;
                                    let a3022 = 2u64;
                                    nat_add(a3021, a3022)?
                                })?;
                                match m3023 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f568({
                                            let a3032 = {
                                                let a3026 = {
                                                    let a3024 = v4;
                                                    let a3025 = 224u64;
                                                    nat_sub(a3024, a3025)
                                                };
                                                let a3027 = 4096u64;
                                                nat_mul(a3026, a3027)?
                                            };
                                            let a3033 = {
                                                let a3030 = {
                                                    let a3028 = v6;
                                                    let a3029 = 64u64;
                                                    nat_mul(a3028, a3029)?
                                                };
                                                let a3031 = v7;
                                                nat_add(a3030, a3031)?
                                            };
                                            nat_add(a3032, a3033)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a3034 = 240u64;
                        let a3035 = v4;
                        if if nat_le(a3034, a3035) {
                            let a3036 = v4;
                            let a3037 = 244u64;
                            nat_le(a3036, a3037)
                        } else {
                            false
                        } {
                            let m3040 = f567(v0.clone(), {
                                let a3038 = v1;
                                let a3039 = 1u64;
                                nat_add(a3038, a3039)?
                            })?;
                            match m3040 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m3043 = f567(v0.clone(), {
                                        let a3041 = v1;
                                        let a3042 = 2u64;
                                        nat_add(a3041, a3042)?
                                    })?;
                                    match m3043 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m3046 = f567(v0, {
                                                let a3044 = v1;
                                                let a3045 = 3u64;
                                                nat_add(a3044, a3045)?
                                            })?;
                                            match m3046 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f568({
                                                        let a3059 = {
                                                            let a3049 = {
                                                                let a3047 = v4;
                                                                let a3048 = 240u64;
                                                                nat_sub(a3047, a3048)
                                                            };
                                                            let a3050 = 262144u64;
                                                            nat_mul(a3049, a3050)?
                                                        };
                                                        let a3060 = {
                                                            let a3057 = {
                                                                let a3051 = v8;
                                                                let a3052 = 4096u64;
                                                                nat_mul(a3051, a3052)?
                                                            };
                                                            let a3058 = {
                                                                let a3055 = {
                                                                    let a3053 = v9;
                                                                    let a3054 = 64u64;
                                                                    nat_mul(a3053, a3054)?
                                                                };
                                                                let a3056 = v10;
                                                                nat_add(a3055, a3056)?
                                                            };
                                                            nat_add(a3057, a3058)?
                                                        };
                                                        nat_add(a3059, a3060)?
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

pub fn f524(v0: u64) -> bool {
    let a3061 = v0;
    let a3062 = 9u64;
    if if if nat_eq(a3061, a3062) {
        true
    } else {
        let a3063 = v0;
        let a3064 = 10u64;
        nat_eq(a3063, a3064)
    } {
        true
    } else {
        let a3065 = v0;
        let a3066 = 13u64;
        nat_eq(a3065, a3066)
    } {
        true
    } else {
        let a3067 = 32u64;
        let a3068 = v0;
        if if if nat_le(a3067, a3068) {
            let a3069 = v0;
            let a3070 = 55295u64;
            nat_le(a3069, a3070)
        } else {
            false
        } {
            true
        } else {
            let a3071 = 57344u64;
            let a3072 = v0;
            if nat_le(a3071, a3072) {
                let a3073 = v0;
                let a3074 = 65533u64;
                nat_le(a3073, a3074)
            } else {
                false
            }
        } {
            true
        } else {
            let a3075 = 65536u64;
            let a3076 = v0;
            if nat_le(a3075, a3076) {
                let a3077 = v0;
                let a3078 = 1114111u64;
                nat_le(a3077, a3078)
            } else {
                false
            }
        }
    }
}

pub fn f525(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f269(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a3080 = v2;
        let a3081 = {
            let a3079 = v0;
            length_bytes(a3079)
        };
        if nat_lt(a3080, a3081) {
            Ok(Some({
                let a3082 = v2;
                let a3083 = 1u64;
                nat_add(a3082, a3083)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f526(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f380(f276(v0, v1)?) {
        Ok(Some({
            let a3084 = v1;
            let a3085 = 1u64;
            nat_add(a3084, a3085)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f527(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m3086 = f523(v0, v2)?;
    match m3086 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a3087 = v2;
            let a3088 = v1;
            if if nat_eq(a3087, a3088) {
                f569({
                    let (h3089, _) = v3;
                    h3089
                })
            } else {
                f570({
                    let (h3090, _) = v3;
                    h3090
                })
            } {
                Ok(Some({
                    let a3092 = v2;
                    let a3093 = {
                        let (_, h3091) = v3;
                        h3091
                    };
                    nat_add(a3092, a3093)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f528(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h3094, _) = f502(Fn13::F571(v1, v0.clone()), {
        let a3096 = {
            let a3095 = v0;
            length_bytes(a3095)
        };
        let a3097 = 1u64;
        nat_add(a3096, a3097)?
    }, (v2, 0u64))?;
    Ok(h3094)
}

pub fn f529(v0: u64) -> R<Bytes> {
    let a3098 = v0;
    let a3099 = 128u64;
    if nat_lt(a3098, a3099) {
        Ok(f572(v0))
    } else {
        let a3100 = v0;
        let a3101 = 2048u64;
        if nat_lt(a3100, a3101) {
            let a3112 = f572({
                let a3105 = 192u64;
                let a3106 = {
                    let a3102 = v0;
                    let a3103 = 64u64;
                    let a3104 = 0u64;
                    nat_quot(a3102, a3103, a3104)
                };
                nat_add(a3105, a3106)?
            });
            let a3113 = f572({
                let a3110 = 128u64;
                let a3111 = {
                    let a3107 = v0;
                    let a3108 = 64u64;
                    let a3109 = 0u64;
                    nat_rem(a3107, a3108, a3109)
                };
                nat_add(a3110, a3111)?
            });
            Ok(append_bytes(a3112, a3113))
        } else {
            let a3114 = v0;
            let a3115 = 65536u64;
            if nat_lt(a3114, a3115) {
                let a3136 = f572({
                    let a3119 = 224u64;
                    let a3120 = {
                        let a3116 = v0;
                        let a3117 = 4096u64;
                        let a3118 = 0u64;
                        nat_quot(a3116, a3117, a3118)
                    };
                    nat_add(a3119, a3120)?
                });
                let a3137 = {
                    let a3134 = f572({
                        let a3127 = 128u64;
                        let a3128 = {
                            let a3124 = {
                                let a3121 = v0;
                                let a3122 = 64u64;
                                let a3123 = 0u64;
                                nat_quot(a3121, a3122, a3123)
                            };
                            let a3125 = 64u64;
                            let a3126 = 0u64;
                            nat_rem(a3124, a3125, a3126)
                        };
                        nat_add(a3127, a3128)?
                    });
                    let a3135 = f572({
                        let a3132 = 128u64;
                        let a3133 = {
                            let a3129 = v0;
                            let a3130 = 64u64;
                            let a3131 = 0u64;
                            nat_rem(a3129, a3130, a3131)
                        };
                        nat_add(a3132, a3133)?
                    });
                    append_bytes(a3134, a3135)
                };
                Ok(append_bytes(a3136, a3137))
            } else {
                let a3168 = f572({
                    let a3141 = 240u64;
                    let a3142 = {
                        let a3138 = v0;
                        let a3139 = 262144u64;
                        let a3140 = 0u64;
                        nat_quot(a3138, a3139, a3140)
                    };
                    nat_add(a3141, a3142)?
                });
                let a3169 = {
                    let a3166 = f572({
                        let a3149 = 128u64;
                        let a3150 = {
                            let a3146 = {
                                let a3143 = v0;
                                let a3144 = 4096u64;
                                let a3145 = 0u64;
                                nat_quot(a3143, a3144, a3145)
                            };
                            let a3147 = 64u64;
                            let a3148 = 0u64;
                            nat_rem(a3146, a3147, a3148)
                        };
                        nat_add(a3149, a3150)?
                    });
                    let a3167 = {
                        let a3164 = f572({
                            let a3157 = 128u64;
                            let a3158 = {
                                let a3154 = {
                                    let a3151 = v0;
                                    let a3152 = 64u64;
                                    let a3153 = 0u64;
                                    nat_quot(a3151, a3152, a3153)
                                };
                                let a3155 = 64u64;
                                let a3156 = 0u64;
                                nat_rem(a3154, a3155, a3156)
                            };
                            nat_add(a3157, a3158)?
                        });
                        let a3165 = f572({
                            let a3162 = 128u64;
                            let a3163 = {
                                let a3159 = v0;
                                let a3160 = 64u64;
                                let a3161 = 0u64;
                                nat_rem(a3159, a3160, a3161)
                            };
                            nat_add(a3162, a3163)?
                        });
                        append_bytes(a3164, a3165)
                    };
                    append_bytes(a3166, a3167)
                };
                Ok(append_bytes(a3168, a3169))
            }
        }
    }
}

pub fn f530(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f573(v2, v0, v1, v3)
}

pub fn f531(p3173: Bytes, p3174: u64) -> Result<Bytes, Bytes> {
    let m3172 = (p3173, p3174);
    {
        let mut m3172 = m3172;
        loop {
            let step = {
                let (v0, v1) = m3172;
                let m3170 = v1;
                if m3170 == 0 {
                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines"))))
                } else {
                    let v2 = m3170 - 1;
                    if f468(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)"))))
                    } else {
                        if f28(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Nat"))))
                        } else {
                            if f28(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Int"))))
                            } else {
                                if f28(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f574()))
                                } else {
                                    if f28(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Bool"))))
                                    } else {
                                        if if f28(v0.clone(), f8(Str::lit("float"))) {
                                            true
                                        } else {
                                            f28(v0.clone(), f8(Str::lit("double")))
                                        } {
                                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil()))))))
                                        } else {
                                            if if f28(v0.clone(), f8(Str::lit("anySimpleType"))) {
                                                true
                                            } else {
                                                f28(v0.clone(), f8(Str::lit("anyAtomicType")))
                                            } {
                                                Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil()))))))
                                            } else {
                                                let m3171 = f575(v0.clone());
                                                match m3171 {
                                                    None => {
                                                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil()))))))
                                                    }
                                                    Some(v3) => {
                                                        if f28(v3.clone(), f8(Str::lit("anyAtomicType"))) {
                                                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Bytes"))))
                                                        } else {
                                                            Ok::<(Bytes, u64), Result<Bytes, Bytes>>((v3, v2))
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
                Ok(next) => { m3172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f532(v0: Adt20) -> R<Bytes> {
    f303(v0)
}

pub fn f533(v0: bool, v1: Adt20) -> R<Option<Bytes>> {
    let m3175 = v1;
    match m3175 {
        Adt20::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt20::C1(v7, v8) => {
            Ok(Some({
                let a3186 = if if v7 {
                    let a3184 = 0u64;
                    let a3185 = v8;
                    nat_lt(a3184, a3185)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3187 = f318(false, v8)?;
                append_bytes(a3186, a3187)
            }))
        }
        Adt20::C2(v3, v4, v5) => {
            let m3178 = f577({
                let a3176 = f8(Str::lit("n"));
                let a3177 = f411(false, v4, v5)?;
                append_bytes(a3176, a3177)
            })?;
            match m3178 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3182 = f8(Str::lit("nMinus"));
                        let a3183 = f35(v6.clone(), 1u64, {
                            let a3180 = {
                                let a3179 = v6;
                                length_bytes(a3179)
                            };
                            let a3181 = 1u64;
                            nat_sub(a3180, a3181)
                        });
                        append_bytes(a3182, a3183)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt20::C3(v2) => {
            f576(v0, v2)
        }
        Adt20::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt20::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt20::C6 => {
            Ok(None::<Bytes>)
        }
        Adt20::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt20::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f534(v0: Bytes) -> R<Option<Bytes>> {
    if f578({
        let a3188 = v0.clone();
        length_bytes(a3188)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f579(v0)?;
        if f580(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3191 = {
                let a3189 = v1.clone();
                let a3190 = 0u64;
                index_bytes(a3189, a3190)
            };
            match m3191 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f486({
                        let v3: u8 = v2;
                        let a3236 = {
                            let a3212 = {
                                let a3200 = {
                                    let a3194 = {
                                        let a3192 = v3;
                                        let a3193 = 128u8;
                                        fixed_u8::bit_and(a3192, a3193)
                                    };
                                    let a3195 = 128u8;
                                    if equal(a3194, a3195) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3201 = {
                                    let a3198 = {
                                        let a3196 = v3;
                                        let a3197 = 64u8;
                                        fixed_u8::bit_and(a3196, a3197)
                                    };
                                    let a3199 = 64u8;
                                    if equal(a3198, a3199) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3200, a3201)?
                            };
                            let a3213 = {
                                let a3210 = {
                                    let a3204 = {
                                        let a3202 = v3;
                                        let a3203 = 32u8;
                                        fixed_u8::bit_and(a3202, a3203)
                                    };
                                    let a3205 = 32u8;
                                    if equal(a3204, a3205) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3211 = {
                                    let a3208 = {
                                        let a3206 = v3;
                                        let a3207 = 16u8;
                                        fixed_u8::bit_and(a3206, a3207)
                                    };
                                    let a3209 = 16u8;
                                    if equal(a3208, a3209) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3210, a3211)?
                            };
                            nat_add(a3212, a3213)?
                        };
                        let a3237 = {
                            let a3234 = {
                                let a3222 = {
                                    let a3216 = {
                                        let a3214 = v3;
                                        let a3215 = 8u8;
                                        fixed_u8::bit_and(a3214, a3215)
                                    };
                                    let a3217 = 8u8;
                                    if equal(a3216, a3217) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3223 = {
                                    let a3220 = {
                                        let a3218 = v3;
                                        let a3219 = 4u8;
                                        fixed_u8::bit_and(a3218, a3219)
                                    };
                                    let a3221 = 4u8;
                                    if equal(a3220, a3221) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3222, a3223)?
                            };
                            let a3235 = {
                                let a3232 = {
                                    let a3226 = {
                                        let a3224 = v3;
                                        let a3225 = 2u8;
                                        fixed_u8::bit_and(a3224, a3225)
                                    };
                                    let a3227 = 2u8;
                                    if equal(a3226, a3227) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3233 = {
                                    let a3230 = {
                                        let a3228 = v3;
                                        let a3229 = 1u8;
                                        fixed_u8::bit_and(a3228, a3229)
                                    };
                                    let a3231 = 1u8;
                                    if equal(a3230, a3231) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3232, a3233)?
                            };
                            nat_add(a3234, a3235)?
                        };
                        nat_add(a3236, a3237)?
                    }) {
                        Ok(Some({
                            let a3238 = f8(Str::lit("n"));
                            let a3239 = v1;
                            append_bytes(a3238, a3239)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f581({
            let a3240 = v0.clone();
            length_bytes(a3240)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f535(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3241 = v1;
    match m3241.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3242 = f576(v0, v2)?;
            match m3242 {
                None => {
                    f535(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f535(v0, v3)?))
                }
            }
        }
    }
}

pub fn f536(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f35(v1.clone(), 0u64, {
        let a3244 = {
            let a3243 = v1.clone();
            length_bytes(a3243)
        };
        let a3245 = f504(v1)?;
        nat_sub(a3244, a3245)
    });
    let v5: Bytes = {
        let a3246 = v0;
        let a3247 = v4.clone();
        append_bytes(a3246, a3247)
    };
    let v6: u64 = f419(v5.clone())?;
    let v7: Bytes = f35(v5.clone(), v6, {
        let a3249 = {
            let a3248 = v5;
            length_bytes(a3248)
        };
        let a3250 = v6;
        nat_sub(a3249, a3250)
    });
    let v8: u64 = {
        let a3251 = v4;
        length_bytes(a3251)
    };
    let v9: u64 = {
        let a3252 = v7.clone();
        length_bytes(a3252)
    };
    let a3253 = v9;
    let a3254 = 0u64;
    if nat_eq(a3253, a3254) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if if v2 {
            true
        } else {
            let a3255 = v3;
            let a3256 = v8;
            nat_lt(a3255, a3256)
        } {
            let v10: u64 = if v2 {
                let a3257 = v3;
                let a3258 = v8;
                nat_add(a3257, a3258)?
            } else {
                let a3259 = v8;
                let a3260 = v3;
                nat_sub(a3259, a3260)
            };
            let v11: u64 = f504(v7.clone())?;
            let a3261 = v10;
            let a3262 = v11;
            if nat_le(a3261, a3262) {
                Ok(Some((f35(v7, 0u64, {
                    let a3263 = v9;
                    let a3264 = v10;
                    nat_sub(a3263, a3264)
                }), 0u64)))
            } else {
                let a3267 = {
                    let a3265 = v10;
                    let a3266 = v11;
                    nat_sub(a3265, a3266)
                };
                let a3268 = 19u64;
                if nat_le(a3267, a3268) {
                    Ok(Some((f35(v7, 0u64, {
                        let a3269 = v9;
                        let a3270 = v11;
                        nat_sub(a3269, a3270)
                    }), {
                        let a3271 = v10;
                        let a3272 = v11;
                        nat_sub(a3271, a3272)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3273 = v3;
                let a3274 = v8;
                nat_sub(a3273, a3274)
            };
            let a3277 = {
                let a3275 = v9;
                let a3276 = v12;
                nat_add(a3275, a3276)?
            };
            let a3278 = 19u64;
            if nat_le(a3277, a3278) {
                Ok(Some(({
                    let a3279 = v7;
                    let a3280 = f496(v12);
                    append_bytes(a3279, a3280)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f537(v0: Adt20) -> List<(Bytes, Adt20)> {
    let m3281 = v0;
    match m3281 {
        Adt20::C0(_) => {
            List::<(Bytes, Adt20)>::nil()
        }
        Adt20::C1(_, _) => {
            List::<(Bytes, Adt20)>::nil()
        }
        Adt20::C2(_, _, _) => {
            List::<(Bytes, Adt20)>::nil()
        }
        Adt20::C3(_) => {
            List::<(Bytes, Adt20)>::nil()
        }
        Adt20::C4(_) => {
            List::<(Bytes, Adt20)>::nil()
        }
        Adt20::C5(r3282) => {
            let v4 = (*r3282).clone();
            f537(v4)
        }
        Adt20::C6 => {
            List::<(Bytes, Adt20)>::nil()
        }
        Adt20::C7(_, _) => {
            List::<(Bytes, Adt20)>::nil()
        }
        Adt20::C8(v1, r3283, r3284) => {
            let v2 = (*r3283).clone();
            let v3 = (*r3284).clone();
            List::cons((v1, v2), f537(v3))
        }
    }
}

pub fn f538(p3289: List<(Bytes, Adt20)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt20)>> {
    let m3288 = p3289;
    {
        let mut m3288 = m3288;
        loop {
            let step = {
                let v0 = m3288;
                let m3285 = v0;
                match m3285.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt20)>, R<Option<(Bytes, Adt20)>>>(Ok(None::<(Bytes, Adt20)>))
                    }
                    Some((v3, v4)) => {
                        if f582(f583({
                            let (_, h3286) = v3.clone();
                            h3286
                        }, List::<Bytes>::nil())?, f300({
                            let (h3287, _) = v3.clone();
                            h3287
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt20)>, R<Option<(Bytes, Adt20)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt20)>, R<Option<(Bytes, Adt20)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3288 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539(v0: List<(Bytes, Adt20)>, v1: Bytes) -> List<(Bytes, Adt20)> {
    let m3290 = v0;
    match m3290.uncons() {
        None => {
            List::<(Bytes, Adt20)>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h3291, _) = v2.clone();
                h3291
            }, v1.clone()) {
                f539(v3, v1)
            } else {
                List::cons(v2, f539(v3, v1))
            }
        }
    }
}

pub fn f540() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f541() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f542(v0: Bytes, v1: Adt20) -> R<Result<(Bytes, List<Adt26>), Bytes>> {
    f584(v0, f585(v1))
}

pub fn f543(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m3292 = v0;
    match m3292.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f543(v3, false))
            } else {
                f543(v3, true)
            }
        }
    }
}

pub fn f544(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f313(v0.clone(), {
        let a3293 = v1;
        let a3294 = 1u64;
        nat_add(a3293, a3294)?
    })?;
    let v4: Bytes = {
        let a3295 = v3;
        let a3296 = 117u64;
        if nat_eq(a3295, a3296) {
            f586(v0, v1)?
        } else {
            f587(v3)
        }
    };
    let a3297 = v4.clone();
    let a3298 = Bytes::lit(&[]);
    if equal(a3297, a3298) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a3299 = v3;
            let a3300 = 117u64;
            if nat_eq(a3299, a3300) {
                let a3301 = v1;
                let a3302 = 6u64;
                nat_add(a3301, a3302)?
            } else {
                let a3303 = v1;
                let a3304 = 2u64;
                nat_add(a3303, a3304)?
            }
        }, ({
            let a3305 = v2;
            let a3306 = v4;
            append_bytes(a3305, a3306)
        }, 0u64))))
    }
}

pub fn f545(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f313(v0, v1)?;
    let a3307 = 47u64;
    let a3308 = v2;
    if if nat_lt(a3307, a3308) {
        let a3309 = v2;
        let a3310 = 58u64;
        nat_lt(a3309, a3310)
    } else {
        false
    } {
        Ok(Some({
            let a3311 = v1;
            let a3312 = 1u64;
            nat_add(a3311, a3312)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f546(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3313 = f313(v0, v1)?;
    let a3314 = 48u64;
    if nat_eq(a3313, a3314) {
        Ok(Some({
            let a3315 = v1;
            let a3316 = 1u64;
            nat_add(a3315, a3316)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f547(v0: Bytes) -> R<u64> {
    let (h3317, _) = f331(Fn3::F588(v0.clone()), {
        let a3318 = v0;
        length_bytes(a3318)
    }, 0u64)?;
    Ok(h3317)
}

pub fn f548(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3322 = {
        let a3320 = v0;
        let a3321 = {
            let (h3319, _) = v1;
            h3319
        };
        index_bytes(a3320, a3321)
    };
    match m3322 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3324 = {
                    let (h3323, _) = v1;
                    h3323
                };
                let a3325 = 1u64;
                nat_add(a3324, a3325)?
            }, {
                let a3329 = {
                    let a3327 = {
                        let (_, h3326) = v1;
                        h3326
                    };
                    let a3328 = 10u64;
                    nat_mul(a3327, a3328)?
                };
                let a3330 = f589(v2)?;
                nat_add(a3329, a3330)?
            })))
        }
    }
}

pub fn f549(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f590(v0, v1)
}

pub fn f550(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3375 = {
            let a3351 = {
                let a3339 = {
                    let a3333 = {
                        let a3331 = v3;
                        let a3332 = 128u8;
                        fixed_u8::bit_and(a3331, a3332)
                    };
                    let a3334 = 128u8;
                    if equal(a3333, a3334) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3340 = {
                    let a3337 = {
                        let a3335 = v3;
                        let a3336 = 64u8;
                        fixed_u8::bit_and(a3335, a3336)
                    };
                    let a3338 = 64u8;
                    if equal(a3337, a3338) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3339, a3340)?
            };
            let a3352 = {
                let a3349 = {
                    let a3343 = {
                        let a3341 = v3;
                        let a3342 = 32u8;
                        fixed_u8::bit_and(a3341, a3342)
                    };
                    let a3344 = 32u8;
                    if equal(a3343, a3344) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3350 = {
                    let a3347 = {
                        let a3345 = v3;
                        let a3346 = 16u8;
                        fixed_u8::bit_and(a3345, a3346)
                    };
                    let a3348 = 16u8;
                    if equal(a3347, a3348) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3349, a3350)?
            };
            nat_add(a3351, a3352)?
        };
        let a3376 = {
            let a3373 = {
                let a3361 = {
                    let a3355 = {
                        let a3353 = v3;
                        let a3354 = 8u8;
                        fixed_u8::bit_and(a3353, a3354)
                    };
                    let a3356 = 8u8;
                    if equal(a3355, a3356) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3362 = {
                    let a3359 = {
                        let a3357 = v3;
                        let a3358 = 4u8;
                        fixed_u8::bit_and(a3357, a3358)
                    };
                    let a3360 = 4u8;
                    if equal(a3359, a3360) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3361, a3362)?
            };
            let a3374 = {
                let a3371 = {
                    let a3365 = {
                        let a3363 = v3;
                        let a3364 = 2u8;
                        fixed_u8::bit_and(a3363, a3364)
                    };
                    let a3366 = 2u8;
                    if equal(a3365, a3366) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3372 = {
                    let a3369 = {
                        let a3367 = v3;
                        let a3368 = 1u8;
                        fixed_u8::bit_and(a3367, a3368)
                    };
                    let a3370 = 1u8;
                    if equal(a3369, a3370) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3371, a3372)?
            };
            nat_add(a3373, a3374)?
        };
        nat_add(a3375, a3376)?
    };
    let a3377 = v4;
    let a3378 = 34u64;
    if nat_eq(a3377, a3378) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3379 = v4;
        let a3380 = 92u64;
        if nat_eq(a3379, a3380) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3381 = v4;
            let a3382 = 8u64;
            if nat_eq(a3381, a3382) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3383 = v4;
                let a3384 = 12u64;
                if nat_eq(a3383, a3384) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3385 = v4;
                    let a3386 = 10u64;
                    if nat_eq(a3385, a3386) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3387 = v4;
                        let a3388 = 13u64;
                        if nat_eq(a3387, a3388) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3389 = v4;
                            let a3390 = 9u64;
                            if nat_eq(a3389, a3390) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3391 = v4;
                                let a3392 = 32u64;
                                if nat_lt(a3391, a3392) {
                                    let a3393 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3394 = f591(v2)?;
                                    Ok(append_bytes(a3393, a3394))
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

pub fn f551(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f592(Fn19::F593, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f552(v0: Fn17, p3398: List<List<(Bytes, Bytes)>>, p3399: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3397 = (p3398, p3399);
    {
        let mut m3397 = m3397;
        loop {
            let step = {
                let (v1, v2) = m3397;
                let m3395 = v2;
                match m3395.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3396: Fn17 = v0.clone();
                            c3396.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3397 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f554(v0: Fn18, p3404: u64, p3405: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3403 = (p3404, p3405);
    {
        let mut m3403 = m3403;
        loop {
            let step = {
                let (v1, v2) = m3403;
                let m3400 = v1;
                if m3400 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3400 - 1;
                    let m3402 = {
                        let c3401: Fn18 = v0.clone();
                        c3401.apply(v2.clone())?
                    };
                    match m3402 {
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
                Ok(next) => { m3403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f594(v0)
}

pub fn f556(v0: u8) -> bool {
    let a3406 = v0;
    let a3407 = 32u8;
    if if if equal(a3406, a3407) {
        true
    } else {
        let a3408 = v0;
        let a3409 = 9u8;
        equal(a3408, a3409)
    } {
        true
    } else {
        let a3410 = v0;
        let a3411 = 10u8;
        equal(a3410, a3411)
    } {
        true
    } else {
        let a3412 = v0;
        let a3413 = 12u8;
        if equal(a3412, a3413) {
            true
        } else {
            let a3414 = v0;
            let a3415 = 13u8;
            equal(a3414, a3415)
        }
    }
}

pub fn f557(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f445({
        let (_, h3416) = {
            let (h3417, _) = f595(Fn20::F596, {
                let a3419 = {
                    let a3418 = v0.clone();
                    length_list(a3418)
                };
                let a3420 = 1u64;
                nat_add(a3419, a3420)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h3417
        };
        h3416
    }))
}

pub fn f558(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f559(v0: List<Adt13>) -> List<Bytes> {
    let m3421 = v0;
    match m3421.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3422 = f89(v1, f8(Str::lit("std")));
            match m3422 {
                None => {
                    f559(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f559(v2))
                }
            }
        }
    }
}

pub fn f560(v0: List<Adt13>) -> List<Bytes> {
    let m3423 = v0;
    match m3423.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3424 = f88(f89(v1, f8(Str::lit("req"))));
            match m3424 {
                None => {
                    f560(v2)
                }
                Some(v3) => {
                    List::cons(v3, f560(v2))
                }
            }
        }
    }
}

pub fn f561(v0: Adt28) -> R<Adt28> {
    let v1: u32 = f597(match v0.clone() {
        Adt28::C0(_, _, _, _, _, _, _, _, _, h3425) => {
            h3425
        }
    });
    let v2: u32 = f521(match v0.clone() {
        Adt28::C0(_, _, _, _, _, _, _, h3426, _, _) => {
            h3426
        }
    }, f521(f598(match v0.clone() {
        Adt28::C0(_, _, _, _, h3427, _, _, _, _, _) => {
            h3427
        }
    }), f521(f599(match v0.clone() {
        Adt28::C0(_, _, _, _, h3428, _, _, _, _, _) => {
            h3428
        }
    }, match v0.clone() {
        Adt28::C0(_, _, _, _, _, h3429, _, _, _, _) => {
            h3429
        }
    }, match v0.clone() {
        Adt28::C0(_, _, _, _, _, _, h3430, _, _, _) => {
            h3430
        }
    }), f521(v1, {
        let m3433 = match v0.clone() {
            Adt28::C0(_, _, _, _, _, _, _, _, h3432, _) => {
                h3432
            }
        };
        match m3433 {
            Adt27::C0(h3431, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3431
            }
        }
    }))));
    let v3: u32 = f521(f600(match v0.clone() {
        Adt28::C0(h3434, _, _, _, _, _, _, _, _, _) => {
            h3434
        }
    }), f601(match v0.clone() {
        Adt28::C0(h3435, _, _, _, _, _, _, _, _, _) => {
            h3435
        }
    }, match v0.clone() {
        Adt28::C0(_, h3436, _, _, _, _, _, _, _, _) => {
            h3436
        }
    }, match v0.clone() {
        Adt28::C0(_, _, h3437, _, _, _, _, _, _, _) => {
            h3437
        }
    }));
    Ok(Adt28::C0(f521(v2, v3), match v0.clone() {
        Adt28::C0(h3438, _, _, _, _, _, _, _, _, _) => {
            h3438
        }
    }, match v0.clone() {
        Adt28::C0(_, h3439, _, _, _, _, _, _, _, _) => {
            h3439
        }
    }, match v0.clone() {
        Adt28::C0(_, _, h3440, _, _, _, _, _, _, _) => {
            h3440
        }
    }, f521(match v0.clone() {
        Adt28::C0(_, _, _, h3441, _, _, _, _, _, _) => {
            h3441
        }
    }, v2), match v0.clone() {
        Adt28::C0(_, _, _, _, h3442, _, _, _, _, _) => {
            h3442
        }
    }, match v0.clone() {
        Adt28::C0(_, _, _, _, _, h3443, _, _, _, _) => {
            h3443
        }
    }, match v0.clone() {
        Adt28::C0(_, _, _, _, _, _, h3444, _, _, _) => {
            h3444
        }
    }, f602(match v0.clone() {
        Adt28::C0(_, _, _, _, _, _, _, _, h3445, _) => {
            h3445
        }
    }), {
        let a3447 = match v0 {
            Adt28::C0(_, _, _, _, _, _, _, _, _, h3446) => {
                h3446
            }
        };
        let a3448 = 1u64;
        nat_add(a3447, a3448)?
    }))
}

pub fn f562(v0: u32) -> u64 {
    let m3450 = {
        let a3449 = v0;
        fixed_u64::convert(i128::from(a3449))
    };
    match m3450 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f563(v0: u64) -> u32 {
    let m3454 = {
        let a3453 = {
            let a3451 = v0;
            let a3452 = 4294967295u64;
            fixed_u64::bit_and(a3451, a3452)
        };
        fixed_u32::convert(i128::from(a3453))
    };
    match m3454 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f564(v0: u64) -> u32 {
    let (_, h3455) = {
        let (_, h3456) = f603(Fn21::F604, 32u64, (v0, (1u32, 0u32)));
        h3456
    };
    h3455
}

pub fn f565(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3461 = f605(v0, 24u32);
    let a3462 = {
        let a3459 = f605(v1, 16u32);
        let a3460 = {
            let a3457 = f605(v2, 8u32);
            let a3458 = v3;
            fixed_u32::bit_or(a3457, a3458)
        };
        fixed_u32::bit_or(a3459, a3460)
    };
    fixed_u32::bit_or(a3461, a3462)
}

pub fn f566(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3463 = v2;
    let a3464 = v1;
    if nat_lt(a3463, a3464) {
        let m3467 = {
            let a3465 = v0;
            let a3466 = v2;
            index_bytes(a3465, a3466)
        };
        match m3467 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3469 = {
                    let a3468 = v3;
                    fixed_u32::convert(i128::from(a3468))
                };
                match m3469 {
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
        let a3470 = v2;
        let a3471 = v1;
        if nat_eq(a3470, a3471) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f567(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3474 = {
        let a3472 = v0;
        let a3473 = v1;
        index_bytes(a3472, a3473)
    };
    match m3474 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3519 = {
                    let a3495 = {
                        let a3483 = {
                            let a3477 = {
                                let a3475 = v3;
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
                                let a3479 = v3;
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
                                let a3485 = v3;
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
                                let a3489 = v3;
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
                                let a3497 = v3;
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
                                let a3501 = v3;
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
                                let a3507 = v3;
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
                                let a3511 = v3;
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
            let a3521 = 128u64;
            let a3522 = v4;
            if if nat_le(a3521, a3522) {
                let a3523 = v4;
                let a3524 = 191u64;
                nat_le(a3523, a3524)
            } else {
                false
            } {
                Ok(Some({
                    let a3525 = v4;
                    let a3526 = 128u64;
                    nat_sub(a3525, a3526)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f568(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3527 = v1;
    let a3528 = v0;
    if if nat_le(a3527, a3528) {
        let a3529 = v0;
        let a3530 = v2;
        if nat_le(a3529, a3530) {
            let a3531 = v0;
            let a3532 = 55296u64;
            if nat_lt(a3531, a3532) {
                true
            } else {
                let a3533 = 57343u64;
                let a3534 = v0;
                nat_lt(a3533, a3534)
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

pub fn f569(v0: u64) -> bool {
    let a3535 = v0;
    let a3536 = 58u64;
    if if if nat_eq(a3535, a3536) {
        true
    } else {
        let a3537 = v0;
        let a3538 = 95u64;
        nat_eq(a3537, a3538)
    } {
        true
    } else {
        let a3539 = 65u64;
        let a3540 = v0;
        if if nat_le(a3539, a3540) {
            let a3541 = v0;
            let a3542 = 90u64;
            nat_le(a3541, a3542)
        } else {
            false
        } {
            true
        } else {
            let a3543 = 97u64;
            let a3544 = v0;
            if nat_le(a3543, a3544) {
                let a3545 = v0;
                let a3546 = 122u64;
                nat_le(a3545, a3546)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a3547 = 192u64;
        let a3548 = v0;
        if if if if nat_le(a3547, a3548) {
            let a3549 = v0;
            let a3550 = 214u64;
            nat_le(a3549, a3550)
        } else {
            false
        } {
            true
        } else {
            let a3551 = 216u64;
            let a3552 = v0;
            if nat_le(a3551, a3552) {
                let a3553 = v0;
                let a3554 = 246u64;
                nat_le(a3553, a3554)
            } else {
                false
            }
        } {
            true
        } else {
            let a3555 = 248u64;
            let a3556 = v0;
            if if nat_le(a3555, a3556) {
                let a3557 = v0;
                let a3558 = 767u64;
                nat_le(a3557, a3558)
            } else {
                false
            } {
                true
            } else {
                let a3559 = 880u64;
                let a3560 = v0;
                if nat_le(a3559, a3560) {
                    let a3561 = v0;
                    let a3562 = 893u64;
                    nat_le(a3561, a3562)
                } else {
                    false
                }
            }
        } {
            true
        } else {
            let a3563 = 895u64;
            let a3564 = v0;
            if if if if nat_le(a3563, a3564) {
                let a3565 = v0;
                let a3566 = 8191u64;
                nat_le(a3565, a3566)
            } else {
                false
            } {
                true
            } else {
                let a3567 = 8204u64;
                let a3568 = v0;
                if nat_le(a3567, a3568) {
                    let a3569 = v0;
                    let a3570 = 8205u64;
                    nat_le(a3569, a3570)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3571 = 8304u64;
                let a3572 = v0;
                if if nat_le(a3571, a3572) {
                    let a3573 = v0;
                    let a3574 = 8591u64;
                    nat_le(a3573, a3574)
                } else {
                    false
                } {
                    true
                } else {
                    let a3575 = 11264u64;
                    let a3576 = v0;
                    if nat_le(a3575, a3576) {
                        let a3577 = v0;
                        let a3578 = 12271u64;
                        nat_le(a3577, a3578)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a3579 = 12289u64;
                let a3580 = v0;
                if if if nat_le(a3579, a3580) {
                    let a3581 = v0;
                    let a3582 = 55295u64;
                    nat_le(a3581, a3582)
                } else {
                    false
                } {
                    true
                } else {
                    let a3583 = 63744u64;
                    let a3584 = v0;
                    if nat_le(a3583, a3584) {
                        let a3585 = v0;
                        let a3586 = 64975u64;
                        nat_le(a3585, a3586)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a3587 = 65008u64;
                    let a3588 = v0;
                    if if nat_le(a3587, a3588) {
                        let a3589 = v0;
                        let a3590 = 65533u64;
                        nat_le(a3589, a3590)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a3591 = 65536u64;
                        let a3592 = v0;
                        if nat_le(a3591, a3592) {
                            let a3593 = v0;
                            let a3594 = 983039u64;
                            nat_le(a3593, a3594)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f570(v0: u64) -> bool {
    if f569(v0) {
        true
    } else {
        let a3595 = v0;
        let a3596 = 45u64;
        if if if nat_eq(a3595, a3596) {
            true
        } else {
            let a3597 = v0;
            let a3598 = 46u64;
            nat_eq(a3597, a3598)
        } {
            true
        } else {
            let a3599 = 48u64;
            let a3600 = v0;
            if nat_le(a3599, a3600) {
                let a3601 = v0;
                let a3602 = 57u64;
                nat_le(a3601, a3602)
            } else {
                false
            }
        } {
            true
        } else {
            let a3603 = v0;
            let a3604 = 183u64;
            if if nat_eq(a3603, a3604) {
                true
            } else {
                let a3605 = 768u64;
                let a3606 = v0;
                if nat_le(a3605, a3606) {
                    let a3607 = v0;
                    let a3608 = 879u64;
                    nat_le(a3607, a3608)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3609 = 8255u64;
                let a3610 = v0;
                if nat_le(a3609, a3610) {
                    let a3611 = v0;
                    let a3612 = 8256u64;
                    nat_le(a3611, a3612)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f571(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f606(v1, v0, v2)
}

pub fn f572(v0: u64) -> Bytes {
    f607(f608({
        let a3613 = v0;
        let a3614 = 16u64;
        let a3615 = 0u64;
        nat_quot(a3613, a3614, a3615)
    }), {
        let a3616 = v0;
        let a3617 = 16u64;
        let a3618 = 0u64;
        nat_rem(a3616, a3617, a3618)
    })
}

pub fn f573(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f609(f276(v0, v3)?, v1, v2) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3619 = v3;
            let a3620 = 1u64;
            nat_add(a3619, a3620)?
        }))
    }
}

pub fn f574() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f575(v0: Bytes) -> Option<Bytes> {
    if f468(v0.clone(), List::cons(f8(Str::lit("string")), List::cons(f8(Str::lit("boolean")), List::cons(f8(Str::lit("decimal")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::cons(f8(Str::lit("duration")), List::cons(f8(Str::lit("dateTime")), List::cons(f8(Str::lit("time")), List::cons(f8(Str::lit("date")), List::cons(f8(Str::lit("gYearMonth")), List::cons(f8(Str::lit("gYear")), List::cons(f8(Str::lit("gMonthDay")), List::cons(f8(Str::lit("gDay")), List::cons(f8(Str::lit("gMonth")), List::cons(f8(Str::lit("hexBinary")), List::cons(f8(Str::lit("base64Binary")), List::cons(f8(Str::lit("anyURI")), List::cons(f8(Str::lit("QName")), List::cons(f8(Str::lit("NOTATION")), List::<Bytes>::nil())))))))))))))))))))) {
        Some(f8(Str::lit("anyAtomicType")))
    } else {
        if f28(v0.clone(), f8(Str::lit("normalizedString"))) {
            Some(f8(Str::lit("string")))
        } else {
            if f28(v0.clone(), f8(Str::lit("token"))) {
                Some(f8(Str::lit("normalizedString")))
            } else {
                if f468(v0.clone(), List::cons(f8(Str::lit("language")), List::cons(f8(Str::lit("NMTOKEN")), List::cons(f8(Str::lit("Name")), List::<Bytes>::nil())))) {
                    Some(f8(Str::lit("token")))
                } else {
                    if f28(v0.clone(), f8(Str::lit("NCName"))) {
                        Some(f8(Str::lit("Name")))
                    } else {
                        if f468(v0.clone(), List::cons(f8(Str::lit("ID")), List::cons(f8(Str::lit("IDREF")), List::cons(f8(Str::lit("ENTITY")), List::<Bytes>::nil())))) {
                            Some(f8(Str::lit("NCName")))
                        } else {
                            if f28(v0.clone(), f8(Str::lit("integer"))) {
                                Some(f8(Str::lit("decimal")))
                            } else {
                                if f468(v0.clone(), List::cons(f8(Str::lit("nonPositiveInteger")), List::cons(f8(Str::lit("long")), List::cons(f8(Str::lit("nonNegativeInteger")), List::<Bytes>::nil())))) {
                                    Some(f8(Str::lit("integer")))
                                } else {
                                    if f28(v0.clone(), f8(Str::lit("negativeInteger"))) {
                                        Some(f8(Str::lit("nonPositiveInteger")))
                                    } else {
                                        if f28(v0.clone(), f8(Str::lit("int"))) {
                                            Some(f8(Str::lit("long")))
                                        } else {
                                            if f28(v0.clone(), f8(Str::lit("short"))) {
                                                Some(f8(Str::lit("int")))
                                            } else {
                                                if f28(v0.clone(), f8(Str::lit("byte"))) {
                                                    Some(f8(Str::lit("short")))
                                                } else {
                                                    if f468(v0.clone(), List::cons(f8(Str::lit("unsignedLong")), List::cons(f8(Str::lit("positiveInteger")), List::<Bytes>::nil()))) {
                                                        Some(f8(Str::lit("nonNegativeInteger")))
                                                    } else {
                                                        if f28(v0.clone(), f8(Str::lit("unsignedInt"))) {
                                                            Some(f8(Str::lit("unsignedLong")))
                                                        } else {
                                                            if f28(v0.clone(), f8(Str::lit("unsignedShort"))) {
                                                                Some(f8(Str::lit("unsignedInt")))
                                                            } else {
                                                                if f28(v0.clone(), f8(Str::lit("unsignedByte"))) {
                                                                    Some(f8(Str::lit("unsignedShort")))
                                                                } else {
                                                                    if f468(v0.clone(), List::cons(f8(Str::lit("yearMonthDuration")), List::cons(f8(Str::lit("dayTimeDuration")), List::<Bytes>::nil()))) {
                                                                        Some(f8(Str::lit("duration")))
                                                                    } else {
                                                                        if f28(v0.clone(), f8(Str::lit("dateTimeStamp"))) {
                                                                            Some(f8(Str::lit("dateTime")))
                                                                        } else {
                                                                            if f468(v0, List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))))) {
                                                                                Some(f8(Str::lit("anySimpleType")))
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
    }
}

pub fn f576(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f577(v1)
    } else {
        f534(v1)
    }
}

pub fn f577(v0: Bytes) -> R<Option<Bytes>> {
    let m3622 = f610({
        let a3621 = v0.clone();
        length_bytes(a3621)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3622 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3624 = {
                let a3623 = v1.clone();
                length_bytes(a3623)
            };
            let a3625 = 0u64;
            if nat_eq(a3624, a3625) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f578(p3681: u64, v1: Bytes, p3682: u64) -> R<bool> {
    let m3680 = (p3681, p3682);
    {
        let mut m3680 = m3680;
        loop {
            let step = {
                let (v0, v2) = m3680;
                let m3626 = v0;
                if m3626 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3626 - 1;
                    let m3629 = {
                        let a3627 = v1.clone();
                        let a3628 = v2;
                        index_bytes(a3627, a3628)
                    };
                    match m3629 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3676 = {
                                let v5: u8 = v4;
                                let a3674 = {
                                    let a3650 = {
                                        let a3638 = {
                                            let a3632 = {
                                                let a3630 = v5;
                                                let a3631 = 128u8;
                                                fixed_u8::bit_and(a3630, a3631)
                                            };
                                            let a3633 = 128u8;
                                            if equal(a3632, a3633) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3639 = {
                                            let a3636 = {
                                                let a3634 = v5;
                                                let a3635 = 64u8;
                                                fixed_u8::bit_and(a3634, a3635)
                                            };
                                            let a3637 = 64u8;
                                            if equal(a3636, a3637) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3638, a3639)?
                                    };
                                    let a3651 = {
                                        let a3648 = {
                                            let a3642 = {
                                                let a3640 = v5;
                                                let a3641 = 32u8;
                                                fixed_u8::bit_and(a3640, a3641)
                                            };
                                            let a3643 = 32u8;
                                            if equal(a3642, a3643) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3649 = {
                                            let a3646 = {
                                                let a3644 = v5;
                                                let a3645 = 16u8;
                                                fixed_u8::bit_and(a3644, a3645)
                                            };
                                            let a3647 = 16u8;
                                            if equal(a3646, a3647) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3648, a3649)?
                                    };
                                    nat_add(a3650, a3651)?
                                };
                                let a3675 = {
                                    let a3672 = {
                                        let a3660 = {
                                            let a3654 = {
                                                let a3652 = v5;
                                                let a3653 = 8u8;
                                                fixed_u8::bit_and(a3652, a3653)
                                            };
                                            let a3655 = 8u8;
                                            if equal(a3654, a3655) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3661 = {
                                            let a3658 = {
                                                let a3656 = v5;
                                                let a3657 = 4u8;
                                                fixed_u8::bit_and(a3656, a3657)
                                            };
                                            let a3659 = 4u8;
                                            if equal(a3658, a3659) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3660, a3661)?
                                    };
                                    let a3673 = {
                                        let a3670 = {
                                            let a3664 = {
                                                let a3662 = v5;
                                                let a3663 = 2u8;
                                                fixed_u8::bit_and(a3662, a3663)
                                            };
                                            let a3665 = 2u8;
                                            if equal(a3664, a3665) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3671 = {
                                            let a3668 = {
                                                let a3666 = v5;
                                                let a3667 = 1u8;
                                                fixed_u8::bit_and(a3666, a3667)
                                            };
                                            let a3669 = 1u8;
                                            if equal(a3668, a3669) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3670, a3671)?
                                    };
                                    nat_add(a3672, a3673)?
                                };
                                nat_add(a3674, a3675)?
                            };
                            let a3677 = 128u64;
                            if nat_lt(a3676, a3677) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3678 = v2;
                                    let a3679 = 1u64;
                                    nat_add(a3678, a3679)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f407(v0.clone(), 0u64, {
        let a3683 = v0;
        length_bytes(a3683)
    }, false, Bytes::lit(&[]))?;
    let m3686 = {
        let a3684 = v1.clone();
        let a3685 = 0u64;
        index_bytes(a3684, a3685)
    };
    match m3686 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3731 = {
                    let a3707 = {
                        let a3695 = {
                            let a3689 = {
                                let a3687 = v3;
                                let a3688 = 128u8;
                                fixed_u8::bit_and(a3687, a3688)
                            };
                            let a3690 = 128u8;
                            if equal(a3689, a3690) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3696 = {
                            let a3693 = {
                                let a3691 = v3;
                                let a3692 = 64u8;
                                fixed_u8::bit_and(a3691, a3692)
                            };
                            let a3694 = 64u8;
                            if equal(a3693, a3694) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3695, a3696)?
                    };
                    let a3708 = {
                        let a3705 = {
                            let a3699 = {
                                let a3697 = v3;
                                let a3698 = 32u8;
                                fixed_u8::bit_and(a3697, a3698)
                            };
                            let a3700 = 32u8;
                            if equal(a3699, a3700) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3706 = {
                            let a3703 = {
                                let a3701 = v3;
                                let a3702 = 16u8;
                                fixed_u8::bit_and(a3701, a3702)
                            };
                            let a3704 = 16u8;
                            if equal(a3703, a3704) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3705, a3706)?
                    };
                    nat_add(a3707, a3708)?
                };
                let a3732 = {
                    let a3729 = {
                        let a3717 = {
                            let a3711 = {
                                let a3709 = v3;
                                let a3710 = 8u8;
                                fixed_u8::bit_and(a3709, a3710)
                            };
                            let a3712 = 8u8;
                            if equal(a3711, a3712) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3718 = {
                            let a3715 = {
                                let a3713 = v3;
                                let a3714 = 4u8;
                                fixed_u8::bit_and(a3713, a3714)
                            };
                            let a3716 = 4u8;
                            if equal(a3715, a3716) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3717, a3718)?
                    };
                    let a3730 = {
                        let a3727 = {
                            let a3721 = {
                                let a3719 = v3;
                                let a3720 = 2u8;
                                fixed_u8::bit_and(a3719, a3720)
                            };
                            let a3722 = 2u8;
                            if equal(a3721, a3722) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3728 = {
                            let a3725 = {
                                let a3723 = v3;
                                let a3724 = 1u8;
                                fixed_u8::bit_and(a3723, a3724)
                            };
                            let a3726 = 1u8;
                            if equal(a3725, a3726) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3727, a3728)?
                    };
                    nat_add(a3729, a3730)?
                };
                nat_add(a3731, a3732)?
            };
            if f485(v4) {
                let a3739 = f35({
                    let a3733 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3733)
                }, {
                    let a3734 = v4;
                    let a3735 = 65u64;
                    nat_sub(a3734, a3735)
                }, 1u64);
                let a3740 = f35(v1.clone(), 1u64, {
                    let a3737 = {
                        let a3736 = v1;
                        length_bytes(a3736)
                    };
                    let a3738 = 1u64;
                    nat_sub(a3737, a3738)
                });
                Ok(append_bytes(a3739, a3740))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f580(v0: Bytes) -> R<bool> {
    let m3743 = {
        let a3741 = v0;
        let a3742 = 0u64;
        index_bytes(a3741, a3742)
    };
    match m3743 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f484({
                let v2: u8 = v1;
                let a3788 = {
                    let a3764 = {
                        let a3752 = {
                            let a3746 = {
                                let a3744 = v2;
                                let a3745 = 128u8;
                                fixed_u8::bit_and(a3744, a3745)
                            };
                            let a3747 = 128u8;
                            if equal(a3746, a3747) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3753 = {
                            let a3750 = {
                                let a3748 = v2;
                                let a3749 = 64u8;
                                fixed_u8::bit_and(a3748, a3749)
                            };
                            let a3751 = 64u8;
                            if equal(a3750, a3751) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3752, a3753)?
                    };
                    let a3765 = {
                        let a3762 = {
                            let a3756 = {
                                let a3754 = v2;
                                let a3755 = 32u8;
                                fixed_u8::bit_and(a3754, a3755)
                            };
                            let a3757 = 32u8;
                            if equal(a3756, a3757) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3763 = {
                            let a3760 = {
                                let a3758 = v2;
                                let a3759 = 16u8;
                                fixed_u8::bit_and(a3758, a3759)
                            };
                            let a3761 = 16u8;
                            if equal(a3760, a3761) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3762, a3763)?
                    };
                    nat_add(a3764, a3765)?
                };
                let a3789 = {
                    let a3786 = {
                        let a3774 = {
                            let a3768 = {
                                let a3766 = v2;
                                let a3767 = 8u8;
                                fixed_u8::bit_and(a3766, a3767)
                            };
                            let a3769 = 8u8;
                            if equal(a3768, a3769) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3775 = {
                            let a3772 = {
                                let a3770 = v2;
                                let a3771 = 4u8;
                                fixed_u8::bit_and(a3770, a3771)
                            };
                            let a3773 = 4u8;
                            if equal(a3772, a3773) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3774, a3775)?
                    };
                    let a3787 = {
                        let a3784 = {
                            let a3778 = {
                                let a3776 = v2;
                                let a3777 = 2u8;
                                fixed_u8::bit_and(a3776, a3777)
                            };
                            let a3779 = 2u8;
                            if equal(a3778, a3779) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3785 = {
                            let a3782 = {
                                let a3780 = v2;
                                let a3781 = 1u8;
                                fixed_u8::bit_and(a3780, a3781)
                            };
                            let a3783 = 1u8;
                            if equal(a3782, a3783) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3784, a3785)?
                    };
                    nat_add(a3786, a3787)?
                };
                nat_add(a3788, a3789)?
            }) {
                Ok(true)
            } else {
                Ok(f485({
                    let v3: u8 = v1;
                    let a3834 = {
                        let a3810 = {
                            let a3798 = {
                                let a3792 = {
                                    let a3790 = v3;
                                    let a3791 = 128u8;
                                    fixed_u8::bit_and(a3790, a3791)
                                };
                                let a3793 = 128u8;
                                if equal(a3792, a3793) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a3799 = {
                                let a3796 = {
                                    let a3794 = v3;
                                    let a3795 = 64u8;
                                    fixed_u8::bit_and(a3794, a3795)
                                };
                                let a3797 = 64u8;
                                if equal(a3796, a3797) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3798, a3799)?
                        };
                        let a3811 = {
                            let a3808 = {
                                let a3802 = {
                                    let a3800 = v3;
                                    let a3801 = 32u8;
                                    fixed_u8::bit_and(a3800, a3801)
                                };
                                let a3803 = 32u8;
                                if equal(a3802, a3803) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a3809 = {
                                let a3806 = {
                                    let a3804 = v3;
                                    let a3805 = 16u8;
                                    fixed_u8::bit_and(a3804, a3805)
                                };
                                let a3807 = 16u8;
                                if equal(a3806, a3807) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3808, a3809)?
                        };
                        nat_add(a3810, a3811)?
                    };
                    let a3835 = {
                        let a3832 = {
                            let a3820 = {
                                let a3814 = {
                                    let a3812 = v3;
                                    let a3813 = 8u8;
                                    fixed_u8::bit_and(a3812, a3813)
                                };
                                let a3815 = 8u8;
                                if equal(a3814, a3815) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a3821 = {
                                let a3818 = {
                                    let a3816 = v3;
                                    let a3817 = 4u8;
                                    fixed_u8::bit_and(a3816, a3817)
                                };
                                let a3819 = 4u8;
                                if equal(a3818, a3819) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3820, a3821)?
                        };
                        let a3833 = {
                            let a3830 = {
                                let a3824 = {
                                    let a3822 = v3;
                                    let a3823 = 2u8;
                                    fixed_u8::bit_and(a3822, a3823)
                                };
                                let a3825 = 2u8;
                                if equal(a3824, a3825) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a3831 = {
                                let a3828 = {
                                    let a3826 = v3;
                                    let a3827 = 1u8;
                                    fixed_u8::bit_and(a3826, a3827)
                                };
                                let a3829 = 1u8;
                                if equal(a3828, a3829) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3830, a3831)?
                        };
                        nat_add(a3832, a3833)?
                    };
                    nat_add(a3834, a3835)?
                }))
            }
        }
    }
}

pub fn f581(p3848: u64, v1: Bytes, p3849: u64, p3850: Bytes) -> R<Option<Bytes>> {
    let m3847 = (p3848, (p3849, p3850));
    {
        let mut m3847 = m3847;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3847;
                let m3836 = v0;
                if m3836 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3836 - 1;
                    let a3838 = {
                        let a3837 = v1.clone();
                        length_bytes(a3837)
                    };
                    let a3839 = v2;
                    if nat_le(a3838, a3839) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3840 = f523(v1.clone(), v2)?;
                        match m3840 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3842 = v2;
                                    let a3843 = {
                                        let (_, h3841) = v5;
                                        h3841
                                    };
                                    nat_add(a3842, a3843)?
                                }, {
                                    let a3845 = v3;
                                    let a3846 = f611({
                                        let (h3844, _) = v5;
                                        h3844
                                    });
                                    append_bytes(a3845, a3846)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(p3853: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m3852 = p3853;
    {
        let mut m3852 = m3852;
        loop {
            let step = {
                let v0 = m3852;
                let m3851 = v0;
                match m3851.uncons() {
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
                Ok(next) => { m3852 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: Adt20, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3854 = v0;
    match m3854 {
        Adt20::C0(_) => {
            Ok(v1)
        }
        Adt20::C1(_, _) => {
            Ok(v1)
        }
        Adt20::C2(_, _, _) => {
            Ok(v1)
        }
        Adt20::C3(_) => {
            Ok(v1)
        }
        Adt20::C4(r3857) => {
            let v2 = (*r3857).clone();
            f583(v2, v1)
        }
        Adt20::C5(r3858) => {
            let v3 = (*r3858).clone();
            f583(v3, v1)
        }
        Adt20::C6 => {
            Ok(v1)
        }
        Adt20::C7(r3859, r3860) => {
            let v4 = (*r3859).clone();
            let v5 = (*r3860).clone();
            f583(v4, f583(v5, v1)?)
        }
        Adt20::C8(v6, r3861, r3862) => {
            let v7 = (*r3861).clone();
            let v8 = (*r3862).clone();
            let v9: List<Bytes> = f583(v7.clone(), f583(v8, v1)?)?;
            if f28(v6, f8(Str::lit("$ref"))) {
                let m3855 = f304(Some(v7));
                match m3855 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m3856 = f612(v10)?;
                        match m3856 {
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

pub fn f584(v0: Bytes, v1: Adt20) -> R<Result<(Bytes, List<Adt26>), Bytes>> {
    let m3863 = f192(v1.clone(), f8(Str::lit("enum")));
    match m3863 {
        None => {
            let m3865 = f192(v1.clone(), f8(Str::lit("oneOf")));
            match m3865 {
                None => {
                    if f617(v1.clone(), v0.clone())? {
                        let m3866 = f618(v0.clone(), f488(f192(v1.clone(), f8(Str::lit("properties")))), f196(f197(f192(v1, f8(Str::lit("required"))))), true)?;
                        match m3866 {
                            Ok(v7) => {
                                Ok(f615(f480(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f265({
                                    let (h3867, _) = v7;
                                    h3867
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h3868) = v7;
                                    h3868
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt26>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m3869 = f488(f192(v1.clone(), f8(Str::lit("properties"))));
                        match m3869.uncons() {
                            None => {
                                let m3872 = f619(v1, {
                                    let a3870 = v0.clone();
                                    let a3871 = f8(Str::lit("Value"));
                                    append_bytes(a3870, a3871)
                                })?;
                                match m3872 {
                                    Ok(v9) => {
                                        Ok(f615(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h3873, _) = v9;
                                            h3873
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3874) = v9;
                                            h3874
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt26>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m3875 = f618(v0.clone(), f488(f192(v1.clone(), f8(Str::lit("properties")))), f196(f197(f192(v1, f8(Str::lit("required"))))), false)?;
                                match m3875 {
                                    Ok(v13) => {
                                        Ok(f615(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m3877 = {
                                                let (h3876, _) = v13.clone();
                                                h3876
                                            };
                                            match m3877.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f265({
                                                        let (h3878, _) = v13;
                                                        h3878
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3879) = v13;
                                            h3879
                                        }))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<(Bytes, List<Adt26>), Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f616(v0, f305(v5))
                }
            }
        }
        Some(v2) => {
            let m3864 = f613(f305(v2), List::<Bytes>::nil())?;
            match m3864 {
                Ok(v4) => {
                    Ok(f615(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f265(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt26>::nil()))
                }
                Err(v3) => {
                    Ok(f614(v3, v0))
                }
            }
        }
    }
}

pub fn f585(v0: Adt20) -> Adt20 {
    let v2: Adt20 = {
        let m3880 = f192(v0.clone(), f8(Str::lit("anyOf")));
        match m3880 {
            None => {
                v0
            }
            Some(v1) => {
                if f620(f305(v1.clone())) {
                    f621(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f622(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f623(f305(v1.clone()))
                    } {
                        v0
                    } else {
                        f624(f621(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt20 = {
        let m3881 = f192(v2.clone(), f8(Str::lit("oneOf")));
        match m3881 {
            None => {
                v2
            }
            Some(v3) => {
                if f620(f305(v3)) {
                    f621(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m3882 = f192(v4.clone(), f8(Str::lit("allOf")));
    match m3882 {
        None => {
            v4
        }
        Some(v5) => {
            if f625(f305(v5)) {
                f621(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f586(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3885 = f35(v0.clone(), {
        let a3883 = v1;
        let a3884 = 2u64;
        nat_add(a3883, a3884)?
    }, 2u64);
    let a3886 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3885, a3886) {
        let m3889 = f626(f35(v0, {
            let a3887 = v1;
            let a3888 = 4u64;
            nat_add(a3887, a3888)?
        }, 2u64))?;
        match m3889 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f313(v2.clone(), 0u64)?;
                let a3890 = v3;
                let a3891 = 32u64;
                if nat_lt(a3890, a3891) {
                    if f627(v3) {
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

pub fn f587(v0: u64) -> Bytes {
    let a3892 = v0;
    let a3893 = 34u64;
    if nat_eq(a3892, a3893) {
        Bytes::lit(&[0x22])
    } else {
        let a3894 = v0;
        let a3895 = 92u64;
        if nat_eq(a3894, a3895) {
            Bytes::lit(&[0x5c])
        } else {
            let a3896 = v0;
            let a3897 = 98u64;
            if nat_eq(a3896, a3897) {
                Bytes::lit(&[0x08])
            } else {
                let a3898 = v0;
                let a3899 = 102u64;
                if nat_eq(a3898, a3899) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3900 = v0;
                    let a3901 = 110u64;
                    if nat_eq(a3900, a3901) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3902 = v0;
                        let a3903 = 114u64;
                        if nat_eq(a3902, a3903) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3904 = v0;
                            let a3905 = 116u64;
                            if nat_eq(a3904, a3905) {
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

pub fn f588(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3908 = {
        let a3906 = v0;
        let a3907 = v1;
        index_bytes(a3906, a3907)
    };
    match m3908 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f628(v2)? {
                Ok(Some({
                    let a3909 = v1;
                    let a3910 = 1u64;
                    nat_add(a3909, a3910)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f589(v0: u8) -> R<u64> {
    let a3957 = {
        let v1: u8 = v0;
        let a3955 = {
            let a3931 = {
                let a3919 = {
                    let a3913 = {
                        let a3911 = v1;
                        let a3912 = 128u8;
                        fixed_u8::bit_and(a3911, a3912)
                    };
                    let a3914 = 128u8;
                    if equal(a3913, a3914) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3920 = {
                    let a3917 = {
                        let a3915 = v1;
                        let a3916 = 64u8;
                        fixed_u8::bit_and(a3915, a3916)
                    };
                    let a3918 = 64u8;
                    if equal(a3917, a3918) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3919, a3920)?
            };
            let a3932 = {
                let a3929 = {
                    let a3923 = {
                        let a3921 = v1;
                        let a3922 = 32u8;
                        fixed_u8::bit_and(a3921, a3922)
                    };
                    let a3924 = 32u8;
                    if equal(a3923, a3924) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3930 = {
                    let a3927 = {
                        let a3925 = v1;
                        let a3926 = 16u8;
                        fixed_u8::bit_and(a3925, a3926)
                    };
                    let a3928 = 16u8;
                    if equal(a3927, a3928) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3929, a3930)?
            };
            nat_add(a3931, a3932)?
        };
        let a3956 = {
            let a3953 = {
                let a3941 = {
                    let a3935 = {
                        let a3933 = v1;
                        let a3934 = 8u8;
                        fixed_u8::bit_and(a3933, a3934)
                    };
                    let a3936 = 8u8;
                    if equal(a3935, a3936) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3942 = {
                    let a3939 = {
                        let a3937 = v1;
                        let a3938 = 4u8;
                        fixed_u8::bit_and(a3937, a3938)
                    };
                    let a3940 = 4u8;
                    if equal(a3939, a3940) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3941, a3942)?
            };
            let a3954 = {
                let a3951 = {
                    let a3945 = {
                        let a3943 = v1;
                        let a3944 = 2u8;
                        fixed_u8::bit_and(a3943, a3944)
                    };
                    let a3946 = 2u8;
                    if equal(a3945, a3946) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3952 = {
                    let a3949 = {
                        let a3947 = v1;
                        let a3948 = 1u8;
                        fixed_u8::bit_and(a3947, a3948)
                    };
                    let a3950 = 1u8;
                    if equal(a3949, a3950) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3951, a3952)?
            };
            nat_add(a3953, a3954)?
        };
        nat_add(a3955, a3956)?
    };
    let a3958 = 48u64;
    Ok(nat_sub(a3957, a3958))
}

pub fn f590(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3959 = v0.clone();
        length_bytes(a3959)
    };
    let a3960 = v1;
    let a3961 = v2;
    if if nat_lt(a3960, a3961) {
        let a3966 = f313(v0, {
            let a3964 = v2;
            let a3965 = {
                let a3962 = v1;
                let a3963 = 1u64;
                nat_add(a3962, a3963)?
            };
            nat_sub(a3964, a3965)
        })?;
        let a3967 = 48u64;
        nat_eq(a3966, a3967)
    } else {
        false
    } {
        Ok(Some({
            let a3968 = v1;
            let a3969 = 1u64;
            nat_add(a3968, a3969)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f591(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4014 = {
            let a3990 = {
                let a3978 = {
                    let a3972 = {
                        let a3970 = v1;
                        let a3971 = 128u8;
                        fixed_u8::bit_and(a3970, a3971)
                    };
                    let a3973 = 128u8;
                    if equal(a3972, a3973) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3979 = {
                    let a3976 = {
                        let a3974 = v1;
                        let a3975 = 64u8;
                        fixed_u8::bit_and(a3974, a3975)
                    };
                    let a3977 = 64u8;
                    if equal(a3976, a3977) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3978, a3979)?
            };
            let a3991 = {
                let a3988 = {
                    let a3982 = {
                        let a3980 = v1;
                        let a3981 = 32u8;
                        fixed_u8::bit_and(a3980, a3981)
                    };
                    let a3983 = 32u8;
                    if equal(a3982, a3983) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3989 = {
                    let a3986 = {
                        let a3984 = v1;
                        let a3985 = 16u8;
                        fixed_u8::bit_and(a3984, a3985)
                    };
                    let a3987 = 16u8;
                    if equal(a3986, a3987) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3988, a3989)?
            };
            nat_add(a3990, a3991)?
        };
        let a4015 = {
            let a4012 = {
                let a4000 = {
                    let a3994 = {
                        let a3992 = v1;
                        let a3993 = 8u8;
                        fixed_u8::bit_and(a3992, a3993)
                    };
                    let a3995 = 8u8;
                    if equal(a3994, a3995) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4001 = {
                    let a3998 = {
                        let a3996 = v1;
                        let a3997 = 4u8;
                        fixed_u8::bit_and(a3996, a3997)
                    };
                    let a3999 = 4u8;
                    if equal(a3998, a3999) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4000, a4001)?
            };
            let a4013 = {
                let a4010 = {
                    let a4004 = {
                        let a4002 = v1;
                        let a4003 = 2u8;
                        fixed_u8::bit_and(a4002, a4003)
                    };
                    let a4005 = 2u8;
                    if equal(a4004, a4005) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4011 = {
                    let a4008 = {
                        let a4006 = v1;
                        let a4007 = 1u8;
                        fixed_u8::bit_and(a4006, a4007)
                    };
                    let a4009 = 1u8;
                    if equal(a4008, a4009) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4010, a4011)?
            };
            nat_add(a4012, a4013)?
        };
        nat_add(a4014, a4015)?
    };
    let a4022 = f389(f629(), {
        let a4016 = v2;
        let a4017 = 16u64;
        let a4018 = 0u64;
        nat_quot(a4016, a4017, a4018)
    });
    let a4023 = f389(f629(), {
        let a4019 = v2;
        let a4020 = 16u64;
        let a4021 = 0u64;
        nat_rem(a4019, a4020, a4021)
    });
    Ok(append_bytes(a4022, a4023))
}

pub fn f592(v0: Fn19, p4027: List<List<(Bytes, Bytes)>>, p4028: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4026 = (p4027, p4028);
    {
        let mut m4026 = m4026;
        loop {
            let step = {
                let (v1, v2) = m4026;
                let m4024 = v2;
                match m4024.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4025: Fn19 = v0.clone();
                            c4025.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f594(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4030 = {
        let a4029 = v0.clone();
        length_list(a4029)
    };
    let a4031 = 1u64;
    if nat_le(a4030, a4031) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f630(v0)?))
    }
}

pub fn f595(v0: Fn20, p4036: u64, p4037: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m4035 = (p4036, p4037);
    {
        let mut m4035 = m4035;
        loop {
            let step = {
                let (v1, v2) = m4035;
                let m4032 = v1;
                if m4032 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4032 - 1;
                    let m4034 = {
                        let c4033: Fn20 = v0.clone();
                        c4033.apply(v2.clone())?
                    };
                    match m4034 {
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
                Ok(next) => { m4035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f631(v0)
}

pub fn f597(v0: u64) -> u32 {
    let a4038 = v0;
    let a4039 = 32u64;
    if nat_lt(a4038, a4039) {
        let a4040 = v0;
        let a4041 = 16u64;
        if nat_lt(a4040, a4041) {
            let a4042 = v0;
            let a4043 = 8u64;
            if nat_lt(a4042, a4043) {
                let a4044 = v0;
                let a4045 = 4u64;
                if nat_lt(a4044, a4045) {
                    let a4046 = v0;
                    let a4047 = 2u64;
                    if nat_lt(a4046, a4047) {
                        let a4048 = v0;
                        let a4049 = 1u64;
                        if nat_lt(a4048, a4049) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4050 = v0;
                        let a4051 = 3u64;
                        if nat_lt(a4050, a4051) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4052 = v0;
                    let a4053 = 6u64;
                    if nat_lt(a4052, a4053) {
                        let a4054 = v0;
                        let a4055 = 5u64;
                        if nat_lt(a4054, a4055) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4056 = v0;
                        let a4057 = 7u64;
                        if nat_lt(a4056, a4057) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4058 = v0;
                let a4059 = 12u64;
                if nat_lt(a4058, a4059) {
                    let a4060 = v0;
                    let a4061 = 10u64;
                    if nat_lt(a4060, a4061) {
                        let a4062 = v0;
                        let a4063 = 9u64;
                        if nat_lt(a4062, a4063) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4064 = v0;
                        let a4065 = 11u64;
                        if nat_lt(a4064, a4065) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4066 = v0;
                    let a4067 = 14u64;
                    if nat_lt(a4066, a4067) {
                        let a4068 = v0;
                        let a4069 = 13u64;
                        if nat_lt(a4068, a4069) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4070 = v0;
                        let a4071 = 15u64;
                        if nat_lt(a4070, a4071) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4072 = v0;
            let a4073 = 24u64;
            if nat_lt(a4072, a4073) {
                let a4074 = v0;
                let a4075 = 20u64;
                if nat_lt(a4074, a4075) {
                    let a4076 = v0;
                    let a4077 = 18u64;
                    if nat_lt(a4076, a4077) {
                        let a4078 = v0;
                        let a4079 = 17u64;
                        if nat_lt(a4078, a4079) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4080 = v0;
                        let a4081 = 19u64;
                        if nat_lt(a4080, a4081) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4082 = v0;
                    let a4083 = 22u64;
                    if nat_lt(a4082, a4083) {
                        let a4084 = v0;
                        let a4085 = 21u64;
                        if nat_lt(a4084, a4085) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4086 = v0;
                        let a4087 = 23u64;
                        if nat_lt(a4086, a4087) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4088 = v0;
                let a4089 = 28u64;
                if nat_lt(a4088, a4089) {
                    let a4090 = v0;
                    let a4091 = 26u64;
                    if nat_lt(a4090, a4091) {
                        let a4092 = v0;
                        let a4093 = 25u64;
                        if nat_lt(a4092, a4093) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4094 = v0;
                        let a4095 = 27u64;
                        if nat_lt(a4094, a4095) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4096 = v0;
                    let a4097 = 30u64;
                    if nat_lt(a4096, a4097) {
                        let a4098 = v0;
                        let a4099 = 29u64;
                        if nat_lt(a4098, a4099) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4100 = v0;
                        let a4101 = 31u64;
                        if nat_lt(a4100, a4101) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4102 = v0;
        let a4103 = 48u64;
        if nat_lt(a4102, a4103) {
            let a4104 = v0;
            let a4105 = 40u64;
            if nat_lt(a4104, a4105) {
                let a4106 = v0;
                let a4107 = 36u64;
                if nat_lt(a4106, a4107) {
                    let a4108 = v0;
                    let a4109 = 34u64;
                    if nat_lt(a4108, a4109) {
                        let a4110 = v0;
                        let a4111 = 33u64;
                        if nat_lt(a4110, a4111) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4112 = v0;
                        let a4113 = 35u64;
                        if nat_lt(a4112, a4113) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4114 = v0;
                    let a4115 = 38u64;
                    if nat_lt(a4114, a4115) {
                        let a4116 = v0;
                        let a4117 = 37u64;
                        if nat_lt(a4116, a4117) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4118 = v0;
                        let a4119 = 39u64;
                        if nat_lt(a4118, a4119) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4120 = v0;
                let a4121 = 44u64;
                if nat_lt(a4120, a4121) {
                    let a4122 = v0;
                    let a4123 = 42u64;
                    if nat_lt(a4122, a4123) {
                        let a4124 = v0;
                        let a4125 = 41u64;
                        if nat_lt(a4124, a4125) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4126 = v0;
                        let a4127 = 43u64;
                        if nat_lt(a4126, a4127) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4128 = v0;
                    let a4129 = 46u64;
                    if nat_lt(a4128, a4129) {
                        let a4130 = v0;
                        let a4131 = 45u64;
                        if nat_lt(a4130, a4131) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4132 = v0;
                        let a4133 = 47u64;
                        if nat_lt(a4132, a4133) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4134 = v0;
            let a4135 = 56u64;
            if nat_lt(a4134, a4135) {
                let a4136 = v0;
                let a4137 = 52u64;
                if nat_lt(a4136, a4137) {
                    let a4138 = v0;
                    let a4139 = 50u64;
                    if nat_lt(a4138, a4139) {
                        let a4140 = v0;
                        let a4141 = 49u64;
                        if nat_lt(a4140, a4141) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4142 = v0;
                        let a4143 = 51u64;
                        if nat_lt(a4142, a4143) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4144 = v0;
                    let a4145 = 54u64;
                    if nat_lt(a4144, a4145) {
                        let a4146 = v0;
                        let a4147 = 53u64;
                        if nat_lt(a4146, a4147) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4148 = v0;
                        let a4149 = 55u64;
                        if nat_lt(a4148, a4149) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4150 = v0;
                let a4151 = 60u64;
                if nat_lt(a4150, a4151) {
                    let a4152 = v0;
                    let a4153 = 58u64;
                    if nat_lt(a4152, a4153) {
                        let a4154 = v0;
                        let a4155 = 57u64;
                        if nat_lt(a4154, a4155) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4156 = v0;
                        let a4157 = 59u64;
                        if nat_lt(a4156, a4157) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4158 = v0;
                    let a4159 = 62u64;
                    if nat_lt(a4158, a4159) {
                        let a4160 = v0;
                        let a4161 = 61u64;
                        if nat_lt(a4160, a4161) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4162 = v0;
                        let a4163 = 63u64;
                        if nat_lt(a4162, a4163) {
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

pub fn f598(v0: u32) -> u32 {
    let a4166 = {
        let a4164 = f632(v0, 6u32, 26u32);
        let a4165 = f632(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4164, a4165)
    };
    let a4167 = f632(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4166, a4167)
}

pub fn f599(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4173 = {
        let a4168 = v0;
        let a4169 = v1;
        fixed_u32::bit_and(a4168, a4169)
    };
    let a4174 = {
        let a4171 = {
            let a4170 = v0;
            fixed_u32::bit_not(a4170)
        };
        let a4172 = v2;
        fixed_u32::bit_and(a4171, a4172)
    };
    fixed_u32::bit_xor(a4173, a4174)
}

pub fn f600(v0: u32) -> u32 {
    let a4177 = {
        let a4175 = f632(v0, 2u32, 30u32);
        let a4176 = f632(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4175, a4176)
    };
    let a4178 = f632(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4177, a4178)
}

pub fn f601(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4187 = {
        let a4183 = {
            let a4179 = v0;
            let a4180 = v1;
            fixed_u32::bit_and(a4179, a4180)
        };
        let a4184 = {
            let a4181 = v0;
            let a4182 = v2;
            fixed_u32::bit_and(a4181, a4182)
        };
        fixed_u32::bit_xor(a4183, a4184)
    };
    let a4188 = {
        let a4185 = v1;
        let a4186 = v2;
        fixed_u32::bit_and(a4185, a4186)
    };
    fixed_u32::bit_xor(a4187, a4188)
}

pub fn f602(v0: Adt27) -> Adt27 {
    Adt27::C0(match v0.clone() {
        Adt27::C0(_, h4189, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4189
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h4190, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4190
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, h4191, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4191
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, h4192, _, _, _, _, _, _, _, _, _, _, _) => {
            h4192
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h4193, _, _, _, _, _, _, _, _, _, _) => {
            h4193
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h4194, _, _, _, _, _, _, _, _, _) => {
            h4194
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h4195, _, _, _, _, _, _, _, _) => {
            h4195
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h4196, _, _, _, _, _, _, _) => {
            h4196
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h4197, _, _, _, _, _, _) => {
            h4197
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, _, h4198, _, _, _, _, _) => {
            h4198
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, _, _, h4199, _, _, _, _) => {
            h4199
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4200, _, _, _) => {
            h4200
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4201, _, _) => {
            h4201
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4202, _) => {
            h4202
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4203) => {
            h4203
        }
    }, f521(f633(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4204, _) => {
            h4204
        }
    }), f521(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h4205, _, _, _, _, _, _) => {
            h4205
        }
    }, f521(f634(match v0.clone() {
        Adt27::C0(_, h4206, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4206
        }
    }), match v0 {
        Adt27::C0(h4207, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4207
        }
    }))))
}

pub fn f603(v0: Fn21, p4211: u64, p4212: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4210 = (p4211, p4212);
    {
        let mut m4210 = m4210;
        loop {
            let step = {
                let (v1, v2) = m4210;
                let m4208 = v1;
                if m4208 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4208 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4209: Fn21 = v0.clone();
                        c4209.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4210 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4213, _) = v0;
        h4213
    };
    let v2: u32 = {
        let (h4214, _) = {
            let (_, h4215) = v0;
            h4215
        };
        h4214
    };
    let v3: u32 = {
        let (_, h4216) = {
            let (_, h4217) = v0;
            h4217
        };
        h4216
    };
    ({
        let a4218 = v1;
        let a4219 = 2u64;
        let a4220 = 0u64;
        nat_quot(a4218, a4219, a4220)
    }, (f605(v2, 1u32), {
        let a4224 = {
            let a4221 = v1;
            let a4222 = 2u64;
            let a4223 = 0u64;
            nat_rem(a4221, a4222, a4223)
        };
        let a4225 = 1u64;
        if nat_eq(a4224, a4225) {
            let a4226 = v3;
            let a4227 = v2;
            fixed_u32::bit_or(a4226, a4227)
        } else {
            v3
        }
    }))
}

pub fn f605(v0: u32, v1: u32) -> u32 {
    let m4230 = {
        let a4228 = v0;
        let a4229 = v1;
        fixed_u32::shift_left(a4228, a4229)
    };
    match m4230 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f606(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f276(v0, {
        let (h4231, _) = v2;
        h4231
    })?;
    let a4233 = 1114111u64;
    let a4234 = {
        let (_, h4232) = v2;
        h4232
    };
    if nat_lt(a4233, a4234) {
        Ok(None::<(u64, u64)>)
    } else {
        let a4235 = 48u64;
        let a4236 = v3;
        if if nat_le(a4235, a4236) {
            let a4237 = v3;
            let a4238 = 57u64;
            nat_le(a4237, a4238)
        } else {
            false
        } {
            Ok(Some(({
                let a4240 = {
                    let (h4239, _) = v2;
                    h4239
                };
                let a4241 = 1u64;
                nat_add(a4240, a4241)?
            }, {
                let a4247 = {
                    let a4243 = {
                        let (_, h4242) = v2;
                        h4242
                    };
                    let a4244 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a4243, a4244)?
                };
                let a4248 = {
                    let a4245 = v3;
                    let a4246 = 48u64;
                    nat_sub(a4245, a4246)
                };
                nat_add(a4247, a4248)?
            })))
        } else {
            if if v1 {
                let a4249 = 97u64;
                let a4250 = v3;
                if nat_le(a4249, a4250) {
                    let a4251 = v3;
                    let a4252 = 102u64;
                    nat_le(a4251, a4252)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some(({
                    let a4254 = {
                        let (h4253, _) = v2;
                        h4253
                    };
                    let a4255 = 1u64;
                    nat_add(a4254, a4255)?
                }, {
                    let a4261 = {
                        let a4257 = {
                            let (_, h4256) = v2;
                            h4256
                        };
                        let a4258 = 16u64;
                        nat_mul(a4257, a4258)?
                    };
                    let a4262 = {
                        let a4259 = v3;
                        let a4260 = 87u64;
                        nat_sub(a4259, a4260)
                    };
                    nat_add(a4261, a4262)?
                })))
            } else {
                if if v1 {
                    let a4263 = 65u64;
                    let a4264 = v3;
                    if nat_le(a4263, a4264) {
                        let a4265 = v3;
                        let a4266 = 70u64;
                        nat_le(a4265, a4266)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(({
                        let a4268 = {
                            let (h4267, _) = v2;
                            h4267
                        };
                        let a4269 = 1u64;
                        nat_add(a4268, a4269)?
                    }, {
                        let a4275 = {
                            let a4271 = {
                                let (_, h4270) = v2;
                                h4270
                            };
                            let a4272 = 16u64;
                            nat_mul(a4271, a4272)?
                        };
                        let a4276 = {
                            let a4273 = v3;
                            let a4274 = 55u64;
                            nat_sub(a4273, a4274)
                        };
                        nat_add(a4275, a4276)?
                    })))
                } else {
                    Ok(None::<(u64, u64)>)
                }
            }
        }
    }
}

pub fn f607(p4283: List<Bytes>, p4284: u64) -> Bytes {
    let m4282 = (p4283, p4284);
    {
        let mut m4282 = m4282;
        loop {
            let step = {
                let (v0, v1) = m4282;
                let m4277 = v0;
                match m4277.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a4278 = v1;
                        let a4279 = 0u64;
                        if nat_eq(a4278, a4279) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a4280 = v1;
                                let a4281 = 1u64;
                                nat_sub(a4280, a4281)
                            }))
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

pub fn f608(v0: u64) -> List<Bytes> {
    let a4285 = v0;
    let a4286 = 0u64;
    if nat_eq(a4285, a4286) {
        f635()
    } else {
        let a4287 = v0;
        let a4288 = 1u64;
        if nat_eq(a4287, a4288) {
            f636()
        } else {
            let a4289 = v0;
            let a4290 = 2u64;
            if nat_eq(a4289, a4290) {
                f637()
            } else {
                let a4291 = v0;
                let a4292 = 3u64;
                if nat_eq(a4291, a4292) {
                    f638()
                } else {
                    let a4293 = v0;
                    let a4294 = 4u64;
                    if nat_eq(a4293, a4294) {
                        f639()
                    } else {
                        let a4295 = v0;
                        let a4296 = 5u64;
                        if nat_eq(a4295, a4296) {
                            f640()
                        } else {
                            let a4297 = v0;
                            let a4298 = 6u64;
                            if nat_eq(a4297, a4298) {
                                f641()
                            } else {
                                let a4299 = v0;
                                let a4300 = 7u64;
                                if nat_eq(a4299, a4300) {
                                    f642()
                                } else {
                                    let a4301 = v0;
                                    let a4302 = 8u64;
                                    if nat_eq(a4301, a4302) {
                                        f643()
                                    } else {
                                        let a4303 = v0;
                                        let a4304 = 9u64;
                                        if nat_eq(a4303, a4304) {
                                            f644()
                                        } else {
                                            let a4305 = v0;
                                            let a4306 = 10u64;
                                            if nat_eq(a4305, a4306) {
                                                f645()
                                            } else {
                                                let a4307 = v0;
                                                let a4308 = 11u64;
                                                if nat_eq(a4307, a4308) {
                                                    f646()
                                                } else {
                                                    let a4309 = v0;
                                                    let a4310 = 12u64;
                                                    if nat_eq(a4309, a4310) {
                                                        f647()
                                                    } else {
                                                        let a4311 = v0;
                                                        let a4312 = 13u64;
                                                        if nat_eq(a4311, a4312) {
                                                            f648()
                                                        } else {
                                                            let a4313 = v0;
                                                            let a4314 = 14u64;
                                                            if nat_eq(a4313, a4314) {
                                                                f649()
                                                            } else {
                                                                f650()
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

pub fn f609(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        let a4315 = v0;
        let a4316 = v2;
        if if if nat_eq(a4315, a4316) {
            true
        } else {
            let a4317 = v0;
            let a4318 = 60u64;
            nat_eq(a4317, a4318)
        } {
            true
        } else {
            let a4319 = v0;
            let a4320 = 38u64;
            if nat_eq(a4319, a4320) {
                true
            } else {
                let a4321 = v0;
                let a4322 = 256u64;
                nat_eq(a4321, a4322)
            }
        } {
            true
        } else {
            let a4323 = v0;
            let a4324 = 9u64;
            if nat_eq(a4323, a4324) {
                true
            } else {
                let a4325 = v0;
                let a4326 = 10u64;
                nat_eq(a4325, a4326)
            }
        }
    } else {
        let a4327 = v0;
        let a4328 = 60u64;
        if if nat_eq(a4327, a4328) {
            true
        } else {
            let a4329 = v0;
            let a4330 = 38u64;
            nat_eq(a4329, a4330)
        } {
            true
        } else {
            let a4331 = v0;
            let a4332 = 93u64;
            if nat_eq(a4331, a4332) {
                true
            } else {
                let a4333 = v0;
                let a4334 = 256u64;
                nat_eq(a4333, a4334)
            }
        }
    }
}

pub fn f610(p4347: u64, v1: Bytes, p4348: u64, p4349: Bytes) -> R<Option<Bytes>> {
    let m4346 = (p4347, (p4348, p4349));
    {
        let mut m4346 = m4346;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4346;
                let m4335 = v0;
                if m4335 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4335 - 1;
                    let a4337 = {
                        let a4336 = v1.clone();
                        length_bytes(a4336)
                    };
                    let a4338 = v2;
                    if nat_le(a4337, a4338) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4339 = f523(v1.clone(), v2)?;
                        match m4339 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4341 = v2;
                                    let a4342 = {
                                        let (_, h4340) = v5;
                                        h4340
                                    };
                                    nat_add(a4341, a4342)?
                                }, {
                                    let a4344 = v3;
                                    let a4345 = f651(v1.clone(), v2, {
                                        let (h4343, _) = v5;
                                        h4343
                                    });
                                    append_bytes(a4344, a4345)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f611(v0: u64) -> Bytes {
    let a4350 = f8(Str::lit("u"));
    let a4351 = f652(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4350, a4351)
}

pub fn f612(v0: Bytes) -> R<Option<Bytes>> {
    if f90(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f300(f35(v0.clone(), 8u64, {
            let a4353 = {
                let a4352 = v0;
                length_bytes(a4352)
            };
            let a4354 = 8u64;
            nat_sub(a4353, a4354)
        }))?))
    } else {
        if f90(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f300(f35(v0.clone(), 14u64, {
                let a4356 = {
                    let a4355 = v0;
                    length_bytes(a4355)
                };
                let a4357 = 14u64;
                nat_sub(a4356, a4357)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f613(v0: List<Adt20>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f478(f479(f196(v0.clone()))?, v0, v1)
}

pub fn f614(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt26>), Bytes> {
    Err::<(Bytes, List<Adt26>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f615(v0: Bytes, v1: List<Adt26>) -> Result<(Bytes, List<Adt26>), Bytes> {
    Ok::<(Bytes, List<Adt26>), Bytes>((v0, v1))
}

pub fn f616(v0: Bytes, v1: List<Adt20>) -> R<Result<(Bytes, List<Adt26>), Bytes>> {
    let m4358 = v1.clone();
    match m4358.uncons() {
        None => {
            Ok(f614(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m4359 = f653(f488(f192(v2, f8(Str::lit("properties")))), v1.clone());
            match m4359.uncons() {
                None => {
                    let m4360 = f654(f479(f655(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m4360 {
                        Ok(v5) => {
                            Ok(f615(f480(v0, {
                                let (h4361, _) = v5;
                                h4361
                            }), {
                                let (_, h4362) = v5;
                                h4362
                            }))
                        }
                        Err(v4) => {
                            Ok(f614(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m4363 = f656(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m4363 {
                        Ok(v9) => {
                            Ok(f615(f480(v0, {
                                let (h4364, _) = v9;
                                h4364
                            }), {
                                let (_, h4365) = v9;
                                h4365
                            }))
                        }
                        Err(v8) => {
                            Ok(f614(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f617(v0: Adt20, v1: Bytes) -> R<bool> {
    let m4366 = v0;
    match m4366 {
        Adt20::C0(_) => {
            Ok(false)
        }
        Adt20::C1(_, _) => {
            Ok(false)
        }
        Adt20::C2(_, _, _) => {
            Ok(false)
        }
        Adt20::C3(_) => {
            Ok(false)
        }
        Adt20::C4(r4367) => {
            let v2 = (*r4367).clone();
            f617(v2, v1)
        }
        Adt20::C5(r4368) => {
            let v3 = (*r4368).clone();
            f617(v3, v1)
        }
        Adt20::C6 => {
            Ok(false)
        }
        Adt20::C7(r4369, r4370) => {
            let v4 = (*r4369).clone();
            let v5 = (*r4370).clone();
            if f617(v4, v1.clone())? {
                Ok(true)
            } else {
                f617(v5, v1)
            }
        }
        Adt20::C8(v6, r4371, r4372) => {
            let v7 = (*r4371).clone();
            let v8 = (*r4372).clone();
            if if f28(v6, f8(Str::lit("$ref"))) {
                f657(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f617(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f617(v8, v1)
                }
            }
        }
    }
}

pub fn f618(v0: Bytes, v1: List<(Bytes, Adt20)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt26>), Bytes>> {
    f658(f479(f659(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f619(v0: Adt20, v1: Bytes) -> R<Result<(Bytes, List<Adt26>), Bytes>> {
    let m4373 = f660(v0.clone());
    match m4373 {
        None => {
            if f28(f195(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4374 = f192(v0, f8(Str::lit("items")));
                match m4374 {
                    None => {
                        Ok(f661(f8(Str::lit("List")), f663()))
                    }
                    Some(v3) => {
                        Ok(f661(f8(Str::lit("List")), f662(v3, {
                            let a4375 = v1;
                            let a4376 = f8(Str::lit("Item"));
                            append_bytes(a4375, a4376)
                        })?))
                    }
                }
            } else {
                f662(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f661(f8(Str::lit("Option")), f662(v2, v1)?))
        }
    }
}

pub fn f620(p4379: List<Adt20>) -> bool {
    let m4378 = p4379;
    {
        let mut m4378 = m4378;
        loop {
            let step = {
                let v0 = m4378;
                let m4377 = v0;
                match m4377.uncons() {
                    None => {
                        Err::<List<Adt20>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f664(f537(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt20>, bool>(v2)
                        } else {
                            Err::<List<Adt20>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: Adt20, v1: Bytes) -> Adt20 {
    let m4380 = v0;
    match m4380 {
        Adt20::C0(v6) => {
            Adt20::C0(v6)
        }
        Adt20::C1(v10, v11) => {
            Adt20::C1(v10, v11)
        }
        Adt20::C2(v7, v8, v9) => {
            Adt20::C2(v7, v8, v9)
        }
        Adt20::C3(v12) => {
            Adt20::C3(v12)
        }
        Adt20::C4(r4381) => {
            let v13 = (*r4381).clone();
            Adt20::C4(std::rc::Rc::new(v13))
        }
        Adt20::C5(r4382) => {
            let v5 = (*r4382).clone();
            Adt20::C5(std::rc::Rc::new(f621(v5, v1)))
        }
        Adt20::C6 => {
            Adt20::C6
        }
        Adt20::C7(r4383, r4384) => {
            let v14 = (*r4383).clone();
            let v15 = (*r4384).clone();
            Adt20::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt20::C8(v2, r4385, r4386) => {
            let v3 = (*r4385).clone();
            let v4 = (*r4386).clone();
            if f28(v2.clone(), v1.clone()) {
                f621(v4, v1)
            } else {
                Adt20::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f621(v4, v1)))
            }
        }
    }
}

pub fn f622(v0: Adt20, v1: Bytes) -> bool {
    let m4387 = f192(v0, v1);
    match m4387 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f623(v0: List<Adt20>) -> bool {
    let m4388 = v0.clone();
    match m4388.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m4389 = f653(f488(f192(v1, f8(Str::lit("properties")))), v0.clone());
            match m4389.uncons() {
                None => {
                    f666(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f665(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f624(v0: Adt20, v1: Bytes, v2: Adt20) -> Adt20 {
    let m4390 = v0;
    match m4390 {
        Adt20::C0(v7) => {
            Adt20::C0(v7)
        }
        Adt20::C1(v11, v12) => {
            Adt20::C1(v11, v12)
        }
        Adt20::C2(v8, v9, v10) => {
            Adt20::C2(v8, v9, v10)
        }
        Adt20::C3(v13) => {
            Adt20::C3(v13)
        }
        Adt20::C4(r4391) => {
            let v14 = (*r4391).clone();
            Adt20::C4(std::rc::Rc::new(v14))
        }
        Adt20::C5(r4392) => {
            let v3 = (*r4392).clone();
            Adt20::C5(std::rc::Rc::new(Adt20::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt20::C6 => {
            Adt20::C6
        }
        Adt20::C7(r4393, r4394) => {
            let v15 = (*r4393).clone();
            let v16 = (*r4394).clone();
            Adt20::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt20::C8(v4, r4395, r4396) => {
            let v5 = (*r4395).clone();
            let v6 = (*r4396).clone();
            Adt20::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f625(p4400: List<Adt20>) -> bool {
    let m4399 = p4400;
    {
        let mut m4399 = m4399;
        loop {
            let step = {
                let v0 = m4399;
                let m4397 = v0;
                match m4397.uncons() {
                    None => {
                        Err::<List<Adt20>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f664(f537(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f620(f305({
                                let m4398 = f192(v1, f8(Str::lit("anyOf")));
                                match m4398 {
                                    None => {
                                        Adt20::C6
                                    }
                                    Some(v3) => {
                                        v3
                                    }
                                }
                            })) {
                                Ok::<List<Adt20>, bool>(v2)
                            } else {
                                Err::<List<Adt20>, bool>(false)
                            }
                        } else {
                            Err::<List<Adt20>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a4401 = v0.clone();
        length_bytes(a4401)
    };
    let a4405 = {
        let a4402 = v1;
        let a4403 = 2u64;
        let a4404 = 0u64;
        nat_rem(a4402, a4403, a4404)
    };
    let a4406 = 0u64;
    if nat_eq(a4405, a4406) {
        let (_, h4407) = {
            let (h4408, _) = f667(Fn22::F668(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h4408
        };
        Ok(h4407)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f627(v0: u64) -> bool {
    let a4409 = v0;
    let a4410 = 8u64;
    if nat_eq(a4409, a4410) {
        true
    } else {
        let a4411 = v0;
        let a4412 = 9u64;
        if nat_eq(a4411, a4412) {
            true
        } else {
            let a4413 = v0;
            let a4414 = 10u64;
            if nat_eq(a4413, a4414) {
                true
            } else {
                let a4415 = v0;
                let a4416 = 12u64;
                if nat_eq(a4415, a4416) {
                    true
                } else {
                    let a4417 = v0;
                    let a4418 = 13u64;
                    nat_eq(a4417, a4418)
                }
            }
        }
    }
}

pub fn f628(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4463 = {
            let a4439 = {
                let a4427 = {
                    let a4421 = {
                        let a4419 = v1;
                        let a4420 = 128u8;
                        fixed_u8::bit_and(a4419, a4420)
                    };
                    let a4422 = 128u8;
                    if equal(a4421, a4422) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4428 = {
                    let a4425 = {
                        let a4423 = v1;
                        let a4424 = 64u8;
                        fixed_u8::bit_and(a4423, a4424)
                    };
                    let a4426 = 64u8;
                    if equal(a4425, a4426) {
                        64u64
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
                        let a4430 = 32u8;
                        fixed_u8::bit_and(a4429, a4430)
                    };
                    let a4432 = 32u8;
                    if equal(a4431, a4432) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4438 = {
                    let a4435 = {
                        let a4433 = v1;
                        let a4434 = 16u8;
                        fixed_u8::bit_and(a4433, a4434)
                    };
                    let a4436 = 16u8;
                    if equal(a4435, a4436) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4437, a4438)?
            };
            nat_add(a4439, a4440)?
        };
        let a4464 = {
            let a4461 = {
                let a4449 = {
                    let a4443 = {
                        let a4441 = v1;
                        let a4442 = 8u8;
                        fixed_u8::bit_and(a4441, a4442)
                    };
                    let a4444 = 8u8;
                    if equal(a4443, a4444) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4450 = {
                    let a4447 = {
                        let a4445 = v1;
                        let a4446 = 4u8;
                        fixed_u8::bit_and(a4445, a4446)
                    };
                    let a4448 = 4u8;
                    if equal(a4447, a4448) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4449, a4450)?
            };
            let a4462 = {
                let a4459 = {
                    let a4453 = {
                        let a4451 = v1;
                        let a4452 = 2u8;
                        fixed_u8::bit_and(a4451, a4452)
                    };
                    let a4454 = 2u8;
                    if equal(a4453, a4454) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4460 = {
                    let a4457 = {
                        let a4455 = v1;
                        let a4456 = 1u8;
                        fixed_u8::bit_and(a4455, a4456)
                    };
                    let a4458 = 1u8;
                    if equal(a4457, a4458) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4459, a4460)?
            };
            nat_add(a4461, a4462)?
        };
        nat_add(a4463, a4464)?
    };
    let a4465 = 48u64;
    let a4466 = v2;
    if nat_le(a4465, a4466) {
        let a4467 = v2;
        let a4468 = 57u64;
        Ok(nat_le(a4467, a4468))
    } else {
        Ok(false)
    }
}

pub fn f629() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f630(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f551({
        let (_, h4469) = {
            let (h4470, _) = f669(Fn23::F670, {
                let a4472 = {
                    let a4471 = v0.clone();
                    length_list(a4471)
                };
                let a4473 = 1u64;
                nat_add(a4472, a4473)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4470
        };
        h4469
    }))
}

pub fn f631(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h4474, _) = v0;
        h4474
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h4475) = v0;
        h4475
    };
    let m4476 = v1;
    match m4476.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m4477 = v4;
            match m4477.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f671(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f632(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4478 = f459(v0, v1);
    let a4479 = f605(v0, v2);
    fixed_u32::bit_or(a4478, a4479)
}

pub fn f633(v0: u32) -> u32 {
    let a4482 = {
        let a4480 = f632(v0, 17u32, 15u32);
        let a4481 = f632(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4480, a4481)
    };
    let a4483 = f459(v0, 10u32);
    fixed_u32::bit_xor(a4482, a4483)
}

pub fn f634(v0: u32) -> u32 {
    let a4486 = {
        let a4484 = f632(v0, 7u32, 25u32);
        let a4485 = f632(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4484, a4485)
    };
    let a4487 = f459(v0, 3u32);
    fixed_u32::bit_xor(a4486, a4487)
}

pub fn f635() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x00]), List::cons(Bytes::lit(&[0x01]), List::cons(Bytes::lit(&[0x02]), List::cons(Bytes::lit(&[0x03]), List::cons(Bytes::lit(&[0x04]), List::cons(Bytes::lit(&[0x05]), List::cons(Bytes::lit(&[0x06]), List::cons(Bytes::lit(&[0x07]), List::cons(Bytes::lit(&[0x08]), List::cons(Bytes::lit(&[0x09]), List::cons(Bytes::lit(&[0x0a]), List::cons(Bytes::lit(&[0x0b]), List::cons(Bytes::lit(&[0x0c]), List::cons(Bytes::lit(&[0x0d]), List::cons(Bytes::lit(&[0x0e]), List::cons(Bytes::lit(&[0x0f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f636() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x10]), List::cons(Bytes::lit(&[0x11]), List::cons(Bytes::lit(&[0x12]), List::cons(Bytes::lit(&[0x13]), List::cons(Bytes::lit(&[0x14]), List::cons(Bytes::lit(&[0x15]), List::cons(Bytes::lit(&[0x16]), List::cons(Bytes::lit(&[0x17]), List::cons(Bytes::lit(&[0x18]), List::cons(Bytes::lit(&[0x19]), List::cons(Bytes::lit(&[0x1a]), List::cons(Bytes::lit(&[0x1b]), List::cons(Bytes::lit(&[0x1c]), List::cons(Bytes::lit(&[0x1d]), List::cons(Bytes::lit(&[0x1e]), List::cons(Bytes::lit(&[0x1f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f637() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x20]), List::cons(Bytes::lit(&[0x21]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x27]), List::cons(Bytes::lit(&[0x28]), List::cons(Bytes::lit(&[0x29]), List::cons(Bytes::lit(&[0x2a]), List::cons(Bytes::lit(&[0x2b]), List::cons(Bytes::lit(&[0x2c]), List::cons(Bytes::lit(&[0x2d]), List::cons(Bytes::lit(&[0x2e]), List::cons(Bytes::lit(&[0x2f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f638() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x30]), List::cons(Bytes::lit(&[0x31]), List::cons(Bytes::lit(&[0x32]), List::cons(Bytes::lit(&[0x33]), List::cons(Bytes::lit(&[0x34]), List::cons(Bytes::lit(&[0x35]), List::cons(Bytes::lit(&[0x36]), List::cons(Bytes::lit(&[0x37]), List::cons(Bytes::lit(&[0x38]), List::cons(Bytes::lit(&[0x39]), List::cons(Bytes::lit(&[0x3a]), List::cons(Bytes::lit(&[0x3b]), List::cons(Bytes::lit(&[0x3c]), List::cons(Bytes::lit(&[0x3d]), List::cons(Bytes::lit(&[0x3e]), List::cons(Bytes::lit(&[0x3f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f639() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x40]), List::cons(Bytes::lit(&[0x41]), List::cons(Bytes::lit(&[0x42]), List::cons(Bytes::lit(&[0x43]), List::cons(Bytes::lit(&[0x44]), List::cons(Bytes::lit(&[0x45]), List::cons(Bytes::lit(&[0x46]), List::cons(Bytes::lit(&[0x47]), List::cons(Bytes::lit(&[0x48]), List::cons(Bytes::lit(&[0x49]), List::cons(Bytes::lit(&[0x4a]), List::cons(Bytes::lit(&[0x4b]), List::cons(Bytes::lit(&[0x4c]), List::cons(Bytes::lit(&[0x4d]), List::cons(Bytes::lit(&[0x4e]), List::cons(Bytes::lit(&[0x4f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f640() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x50]), List::cons(Bytes::lit(&[0x51]), List::cons(Bytes::lit(&[0x52]), List::cons(Bytes::lit(&[0x53]), List::cons(Bytes::lit(&[0x54]), List::cons(Bytes::lit(&[0x55]), List::cons(Bytes::lit(&[0x56]), List::cons(Bytes::lit(&[0x57]), List::cons(Bytes::lit(&[0x58]), List::cons(Bytes::lit(&[0x59]), List::cons(Bytes::lit(&[0x5a]), List::cons(Bytes::lit(&[0x5b]), List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x5d]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f641() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x60]), List::cons(Bytes::lit(&[0x61]), List::cons(Bytes::lit(&[0x62]), List::cons(Bytes::lit(&[0x63]), List::cons(Bytes::lit(&[0x64]), List::cons(Bytes::lit(&[0x65]), List::cons(Bytes::lit(&[0x66]), List::cons(Bytes::lit(&[0x67]), List::cons(Bytes::lit(&[0x68]), List::cons(Bytes::lit(&[0x69]), List::cons(Bytes::lit(&[0x6a]), List::cons(Bytes::lit(&[0x6b]), List::cons(Bytes::lit(&[0x6c]), List::cons(Bytes::lit(&[0x6d]), List::cons(Bytes::lit(&[0x6e]), List::cons(Bytes::lit(&[0x6f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f642() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x70]), List::cons(Bytes::lit(&[0x71]), List::cons(Bytes::lit(&[0x72]), List::cons(Bytes::lit(&[0x73]), List::cons(Bytes::lit(&[0x74]), List::cons(Bytes::lit(&[0x75]), List::cons(Bytes::lit(&[0x76]), List::cons(Bytes::lit(&[0x77]), List::cons(Bytes::lit(&[0x78]), List::cons(Bytes::lit(&[0x79]), List::cons(Bytes::lit(&[0x7a]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7c]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x7f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f643() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x80]), List::cons(Bytes::lit(&[0x81]), List::cons(Bytes::lit(&[0x82]), List::cons(Bytes::lit(&[0x83]), List::cons(Bytes::lit(&[0x84]), List::cons(Bytes::lit(&[0x85]), List::cons(Bytes::lit(&[0x86]), List::cons(Bytes::lit(&[0x87]), List::cons(Bytes::lit(&[0x88]), List::cons(Bytes::lit(&[0x89]), List::cons(Bytes::lit(&[0x8a]), List::cons(Bytes::lit(&[0x8b]), List::cons(Bytes::lit(&[0x8c]), List::cons(Bytes::lit(&[0x8d]), List::cons(Bytes::lit(&[0x8e]), List::cons(Bytes::lit(&[0x8f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f644() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x90]), List::cons(Bytes::lit(&[0x91]), List::cons(Bytes::lit(&[0x92]), List::cons(Bytes::lit(&[0x93]), List::cons(Bytes::lit(&[0x94]), List::cons(Bytes::lit(&[0x95]), List::cons(Bytes::lit(&[0x96]), List::cons(Bytes::lit(&[0x97]), List::cons(Bytes::lit(&[0x98]), List::cons(Bytes::lit(&[0x99]), List::cons(Bytes::lit(&[0x9a]), List::cons(Bytes::lit(&[0x9b]), List::cons(Bytes::lit(&[0x9c]), List::cons(Bytes::lit(&[0x9d]), List::cons(Bytes::lit(&[0x9e]), List::cons(Bytes::lit(&[0x9f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f645() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xa0]), List::cons(Bytes::lit(&[0xa1]), List::cons(Bytes::lit(&[0xa2]), List::cons(Bytes::lit(&[0xa3]), List::cons(Bytes::lit(&[0xa4]), List::cons(Bytes::lit(&[0xa5]), List::cons(Bytes::lit(&[0xa6]), List::cons(Bytes::lit(&[0xa7]), List::cons(Bytes::lit(&[0xa8]), List::cons(Bytes::lit(&[0xa9]), List::cons(Bytes::lit(&[0xaa]), List::cons(Bytes::lit(&[0xab]), List::cons(Bytes::lit(&[0xac]), List::cons(Bytes::lit(&[0xad]), List::cons(Bytes::lit(&[0xae]), List::cons(Bytes::lit(&[0xaf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f646() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xb0]), List::cons(Bytes::lit(&[0xb1]), List::cons(Bytes::lit(&[0xb2]), List::cons(Bytes::lit(&[0xb3]), List::cons(Bytes::lit(&[0xb4]), List::cons(Bytes::lit(&[0xb5]), List::cons(Bytes::lit(&[0xb6]), List::cons(Bytes::lit(&[0xb7]), List::cons(Bytes::lit(&[0xb8]), List::cons(Bytes::lit(&[0xb9]), List::cons(Bytes::lit(&[0xba]), List::cons(Bytes::lit(&[0xbb]), List::cons(Bytes::lit(&[0xbc]), List::cons(Bytes::lit(&[0xbd]), List::cons(Bytes::lit(&[0xbe]), List::cons(Bytes::lit(&[0xbf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f647() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xc0]), List::cons(Bytes::lit(&[0xc1]), List::cons(Bytes::lit(&[0xc2]), List::cons(Bytes::lit(&[0xc3]), List::cons(Bytes::lit(&[0xc4]), List::cons(Bytes::lit(&[0xc5]), List::cons(Bytes::lit(&[0xc6]), List::cons(Bytes::lit(&[0xc7]), List::cons(Bytes::lit(&[0xc8]), List::cons(Bytes::lit(&[0xc9]), List::cons(Bytes::lit(&[0xca]), List::cons(Bytes::lit(&[0xcb]), List::cons(Bytes::lit(&[0xcc]), List::cons(Bytes::lit(&[0xcd]), List::cons(Bytes::lit(&[0xce]), List::cons(Bytes::lit(&[0xcf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f648() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xd0]), List::cons(Bytes::lit(&[0xd1]), List::cons(Bytes::lit(&[0xd2]), List::cons(Bytes::lit(&[0xd3]), List::cons(Bytes::lit(&[0xd4]), List::cons(Bytes::lit(&[0xd5]), List::cons(Bytes::lit(&[0xd6]), List::cons(Bytes::lit(&[0xd7]), List::cons(Bytes::lit(&[0xd8]), List::cons(Bytes::lit(&[0xd9]), List::cons(Bytes::lit(&[0xda]), List::cons(Bytes::lit(&[0xdb]), List::cons(Bytes::lit(&[0xdc]), List::cons(Bytes::lit(&[0xdd]), List::cons(Bytes::lit(&[0xde]), List::cons(Bytes::lit(&[0xdf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f649() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xe0]), List::cons(Bytes::lit(&[0xe1]), List::cons(Bytes::lit(&[0xe2]), List::cons(Bytes::lit(&[0xe3]), List::cons(Bytes::lit(&[0xe4]), List::cons(Bytes::lit(&[0xe5]), List::cons(Bytes::lit(&[0xe6]), List::cons(Bytes::lit(&[0xe7]), List::cons(Bytes::lit(&[0xe8]), List::cons(Bytes::lit(&[0xe9]), List::cons(Bytes::lit(&[0xea]), List::cons(Bytes::lit(&[0xeb]), List::cons(Bytes::lit(&[0xec]), List::cons(Bytes::lit(&[0xed]), List::cons(Bytes::lit(&[0xee]), List::cons(Bytes::lit(&[0xef]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f650() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xf0]), List::cons(Bytes::lit(&[0xf1]), List::cons(Bytes::lit(&[0xf2]), List::cons(Bytes::lit(&[0xf3]), List::cons(Bytes::lit(&[0xf4]), List::cons(Bytes::lit(&[0xf5]), List::cons(Bytes::lit(&[0xf6]), List::cons(Bytes::lit(&[0xf7]), List::cons(Bytes::lit(&[0xf8]), List::cons(Bytes::lit(&[0xf9]), List::cons(Bytes::lit(&[0xfa]), List::cons(Bytes::lit(&[0xfb]), List::cons(Bytes::lit(&[0xfc]), List::cons(Bytes::lit(&[0xfd]), List::cons(Bytes::lit(&[0xfe]), List::cons(Bytes::lit(&[0xff]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f651(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f484(v2) {
        true
    } else {
        f485(v2)
    } {
        let a4488 = v2;
        let a4489 = 117u64;
        if !nat_eq(a4488, a4489) {
            let a4490 = v2;
            let a4491 = 85u64;
            !nat_eq(a4490, a4491)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f486(v2) {
            let a4492 = 0u64;
            let a4493 = v1;
            nat_lt(a4492, a4493)
        } else {
            false
        }
    } {
        f35(v0, v1, 1u64)
    } else {
        let a4494 = v2;
        let a4495 = 65536u64;
        if nat_lt(a4494, a4495) {
            let a4496 = f8(Str::lit("u"));
            let a4497 = f652(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4496, a4497)
        } else {
            let a4498 = f8(Str::lit("U"));
            let a4499 = f652(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4498, a4499)
        }
    }
}

pub fn f652(p4516: u64, p4517: u64, p4518: u64, p4519: Bytes) -> Bytes {
    let m4515 = (p4516, (p4517, (p4518, p4519)));
    {
        let mut m4515 = m4515;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m4515;
                let m4500 = v0;
                if m4500 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m4500 - 1;
                    let a4501 = v1;
                    let a4502 = 0u64;
                    if if nat_eq(a4501, a4502) {
                        let a4503 = v2;
                        let a4504 = 0u64;
                        nat_eq(a4503, a4504)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a4505 = v1;
                            let a4506 = 16u64;
                            let a4507 = 0u64;
                            nat_quot(a4505, a4506, a4507)
                        }, ({
                            let a4508 = v2;
                            let a4509 = 1u64;
                            nat_sub(a4508, a4509)
                        }, {
                            let a4513 = f672({
                                let a4510 = v1;
                                let a4511 = 16u64;
                                let a4512 = 0u64;
                                nat_rem(a4510, a4511, a4512)
                            });
                            let a4514 = v3;
                            append_bytes(a4513, a4514)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m4515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f653(v0: List<(Bytes, Adt20)>, v1: List<Adt20>) -> List<Bytes> {
    let m4520 = v0;
    match m4520.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f673(v1.clone(), {
                let (h4521, _) = v2.clone();
                h4521
            }) {
                List::cons({
                    let (h4522, _) = v2;
                    h4522
                }, f653(v3, v1))
            } else {
                f653(v3, v1)
            }
        }
    }
}

pub fn f654(v0: bool, v1: Bytes, v2: List<Adt20>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt26>), Bytes>> {
    let m4523 = v2;
    match m4523.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt26>), Bytes>((List::<Bytes>::nil(), List::<Adt26>::nil())))
        }
        Some((v4, v5)) => {
            let m4524 = f674(v4.clone())?;
            match m4524 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m4525 = f576(v0, v6.clone())?;
                    match m4525 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f41(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4528 = f675(v4, {
                                    let a4526 = v1.clone();
                                    let a4527 = f300(v6)?;
                                    append_bytes(a4526, a4527)
                                })?;
                                match m4528 {
                                    Ok(v9) => {
                                        let m4529 = f654(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m4529 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt26>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h4530, _) = v9;
                                                    h4530
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4531, _) = v11;
                                                    h4531
                                                }), f492({
                                                    let (_, h4532) = v9;
                                                    h4532
                                                }, {
                                                    let (_, h4533) = v11;
                                                    h4533
                                                }))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(v8))
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

pub fn f655(v0: List<Adt20>) -> R<List<Bytes>> {
    let m4534 = v0;
    match m4534.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m4535 = f674(v1)?;
            match m4535 {
                None => {
                    f655(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f655(v2)?))
                }
            }
        }
    }
}

pub fn f656(v0: Bytes, v1: List<Adt20>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt26>), Bytes>> {
    f676(f479(f677(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f657(v0: Adt20, v1: Bytes) -> R<bool> {
    let m4536 = f304(Some(v0));
    match m4536 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m4537 = f612(v2)?;
            match m4537 {
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

pub fn f658(v0: bool, v1: Bytes, v2: List<(Bytes, Adt20)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt26>), Bytes>> {
    let m4538 = v2;
    match m4538.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt26>), Bytes>((List::<Bytes>::nil(), List::<Adt26>::nil())))
        }
        Some((v5, v6)) => {
            let m4540 = f576(v0, {
                let (h4539, _) = v5.clone();
                h4539
            })?;
            match m4540 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h4541, _) = v5;
                        h4541
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f41(v7.clone(), f535(v0, f659(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m4546 = f619({
                            let (_, h4542) = v5.clone();
                            h4542
                        }, {
                            let a4544 = v1.clone();
                            let a4545 = f300({
                                let (h4543, _) = v5.clone();
                                h4543
                            })?;
                            append_bytes(a4544, a4545)
                        })?;
                        match m4546 {
                            Ok(v9) => {
                                let v10: Bytes = if f41({
                                    let (h4547, _) = v5;
                                    h4547
                                }, v3.clone()) {
                                    let (h4548, _) = v9;
                                    h4548
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h4549, _) = v9;
                                        h4549
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m4550 = f658(v0, v1, v6, v3, v4)?;
                                match m4550 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt26>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h4551, _) = v12;
                                            h4551
                                        }), f492({
                                            let (_, h4552) = v9;
                                            h4552
                                        }, {
                                            let (_, h4553) = v12;
                                            h4553
                                        }))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f659(v0: List<(Bytes, Adt20)>) -> List<Bytes> {
    let m4554 = v0;
    match m4554.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h4555, _) = v1;
                h4555
            }, f659(v2))
        }
    }
}

pub fn f660(v0: Adt20) -> Option<Adt20> {
    let m4556 = f197(f192(v0, f8(Str::lit("oneOf"))));
    match m4556.uncons() {
        None => {
            None::<Adt20>
        }
        Some((v1, v2)) => {
            let m4557 = v2;
            match m4557.uncons() {
                None => {
                    None::<Adt20>
                }
                Some((v3, v4)) => {
                    let m4558 = v4;
                    match m4558.uncons() {
                        None => {
                            if f678(v3.clone()) {
                                Some(v1)
                            } else {
                                if f678(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt20>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt20>
                        }
                    }
                }
            }
        }
    }
}

pub fn f661(v0: Bytes, v1: Result<(Bytes, List<Adt26>), Bytes>) -> Result<(Bytes, List<Adt26>), Bytes> {
    let m4559 = v1;
    match m4559 {
        Ok(v3) => {
            f615(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h4560, _) = v3;
                h4560
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h4561) = v3;
                h4561
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt26>), Bytes>(v2)
        }
    }
}

pub fn f662(v0: Adt20, v1: Bytes) -> R<Result<(Bytes, List<Adt26>), Bytes>> {
    let m4562 = f660(v0.clone());
    match m4562 {
        None => {
            if f28(f195(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4563 = f192(v0, f8(Str::lit("items")));
                match m4563 {
                    None => {
                        Ok(f661(f8(Str::lit("List")), f663()))
                    }
                    Some(v3) => {
                        Ok(f661(f8(Str::lit("List")), f675(v3, {
                            let a4564 = v1;
                            let a4565 = f8(Str::lit("Item"));
                            append_bytes(a4564, a4565)
                        })?))
                    }
                }
            } else {
                f675(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f661(f8(Str::lit("Option")), f675(v2, v1)?))
        }
    }
}

pub fn f663() -> Result<(Bytes, List<Adt26>), Bytes> {
    Ok::<(Bytes, List<Adt26>), Bytes>((f540(), List::cons(Adt26::C2, List::<Adt26>::nil())))
}

pub fn f664(p4569: List<(Bytes, Adt20)>, v1: List<Bytes>) -> bool {
    let m4568 = p4569;
    {
        let mut m4568 = m4568;
        loop {
            let step = {
                let v0 = m4568;
                let m4566 = v0;
                match m4566.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt20)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h4567, _) = v2;
                            h4567
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt20)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt20)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f665(p4573: List<Adt20>, v1: Bytes, p4574: List<Bytes>) -> bool {
    let m4572 = (p4573, p4574);
    {
        let mut m4572 = m4572;
        loop {
            let step = {
                let (v0, v2) = m4572;
                let m4570 = v0;
                match m4570.uncons() {
                    None => {
                        Err::<(List<Adt20>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m4571 = f679(v3, v1.clone());
                        match m4571 {
                            None => {
                                Err::<(List<Adt20>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f41(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt20>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt20>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f666(p4577: List<Adt20>, p4578: List<Bytes>) -> bool {
    let m4576 = (p4577, p4578);
    {
        let mut m4576 = m4576;
        loop {
            let step = {
                let (v0, v1) = m4576;
                let m4575 = v0;
                match m4575.uncons() {
                    None => {
                        Err::<(List<Adt20>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f195(v2, f8(Str::lit("type")));
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
                                    Ok::<(List<Adt20>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt20>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt20>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt20>, List<Bytes>), bool>(false)
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

pub fn f667(v0: Fn22, p4583: u64, p4584: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4582 = (p4583, p4584);
    {
        let mut m4582 = m4582;
        loop {
            let step = {
                let (v1, v2) = m4582;
                let m4579 = v1;
                if m4579 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4579 - 1;
                    let m4581 = {
                        let c4580: Fn22 = v0.clone();
                        c4580.apply(v2.clone())?
                    };
                    match m4581 {
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
                Ok(next) => { m4582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f668(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f680(v0, v1)
}

pub fn f669(v0: Fn23, p4589: u64, p4590: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4588 = (p4589, p4590);
    {
        let mut m4588 = m4588;
        loop {
            let step = {
                let (v1, v2) = m4588;
                let m4585 = v1;
                if m4585 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4585 - 1;
                    let m4587 = {
                        let c4586: Fn23 = v0.clone();
                        c4586.apply(v2.clone())?
                    };
                    match m4587 {
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
                Ok(next) => { m4588 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f670(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f681(v0)
}

pub fn f671(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a4595 = {
            let a4593 = {
                let a4591 = v0.clone();
                length_list(a4591)
            };
            let a4594 = {
                let a4592 = v1.clone();
                length_list(a4592)
            };
            nat_add(a4593, a4594)?
        };
        let a4596 = 1u64;
        nat_add(a4595, a4596)?
    };
    Ok(f450({
        let (_, h4597) = {
            let (_, h4598) = {
                let (h4599, _) = f682(Fn24::F683, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h4599
            };
            h4598
        };
        h4597
    }))
}

pub fn f672(v0: u64) -> Bytes {
    f35({
        let a4600 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4600)
    }, v0, 1u64)
}

pub fn f673(p4604: List<Adt20>, v1: Bytes) -> bool {
    let m4603 = p4604;
    {
        let mut m4603 = m4603;
        loop {
            let step = {
                let v0 = m4603;
                let m4601 = v0;
                match m4601.uncons() {
                    None => {
                        Err::<List<Adt20>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m4602 = f679(v2, v1.clone());
                        if match m4602 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt20>, bool>(v3)
                        } else {
                            Err::<List<Adt20>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4603 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f674(v0: Adt20) -> R<Option<Bytes>> {
    let m4605 = f304(f192(v0.clone(), f8(Str::lit("$ref"))));
    match m4605 {
        None => {
            Ok(f304(f192(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f612(v1)
        }
    }
}

pub fn f675(v0: Adt20, v1: Bytes) -> R<Result<(Bytes, List<Adt26>), Bytes>> {
    f684(f585(v0), v1)
}

pub fn f676(v0: bool, v1: Bytes, v2: List<Adt20>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt26>), Bytes>> {
    let m4606 = v2;
    match m4606.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt26>), Bytes>((List::<Bytes>::nil(), List::<Adt26>::nil())))
        }
        Some((v5, v6)) => {
            let m4607 = f679(v5.clone(), v3.clone());
            match m4607 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m4608 = f576(v0, v7.clone())?;
                    match m4608 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f41(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4611 = f618({
                                    let a4609 = v1.clone();
                                    let a4610 = f300(v7)?;
                                    append_bytes(a4609, a4610)
                                }, f539(f488(f192(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f196(f197(f192(v5, f8(Str::lit("required"))))), true)?;
                                match m4611 {
                                    Ok(v10) => {
                                        let m4612 = f676(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m4612 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt26>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f265({
                                                    let (h4613, _) = v10;
                                                    h4613
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4614, _) = v12;
                                                    h4614
                                                }), f492({
                                                    let (_, h4615) = v10;
                                                    h4615
                                                }, {
                                                    let (_, h4616) = v12;
                                                    h4616
                                                }))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(List<Bytes>, List<Adt26>), Bytes>(v9))
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

pub fn f677(v0: List<Adt20>, v1: Bytes) -> List<Bytes> {
    let m4617 = v0;
    match m4617.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4618 = f679(v2, v1.clone());
            match m4618 {
                None => {
                    f677(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f677(v3, v1))
                }
            }
        }
    }
}

pub fn f678(v0: Adt20) -> bool {
    f28(f195(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f679(v0: Adt20, v1: Bytes) -> Option<Bytes> {
    if f41(v1.clone(), f196(f197(f192(v0.clone(), f8(Str::lit("required")))))) {
        let m4619 = f192(v0, f8(Str::lit("properties")));
        match m4619 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m4620 = f192(v2, v1);
                match m4620 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f304(f192(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f680(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4621, _) = v1;
        h4621
    };
    let m4623 = {
        let (_, h4622) = v1;
        h4622
    };
    match m4623 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4626 = {
                let a4624 = v0.clone();
                let a4625 = v2;
                index_bytes(a4624, a4625)
            };
            match m4626 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4631 = {
                        let a4629 = v0;
                        let a4630 = {
                            let a4627 = v2;
                            let a4628 = 1u64;
                            nat_add(a4627, a4628)?
                        };
                        index_bytes(a4629, a4630)
                    };
                    match m4631 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f685(v4)? {
                                f685(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4632 = v2;
                                    let a4633 = 2u64;
                                    nat_add(a4632, a4633)?
                                }, Some({
                                    let a4638 = v3;
                                    let a4639 = f389(f686(), {
                                        let a4636 = {
                                            let a4634 = f687(v4)?;
                                            let a4635 = 16u64;
                                            nat_mul(a4634, a4635)?
                                        };
                                        let a4637 = f687(v5)?;
                                        nat_add(a4636, a4637)?
                                    });
                                    append_bytes(a4638, a4639)
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

pub fn f681(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4640, _) = v0;
        h4640
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4641) = v0;
        h4641
    };
    let m4642 = v1;
    match m4642.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4643 = v4;
            match m4643.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f688(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f682(v0: Fn24, p4648: u64, p4649: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4647 = (p4648, p4649);
    {
        let mut m4647 = m4647;
        loop {
            let step = {
                let (v1, v2) = m4647;
                let m4644 = v1;
                if m4644 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4644 - 1;
                    let m4646 = {
                        let c4645: Fn24 = v0.clone();
                        c4645.apply(v2.clone())
                    };
                    match m4646 {
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
                Ok(next) => { m4647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f683(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f689(v0)
}

pub fn f684(v0: Adt20, v1: Bytes) -> R<Result<(Bytes, List<Adt26>), Bytes>> {
    let m4650 = v0.clone();
    match m4650 {
        Adt20::C0(v2) => {
            if v2 {
                Ok(f663())
            } else {
                Ok(f614(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt20::C1(_, _) => {
            Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt20::C2(_, _, _) => {
            Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt20::C3(_) => {
            Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt20::C4(_) => {
            Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt20::C5(_) => {
            let m4651 = f690(f537(v0.clone()));
            match m4651 {
                None => {
                    let m4652 = f304(f192(v0.clone(), f8(Str::lit("$ref"))));
                    match m4652 {
                        None => {
                            let m4654 = f192(v0.clone(), f8(Str::lit("allOf")));
                            match m4654 {
                                None => {
                                    let m4659 = f192(v0.clone(), f8(Str::lit("enum")));
                                    match m4659 {
                                        None => {
                                            let m4660 = f192(v0.clone(), f8(Str::lit("const")));
                                            match m4660 {
                                                None => {
                                                    let m4662 = f192(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4662 {
                                                        None => {
                                                            let v30: Bytes = f195(v0.clone(), f8(Str::lit("type")));
                                                            if if f28(v30.clone(), Bytes::lit(&[])) {
                                                                !f691(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f663())
                                                            } else {
                                                                if f28(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f615(f8(Str::lit("Bytes")), List::<Adt26>::nil()))
                                                                } else {
                                                                    if f28(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f615(f692(v0), List::<Adt26>::nil()))
                                                                    } else {
                                                                        if f28(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f615(f574(), List::<Adt26>::nil()))
                                                                        } else {
                                                                            if f28(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f615(f8(Str::lit("Bool")), List::<Adt26>::nil()))
                                                                            } else {
                                                                                if f28(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m4663 = f488(f192(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4663.uncons() {
                                                                                        None => {
                                                                                            let m4664 = f693(v0);
                                                                                            match m4664 {
                                                                                                None => {
                                                                                                    Ok(f661(f8(Str::lit("List")), f694(f663())))
                                                                                                }
                                                                                                Some(v33) => {
                                                                                                    Ok(f661(f8(Str::lit("List")), f694(f695(v33, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f615(v1.clone(), List::cons(Adt26::C0(v1, v0), List::<Adt26>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f614(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f615(v1.clone(), List::cons(Adt26::C0(v1, v0), List::<Adt26>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m4661 = f304(Some(v27));
                                                    match m4661 {
                                                        None => {
                                                            Ok(f614(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f615(f8(Str::lit("Bytes")), List::<Adt26>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f615(v1.clone(), List::cons(Adt26::C0(v1, v0), List::<Adt26>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m4655 = f305(v19);
                                    match m4655.uncons() {
                                        None => {
                                            Ok(f614(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m4656 = v21;
                                            match m4656.uncons() {
                                                None => {
                                                    let m4657 = f304(f192(v20, f8(Str::lit("$ref"))));
                                                    match m4657 {
                                                        None => {
                                                            Ok(f614(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m4658 = f612(v24)?;
                                                            match m4658 {
                                                                None => {
                                                                    Ok(f614(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f615(v25, List::<Adt26>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f614(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m4653 = f612(v17)?;
                            match m4653 {
                                None => {
                                    Ok(f614(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f615(v18, List::<Adt26>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f614(if f28(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt20::C6 => {
            Ok(f663())
        }
        Adt20::C7(_, _) => {
            Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt20::C8(_, _, _) => {
            Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f685(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4709 = {
            let a4685 = {
                let a4673 = {
                    let a4667 = {
                        let a4665 = v1;
                        let a4666 = 128u8;
                        fixed_u8::bit_and(a4665, a4666)
                    };
                    let a4668 = 128u8;
                    if equal(a4667, a4668) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4674 = {
                    let a4671 = {
                        let a4669 = v1;
                        let a4670 = 64u8;
                        fixed_u8::bit_and(a4669, a4670)
                    };
                    let a4672 = 64u8;
                    if equal(a4671, a4672) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4673, a4674)?
            };
            let a4686 = {
                let a4683 = {
                    let a4677 = {
                        let a4675 = v1;
                        let a4676 = 32u8;
                        fixed_u8::bit_and(a4675, a4676)
                    };
                    let a4678 = 32u8;
                    if equal(a4677, a4678) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4684 = {
                    let a4681 = {
                        let a4679 = v1;
                        let a4680 = 16u8;
                        fixed_u8::bit_and(a4679, a4680)
                    };
                    let a4682 = 16u8;
                    if equal(a4681, a4682) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4683, a4684)?
            };
            nat_add(a4685, a4686)?
        };
        let a4710 = {
            let a4707 = {
                let a4695 = {
                    let a4689 = {
                        let a4687 = v1;
                        let a4688 = 8u8;
                        fixed_u8::bit_and(a4687, a4688)
                    };
                    let a4690 = 8u8;
                    if equal(a4689, a4690) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4696 = {
                    let a4693 = {
                        let a4691 = v1;
                        let a4692 = 4u8;
                        fixed_u8::bit_and(a4691, a4692)
                    };
                    let a4694 = 4u8;
                    if equal(a4693, a4694) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4695, a4696)?
            };
            let a4708 = {
                let a4705 = {
                    let a4699 = {
                        let a4697 = v1;
                        let a4698 = 2u8;
                        fixed_u8::bit_and(a4697, a4698)
                    };
                    let a4700 = 2u8;
                    if equal(a4699, a4700) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4706 = {
                    let a4703 = {
                        let a4701 = v1;
                        let a4702 = 1u8;
                        fixed_u8::bit_and(a4701, a4702)
                    };
                    let a4704 = 1u8;
                    if equal(a4703, a4704) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4705, a4706)?
            };
            nat_add(a4707, a4708)?
        };
        nat_add(a4709, a4710)?
    };
    let a4711 = 48u64;
    let a4712 = v2;
    if if nat_le(a4711, a4712) {
        let a4713 = v2;
        let a4714 = 57u64;
        nat_le(a4713, a4714)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4715 = 97u64;
        let a4716 = v2;
        if nat_le(a4715, a4716) {
            let a4717 = v2;
            let a4718 = 102u64;
            Ok(nat_le(a4717, a4718))
        } else {
            Ok(false)
        }
    }
}

pub fn f686() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f687(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4763 = {
            let a4739 = {
                let a4727 = {
                    let a4721 = {
                        let a4719 = v1;
                        let a4720 = 128u8;
                        fixed_u8::bit_and(a4719, a4720)
                    };
                    let a4722 = 128u8;
                    if equal(a4721, a4722) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4728 = {
                    let a4725 = {
                        let a4723 = v1;
                        let a4724 = 64u8;
                        fixed_u8::bit_and(a4723, a4724)
                    };
                    let a4726 = 64u8;
                    if equal(a4725, a4726) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4727, a4728)?
            };
            let a4740 = {
                let a4737 = {
                    let a4731 = {
                        let a4729 = v1;
                        let a4730 = 32u8;
                        fixed_u8::bit_and(a4729, a4730)
                    };
                    let a4732 = 32u8;
                    if equal(a4731, a4732) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4738 = {
                    let a4735 = {
                        let a4733 = v1;
                        let a4734 = 16u8;
                        fixed_u8::bit_and(a4733, a4734)
                    };
                    let a4736 = 16u8;
                    if equal(a4735, a4736) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4737, a4738)?
            };
            nat_add(a4739, a4740)?
        };
        let a4764 = {
            let a4761 = {
                let a4749 = {
                    let a4743 = {
                        let a4741 = v1;
                        let a4742 = 8u8;
                        fixed_u8::bit_and(a4741, a4742)
                    };
                    let a4744 = 8u8;
                    if equal(a4743, a4744) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4750 = {
                    let a4747 = {
                        let a4745 = v1;
                        let a4746 = 4u8;
                        fixed_u8::bit_and(a4745, a4746)
                    };
                    let a4748 = 4u8;
                    if equal(a4747, a4748) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4749, a4750)?
            };
            let a4762 = {
                let a4759 = {
                    let a4753 = {
                        let a4751 = v1;
                        let a4752 = 2u8;
                        fixed_u8::bit_and(a4751, a4752)
                    };
                    let a4754 = 2u8;
                    if equal(a4753, a4754) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4760 = {
                    let a4757 = {
                        let a4755 = v1;
                        let a4756 = 1u8;
                        fixed_u8::bit_and(a4755, a4756)
                    };
                    let a4758 = 1u8;
                    if equal(a4757, a4758) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4759, a4760)?
            };
            nat_add(a4761, a4762)?
        };
        nat_add(a4763, a4764)?
    };
    let a4765 = v2;
    let a4766 = 57u64;
    if nat_le(a4765, a4766) {
        let a4767 = v2;
        let a4768 = 48u64;
        Ok(nat_sub(a4767, a4768))
    } else {
        let a4769 = v2;
        let a4770 = 87u64;
        Ok(nat_sub(a4769, a4770))
    }
}

pub fn f688(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4775 = {
            let a4773 = {
                let a4771 = v0.clone();
                length_list(a4771)
            };
            let a4774 = {
                let a4772 = v1.clone();
                length_list(a4772)
            };
            nat_add(a4773, a4774)?
        };
        let a4776 = 1u64;
        nat_add(a4775, a4776)?
    };
    Ok(f696({
        let (_, h4777) = {
            let (_, h4778) = {
                let (h4779, _) = f697(Fn25::F698, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4779
            };
            h4778
        };
        h4777
    }))
}

pub fn f689(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4780, _) = v0.clone();
        h4780
    };
    let v2: List<(Bytes, ())> = {
        let (h4781, _) = {
            let (_, h4782) = v0.clone();
            h4782
        };
        h4781
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4783) = {
            let (_, h4784) = v0;
            h4784
        };
        h4783
    };
    let m4785 = v1.clone();
    match m4785.uncons() {
        None => {
            let m4786 = v2;
            match m4786.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4787 = v2.clone();
            match m4787.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f699({
                        let (h4788, _) = v8.clone();
                        h4788
                    }, {
                        let (h4789, _) = v6.clone();
                        h4789
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

pub fn f690(p4794: List<(Bytes, Adt20)>) -> Option<Bytes> {
    let m4793 = p4794;
    {
        let mut m4793 = m4793;
        loop {
            let step = {
                let v0 = m4793;
                let m4790 = v0;
                match m4790.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt20)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h4791, _) = v1.clone();
                            h4791
                        }, f700()) {
                            Err::<List<(Bytes, Adt20)>, Option<Bytes>>(Some({
                                let (h4792, _) = v1;
                                h4792
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt20)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4793 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f691(v0: Adt20) -> bool {
    if f622(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f622(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f622(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f622(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f622(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f622(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f692(v0: Adt20) -> Bytes {
    let m4795 = f701(f192(v0, f8(Str::lit("minimum"))));
    match m4795 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f693(v0: Adt20) -> Option<Adt20> {
    let m4796 = f192(v0.clone(), f8(Str::lit("additionalProperties")));
    match m4796 {
        None => {
            let m4797 = f488(f192(v0, f8(Str::lit("patternProperties"))));
            match m4797.uncons() {
                None => {
                    None::<Adt20>
                }
                Some((v2, v3)) => {
                    let m4798 = v3;
                    match m4798.uncons() {
                        None => {
                            Some({
                                let (_, h4799) = v2;
                                h4799
                            })
                        }
                        Some((_, _)) => {
                            None::<Adt20>
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

pub fn f694(v0: Result<(Bytes, List<Adt26>), Bytes>) -> Result<(Bytes, List<Adt26>), Bytes> {
    let m4800 = v0;
    match m4800 {
        Ok(v2) => {
            f615(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h4801, _) = v2;
                h4801
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h4802) = v2;
                h4802
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt26>), Bytes>(v1)
        }
    }
}

pub fn f695(v0: Adt20, v1: Bytes) -> R<Result<(Bytes, List<Adt26>), Bytes>> {
    f702(16u64, v0, v1)
}

pub fn f696(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f426(Fn7::F703, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f697(v0: Fn25, p4807: u64, p4808: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4806 = (p4807, p4808);
    {
        let mut m4806 = m4806;
        loop {
            let step = {
                let (v1, v2) = m4806;
                let m4803 = v1;
                if m4803 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4803 - 1;
                    let m4805 = {
                        let c4804: Fn25 = v0.clone();
                        c4804.apply(v2.clone())
                    };
                    match m4805 {
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
                Ok(next) => { m4806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f698(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f704(v0)
}

pub fn f699(v0: Bytes, v1: Bytes) -> bool {
    let a4813 = {
        let a4809 = v0;
        let a4810 = v1;
        compare_bytes(a4809, a4810)
    };
    let a4814 = {
        let a4811 = Bytes::lit(&[]);
        let a4812 = Bytes::lit(&[0x01]);
        compare_bytes(a4811, a4812)
    };
    equal(a4813, a4814)
}

pub fn f700() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f701(v0: Option<Adt20>) -> Option<u64> {
    let m4815 = v0;
    match m4815 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m4816 = v1;
            match m4816 {
                Adt20::C0(_) => {
                    None::<u64>
                }
                Adt20::C1(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt20::C2(_, _, _) => {
                    None::<u64>
                }
                Adt20::C3(_) => {
                    None::<u64>
                }
                Adt20::C4(_) => {
                    None::<u64>
                }
                Adt20::C5(_) => {
                    None::<u64>
                }
                Adt20::C6 => {
                    None::<u64>
                }
                Adt20::C7(_, _) => {
                    None::<u64>
                }
                Adt20::C8(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f702(v0: u64, v1: Adt20, v2: Bytes) -> R<Result<(Bytes, List<Adt26>), Bytes>> {
    let m4817 = v0;
    if m4817 == 0 {
        Ok(f614(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4817 - 1;
        let m4818 = v1.clone();
        match m4818 {
            Adt20::C0(v4) => {
                if v4 {
                    Ok(f663())
                } else {
                    Ok(f614(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt20::C1(_, _) => {
                Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt20::C2(_, _, _) => {
                Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt20::C3(_) => {
                Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt20::C4(_) => {
                Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt20::C5(_) => {
                let m4819 = f304(f192(v1.clone(), f8(Str::lit("$ref"))));
                match m4819 {
                    None => {
                        if if f622(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f622(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f622(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f615(v2.clone(), List::cons(Adt26::C0(v2, v1), List::<Adt26>::nil())))
                        } else {
                            let v20: Bytes = f195(v1.clone(), f8(Str::lit("type")));
                            if if f28(v20.clone(), Bytes::lit(&[])) {
                                !f691(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f663())
                            } else {
                                if f28(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f615(f8(Str::lit("Bytes")), List::<Adt26>::nil()))
                                } else {
                                    if f28(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f615(f692(v1), List::<Adt26>::nil()))
                                    } else {
                                        if f28(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f615(f574(), List::<Adt26>::nil()))
                                        } else {
                                            if f28(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f615(f8(Str::lit("Bool")), List::<Adt26>::nil()))
                                            } else {
                                                if f28(v20.clone(), f8(Str::lit("array"))) {
                                                    let m4821 = f192(v1, f8(Str::lit("items")));
                                                    match m4821 {
                                                        None => {
                                                            Ok(f661(f8(Str::lit("List")), f663()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f661(f8(Str::lit("List")), f702(v3, v21, {
                                                                let a4822 = v2;
                                                                let a4823 = f8(Str::lit("Item"));
                                                                append_bytes(a4822, a4823)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f28(v20.clone(), f8(Str::lit("object"))) {
                                                        let m4824 = f693(v1);
                                                        match m4824 {
                                                            None => {
                                                                Ok(f661(f8(Str::lit("List")), f694(f663())))
                                                            }
                                                            Some(v22) => {
                                                                Ok(f661(f8(Str::lit("List")), f694(f702(v3, v22, {
                                                                    let a4825 = v2;
                                                                    let a4826 = f8(Str::lit("Value"));
                                                                    append_bytes(a4825, a4826)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f614(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m4820 = f612(v18)?;
                        match m4820 {
                            None => {
                                Ok(f614(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f615(v19, List::<Adt26>::nil()))
                            }
                        }
                    }
                }
            }
            Adt20::C6 => {
                Ok(f663())
            }
            Adt20::C7(_, _) => {
                Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt20::C8(_, _, _) => {
                Ok(f614(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f703(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f704(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4827, _) = v0.clone();
        h4827
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4828, _) = {
            let (_, h4829) = v0.clone();
            h4829
        };
        h4828
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4830) = {
            let (_, h4831) = v0;
            h4831
        };
        h4830
    };
    let m4832 = v1.clone();
    match m4832.uncons() {
        None => {
            let m4833 = v2;
            match m4833.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4834 = v2.clone();
            match m4834.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f699({
                        let (h4835, _) = v8.clone();
                        h4835
                    }, {
                        let (h4836, _) = v6.clone();
                        h4836
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
