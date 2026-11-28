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
    C0,
    C1,
    C2,
    C3,
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
    C0(Adt15, List<Adt2>, List<Adt2>, List<Adt2>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(bool, bool, Bytes, List<Bytes>, List<List<Bytes>>),
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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt24, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F78(u8, Bytes),
    F191(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F78(k0, k1) => f78(*k0, k1.clone(), p0),
            Fn0::F191(k0) => f191(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F122,
    F244,
    F362,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F122 => f122(p0, p1),
            Fn1::F244 => f244(p0, p1),
            Fn1::F362 => f362(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F214(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F214(k0) => f214(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F331(Bytes),
    F332(Bytes),
    F435(Bytes),
    F436(u64, Bytes),
    F443(Bytes),
    F451(Bytes, u64, Bytes, u64),
    F515(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F331(k0) => f331(k0.clone(), p0),
            Fn3::F332(k0) => f332(k0.clone(), p0),
            Fn3::F435(k0) => Ok(f435(k0.clone(), p0)),
            Fn3::F436(k0, k1) => f436(*k0, k1.clone(), p0),
            Fn3::F443(k0) => f443(k0.clone(), p0),
            Fn3::F451(k0, k1, k2, k3) => f451(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F515(k0) => f515(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F335(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F335(k0) => f335(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F427,
    F622,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F427 => f427(p0, p1),
            Fn5::F622 => f622(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F430,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F430 => f430(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F462,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F462 => f462(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F465,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F465 => f465(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F467,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F467 => f467(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F470,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F470 => f470(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F495(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt23)) -> R<(u64, Adt23)> {
        match self {
            Fn11::F495(k0, k1, k2) => f495(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F526,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F526 => f526(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F529,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F529 => f529(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F551(u64, Bytes),
    F562(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn14::F551(k0, k1) => Ok(f551(*k0, k1.clone(), p0)),
            Fn14::F562(k0) => f562(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F559,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F559 => f559(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F561,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F561 => f561(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F565(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F565(k0) => f565(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F574,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn18::F574 => f574(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F581,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F581 => f581(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F586,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F586 => f586(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F605,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F605 => f605(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F608,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F608 => f608(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F615,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F615 => f615(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F620,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F620 => f620(p0),
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
    let m127 = v0.clone();
    match m127.uncons() {
        None => {
            f31(v0, v1)
        }
        Some((v2, _)) => {
            if f32(v2.clone(), f8(Str::lit("stage-engine"))) {
                f33(f34(v1)?)
            } else {
                if f32(v2, f8(Str::lit("publish"))) {
                    Ok(f35(f31(v0, v1)?))
                } else {
                    f31(v0, v1)
                }
            }
        }
    }
}

pub fn f21(v0: Adt0, v1: List<Bytes>) -> Adt0 {
    Adt0::C0(v1, match v0.clone() {
        Adt0::C0(_, h128, _, _, _, _, _, _, _, _, _) => {
            h128
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h129, _, _, _, _, _, _, _, _) => {
            h129
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h130, _, _, _, _, _, _, _) => {
            h130
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h131, _, _, _, _, _, _) => {
            h131
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h132, _, _, _, _, _) => {
            h132
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h133, _, _, _, _) => {
            h133
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h134, _, _, _) => {
            h134
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h135, _, _) => {
            h135
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h136, _) => {
            h136
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h137) => {
            h137
        }
    })
}

pub fn f22() -> Adt1 {
    Adt1::C0(f36(64u64), List::<(Bytes, List<Bytes>)>::nil(), List::<Bytes>::nil(), f37(64u64))
}

pub fn f23(v0: Bytes, v1: u64, v2: u64) -> R<List<Str>> {
    let m138 = v2;
    if m138 == 0 {
        Ok(List::<Str>::nil())
    } else {
        let v3 = m138 - 1;
        let m141 = {
            let a139 = v0.clone();
            let a140 = v1;
            index_bytes(a139, a140)
        };
        match m141 {
            None => {
                Ok(List::<Str>::nil())
            }
            Some(v4) => {
                Ok(List::cons(f38(v4)?, f23(v0, {
                    let a142 = v1;
                    let a143 = 1u64;
                    nat_add(a142, a143)?
                }, v3)?))
            }
        }
    }
}

pub fn f24(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h144, _, _, _, _, _, _, _, _, _, _) => {
            h144
        }
    }, match v0.clone() {
        Adt0::C0(_, h145, _, _, _, _, _, _, _, _, _) => {
            h145
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h146, _, _, _, _, _, _, _, _) => {
            h146
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h147, _, _, _, _, _) => {
            h147
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h148, _, _, _, _) => {
            h148
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h149, _, _, _) => {
            h149
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h150, _, _) => {
            h150
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h151, _) => {
            h151
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h152) => {
            h152
        }
    })
}

pub fn f25(v0: Adt1, v1: Bytes, v2: Option<Bytes>) -> R<Adt1> {
    Ok(Adt1::C0(f39(match v0.clone() {
        Adt1::C0(h153, _, _, _) => {
            h153
        }
    }, f40(v1.clone()), (v1.clone(), v2.clone())), match v0.clone() {
        Adt1::C0(_, h154, _, _) => {
            h154
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h155, _) => {
            h155
        }
    }, if f41(v1.clone(), f8(Str::lit(".json"))) {
        f42(match v0 {
            Adt1::C0(_, _, _, h156) => {
                h156
            }
        }, f40(v1.clone()), (v1, f43({
            let m157 = v2;
            match m157 {
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
            Adt1::C0(_, _, _, h158) => {
                h158
            }
        }
    }))
}

pub fn f26(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h159, _, _, _) => {
            h159
        }
    }, List::cons((v1, v2), match v0.clone() {
        Adt1::C0(_, h160, _, _) => {
            h160
        }
    }), match v0.clone() {
        Adt1::C0(_, _, h161, _) => {
            h161
        }
    }, match v0 {
        Adt1::C0(_, _, _, h162) => {
            h162
        }
    })
}

pub fn f27(v0: List<Bytes>) -> R<List<Str>> {
    let m163 = v0;
    match m163.uncons() {
        None => {
            Ok(List::<Str>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f4(v1)?, f27(v2)?))
        }
    }
}

pub fn f28(v0: List<(Bytes, Bytes)>) -> R<List<(Str, Str)>> {
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
            })?), f28(v2)?))
        }
    }
}

pub fn f29(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f44(v0.clone(), 47u8)?;
    let v2: Bytes = f45(v1);
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
        Ok(f46(v0.clone(), 0u64, {
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

pub fn f30(v0: Adt1, v1: Bytes) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h177, _, _, _) => {
            h177
        }
    }, match v0.clone() {
        Adt1::C0(_, h178, _, _) => {
            h178
        }
    }, List::cons(v1, match v0.clone() {
        Adt1::C0(_, _, h179, _) => {
            h179
        }
    }), match v0 {
        Adt1::C0(_, _, _, h180) => {
            h180
        }
    })
}

pub fn f31(v0: List<Bytes>, v1: Adt1) -> R<Adt12> {
    f33(f47(v0, v1)?)
}

pub fn f32(v0: Bytes, v1: Bytes) -> bool {
    let a181 = v0;
    let a182 = v1;
    equal(a181, a182)
}

pub fn f33(v0: Adt12) -> R<Adt12> {
    let m184 = {
        let m183 = v0.clone();
        match m183 {
            Adt12::C0(v1) => {
                f48(v1)?
            }
            Adt12::C1(v2) => {
                f48(v2)?
            }
            Adt12::C2(v3, v4, _) => {
                f48(List::cons(v3, v4))?
            }
            Adt12::C3(v6, _) => {
                f48(f49(v6))?
            }
            Adt12::C4(_) => {
                None::<Bytes>
            }
        }
    };
    match m184 {
        None => {
            Ok(v0)
        }
        Some(v9) => {
            Ok(Adt12::C4(f50(v9)))
        }
    }
}

pub fn f34(v0: Adt1) -> R<Adt12> {
    if f51(f52(f53(), match v0.clone() {
        Adt1::C0(_, _, h185, _) => {
            h185
        }
    })) {
        Ok(f54(f53(), f55(f8(Str::lit("releases/latest")))))
    } else {
        let m186 = f56(v0.clone(), List::cons(f53(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil()))));
        match m186.uncons() {
            None => {
                let v4: Bytes = {
                    let m187 = f57(v0.clone(), f53())?;
                    match m187 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v3) => {
                            f58(v3, f8(Str::lit("tag_name")))
                        }
                    }
                };
                let a189 = {
                    let a188 = v4.clone();
                    length_bytes(a188)
                };
                let a190 = 0u64;
                if nat_eq(a189, a190) {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("waiting engine: the latest verdant-engine release names no tag\u{a}"))))
                } else {
                    let v5: List<Bytes> = f59(f60(), f61(v4.clone()));
                    let a192 = {
                        let a191 = f62(v5.clone(), f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?);
                        length_list(a191)
                    };
                    let a193 = 0u64;
                    if nat_eq(a192, a193) {
                        f65(v0, v4)
                    } else {
                        let m194 = f56(v0.clone(), f66(v5));
                        match m194.uncons() {
                            None => {
                                if f67(v0.clone(), f68(f69(Adt13::C1, f61(v4.clone())))) {
                                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting ingest: the verdant-engine ")), List::cons(v4, List::cons(f8(Str::lit(" bindings are staged, and advance ingests them\u{a}")), List::<Bytes>::nil()))))))
                                } else {
                                    f70(v0, v4)
                                }
                            }
                            Some((v6, v7)) => {
                                Ok(Adt12::C0(List::cons(v6, v7)))
                            }
                        }
                    }
                }
            }
            Some((v1, v2)) => {
                Ok(Adt12::C0(List::cons(v1, v2)))
            }
        }
    }
}

pub fn f35(v0: Adt12) -> Adt12 {
    let m195 = v0;
    match m195 {
        Adt12::C0(v3) => {
            Adt12::C0(v3)
        }
        Adt12::C1(v4) => {
            Adt12::C1(v4)
        }
        Adt12::C2(v5, v6, v7) => {
            Adt12::C2(v5, v6, v7)
        }
        Adt12::C3(v1, v2) => {
            Adt12::C3(List::cons((f8(Str::lit(".github/workflows/track.yml")), f71()), List::cons((f8(Str::lit(".github/scripts/track-pins.sh")), f72()), v1)), v2)
        }
        Adt12::C4(v8) => {
            Adt12::C4(v8)
        }
    }
}

pub fn f36(v0: u64) -> List<List<(Bytes, Option<Bytes>)>> {
    let m196 = v0;
    if m196 == 0 {
        List::<List<(Bytes, Option<Bytes>)>>::nil()
    } else {
        let v1 = m196 - 1;
        List::cons(List::<(Bytes, Option<Bytes>)>::nil(), f36(v1))
    }
}

pub fn f37(v0: u64) -> List<List<(Bytes, Option<Adt2>)>> {
    let m197 = v0;
    if m197 == 0 {
        List::<List<(Bytes, Option<Adt2>)>>::nil()
    } else {
        let v1 = m197 - 1;
        List::cons(List::<(Bytes, Option<Adt2>)>::nil(), f37(v1))
    }
}

pub fn f38(v0: u8) -> R<Str> {
    let v1: u64 = f73(v0)?;
    let m206 = {
        let a204 = f74({
            let a198 = v1;
            let a199 = 16u64;
            let a200 = 0u64;
            nat_quot(a198, a199, a200)
        });
        let a205 = {
            let a201 = v1;
            let a202 = 16u64;
            let a203 = 0u64;
            nat_rem(a201, a202, a203)
        };
        index_list(a204, a205)
    };
    match m206 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f39(v0: List<List<(Bytes, Option<Bytes>)>>, v1: u64, v2: (Bytes, Option<Bytes>)) -> List<List<(Bytes, Option<Bytes>)>> {
    let m207 = v0;
    match m207.uncons() {
        None => {
            List::<List<(Bytes, Option<Bytes>)>>::nil()
        }
        Some((v3, v4)) => {
            let a208 = v1;
            let a209 = 0u64;
            if nat_eq(a208, a209) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f39(v4, {
                    let a210 = v1;
                    let a211 = 1u64;
                    nat_sub(a210, a211)
                }, v2))
            }
        }
    }
}

pub fn f40(v0: Bytes) -> u64 {
    let a213 = {
        let a212 = v0;
        length_bytes(a212)
    };
    let a214 = 64u64;
    let a215 = 0u64;
    nat_rem(a213, a214, a215)
}

pub fn f41(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a216 = v0.clone();
        length_bytes(a216)
    };
    let v3: u64 = {
        let a217 = v1.clone();
        length_bytes(a217)
    };
    let a218 = v3;
    let a219 = v2;
    if nat_le(a218, a219) {
        let m225 = {
            let a222 = v0;
            let a223 = {
                let a220 = v2;
                let a221 = v3;
                nat_sub(a220, a221)
            };
            let a224 = v3;
            slice_bytes(a222, a223, a224)
        };
        match m225 {
            None => {
                false
            }
            Some(v4) => {
                let a226 = v4;
                let a227 = v1;
                equal(a226, a227)
            }
        }
    } else {
        false
    }
}

pub fn f42(v0: List<List<(Bytes, Option<Adt2>)>>, v1: u64, v2: (Bytes, Option<Adt2>)) -> List<List<(Bytes, Option<Adt2>)>> {
    let m228 = v0;
    match m228.uncons() {
        None => {
            List::<List<(Bytes, Option<Adt2>)>>::nil()
        }
        Some((v3, v4)) => {
            let a229 = v1;
            let a230 = 0u64;
            if nat_eq(a229, a230) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f42(v4, {
                    let a231 = v1;
                    let a232 = 1u64;
                    nat_sub(a231, a232)
                }, v2))
            }
        }
    }
}

pub fn f43(v0: Bytes) -> R<Option<Adt2>> {
    let v1: Bytes = f75(v0)?;
    let m236 = f76(v1.clone(), 0u64, {
        let a234 = {
            let a233 = v1.clone();
            length_bytes(a233)
        };
        let a235 = 2u64;
        nat_add(a234, a235)?
    }, 0u64)?;
    match m236 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v2) => {
            let a239 = {
                let (_, h237) = v2;
                h237
            };
            let a240 = {
                let a238 = v1;
                length_bytes(a238)
            };
            if nat_eq(a239, a240) {
                Ok(Some({
                    let (h241, _) = v2;
                    h241
                }))
            } else {
                Ok(None::<Adt2>)
            }
        }
    }
}

pub fn f44(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a242 = v0.clone();
        length_bytes(a242)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h243, _) = f77(Fn0::F78(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h243
    };
    let v4: u64 = {
        let (h244, _) = {
            let (_, h245) = v3.clone();
            h245
        };
        h244
    };
    Ok(f79(List::cons(f46(v0, v4, {
        let a246 = v2;
        let a247 = v4;
        nat_sub(a246, a247)
    }), {
        let (_, h248) = {
            let (_, h249) = v3;
            h249
        };
        h248
    })))
}

pub fn f45(p253: List<Bytes>) -> Bytes {
    let m252 = p253;
    {
        let mut m252 = m252;
        loop {
            let step = {
                let v0 = m252;
                let m250 = v0;
                match m250.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m251 = v2.clone();
                        match m251.uncons() {
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
                Ok(next) => { m252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m257 = {
        let a254 = v0;
        let a255 = v1;
        let a256 = v2;
        slice_bytes(a254, a255, a256)
    };
    match m257 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f47(v0: List<Bytes>, v1: Adt1) -> R<Adt12> {
    let m258 = v0;
    match m258.uncons() {
        None => {
            Ok(Adt12::C4(f80()))
        }
        Some((v2, v3)) => {
            if f32(v2.clone(), f8(Str::lit("agenda"))) {
                f81(v1)
            } else {
                if f32(v2.clone(), f8(Str::lit("build"))) {
                    Ok(f82(v1))
                } else {
                    if f32(v2.clone(), f8(Str::lit("check"))) {
                        f83(v1)
                    } else {
                        if f32(v2.clone(), f8(Str::lit("next"))) {
                            f84(v1, v3)
                        } else {
                            if f32(v2.clone(), f8(Str::lit("publish"))) {
                                f85(v1)
                            } else {
                                let m259 = v3;
                                match m259.uncons() {
                                    None => {
                                        Ok(Adt12::C4(f80()))
                                    }
                                    Some((v4, v5)) => {
                                        if if f86(v2.clone(), f8(Str::lit("read-"))) {
                                            true
                                        } else {
                                            f32(v2.clone(), f8(Str::lit("check-yaml")))
                                        } {
                                            Ok(Adt12::C4(f8(Str::lit("check-yaml, read-webidl and read-yaml run in verdant_readers, the command of the readers project, and read-xml, read-xsd and read-xsd-components in verdant_xml, the command of the XML readers project\u{a}"))))
                                        } else {
                                            if f86(v2.clone(), f8(Str::lit("stage-"))) {
                                                f87(v2, v4, v5, v1)
                                            } else {
                                                f88(v2, v4, v5, v1)
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

pub fn f48(p262: List<Bytes>) -> R<Option<Bytes>> {
    let m261 = p262;
    {
        let mut m261 = m261;
        loop {
            let step = {
                let v0 = m261;
                let m260 = v0;
                match m260.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f89(v1.clone())? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
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

pub fn f49(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m263 = v0;
    match m263.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h264, _) = v1;
                h264
            }, f49(v2))
        }
    }
}

pub fn f50(v0: Bytes) -> Bytes {
    let a269 = {
        let a265 = Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ");
        utf8_encode(a265)
    };
    let a270 = {
        let a267 = v0;
        let a268 = {
            let a266 = Str::lit(" is not\u{a}");
            utf8_encode(a266)
        };
        append_bytes(a267, a268)
    };
    append_bytes(a269, a270)
}

pub fn f51(v0: bool) -> bool {
    !v0
}

pub fn f52(v0: Bytes, p273: List<Bytes>) -> bool {
    let m272 = p273;
    {
        let mut m272 = m272;
        loop {
            let step = {
                let v1 = m272;
                let m271 = v1;
                match m271.uncons() {
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
                Ok(next) => { m272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53() -> Bytes {
    f8(Str::lit("target/engine/release.json"))
}

pub fn f54(v0: Bytes, v1: Bytes) -> Adt12 {
    Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v0.clone(), List::cons(v1, List::<Bytes>::nil()))))), v0)
}

pub fn f55(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://api.github.com/repos/afflom/verdant-engine/")), List::cons(v0, List::<Bytes>::nil())))
}

pub fn f56(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m274 = v1;
    match m274.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m275 = f90(f91(v0.clone(), v2.clone()), v2.clone());
            match m275 {
                None => {
                    List::cons(v2, f56(v0, v3))
                }
                Some(_) => {
                    f56(v0, v3)
                }
            }
        }
    }
}

pub fn f57(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    f92(v0, v1)
}

pub fn f58(v0: Adt2, v1: Bytes) -> Bytes {
    let m276 = f93(f94(v0, v1));
    match m276 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f59(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m277 = v0;
    match m277.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f69(v2, v1.clone()), f59(v3, v1))
        }
    }
}

pub fn f60() -> List<Adt13> {
    List::cons(Adt13::C0, List::cons(Adt13::C1, List::cons(Adt13::C2, List::cons(Adt13::C3, List::<Adt13>::nil()))))
}

pub fn f61(v0: Bytes) -> Bytes {
    if f86(v0.clone(), f8(Str::lit("v"))) {
        f46(v0.clone(), 1u64, {
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

pub fn f62(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m281 = v0;
    match m281.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f62(v3, v1)
            } else {
                List::cons(v2, f62(v3, v1))
            }
        }
    }
}

pub fn f63(v0: Bytes) -> R<List<Bytes>> {
    f95(f96(v0)?, f8(Str::lit("id = \"")))
}

pub fn f64(v0: Adt1, v1: Bytes) -> Bytes {
    let m282 = f90(f91(v0, v1.clone()), v1);
    match m282 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m283 = v2;
            match m283 {
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

pub fn f65(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let v2: List<Adt2> = f97(v0, f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt2> = f98(v2.clone(), f61(v1.clone()));
    if f99(f100(v2), f100(v3.clone())) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" is pinned, and every admission of its registers cites it\u{a}")), List::<Bytes>::nil()))))))
    } else {
        Ok(Adt12::C3(List::cons((f8(Str::lit("model/admissions.json")), f101(Adt2::C7(std::rc::Rc::new(f102(v3))))?), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("re-cited the admissions of the verdant-engine registers at ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
    }
}

pub fn f66(v0: List<Bytes>) -> List<Bytes> {
    let m284 = v0;
    match m284.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f68(v1), f66(v2))
        }
    }
}

pub fn f67(v0: Adt1, v1: Bytes) -> bool {
    let m285 = f90(f91(v0, v1.clone()), v1);
    match m285 {
        None => {
            false
        }
        Some(v2) => {
            let m286 = v2;
            match m286 {
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

pub fn f68(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f69(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f103(v0), List::cons(v1, List::<Bytes>::nil())))
}

pub fn f70(v0: Adt1, v1: Bytes) -> R<Adt12> {
    if f51(f52(f104(), match v0.clone() {
        Adt1::C0(_, _, h287, _) => {
            h287
        }
    })) {
        Ok(f54(f104(), f55(f7(List::cons(f8(Str::lit("git/ref/tags/")), List::cons(v1, List::<Bytes>::nil()))))))
    } else {
        let m288 = f56(v0.clone(), List::cons(f104(), List::<Bytes>::nil()));
        match m288.uncons() {
            None => {
                let v6: Adt2 = {
                    let m290 = f94({
                        let m289 = f57(v0.clone(), f104())?;
                        match m289 {
                            None => {
                                Adt2::C9
                            }
                            Some(v4) => {
                                v4
                            }
                        }
                    }, f8(Str::lit("object")));
                    match m290 {
                        None => {
                            Adt2::C9
                        }
                        Some(v5) => {
                            v5
                        }
                    }
                };
                if f32(f58(v6.clone(), f8(Str::lit("type"))), f8(Str::lit("tag"))) {
                    if f51(f52(f105(), match v0.clone() {
                        Adt1::C0(_, _, h291, _) => {
                            h291
                        }
                    })) {
                        Ok(f54(f105(), f55(f7(List::cons(f8(Str::lit("git/tags/")), List::cons(f58(v6, f8(Str::lit("sha"))), List::<Bytes>::nil()))))))
                    } else {
                        let m292 = f56(v0.clone(), List::cons(f105(), List::<Bytes>::nil()));
                        match m292.uncons() {
                            None => {
                                f106(v0.clone(), v1, {
                                    let m294 = f94({
                                        let m293 = f57(v0, f105())?;
                                        match m293 {
                                            None => {
                                                Adt2::C9
                                            }
                                            Some(v9) => {
                                                v9
                                            }
                                        }
                                    }, f8(Str::lit("object")));
                                    match m294 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v10) => {
                                            f58(v10, f8(Str::lit("sha")))
                                        }
                                    }
                                })
                            }
                            Some((v7, v8)) => {
                                Ok(Adt12::C0(List::cons(v7, v8)))
                            }
                        }
                    }
                } else {
                    f106(v0, v1, f58(v6, f8(Str::lit("sha"))))
                }
            }
            Some((v2, v3)) => {
                Ok(Adt12::C0(List::cons(v2, v3)))
            }
        }
    }
}

pub fn f71() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Engine.lex.tex. Do not edit.\u{a}# REQ-ENGINE-TRACKING: Verdant Studio tracks Verdant Engine releases. Every hour this runs\u{a}# operate stage-engine, which stages the latest release's four registers when they are new\u{a}# (advance then ingests them by sha256), and re-cites their admissions once they are ingested.\u{a}# It then runs track-pins.sh, which proposes moving a toolchain pin once a release contains the fix it waits on.\u{a}name: track\u{a}\u{a}on:\u{a}  schedule:\u{a}    - cron: '17 * * * *'\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}  actions: write\u{a}\u{a}jobs:\u{a}  track:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 5\u{a}    steps:\u{a}      - env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}        run: gh workflow run operate.yml --repo \"$GITHUB_REPOSITORY\" --ref main -f args=stage-engine\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}      - env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}        run: bash .github/scripts/track-pins.sh\u{a}"))
}

pub fn f72() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env bash\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Engine.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# REQ-ENGINE-TRACKING for the toolchain: a pin that waits on an upstream fix moves itself. When the latest\u{a}")), List::cons(f8(Str::lit("# release of LexLean or prism-code contains the commit its pin waits on, this proposes moving the pin to that\u{a}")), List::cons(f8(Str::lit("# release through propose-command, whose export and studio check test it before it merges.\u{a}")), List::cons(f8(Str::lit("set -euo pipefail\u{a}")), List::cons(f8(Str::lit("cd \"$(git rev-parse --show-toplevel)\"\u{a}")), List::cons(f8(Str::lit("contains() { case \"$(gh api \"repos/$1/compare/$2...$3\" --jq .status || true)\" in ahead|identical) return 0;; *) return 1;; esac; }\u{a}")), List::cons(f8(Str::lit("latest() { gh api \"repos/$1/releases?per_page=30\" --jq \"[.[] | select(.prerelease | not) | select(.tag_name | startswith(\\\"$2\\\"))][0].tag_name // empty\"; }\u{a}")), List::cons(f8(Str::lit("commit() { gh api \"repos/$1/commits/$2\" --jq .sha; }\u{a}")), List::cons(f8(Str::lit("moved=''\u{a}")), List::cons(f8(Str::lit("# LexLean waits on eacdb95c, UOR-Foundation/lexlean#639.\u{a}")), List::cons(f8(Str::lit("tag=$(latest UOR-Foundation/lexlean v)\u{a}")), List::cons(f8(Str::lit("if [ -n \"$tag\" ] && contains UOR-Foundation/lexlean eacdb95c4d997c36decedd1c4bc9bbea6600fadf \"$tag\"; then\u{a}")), List::cons(f8(Str::lit("  sha=$(commit UOR-Foundation/lexlean \"$tag\")\u{a}")), List::cons(f8(Str::lit("  if ! grep -q \"$sha\" .github/lexlean.pin; then\u{a}")), List::cons(f8(Str::lit("    echo \"--git https://github.com/UOR-Foundation/lexlean --rev $sha\" > .github/lexlean.pin\u{a}")), List::cons(f8(Str::lit("    moved=\"${moved:+$moved,} LexLean $tag ($sha)\"\u{a}")), List::cons(f8(Str::lit("  fi\u{a}")), List::cons(f8(Str::lit("fi\u{a}")), List::cons(f8(Str::lit("# prism-code waits on 93f5640c, UOR-Foundation/prismpm#931.\u{a}")), List::cons(f8(Str::lit("tag=$(latest UOR-Foundation/prismpm prism-code-v)\u{a}")), List::cons(f8(Str::lit("if [ -n \"$tag\" ] && contains UOR-Foundation/prismpm 93f5640c6014c9a4700ec5f5c0069b3f6e50d444 \"$tag\"; then\u{a}")), List::cons(f8(Str::lit("  sha=$(commit UOR-Foundation/prismpm \"$tag\")\u{a}")), List::cons(f8(Str::lit("  ops=studio/src/Studio/Ops.lex.tex\u{a}")), List::cons(f8(Str::lit("  line=$(grep -n '^  name: prismCodeRev,' \"$ops\" | cut -d: -f1)\u{a}")), List::cons(f8(Str::lit("  if [ -n \"$line\" ] && ! sed -n \"$((line + 3))p\" \"$ops\" | grep -q \"$sha\"; then\u{a}")), List::cons(f8(Str::lit("    sed -i \"$((line + 3))s|lit \\\"[0-9a-f]*\\\"|lit \\\"$sha\\\"|\" \"$ops\"\u{a}")), List::cons(f8(Str::lit("    moved=\"${moved:+$moved,} prism-code $tag ($sha)\"\u{a}")), List::cons(f8(Str::lit("  fi\u{a}")), List::cons(f8(Str::lit("fi\u{a}")), List::cons(f8(Str::lit("if [ -z \"$moved\" ]; then echo \"no pin moves: no latest release contains the commit a pin waits on\"; exit 0; fi\u{a}")), List::cons(f8(Str::lit("if gh api \"repos/$GITHUB_REPOSITORY/pulls?state=open&per_page=100\" --jq '.[].title' | grep -q '^verdant_studio propose-command: Move the toolchain pins'; then\u{a}")), List::cons(f8(Str::lit("  echo \"a pin move is already proposed\"; exit 0\u{a}")), List::cons(f8(Str::lit("fi\u{a}")), List::cons(f8(Str::lit("summary=\"Move the toolchain pins to the upstream releases that now contain the fixes they waited on:$moved. The propose-command export and the studio check test the move before it merges (REQ-ENGINE-TRACKING)\"\u{a}")), List::cons(f8(Str::lit("gh workflow run operate.yml --repo \"$GITHUB_REPOSITORY\" --ref main -f args=propose-command -f input=\"$summary\u{a}")), List::cons(f8(Str::lit("$(git diff -- .github/lexlean.pin studio/src)\"\u{a}")), List::cons(f8(Str::lit("echo \"proposed:$moved\"\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))
}

pub fn f73(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a339 = {
        let a315 = {
            let a303 = {
                let a297 = {
                    let a295 = v1;
                    let a296 = 128u8;
                    fixed_u8::bit_and(a295, a296)
                };
                let a298 = 128u8;
                if equal(a297, a298) {
                    128u64
                } else {
                    0u64
                }
            };
            let a304 = {
                let a301 = {
                    let a299 = v1;
                    let a300 = 64u8;
                    fixed_u8::bit_and(a299, a300)
                };
                let a302 = 64u8;
                if equal(a301, a302) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a303, a304)?
        };
        let a316 = {
            let a313 = {
                let a307 = {
                    let a305 = v1;
                    let a306 = 32u8;
                    fixed_u8::bit_and(a305, a306)
                };
                let a308 = 32u8;
                if equal(a307, a308) {
                    32u64
                } else {
                    0u64
                }
            };
            let a314 = {
                let a311 = {
                    let a309 = v1;
                    let a310 = 16u8;
                    fixed_u8::bit_and(a309, a310)
                };
                let a312 = 16u8;
                if equal(a311, a312) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a313, a314)?
        };
        nat_add(a315, a316)?
    };
    let a340 = {
        let a337 = {
            let a325 = {
                let a319 = {
                    let a317 = v1;
                    let a318 = 8u8;
                    fixed_u8::bit_and(a317, a318)
                };
                let a320 = 8u8;
                if equal(a319, a320) {
                    8u64
                } else {
                    0u64
                }
            };
            let a326 = {
                let a323 = {
                    let a321 = v1;
                    let a322 = 4u8;
                    fixed_u8::bit_and(a321, a322)
                };
                let a324 = 4u8;
                if equal(a323, a324) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a325, a326)?
        };
        let a338 = {
            let a335 = {
                let a329 = {
                    let a327 = v1;
                    let a328 = 2u8;
                    fixed_u8::bit_and(a327, a328)
                };
                let a330 = 2u8;
                if equal(a329, a330) {
                    2u64
                } else {
                    0u64
                }
            };
            let a336 = {
                let a333 = {
                    let a331 = v1;
                    let a332 = 1u8;
                    fixed_u8::bit_and(a331, a332)
                };
                let a334 = 1u8;
                if equal(a333, a334) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a335, a336)?
        };
        nat_add(a337, a338)?
    };
    nat_add(a339, a340)
}

pub fn f74(v0: u64) -> List<Str> {
    let a341 = v0;
    let a342 = 0u64;
    if nat_eq(a341, a342) {
        f107()
    } else {
        let a343 = v0;
        let a344 = 1u64;
        if nat_eq(a343, a344) {
            f108()
        } else {
            let a345 = v0;
            let a346 = 2u64;
            if nat_eq(a345, a346) {
                f109()
            } else {
                let a347 = v0;
                let a348 = 3u64;
                if nat_eq(a347, a348) {
                    f110()
                } else {
                    let a349 = v0;
                    let a350 = 4u64;
                    if nat_eq(a349, a350) {
                        f111()
                    } else {
                        let a351 = v0;
                        let a352 = 5u64;
                        if nat_eq(a351, a352) {
                            f112()
                        } else {
                            let a353 = v0;
                            let a354 = 6u64;
                            if nat_eq(a353, a354) {
                                f113()
                            } else {
                                let a355 = v0;
                                let a356 = 7u64;
                                if nat_eq(a355, a356) {
                                    f114()
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

pub fn f75(v0: Bytes) -> R<Bytes> {
    f115(v0.clone(), 0u64, {
        let a357 = v0;
        length_bytes(a357)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f76(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m358 = v2;
    if m358 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m358 - 1;
        let v5: u64 = f116(v0.clone(), v1)?;
        let a359 = v3;
        let a360 = 1u64;
        if nat_eq(a359, a360) {
            let m361 = f76(v0.clone(), v1, v4, 0u64)?;
            match m361 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f116(v0.clone(), {
                        let (_, h362) = v6.clone();
                        h362
                    })?;
                    let a363 = v7;
                    let a364 = 44u64;
                    if nat_eq(a363, a364) {
                        let m368 = f76(v0, {
                            let a366 = {
                                let (_, h365) = v6;
                                h365
                            };
                            let a367 = 1u64;
                            nat_add(a366, a367)?
                        }, v4, 1u64)?;
                        match m368 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C10(std::rc::Rc::new({
                                    let (h369, _) = v6;
                                    h369
                                }), std::rc::Rc::new({
                                    let (h370, _) = v8;
                                    h370
                                })), {
                                    let (_, h371) = v8;
                                    h371
                                })))
                            }
                        }
                    } else {
                        let a372 = v7;
                        let a373 = 93u64;
                        if nat_eq(a372, a373) {
                            Ok(Some((Adt2::C10(std::rc::Rc::new({
                                let (h374, _) = v6;
                                h374
                            }), std::rc::Rc::new(Adt2::C9)), {
                                let a376 = {
                                    let (_, h375) = v6;
                                    h375
                                };
                                let a377 = 1u64;
                                nat_add(a376, a377)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a378 = v3;
            let a379 = 2u64;
            if nat_eq(a378, a379) {
                let m382 = f117(v0.clone(), {
                    let a380 = v1;
                    let a381 = 1u64;
                    nat_add(a380, a381)?
                })?;
                match m382 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f118({
                            let a383 = v5;
                            let a384 = 34u64;
                            nat_eq(a383, a384)
                        }, {
                            let a386 = f116(v0.clone(), {
                                let (h385, _) = v9.clone();
                                h385
                            })?;
                            let a387 = 58u64;
                            nat_eq(a386, a387)
                        }) {
                            let m391 = f76(v0.clone(), {
                                let a389 = {
                                    let (h388, _) = v9;
                                    h388
                                };
                                let a390 = 1u64;
                                nat_add(a389, a390)?
                            }, v4, 0u64)?;
                            match m391 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f116(v0.clone(), {
                                        let (_, h392) = v10.clone();
                                        h392
                                    })?;
                                    let a393 = v11;
                                    let a394 = 44u64;
                                    if nat_eq(a393, a394) {
                                        let m398 = f76(v0, {
                                            let a396 = {
                                                let (_, h395) = v10;
                                                h395
                                            };
                                            let a397 = 1u64;
                                            nat_add(a396, a397)?
                                        }, v4, 2u64)?;
                                        match m398 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C11({
                                                    let (_, h399) = v9;
                                                    h399
                                                }, std::rc::Rc::new({
                                                    let (h400, _) = v10;
                                                    h400
                                                }), std::rc::Rc::new({
                                                    let (h401, _) = v12;
                                                    h401
                                                })), {
                                                    let (_, h402) = v12;
                                                    h402
                                                })))
                                            }
                                        }
                                    } else {
                                        let a403 = v11;
                                        let a404 = 125u64;
                                        if nat_eq(a403, a404) {
                                            Ok(Some((Adt2::C11({
                                                let (_, h405) = v9;
                                                h405
                                            }, std::rc::Rc::new({
                                                let (h406, _) = v10;
                                                h406
                                            }), std::rc::Rc::new(Adt2::C9)), {
                                                let a408 = {
                                                    let (_, h407) = v10;
                                                    h407
                                                };
                                                let a409 = 1u64;
                                                nat_add(a408, a409)?
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
                let a410 = v5;
                let a411 = 34u64;
                if nat_eq(a410, a411) {
                    let m414 = f117(v0, {
                        let a412 = v1;
                        let a413 = 1u64;
                        nat_add(a412, a413)?
                    })?;
                    match m414 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C6({
                                let (_, h415) = v13;
                                h415
                            }), {
                                let (h416, _) = v13;
                                h416
                            })))
                        }
                    }
                } else {
                    if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a417 = v1;
                            let a418 = 4u64;
                            nat_add(a417, a418)?
                        })))
                    } else {
                        if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a419 = v1;
                                let a420 = 4u64;
                                nat_add(a419, a420)?
                            })))
                        } else {
                            if f32(f46(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a421 = v1;
                                    let a422 = 5u64;
                                    nat_add(a421, a422)?
                                })))
                            } else {
                                let a423 = v5;
                                let a424 = 91u64;
                                if nat_eq(a423, a424) {
                                    let a427 = f116(v0.clone(), {
                                        let a425 = v1;
                                        let a426 = 1u64;
                                        nat_add(a425, a426)?
                                    })?;
                                    let a428 = 93u64;
                                    if nat_eq(a427, a428) {
                                        Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C9)), {
                                            let a429 = v1;
                                            let a430 = 2u64;
                                            nat_add(a429, a430)?
                                        })))
                                    } else {
                                        let m433 = f76(v0, {
                                            let a431 = v1;
                                            let a432 = 1u64;
                                            nat_add(a431, a432)?
                                        }, v4, 1u64)?;
                                        match m433 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                    let (h434, _) = v14;
                                                    h434
                                                })), {
                                                    let (_, h435) = v14;
                                                    h435
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a436 = v5;
                                    let a437 = 123u64;
                                    if nat_eq(a436, a437) {
                                        let a440 = f116(v0.clone(), {
                                            let a438 = v1;
                                            let a439 = 1u64;
                                            nat_add(a438, a439)?
                                        })?;
                                        let a441 = 125u64;
                                        if nat_eq(a440, a441) {
                                            Ok(Some((Adt2::C8(std::rc::Rc::new(Adt2::C9)), {
                                                let a442 = v1;
                                                let a443 = 2u64;
                                                nat_add(a442, a443)?
                                            })))
                                        } else {
                                            let m446 = f76(v0, {
                                                let a444 = v1;
                                                let a445 = 1u64;
                                                nat_add(a444, a445)?
                                            }, v4, 2u64)?;
                                            match m446 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C8(std::rc::Rc::new({
                                                        let (h447, _) = v15;
                                                        h447
                                                    })), {
                                                        let (_, h448) = v15;
                                                        h448
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m449 = f119(v0, v1)?;
                                        match m449 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h450) = v16;
                                                    h450
                                                }, {
                                                    let (h451, _) = v16;
                                                    h451
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

pub fn f77(v0: Fn0, p456: u64, p457: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m455 = (p456, p457);
    {
        let mut m455 = m455;
        loop {
            let step = {
                let (v1, v2) = m455;
                let m452 = v1;
                if m452 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m452 - 1;
                    let m454 = {
                        let c453: Fn0 = v0.clone();
                        c453.apply(v2.clone())?
                    };
                    match m454 {
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
                Ok(next) => { m455 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f120(v1, v0, v2)
}

pub fn f79(v0: List<Bytes>) -> List<Bytes> {
    f121(Fn1::F122, List::<Bytes>::nil(), v0)
}

pub fn f80() -> Bytes {
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f123(f124(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f81(v0: Adt1) -> R<Adt12> {
    let m458 = f125(v0.clone());
    match m458.uncons() {
        None => {
            let m459 = f126(f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f127(v0.clone())?);
            match m459.uncons() {
                None => {
                    let v5: Adt14 = f128(v0.clone())?;
                    let v6: List<Adt2> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(f126(f129(f130(match v5.clone() {
                        Adt14::C0(h460, _, _) => {
                            h460
                        }
                    }, f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f132(v6.clone())), f133(f130(match v5.clone() {
                        Adt14::C0(h461, _, _) => {
                            h461
                        }
                    }, f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f134(v5.clone(), v6, f135(v5, f136(v0.clone(), f137(v0)?)?))?)))))
                }
                Some((v3, v4)) => {
                    Ok(Adt12::C0(List::cons(v3, v4)))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt12::C1(List::cons(v1, v2)))
        }
    }
}

pub fn f82(v0: Adt1) -> Adt12 {
    if f51(f52(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h462, _) => {
            h462
        }
    })) {
        Adt12::C2(f8(Str::lit("bash")), f138(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f51(f52(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h463, _) => {
                h463
            }
        })) {
            Adt12::C2(f8(Str::lit("bash")), f138(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f83(v0: Adt1) -> R<Adt12> {
    if f51(f139(match v0.clone() {
        Adt1::C0(_, h464, _, _) => {
            h464
        }
    }, f140())) {
        Ok(Adt12::C1(List::cons(f140(), List::<Bytes>::nil())))
    } else {
        let m465 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f141(v0.clone())?)));
        match m465.uncons() {
            None => {
                let m466 = f142(v0.clone(), f141(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m466.uncons() {
                    None => {
                        f144(v0)
                    }
                    Some((v3, v4)) => {
                        Ok(Adt12::C4(f7(List::cons(v3, v4))))
                    }
                }
            }
            Some((v1, v2)) => {
                Ok(Adt12::C0(List::cons(v1, v2)))
            }
        }
    }
}

pub fn f84(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f139(match v0.clone() {
        Adt1::C0(_, h467, _, _) => {
            h467
        }
    }, f145())) {
        Ok(Adt12::C1(List::cons(f145(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f146(f147(f145(), f148(match v0.clone() {
            Adt1::C0(_, h468, _, _) => {
                h468
            }
        }, f145())))?;
        let m469 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m469.uncons() {
            None => {
                let m470 = f149(v0.clone(), f150(f151(f96(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?, f152(v2, v1.clone()))?;
                match m470 {
                    None => {
                        f153(v0, v1)
                    }
                    Some(v5) => {
                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("ingest-authority ")), List::cons(v5, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt12::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f85(v0: Adt1) -> R<Adt12> {
    let m471 = f125(v0.clone());
    match m471.uncons() {
        None => {
            let m472 = f126(f56(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f154(), List::<Bytes>::nil())))), f127(v0.clone())?);
            match m472.uncons() {
                None => {
                    Ok(Adt12::C3(f155(f156(v0.clone())?, List::cons((f157(), f158()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f159()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f160()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f161()), List::cons((f8(Str::lit(".github/scripts/readmit.sh")), f162()), List::cons((f8(Str::lit(".github/scripts/merge-entries.py")), f163()), List::cons((f8(Str::lit(".github/scripts/prism-acceptance.py")), f164()), List::cons((f8(Str::lit(".github/scripts/prepare-change.py")), f165()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f166()), List::cons((f8(Str::lit(".github/lexlean.pin")), f167()), List::cons((f8(Str::lit("docs/catalog.md")), f168(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f169(v0.clone())?), List::cons((f8(Str::lit("app/src/Studio/Prism/Closure/Register.lex.tex")), f170(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f171(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f172()), List::cons((f8(Str::lit("surface/mcp-tools.json")), f173()?), List::cons((f8(Str::lit(".github/scripts/surface-check.py")), f174()), List::cons((f8(Str::lit(".github/scripts/gui-tree.py")), f175()), List::<(Bytes, Bytes)>::nil()))))))))))))))))))), f8(Str::lit("published the workflows, the export, readmit, entry-merge, change-preparing, Prism acceptance and surface-check and GUI-tree scripts, CODEOWNERS, the lexlean pin, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md, docs/owner-decisions.md, the Prism closure register and the MCP tools of surface/mcp-tools.json\u{a}"))))
                }
                Some((v3, v4)) => {
                    Ok(Adt12::C0(List::cons(v3, v4)))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt12::C1(List::cons(v1, v2)))
        }
    }
}

pub fn f86(v0: Bytes, v1: Bytes) -> bool {
    let m477 = {
        let a474 = v0;
        let a475 = 0u64;
        let a476 = {
            let a473 = v1.clone();
            length_bytes(a473)
        };
        slice_bytes(a474, a475, a476)
    };
    match m477 {
        None => {
            false
        }
        Some(v2) => {
            let a478 = v2;
            let a479 = v1;
            equal(a478, a479)
        }
    }
}

pub fn f87(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f176(v3.clone(), v1.clone(), f177(v3, v1, false)?)
    } else {
        if f32(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f176(v3.clone(), v1.clone(), f178(v3, v1)?)
        } else {
            if f32(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f176(v3.clone(), v1.clone(), f177(v3, v1, true)?)
            } else {
                if f32(v0.clone(), f8(Str::lit("stage-registries"))) {
                    f176(v3.clone(), v1.clone(), f179(v3, v1)?)
                } else {
                    if f32(v0.clone(), f8(Str::lit("stage-encodings"))) {
                        f176(v3.clone(), v1.clone(), f180(v3, v1)?)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("stage-mappings"))) {
                            f176(v3.clone(), v1.clone(), f181(v3, v1, false)?)
                        } else {
                            if f32(v0, f8(Str::lit("stage-matrix"))) {
                                f176(v3.clone(), v1.clone(), f181(v3, v1, true)?)
                            } else {
                                Ok(Adt12::C4(f80()))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f88(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if if f32(v0.clone(), f8(Str::lit("admit"))) {
        true
    } else {
        if f32(v0.clone(), f8(Str::lit("admit-encoding"))) {
            true
        } else {
            if f32(v0.clone(), f8(Str::lit("admit-registry"))) {
                true
            } else {
                if f32(v0.clone(), f8(Str::lit("admit-translated"))) {
                    true
                } else {
                    f32(v0.clone(), f8(Str::lit("propose-encoding")))
                }
            }
        }
    } {
        Ok(Adt12::C4(f8(Str::lit("admit, admit-encoding, admit-registry, admit-translated and propose-encoding run in verdant_admit, the command of the admission project (admit)\u{a}"))))
    } else {
        if f32(v0.clone(), f8(Str::lit("ingest-authority"))) {
            f182(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("ingest-index"))) {
                f183(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("merge-gate"))) {
                    f184(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                        f185(v3, v1)
                    } else {
                        if f32(v0, f8(Str::lit("record-error"))) {
                            f186(v3, v1, v2)
                        } else {
                            Ok(Adt12::C4(f80()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f89(v0: Bytes) -> R<bool> {
    f187(v0.clone(), 0u64, {
        let a480 = v0;
        length_bytes(a480)
    })
}

pub fn f90(p485: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m484 = p485;
    {
        let mut m484 = m484;
        loop {
            let step = {
                let v0 = m484;
                let m481 = v0;
                match m481.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h482, _) = v2;
                            h482
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h483) = v2;
                                h483
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f91(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m489 = {
        let a487 = match v0 {
            Adt1::C0(h486, _, _, _) => {
                h486
            }
        };
        let a488 = f40(v1);
        index_list(a487, a488)
    };
    match m489 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m490 = f188(f189(v0.clone(), v1.clone()), v1.clone());
    match m490 {
        None => {
            f43(f64(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f93(v0: Option<Adt2>) -> Option<Bytes> {
    let m491 = v0;
    match m491 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m492 = v1;
            match m492 {
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

pub fn f94(p498: Adt2, v1: Bytes) -> Option<Adt2> {
    let m497 = p498;
    {
        let mut m497 = m497;
        loop {
            let step = {
                let v0 = m497;
                let m493 = v0;
                match m493 {
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
                    Adt2::C8(r494) => {
                        let v5 = (*r494).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C9 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(v2, r495, r496) => {
                        let v3 = (*r495).clone();
                        let v4 = (*r496).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m499 = v0;
    match m499.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m500 = f190(v2, v1.clone())?;
            match m500 {
                None => {
                    f95(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f95(v3, v1)?))
                }
            }
        }
    }
}

pub fn f96(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a501 = v0.clone();
        length_bytes(a501)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h502, _) = f77(Fn0::F191(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h502
    };
    let v3: u64 = {
        let (h503, _) = {
            let (_, h504) = v2.clone();
            h504
        };
        h503
    };
    let v4: List<Bytes> = {
        let (_, h505) = {
            let (_, h506) = v2;
            h506
        };
        h505
    };
    let a507 = v3;
    let a508 = v1;
    if nat_lt(a507, a508) {
        Ok(f79(List::cons(f46(v0, v3, {
            let a509 = v1;
            let a510 = v3;
            nat_sub(a509, a510)
        }), v4)))
    } else {
        Ok(f79(v4))
    }
}

pub fn f97(v0: Adt1, v1: Bytes) -> R<List<Adt2>> {
    Ok(f192(f57(v0, v1)?))
}

pub fn f98(v0: List<Adt2>, v1: Bytes) -> List<Adt2> {
    let m511 = v0;
    match m511.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f193(v2, v1.clone()), f98(v3, v1))
        }
    }
}

pub fn f99(p516: List<Bytes>, p517: List<Bytes>) -> bool {
    let m515 = (p516, p517);
    {
        let mut m515 = m515;
        loop {
            let step = {
                let (v0, v1) = m515;
                let m512 = v0;
                match m512.uncons() {
                    None => {
                        let m513 = v1;
                        match m513.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m514 = v1;
                        match m514.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f32(v4, v6) {
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
                Ok(next) => { m515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: List<Adt2>) -> List<Bytes> {
    let m518 = v0;
    match m518.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m519 = f94(v1, f8(Str::lit("std")));
            match m519 {
                None => {
                    f100(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f100(v2))
                }
            }
        }
    }
}

pub fn f101(v0: Adt2) -> R<Bytes> {
    let a520 = f194(v0)?;
    let a521 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a520, a521))
}

pub fn f102(v0: List<Adt2>) -> Adt2 {
    let m522 = v0;
    match m522.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(v1), std::rc::Rc::new(f102(v2)))
        }
    }
}

pub fn f103(v0: Adt13) -> Bytes {
    let m523 = v0;
    match m523 {
        Adt13::C0 => {
            f8(Str::lit("VERDANT-ENGINE-AUTHORITIES-"))
        }
        Adt13::C1 => {
            f8(Str::lit("VERDANT-ENGINE-IDS-"))
        }
        Adt13::C2 => {
            f8(Str::lit("VERDANT-ENGINE-RELEASE-LEDGER-"))
        }
        Adt13::C3 => {
            f8(Str::lit("VERDANT-ENGINE-STANDARDS-LOCK-"))
        }
    }
}

pub fn f104() -> Bytes {
    f8(Str::lit("target/engine/ref.json"))
}

pub fn f105() -> Bytes {
    f8(Str::lit("target/engine/tag.json"))
}

pub fn f106(v0: Adt1, v1: Bytes, v2: Bytes) -> R<Adt12> {
    let a525 = {
        let a524 = v2.clone();
        length_bytes(a524)
    };
    let a526 = 0u64;
    if nat_eq(a525, a526) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m527 = f195(v0.clone(), f60())?;
        match m527 {
            None => {
                if f51(f52(f198(), match v0.clone() {
                    Adt1::C0(_, _, h528, _) => {
                        h528
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f198()))
                } else {
                    let m529 = f56(v0.clone(), List::cons(f199(), f200(f60())?));
                    match m529.uncons() {
                        None => {
                            Ok(Adt12::C3(f201(v0.clone(), f60(), v1.clone(), v2.clone(), f202(f64(v0, f199()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f196(v3.clone())?, f197(v3, v2)))
            }
        }
    }
}

pub fn f107() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f108() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f109() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f110() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f111() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f112() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f113() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f114() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f115(v0: Bytes, p557: u64, p558: u64, p559: bool, p560: bool, p561: u64, p562: Bytes) -> R<Bytes> {
    let m556 = (p557, (p558, (p559, (p560, (p561, p562)))));
    {
        let mut m556 = m556;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m556;
                let m530 = v2;
                if m530 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f203(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m530 - 1;
                    let m533 = {
                        let a531 = v0.clone();
                        let a532 = v1;
                        index_bytes(a531, a532)
                    };
                    match m533 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f203(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f73(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a534 = v1;
                                    let a535 = 1u64;
                                    nat_add(a534, a535)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a536 = v9;
                                    let a537 = 34u64;
                                    !nat_eq(a536, a537)
                                }, (if v4 {
                                    false
                                } else {
                                    let a538 = v9;
                                    let a539 = 92u64;
                                    nat_eq(a538, a539)
                                }, (v5, v6))))))
                            } else {
                                let a540 = v9;
                                let a541 = 32u64;
                                if if if if nat_eq(a540, a541) {
                                    true
                                } else {
                                    let a542 = v9;
                                    let a543 = 9u64;
                                    nat_eq(a542, a543)
                                } {
                                    true
                                } else {
                                    let a544 = v9;
                                    let a545 = 10u64;
                                    nat_eq(a544, a545)
                                } {
                                    true
                                } else {
                                    let a546 = v9;
                                    let a547 = 13u64;
                                    nat_eq(a546, a547)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a548 = v1;
                                        let a549 = 1u64;
                                        nat_add(a548, a549)?
                                    }, (v7, (false, (false, ({
                                        let a550 = v1;
                                        let a551 = 1u64;
                                        nat_add(a550, a551)?
                                    }, f203(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a552 = v1;
                                        let a553 = 1u64;
                                        nat_add(a552, a553)?
                                    }, (v7, ({
                                        let a554 = v9;
                                        let a555 = 34u64;
                                        nat_eq(a554, a555)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: Bytes, v1: u64) -> R<u64> {
    let m565 = {
        let a563 = v0;
        let a564 = v1;
        index_bytes(a563, a564)
    };
    match m565 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a610 = {
                let a586 = {
                    let a574 = {
                        let a568 = {
                            let a566 = v3;
                            let a567 = 128u8;
                            fixed_u8::bit_and(a566, a567)
                        };
                        let a569 = 128u8;
                        if equal(a568, a569) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a575 = {
                        let a572 = {
                            let a570 = v3;
                            let a571 = 64u8;
                            fixed_u8::bit_and(a570, a571)
                        };
                        let a573 = 64u8;
                        if equal(a572, a573) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a574, a575)?
                };
                let a587 = {
                    let a584 = {
                        let a578 = {
                            let a576 = v3;
                            let a577 = 32u8;
                            fixed_u8::bit_and(a576, a577)
                        };
                        let a579 = 32u8;
                        if equal(a578, a579) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a585 = {
                        let a582 = {
                            let a580 = v3;
                            let a581 = 16u8;
                            fixed_u8::bit_and(a580, a581)
                        };
                        let a583 = 16u8;
                        if equal(a582, a583) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a584, a585)?
                };
                nat_add(a586, a587)?
            };
            let a611 = {
                let a608 = {
                    let a596 = {
                        let a590 = {
                            let a588 = v3;
                            let a589 = 8u8;
                            fixed_u8::bit_and(a588, a589)
                        };
                        let a591 = 8u8;
                        if equal(a590, a591) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a597 = {
                        let a594 = {
                            let a592 = v3;
                            let a593 = 4u8;
                            fixed_u8::bit_and(a592, a593)
                        };
                        let a595 = 4u8;
                        if equal(a594, a595) {
                            4u64
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
                            let a599 = 2u8;
                            fixed_u8::bit_and(a598, a599)
                        };
                        let a601 = 2u8;
                        if equal(a600, a601) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a607 = {
                        let a604 = {
                            let a602 = v3;
                            let a603 = 1u8;
                            fixed_u8::bit_and(a602, a603)
                        };
                        let a605 = 1u8;
                        if equal(a604, a605) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a606, a607)?
                };
                nat_add(a608, a609)?
            };
            nat_add(a610, a611)
        }
    }
}

pub fn f117(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f204(v0.clone(), v1, {
        let a613 = {
            let a612 = v0.clone();
            length_bytes(a612)
        };
        let a614 = 1u64;
        nat_add(a613, a614)?
    })?;
    let a615 = f116(v0.clone(), v2)?;
    let a616 = 34u64;
    if nat_eq(a615, a616) {
        Ok(Some(({
            let a617 = v2;
            let a618 = 1u64;
            nat_add(a617, a618)?
        }, f46(v0, v1, {
            let a619 = v2;
            let a620 = v1;
            nat_sub(a619, a620)
        }))))
    } else {
        f205(v0.clone(), v1, {
            let a622 = {
                let a621 = v0;
                length_bytes(a621)
            };
            let a623 = 1u64;
            nat_add(a622, a623)?
        }, Bytes::lit(&[]))
    }
}

pub fn f118(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f119(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a624 = f116(v0.clone(), v1)?;
        let a625 = 45u64;
        nat_eq(a624, a625)
    };
    let v3: u64 = if v2 {
        let a626 = v1;
        let a627 = 1u64;
        nat_add(a626, a627)?
    } else {
        v1
    };
    let v4: u64 = f206(v0.clone(), v3)?;
    let v5: u64 = {
        let a628 = v4;
        let a629 = v3;
        nat_sub(a628, a629)
    };
    let v6: bool = {
        let a630 = f116(v0.clone(), v4)?;
        let a631 = 46u64;
        nat_eq(a630, a631)
    };
    let v7: u64 = if v6 {
        let a632 = v4;
        let a633 = 1u64;
        nat_add(a632, a633)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f206(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f116(v0.clone(), v8)?;
    let v10: bool = {
        let a634 = v9;
        let a635 = 101u64;
        if nat_eq(a634, a635) {
            true
        } else {
            let a636 = v9;
            let a637 = 69u64;
            nat_eq(a636, a637)
        }
    };
    let v11: u64 = f116(v0.clone(), {
        let a638 = v8;
        let a639 = 1u64;
        nat_add(a638, a639)?
    })?;
    let v12: bool = f118(v10, f207({
        let a640 = v11;
        let a641 = 43u64;
        nat_eq(a640, a641)
    }, {
        let a642 = v11;
        let a643 = 45u64;
        nat_eq(a642, a643)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a644 = v8;
            let a645 = 2u64;
            nat_add(a644, a645)?
        } else {
            let a646 = v8;
            let a647 = 1u64;
            nat_add(a646, a647)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f206(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a648 = v14;
        let a649 = v13;
        nat_sub(a648, a649)
    });
    let v16: u64 = f208(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a651 = {
            let a650 = v15;
            length_bytes(a650)
        };
        let a652 = v16;
        nat_sub(a651, a652)
    });
    let a653 = v5;
    let a654 = 0u64;
    if if nat_eq(a653, a654) {
        true
    } else {
        if f118({
            let a655 = 1u64;
            let a656 = v5;
            nat_lt(a655, a656)
        }, {
            let a657 = f116(v0.clone(), v3)?;
            let a658 = 48u64;
            nat_eq(a657, a658)
        }) {
            true
        } else {
            if f118(v6, {
                let a659 = v8;
                let a660 = v7;
                nat_eq(a659, a660)
            }) {
                true
            } else {
                f207(if v10 {
                    let a661 = v14;
                    let a662 = v13;
                    nat_eq(a661, a662)
                } else {
                    false
                }, {
                    let a664 = 18u64;
                    let a665 = {
                        let a663 = v17.clone();
                        length_bytes(a663)
                    };
                    nat_lt(a664, a665)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m669 = {
            let a667 = {
                let a666 = v17.clone();
                length_bytes(a666)
            };
            let a668 = 0u64;
            if nat_eq(a667, a668) {
                Some(0u64)
            } else {
                f209(v17)?
            }
        };
        match m669 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m674 = f210(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a670 = v8;
                    let a671 = v7;
                    nat_sub(a670, a671)
                }), f118(v12, {
                    let a672 = v11;
                    let a673 = 45u64;
                    nat_eq(a672, a673)
                }), v18)?;
                match m674 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a677 = 19u64;
                        let a678 = {
                            let a676 = {
                                let (h675, _) = v19.clone();
                                h675
                            };
                            length_bytes(a676)
                        };
                        if nat_lt(a677, a678) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m680 = f209({
                                let (h679, _) = v19.clone();
                                h679
                            })?;
                            match m680 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f118(v2, {
                                        let a681 = 0u64;
                                        let a682 = v20;
                                        nat_lt(a681, a682)
                                    });
                                    if f211(v21, v20) {
                                        Ok(Some((v14, {
                                            let a684 = {
                                                let (_, h683) = v19.clone();
                                                h683
                                            };
                                            let a685 = 0u64;
                                            if nat_eq(a684, a685) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h686) = v19;
                                                    h686
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

pub fn f120(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h687, _) = v2.clone();
        h687
    };
    let v4: u64 = {
        let (h688, _) = {
            let (_, h689) = v2.clone();
            h689
        };
        h688
    };
    let v5: List<Bytes> = {
        let (_, h690) = {
            let (_, h691) = v2;
            h691
        };
        h690
    };
    let m692 = f212(v0.clone(), v3);
    match m692 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a693 = v6;
            let a694 = v1;
            if equal(a693, a694) {
                Ok(Some(({
                    let a695 = v3;
                    let a696 = 1u64;
                    nat_add(a695, a696)?
                }, ({
                    let a697 = v3;
                    let a698 = 1u64;
                    nat_add(a697, a698)?
                }, List::cons(f46(v0, v4, {
                    let a699 = v3;
                    let a700 = v4;
                    nat_sub(a699, a700)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a701 = v3;
                    let a702 = 1u64;
                    nat_add(a701, a702)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f121(v0: Fn1, p706: List<Bytes>, p707: List<Bytes>) -> List<Bytes> {
    let m705 = (p706, p707);
    {
        let mut m705 = m705;
        loop {
            let step = {
                let (v1, v2) = m705;
                let m703 = v2;
                match m703.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c704: Fn1 = v0.clone();
                            c704.apply(v1, v3)
                        }, v4))
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

pub fn f122(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m708 = f213(Fn2::F214(v1), None::<Bytes>, v0);
    match m708 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP|check-yaml ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("check-yaml FILE")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-admit NAME read-OP FILE ARG... CITATION...")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::cons(f8(Str::lit("verify-translations ADMISSIONS")), List::<Bytes>::nil())))))))))))))))))))))))))))))))
}

pub fn f125(v0: Adt1) -> List<Bytes> {
    if f139(match v0.clone() {
        Adt1::C0(_, h709, _, _) => {
            h709
        }
    }, f215()) {
        if f139(match v0 {
            Adt1::C0(_, h710, _, _) => {
                h710
            }
        }, f216()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f216(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f215(), List::<Bytes>::nil())
    }
}

pub fn f126(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m711 = v0;
    match m711.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt1) -> R<List<Bytes>> {
    Ok(f56(v0.clone(), List::cons(f217(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f137(v0)?)))))
}

pub fn f128(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f218(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f143(f97(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f129(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m712 = v0;
    match m712.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f129(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f129(v3, v1))
            }
        }
    }
}

pub fn f130(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m713 = v0;
    match m713.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f130(v3, v1)
            } else {
                List::cons(v2, f130(v3, v1))
            }
        }
    }
}

pub fn f131(v0: List<Adt2>) -> List<Bytes> {
    let m714 = v0;
    match m714.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f126(f219(f192(f94(v1, f8(Str::lit("defers"))))), f131(v2))
        }
    }
}

pub fn f132(v0: List<Adt2>) -> List<Bytes> {
    let m715 = v0;
    match m715.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m716 = f94(v1, f8(Str::lit("std")));
            match m716 {
                None => {
                    f132(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("row"))), f132(v2))
                }
            }
        }
    }
}

pub fn f133(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m717 = v0;
    match m717.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f133(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f133(v3, v1))
            }
        }
    }
}

pub fn f134(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f155(f220(v2.clone()), f221(f222(v0, v1.clone(), v2)?));
    f223(f146(f224(v1))?, v3.clone(), {
        let a719 = {
            let a718 = v3;
            length_list(a718)
        };
        let a720 = 1u64;
        nat_add(a719, a720)?
    })
}

pub fn f135(v0: Adt14, v1: List<Adt2>) -> List<Adt2> {
    let m721 = v1;
    match m721.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            if if f225(v2.clone(), f8(Str::lit("subject"))) {
                f226(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f135(v0, v3))
            } else {
                f135(v0, v3)
            }
        }
    }
}

pub fn f136(v0: Adt1, v1: List<Bytes>) -> R<List<Adt2>> {
    let m722 = v1;
    match m722.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let m723 = f227(f96(f64(v0.clone(), v2.clone()))?, f86(v2, f216()))?;
            match m723 {
                None => {
                    f136(v0, v3)
                }
                Some(v4) => {
                    Ok(f228(v4, f136(v0, v3)?))
                }
            }
        }
    }
}

pub fn f137(v0: Adt1) -> R<List<Bytes>> {
    Ok(f126(f146(f229(f215(), f148(match v0.clone() {
        Adt1::C0(_, h724, _, _) => {
            h724
        }
    }, f215())))?, f146(f229(f216(), f148(match v0 {
        Adt1::C0(_, h725, _, _) => {
            h725
        }
    }, f216())))?))
}

pub fn f138(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("-c")), List::cons(f230(), List::cons(f8(Str::lit("prism-code")), List::cons(f8(Str::lit("UOR-Foundation/prismpm")), List::cons(f231(), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil())))))))))
}

pub fn f139(p729: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m728 = p729;
    {
        let mut m728 = m728;
        loop {
            let step = {
                let v0 = m728;
                let m726 = v0;
                match m726.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h727, _) = v2;
                            h727
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m728 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f141(v0: Adt1) -> R<List<Bytes>> {
    f146(f229(f140(), f148(match v0 {
        Adt1::C0(_, h730, _, _) => {
            h730
        }
    }, f140())))
}

pub fn f142(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m731 = v1;
    match m731.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f126(if f52(v4.clone(), v3.clone()) {
                f232(v4.clone(), f96(f64(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f233(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
            }, f142(v0, v5, v2, v3)?))
        }
    }
}

pub fn f143(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m732 = v0;
    match m732.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f143(v3, v1))
        }
    }
}

pub fn f144(v0: Adt1) -> R<Adt12> {
    let m733 = f56(v0.clone(), f234());
    match m733.uncons() {
        None => {
            let m734 = f235(v0.clone(), f234())?;
            match m734.uncons() {
                None => {
                    let m737 = f236(match v0.clone() {
                        Adt1::C0(_, h735, _, _) => {
                            h735
                        }
                    }, f126(f237(), f238(match v0.clone() {
                        Adt1::C0(_, h736, _, _) => {
                            h736
                        }
                    }, false)));
                    match m737.uncons() {
                        None => {
                            let m739 = f56(v0.clone(), f238(match v0.clone() {
                                Adt1::C0(_, h738, _, _) => {
                                    h738
                                }
                            }, true));
                            match m739.uncons() {
                                None => {
                                    let m740 = f56(v0.clone(), f239());
                                    match m740.uncons() {
                                        None => {
                                            let m741 = f240(v0.clone())?;
                                            match m741 {
                                                None => {
                                                    let m743 = f241(v0.clone(), f238(match v0.clone() {
                                                        Adt1::C0(_, h742, _, _) => {
                                                            h742
                                                        }
                                                    }, true))?;
                                                    match m743 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f242(Adt16::C0(Adt15::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f218(v0.clone())?, f143(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f219(f97(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f238(match v0 {
                                                                Adt1::C0(_, h744, _, _) => {
                                                                    h744
                                                                }
                                                            }, true)))?;
                                                            let (h745, _) = v13;
                                                            if h745 {
                                                                Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h746) = v13;
                                                                    h746
                                                                }))
                                                            } else {
                                                                Ok(Adt12::C4({
                                                                    let (_, h747) = v13;
                                                                    h747
                                                                }))
                                                            }
                                                        }
                                                        Some(v12) => {
                                                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: classical choice is an assumption, and ")), List::cons(v12, List::cons(f8(Str::lit(" depends on it\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                    }
                                                }
                                                Some(v11) => {
                                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: a negated guard in a generated workflow is one command, because `if !` negates only the first command of a chain: ")), List::cons(v11, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                                }
                                            }
                                        }
                                        Some((v9, v10)) => {
                                            Ok(Adt12::C0(List::cons(v9, v10)))
                                        }
                                    }
                                }
                                Some((v7, v8)) => {
                                    Ok(Adt12::C0(List::cons(v7, v8)))
                                }
                            }
                        }
                        Some((v5, v6)) => {
                            Ok(Adt12::C1(List::cons(v5, v6)))
                        }
                    }
                }
                Some((v3, _)) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("register is not JSON: ")), List::cons(v3, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt12::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f145() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f146(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f243(f121(Fn1::F244, List::<Bytes>::nil(), f245(v0)?)))
}

pub fn f147(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m748 = v1;
    match m748.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f246(v0.clone(), v2), f147(v0, v3))
            } else {
                f147(v0, v3)
            }
        }
    }
}

pub fn f148(p753: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m752 = p753;
    {
        let mut m752 = m752;
        loop {
            let step = {
                let v0 = m752;
                let m749 = v0;
                match m749.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h750, _) = v2;
                            h750
                        }, v1.clone()) {
                            let (_, h751) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h751)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
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

pub fn f149(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p756: List<Bytes>) -> R<Option<Bytes>> {
    let m755 = p756;
    {
        let mut m755 = m755;
        loop {
            let step = {
                let v2 = m755;
                let m754 = v2;
                match m754.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f247(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
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

pub fn f150(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m757 = v0;
    match m757.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f248(v1.clone())?, v1), f150(v2)?))
        }
    }
}

pub fn f151(p760: List<Bytes>, p761: List<Bytes>, p762: List<List<Bytes>>) -> List<List<Bytes>> {
    let m759 = (p760, (p761, p762));
    {
        let mut m759 = m759;
        loop {
            let step = {
                let (v0, (v1, v2)) = m759;
                let m758 = v0;
                match m758.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f249(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f249(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m759 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m763 = v0;
    match m763.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f152(v3, v1)
            } else {
                List::cons(v2, f152(v3, v1))
            }
        }
    }
}

pub fn f153(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f139(match v0.clone() {
        Adt1::C0(_, h764, _, _) => {
            h764
        }
    }, f250())) {
        Ok(Adt12::C1(List::cons(f250(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f152(f146(f147(f250(), f148(match v0.clone() {
            Adt1::C0(_, h765, _, _) => {
                h765
            }
        }, f250())))?, v1.clone());
        let m766 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m766.uncons() {
            None => {
                let v5: List<Bytes> = f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m767 = f56(v0.clone(), f251(v0.clone(), v2.clone(), v5.clone())?);
                match m767.uncons() {
                    None => {
                        let m768 = f252(v0.clone(), v2, v5)?;
                        match m768 {
                            None => {
                                f253(v0, v1)
                            }
                            Some(v8) => {
                                Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("admit-encoding ")), List::cons(v8, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                            }
                        }
                    }
                    Some((v6, v7)) => {
                        Ok(Adt12::C0(List::cons(v6, v7)))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt12::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f154() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f155(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m769 = v0;
    match m769.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f155(v3, v1))
        }
    }
}

pub fn f156(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m770 = f97(v0.clone(), f154())?;
    match m770.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f254(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f157() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f158() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}            target/release/verdant_xml\u{a}            target/release/verdant_admit\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      # Restored on every run, not only when certified/ changed: the end-to-end propose-command step below\u{a}      # exports with the attested lexlean binary, and a fresh cargo install does not reproduce its hash.\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers certified-xml certified-admit)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/, certified-readers/, certified-xml/ or certified-admit/:\"\u{a}            git status --short -- certified certified-readers certified-xml certified-admit | head -20\u{a}            exit 1\u{a}          fi\u{a}      # verify-artifact replays a bundle but does not compare it with the sources. So each LexLean project's\u{a}      # semantic id must equal the one its bundle records. Lock-level ids move with every admission to app/,\u{a}      # which readers depends on without using it, so they are not compared.\u{a}      - name: the certified bundles were exported from these sources (semantic ids, no Lean)\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The lexlean cache is keyed by .github/lexlean.pin, so after the pin moves the replay above may have been skipped with no binary restored.\u{a}          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          for pair in studio:certified readers:certified-readers readers/xml:certified-xml admit:certified-admit; do\u{a}            project=$(cut -d : -f 1 <<< \"$pair\")\u{a}            bundle=$(cut -d : -f 2 <<< \"$pair\")\u{a}            if [ ! -f \"$project/lexlean.toml\" ]; then continue; fi\u{a}            if [ ! -f \"$bundle/result.json\" ]; then echo \"::warning::$project has no committed bundle $bundle, so nothing records what it was exported from\"; continue; fi\u{a}            now=$(cd \"$project\" && \"$ll\" lock >/dev/null && \"$ll\" check | sed -n 's/.*semantic \\([0-9a-f]*\\)).*/\\1/p')\u{a}            git checkout -q -- \"$project/lexlean.lock\"\u{a}            was=$(jq -r .source.semantic_id \"$bundle/result.json\")\u{a}            if [ \"$now\" != \"$was\" ]; then echo \"::error::$project now has semantic id $now, but $bundle was exported from $was: export it again through propose-command\"; exit 1; fi\u{a}            echo \"$project matches $bundle (semantic $now)\"\u{a}          done\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          # Each encoding admitted through admit-translated recorded the reader command that printed it: run it again and fail if the output differs.\u{a}          target/release/verdant_readers verify-translations model/admissions.json\u{a}          if [ -x target/release/verdant_xml ]; then target/release/verdant_xml verify-translations model/admissions.json; fi\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept the Prism of the seed Verdant Studio, every facet at its media type and its generator's fidelity\u{a}        run: python3 .github/scripts/prism-acceptance.py node target/prismpm/sdk/npm/prism-code/bin/prism-code.mjs\u{a}      - name: the workspace API's operations are MCP tools valid against the bound MCP schema (Studio.Surface)\u{a}        run: python3 .github/scripts/surface-check.py\u{a}      - name: the Studio UI Prism's view tree is published as MCP resources valid against the bound MCP schema (Studio.Agent.Interface.Resource)\u{a}        run: python3 .github/scripts/gui-tree.py node target/prismpm/sdk/npm/prism-code/bin/prism-code.mjs\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          # certified-xml/ and certified-admit/ join the comparison once main holds them: the export that first writes it is the one that adds it.\u{a}          bundles=\"certified certified-readers\"\u{a}          if [ -n \"$(git ls-files certified-xml | head -n 1)\" ]; then bundles=\"$bundles certified-xml\"; fi\u{a}          if [ -n \"$(git ls-files certified-admit | head -n 1)\" ]; then bundles=\"$bundles certified-admit\"; fi\u{a}          if [ -n \"$(git status --porcelain -- $bundles)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- $bundles | head -20\u{a}            attested=$(git show HEAD:certified/evidence/attestation.json | jq -c .lexlean); used=$(jq -c .lexlean certified/evidence/attestation.json)\u{a}            [ \"$attested\" = \"$used\" ] || echo \"::error::the export ran with lexlean $used, but certified/ attests lexlean $attested. lexlean is built from .github/lexlean.pin with rust 1.97.0 and its cargo home remapped, so a different executable means the toolchain or the runner image changed\"\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f159() -> Bytes {
    f7(List::cons(f255(), List::cons(f256(), List::cons(f257(), List::cons(f258(), List::cons(f259(), List::cons(f260(), List::<Bytes>::nil())))))))
}

pub fn f160() -> Bytes {
    f7(List::cons(f261(), List::cons(f262(), List::cons(f263(), List::cons(f264(), List::<Bytes>::nil())))))
}

pub fn f161() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports under its own limits: studio, readers, admit and readers/xml when they exist, largest first.\u{a}# Lean's proof of a project grows with the square of its function count, so the readers are split by family and admission is its own project.\u{a}# Each export runs two Lean phases at once, so at most half as many exports as cores run together: more slows every phase.\u{a}slots=$(( $(nproc) / 2 ))\u{a}if [ \"$slots\" -lt 1 ]; then slots=1; fi\u{a}running=0\u{a}exported=' '\u{a}for project in studio readers admit readers/xml; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    # A project whose semantic id is the one its committed bundle was exported from, under the same lexlean, is not\u{a}    # exported again: the studio check compares exactly these ids, so its bundle stays the bundle of its sources.\u{a}    case \"$project\" in studio) bundle=certified;; readers/xml) bundle=certified-xml;; *) bundle=\"certified-$project\";; esac\u{a}    if [ -f \"$bundle/result.json\" ] && git diff --quiet HEAD -- .github/lexlean.pin; then\u{a}      now=$(cd \"$project\" && \"$ll\" lock >/dev/null && \"$ll\" check | sed -n 's/.*semantic \\([0-9a-f]*\\)).*/\\1/p')\u{a}      if [ -n \"$now\" ] && [ \"$now\" = \"$(jq -r .source.semantic_id \"$bundle/result.json\")\" ]; then\u{a}        echo \"$project is unchanged since $bundle was exported (semantic $now), so it is not exported again\"\u{a}        continue\u{a}      fi\u{a}    fi\u{a}    exported=\"$exported$project \"\u{a}    if [ \"$running\" -ge \"$slots\" ]; then wait -n || status=$?; running=$((running - 1)); fi\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.${project//\\//-}\" 2>&1 &\u{a}    running=$((running + 1))\u{a}  fi\u{a}done\u{a}while [ \"$running\" -gt 0 ]; do wait -n || status=$?; running=$((running - 1)); done\u{a}: > \"$log\"\u{a}for part in studio readers readers-xml admit; do\u{a}  if [ -f \"$log.$part\" ]; then cat \"$log.$part\" >> \"$log\"; fi\u{a}done\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}if [[ \"$exported\" == *\" studio \"* ]]; then\u{a}  rm -rf certified\u{a}  cp -r studio/.lexlean/production/*/ certified\u{a}  rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified\u{a}fi\u{a}if [[ \"$exported\" == *\" readers \"* ]]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}if [[ \"$exported\" == *\" readers/xml \"* ]]; then\u{a}  rm -rf certified-xml\u{a}  cp -r readers/xml/.lexlean/production/*/ certified-xml\u{a}  rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-xml\u{a}fi\u{a}if [[ \"$exported\" == *\" admit \"* ]]; then\u{a}  rm -rf certified-admit\u{a}  cp -r admit/.lexlean/production/*/ certified-admit\u{a}  rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-admit\u{a}fi\u{a}"))
}

pub fn f162() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env bash\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# A change to a reader can change what it prints for an encoding already admitted, and the studio check\u{a}")), List::cons(f8(Str::lit("# replays every admitted reader command (verify-translations). So the change re-derives those encodings\u{a}")), List::cons(f8(Str::lit("# itself: each Catalog encoding whose replay now differs is admitted again through read-admit, with the\u{a}")), List::cons(f8(Str::lit("# command and citation its admission row records, and the pull request carries both (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("# It prints each construct it admitted again, one per line.\u{a}")), List::cons(f8(Str::lit("set -euo pipefail\u{a}")), List::cons(f8(Str::lit("readers=$1\u{a}")), List::cons(f8(Str::lit("mkdir -p target\u{a}")), List::cons(f8(Str::lit("rows=$(jq length model/admissions.json)\u{a}")), List::cons(f8(Str::lit("for try in $(seq 1 \"$rows\"); do\u{a}")), List::cons(f8(Str::lit("  if \"$readers\" verify-translations model/admissions.json > target/replay.txt 2>&1; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("  construct=$(sed -n 's/^\\([^:]*\\): its reader command now prints .*/\\1/p' target/replay.txt | head -n 1)\u{a}")), List::cons(f8(Str::lit("  case \"$construct\" in\u{a}")), List::cons(f8(Str::lit("    Catalog.*) ;;\u{a}")), List::cons(f8(Str::lit("    *) cat target/replay.txt >&2; echo \"readmit: only a Catalog encoding whose reader now prints other output is admitted again\" >&2; exit 1;;\u{a}")), List::cons(f8(Str::lit("  esac\u{a}")), List::cons(f8(Str::lit("  row=$(jq -c --arg construct \"$construct\" '[.[] | select(.construct == $construct and .translation != null)][0]' model/admissions.json)\u{a}")), List::cons(f8(Str::lit("  authority=$(jq -r '.std.authority // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  cited=$(jq -r '.std.row // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  if [ -z \"$authority\" ]; then echo \"readmit: $construct cites no std authority, so read-admit cannot cite it again\" >&2; exit 1; fi\u{a}")), List::cons(f8(Str::lit("  mapfile -t command < <(jq -r '.translation.command[]' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  \"$readers\" read-admit \"${construct#Catalog.}\" \"${command[@]}\" std \"$authority\" $cited >/dev/null\u{a}")), List::cons(f8(Str::lit("  echo \"$construct\"\u{a}")), List::cons(f8(Str::lit("done\u{a}")), List::cons(f8(Str::lit("echo \"readmit: the replay did not settle after $rows admissions\" >&2\u{a}")), List::cons(f8(Str::lit("exit 1\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))
}

pub fn f163() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# app/src/Contract.lex.tex and model/strata.json each keep their entries on one long line, so two changes that\u{a}")), List::cons(f8(Str::lit("# each change entries there always conflict, whatever the entries. A change that only adds, removes or replaces\u{a}")), List::cons(f8(Str::lit("# entries is merged into main's file by entry instead: Contract's importmodule lines, dispatch routes and\u{a}")), List::cons(f8(Str::lit("# acceptance vectors, and the strata rows keyed by path (VS-05). Anything else is left to git apply.\u{a}")), List::cons(f8(Str::lit("#   merge BASE NEW MAIN PATH  prints NEW's entry changes merged into MAIN, or exits 1 when NEW changes more.\u{a}")), List::cons(f8(Str::lit("#   patch FILE                merges each of the two files a patch changes that way, and leaves the rest in FILE.\u{a}")), List::cons(f8(Str::lit("import json, os, re, subprocess, sys, tempfile\u{a}")), List::cons(f8(Str::lit("CONTRACT = 'app/src/Contract.lex.tex'\u{a}")), List::cons(f8(Str::lit("STRATA = 'model/strata.json'\u{a}")), List::cons(f8(Str::lit("NIL = '(nil Foundation.View.Text.V1.Model::AcceptanceVector)'\u{a}")), List::cons(f8(Str::lit("ROUTE = re.compile(r'[(]if [(]routed value (0x[0-9a-f]+)[)] then [(][A-Za-z0-9_.]+::[A-Za-z0-9_]+ [(]routedValue value \\1[)][)] else ')\u{a}")), List::cons(f8(Str::lit("VECTOR = re.compile(r'[(]cons [(]record Foundation[.]View[.]Text[.]V1[.]Model::AcceptanceVector [{]request := 0x[0-9a-f]*, response := 0x[0-9a-f]*[}][)] ')\u{a}")), List::cons(f8(Str::lit("IMPORT = re.compile(r'(?m)^[\\\\]importmodule[{][^}]+[}]$')\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def entries(text):\u{a}")), List::cons(f8(Str::lit("    return IMPORT.findall(text), [m.group(0) for m in ROUTE.finditer(text)], VECTOR.findall(text)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def dispatch_line(text):\u{a}")), List::cons(f8(Str::lit("    at = text.index('body: ', text.index('name: dispatch')) + 6\u{a}")), List::cons(f8(Str::lit("    return at, text.index(chr(10), at)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def skeleton(text):\u{a}")), List::cons(f8(Str::lit("    text = re.sub('[' + chr(10) + ']+', chr(10), IMPORT.sub('', ROUTE.sub('', VECTOR.sub('', text))))\u{a}")), List::cons(f8(Str::lit("    at, end = dispatch_line(text)\u{a}")), List::cons(f8(Str::lit("    line = text[at:end].rstrip(',').rstrip(')')\u{a}")), List::cons(f8(Str::lit("    text = text[:at] + line + text[end:]\u{a}")), List::cons(f8(Str::lit("    n = text.index(NIL) + len(NIL)\u{a}")), List::cons(f8(Str::lit("    return text[:n] + text[n:].lstrip(')')\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def drop(text, gone):\u{a}")), List::cons(f8(Str::lit("    for entry in gone:\u{a}")), List::cons(f8(Str::lit("        text = text.replace(entry, '', 1) if not entry.startswith(chr(92)) else text.replace(entry + chr(10), '', 1)\u{a}")), List::cons(f8(Str::lit("    return text\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def edit_contract(text, removed, added):\u{a}")), List::cons(f8(Str::lit("    imports, routes, vectors = added\u{a}")), List::cons(f8(Str::lit("    at, end = dispatch_line(text)\u{a}")), List::cons(f8(Str::lit("    line = drop(text[at:end], removed[1])\u{a}")), List::cons(f8(Str::lit("    tail = ',' if line.endswith(',') else ''\u{a}")), List::cons(f8(Str::lit("    line = line[:len(line) - len(tail)]\u{a}")), List::cons(f8(Str::lit("    line = ''.join(routes) + line[:len(line) - len(removed[1])] + ')' * len(routes) + tail\u{a}")), List::cons(f8(Str::lit("    text = drop(drop(text[:at] + line + text[end:], removed[2]), removed[0])\u{a}")), List::cons(f8(Str::lit("    if imports:\u{a}")), List::cons(f8(Str::lit("        last = list(IMPORT.finditer(text))[-1].end()\u{a}")), List::cons(f8(Str::lit("        text = text[:last] + ''.join(chr(10) + i for i in imports) + text[last:]\u{a}")), List::cons(f8(Str::lit("    n = text.index(NIL)\u{a}")), List::cons(f8(Str::lit("    after = text[n + len(NIL):]\u{a}")), List::cons(f8(Str::lit("    if not after.startswith(')' * len(removed[2])):\u{a}")), List::cons(f8(Str::lit("        raise ValueError('the vectors are not closed after their list')\u{a}")), List::cons(f8(Str::lit("    return text[:n] + ''.join(vectors) + NIL + ')' * len(vectors) + after[len(removed[2]):]\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def merge_contract(base, new, main):\u{a}")), List::cons(f8(Str::lit("    if skeleton(base) != skeleton(new) or skeleton(base) != skeleton(main):\u{a}")), List::cons(f8(Str::lit("        return None\u{a}")), List::cons(f8(Str::lit("    b, n, m = entries(base), entries(new), entries(main)\u{a}")), List::cons(f8(Str::lit("    removed = [[e for e in old if e not in now and e in have] for old, now, have in zip(b, n, m)]\u{a}")), List::cons(f8(Str::lit("    added = [[e for e in now if e not in old and e not in have] for old, now, have in zip(b, n, m)]\u{a}")), List::cons(f8(Str::lit("    return edit_contract(main, removed, added)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def merge_strata(base, new, main):\u{a}")), List::cons(f8(Str::lit("    b, n, m = json.loads(base), json.loads(new), json.loads(main)\u{a}")), List::cons(f8(Str::lit("    kept = [r for r in m if r in n or r not in b]\u{a}")), List::cons(f8(Str::lit("    have = set(r['path'] for r in kept)\u{a}")), List::cons(f8(Str::lit("    rows = kept + [r for r in n if r not in b and r['path'] not in have]\u{a}")), List::cons(f8(Str::lit("    for ascii in (True, False):\u{a}")), List::cons(f8(Str::lit("        if json.dumps(m, separators=(',', ':'), ensure_ascii=ascii) + chr(10) == main:\u{a}")), List::cons(f8(Str::lit("            return json.dumps(rows, separators=(',', ':'), ensure_ascii=ascii) + chr(10)\u{a}")), List::cons(f8(Str::lit("    return None\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def merge(base, new, main, path):\u{a}")), List::cons(f8(Str::lit("    return merge_contract(base, new, main) if path == CONTRACT else merge_strata(base, new, main)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def blob(sha):\u{a}")), List::cons(f8(Str::lit("    got = subprocess.run(['git', 'cat-file', '-p', sha], capture_output=True, text=True)\u{a}")), List::cons(f8(Str::lit("    if got.returncode == 0:\u{a}")), List::cons(f8(Str::lit("        return got.stdout\u{a}")), List::cons(f8(Str::lit("    return subprocess.run(['gh', 'api', '-H', 'Accept: application/vnd.github.raw', 'repos/' + os.environ['GITHUB_REPOSITORY'] + chr(47) + 'git/blobs/' + sha], capture_output=True, text=True, check=True).stdout\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def changed(path, base, section):\u{a}")), List::cons(f8(Str::lit("    with tempfile.TemporaryDirectory() as work:\u{a}")), List::cons(f8(Str::lit("        subprocess.run(['git', 'init', '-q', work], check=True)\u{a}")), List::cons(f8(Str::lit("        os.makedirs(os.path.join(work, os.path.dirname(path)), exist_ok=True)\u{a}")), List::cons(f8(Str::lit("        open(os.path.join(work, path), 'w').write(base)\u{a}")), List::cons(f8(Str::lit("        subprocess.run(['git', 'apply', '-'], cwd=work, input=section, text=True, check=True)\u{a}")), List::cons(f8(Str::lit("        return open(os.path.join(work, path)).read()\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def patch(name):\u{a}")), List::cons(f8(Str::lit("    text = open(name).read()\u{a}")), List::cons(f8(Str::lit("    parts = re.split('(?m)^(?=diff --git )', text)\u{a}")), List::cons(f8(Str::lit("    kept = [parts[0]]\u{a}")), List::cons(f8(Str::lit("    for section in parts[1:]:\u{a}")), List::cons(f8(Str::lit("        path = section.split(chr(10), 1)[0].split(' ')[-1][2:]\u{a}")), List::cons(f8(Str::lit("        index = re.search('(?m)^index ([0-9a-f]{40})[.][.]', section)\u{a}")), List::cons(f8(Str::lit("        merged = None\u{a}")), List::cons(f8(Str::lit("        if path in (CONTRACT, STRATA) and index:\u{a}")), List::cons(f8(Str::lit("            try:\u{a}")), List::cons(f8(Str::lit("                base = blob(index.group(1))\u{a}")), List::cons(f8(Str::lit("                merged = merge(base, changed(path, base, section), open(path).read(), path)\u{a}")), List::cons(f8(Str::lit("            except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as failure:\u{a}")), List::cons(f8(Str::lit("                print('::warning::' + path + ' is left to git apply: ' + str(failure).splitlines()[0])\u{a}")), List::cons(f8(Str::lit("        if merged is None:\u{a}")), List::cons(f8(Str::lit("            kept.append(section)\u{a}")), List::cons(f8(Str::lit("        else:\u{a}")), List::cons(f8(Str::lit("            open(path, 'w').write(merged)\u{a}")), List::cons(f8(Str::lit("            print('::notice::merged the entries this change adds to ' + path + ' into main by entry')\u{a}")), List::cons(f8(Str::lit("    open(name, 'w').write(''.join(kept))\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("if sys.argv[1] == 'merge':\u{a}")), List::cons(f8(Str::lit("    merged = merge(*[open(p).read() for p in sys.argv[2:5]], sys.argv[5])\u{a}")), List::cons(f8(Str::lit("    if merged is None:\u{a}")), List::cons(f8(Str::lit("        sys.exit(1)\u{a}")), List::cons(f8(Str::lit("    sys.stdout.write(merged)\u{a}")), List::cons(f8(Str::lit("else:\u{a}")), List::cons(f8(Str::lit("    patch(sys.argv[2])\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f164() -> Bytes {
    f7(List::cons(f265(), List::cons(f266(), List::cons(f267(), List::<Bytes>::nil()))))
}

pub fn f165() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# Parallel lanes each register the next VS IDs they see, so two changes can claim the same ones, and each\u{a}")), List::cons(f8(Str::lit("# appends its rows at the end of model/ids.toml and of a suite. So each VS ID a change adds that main already\u{a}")), List::cons(f8(Str::lit("# has becomes the next free one throughout the change, and a file the change only appends to is appended to on\u{a}")), List::cons(f8(Str::lit("# main instead of patched, so a change made against an older main still applies. propose-command and refresh\u{a}")), List::cons(f8(Str::lit("# both take this path.\u{a}")), List::cons(f8(Str::lit("#   prepare PATCH APPENDS  renumbers PATCH in place and moves its pure appends into APPENDS\u{a}")), List::cons(f8(Str::lit("#   append APPENDS         appends them, after the rest of the patch is applied\u{a}")), List::cons(f8(Str::lit("#   lint PATCH             refuses an added string literal the Lean export would refuse as an absolute path\u{a}")), List::cons(f8(Str::lit("import re, json, sys\u{a}")), List::cons(f8(Str::lit("nl = chr(10)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def prepare(name, appends_name):\u{a}")), List::cons(f8(Str::lit("    text = open(name).read()\u{a}")), List::cons(f8(Str::lit("    parts = re.split('(?m)^(?=diff --git )', text)\u{a}")), List::cons(f8(Str::lit("    head, files = parts[0], parts[1:]\u{a}")), List::cons(f8(Str::lit("    have = set(int(n) for n in re.findall('(?m)^id = .VS-([0-9]+).', open('model/ids.toml').read()))\u{a}")), List::cons(f8(Str::lit("    added = []\u{a}")), List::cons(f8(Str::lit("    for f in files:\u{a}")), List::cons(f8(Str::lit("        added += [int(n) for n in re.findall('(?m)^[+]id = .VS-([0-9]+).', f)]\u{a}")), List::cons(f8(Str::lit("    top = max(have | set(added) | {0})\u{a}")), List::cons(f8(Str::lit("    ren = {}\u{a}")), List::cons(f8(Str::lit("    for n in added:\u{a}")), List::cons(f8(Str::lit("        if n in have and n not in ren:\u{a}")), List::cons(f8(Str::lit("            top += 1\u{a}")), List::cons(f8(Str::lit("            ren[n] = top\u{a}")), List::cons(f8(Str::lit("    def fix(line):\u{a}")), List::cons(f8(Str::lit("        return re.sub('VS-([0-9]+)(?![0-9])', lambda m: 'VS-' + str(ren[int(m.group(1))]).zfill(2) if int(m.group(1)) in ren else m.group(0), line)\u{a}")), List::cons(f8(Str::lit("    appends = {}\u{a}")), List::cons(f8(Str::lit("    kept = []\u{a}")), List::cons(f8(Str::lit("    for f in files:\u{a}")), List::cons(f8(Str::lit("        lines = [fix(l) if l.startswith('+') and not l.startswith('+++') else l for l in f.split(nl)]\u{a}")), List::cons(f8(Str::lit("        hunks = [i for i, l in enumerate(lines) if l.startswith('@@')]\u{a}")), List::cons(f8(Str::lit("        # The last hunk is a pure append when it removes nothing and ends on an added line, so no\u{a}")), List::cons(f8(Str::lit("        # context follows it: it reaches the end of the file. Any hunks before it are patched as usual.\u{a}")), List::cons(f8(Str::lit("        body = [l for l in lines[hunks[-1] + 1:] if l != ''] if hunks else []\u{a}")), List::cons(f8(Str::lit("        plain = 'new file mode' not in f and 'GIT binary patch' not in f\u{a}")), List::cons(f8(Str::lit("        if plain and body and body[-1].startswith('+') and not any(l[:1] in '-' + chr(92) for l in body):\u{a}")), List::cons(f8(Str::lit("            appends[lines[0].split(' ')[2][2:]] = [l[1:] for l in body if l.startswith('+')]\u{a}")), List::cons(f8(Str::lit("            if len(hunks) > 1:\u{a}")), List::cons(f8(Str::lit("                kept.append(nl.join(lines[:hunks[-1]]) + nl)\u{a}")), List::cons(f8(Str::lit("        else:\u{a}")), List::cons(f8(Str::lit("            kept.append(nl.join(lines))\u{a}")), List::cons(f8(Str::lit("    open(name, 'w').write(head + ''.join(kept))\u{a}")), List::cons(f8(Str::lit("    json.dump(appends, open(appends_name, 'w'))\u{a}")), List::cons(f8(Str::lit("    for n in sorted(ren):\u{a}")), List::cons(f8(Str::lit("        print('::notice::VS-' + str(n).zfill(2) + ' is already on main, so this change registers it as VS-' + str(ren[n]).zfill(2))\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def append(appends_name):\u{a}")), List::cons(f8(Str::lit("    for path, add in json.load(open(appends_name)).items():\u{a}")), List::cons(f8(Str::lit("        old = open(path).read()\u{a}")), List::cons(f8(Str::lit("        if old and not old.endswith(nl):\u{a}")), List::cons(f8(Str::lit("            old += nl\u{a}")), List::cons(f8(Str::lit("        open(path, 'w').write(old + nl.join(add) + nl)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def rooted(text):\u{a}")), List::cons(f8(Str::lit("    # LexLean's own test for an absolute path in Lean's output (first_unexpected_absolute_path in its verifier),\u{a}")), List::cons(f8(Str::lit("    # run over each added string literal as written, which is how Lean prints it: escapes stay escaped, so a line\u{a}")), List::cons(f8(Str::lit("    # ending in a one-letter name and a colon reads as a Windows path.\u{a}")), List::cons(f8(Str::lit("    slash, colon, backslash = chr(47), chr(58), chr(92)\u{a}")), List::cons(f8(Str::lit("    boundary = lambda char: char.isspace() or char in chr(34) + chr(39) + chr(96) + '([{<,;='\u{a}")), List::cons(f8(Str::lit("    segment = lambda char: char.isalnum() or char in '_.'\u{a}")), List::cons(f8(Str::lit("    for at, char in enumerate(text):\u{a}")), List::cons(f8(Str::lit("        before = at == 0 or boundary(text[at - 1])\u{a}")), List::cons(f8(Str::lit("        ahead = text[at + 1:at + 4]\u{a}")), List::cons(f8(Str::lit("        posix = char == slash and before and len(ahead) > 0 and segment(ahead[0])\u{a}")), List::cons(f8(Str::lit("        windows = char.isascii() and char.isalpha() and before and len(ahead) == 3 and ahead[0] == colon and ahead[1] in slash + backslash and segment(ahead[2])\u{a}")), List::cons(f8(Str::lit("        if posix or windows:\u{a}")), List::cons(f8(Str::lit("            return text[at:at + 40]\u{a}")), List::cons(f8(Str::lit("    return None\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def lint(patch_name):\u{a}")), List::cons(f8(Str::lit("    found = []\u{a}")), List::cons(f8(Str::lit("    for line in open(patch_name).read().split(nl):\u{a}")), List::cons(f8(Str::lit("        if line.startswith('+') and not line.startswith('+++'):\u{a}")), List::cons(f8(Str::lit("            for literal in re.findall(chr(34) + '((?:[^' + chr(34) + chr(92) * 2 + ']|' + chr(92) * 2 + '.)*)' + chr(34), line):\u{a}")), List::cons(f8(Str::lit("                hit = rooted(literal)\u{a}")), List::cons(f8(Str::lit("                if hit is not None:\u{a}")), List::cons(f8(Str::lit("                    found.append(hit)\u{a}")), List::cons(f8(Str::lit("    for hit in found:\u{a}")), List::cons(f8(Str::lit("        print('::error::LLV7006: an added string literal holds ' + repr(hit) + ', which the Lean export refuses as an absolute path')\u{a}")), List::cons(f8(Str::lit("    return 1 if found else 0\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("if sys.argv[1] == 'prepare':\u{a}")), List::cons(f8(Str::lit("    prepare(sys.argv[2], sys.argv[3])\u{a}")), List::cons(f8(Str::lit("elif sys.argv[1] == 'lint':\u{a}")), List::cons(f8(Str::lit("    sys.exit(lint(sys.argv[2]))\u{a}")), List::cons(f8(Str::lit("else:\u{a}")), List::cons(f8(Str::lit("    append(sys.argv[2])\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f166() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f167() -> Bytes {
    f8(Str::lit("--git https://github.com/UOR-Foundation/lexlean --rev 82a32815c45762ea8931c84cb9465a5aac902866\u{a}"))
}

pub fn f168(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f128(v0.clone())?;
    let v2: List<Adt2> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt2> = f135(v1.clone(), f136(v0.clone(), f137(v0.clone())?)?);
    let v4: List<List<Bytes>> = f268(f269(f64(v0, f217()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f270(v4.clone(), f271(v4), f222(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f134(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f169(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f272(f128(v0.clone())?, f136(v0.clone(), f137(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f170(v0: Adt1) -> R<Bytes> {
    let v1: List<List<Bytes>> = f268(f269(f64(v0.clone(), f217()))?);
    let v2: List<(Bytes, Bytes)> = f273(f271(v1.clone()), f136(v0.clone(), f137(v0)?)?);
    let v3: Bytes = f7(f274(v1, 0u64, 26u64)?);
    let v4: Bytes = f7(f146(f275(v2.clone(), v2)?)?);
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Studio.Prism.Closure.Register}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}definition{\u{a}  name: cells,\u{a}  parameters: [],\u{a}  result: Bytes,\u{a}  body: 0x")), List::cons(f276(v3.clone(), 0u64, {
        let a771 = v3;
        length_bytes(a771)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\u{a}}\u{a}\u{a}definition{\u{a}  name: pairs,\u{a}  parameters: [],\u{a}  result: Bytes,\u{a}  body: 0x")), List::cons(f276(v4.clone(), 0u64, {
        let a772 = v4;
        length_bytes(a772)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\u{a}}\u{a}\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f171(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f277(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f278(f97(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f172() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f173() -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"_meta\":{\"io.github.afflom/instance\":\"verdant-studio\"},\"cacheScope\":\"public\",\"resultType\":\"complete\",\"ttlMs\":0,\"tools\":[")), List::cons(f123(f279(f124())?, f8(Str::lit(","))), List::cons(f8(Str::lit(",")), List::cons(f123(f280(f281())?, f8(Str::lit(","))), List::cons(f8(Str::lit("]}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f174() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# Studio.Surface (REQ-AGENT-INTERFACE): the workspace API's operations, published as MCP Tool definitions in\u{a}")), List::cons(f8(Str::lit("# surface/mcp-tools.json, must be a ListToolsResult of the bound MCP-SCHEMA-2026-07-28. This validates it against\u{a}")), List::cons(f8(Str::lit("# that schema's own definitions (type, required, properties, items, enum, const, additionalProperties, $ref), and\u{a}")), List::cons(f8(Str::lit("# requires every tool's inputSchema to be a JSON Schema object, one tool for each operation the usage lists.\u{a}")), List::cons(f8(Str::lit("import json, sys\u{a}")), List::cons(f8(Str::lit("TYPES = {'object': dict, 'array': list, 'string': str, 'boolean': bool, 'null': type(None)}\u{a}")), List::cons(f8(Str::lit("schema = json.load(open('authorities' + chr(47) + 'MCP-SCHEMA-2026-07-28' + chr(47) + 'schema.json'))\u{a}")), List::cons(f8(Str::lit("defs = schema['$defs']\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def numeric(value):\u{a}")), List::cons(f8(Str::lit("    return isinstance(value, (int, float)) and not isinstance(value, bool)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def check(value, rule, where, problems):\u{a}")), List::cons(f8(Str::lit("    if not isinstance(rule, dict):\u{a}")), List::cons(f8(Str::lit("        return\u{a}")), List::cons(f8(Str::lit("    if '$ref' in rule:\u{a}")), List::cons(f8(Str::lit("        check(value, defs[rule['$ref'].split(chr(47))[-1]], where, problems)\u{a}")), List::cons(f8(Str::lit("    kinds = rule.get('type')\u{a}")), List::cons(f8(Str::lit("    if kinds is not None:\u{a}")), List::cons(f8(Str::lit("        kinds = kinds if isinstance(kinds, list) else [kinds]\u{a}")), List::cons(f8(Str::lit("        fits = any((kind in ('number', 'integer') and numeric(value)) or (kind in TYPES and isinstance(value, TYPES[kind])) for kind in kinds)\u{a}")), List::cons(f8(Str::lit("        if not fits:\u{a}")), List::cons(f8(Str::lit("            problems.append(where + ' is not of type ' + repr(kinds))\u{a}")), List::cons(f8(Str::lit("            return\u{a}")), List::cons(f8(Str::lit("    if 'enum' in rule and value not in rule['enum']:\u{a}")), List::cons(f8(Str::lit("        problems.append(where + ' is not one of ' + repr(rule['enum']))\u{a}")), List::cons(f8(Str::lit("    if 'const' in rule and value != rule['const']:\u{a}")), List::cons(f8(Str::lit("        problems.append(where + ' is not ' + repr(rule['const']))\u{a}")), List::cons(f8(Str::lit("    if isinstance(value, dict):\u{a}")), List::cons(f8(Str::lit("        for name in rule.get('required', []):\u{a}")), List::cons(f8(Str::lit("            if name not in value:\u{a}")), List::cons(f8(Str::lit("                problems.append(where + ' lacks ' + name)\u{a}")), List::cons(f8(Str::lit("        known = rule.get('properties', {})\u{a}")), List::cons(f8(Str::lit("        for name, item in value.items():\u{a}")), List::cons(f8(Str::lit("            if name in known:\u{a}")), List::cons(f8(Str::lit("                check(item, known[name], where + '.' + name, problems)\u{a}")), List::cons(f8(Str::lit("            elif rule.get('additionalProperties') is False:\u{a}")), List::cons(f8(Str::lit("                problems.append(where + ' has ' + name + ', which the schema does not allow')\u{a}")), List::cons(f8(Str::lit("            else:\u{a}")), List::cons(f8(Str::lit("                check(item, rule.get('additionalProperties'), where + '.' + name, problems)\u{a}")), List::cons(f8(Str::lit("    if isinstance(value, list) and 'items' in rule:\u{a}")), List::cons(f8(Str::lit("        for index, item in enumerate(value):\u{a}")), List::cons(f8(Str::lit("            check(item, rule['items'], where + '[' + str(index) + ']', problems)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("result = json.load(open('surface' + chr(47) + 'mcp-tools.json'))\u{a}")), List::cons(f8(Str::lit("problems = []\u{a}")), List::cons(f8(Str::lit("check(result, defs['ListToolsResult'], 'tools-list', problems)\u{a}")), List::cons(f8(Str::lit("names = [tool.get('name') for tool in result.get('tools', [])]\u{a}")), List::cons(f8(Str::lit("if len(names) != len(set(names)):\u{a}")), List::cons(f8(Str::lit("    problems.append('tools-list names a tool twice')\u{a}")), List::cons(f8(Str::lit("for tool in result.get('tools', []):\u{a}")), List::cons(f8(Str::lit("    shape = tool.get('inputSchema', {})\u{a}")), List::cons(f8(Str::lit("    for name in shape.get('required', []):\u{a}")), List::cons(f8(Str::lit("        if name not in shape.get('properties', {}):\u{a}")), List::cons(f8(Str::lit("            problems.append(tool.get('name', '?') + ' requires ' + name + ', which it does not define')\u{a}")), List::cons(f8(Str::lit("for problem in problems:\u{a}")), List::cons(f8(Str::lit("    print('::error::surface' + chr(47) + 'mcp-tools.json ' + problem)\u{a}")), List::cons(f8(Str::lit("if not problems:\u{a}")), List::cons(f8(Str::lit("    print('accepted: surface' + chr(47) + 'mcp-tools.json holds ' + str(len(names)) + ' MCP tools, valid against MCP-SCHEMA-2026-07-28')\u{a}")), List::cons(f8(Str::lit("sys.exit(1 if problems else 0)\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f175() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# Studio.Agent.Interface.Resource (REQ-UI-PRISM, REQ-AGENT-INTERFACE): the view tree of the Studio UI Prism, published\u{a}")), List::cons(f8(Str::lit("# as MCP resources. Each journey's routes, views and actions are read from the app's own acceptance vectors, the\u{a}")), List::cons(f8(Str::lit("# answers its model gives, and its guards from the build rows of its suite in model/ids.toml, so nothing here\u{a}")), List::cons(f8(Str::lit("# restates the model. The resource list must be a ListResourcesResult, and each journey a ReadResourceResult, of\u{a}")), List::cons(f8(Str::lit("# the bound MCP-SCHEMA-2026-07-28. Each role of the D10 matrix (model/matrix) gets the view tree Studio.Gui.View answers\u{a}")), List::cons(f8(Str::lit("# for its relations, and a tree whose WCAG 2.2 audit fails stops the publish. gui-tree.py COMMAND... runs COMMAND record on the app's holo.\u{a}")), List::cons(f8(Str::lit("import glob, json, subprocess, sys, tomllib\u{a}")), List::cons(f8(Str::lit("SLASH = chr(47)\u{a}")), List::cons(f8(Str::lit("TYPES = {'object': dict, 'array': list, 'string': str, 'boolean': bool, 'null': type(None)}\u{a}")), List::cons(f8(Str::lit("schema = json.load(open(SLASH.join(['authorities', 'MCP-SCHEMA-2026-07-28', 'schema.json'])))\u{a}")), List::cons(f8(Str::lit("defs = schema['$defs']\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def numeric(value):\u{a}")), List::cons(f8(Str::lit("    return isinstance(value, (int, float)) and not isinstance(value, bool)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def check(value, rule, where, problems):\u{a}")), List::cons(f8(Str::lit("    if not isinstance(rule, dict):\u{a}")), List::cons(f8(Str::lit("        return\u{a}")), List::cons(f8(Str::lit("    if '$ref' in rule:\u{a}")), List::cons(f8(Str::lit("        check(value, defs[rule['$ref'].split(SLASH)[-1]], where, problems)\u{a}")), List::cons(f8(Str::lit("    kinds = rule.get('type')\u{a}")), List::cons(f8(Str::lit("    if kinds is not None:\u{a}")), List::cons(f8(Str::lit("        kinds = kinds if isinstance(kinds, list) else [kinds]\u{a}")), List::cons(f8(Str::lit("        fits = any((kind in ('number', 'integer') and numeric(value)) or (kind in TYPES and isinstance(value, TYPES[kind])) for kind in kinds)\u{a}")), List::cons(f8(Str::lit("        if not fits:\u{a}")), List::cons(f8(Str::lit("            problems.append(where + ' is not of type ' + repr(kinds))\u{a}")), List::cons(f8(Str::lit("            return\u{a}")), List::cons(f8(Str::lit("    if 'enum' in rule and value not in rule['enum']:\u{a}")), List::cons(f8(Str::lit("        problems.append(where + ' is not one of ' + repr(rule['enum']))\u{a}")), List::cons(f8(Str::lit("    if 'const' in rule and value != rule['const']:\u{a}")), List::cons(f8(Str::lit("        problems.append(where + ' is not ' + repr(rule['const']))\u{a}")), List::cons(f8(Str::lit("    if isinstance(value, dict):\u{a}")), List::cons(f8(Str::lit("        for name in rule.get('required', []):\u{a}")), List::cons(f8(Str::lit("            if name not in value:\u{a}")), List::cons(f8(Str::lit("                problems.append(where + ' lacks ' + name)\u{a}")), List::cons(f8(Str::lit("        known = rule.get('properties', {})\u{a}")), List::cons(f8(Str::lit("        for name, item in value.items():\u{a}")), List::cons(f8(Str::lit("            if name in known:\u{a}")), List::cons(f8(Str::lit("                check(item, known[name], where + '.' + name, problems)\u{a}")), List::cons(f8(Str::lit("            elif rule.get('additionalProperties') is False:\u{a}")), List::cons(f8(Str::lit("                problems.append(where + ' has ' + name + ', which the schema does not allow')\u{a}")), List::cons(f8(Str::lit("            else:\u{a}")), List::cons(f8(Str::lit("                check(item, rule.get('additionalProperties'), where + '.' + name, problems)\u{a}")), List::cons(f8(Str::lit("    if isinstance(value, list) and 'items' in rule:\u{a}")), List::cons(f8(Str::lit("        for index, item in enumerate(value):\u{a}")), List::cons(f8(Str::lit("            check(item, rule['items'], where + '[' + str(index) + ']', problems)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("holo = sorted(glob.glob(SLASH.join(['app', '.prism', 'build', '*.holo'])))[0]\u{a}")), List::cons(f8(Str::lit("record = SLASH.join(['target', 'gui-record.json'])\u{a}")), List::cons(f8(Str::lit("subprocess.run(sys.argv[1:] + ['record', holo, '--output', record], check=True, capture_output=True)\u{a}")), List::cons(f8(Str::lit("events = json.load(open(record))['events']\u{a}")), List::cons(f8(Str::lit("rows = tomllib.load(open(SLASH.join(['model', 'ids.toml']), 'rb'))['id']\u{a}")), List::cons(f8(Str::lit("prefix = 'studio' + chr(9) + 'gui' + chr(9)\u{a}")), List::cons(f8(Str::lit("journeys = {}\u{a}")), List::cons(f8(Str::lit("for event in events:\u{a}")), List::cons(f8(Str::lit("    request = bytes.fromhex(event['request']).decode('utf-8', 'replace')\u{a}")), List::cons(f8(Str::lit("    if not request.startswith(prefix):\u{a}")), List::cons(f8(Str::lit("        continue\u{a}")), List::cons(f8(Str::lit("    route = request[len(prefix):].split(chr(9))[0]\u{a}")), List::cons(f8(Str::lit("    entry = journeys.setdefault(route, {'route': prefix + route + chr(9), 'views': [], 'actions': {}})\u{a}")), List::cons(f8(Str::lit("    for line in bytes.fromhex(event['response']).decode('utf-8', 'replace').split(chr(10)):\u{a}")), List::cons(f8(Str::lit("        kind, _, rest = line.partition(' ')\u{a}")), List::cons(f8(Str::lit("        name, _, text = rest.partition(': ')\u{a}")), List::cons(f8(Str::lit("        if kind == 'journey':\u{a}")), List::cons(f8(Str::lit("            entry['journey'] = name\u{a}")), List::cons(f8(Str::lit("            entry['title'] = text\u{a}")), List::cons(f8(Str::lit("        elif kind == 'view' and name not in entry['views']:\u{a}")), List::cons(f8(Str::lit("            entry['views'].append(name)\u{a}")), List::cons(f8(Str::lit("        elif kind == 'action':\u{a}")), List::cons(f8(Str::lit("            states = entry['actions'].setdefault(name, [])\u{a}")), List::cons(f8(Str::lit("            state = text.split(',')[0]\u{a}")), List::cons(f8(Str::lit("            if state not in states:\u{a}")), List::cons(f8(Str::lit("                states.append(state)\u{a}")), List::cons(f8(Str::lit("problems = []\u{a}")), List::cons(f8(Str::lit("resources = []\u{a}")), List::cons(f8(Str::lit("reads = {}\u{a}")), List::cons(f8(Str::lit("for route, entry in sorted(journeys.items()):\u{a}")), List::cons(f8(Str::lit("    suite = 'gui-' + route\u{a}")), List::cons(f8(Str::lit("    rules = [{'id': row['id'], 'statement': row['statement']} for row in rows if row.get('suite') == suite and row.get('level') == 'build']\u{a}")), List::cons(f8(Str::lit("    journey = entry.get('journey', route)\u{a}")), List::cons(f8(Str::lit("    tree = {'journey': journey, 'title': entry.get('title', ''), 'route': entry['route'], 'views': entry['views'],\u{a}")), List::cons(f8(Str::lit("            'actions': [{'name': name, 'states': sorted(states)} for name, states in entry['actions'].items()], 'rules': rules}\u{a}")), List::cons(f8(Str::lit("    if not tree['views'] or not tree['actions'] or not rules:\u{a}")), List::cons(f8(Str::lit("        problems.append(journey + ' needs a view, an action and a build rule of suite ' + suite)\u{a}")), List::cons(f8(Str::lit("    uri = 'verdant-studio:' + SLASH + SLASH + 'gui' + SLASH + journey\u{a}")), List::cons(f8(Str::lit("    resources.append({'uri': uri, 'name': journey, 'title': tree['title'], 'mimeType': 'application' + SLASH + 'json',\u{a}")), List::cons(f8(Str::lit("                      'description': 'The ' + journey + ' journey of the Studio UI Prism: its route, views, actions and guards'})\u{a}")), List::cons(f8(Str::lit("    reads[journey] = {'cacheScope': 'public', 'resultType': 'complete', 'ttlMs': 0,\u{a}")), List::cons(f8(Str::lit("                      'contents': [{'uri': uri, 'mimeType': 'application' + SLASH + 'json', 'text': json.dumps(tree, sort_keys=True)}]}\u{a}")), List::cons(f8(Str::lit("answers = {}\u{a}")), List::cons(f8(Str::lit("vprefix = 'studio' + chr(9) + 'view' + chr(9)\u{a}")), List::cons(f8(Str::lit("for event in events:\u{a}")), List::cons(f8(Str::lit("    request = bytes.fromhex(event['request']).decode('utf-8', 'replace')\u{a}")), List::cons(f8(Str::lit("    if request.startswith(vprefix):\u{a}")), List::cons(f8(Str::lit("        answers[request[len(vprefix):]] = bytes.fromhex(event['response']).decode('utf-8', 'replace')\u{a}")), List::cons(f8(Str::lit("roles = {}\u{a}")), List::cons(f8(Str::lit("for path in sorted(glob.glob(SLASH.join(['model', 'matrix', '*.jsonl']))):\u{a}")), List::cons(f8(Str::lit("    for line in open(path):\u{a}")), List::cons(f8(Str::lit("        edge = json.loads(line)\u{a}")), List::cons(f8(Str::lit("        term = edge['role']['term']\u{a}")), List::cons(f8(Str::lit("        slug = '-'.join(''.join(ch if ch.isalnum() else ' ' for ch in term.lower()).split())\u{a}")), List::cons(f8(Str::lit("        role = roles.setdefault(slug, {'terms': [], 'relations': [], 'rows': []})\u{a}")), List::cons(f8(Str::lit("        for key, value in (('terms', term), ('relations', edge['relation']), ('rows', edge['row'])):\u{a}")), List::cons(f8(Str::lit("            if value not in role[key]:\u{a}")), List::cons(f8(Str::lit("                role[key].append(value)\u{a}")), List::cons(f8(Str::lit("unbuilt = []\u{a}")), List::cons(f8(Str::lit("for slug, role in sorted(roles.items()):\u{a}")), List::cons(f8(Str::lit("    trees = {relation: answers[relation] for relation in role['relations'] if not answers.get(relation, 'refused:').startswith('refused:')}\u{a}")), List::cons(f8(Str::lit("    if not trees:\u{a}")), List::cons(f8(Str::lit("        unbuilt.append(role['terms'][0])\u{a}")), List::cons(f8(Str::lit("        continue\u{a}")), List::cons(f8(Str::lit("    for relation, answer in trees.items():\u{a}")), List::cons(f8(Str::lit("        for line in answer.split(chr(10)):\u{a}")), List::cons(f8(Str::lit("            if line.startswith('audit: ') and not line.startswith('audit: conforms'):\u{a}")), List::cons(f8(Str::lit("                problems.append('of the role ' + role['terms'][0] + ' (' + relation + ') fails its WCAG 2.2 audit: ' + line)\u{a}")), List::cons(f8(Str::lit("    opened = sorted({line.split(' ')[1].rstrip(':') for answer in trees.values() for line in answer.split(chr(10)) if line.startswith('journey ')})\u{a}")), List::cons(f8(Str::lit("    name = 'view' + SLASH + slug\u{a}")), List::cons(f8(Str::lit("    uri = 'verdant-studio:' + SLASH + SLASH + 'gui' + SLASH + name\u{a}")), List::cons(f8(Str::lit("    tree = {'role': role['terms'], 'relations': role['relations'], 'rows': role['rows'], 'journeys': opened, 'trees': trees}\u{a}")), List::cons(f8(Str::lit("    resources.append({'uri': uri, 'name': name, 'title': 'The Studio UI of the ' + role['terms'][0] + ' role', 'mimeType': 'application' + SLASH + 'json',\u{a}")), List::cons(f8(Str::lit("                      'description': 'The view tree the ' + role['terms'][0] + ' role of the D10 matrix opens (Studio.Gui.View), each journey audited against WCAG 2.2'})\u{a}")), List::cons(f8(Str::lit("    reads[name] = {'cacheScope': 'public', 'resultType': 'complete', 'ttlMs': 0,\u{a}")), List::cons(f8(Str::lit("                   'contents': [{'uri': uri, 'mimeType': 'application' + SLASH + 'json', 'text': json.dumps(tree, sort_keys=True)}]}\u{a}")), List::cons(f8(Str::lit("print('::notice::no view tree for ' + str(len(unbuilt)) + ' roles whose relations Studio.Gui.View builds none for: ' + ', '.join(unbuilt))\u{a}")), List::cons(f8(Str::lit("listing = {'cacheScope': 'public', 'resultType': 'complete', 'ttlMs': 0, 'resources': resources}\u{a}")), List::cons(f8(Str::lit("check(listing, defs['ListResourcesResult'], 'resources-list', problems)\u{a}")), List::cons(f8(Str::lit("for journey, read in reads.items():\u{a}")), List::cons(f8(Str::lit("    check(read, defs['ReadResourceResult'], 'resources-read ' + journey, problems)\u{a}")), List::cons(f8(Str::lit("if not resources:\u{a}")), List::cons(f8(Str::lit("    problems.append('the app answers no journey of the Studio UI Prism')\u{a}")), List::cons(f8(Str::lit("for problem in problems:\u{a}")), List::cons(f8(Str::lit("    print('::error::the GUI view tree ' + problem)\u{a}")), List::cons(f8(Str::lit("if problems:\u{a}")), List::cons(f8(Str::lit("    sys.exit(1)\u{a}")), List::cons(f8(Str::lit("site = SLASH.join(['app', '.prism', 'site', 'surface'])\u{a}")), List::cons(f8(Str::lit("subprocess.run(['mkdir', '-p', site + SLASH + 'gui' + SLASH + 'view'], check=True)\u{a}")), List::cons(f8(Str::lit("json.dump(listing, open(site + SLASH + 'mcp-resources.json', 'w'), sort_keys=True)\u{a}")), List::cons(f8(Str::lit("for journey, read in reads.items():\u{a}")), List::cons(f8(Str::lit("    json.dump(read, open(site + SLASH + 'gui' + SLASH + journey + '.json', 'w'), sort_keys=True)\u{a}")), List::cons(f8(Str::lit("subprocess.run(['cp', SLASH.join(['surface', 'mcp-tools.json']), site], check=True)\u{a}")), List::cons(f8(Str::lit("print('accepted: ' + str(len(resources)) + ' journeys and role view trees of the Studio UI Prism as MCP resources, valid against MCP-SCHEMA-2026-07-28: ' + ', '.join(sorted(reads)))\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f176(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m773 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m773.uncons() {
        None => {
            if f282(f96(f64(v0, v1))?)? {
                Ok(Adt12::C4(f8(Str::lit("the input has a blank line; send one record per line and no trailing newline, because the workflow adds the final one\u{a}"))))
            } else {
                Ok(v2)
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f177(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m774 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m774.uncons() {
        None => {
            let v5: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f283(v5)?
                } else {
                    f284(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f246(f285(), f286(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f246(f285(), f286(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f287()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f178(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m775 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m775.uncons() {
        None => {
            let m776 = f288(f96(f64(v0.clone(), v1.clone()))?)?;
            match m776 {
                None => {
                    Ok(Adt12::C4(f289()))
                }
                Some(v4) => {
                    let m777 = f290(f96(f64(v0, v1))?)?;
                    match m777 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f291(v5)))
                        }
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f179(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let v2: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
    if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f118(f292(f293(v2.clone())?), f51({
            let a779 = {
                let a778 = f294(f293(v2)?);
                length_list(a778)
            };
            let a780 = 0u64;
            nat_eq(a779, a780)
        })) {
            Ok(Adt12::C3(List::cons((f246(f285(), f286(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f246(f285(), f286(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f180(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m781 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m781.uncons() {
        None => {
            let m782 = f295(f96(f64(v0, v1))?)?;
            match m782 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f181(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m783 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m783.uncons() {
        None => {
            let v5: Bytes = f246(if v2 {
                f216()
            } else {
                f215()
            }, f286(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m784 = f227(f96(f64(v0.clone(), v1.clone()))?, v2)?;
                match m784 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a786 = {
                            let a785 = v6;
                            length_list(a785)
                        };
                        let a787 = 0u64;
                        if nat_eq(a786, a787) {
                            Ok(Adt12::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt12::C3(List::cons((v5.clone(), f64(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f182(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m788 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m788.uncons() {
        None => {
            let m789 = f57(v0.clone(), v1.clone())?;
            match m789 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f296())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt2 = {
                            let m790 = f94(v4.clone(), f8(Str::lit("source")));
                            match m790 {
                                None => {
                                    Adt2::C9
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f93(f94(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f246(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f246(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m791 = v8;
                        match m791 {
                            None => {
                                f297(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f297(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f246(v9, f286(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h792, _) => {
                                            h792
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m793 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m793.uncons() {
                                            None => {
                                                if f32(f298(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f297(v0, v4, v5, v10, Some((v12, v11)))
                                                } else {
                                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("checksum mismatch: ")), List::cons(v12, List::cons(f8(Str::lit(" is not the sha256 the binding pins\u{a}")), List::<Bytes>::nil()))))))
                                                }
                                            }
                                            Some((v13, v14)) => {
                                                Ok(Adt12::C0(List::cons(v13, v14)))
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
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f183(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m794 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m794.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f269(v4.clone())?;
            if f51(f99(f299(v5.clone()), f300())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f301(Str::lit("id"), v6.clone())?, List::cons(f301(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f301(Str::lit("artifact"), f217())?, List::cons(f301(Str::lit("checksum"), f298(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f217(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f302(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f303(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f123(f271(f268(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f184(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m795 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m795.uncons() {
        None => {
            let m796 = f57(v0, v1)?;
            match m796 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f304(f192(Some(v4))) {
                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("merge\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("wait\u{a}"))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f185(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m797 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f154(), List::<Bytes>::nil())));
    match m797.uncons() {
        None => {
            let m798 = f305(f96(f64(v0.clone(), v1))?)?;
            match m798 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt2> = f97(v0, f154())?;
                    let v6: List<Bytes> = f143(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f146(f306(v5.clone()))?;
                    let a800 = {
                        let a799 = v6.clone();
                        length_list(a799)
                    };
                    let a801 = 0u64;
                    if if nat_eq(a800, a801) {
                        true
                    } else {
                        f51({
                            let a804 = {
                                let a802 = f146(f126(f306(v4.clone()), v7.clone()))?;
                                length_list(a802)
                            };
                            let a805 = {
                                let a803 = f126(f306(v4.clone()), v7);
                                length_list(a803)
                            };
                            nat_eq(a804, a805)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f307(f154(), f228(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f123(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f186(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m806 = v2;
    match m806.uncons() {
        None => {
            Ok(Adt12::C4(f80()))
        }
        Some((v3, v4)) => {
            let m807 = v4;
            match m807.uncons() {
                None => {
                    Ok(Adt12::C4(f80()))
                }
                Some((v5, v6)) => {
                    let m808 = v6.clone();
                    match m808.uncons() {
                        None => {
                            Ok(Adt12::C4(f80()))
                        }
                        Some((_, _)) => {
                            let m809 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f308(), List::<Bytes>::nil())));
                            match m809.uncons() {
                                None => {
                                    let v11: Bytes = f123(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt2> = f97(v0, f308())?;
                                    if f309(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting fix: ")), List::cons(v11, List::cons(f8(Str::lit(" is refused again with the refusal model/errors.json records, so the API needs a fix before it runs (REQ-SELF-HEALING): ")), List::cons({
                                            let m810 = f96(v12)?;
                                            match m810.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((v14, _)) => {
                                                    v14
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f307(f308(), f228(v13, List::cons(f310(v11.clone(), v12, v5, v3), List::<Adt2>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
                                    }
                                }
                                Some((v9, v10)) => {
                                    Ok(Adt12::C0(List::cons(v9, v10)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f187(v0: Bytes, p864: u64, p865: u64) -> R<bool> {
    let m863 = (p864, p865);
    {
        let mut m863 = m863;
        loop {
            let step = {
                let (v1, v2) = m863;
                let m811 = v2;
                if m811 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m811 - 1;
                    let m814 = {
                        let a812 = v0.clone();
                        let a813 = v1;
                        index_bytes(a812, a813)
                    };
                    match m814 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f311({
                                let v5: u8 = v4;
                                let a859 = {
                                    let a835 = {
                                        let a823 = {
                                            let a817 = {
                                                let a815 = v5;
                                                let a816 = 128u8;
                                                fixed_u8::bit_and(a815, a816)
                                            };
                                            let a818 = 128u8;
                                            if equal(a817, a818) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a824 = {
                                            let a821 = {
                                                let a819 = v5;
                                                let a820 = 64u8;
                                                fixed_u8::bit_and(a819, a820)
                                            };
                                            let a822 = 64u8;
                                            if equal(a821, a822) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a823, a824)?
                                    };
                                    let a836 = {
                                        let a833 = {
                                            let a827 = {
                                                let a825 = v5;
                                                let a826 = 32u8;
                                                fixed_u8::bit_and(a825, a826)
                                            };
                                            let a828 = 32u8;
                                            if equal(a827, a828) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a834 = {
                                            let a831 = {
                                                let a829 = v5;
                                                let a830 = 16u8;
                                                fixed_u8::bit_and(a829, a830)
                                            };
                                            let a832 = 16u8;
                                            if equal(a831, a832) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a833, a834)?
                                    };
                                    nat_add(a835, a836)?
                                };
                                let a860 = {
                                    let a857 = {
                                        let a845 = {
                                            let a839 = {
                                                let a837 = v5;
                                                let a838 = 8u8;
                                                fixed_u8::bit_and(a837, a838)
                                            };
                                            let a840 = 8u8;
                                            if equal(a839, a840) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a846 = {
                                            let a843 = {
                                                let a841 = v5;
                                                let a842 = 4u8;
                                                fixed_u8::bit_and(a841, a842)
                                            };
                                            let a844 = 4u8;
                                            if equal(a843, a844) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a845, a846)?
                                    };
                                    let a858 = {
                                        let a855 = {
                                            let a849 = {
                                                let a847 = v5;
                                                let a848 = 2u8;
                                                fixed_u8::bit_and(a847, a848)
                                            };
                                            let a850 = 2u8;
                                            if equal(a849, a850) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a856 = {
                                            let a853 = {
                                                let a851 = v5;
                                                let a852 = 1u8;
                                                fixed_u8::bit_and(a851, a852)
                                            };
                                            let a854 = 1u8;
                                            if equal(a853, a854) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a855, a856)?
                                    };
                                    nat_add(a857, a858)?
                                };
                                nat_add(a859, a860)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a861 = v1;
                                    let a862 = 1u64;
                                    nat_add(a861, a862)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(p870: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m869 = p870;
    {
        let mut m869 = m869;
        loop {
            let step = {
                let v0 = m869;
                let m866 = v0;
                match m866.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h867, _) = v2;
                            h867
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h868) = v2;
                                h868
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m874 = {
        let a872 = match v0 {
            Adt1::C0(_, _, _, h871) => {
                h871
            }
        };
        let a873 = f40(v1);
        index_list(a872, a873)
    };
    match m874 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f190(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f86(v0.clone(), v1) {
        let m875 = f44(v0, 34u8)?;
        match m875.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m876 = v3;
                match m876.uncons() {
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

pub fn f191(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f312(v0, v1)
}

pub fn f192(v0: Option<Adt2>) -> List<Adt2> {
    let m877 = v0;
    match m877 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f313(v1)
        }
    }
}

pub fn f193(v0: Adt2, v1: Bytes) -> Adt2 {
    let m878 = v0;
    match m878 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r879) => {
            let v12 = (*r879).clone();
            Adt2::C1(v11, std::rc::Rc::new(v12))
        }
        Adt2::C2(v6) => {
            Adt2::C2(v6)
        }
        Adt2::C3(v7, v8) => {
            Adt2::C3(v7, v8)
        }
        Adt2::C4(v13, v14, v15) => {
            Adt2::C4(v13, v14, v15)
        }
        Adt2::C5(v16, v17) => {
            Adt2::C5(v16, v17)
        }
        Adt2::C6(v9) => {
            Adt2::C6(v9)
        }
        Adt2::C7(r880) => {
            let v10 = (*r880).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r881) => {
            let v5 = (*r881).clone();
            Adt2::C8(std::rc::Rc::new(f193(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r882, r883) => {
            let v18 = (*r882).clone();
            let v19 = (*r883).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r884, r885) => {
            let v3 = (*r884).clone();
            let v4 = (*r885).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt2::C11(v2, std::rc::Rc::new(f314(v3, v1.clone())), std::rc::Rc::new(f193(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f193(v4, v1)))
            }
        }
    }
}

pub fn f194(v0: Adt2) -> R<Bytes> {
    let m886 = v0;
    match m886 {
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
            f315(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f316(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f317(v9, v10)
        }
        Adt2::C6(v11) => {
            f318(v11)
        }
        Adt2::C7(r887) => {
            let v12 = (*r887).clone();
            Ok(f319(f320(v12)?))
        }
        Adt2::C8(r888) => {
            let v13 = (*r888).clone();
            f321(f322(v13)?)
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

pub fn f195(v0: Adt1, p892: List<Adt13>) -> R<Option<Adt13>> {
    let m891 = p892;
    {
        let mut m891 = m891;
        loop {
            let step = {
                let v1 = m891;
                let m889 = v1;
                match m889.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f196(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h890, _) => {
                                h890
                            }
                        }) {
                            Ok::<List<Adt13>, R<Option<Adt13>>>(v3)
                        } else {
                            Err::<List<Adt13>, R<Option<Adt13>>>(Ok(Some(v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m891 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Adt13) -> R<Bytes> {
    Ok(f246(f323(), f286(f324(v0))?))
}

pub fn f197(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f324(v0), List::<Bytes>::nil())))))
}

pub fn f198() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f199() -> Bytes {
    f325()
}

pub fn f200(v0: List<Adt13>) -> R<List<Bytes>> {
    let m893 = v0;
    match m893.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f196(v1)?, f200(v2)?))
        }
    }
}

pub fn f201(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m894 = v1;
    match m894.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f326(v5.clone(), v2.clone(), v3.clone(), f298(f64(v0.clone(), f196(v5)?))?, v4.clone())?), f201(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f202(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f327(v0.clone())?;
    let v2: u64 = f328(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a895 = v1;
        let a896 = v2;
        nat_sub(a895, a896)
    }))
}

pub fn f203(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a899 = v3;
    let a900 = f46(v0, v1, {
        let a897 = v2;
        let a898 = v1;
        nat_sub(a897, a898)
    });
    append_bytes(a899, a900)
}

pub fn f204(v0: Bytes, p913: u64, p914: u64) -> R<u64> {
    let m912 = (p913, p914);
    {
        let mut m912 = m912;
        loop {
            let step = {
                let (v1, v2) = m912;
                let m901 = v2;
                if m901 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m901 - 1;
                    let v4: u64 = f116(v0.clone(), v1)?;
                    let a902 = v4;
                    let a903 = 34u64;
                    if nat_eq(a902, a903) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a904 = v4;
                        let a905 = 92u64;
                        if nat_eq(a904, a905) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a906 = v4;
                            let a907 = 32u64;
                            if nat_lt(a906, a907) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a908 = v4;
                                let a909 = 256u64;
                                if nat_eq(a908, a909) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a910 = v1;
                                        let a911 = 1u64;
                                        nat_add(a910, a911)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, p940: u64, p941: u64, p942: Bytes) -> R<Option<(u64, Bytes)>> {
    let m939 = (p940, (p941, p942));
    {
        let mut m939 = m939;
        loop {
            let step = {
                let (v1, (v2, v3)) = m939;
                let m915 = v2;
                if m915 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m915 - 1;
                    let v5: u64 = f204(v0.clone(), v1, {
                        let a919 = {
                            let a917 = {
                                let a916 = v0.clone();
                                length_bytes(a916)
                            };
                            let a918 = v1;
                            nat_sub(a917, a918)
                        };
                        let a920 = 1u64;
                        nat_add(a919, a920)?
                    })?;
                    let v6: Bytes = {
                        let a923 = v3;
                        let a924 = f46(v0.clone(), v1, {
                            let a921 = v5;
                            let a922 = v1;
                            nat_sub(a921, a922)
                        });
                        append_bytes(a923, a924)
                    };
                    let v7: u64 = f116(v0.clone(), v5)?;
                    let a925 = v7;
                    let a926 = 34u64;
                    if nat_eq(a925, a926) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a927 = v5;
                            let a928 = 1u64;
                            nat_add(a927, a928)?
                        }, v6))))
                    } else {
                        let a929 = v7;
                        let a930 = 92u64;
                        if nat_eq(a929, a930) {
                            let m931 = f329(v0.clone(), v5, v6)?;
                            match m931 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a934 = {
                                        let (_, h932) = {
                                            let (_, h933) = v8.clone();
                                            h933
                                        };
                                        h932
                                    };
                                    let a935 = 0u64;
                                    if nat_eq(a934, a935) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h936, _) = v8;
                                            h936
                                        }, (v4, {
                                            let (h937, _) = {
                                                let (_, h938) = v8;
                                                h938
                                            };
                                            h937
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
                Ok(next) => { m939 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Bytes, v1: u64) -> R<u64> {
    let (h943, _) = f330(Fn3::F331(v0.clone()), {
        let a945 = {
            let a944 = v0;
            length_bytes(a944)
        };
        let a946 = 1u64;
        nat_add(a945, a946)?
    }, v1)?;
    Ok(h943)
}

pub fn f207(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f208(v0: Bytes) -> R<u64> {
    let (h947, _) = f330(Fn3::F332(v0.clone()), {
        let a949 = {
            let a948 = v0;
            length_bytes(a948)
        };
        let a950 = 1u64;
        nat_add(a949, a950)?
    }, 0u64)?;
    Ok(h947)
}

pub fn f209(v0: Bytes) -> R<Option<u64>> {
    if f333(v0.clone())? {
        Ok(Some({
            let (_, h951) = {
                let (h952, _) = f334(Fn4::F335(v0.clone()), {
                    let a953 = v0;
                    length_bytes(a953)
                }, (0u64, 0u64))?;
                h952
            };
            h951
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f210(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a955 = {
            let a954 = v1.clone();
            length_bytes(a954)
        };
        let a956 = f336(v1)?;
        nat_sub(a955, a956)
    });
    let v5: Bytes = {
        let a957 = v0;
        let a958 = v4.clone();
        append_bytes(a957, a958)
    };
    let v6: u64 = f208(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a960 = {
            let a959 = v5;
            length_bytes(a959)
        };
        let a961 = v6;
        nat_sub(a960, a961)
    });
    let v8: u64 = {
        let a962 = v4;
        length_bytes(a962)
    };
    let v9: u64 = {
        let a963 = v7.clone();
        length_bytes(a963)
    };
    let a964 = v9;
    let a965 = 0u64;
    if nat_eq(a964, a965) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f207(v2, {
            let a966 = v3;
            let a967 = v8;
            nat_lt(a966, a967)
        }) {
            let v10: u64 = if v2 {
                let a968 = v3;
                let a969 = v8;
                nat_add(a968, a969)?
            } else {
                let a970 = v8;
                let a971 = v3;
                nat_sub(a970, a971)
            };
            let v11: u64 = f336(v7.clone())?;
            let a972 = v10;
            let a973 = v11;
            if nat_le(a972, a973) {
                Ok(Some((f46(v7, 0u64, {
                    let a974 = v9;
                    let a975 = v10;
                    nat_sub(a974, a975)
                }), 0u64)))
            } else {
                let a978 = {
                    let a976 = v10;
                    let a977 = v11;
                    nat_sub(a976, a977)
                };
                let a979 = 19u64;
                if nat_le(a978, a979) {
                    Ok(Some((f46(v7, 0u64, {
                        let a980 = v9;
                        let a981 = v11;
                        nat_sub(a980, a981)
                    }), {
                        let a982 = v10;
                        let a983 = v11;
                        nat_sub(a982, a983)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a984 = v3;
                let a985 = v8;
                nat_sub(a984, a985)
            };
            let a988 = {
                let a986 = v9;
                let a987 = v12;
                nat_add(a986, a987)?
            };
            let a989 = 19u64;
            if nat_le(a988, a989) {
                Ok(Some(({
                    let a990 = v7;
                    let a991 = f337(v12);
                    append_bytes(a990, a991)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f211(v0: bool, v1: u64) -> bool {
    if v0 {
        let a992 = 0u64;
        let a993 = v1;
        if nat_lt(a992, a993) {
            let a994 = v1;
            let a995 = 9223372036854775808u64;
            nat_le(a994, a995)
        } else {
            false
        }
    } else {
        let a996 = v1;
        let a997 = 9223372036854775807u64;
        nat_le(a996, a997)
    }
}

pub fn f212(v0: Bytes, v1: u64) -> Option<u8> {
    let a998 = v0;
    let a999 = v1;
    index_bytes(a998, a999)
}

pub fn f213(v0: Fn2, p1003: Option<Bytes>, p1004: List<Bytes>) -> Option<Bytes> {
    let m1002 = (p1003, p1004);
    {
        let mut m1002 = m1002;
        loop {
            let step = {
                let (v1, v2) = m1002;
                let m1000 = v2;
                match m1000.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1001: Fn2 = v0.clone();
                            c1001.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1002 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1005 = v1;
    match m1005 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1008 = {
                    let a1006 = v3;
                    let a1007 = v0;
                    append_bytes(a1006, a1007)
                };
                let a1009 = v2;
                append_bytes(a1008, a1009)
            })
        }
    }
}

pub fn f215() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f216() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f217() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f218(v0: Adt1) -> R<List<Bytes>> {
    let m1010 = f269(f64(v0, f217()))?;
    match m1010.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f271(v2))
        }
    }
}

pub fn f219(v0: List<Adt2>) -> List<Bytes> {
    let m1011 = v0;
    match m1011.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1012 = f93(Some(v1));
            match m1012 {
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

pub fn f220(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1013 = v0;
    match m1013.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f220(v2))
        }
    }
}

pub fn f221(v0: List<Adt17>) -> List<(Bytes, Bytes)> {
    let m1014 = v0;
    match m1014.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt17::C0(h1015, _, _) => {
                    h1015
                }
            }, match v1 {
                Adt17::C0(_, _, h1016) => {
                    h1016
                }
            }), f221(v2))
        }
    }
}

pub fn f222(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Adt17>> {
    let v3: List<(Bytes, Bytes)> = f338(v1);
    let v4: List<(Bytes, Bytes)> = f339(v0, v2);
    f340(v3.clone(), f155(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f223(p1023: List<Bytes>, v1: List<(Bytes, Bytes)>, p1024: u64) -> R<List<Bytes>> {
    let m1022 = (p1023, p1024);
    {
        let mut m1022 = m1022;
        loop {
            let step = {
                let (v0, v2) = m1022;
                let m1017 = v2;
                if m1017 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1017 - 1;
                    let v4: List<Bytes> = f146(f126(v0.clone(), f341(v0.clone(), v1.clone())))?;
                    let a1020 = {
                        let a1018 = v4.clone();
                        length_list(a1018)
                    };
                    let a1021 = {
                        let a1019 = v0.clone();
                        length_list(a1019)
                    };
                    if nat_eq(a1020, a1021) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1022 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: List<Adt2>) -> List<Bytes> {
    let m1025 = v0;
    match m1025.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1026 = f94(v1, f8(Str::lit("std")));
            match m1026 {
                None => {
                    f224(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f225(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f224(v2))
                    } else {
                        f224(v2)
                    })
                }
            }
        }
    }
}

pub fn f225(v0: Adt2, v1: Bytes) -> bool {
    f51({
        let a1028 = {
            let a1027 = f58(v0, v1);
            length_bytes(a1027)
        };
        let a1029 = 0u64;
        nat_eq(a1028, a1029)
    })
}

pub fn f226(v0: Adt14, v1: Adt2) -> bool {
    if f342(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f342(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f227(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt2>>> {
    let m1030 = v0;
    match m1030.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f202(v2)?;
            let a1032 = {
                let a1031 = v4.clone();
                length_bytes(a1031)
            };
            let a1033 = 0u64;
            if nat_eq(a1032, a1033) {
                f227(v3, v1)
            } else {
                let m1034 = f43(v4)?;
                match m1034 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f343(v5.clone())
                        } else {
                            f344(v5.clone())
                        } {
                            let m1035 = f227(v3, v1)?;
                            match m1035 {
                                None => {
                                    Ok(None::<List<Adt2>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons(v5, v6)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt2>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f228(v0: List<Adt2>, v1: List<Adt2>) -> List<Adt2> {
    let m1036 = v0;
    match m1036.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f228(v3, v1))
        }
    }
}

pub fn f229(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1037 = v1;
    match m1037.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f246(v0.clone(), v2), f229(v0, v3))
            } else {
                f229(v0, v3)
            }
        }
    }
}

pub fn f230() -> Bytes {
    f8(Str::lit("set -euo pipefail; repo=$1; rev=$2; shift 2; d=target/prismpm; if [ \"$(git -C $d rev-parse HEAD 2>/dev/null)\" != \"$rev\" ]; then rm -rf $d; git init -q $d; git -C $d remote add origin \"https://github.com/$repo\"; git -C $d sparse-checkout set --no-cone sdk/npm/prism-code; git -C $d fetch -q --depth=1 --filter=blob:none origin \"$rev\"; git -C $d checkout -q FETCH_HEAD; fi; exec node $d/sdk/npm/prism-code/bin/prism-code.mjs \"$@\""))
}

pub fn f231() -> Bytes {
    f8(Str::lit("047c868227ca45c5837bf07c1e7ed3b762d0f490"))
}

pub fn f232(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1038 = v1;
    match m1038.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f126({
                let a1040 = {
                    let a1039 = f202(v4.clone())?;
                    length_bytes(a1039)
                };
                let a1041 = 0u64;
                if nat_eq(a1040, a1041) {
                    List::<Bytes>::nil()
                } else {
                    f345(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f232(v0, v5, v2, v3)?))
        }
    }
}

pub fn f233(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f234() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f217(), List::<Bytes>::nil())))))))
}

pub fn f235(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1042 = v1;
    match m1042.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m1043 = f57(v0.clone(), v2.clone())?;
                match m1043 {
                    None => {
                        Ok(List::cons(v2, f235(v0, v3)?))
                    }
                    Some(_) => {
                        f235(v0, v3)
                    }
                }
            } else {
                f235(v0, v3)
            }
        }
    }
}

pub fn f236(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m1044 = v1;
    match m1044.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f139(v0.clone(), v2.clone()) {
                f236(v0, v3)
            } else {
                List::cons(v2, f236(v0, v3))
            }
        }
    }
}

pub fn f237() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f238(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m1045 = v0;
    match m1045.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f126(f346({
                let (h1046, _) = v2;
                h1046
            }, {
                let (_, h1047) = v2;
                h1047
            }, v1), f238(v3, v1))
        }
    }
}

pub fn f239() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::cons(f8(Str::lit(".github/scripts/readmit.sh")), List::<Bytes>::nil())))))
}

pub fn f240(v0: Adt1) -> R<Option<Bytes>> {
    f347(f348(v0, f239())?)
}

pub fn f241(v0: Adt1, p1051: List<Bytes>) -> R<Option<Bytes>> {
    let m1050 = p1051;
    {
        let mut m1050 = m1050;
        loop {
            let step = {
                let v1 = m1050;
                let m1048 = v1;
                match m1048.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1049 = f349(f64(v0.clone(), v2.clone()), f350())?;
                        match m1049 {
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
                Ok(next) => { m1050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h1052, _, _, _, _, _) => {
            h1052
        }
    };
    let v2: Adt18 = f351(v1.clone())?;
    let v3: List<Bytes> = f352(v1.clone());
    let v4: List<Bytes> = f143(match v0.clone() {
        Adt16::C0(_, _, _, h1053, _, _) => {
            h1053
        }
    }, f8(Str::lit("path")));
    let m1055 = f353(v2, List::<u64>::nil(), f354(v1.clone(), match v0.clone() {
        Adt16::C0(_, h1054, _, _, _, _) => {
            h1054
        }
    }, 0u64)?);
    match m1055 {
        None => {
            if f356(f357(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1057) => {
                    h1057
                }
            }, f126(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1058) => {
                    h1058
                }
            }, v4.clone()))?, f357(v4.clone(), f126(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1059) => {
                    h1059
                }
            }, v4.clone()))?, f358(match v0.clone() {
                Adt16::C0(_, _, _, h1060, _, _) => {
                    h1060
                }
            })) {
                let m1064 = f359(f357(match v0.clone() {
                    Adt16::C0(_, _, _, _, h1061, _) => {
                        h1061
                    }
                }, f126(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, _, h1062, _) => {
                        h1062
                    }
                }))?, f357(v3.clone(), f126(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, _, h1063, _) => {
                        h1063
                    }
                }))?);
                match m1064 {
                    None => {
                        let m1067 = f360(f143(match v0.clone() {
                            Adt16::C0(_, h1066, _, _, _, _) => {
                                h1066
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1067 {
                            None => {
                                let m1068 = f360(v4, List::<Bytes>::nil());
                                match m1068 {
                                    None => {
                                        let m1070 = f361(v1, match v0 {
                                            Adt16::C0(_, _, h1069, _, _, _) => {
                                                h1069
                                            }
                                        });
                                        match m1070 {
                                            None => {
                                                Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f123(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                            }
                                            Some(v9) => {
                                                Ok((false, f7(List::cons(f8(Str::lit("unbound constraint: ")), List::cons(v9, List::cons(f8(Str::lit(" cites an authority or catalog row that is not bound\u{a}")), List::<Bytes>::nil()))))))
                                            }
                                        }
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f355(f126(v3, match v0 {
                            Adt16::C0(_, _, _, _, h1065, _) => {
                                h1065
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f355(match v1 {
                Adt15::C0(_, _, _, _, h1056) => {
                    h1056
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f243(v0: List<Bytes>) -> List<Bytes> {
    f121(Fn1::F362, List::<Bytes>::nil(), v0)
}

pub fn f244(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f363(v0, v1)
}

pub fn f245(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f364(f365(f366(v0))?))
}

pub fn f246(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f247(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1071 = f57(v0, v2)?;
    match m1071 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f367(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1072 = v4.clone();
            match m1072.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1074 = f93(f94({
                        let m1073 = f94(v3.clone(), f8(Str::lit("source")));
                        match m1073 {
                            None => {
                                Adt2::C9
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1074 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f368(f369(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f248(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1075 = v0;
    match m1075.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f370(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1079 = f44(f46(v1.clone(), 6u64, {
                    let a1077 = {
                        let a1076 = v1;
                        length_bytes(a1076)
                    };
                    let a1078 = 6u64;
                    nat_sub(a1077, a1078)
                }), 34u8)?;
                match m1079.uncons() {
                    None => {
                        f248(v2)
                    }
                    Some((v3, v4)) => {
                        let m1080 = v4;
                        match m1080.uncons() {
                            None => {
                                f248(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f248(v2)?))
                            }
                        }
                    }
                }
            } else {
                f248(v2)
            }
        }
    }
}

pub fn f249(p1083: List<Bytes>, p1084: List<Bytes>) -> List<Bytes> {
    let m1082 = (p1083, p1084);
    {
        let mut m1082 = m1082;
        loop {
            let step = {
                let (v0, v1) = m1082;
                let m1081 = v0;
                match m1081.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f250() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f251(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1085 = v1;
    match m1085.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f371(v0.clone(), v3)?;
            if f52(f372(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f373(v5)?, f251(v0, v4, v2)?))
            } else {
                f251(v0, v4, v2)
            }
        }
    }
}

pub fn f252(v0: Adt1, p1089: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1088 = p1089;
    {
        let mut m1088 = m1088;
        loop {
            let step = {
                let v1 = m1088;
                let m1086 = v1;
                match m1086.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f371(v0.clone(), v3.clone())?;
                        if if f52(f372(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f101({
                                let m1087 = f94(v5.clone(), f8(Str::lit("schema")));
                                match m1087 {
                                    None => {
                                        Adt2::C9
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f373(v5)?))
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
                Ok(next) => { m1088 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f139(match v0.clone() {
        Adt1::C0(_, h1090, _, _) => {
            h1090
        }
    }, f285())) {
        Ok(Adt12::C1(List::cons(f285(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f152(f146(f374(f148(match v0.clone() {
            Adt1::C0(_, h1091, _, _) => {
                h1091
            }
        }, f285())))?, v1.clone());
        let m1092 = f56(v0.clone(), List::cons(f154(), List::cons(f217(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1092.uncons() {
            None => {
                let m1093 = f56(v0.clone(), f375(v0.clone(), v2.clone())?);
                match m1093.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f376(v0.clone(), v2, v1)?;
                        let (h1094, _) = v7;
                        if h1094 {
                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h1095) = v7;
                                h1095
                            }))
                        } else {
                            let m1096 = f125(v0.clone());
                            match m1096.uncons() {
                                None => {
                                    let m1097 = f127(v0.clone())?;
                                    match m1097.uncons() {
                                        None => {
                                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h1098) = v7;
                                                h1098
                                            }, List::cons(f7(f146(f377(f128(v0.clone())?, f136(v0.clone(), f137(v0.clone())?)?))?), List::cons(f378(v0)?, List::<Bytes>::nil()))))))
                                        }
                                        Some((v10, v11)) => {
                                            Ok(Adt12::C0(List::cons(v10, v11)))
                                        }
                                    }
                                }
                                Some((v8, v9)) => {
                                    Ok(Adt12::C1(List::cons(v8, v9)))
                                }
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        Ok(Adt12::C0(List::cons(v5, v6)))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt12::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f254(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f379(f97(v0, f154())?)?)))
}

pub fn f255() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The one way an agent changes this repository: dispatch this workflow with an\u{a}")), List::cons(f8(Str::lit("# operation of the API. It runs the certified command, and opens a pull request\u{a}")), List::cons(f8(Str::lit("# as the Verdant Studio GitHub App, which merges it once the studio check passes.\u{a}")), List::cons(f8(Str::lit("name: operate\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}")), List::cons(f8(Str::lit("# that defers to an open advance pull request always has a successor when that merges.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("    inputs:\u{a}")), List::cons(f8(Str::lit("      args:\u{a}")), List::cons(f8(Str::lit("        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}")), List::cons(f8(Str::lit("        required: true\u{a}")), List::cons(f8(Str::lit("        default: 'advance'\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input:\u{a}")), List::cons(f8(Str::lit("        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input_name:\u{a}")), List::cons(f8(Str::lit("        description: 'File name for input'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("        default: 'input'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Only operations that write the registers share one group. GitHub keeps one\u{a}")), List::cons(f8(Str::lit("# pending run per group, so a staging or propose-command run gets its own.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  # The wait is its own job under its own limit, so time spent waiting never comes out of the 30 minutes the operation has.\u{a}  wait:\u{a}    if: ${{ !startsWith(inputs.args, 'revert-merge ') }}\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 12\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      # Main moves about every 10 minutes, and refresh's regeneration of an API change plus its check takes\u{a}")), List::cons(f8(Str::lit("      # about 12, so an operation merged in that window pushes the change behind main again (#309, #315).\u{a}")), List::cons(f8(Str::lit("      # refresh keeps a regenerated change ready with its auto-merge set, so this run waits for every such\u{a}")), List::cons(f8(Str::lit("      # change whose studio check has not failed to merge first, then starts from the new main. It waits at\u{a}")), List::cons(f8(Str::lit("      # most 10 minutes, so neither queue stalls. A new propose-command waits too, so it never merges ahead of an\u{a}")), List::cons(f8(Str::lit("      # older change that is being regenerated; revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("      - name: wait for the API changes that are next to merge (every operation but revert-merge)\u{a}")), List::cons(f8(Str::lit("")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          for try in $(seq 1 20); do\u{a}")), List::cons(f8(Str::lit("            next=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio propose-command\")) and .auto_merge != null and (.draft | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then next=\"$next #${pr}\"; fi\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f256() -> Bytes {
    f7(List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$next\" ]; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("            echo \"waiting for${next} to merge (check $try of 20)\"\u{a}")), List::cons(f8(Str::lit("            sleep 30\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          echo \"::warning::${next# } still open after 10 minutes, so this run goes on\"\u{a}  operate:\u{a}    needs: wait\u{a}    if: ${{ !cancelled() && (needs.wait.result == 'success' || needs.wait.result == 'skipped') }}\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}")), List::cons(f8(Str::lit("        if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          mkdir -p target/input\u{a}")), List::cons(f8(Str::lit("          cat > target/input/change.patch <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          # A sender can name the sha256 of the patch it means to send on a first line of its own, sha256:HEX, the digest of\u{a}")), List::cons(f8(Str::lit("          # the patch text as applied from its first diff --git line, ending in one newline (or of the gzip's content), so\u{a}")), List::cons(f8(Str::lit("          # neither the account before it nor a newline the dispatch drops changes it.\u{a}")), List::cons(f8(Str::lit("          # A slip in transcription then fails here in seconds.\u{a}")), List::cons(f8(Str::lit("          want=''\u{a}")), List::cons(f8(Str::lit("          if head -n 1 target/input/change.patch | grep -qE '^sha256:[0-9a-f]{64}$'; then want=$(head -n 1 target/input/change.patch | cut -c8-); tail -n +2 target/input/change.patch > target/input/change.body; mv target/input/change.body target/input/change.patch; fi\u{a}")), List::cons(f8(Str::lit("          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}")), List::cons(f8(Str::lit("          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$want\" ]; then got=$(awk 'f || index($0, \"diff --git \") == 1 { f = 1; if (length($0) == 0) { n++ } else { while (n > 0) { print \"\"; n-- } print } }' target/input/change.patch | sha256sum | cut -c1-64); if [ \"$got\" != \"$want\" ]; then echo \"::error::the patch decodes to sha256 $got, not the $want its sender named, so it changed in transcription: send it again\"; exit 1; fi; echo \"the patch matches the sha256 its sender named\"; fi\u{a}")), List::cons(f8(Str::lit("          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^admit/' -e '^lexlean-workspace.toml$' -e '^studio/lexlean.toml$' -e '^studio/production-request.json$' -e '^app/lexlean.toml$' -e '^stdlib/lexlean.toml$' -e '^.github/lexlean.pin$' || true)\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers, admit, lexlean-workspace.toml, the studio, app and stdlib lexlean.toml, studio/production-request.json and .github/lexlean.pin, not: $outside\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          # studio/lexlean.lock, readers/lexlean.lock, readers/xml/lexlean.lock and admit/lexlean.lock are generated, and the export below writes them again.\u{a}")), List::cons(f8(Str::lit("          # A VS ID this change adds that main already has becomes the next free one, and a pure append is\u{a}          # appended on main instead of patched (prepare-change.py, which refresh runs too).\u{a}          python3 .github/scripts/prepare-change.py prepare target/input/change.patch target/input/appends.json\u{a}          # git apply drops a file whose binary patch is corrupt, says so on one error line and still exits 0, so any error line stops the run here, before the export.\u{a}          # Contract and strata keep their entries on one line, so a change that only adds entries there is merged\u{a}          # into main by entry, and never conflicts with another change that adds others (merge-entries.py).\u{a}          python3 .github/scripts/merge-entries.py patch target/input/change.patch\u{a}          : > target/apply.err\u{a}          if grep -q '^diff --git' target/input/change.patch; then git apply --exclude=studio/lexlean.lock --exclude=readers/lexlean.lock --exclude=readers/xml/lexlean.lock --exclude=admit/lexlean.lock target/input/change.patch 2> target/apply.err || { sed 's/^/::error::/' target/apply.err; exit 1; }; fi\u{a}          if grep -q '^error' target/apply.err; then sed 's/^/::error::the patch did not apply whole: /' target/apply.err; exit 1; fi\u{a}          python3 .github/scripts/prepare-change.py append target/input/appends.json\u{a}")), List::cons(f8(Str::lit("          # The owner reads this pull request, so its body leads with the proposer's own account (the prose\u{a}")), List::cons(f8(Str::lit("          # before the first diff), then the definitions it changes. Checked before the export, which takes minutes.\u{a}")), List::cons(f8(Str::lit("          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}")), List::cons(f8(Str::lit("          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The export refuses Lean output holding an absolute path (LLV7006) only at its end, so refuse here, in seconds,\u{a}")), List::cons(f8(Str::lit("          # an added string literal that LexLean's own test would refuse, read as Lean prints it (prepare-change.py lint).\u{a}")), List::cons(f8(Str::lit("          python3 .github/scripts/prepare-change.py lint target/input/change.patch\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          # The one export, shared with refresh and run end to end by the studio check.\u{a}")), List::cons(f8(Str::lit("          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f380(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("            tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}")), List::cons(f8(Str::lit("          {\u{a}")), List::cons(f8(Str::lit("            cat target/summary.md\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"### Definitions this adds or changes\"\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f257() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            {\u{a}")), List::cons(f8(Str::lit("              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}")), List::cons(f8(Str::lit("              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}")), List::cons(f8(Str::lit("            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            slash=/\u{a}")), List::cons(f8(Str::lit("            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            head -n 8 target/export.log.phases || true\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"<${slash}details>\"\u{a}")), List::cons(f8(Str::lit("          } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("      - id: command\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: |\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_readers\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_xml\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_admit\u{a}")), List::cons(f8(Str::lit("          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}")), List::cons(f8(Str::lit("        if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          target/lexlean/bin/lexlean production verify-artifact certified\u{a}")), List::cons(f8(Str::lit("          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}")), List::cons(f8(Str::lit("          if [ -d certified-readers ]; then\u{a}")), List::cons(f8(Str::lit("            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}")), List::cons(f8(Str::lit("            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}")), List::cons(f8(Str::lit("      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("          INPUT_NAME: ${{ inputs.input_name }}\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail -f\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          # read-* operations run in their reader project's command when it is built (the XML ones in verdant_xml), the admission operations in verdant_admit, every other in the studio command.\u{a}")), List::cons(f8(Str::lit("          vs() { case \"$1\" in admit|admit-encoding|admit-registry|admit-translated|propose-encoding) if [ -x target/release/verdant_admit ]; then target/release/verdant_admit \"$@\"; return; fi;; read-xml|read-xsd|read-xsd-components) if [ -x target/release/verdant_xml ]; then target/release/verdant_xml \"$@\"; return; fi;; read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f258() -> Bytes {
    f7(List::cons(f8(Str::lit("          # An App pull request whose files already hold on main has nothing left to merge,\u{a}")), List::cons(f8(Str::lit("          # so it is closed; the propose-command ones change the API and stay open until they merge.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}")), List::cons(f8(Str::lit("              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}")), List::cons(f8(Str::lit("              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}")), List::cons(f8(Str::lit("              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}")), List::cons(f8(Str::lit("                exit 0\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            # A change to a reader re-derives the encodings it printed before the studio check replays them.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then\u{a}")), List::cons(f8(Str::lit("              readmitted=$(bash .github/scripts/readmit.sh target/release/verdant_readers; if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi)\u{a}")), List::cons(f8(Str::lit("              if [ -n \"$readmitted\" ]; then\u{a}")), List::cons(f8(Str::lit("                { echo; echo \"### Encodings admitted again\"; echo; echo \"This change alters what a reader prints for these admitted encodings, so each was admitted again through read-admit with the command and citation its row records:\"; echo; sed 's/^/- /' <<< \"$readmitted\"; } >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            cat target/operation.txt\u{a}")), List::cons(f8(Str::lit("          elif [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            : > target/operation.txt\u{a}")), List::cons(f8(Str::lit("            last=''\u{a}")), List::cons(f8(Str::lit("            skipped=''\u{a}")), List::cons(f8(Str::lit("            : > target/repeated.txt\u{a}")), List::cons(f8(Str::lit("            # next prints one operation it can dispatch, or else the lines that say why the\u{a}")), List::cons(f8(Str::lit("            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}")), List::cons(f8(Str::lit("            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}")), List::cons(f8(Str::lit("              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("              if vs $op > target/attempt.txt 2>&1; then\u{a}")), List::cons(f8(Str::lit("                tee -a target/operation.txt < target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                last=$op\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                cat target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                binding=${op#* }\u{a}")), List::cons(f8(Str::lit("                echo \"::warning::$op was refused; the other bindings continue\"\u{a}")), List::cons(f8(Str::lit("                echo \"refused: $op\" >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}")), List::cons(f8(Str::lit("                skipped=\"$skipped $binding\"\u{a}")), List::cons(f8(Str::lit("                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt >> target/repeated.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            t0=$EPOCHREALTIME\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}")), List::cons(f8(Str::lit("            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}")), List::cons(f8(Str::lit("            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}")), List::cons(f8(Str::lit("            grep '^waiting fix' target/repeated.txt >> target/waiting.txt || true\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}")), List::cons(f8(Str::lit("              echo \"waiting merge: #${pr} changes the API, and merges once the studio check passes\" >> target/waiting.txt\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}")), List::cons(f8(Str::lit("          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f259() -> Bytes {
    f7(List::cons(f8(Str::lit("            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}")), List::cons(f8(Str::lit("            # good main commit, it reverts the merge on main as the App, and merges once the studio check passes.\u{a}")), List::cons(f8(Str::lit("            merge=${ARGS#revert-merge }\u{a}")), List::cons(f8(Str::lit("            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B revert origin/main\u{a}")), List::cons(f8(Str::lit("            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("            git revert -m 1 --no-commit \"$merge\"\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::cons(f8(Str::lit("            vs $ARGS | tee target/operation.txt\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("      - name: open a pull request as the Verdant Studio app\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          git add -A\u{a}")), List::cons(f8(Str::lit("          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}")), List::cons(f8(Str::lit("          verdant_studio $ARGS\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("          $(cat target/operation.txt)\u{a}")), List::cons(f8(Str::lit("          MSG\u{a}")), List::cons(f8(Str::lit("          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}")), List::cons(f8(Str::lit("          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}")), List::cons(f8(Str::lit("            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          title=\"verdant_studio $ARGS\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}")), List::cons(f8(Str::lit("          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}")), List::cons(f8(Str::lit("          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}")), List::cons(f8(Str::lit("          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}")), List::cons(f8(Str::lit("          pr=\"\"\u{a}")), List::cons(f8(Str::lit("          for try in 1 2 3 4; do\u{a}")), List::cons(f8(Str::lit("            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            pr=\"\"\u{a}")), List::cons(f8(Str::lit("            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}")), List::cons(f8(Str::lit("            sleep $((2 ** try))\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          echo \"opened #${pr}\"\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("          # A change to the API itself (propose-command, revert-merge) merges as every App pull request does:\u{a}")), List::cons(f8(Str::lit("          # the owner automated these approvals (2026-11-26), so the studio check, which runs the API's own\u{a}")), List::cons(f8(Str::lit("          # end-to-end propose run, is its gate. refresh keeps its auto-merge while it regenerates it.\u{a}")), List::cons(f8(Str::lit("          # An API change merges before every change opened after it, however long each check takes: #478 and\u{a}")), List::cons(f8(Str::lit("          # #479 merged ahead of #474 after the wait above gave up, and each pushed it behind main again.\u{a}")), List::cons(f8(Str::lit("          # So while an older propose-command pull request keeps its auto-merge and its studio check has not\u{a}")), List::cons(f8(Str::lit("          # failed, this one is left ready without auto-merge, and refresh sets it once that one merges.\u{a}")), List::cons(f8(Str::lit("          # revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("          ahead=''\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" = \"$ARGS\" ]; then\u{a}")), List::cons(f8(Str::lit("            for older in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select(.user.type == \\\"Bot\\\" and (.title | startswith(\\\"verdant_studio propose-command\\\")) and .auto_merge != null and .number < ${pr}) | .number\"); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${older}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then ahead=\"$ahead #${older}\"; fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("          if [ \"$gate\" = merge ] && [ -n \"$ahead\" ]; then\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::#${pr} merges after${ahead}: refresh sets its auto-merge once they merge\"\u{a}")), List::cons(f8(Str::lit("          elif [ \"$gate\" = merge ]; then\u{a}")), List::cons(f8(Str::lit("            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f260() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::<Bytes>::nil())))
}

pub fn f261() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# When main moves, every open propose-command pull request that no longer merges\u{a}")), List::cons(f8(Str::lit("# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}")), List::cons(f8(Str::lit("# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}")), List::cons(f8(Str::lit("# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}")), List::cons(f8(Str::lit("# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}")), List::cons(f8(Str::lit("# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}")), List::cons(f8(Str::lit("# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("# The exception is one whose auto-merge is set, the next change to merge: it stays ready and keeps its\u{a}")), List::cons(f8(Str::lit("# auto-merge, so it merges as soon as its regeneration passes the studio check. It is\u{a}")), List::cons(f8(Str::lit("# regenerated first, and only the oldest such one at a time, since two changes to the\u{a}")), List::cons(f8(Str::lit("# API conflict on their export (#309 and #315 were pushed behind main this way).\u{a}")), List::cons(f8(Str::lit("name: refresh\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}")), List::cons(f8(Str::lit("# operate run completes, which needs no token that may dispatch workflows, so every\u{a}")), List::cons(f8(Str::lit("# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_run:\u{a}")), List::cons(f8(Str::lit("    workflows: [operate]\u{a}")), List::cons(f8(Str::lit("    types: [completed]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A running refresh always finishes, because an export takes minutes and main moves more\u{a}")), List::cons(f8(Str::lit("# often than that; GitHub keeps only the newest pending run behind it.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: refresh\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  find:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 15\u{a}")), List::cons(f8(Str::lit("    outputs:\u{a}")), List::cons(f8(Str::lit("      stale: ${{ steps.stale.outputs.stale }}\u{a}")), List::cons(f8(Str::lit("      matrix: ${{ steps.stale.outputs.matrix }}\u{a}")), List::cons(f8(Str::lit("      auto: ${{ steps.stale.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - id: stale\u{a}")), List::cons(f8(Str::lit("        name: find the propose-command pull requests that conflict with main\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          stale=''\u{a}")), List::cons(f8(Str::lit("          auto=''\u{a}")), List::cons(f8(Str::lit("          # Oldest first, so the oldest one whose auto-merge is set is the one regenerated now.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}")), List::cons(f8(Str::lit("            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}")), List::cons(f8(Str::lit("            # which a cancelled run may have left behind.\u{a}")), List::cons(f8(Str::lit("            mergeable=null\u{a}")), List::cons(f8(Str::lit("            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}")), List::cons(f8(Str::lit("              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f262() -> Bytes {
    f7(List::cons(f8(Str::lit("              if [ \"$mergeable\" != null ]; then break; fi\u{a}")), List::cons(f8(Str::lit("              sleep 10\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            # One that changes .github is exported again by the studio check, byte for byte, so it also fails\u{a}")), List::cons(f8(Str::lit("            # when main changed its export inputs since it was exported, even without a conflict: readers\u{a}")), List::cons(f8(Str::lit("            # depends on app, so admissions merged during its export (#357 and #361 under #362) turn it red.\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = true ] && [ \"$(gh api --paginate \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files\" --jq '.[].filename' | grep -c '^\\.github/' || true)\" != 0 ]; then\u{a}")), List::cons(f8(Str::lit("              base=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/main...$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\" --jq .merge_base_commit.sha)\u{a}")), List::cons(f8(Str::lit("              moved=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/${base}...main\" --jq '[.files[]?.filename | select(test(\"^(app|stdlib|studio|readers)/\"))] | length')\u{a}")), List::cons(f8(Str::lit("              if [ \"$moved\" != 0 ]; then echo \"#${pr}: main changed ${moved} of its export inputs since it was exported\"; mergeable=false; fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = false ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              # Drafting would drop its auto-merge, so it stays ready. A later one waits: it is\u{a}")), List::cons(f8(Str::lit("              # regenerated once this one merges, when it would conflict again anyway.\u{a}")), List::cons(f8(Str::lit("              if [ -z \"$auto\" ]; then auto=$pr; else echo \"#${pr} waits for #${auto} to merge\"; fi\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = false ]; then\u{a}")), List::cons(f8(Str::lit("              stale=\"$stale $pr\"\u{a}")), List::cons(f8(Str::lit("              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}")), List::cons(f8(Str::lit("              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          # A change operate left without auto-merge, behind an older API change, gets it here once no\u{a}")), List::cons(f8(Str::lit("          # propose-command pull request keeps its auto-merge with a studio check that has not failed: oldest\u{a}")), List::cons(f8(Str::lit("          # first, up to and including the first API change, so API changes still merge one at a time in order.\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > \"$RUNNER_TEMP/rules.json\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$(jq '[.[] | select(.type == \"required_status_checks\") | .parameters.required_status_checks[]?.context] | index(\"studio\") != null' \"$RUNNER_TEMP/rules.json\")\" = true ]; then\u{a}")), List::cons(f8(Str::lit("            armed=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select(.user.login == \\\"${SLUG}[bot]\\\" and (.title | startswith(\\\"verdant_studio propose-command\\\")) and .auto_merge != null) | .number\"); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then armed=\"$armed #${pr}\"; fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$armed\" ]; then\u{a}")), List::cons(f8(Str::lit("              for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select(.user.login == \\\"${SLUG}[bot]\\\" and .auto_merge == null and (.draft | not)) | .number\"); do\u{a}")), List::cons(f8(Str::lit("                if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("                  gh pr merge \"$pr\" --auto --squash --delete-branch --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("                  echo \"#${pr}: auto-merge set\"\u{a}")), List::cons(f8(Str::lit("                  if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.title | startswith(\"verdant_studio propose-command\")')\" = true ]; then break; fi\u{a}")), List::cons(f8(Str::lit("                fi\u{a}")), List::cons(f8(Str::lit("              done\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$auto\" ]; then stale=\" $auto$stale\"; fi\u{a}")), List::cons(f8(Str::lit("          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"auto=$auto\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("  regenerate:\u{a}")), List::cons(f8(Str::lit("    needs: find\u{a}")), List::cons(f8(Str::lit("    if: needs.find.outputs.stale != ''\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 60\u{a}")), List::cons(f8(Str::lit("    strategy:\u{a}")), List::cons(f8(Str::lit("      fail-fast: false\u{a}")), List::cons(f8(Str::lit("      matrix:\u{a}")), List::cons(f8(Str::lit("        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          fetch-depth: 0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          STALE: ${{ matrix.pr }}\u{a}")), List::cons(f8(Str::lit("          AUTO: ${{ needs.find.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A regeneration of the one whose auto-merge is set that fails leaves it a draft, which drops\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f263() -> Bytes {
    f7(List::cons(f8(Str::lit("          # its auto-merge, so it is no longer the next change to merge and nothing is handed over red.\u{a}")), List::cons(f8(Str::lit("          trap 'rc=$?; if [ \"$rc\" != 0 ] && [ \"$STALE\" = \"$AUTO\" ]; then gh pr ready --undo \"$STALE\" || true; fi' EXIT\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            mkdir -p target\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          failed=''\u{a}")), List::cons(f8(Str::lit("          for pr in $STALE; do\u{a}")), List::cons(f8(Str::lit("            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}")), List::cons(f8(Str::lit("            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}")), List::cons(f8(Str::lit("            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("            # The one whose auto-merge is set stays ready, so that it keeps its auto-merge.\u{a}")), List::cons(f8(Str::lit("            if [ \"$pr\" != \"$AUTO\" ]; then gh pr ready --undo \"$pr\" || true; fi\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin \"$branch\" main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B refresh origin/main\u{a}")), List::cons(f8(Str::lit("            git clean -fdq\u{a}")), List::cons(f8(Str::lit("            # readers/lexlean.lock and readers/xml/lexlean.lock are generated: the export below writes them again, so they are left out of the\u{a}")), List::cons(f8(Str::lit("            # change, and two reader pull requests never conflict on it.\u{a}")), List::cons(f8(Str::lit("            git diff --full-index \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml ':!readers/lexlean.lock' ':!readers/xml/lexlean.lock' ':!admit/lexlean.lock' > target/change.patch\u{a}")), List::cons(f8(Str::lit("            # app/src/Contract.lex.tex and model/strata.json each keep their entries on one line, so two changes\u{a}")), List::cons(f8(Str::lit("            # that each add entries there always conflict. A conflict on those two files alone is merged by entry\u{a}")), List::cons(f8(Str::lit("            # (merge-entries.py): main's file, then each entry the pull request adds that main does not have.\u{a}")), List::cons(f8(Str::lit("            # git apply drops a file whose binary patch is corrupt and still exits 0, so that error counts as not applying.\u{a}            # The same path as propose-command: renumber colliding VS IDs, move pure appends aside, and merge\u{a}            # Contract's and strata's entries by entry, so a lane that loses an ID or entry race never dispatches again.\u{a}            python3 .github/scripts/prepare-change.py prepare target/change.patch target/appends.json\u{a}            python3 .github/scripts/merge-entries.py patch target/change.patch\u{a}            applied=yes\u{a}            : > target/apply.err\u{a}            if grep -q '^diff --git' target/change.patch; then git apply --3way target/change.patch 2> target/apply.err || applied=no; fi\u{a}            if grep -q '^error: corrupt' target/apply.err; then applied=no; fi\u{a}            if [ \"$applied\" = no ]; then\u{a}")), List::cons(f8(Str::lit("              merged=no\u{a}")), List::cons(f8(Str::lit("              if [ -n \"$(git diff --name-only --diff-filter=U)\" ] && [ -z \"$(git diff --name-only --diff-filter=U | grep -v -x -e app/src/Contract.lex.tex -e model/strata.json || true)\" ]; then\u{a}")), List::cons(f8(Str::lit("                merged=yes\u{a}")), List::cons(f8(Str::lit("                base=$(git merge-base origin/main \"origin/$branch\")\u{a}")), List::cons(f8(Str::lit("                for f in $(git diff --name-only --diff-filter=U); do\u{a}")), List::cons(f8(Str::lit("                  git show \"$base:$f\" > target/entries-base\u{a}")), List::cons(f8(Str::lit("                  git show \"origin/$branch:$f\" > target/entries-pr\u{a}")), List::cons(f8(Str::lit("                  git show \"origin/main:$f\" > target/entries-main\u{a}")), List::cons(f8(Str::lit("                  if python3 .github/scripts/merge-entries.py merge target/entries-base target/entries-pr target/entries-main \"$f\" > target/entries-merged; then cp target/entries-merged \"$f\"; git add \"$f\"; else merged=no; fi\u{a}")), List::cons(f8(Str::lit("                done\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("              if [ \"$merged\" = yes ]; then\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::#${pr}: merged the entries it adds to Contract and strata into main's by entry\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}")), List::cons(f8(Str::lit("                git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("                failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}")), List::cons(f8(Str::lit("                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("                continue\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            python3 .github/scripts/prepare-change.py append target/appends.json\u{a}")), List::cons(f8(Str::lit("            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f380(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("              tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("              git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("              failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("              continue\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            rm -rf studio/.lexlean/production readers/.lexlean/production readers/xml/.lexlean/production admit/.lexlean/production\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            # The regenerated change re-derives the encodings whose reader output it changes, as propose-command does.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then bash .github/scripts/readmit.sh target/release/verdant_readers; fi\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("            git add -A\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f264() -> Bytes {
    f7(List::cons(f8(Str::lit("            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml; } > target/message.txt\u{a}")), List::cons(f8(Str::lit("            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}")), List::cons(f8(Str::lit("            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}")), List::cons(f8(Str::lit("            gh pr ready \"$pr\" || true\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::regenerated #${pr} on the new main\"\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))))))
}

pub fn f265() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The owner's acceptance test for the whole pipeline (REQ-SELF-HOSTED, REQ-PRISM-GENERATION): prompt Studio\u{a}")), List::cons(f8(Str::lit("# with the seed \"Verdant Studio\" and require every facet the Prism lists to have its media type at the\u{a}")), List::cons(f8(Str::lit("# fidelity its generator's theorems state. Run from the repository root after the build, so the app is not\u{a}")), List::cons(f8(Str::lit("# proved again: prism-acceptance.py COMMAND... runs COMMAND run app --input REQUEST for each request.\u{a}")), List::cons(f8(Str::lit("import math, struct, subprocess, sys, xml.etree.ElementTree as tree\u{a}")), List::cons(f8(Str::lit("SEED = 'Verdant Studio'\u{a}")), List::cons(f8(Str::lit("MIDI_HEADER = bytes.fromhex('4d54686400000006000000010060')\u{a}")), List::cons(f8(Str::lit("FMT = bytes.fromhex('0100010080bb00000077010002001000')\u{a}")), List::cons(f8(Str::lit("# SONY-ASWG-R001 (the AswgLoudnessTarget catalog module), the home-based platform target REQ-ENGINE-FIDELITY's high-end gaming\u{a}")), List::cons(f8(Str::lit("# systems name: -24 (+-2) LKFS integrated, at most -1 dBTP true peak, both measured as ITU-R-BS-1770-5 states.\u{a}")), List::cons(f8(Str::lit("TARGET, TOLERANCE, TRUE_PEAK = -24.0, 2.0, -1.0\u{a}")), List::cons(f8(Str::lit("# BS.1770-5 Annex 1 pre-filter and RLB filter at 48 kHz, as (b0, b1, b2, a1, a2).\u{a}")), List::cons(f8(Str::lit("K_WEIGHTING = ((1.53512485958697, -2.69169618940638, 1.19839281085285, -1.69065929318241, 0.73248077421585),\u{a}")), List::cons(f8(Str::lit("               (1.0, -2.0, 1.0, -1.99004745483398, 0.99007225036621))\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def loudness(x):\u{a}")), List::cons(f8(Str::lit("    # Annex 1: K-weighting, 400 ms blocks overlapping by three quarters, absolute gate -70 LKFS, relative gate -10 LU.\u{a}")), List::cons(f8(Str::lit("    for b0, b1, b2, a1, a2 in K_WEIGHTING:\u{a}")), List::cons(f8(Str::lit("        y, x1, x2, y1, y2 = [], 0.0, 0.0, 0.0, 0.0\u{a}")), List::cons(f8(Str::lit("        for v in list(x):\u{a}")), List::cons(f8(Str::lit("            o = b0 * v + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2\u{a}")), List::cons(f8(Str::lit("            x2, x1, y2, y1 = x1, v, y1, o\u{a}")), List::cons(f8(Str::lit("            y.append(o)\u{a}")), List::cons(f8(Str::lit("        x = y\u{a}")), List::cons(f8(Str::lit("    squares = [0.0]\u{a}")), List::cons(f8(Str::lit("    for v in list(x):\u{a}")), List::cons(f8(Str::lit("        squares.append(squares[-1] + v * v)\u{a}")), List::cons(f8(Str::lit("    size, hop = 19200, 4800\u{a}")), List::cons(f8(Str::lit("    blocks = [(squares[at + size] - squares[at]) * size ** -1 for at in range(0, len(x) - size + 1, hop)]\u{a}")), List::cons(f8(Str::lit("    level = lambda power: -0.691 + 10 * math.log10(power) if power > 0 else -math.inf\u{a}")), List::cons(f8(Str::lit("    gated = [power for power in blocks if level(power) > -70]\u{a}")), List::cons(f8(Str::lit("    if not gated:\u{a}")), List::cons(f8(Str::lit("        return -math.inf\u{a}")), List::cons(f8(Str::lit("    relative = level(sum(gated) * len(gated) ** -1) - 10\u{a}")), List::cons(f8(Str::lit("    gated = [power for power in gated if level(power) > relative]\u{a}")), List::cons(f8(Str::lit("    return level(sum(gated) * len(gated) ** -1)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def true_peak(x):\u{a}")), List::cons(f8(Str::lit("    # Annex 2: oversample by 4 and take the greatest magnitude, here with a 48-tap Hann-windowed sinc interpolator.\u{a}")), List::cons(f8(Str::lit("    taps = [[(math.sin(math.pi * (k - 24 + phase * 0.25)) * (math.pi * (k - 24 + phase * 0.25)) ** -1 if k - 24 + phase * 0.25 else 1.0)\u{a}")), List::cons(f8(Str::lit("             * (0.5 - 0.5 * math.cos(2 * math.pi * (k + phase * 0.25) * 48 ** -1)) for k in range(48)] for phase in range(1, 4)]\u{a}")), List::cons(f8(Str::lit("    peak = max(map(abs, x), default=0.0)\u{a}")), List::cons(f8(Str::lit("    for at in range(len(x)):\u{a}")), List::cons(f8(Str::lit("        if abs(x[at]) < peak * 0.7:\u{a}")), List::cons(f8(Str::lit("            continue\u{a}")), List::cons(f8(Str::lit("        window = x[max(0, at - 23):at + 25]\u{a}")), List::cons(f8(Str::lit("        for row in taps:\u{a}")), List::cons(f8(Str::lit("            peak = max(peak, abs(sum(c * v for c, v in zip(row[23 - (at - max(0, at - 23)):], window))))\u{a}")), List::cons(f8(Str::lit("    return 20 * math.log10(peak) if peak > 0 else -math.inf\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("SVG_ROOT = '{http:' + chr(47) * 2 + 'www.w3.org' + chr(47) + '2000' + chr(47) + 'svg}svg'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def run(request):\u{a}")), List::cons(f8(Str::lit("    got = subprocess.run(sys.argv[1:] + ['run', 'app', '--input', request, '--rm'], capture_output=True)\u{a}")), List::cons(f8(Str::lit("    if got.returncode != 0:\u{a}")), List::cons(f8(Str::lit("        raise SystemExit('::error::' + repr(request) + ' failed: ' + got.stderr.decode(errors='replace')[-400:])\u{a}")), List::cons(f8(Str::lit("    return got.stdout\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f266() -> Bytes {
    f7(List::cons(f8(Str::lit("def midi(data):\u{a}")), List::cons(f8(Str::lit("    # Midi.lex.tex writes_a_format_0_file: format 0, one track, 96 ticks per quarter note.\u{a}")), List::cons(f8(Str::lit("    if not data.startswith(MIDI_HEADER) or data[14:18] != b'MTrk':\u{a}")), List::cons(f8(Str::lit("        return 'is no format 0 SMF with one track at 96 ticks per quarter note'\u{a}")), List::cons(f8(Str::lit("    if struct.unpack('>I', data[18:22])[0] != len(data) - 22:\u{a}")), List::cons(f8(Str::lit("        return 'has a track length that is not the rest of the file'\u{a}")), List::cons(f8(Str::lit("    # Coverage (Der from REQ-PRISM-GENERATION): every facet carries every feature. The SMF carries each as a Marker\u{a}")), List::cons(f8(Str::lit("    # meta event (FF 06, MIDI-SMF-RP-001-1.0-PAGE), in the order the features route lists them.\u{a}")), List::cons(f8(Str::lit("    markers, at = [], 22\u{a}")), List::cons(f8(Str::lit("    while at < len(data):\u{a}")), List::cons(f8(Str::lit("        while data[at] & 128:\u{a}")), List::cons(f8(Str::lit("            at += 1\u{a}")), List::cons(f8(Str::lit("        at += 1\u{a}")), List::cons(f8(Str::lit("        if data[at] == 255:\u{a}")), List::cons(f8(Str::lit("            kind, size, at = data[at + 1], 0, at + 2\u{a}")), List::cons(f8(Str::lit("            while data[at] & 128:\u{a}")), List::cons(f8(Str::lit("                size, at = size * 128 + (data[at] & 127), at + 1\u{a}")), List::cons(f8(Str::lit("            size, at = size * 128 + data[at], at + 1\u{a}")), List::cons(f8(Str::lit("            if kind == 6:\u{a}")), List::cons(f8(Str::lit("                markers.append(data[at:at + size].decode(errors='replace'))\u{a}")), List::cons(f8(Str::lit("            at += size\u{a}")), List::cons(f8(Str::lit("        else:\u{a}")), List::cons(f8(Str::lit("            at += 2 if data[at] >> 4 in (12, 13) else 3\u{a}")), List::cons(f8(Str::lit("    if markers != FEATURES:\u{a}")), List::cons(f8(Str::lit("        return 'marks the features ' + repr(markers) + ', not ' + repr(FEATURES)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def wave(data):\u{a}")), List::cons(f8(Str::lit("    # Bwf.lex.tex format_is_pcm_mono_48000_16 and wave_orders_bext_fmt_data: bext (646 bytes), fmt, data.\u{a}")), List::cons(f8(Str::lit("    if data[:4] != b'RIFF' or data[8:12] != b'WAVE' or struct.unpack('<I', data[4:8])[0] != len(data) - 8:\u{a}")), List::cons(f8(Str::lit("        return 'is no RIFF WAVE file of its own length'\u{a}")), List::cons(f8(Str::lit("    chunks, at = [], 12\u{a}")), List::cons(f8(Str::lit("    while at + 8 <= len(data):\u{a}")), List::cons(f8(Str::lit("        name, size = data[at:at + 4], struct.unpack('<I', data[at + 4:at + 8])[0]\u{a}")), List::cons(f8(Str::lit("        chunks.append((name, data[at + 8:at + 8 + size]))\u{a}")), List::cons(f8(Str::lit("        at += 8 + size + (size & 1)\u{a}")), List::cons(f8(Str::lit("    if [name for name, body in chunks] != [b'bext', b'fmt ', b'data']:\u{a}")), List::cons(f8(Str::lit("        return 'has chunks ' + repr([name for name, body in chunks]) + ', not bext, fmt and data'\u{a}")), List::cons(f8(Str::lit("    if len(chunks[0][1]) != 646:\u{a}")), List::cons(f8(Str::lit("        return 'has a bext chunk of ' + str(len(chunks[0][1])) + ' bytes, not 646'\u{a}")), List::cons(f8(Str::lit("    if chunks[1][1] != FMT:\u{a}")), List::cons(f8(Str::lit("        return 'is not PCM, mono, 48000 Hz, 16-bit'\u{a}")), List::cons(f8(Str::lit("    if not chunks[2][1]:\u{a}")), List::cons(f8(Str::lit("        return 'has no samples'\u{a}")), List::cons(f8(Str::lit("    samples = [value * 2.0 ** -15 for value in struct.unpack('<' + str(len(chunks[2][1]) >> 1) + 'h', chunks[2][1])]\u{a}")), List::cons(f8(Str::lit("    measured, peak = loudness(samples), true_peak(samples)\u{a}")), List::cons(f8(Str::lit("    if abs(measured - TARGET) > TOLERANCE:\u{a}")), List::cons(f8(Str::lit("        return 'reads ' + format(measured, '.1f') + ' LKFS, outside the -24 (+-2) LKFS home-platform target of ASWG-R001'\u{a}")), List::cons(f8(Str::lit("    if peak > TRUE_PEAK:\u{a}")), List::cons(f8(Str::lit("        return 'peaks at ' + format(peak, '.2f') + ' dBTP, above the -1 dBTP of ASWG-R001'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def svg(data):\u{a}")), List::cons(f8(Str::lit("    try:\u{a}")), List::cons(f8(Str::lit("        root = tree.fromstring(data)\u{a}")), List::cons(f8(Str::lit("    except tree.ParseError as failure:\u{a}")), List::cons(f8(Str::lit("        return 'is not well-formed XML: ' + str(failure)\u{a}")), List::cons(f8(Str::lit("    if root.tag != SVG_ROOT:\u{a}")), List::cons(f8(Str::lit("        return 'has the root ' + root.tag + ', not svg in the SVG namespace'\u{a}")), List::cons(f8(Str::lit("    # Coverage (Der from REQ-PRISM-GENERATION): the artwork's text carries every feature of the Prism.\u{a}")), List::cons(f8(Str::lit("    words = ' '.join(root.itertext()).split()\u{a}")), List::cons(f8(Str::lit("    missing = [feature for feature in FEATURES if feature not in words]\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f267() -> Bytes {
    f7(List::cons(f8(Str::lit("    if missing:\u{a}")), List::cons(f8(Str::lit("        return 'leaves out the features ' + repr(missing)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def bundle(data):\u{a}")), List::cons(f8(Str::lit("    if data[:4] != b'PK' + bytes([3, 4]) or b'bagit.txt' not in data:\u{a}")), List::cons(f8(Str::lit("        return 'is no ZIP holding a BagIt bag'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("RULES = {'audio/midi': midi, 'audio/wave': wave, 'image/svg+xml': svg, 'application/zip': bundle}\u{a}")), List::cons(f8(Str::lit("FEATURES = [line for line in run('prism' + chr(9) + 'features' + chr(9) + SEED).decode().split(chr(10)) if line]\u{a}")), List::cons(f8(Str::lit("facets = [line for line in run('prism' + chr(9) + chr(9) + SEED).decode().split(chr(10)) if line]\u{a}")), List::cons(f8(Str::lit("if not facets:\u{a}")), List::cons(f8(Str::lit("    raise SystemExit('::error::the Prism of ' + SEED + ' lists no facet')\u{a}")), List::cons(f8(Str::lit("failed = False\u{a}")), List::cons(f8(Str::lit("for facet in facets:\u{a}")), List::cons(f8(Str::lit("    data = run('prism' + chr(9) + facet + chr(9) + SEED)\u{a}")), List::cons(f8(Str::lit("    rule = RULES.get(facet)\u{a}")), List::cons(f8(Str::lit("    problem = 'has no acceptance rule, so its fidelity is unchecked' if rule is None else rule(data)\u{a}")), List::cons(f8(Str::lit("    if problem:\u{a}")), List::cons(f8(Str::lit("        failed = True\u{a}")), List::cons(f8(Str::lit("        print('::error::the ' + facet + ' facet of ' + SEED + ' ' + problem)\u{a}")), List::cons(f8(Str::lit("    else:\u{a}")), List::cons(f8(Str::lit("        print('accepted: the ' + facet + ' facet of ' + SEED + ', ' + str(len(data)) + ' bytes')\u{a}")), List::cons(f8(Str::lit("sys.exit(1 if failed else 0)\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))
}

pub fn f268(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1099 = v0;
    match m1099.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f269(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt19 = f381(v0.clone(), 0u64, {
        let a1100 = v0;
        length_bytes(a1100)
    }, Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt19 = {
        let a1103 = {
            let a1102 = match v1.clone() {
                Adt19::C0(_, _, h1101, _, _) => {
                    h1101
                }
            };
            length_bytes(a1102)
        };
        let a1104 = 0u64;
        if if nat_eq(a1103, a1104) {
            let m1106 = match v1.clone() {
                Adt19::C0(_, _, _, h1105, _) => {
                    h1105
                }
            };
            match m1106.uncons() {
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
            f382(v1)
        }
    };
    Ok(f383(match v4 {
        Adt19::C0(_, _, _, _, h1107) => {
            h1107
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f270(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt17>, v3: List<Adt2>, v4: List<Bytes>) -> List<Bytes> {
    let m1108 = v1;
    match m1108.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f126(f384(v5.clone(), v2.clone()), f385(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f386(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1110 = {
                    let a1109 = v7.clone();
                    length_list(a1109)
                };
                let a1111 = 0u64;
                if nat_eq(a1110, a1111) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f123(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f270(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f271(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1112 = v0;
    match m1112.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1113 = v1;
            match m1113.uncons() {
                None => {
                    f271(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f271(v2))
                }
            }
        }
    }
}

pub fn f272(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1114 = v1;
    match m1114.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f225(v2.clone(), f8(Str::lit("domain"))) {
                f387(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1115 = f94(v2.clone(), f8(Str::lit("role")));
                    match m1115 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f388(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1116 = f94(v2.clone(), f8(Str::lit("origin")));
                    match m1116 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1117 = f94(v2, f8(Str::lit("evidence")));
                    match m1117 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f272(v0, v3))
            } else {
                f272(v0, v3)
            }
        }
    }
}

pub fn f273(v0: List<Bytes>, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1118 = v1;
    match m1118.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            if f225(v2.clone(), f8(Str::lit("subject"))) {
                if f52(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                    List::cons(({
                        let a1119 = f8(Str::lit("a:"));
                        let a1120 = f58(v2.clone(), f8(Str::lit("object")));
                        append_bytes(a1119, a1120)
                    }, f46(f58(v2, f8(Str::lit("subject"))), 0u64, 1u64)), f273(v0, v3))
                } else {
                    f273(v0, v3)
                }
            } else {
                let v5: Bytes = {
                    let m1121 = f94(v2.clone(), f8(Str::lit("role")));
                    match m1121 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                };
                if f118(f52(f58(v2.clone(), f8(Str::lit("row"))), v0.clone()), f51({
                    let a1123 = {
                        let a1122 = v5.clone();
                        length_bytes(a1122)
                    };
                    let a1124 = 0u64;
                    nat_eq(a1123, a1124)
                })) {
                    List::cons(({
                        let a1125 = f8(Str::lit("r:"));
                        let a1126 = v5;
                        append_bytes(a1125, a1126)
                    }, f46(f58(v2, f8(Str::lit("row"))), 0u64, 1u64)), f273(v0, v3))
                } else {
                    f273(v0, v3)
                }
            }
        }
    }
}

pub fn f274(v0: List<List<Bytes>>, v1: u64, v2: u64) -> R<List<Bytes>> {
    let m1127 = v2;
    if m1127 == 0 {
        Ok(List::<Bytes>::nil())
    } else {
        let v3 = m1127 - 1;
        Ok(f126(f389(v0.clone(), f46(f390(), v1, 1u64), f391()?)?, f274(v0, {
            let a1128 = v1;
            let a1129 = 1u64;
            nat_add(a1128, a1129)?
        }, v3)?))
    }
}

pub fn f275(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1130 = v0;
    match m1130.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f392({
                let (h1131, _) = v2;
                h1131
            }, {
                let (_, h1132) = v2;
                h1132
            }, v1.clone())?, f275(v3, v1)?))
        }
    }
}

pub fn f276(v0: Bytes, p1150: u64, p1151: u64, p1152: Bytes) -> R<Bytes> {
    let m1149 = (p1150, (p1151, p1152));
    {
        let mut m1149 = m1149;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1149;
                let m1133 = v2;
                if m1133 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1133 - 1;
                    let v6: u64 = {
                        let m1136 = {
                            let a1134 = v0.clone();
                            let a1135 = v1;
                            index_bytes(a1134, a1135)
                        };
                        match m1136 {
                            None => {
                                0u64
                            }
                            Some(v5) => {
                                f73(v5)?
                            }
                        }
                    };
                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                        let a1137 = v1;
                        let a1138 = 1u64;
                        nat_add(a1137, a1138)?
                    }, (v4, {
                        let a1147 = {
                            let a1142 = v3;
                            let a1143 = f46(f8(Str::lit("0123456789abcdef")), {
                                let a1139 = v6;
                                let a1140 = 16u64;
                                let a1141 = 0u64;
                                nat_quot(a1139, a1140, a1141)
                            }, 1u64);
                            append_bytes(a1142, a1143)
                        };
                        let a1148 = f46(f8(Str::lit("0123456789abcdef")), {
                            let a1144 = v6;
                            let a1145 = 16u64;
                            let a1146 = 0u64;
                            nat_rem(a1144, a1145, a1146)
                        }, 1u64);
                        append_bytes(a1147, a1148)
                    })))
                }
            };
            match step {
                Ok(next) => { m1149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(v0: List<Adt2>) -> List<Bytes> {
    let m1153 = v0;
    match m1153.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f277(v2))
        }
    }
}

pub fn f278(v0: List<Adt2>) -> List<Bytes> {
    let m1154 = v0;
    match m1154.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f393(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f278(v2))
        }
    }
}

pub fn f279(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1155 = v0;
    match m1155.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f394(v1)?, f279(v2)?))
        }
    }
}

pub fn f280(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1156 = v0;
    match m1156.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f395(v1)?, f280(v2)?))
        }
    }
}

pub fn f281() -> List<Bytes> {
    List::cons(f8(Str::lit("prism FACET SEED")), List::<Bytes>::nil())
}

pub fn f282(p1162: List<Bytes>) -> R<bool> {
    let m1161 = p1162;
    {
        let mut m1161 = m1161;
        loop {
            let step = {
                let v0 = m1161;
                let m1157 = v0;
                match m1157.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1159 = {
                            let a1158 = f202(v1)?;
                            length_bytes(a1158)
                        };
                        let a1160 = 0u64;
                        if nat_eq(a1159, a1160) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: List<Bytes>) -> R<bool> {
    if f292(f305(v0.clone())?) {
        Ok(f51({
            let a1164 = {
                let a1163 = f294(f305(v0)?);
                length_list(a1163)
            };
            let a1165 = 0u64;
            nat_eq(a1164, a1165)
        }))
    } else {
        Ok(false)
    }
}

pub fn f284(v0: List<Bytes>) -> R<bool> {
    if f292(f396(v0.clone(), f8(Str::lit("requirement")))?) {
        if f292(f396(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1167 = {
                    let a1166 = f228(f294(f396(v0.clone(), f8(Str::lit("requirement")))?), f294(f396(v0, f8(Str::lit("admission")))?));
                    length_list(a1166)
                };
                let a1168 = 0u64;
                nat_eq(a1167, a1168)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f285() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f286(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f287() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f288(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1169 = v0;
    match m1169.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f202(v1)?;
            let a1171 = {
                let a1170 = v3.clone();
                length_bytes(a1170)
            };
            let a1172 = 0u64;
            if nat_eq(a1171, a1172) {
                f288(v2)
            } else {
                let m1173 = f43(v3.clone())?;
                match m1173 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f397(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1174 = f288(v2)?;
                            match m1174 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f398(v5), {
                                        let a1175 = v3;
                                        let a1176 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1175, a1176)
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

pub fn f289() -> Bytes {
    let a1177 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1177)
}

pub fn f290(p1182: List<Bytes>) -> R<Option<Bytes>> {
    let m1181 = p1182;
    {
        let mut m1181 = m1181;
        loop {
            let step = {
                let v0 = m1181;
                let m1178 = v0;
                match m1178.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1179 = f43(f202(v1)?)?;
                        match m1179 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1180 = f94(v3.clone(), f8(Str::lit("source")));
                                match m1180 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f399(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
                                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f58(v3, f8(Str::lit("id"))))))
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
                Ok(next) => { m1181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: Bytes) -> Bytes {
    let a1187 = {
        let a1183 = Str::lit("binding ");
        utf8_encode(a1183)
    };
    let a1188 = {
        let a1185 = v0;
        let a1186 = {
            let a1184 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1184)
        };
        append_bytes(a1185, a1186)
    };
    append_bytes(a1187, a1188)
}

pub fn f292(v0: Option<List<Adt2>>) -> bool {
    let m1189 = v0;
    match m1189 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f293(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1190 = v0;
    match m1190.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f202(v1)?;
            let a1192 = {
                let a1191 = v3.clone();
                length_bytes(a1191)
            };
            let a1193 = 0u64;
            if nat_eq(a1192, a1193) {
                f293(v2)
            } else {
                let m1194 = f43(v3)?;
                match m1194 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if f118(f118(f225(v4.clone(), f8(Str::lit("authority"))), f225(v4.clone(), f8(Str::lit("row")))), f118(f118(f225(v4.clone(), f8(Str::lit("construct"))), f225(v4.clone(), f8(Str::lit("artifact")))), f400(f58(v4.clone(), f8(Str::lit("reader"))), f58(v4.clone(), f8(Str::lit("construct")))))) {
                            let m1195 = f293(v2)?;
                            match m1195 {
                                None => {
                                    Ok(None::<List<Adt2>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons(v4, v5)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt2>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f294(v0: Option<List<Adt2>>) -> List<Adt2> {
    let m1196 = v0;
    match m1196 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f295(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1197 = v0;
    match m1197.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f202(v1)?;
            let a1199 = {
                let a1198 = v3.clone();
                length_bytes(a1198)
            };
            let a1200 = 0u64;
            if nat_eq(a1199, a1200) {
                f295(v2)
            } else {
                let m1201 = f43(v3.clone())?;
                match m1201 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f401(v4.clone()) {
                            let m1202 = f295(v2)?;
                            match m1202 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f250(), List::cons(f8(Str::lit("/")), List::cons(f402(f58(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1203 = v3;
                                        let a1204 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1203, a1204)
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

pub fn f296() -> Bytes {
    f403()
}

pub fn f297(v0: Adt1, v1: Adt2, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f302(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f404(v1, v4)?)?), List::cons(f303(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f298(v0: Bytes) -> R<Bytes> {
    Ok(f405(f406(v0)?))
}

pub fn f299(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1205 = v0;
    match m1205.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f300() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f301(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f368(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f302(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f407(f151(f96(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f408(f409(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f303(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f101(Adt2::C7(std::rc::Rc::new(f410(f146(f126(v1, f219(f97(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f304(p1208: List<Adt2>) -> bool {
    let m1207 = p1208;
    {
        let mut m1207 = m1207;
        loop {
            let step = {
                let v0 = m1207;
                let m1206 = v0;
                match m1206.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f143(f411(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
                        } else {
                            false
                        } {
                            Err::<List<Adt2>, bool>(true)
                        } else {
                            Ok::<List<Adt2>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f305(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1209 = v0;
    match m1209.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f202(v1)?;
            let a1211 = {
                let a1210 = v3.clone();
                length_bytes(a1210)
            };
            let a1212 = 0u64;
            if nat_eq(a1211, a1212) {
                f305(v2)
            } else {
                let m1213 = f43(v3)?;
                match m1213 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if if f225(v4.clone(), f8(Str::lit("agent"))) {
                            if f225(v4.clone(), f8(Str::lit("at"))) {
                                if f225(v4.clone(), f8(Str::lit("quote"))) {
                                    f412(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1214 = f305(v2)?;
                            match m1214 {
                                None => {
                                    Ok(None::<List<Adt2>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons(v4, v5)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt2>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f306(v0: List<Adt2>) -> List<Bytes> {
    let m1215 = v0;
    match m1215.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1218 = f58(v1.clone(), f8(Str::lit("at")));
                let a1219 = {
                    let a1216 = Bytes::lit(&[0x09]);
                    let a1217 = f58(v1, f8(Str::lit("quote")));
                    append_bytes(a1216, a1217)
                };
                append_bytes(a1218, a1219)
            }, f306(v2))
        }
    }
}

pub fn f307(v0: Bytes, v1: List<Adt2>) -> R<(Bytes, Bytes)> {
    Ok((v0, f101(Adt2::C7(std::rc::Rc::new(f102(v1))))?))
}

pub fn f308() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f309(p1222: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m1221 = p1222;
    {
        let mut m1221 = m1221;
        loop {
            let step = {
                let v0 = m1221;
                let m1220 = v0;
                match m1220.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f118(f32(f58(v3.clone(), f8(Str::lit("observedProperty"))), v1.clone()), f32(f58(v3, f8(Str::lit("hasSimpleResult"))), v2.clone())) {
                            Err::<List<Adt2>, bool>(true)
                        } else {
                            Ok::<List<Adt2>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt2::C6(v0)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))))))))
}

pub fn f311(v0: u64) -> bool {
    let a1223 = v0;
    let a1224 = 127u64;
    nat_le(a1223, a1224)
}

pub fn f312(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1225, _) = v1.clone();
        h1225
    };
    let v3: u64 = {
        let (h1226, _) = {
            let (_, h1227) = v1.clone();
            h1227
        };
        h1226
    };
    let v4: List<Bytes> = {
        let (_, h1228) = {
            let (_, h1229) = v1;
            h1229
        };
        h1228
    };
    let m1230 = f212(v0.clone(), v2);
    match m1230 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1231 = v5;
            let a1232 = 10u8;
            if equal(a1231, a1232) {
                Ok(Some(({
                    let a1233 = v2;
                    let a1234 = 1u64;
                    nat_add(a1233, a1234)?
                }, ({
                    let a1235 = v2;
                    let a1236 = 1u64;
                    nat_add(a1235, a1236)?
                }, List::cons(f413(f46(v0, v3, {
                    let a1237 = v2;
                    let a1238 = v3;
                    nat_sub(a1237, a1238)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1239 = v2;
                    let a1240 = 1u64;
                    nat_add(a1239, a1240)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f313(v0: Adt2) -> List<Adt2> {
    let m1241 = v0;
    match m1241 {
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
        Adt2::C7(r1242) => {
            let v3 = (*r1242).clone();
            f313(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r1243, r1244) => {
            let v1 = (*r1243).clone();
            let v2 = (*r1244).clone();
            List::cons(v1, f313(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f314(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1245 = v0;
    match m1245 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r1246) => {
            let v12 = (*r1246).clone();
            Adt2::C1(v11, std::rc::Rc::new(v12))
        }
        Adt2::C2(v6) => {
            Adt2::C2(v6)
        }
        Adt2::C3(v7, v8) => {
            Adt2::C3(v7, v8)
        }
        Adt2::C4(v13, v14, v15) => {
            Adt2::C4(v13, v14, v15)
        }
        Adt2::C5(v16, v17) => {
            Adt2::C5(v16, v17)
        }
        Adt2::C6(v9) => {
            Adt2::C6(v9)
        }
        Adt2::C7(r1247) => {
            let v10 = (*r1247).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r1248) => {
            let v5 = (*r1248).clone();
            Adt2::C8(std::rc::Rc::new(f314(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1249, r1250) => {
            let v18 = (*r1249).clone();
            let v19 = (*r1250).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r1251, r1252) => {
            let v3 = (*r1251).clone();
            let v4 = (*r1252).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt2::C11(v2, std::rc::Rc::new(f414(v3, v1.clone())), std::rc::Rc::new(f314(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f314(v4, v1)))
            }
        }
    }
}

pub fn f315(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1253 = 0u64;
        let a1254 = v1;
        nat_lt(a1253, a1254)
    } else {
        false
    } {
        let a1255 = Bytes::lit(&[0x2d]);
        let a1256 = f415(v1)?;
        Ok(append_bytes(a1255, a1256))
    } else {
        f415(v1)
    }
}

pub fn f316(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f415(v1)?;
    let v4: u64 = {
        let a1257 = v3.clone();
        length_bytes(a1257)
    };
    let v5: Bytes = {
        let a1258 = v2;
        let a1259 = v4;
        if nat_lt(a1258, a1259) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1260 = v4;
                let a1261 = v2;
                nat_sub(a1260, a1261)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1262 = v4;
                let a1263 = v2;
                nat_sub(a1262, a1263)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f337({
                let a1264 = v2;
                let a1265 = v4;
                nat_sub(a1264, a1265)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1266 = f8(Str::lit("-"));
        let a1267 = v5;
        Ok(append_bytes(a1266, a1267))
    } else {
        Ok(v5)
    }
}

pub fn f317(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f416(v0)?;
    let v3: bool = {
        let a1268 = f417(v0)?;
        let a1269 = v1;
        nat_le(a1268, a1269)
    };
    let v4: u64 = {
        let a1273 = {
            let a1270 = v1;
            let a1271 = v2;
            let a1272 = 0u64;
            nat_quot(a1270, a1271, a1272)
        };
        let a1274 = f418(f419(v0))?;
        let a1275 = 0u64;
        nat_rem(a1273, a1274, a1275)
    };
    let v5: u64 = {
        let a1276 = v1;
        let a1277 = v2;
        let a1278 = 0u64;
        nat_rem(a1276, a1277, a1278)
    };
    let a1279 = v4;
    let a1280 = f420(v0)?;
    if nat_eq(a1279, a1280) {
        let a1281 = v5;
        let a1282 = 0u64;
        if nat_eq(a1281, a1282) {
            Ok(f421(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f421(v3, {
                let a1286 = {
                    let a1283 = v2;
                    let a1284 = 2u64;
                    let a1285 = 0u64;
                    nat_quot(a1283, a1284, a1285)
                };
                let a1287 = v5;
                if nat_le(a1286, a1287) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f118({
            let a1288 = v4;
            let a1289 = 0u64;
            nat_eq(a1288, a1289)
        }, {
            let a1290 = v5;
            let a1291 = 0u64;
            nat_eq(a1290, a1291)
        }) {
            Ok(f421(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1292 = v4;
                let a1293 = 0u64;
                if nat_eq(a1292, a1293) {
                    v5
                } else {
                    let a1294 = v2;
                    let a1295 = v5;
                    nat_add(a1294, a1295)?
                }
            };
            let v7: u64 = {
                let a1296 = v4;
                let a1297 = 0u64;
                if nat_eq(a1296, a1297) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1304 = {
                    let a1300 = {
                        let a1298 = v7;
                        let a1299 = 2000u64;
                        nat_add(a1298, a1299)?
                    };
                    let a1301 = f422(v0);
                    nat_sub(a1300, a1301)
                };
                let a1305 = {
                    let a1302 = f423(v0);
                    let a1303 = 1u64;
                    nat_sub(a1302, a1303)
                };
                nat_sub(a1304, a1305)
            };
            Ok(f421(v3, {
                let a1306 = 2000u64;
                let a1307 = v8;
                if nat_le(a1306, a1307) {
                    f424(v6, true, {
                        let a1308 = v8;
                        let a1309 = 2000u64;
                        nat_sub(a1308, a1309)
                    })?
                } else {
                    f424(v6, false, {
                        let a1310 = 2000u64;
                        let a1311 = v8;
                        nat_sub(a1310, a1311)
                    })?
                }
            }))
        }
    }
}

pub fn f318(v0: Bytes) -> R<Bytes> {
    let a1316 = f204(v0.clone(), 0u64, {
        let a1313 = {
            let a1312 = v0.clone();
            length_bytes(a1312)
        };
        let a1314 = 1u64;
        nat_add(a1313, a1314)?
    })?;
    let a1317 = {
        let a1315 = v0.clone();
        length_bytes(a1315)
    };
    if nat_eq(a1316, a1317) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1323 = {
            let a1321 = Bytes::lit(&[0x22]);
            let a1322 = f425(v0.clone(), 0u64, {
                let a1319 = {
                    let a1318 = v0;
                    length_bytes(a1318)
                };
                let a1320 = 1u64;
                nat_add(a1319, a1320)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1321, a1322)
        };
        let a1324 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1323, a1324))
    }
}

pub fn f319(v0: List<Bytes>) -> Bytes {
    let a1327 = {
        let a1325 = Bytes::lit(&[0x5b]);
        let a1326 = f123(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1325, a1326)
    };
    let a1328 = Bytes::lit(&[0x5d]);
    append_bytes(a1327, a1328)
}

pub fn f320(v0: Adt2) -> R<List<Bytes>> {
    let m1329 = v0;
    match m1329 {
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
        Adt2::C10(r1330, r1331) => {
            let v14 = (*r1330).clone();
            let v15 = (*r1331).clone();
            Ok(List::cons(f194(v14)?, f320(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f321(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f426(Fn5::F427, List::<(Bytes, Bytes)>::nil(), f428(v0)?);
    let v2: List<Bytes> = f429(Fn6::F430, List::<Bytes>::nil(), v1)?;
    let a1334 = {
        let a1332 = Bytes::lit(&[0x7b]);
        let a1333 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1332, a1333)
    };
    let a1335 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1334, a1335))
}

pub fn f322(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1336 = v0;
    match m1336 {
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
        Adt2::C11(v16, r1337, r1338) => {
            let v17 = (*r1337).clone();
            let v18 = (*r1338).clone();
            Ok(List::cons((v16, f194(v17)?), f322(v18)?))
        }
    }
}

pub fn f323() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f324(v0: Adt13) -> Bytes {
    let m1339 = v0;
    match m1339 {
        Adt13::C0 => {
            f8(Str::lit("model/authorities.toml"))
        }
        Adt13::C1 => {
            f8(Str::lit("model/ids.toml"))
        }
        Adt13::C2 => {
            f8(Str::lit("site/generated/release-ledger.json"))
        }
        Adt13::C3 => {
            f8(Str::lit("standards.lock"))
        }
    }
}

pub fn f325() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f326(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f431(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f431(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f431(f8(Str::lit("name")), f432(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f431(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f431(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f431(f8(Str::lit("statement")), f433(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f431(f8(Str::lit("media_type")), f434(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f431(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f431(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f431(f8(Str::lit("url")), f197(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f327(v0: Bytes) -> R<u64> {
    let (h1340, _) = f330(Fn3::F435(v0.clone()), {
        let a1341 = v0.clone();
        length_bytes(a1341)
    }, {
        let a1342 = v0;
        length_bytes(a1342)
    })?;
    Ok(h1340)
}

pub fn f328(v0: Bytes, v1: u64) -> R<u64> {
    let (h1343, _) = f330(Fn3::F436(v1, v0), v1, 0u64)?;
    Ok(h1343)
}

pub fn f329(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f116(v0.clone(), {
        let a1344 = v1;
        let a1345 = 1u64;
        nat_add(a1344, a1345)?
    })?;
    let v4: Bytes = {
        let a1346 = v3;
        let a1347 = 117u64;
        if nat_eq(a1346, a1347) {
            f437(v0, v1)?
        } else {
            f438(v3)
        }
    };
    let a1348 = v4.clone();
    let a1349 = Bytes::lit(&[]);
    if equal(a1348, a1349) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1350 = v3;
            let a1351 = 117u64;
            if nat_eq(a1350, a1351) {
                let a1352 = v1;
                let a1353 = 6u64;
                nat_add(a1352, a1353)?
            } else {
                let a1354 = v1;
                let a1355 = 2u64;
                nat_add(a1354, a1355)?
            }
        }, ({
            let a1356 = v2;
            let a1357 = v4;
            append_bytes(a1356, a1357)
        }, 0u64))))
    }
}

pub fn f330(v0: Fn3, p1362: u64, p1363: u64) -> R<(u64, bool)> {
    let m1361 = (p1362, p1363);
    {
        let mut m1361 = m1361;
        loop {
            let step = {
                let (v1, v2) = m1361;
                let m1358 = v1;
                if m1358 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1358 - 1;
                    let m1360 = {
                        let c1359: Fn3 = v0.clone();
                        c1359.apply(v2)?
                    };
                    match m1360 {
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
                Ok(next) => { m1361 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f439(v0, v1)
}

pub fn f332(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f440(v0, v1)
}

pub fn f333(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1364 = v0.clone();
        length_bytes(a1364)
    };
    let a1365 = 0u64;
    let a1366 = v1;
    if nat_lt(a1365, a1366) {
        let a1367 = f441(v0.clone())?;
        let a1368 = v1;
        if nat_eq(a1367, a1368) {
            let a1369 = 1u64;
            let a1370 = v1;
            if nat_lt(a1369, a1370) {
                let m1373 = {
                    let a1371 = v0;
                    let a1372 = 0u64;
                    index_bytes(a1371, a1372)
                };
                match m1373 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1374 = v2;
                        let a1375 = 48u8;
                        Ok(!equal(a1374, a1375))
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

pub fn f334(v0: Fn4, p1380: u64, p1381: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1379 = (p1380, p1381);
    {
        let mut m1379 = m1379;
        loop {
            let step = {
                let (v1, v2) = m1379;
                let m1376 = v1;
                if m1376 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1376 - 1;
                    let m1378 = {
                        let c1377: Fn4 = v0.clone();
                        c1377.apply(v2)?
                    };
                    match m1378 {
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
                Ok(next) => { m1379 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f442(v0, v1)
}

pub fn f336(v0: Bytes) -> R<u64> {
    let (h1382, _) = f330(Fn3::F443(v0.clone()), {
        let a1384 = {
            let a1383 = v0;
            length_bytes(a1383)
        };
        let a1385 = 1u64;
        nat_add(a1384, a1385)?
    }, 0u64)?;
    Ok(h1382)
}

pub fn f337(v0: u64) -> Bytes {
    let m1386 = v0;
    if m1386 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1386 - 1;
        let a1387 = f8(Str::lit("0"));
        let a1388 = f337(v1);
        append_bytes(a1387, a1388)
    }
}

pub fn f338(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1389 = v0;
    match m1389.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1390 = f94(v1, f8(Str::lit("std")));
            match m1390 {
                None => {
                    f338(v2)
                }
                Some(v3) => {
                    if f225(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f338(v2))
                    } else {
                        f338(v2)
                    }
                }
            }
        }
    }
}

pub fn f339(v0: Adt14, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1391 = v1;
    match m1391.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt14::C0(h1392, _, _) => {
                    h1392
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1393, _) => {
                        h1393
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f339(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt14::C0(h1394, _, _) => {
                        h1394
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1395, _) => {
                            h1395
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f339(v0, v3))
                } else {
                    f339(v0, v3)
                }
            }
        }
    }
}

pub fn f340(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt17>> {
    let m1396 = v0;
    match m1396.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v4, v5)) => {
            if if f444(v4.clone(), v2.clone()) {
                true
            } else {
                f444(v4.clone(), v3.clone())
            } {
                f340(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt17::C0({
                    let (h1397, _) = v4.clone();
                    h1397
                }, {
                    let a1400 = {
                        let a1399 = f146(f445({
                            let (h1398, _) = v4.clone();
                            h1398
                        }, v1.clone()))?;
                        length_list(a1399)
                    };
                    let a1401 = 1u64;
                    if nat_eq(a1400, a1401) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1402) = v4.clone();
                    h1402
                }), f340(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f341(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1403 = v1;
    match m1403.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1404, _) = v2.clone();
                h1404
            }, v0.clone()) {
                List::cons({
                    let (_, h1405) = v2;
                    h1405
                }, f341(v0, v3))
            } else {
                if f52({
                    let (_, h1406) = v2;
                    h1406
                }, v0.clone()) {
                    List::cons({
                        let (h1407, _) = v2;
                        h1407
                    }, f341(v0, v3))
                } else {
                    f341(v0, v3)
                }
            }
        }
    }
}

pub fn f342(v0: Adt14, v1: Adt2, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt14::C0(h1408, _, _) => {
            h1408
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1409, _) => {
                h1409
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt14::C0(_, _, h1410) => {
                    h1410
                }
            }) {
                true
            } else {
                if f225(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1411, _) => {
                            h1411
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f343(v0: Adt2) -> bool {
    if f225(v0.clone(), f8(Str::lit("domain"))) {
        if f225(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f446()) {
                if f447(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f447(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f447(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f447(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f344(v0: Adt2) -> bool {
    if f225(v0.clone(), f8(Str::lit("subject"))) {
        if f225(v0.clone(), f8(Str::lit("object"))) {
            if f225(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f448())
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

pub fn f345(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1412 = f43(v1)?;
    match m1412 {
        None => {
            Ok(f233(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f219(f192(f94(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1413 = f93(f94(v4.clone(), f8(Str::lit("construct"))));
                match m1413 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1414 = v5;
            match m1414.uncons() {
                None => {
                    Ok(f233(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1415 = v9;
                    match m1415.uncons() {
                        None => {
                            Ok(f233(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f51(f32(v8, f8(Str::lit("std")))) {
                                Ok(f233(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f51(f52(v10.clone(), v2)) {
                                    Ok(f233(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f118(f51(f32(v7.clone(), f8(Str::lit("none")))), f51(f52(v7.clone(), v3))) {
                                        Ok(f233(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1416 = f93(f94(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1416 {
                                            None => {
                                                Ok(f233(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1417 = f94(v4.clone(), f8(Str::lit("reader")));
                                                match m1417 {
                                                    None => {
                                                        let m1418 = f94(v4, f8(Str::lit("value")));
                                                        match m1418 {
                                                            None => {
                                                                Ok(f233(v0, f8(Str::lit("a row names neither the reader of its artifact nor a value"))))
                                                            }
                                                            Some(_) => {
                                                                Ok(List::<Bytes>::nil())
                                                            }
                                                        }
                                                    }
                                                    Some(_) => {
                                                        if f225(v4, f8(Str::lit("artifact"))) {
                                                            Ok(List::<Bytes>::nil())
                                                        } else {
                                                            Ok(f233(v0, f8(Str::lit("a row names its reader but not its artifact"))))
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

pub fn f346(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1419 = v1;
    match m1419.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f449(v3.clone())
            } else {
                f51(f449(v3.clone()))
            } {
                List::cons(f246(v0.clone(), v3), f346(v0, v4, v2))
            } else {
                f346(v0, v4, v2)
            }
        }
    }
}

pub fn f347(p1422: List<Bytes>) -> R<Option<Bytes>> {
    let m1421 = p1422;
    {
        let mut m1421 = m1421;
        loop {
            let step = {
                let v0 = m1421;
                let m1420 = v0;
                match m1420.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f450(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f202(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f348(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1423 = v1;
    match m1423.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f96(f64(v0.clone(), v2))?, f348(v0, v3)?))
        }
    }
}

pub fn f349(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1424 = v0.clone();
        length_bytes(a1424)
    };
    let v3: u64 = {
        let a1425 = v1.clone();
        length_bytes(a1425)
    };
    let v4: u64 = {
        let (h1426, _) = f330(Fn3::F451(v1.clone(), v3, v0.clone(), v2), {
            let a1427 = v2;
            let a1428 = 1u64;
            nat_add(a1427, a1428)?
        }, 0u64)?;
        h1426
    };
    if f452(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f350() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f351(v0: Adt15) -> R<Adt18> {
    Ok(Adt18::C0(f453(match v0.clone() {
        Adt15::C0(h1429, _, _, _, _) => {
            h1429
        }
    }, 0u64)?, f453(match v0.clone() {
        Adt15::C0(_, h1430, _, _, _) => {
            h1430
        }
    }, 0u64)?, f453(match v0.clone() {
        Adt15::C0(_, _, h1431, _, _) => {
            h1431
        }
    }, 0u64)?, f453(match v0 {
        Adt15::C0(_, _, _, h1432, _) => {
            h1432
        }
    }, 0u64)?))
}

pub fn f352(v0: Adt15) -> List<Bytes> {
    f126(match v0.clone() {
        Adt15::C0(h1433, _, _, _, _) => {
            h1433
        }
    }, f126(match v0.clone() {
        Adt15::C0(_, _, h1434, _, _) => {
            h1434
        }
    }, f126(match v0.clone() {
        Adt15::C0(_, _, _, h1435, _) => {
            h1435
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1436) => {
            h1436
        }
    })))
}

pub fn f353(v0: Adt18, p1442: List<u64>, p1443: List<Adt20>) -> Option<u64> {
    let m1441 = (p1442, p1443);
    {
        let mut m1441 = m1441;
        loop {
            let step = {
                let (v1, v2) = m1441;
                let m1437 = v2;
                match m1437.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f454(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1438) => {
                                h1438
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1439, _) => {
                                    h1439
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1440, _) => {
                                    h1440
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: Adt15, v1: List<Adt2>, v2: u64) -> R<List<Adt20>> {
    let m1444 = v1;
    match m1444.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f455(v0.clone(), v3)?), f354(v0, v4, {
                let a1445 = v2;
                let a1446 = 1u64;
                nat_add(a1445, a1446)?
            })?))
        }
    }
}

pub fn f355(p1450: List<Bytes>, p1451: u64) -> Bytes {
    let m1449 = (p1450, p1451);
    {
        let mut m1449 = m1449;
        loop {
            let step = {
                let (v0, v1) = m1449;
                let m1447 = v0;
                match m1447.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1448 = v1;
                        if m1448 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1448 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f456(v0.clone(), v1.clone()) {
        if f456(v1, v0) {
            f457(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f357(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1452 = v0;
    match m1452.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f458(v2, v1.clone())?, f357(v3, v1)?))
        }
    }
}

pub fn f358(v0: List<Adt2>) -> List<u64> {
    let m1453 = v0;
    match m1453.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f459(v1), f358(v2))
        }
    }
}

pub fn f359(p1456: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1455 = p1456;
    {
        let mut m1455 = m1455;
        loop {
            let step = {
                let v0 = m1455;
                let m1454 = v0;
                match m1454.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f460(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1455 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360(p1459: List<Bytes>, p1460: List<Bytes>) -> Option<Bytes> {
    let m1458 = (p1459, p1460);
    {
        let mut m1458 = m1458;
        loop {
            let step = {
                let (v0, v1) = m1458;
                let m1457 = v0;
                match m1457.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f52(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: Adt15, p1468: List<Adt2>) -> Option<Bytes> {
    let m1467 = p1468;
    {
        let mut m1467 = m1467;
        loop {
            let step = {
                let v1 = m1467;
                let m1461 = v1;
                match m1461.uncons() {
                    None => {
                        Err::<List<Adt2>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1462 = f94(v2.clone(), f8(Str::lit("evidence")));
                        match m1462 {
                            None => {
                                Ok::<List<Adt2>, Option<Bytes>>(v3)
                            }
                            Some(v4) => {
                                let m1463 = f94(v4, f8(Str::lit("constraint")));
                                match m1463 {
                                    None => {
                                        Ok::<List<Adt2>, Option<Bytes>>(v3)
                                    }
                                    Some(v5) => {
                                        if f52(f58(v5.clone(), f8(Str::lit("authority"))), match v0.clone() {
                                            Adt15::C0(h1464, _, _, _, _) => {
                                                h1464
                                            }
                                        }) {
                                            let m1465 = f93(f94(v5, f8(Str::lit("row"))));
                                            match m1465 {
                                                None => {
                                                    Ok::<List<Adt2>, Option<Bytes>>(v3)
                                                }
                                                Some(v6) => {
                                                    if f52(v6, match v0.clone() {
                                                        Adt15::C0(_, h1466, _, _, _) => {
                                                            h1466
                                                        }
                                                    }) {
                                                        Ok::<List<Adt2>, Option<Bytes>>(v3)
                                                    } else {
                                                        Err::<List<Adt2>, Option<Bytes>>(Some(f58(v2, f8(Str::lit("id")))))
                                                    }
                                                }
                                            }
                                        } else {
                                            Err::<List<Adt2>, Option<Bytes>>(Some(f58(v2, f8(Str::lit("id")))))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f363(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1469 = v0.clone();
    match m1469.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1470 = v2;
            let a1471 = v1.clone();
            if equal(a1470, a1471) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f364(v0: List<(Bytes, ())>) -> List<Bytes> {
    f243(f461(Fn7::F462, List::<Bytes>::nil(), v0))
}

pub fn f365(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f463(f464(Fn8::F465, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1476 = {
        let (h1472, _) = f466(Fn9::F467, {
            let a1474 = {
                let a1473 = v0;
                length_list(a1473)
            };
            let a1475 = 1u64;
            nat_add(a1474, a1475)?
        }, v1)?;
        h1472
    };
    match m1476.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f366(v0: List<Bytes>) -> List<(Bytes, ())> {
    f468(f469(Fn10::F470, List::<(Bytes, ())>::nil(), v0))
}

pub fn f367(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1477 = f44(v1, 34u8)?;
    match m1477.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1478 = v3;
            match m1478.uncons() {
                None => {
                    Ok(f471(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f368(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f472(v0.clone(), 0u64, {
        let a1479 = v0;
        length_bytes(a1479)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f369(v0: Adt2, v1: Bytes) -> Bytes {
    if f32(f473(f94(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f370(v0: Bytes, v1: Bytes, p1492: u64, p1493: u64) -> R<bool> {
    let m1491 = (p1492, p1493);
    {
        let mut m1491 = m1491;
        loop {
            let step = {
                let (v2, v3) = m1491;
                let m1480 = v3;
                if m1480 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1480 - 1;
                    let m1483 = {
                        let a1481 = v1.clone();
                        let a1482 = v2;
                        index_bytes(a1481, a1482)
                    };
                    match m1483 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1486 = {
                                let a1484 = v0.clone();
                                let a1485 = v2;
                                index_bytes(a1484, a1485)
                            };
                            match m1486 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1487 = v5;
                                    let a1488 = v6;
                                    if equal(a1487, a1488) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1489 = v2;
                                            let a1490 = 1u64;
                                            nat_add(a1489, a1490)?
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
                Ok(next) => { m1491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1494 = f92(v0, v1)?;
    match m1494 {
        None => {
            Ok(Adt2::C9)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f372(v0: Bytes) -> R<Bytes> {
    let a1495 = f8(Str::lit("Catalog."));
    let a1496 = f402(v0)?;
    Ok(append_bytes(a1495, a1496))
}

pub fn f373(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f402(f58(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f374(v0: List<Bytes>) -> List<Bytes> {
    let m1497 = v0;
    match m1497.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f246(f285(), v1), f374(v2))
            } else {
                f374(v2)
            }
        }
    }
}

pub fn f375(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1498 = v1;
    match m1498.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f474(f475(f96(f64(v0.clone(), v2))?)?), f375(v0, v3)?))
        }
    }
}

pub fn f376(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1499 = v1;
    match m1499.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f476(v0.clone(), v3, v2.clone())?;
            let (h1500, _) = v5.clone();
            if h1500 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f376(v0, v4, v2)?;
                let (h1501, _) = v6.clone();
                if h1501 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1504 = {
                            let (_, h1502) = v5;
                            h1502
                        };
                        let a1505 = {
                            let (_, h1503) = v6;
                            h1503
                        };
                        append_bytes(a1504, a1505)
                    }))
                }
            }
        }
    }
}

pub fn f377(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1506 = v1;
    match m1506.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f225(v2.clone(), f8(Str::lit("subject"))) {
                if f226(v0.clone(), v2.clone()) {
                    f377(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f377(v0, v3))
                }
            } else {
                if f387(v0.clone(), v2.clone()) {
                    f377(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f388(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f377(v0, v3))
                }
            }
        }
    }
}

pub fn f378(v0: Adt1) -> R<Bytes> {
    let a1508 = {
        let a1507 = f129(f130(f271(f268(f269(f64(v0.clone(), f217()))?)), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f132(f97(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1507)
    };
    let a1509 = 0u64;
    if nat_eq(a1508, a1509) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f379(v0: List<Adt2>) -> R<List<Bytes>> {
    let m1510 = v0;
    match m1510.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f123(f219(f192(f94(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f477(f96(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f379(v2)?))
        }
    }
}

pub fn f380() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f381(v0: Bytes, p1568: u64, p1569: u64, p1570: Adt19) -> R<Adt19> {
    let m1567 = (p1568, (p1569, p1570));
    {
        let mut m1567 = m1567;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1567;
                let m1511 = v2;
                if m1511 == 0 {
                    Err::<(u64, (u64, Adt19)), R<Adt19>>(Ok(v3))
                } else {
                    let v4 = m1511 - 1;
                    let v5: u64 = f478(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt19::C0(_, h1512, _, _, _) => {
                            h1512
                        }
                    } {
                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                            let a1513 = v1;
                            let a1514 = 1u64;
                            nat_add(a1513, a1514)?
                        }, (v4, Adt19::C0(true, false, match v3.clone() {
                            Adt19::C0(_, _, h1515, _, _) => {
                                h1515
                            }
                        }, match v3.clone() {
                            Adt19::C0(_, _, _, h1516, _) => {
                                h1516
                            }
                        }, match v3 {
                            Adt19::C0(_, _, _, _, h1517) => {
                                h1517
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt19::C0(h1518, _, _, _, _) => {
                                h1518
                            }
                        } {
                            let a1519 = v5;
                            let a1520 = 34u64;
                            if nat_eq(a1519, a1520) {
                                let a1523 = f478(v0.clone(), {
                                    let a1521 = v1;
                                    let a1522 = 1u64;
                                    nat_add(a1521, a1522)?
                                })?;
                                let a1524 = 34u64;
                                if nat_eq(a1523, a1524) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1525 = v1;
                                        let a1526 = 1u64;
                                        nat_add(a1525, a1526)?
                                    }, (v4, Adt19::C0(true, true, f479(match v3.clone() {
                                        Adt19::C0(_, _, h1527, _, _) => {
                                            h1527
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt19::C0(_, _, _, h1528, _) => {
                                            h1528
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1529) => {
                                            h1529
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1530 = v1;
                                        let a1531 = 1u64;
                                        nat_add(a1530, a1531)?
                                    }, (v4, Adt19::C0(false, false, match v3.clone() {
                                        Adt19::C0(_, _, h1532, _, _) => {
                                            h1532
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1533, _) => {
                                            h1533
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1534) => {
                                            h1534
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1535 = v1;
                                    let a1536 = 1u64;
                                    nat_add(a1535, a1536)?
                                }, (v4, Adt19::C0(true, false, f479(match v3.clone() {
                                    Adt19::C0(_, _, h1537, _, _) => {
                                        h1537
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt19::C0(_, _, _, h1538, _) => {
                                        h1538
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1539) => {
                                        h1539
                                    }
                                }))))
                            }
                        } else {
                            let a1540 = v5;
                            let a1541 = 34u64;
                            if nat_eq(a1540, a1541) {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1542 = v1;
                                    let a1543 = 1u64;
                                    nat_add(a1542, a1543)?
                                }, (v4, Adt19::C0(true, false, match v3.clone() {
                                    Adt19::C0(_, _, h1544, _, _) => {
                                        h1544
                                    }
                                }, match v3.clone() {
                                    Adt19::C0(_, _, _, h1545, _) => {
                                        h1545
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1546) => {
                                        h1546
                                    }
                                }))))
                            } else {
                                let a1547 = v5;
                                let a1548 = 44u64;
                                if nat_eq(a1547, a1548) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1549 = v1;
                                        let a1550 = 1u64;
                                        nat_add(a1549, a1550)?
                                    }, (v4, Adt19::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt19::C0(_, _, h1551, _, _) => {
                                            h1551
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1552, _) => {
                                            h1552
                                        }
                                    }), match v3 {
                                        Adt19::C0(_, _, _, _, h1553) => {
                                            h1553
                                        }
                                    }))))
                                } else {
                                    let a1554 = v5;
                                    let a1555 = 13u64;
                                    if nat_eq(a1554, a1555) {
                                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                            let a1556 = v1;
                                            let a1557 = 1u64;
                                            nat_add(a1556, a1557)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1558 = v5;
                                        let a1559 = 10u64;
                                        if nat_eq(a1558, a1559) {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1560 = v1;
                                                let a1561 = 1u64;
                                                nat_add(a1560, a1561)?
                                            }, (v4, f382(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1562 = v1;
                                                let a1563 = 1u64;
                                                nat_add(a1562, a1563)?
                                            }, (v4, Adt19::C0(false, false, f479(match v3.clone() {
                                                Adt19::C0(_, _, h1564, _, _) => {
                                                    h1564
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt19::C0(_, _, _, h1565, _) => {
                                                    h1565
                                                }
                                            }, match v3 {
                                                Adt19::C0(_, _, _, _, h1566) => {
                                                    h1566
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
                Ok(next) => { m1567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(v0: Adt19) -> Adt19 {
    Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f480(List::cons(match v0.clone() {
        Adt19::C0(_, _, h1571, _, _) => {
            h1571
        }
    }, match v0.clone() {
        Adt19::C0(_, _, _, h1572, _) => {
            h1572
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt19::C0(_, _, _, _, h1573) => {
            h1573
        }
    }))
}

pub fn f383(p1576: List<List<Bytes>>, p1577: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1575 = (p1576, p1577);
    {
        let mut m1575 = m1575;
        loop {
            let step = {
                let (v0, v1) = m1575;
                let m1574 = v0;
                match m1574.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: Bytes, v1: List<Adt17>) -> List<Bytes> {
    let m1578 = v1;
    match m1578.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt17::C0(h1579, _, _) => {
                    h1579
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt17::C0(_, h1580, _) => {
                        h1580
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt17::C0(_, _, h1581) => {
                        h1581
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f384(v0, v3))
            } else {
                f384(v0, v3)
            }
        }
    }
}

pub fn f385(v0: Bytes, v1: List<Adt2>) -> List<Bytes> {
    let m1582 = v1;
    match m1582.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f385(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f385(v0, v3))
                } else {
                    f385(v0, v3)
                }
            }
        }
    }
}

pub fn f386(p1587: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1586 = p1587;
    {
        let mut m1586 = m1586;
        loop {
            let step = {
                let v0 = m1586;
                let m1583 = v0;
                match m1583.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1584 = v2;
                        match m1584.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1585 = v5;
                                    match m1585.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f481(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1586 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(v0: Adt14, v1: Adt2) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1588, _, _) => {
            h1588
        }
    }) {
        f52(f388(v1), match v0 {
            Adt14::C0(_, h1589, _) => {
                h1589
            }
        })
    } else {
        false
    }
}

pub fn f388(v0: Adt2) -> Bytes {
    let m1590 = f94(v0, f8(Str::lit("evidence")));
    match m1590 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f389(v0: List<List<Bytes>>, v1: Bytes, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1591 = v2;
    match m1591.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            if f482(v0.clone(), v1.clone(), v3.clone())? {
                Ok(List::cons(f7(List::cons(v1.clone(), List::cons(f8(Str::lit("/")), List::cons(v3, List::cons(f8(Str::lit("\u{9}\u{a}")), List::<Bytes>::nil()))))), f389(v0, v1, v4)?))
            } else {
                f389(v0, v1, v4)
            }
        }
    }
}

pub fn f390() -> Bytes {
    f8(Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))
}

pub fn f391() -> R<List<Bytes>> {
    f44(f8(Str::lit("CON PRE PRO QA LCH OPS MNT EOL")), 32u8)
}

pub fn f392(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1592 = v2;
    match m1592.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            if f118(f32({
                let (h1593, _) = v3.clone();
                h1593
            }, v0.clone()), {
                let a1595 = f483(v1.clone())?;
                let a1596 = f483({
                    let (_, h1594) = v3.clone();
                    h1594
                })?;
                nat_lt(a1595, a1596)
            }) {
                Ok(List::cons({
                    let a1598 = v1.clone();
                    let a1599 = {
                        let (_, h1597) = v3;
                        h1597
                    };
                    append_bytes(a1598, a1599)
                }, f392(v0, v1, v4)?))
            } else {
                f392(v0, v1, v4)
            }
        }
    }
}

pub fn f393(v0: Adt2) -> Bytes {
    if f225(v0.clone(), f8(Str::lit("req"))) {
        let a1600 = f8(Str::lit("req "));
        let a1601 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1600, a1601)
    } else {
        let m1602 = f94(v0.clone(), f8(Str::lit("std")));
        match m1602 {
            None => {
                let m1605 = f94(v0, f8(Str::lit("der")));
                match m1605 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f123(f219(f192(f94(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f225(v1.clone(), f8(Str::lit("row"))) {
                    let a1603 = f8(Str::lit(" row "));
                    let a1604 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1603, a1604)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f394(v0: Bytes) -> R<Bytes> {
    let m1606 = f44(v0.clone(), 32u8)?;
    match m1606.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            Ok(f7(List::cons(f8(Str::lit("{\"name\":")), List::cons(f318(v1)?, List::cons(f8(Str::lit(",\"description\":")), List::cons(f318({
                let a1607 = f8(Str::lit("verdant_studio "));
                let a1608 = v0;
                append_bytes(a1607, a1608)
            })?, List::cons(f8(Str::lit(",\"inputSchema\":{\"type\":\"object\",\"properties\":{")), List::cons(f123(f484(v2.clone())?, f8(Str::lit(","))), List::cons(f8(Str::lit("},\"required\":[")), List::cons(f123(f485(v2)?, f8(Str::lit(","))), List::cons(f8(Str::lit("],\"additionalProperties\":false}}")), List::<Bytes>::nil())))))))))))
        }
    }
}

pub fn f395(v0: Bytes) -> R<Bytes> {
    let m1609 = f44(v0.clone(), 32u8)?;
    match m1609.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            Ok(f7(List::cons(f8(Str::lit("{\"name\":")), List::cons(f318(v1)?, List::cons(f8(Str::lit(",\"description\":")), List::cons(f318({
                let a1610 = f8(Str::lit("the instance's app route "));
                let a1611 = v0;
                append_bytes(a1610, a1611)
            })?, List::cons(f8(Str::lit(",\"inputSchema\":{\"type\":\"object\",\"properties\":{")), List::cons(f123(f484(v2.clone())?, f8(Str::lit(","))), List::cons(f8(Str::lit("},\"required\":[")), List::cons(f123(f485(v2)?, f8(Str::lit(","))), List::cons(f8(Str::lit("],\"additionalProperties\":false}}")), List::<Bytes>::nil())))))))))))
        }
    }
}

pub fn f396(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt2>>> {
    let m1612 = v0;
    match m1612.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f202(v2)?;
            let a1614 = {
                let a1613 = v4.clone();
                length_bytes(a1613)
            };
            let a1615 = 0u64;
            if nat_eq(a1614, a1615) {
                f396(v3, v1)
            } else {
                let m1616 = f43(v4)?;
                match m1616 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f486(v5.clone());
                        let a1618 = {
                            let a1617 = v6.clone();
                            length_bytes(a1617)
                        };
                        let a1619 = 0u64;
                        if nat_eq(a1618, a1619) {
                            Ok(None::<List<Adt2>>)
                        } else {
                            let m1620 = f396(v3, v1.clone())?;
                            match m1620 {
                                None => {
                                    Ok(None::<List<Adt2>>)
                                }
                                Some(v7) => {
                                    if f32(v6, v1) {
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

pub fn f397(v0: Bytes, v1: Bytes) -> bool {
    let a1621 = v0;
    let a1622 = f403();
    if equal(a1621, a1622) {
        let a1626 = {
            let a1624 = {
                let a1623 = v1;
                length_bytes(a1623)
            };
            let a1625 = 0u64;
            nat_eq(a1624, a1625)
        };
        bool_not(a1626)
    } else {
        false
    }
}

pub fn f398(v0: Bytes) -> Bytes {
    f7(List::cons(f145(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f399(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1628 = v0;
    let a1629 = {
        let a1627 = Str::lit("redistributable");
        utf8_encode(a1627)
    };
    if equal(a1628, a1629) {
        f487(v1)
    } else {
        Ok(false)
    }
}

pub fn f400(v0: Bytes, v1: Bytes) -> bool {
    f207(f86(v0.clone(), f8(Str::lit("read-"))), f118(f32(v0, f8(Str::lit("check-yaml"))), f32(v1, f8(Str::lit("none")))))
}

pub fn f401(v0: Adt2) -> bool {
    if f488(v0.clone(), f8(Str::lit("name"))) {
        if f489(v0.clone(), f8(Str::lit("schema"))) {
            let m1630 = f490(f219(f192(f94(v0, f8(Str::lit("citation"))))));
            match m1630 {
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

pub fn f402(v0: Bytes) -> R<Bytes> {
    f491(v0.clone(), 0u64, {
        let a1631 = v0;
        length_bytes(a1631)
    }, true, Bytes::lit(&[]))
}

pub fn f403() -> Bytes {
    let a1632 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1632)
}

pub fn f404(v0: Adt2, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt2> = f94(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f301(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f301(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f301(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f301(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f301(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f301(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f301(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f301(Str::lit("url"), f473(v2.clone(), f8(Str::lit("url"))))?, List::cons(f301(Str::lit("media_type"), f473(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f301(Str::lit("license"), f473(v2.clone(), f8(Str::lit("license"))))?, List::cons(f301(Str::lit("redistribution"), f473(v2, f8(Str::lit("redistribution"))))?, List::cons(f301(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1633 = v1;
        match m1633 {
            None => {
                f7(List::cons(f301(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f301(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f301(Str::lit("artifact"), {
                    let (h1634, _) = v3;
                    h1634
                })?, List::cons(f301(Str::lit("checksum"), {
                    let (_, h1635) = v3;
                    h1635
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f492(f219(f192(f94(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f405(v0: Adt23) -> Bytes {
    let a1656 = f493(match v0.clone() {
        Adt23::C0(h1636, _, _, _, _, _, _, _) => {
            h1636
        }
    });
    let a1657 = {
        let a1654 = f493(match v0.clone() {
            Adt23::C0(_, h1637, _, _, _, _, _, _) => {
                h1637
            }
        });
        let a1655 = {
            let a1652 = f493(match v0.clone() {
                Adt23::C0(_, _, h1638, _, _, _, _, _) => {
                    h1638
                }
            });
            let a1653 = {
                let a1650 = f493(match v0.clone() {
                    Adt23::C0(_, _, _, h1639, _, _, _, _) => {
                        h1639
                    }
                });
                let a1651 = {
                    let a1648 = f493(match v0.clone() {
                        Adt23::C0(_, _, _, _, h1640, _, _, _) => {
                            h1640
                        }
                    });
                    let a1649 = {
                        let a1646 = f493(match v0.clone() {
                            Adt23::C0(_, _, _, _, _, h1641, _, _) => {
                                h1641
                            }
                        });
                        let a1647 = {
                            let a1644 = f493(match v0.clone() {
                                Adt23::C0(_, _, _, _, _, _, h1642, _) => {
                                    h1642
                                }
                            });
                            let a1645 = f493(match v0 {
                                Adt23::C0(_, _, _, _, _, _, _, h1643) => {
                                    h1643
                                }
                            });
                            append_bytes(a1644, a1645)
                        };
                        append_bytes(a1646, a1647)
                    };
                    append_bytes(a1648, a1649)
                };
                append_bytes(a1650, a1651)
            };
            append_bytes(a1652, a1653)
        };
        append_bytes(a1654, a1655)
    };
    append_bytes(a1656, a1657)
}

pub fn f406(v0: Bytes) -> R<Adt23> {
    let v1: u64 = {
        let a1658 = v0.clone();
        length_bytes(a1658)
    };
    let v2: u64 = {
        let a1664 = {
            let a1661 = {
                let a1659 = v1;
                let a1660 = 72u64;
                nat_add(a1659, a1660)?
            };
            let a1662 = 64u64;
            let a1663 = 0u64;
            nat_quot(a1661, a1662, a1663)
        };
        let a1665 = 64u64;
        nat_mul(a1664, a1665)?
    };
    let (_, h1666) = f494(Fn11::F495(v0, v1, v2), {
        let a1667 = v2;
        let a1668 = 64u64;
        let a1669 = 0u64;
        nat_quot(a1667, a1668, a1669)
    }, (0u64, f496()))?;
    Ok(h1666)
}

pub fn f407(p1672: List<List<Bytes>>, v1: Bytes, p1673: List<Bytes>) -> R<List<Bytes>> {
    let m1671 = (p1672, p1673);
    {
        let mut m1671 = m1671;
        loop {
            let step = {
                let (v0, v2) = m1671;
                let m1670 = v0;
                match m1670.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f497(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f409(f498(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f327(v0)?))
}

pub fn f409(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1674 = v0;
    match m1674.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1675 = v3.clone();
            match m1675.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1678 = v2;
                    let a1679 = {
                        let a1676 = v1.clone();
                        let a1677 = f409(v3, v1);
                        append_bytes(a1676, a1677)
                    };
                    append_bytes(a1678, a1679)
                }
            }
        }
    }
}

pub fn f410(v0: List<Bytes>) -> Adt2 {
    let m1680 = v0;
    match m1680.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f410(v2)))
        }
    }
}

pub fn f411(v0: Adt2, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1681 = f94(v0, v1);
    match m1681 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v3) => {
            f192(f94(v3, v2))
        }
    }
}

pub fn f412(v0: Adt2, v1: Bytes) -> bool {
    let m1682 = f94(v0, v1);
    match m1682 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f413(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1684 = {
                let a1683 = v0;
                length_bytes(a1683)
            };
            let a1685 = 1u64;
            nat_sub(a1684, a1685)
        })
    } else {
        v0
    }
}

pub fn f414(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1686 = v0;
    match m1686 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v8, r1687) => {
            let v9 = (*r1687).clone();
            Adt2::C1(v8, std::rc::Rc::new(v9))
        }
        Adt2::C2(v3) => {
            Adt2::C2(v3)
        }
        Adt2::C3(v4, v5) => {
            Adt2::C3(v4, v5)
        }
        Adt2::C4(v10, v11, v12) => {
            Adt2::C4(v10, v11, v12)
        }
        Adt2::C5(v13, v14) => {
            Adt2::C5(v13, v14)
        }
        Adt2::C6(v2) => {
            Adt2::C6(f499(f60(), v2, v1))
        }
        Adt2::C7(r1688) => {
            let v6 = (*r1688).clone();
            Adt2::C7(std::rc::Rc::new(v6))
        }
        Adt2::C8(r1689) => {
            let v7 = (*r1689).clone();
            Adt2::C8(std::rc::Rc::new(v7))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1690, r1691) => {
            let v15 = (*r1690).clone();
            let v16 = (*r1691).clone();
            Adt2::C10(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt2::C11(v17, r1692, r1693) => {
            let v18 = (*r1692).clone();
            let v19 = (*r1693).clone();
            Adt2::C11(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
    }
}

pub fn f415(v0: u64) -> R<Bytes> {
    f500(f501(), 10u64, v0)
}

pub fn f416(v0: bool) -> R<u64> {
    f418({
        let a1694 = f423(v0);
        let a1695 = 1u64;
        nat_sub(a1694, a1695)
    })
}

pub fn f417(v0: bool) -> R<u64> {
    f418(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f418(v0: u64) -> R<u64> {
    let m1696 = v0;
    if m1696 == 0 {
        Ok(1u64)
    } else {
        let v1 = m1696 - 1;
        let a1697 = 2u64;
        let a1698 = f418(v1)?;
        nat_mul(a1697, a1698)
    }
}

pub fn f419(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f420(v0: bool) -> R<u64> {
    let a1699 = f418(f419(v0))?;
    let a1700 = 1u64;
    Ok(nat_sub(a1699, a1700))
}

pub fn f421(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a1701 = Bytes::lit(&[0x2d]);
        let a1702 = v1;
        append_bytes(a1701, a1702)
    } else {
        v1
    }
}

pub fn f422(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f423(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f424(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f502(f503(v2, f504(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f502(f503(v2, f504(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a1703 = v3.clone();
            length_bytes(a1703)
        };
        let v5: Bytes = {
            let a1704 = v2;
            let a1705 = v4;
            if nat_lt(a1704, a1705) {
                f46(v3.clone(), 0u64, {
                    let a1706 = v4;
                    let a1707 = v2;
                    nat_sub(a1706, a1707)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f505({
            let a1708 = v2;
            let a1709 = v4;
            if nat_lt(a1708, a1709) {
                f46(v3, {
                    let a1710 = v4;
                    let a1711 = v2;
                    nat_sub(a1710, a1711)
                }, v2)
            } else {
                let a1714 = f506({
                    let a1712 = v2;
                    let a1713 = v4;
                    nat_sub(a1712, a1713)
                });
                let a1715 = v3;
                append_bytes(a1714, a1715)
            }
        }, {
            let a1716 = v2;
            let a1717 = 1u64;
            nat_add(a1716, a1717)?
        })?;
        let a1719 = {
            let a1718 = v6.clone();
            length_bytes(a1718)
        };
        let a1720 = 0u64;
        if nat_eq(a1719, a1720) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f425(v0: Bytes, p1739: u64, p1740: u64, p1741: Bytes) -> R<Bytes> {
    let m1738 = (p1739, (p1740, p1741));
    {
        let mut m1738 = m1738;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1738;
                let m1721 = v2;
                if m1721 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1721 - 1;
                    let v5: u64 = f204(v0.clone(), v1, {
                        let a1725 = {
                            let a1723 = {
                                let a1722 = v0.clone();
                                length_bytes(a1722)
                            };
                            let a1724 = v1;
                            nat_sub(a1723, a1724)
                        };
                        let a1726 = 1u64;
                        nat_add(a1725, a1726)?
                    })?;
                    let v6: Bytes = {
                        let a1729 = v3;
                        let a1730 = f46(v0.clone(), v1, {
                            let a1727 = v5;
                            let a1728 = v1;
                            nat_sub(a1727, a1728)
                        });
                        append_bytes(a1729, a1730)
                    };
                    let m1733 = {
                        let a1731 = v0.clone();
                        let a1732 = v5;
                        index_bytes(a1731, a1732)
                    };
                    match m1733 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a1734 = v5;
                                let a1735 = 1u64;
                                nat_add(a1734, a1735)?
                            }, (v4, {
                                let a1736 = v6;
                                let a1737 = f507(v0.clone(), v5, v7)?;
                                append_bytes(a1736, a1737)
                            })))
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

pub fn f426(v0: Fn5, p1745: List<(Bytes, Bytes)>, p1746: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1744 = (p1745, p1746);
    {
        let mut m1744 = m1744;
        loop {
            let step = {
                let (v1, v2) = m1744;
                let m1742 = v2;
                match m1742.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1743: Fn5 = v0.clone();
                            c1743.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f427(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f508(v0, v1)
}

pub fn f428(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f509(v0)
}

pub fn f429(v0: Fn6, p1750: List<Bytes>, p1751: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1749 = (p1750, p1751);
    {
        let mut m1749 = m1749;
        loop {
            let step = {
                let (v1, v2) = m1749;
                let m1747 = v2;
                match m1747.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1748: Fn6 = v0.clone();
                            c1748.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1749 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f430(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f510(v1)?, v0))
}

pub fn f431(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f511(v0)?, List::cons(f8(Str::lit(":")), List::cons(f511(v1)?, List::<Bytes>::nil())))))
}

pub fn f432(v0: Adt13) -> Bytes {
    let m1752 = v0;
    match m1752 {
        Adt13::C0 => {
            f8(Str::lit("Verdant Engine authority register"))
        }
        Adt13::C1 => {
            f8(Str::lit("Verdant Engine conformance ID register"))
        }
        Adt13::C2 => {
            f8(Str::lit("Verdant Engine release ledger"))
        }
        Adt13::C3 => {
            f8(Str::lit("Verdant Engine standards lock"))
        }
    }
}

pub fn f433(v0: Adt13) -> Bytes {
    let m1753 = v0;
    match m1753 {
        Adt13::C0 => {
            f8(Str::lit("The engine's imported authorities, each catalog pinned by sha256 and mapped to the IDs that realize it (realized_by)."))
        }
        Adt13::C1 => {
            f8(Str::lit("The engine's conformance IDs, each with level, suite, refusal and statement. Every engine capability Verdant Studio uses is cited by one of these IDs."))
        }
        Adt13::C2 => {
            f8(Str::lit("The engine's verdant/release-ledger/1: its current SemVer version, release epoch and change history."))
        }
        Adt13::C3 => {
            f8(Str::lit("The engine's prismpm/standards-lock/1: the standards it binds and the executable oracles, corpora and failure mappings it runs against them."))
        }
    }
}

pub fn f434(v0: Adt13) -> Bytes {
    let m1754 = v0;
    match m1754 {
        Adt13::C0 => {
            f8(Str::lit("application/toml"))
        }
        Adt13::C1 => {
            f8(Str::lit("application/toml"))
        }
        Adt13::C2 => {
            f8(Str::lit("application/json"))
        }
        Adt13::C3 => {
            f8(Str::lit("application/json"))
        }
    }
}

pub fn f435(v0: Bytes, v1: u64) -> Option<u64> {
    let a1755 = 0u64;
    let a1756 = v1;
    if nat_lt(a1755, a1756) {
        if f512(v0, {
            let a1757 = v1;
            let a1758 = 1u64;
            nat_sub(a1757, a1758)
        }) {
            Some({
                let a1759 = v1;
                let a1760 = 1u64;
                nat_sub(a1759, a1760)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f436(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1761 = v2;
    let a1762 = v0;
    if nat_lt(a1761, a1762) {
        if f512(v1, v2) {
            Ok(Some({
                let a1763 = v2;
                let a1764 = 1u64;
                nat_add(a1763, a1764)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f437(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1767 = f46(v0.clone(), {
        let a1765 = v1;
        let a1766 = 2u64;
        nat_add(a1765, a1766)?
    }, 2u64);
    let a1768 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1767, a1768) {
        let m1771 = f513(f46(v0, {
            let a1769 = v1;
            let a1770 = 4u64;
            nat_add(a1769, a1770)?
        }, 2u64))?;
        match m1771 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f116(v2.clone(), 0u64)?;
                let a1772 = v3;
                let a1773 = 32u64;
                if nat_lt(a1772, a1773) {
                    if f514(v3) {
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

pub fn f438(v0: u64) -> Bytes {
    let a1774 = v0;
    let a1775 = 34u64;
    if nat_eq(a1774, a1775) {
        Bytes::lit(&[0x22])
    } else {
        let a1776 = v0;
        let a1777 = 92u64;
        if nat_eq(a1776, a1777) {
            Bytes::lit(&[0x5c])
        } else {
            let a1778 = v0;
            let a1779 = 98u64;
            if nat_eq(a1778, a1779) {
                Bytes::lit(&[0x08])
            } else {
                let a1780 = v0;
                let a1781 = 102u64;
                if nat_eq(a1780, a1781) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1782 = v0;
                    let a1783 = 110u64;
                    if nat_eq(a1782, a1783) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1784 = v0;
                        let a1785 = 114u64;
                        if nat_eq(a1784, a1785) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1786 = v0;
                            let a1787 = 116u64;
                            if nat_eq(a1786, a1787) {
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

pub fn f439(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f116(v0, v1)?;
    let a1788 = 47u64;
    let a1789 = v2;
    if if nat_lt(a1788, a1789) {
        let a1790 = v2;
        let a1791 = 58u64;
        nat_lt(a1790, a1791)
    } else {
        false
    } {
        Ok(Some({
            let a1792 = v1;
            let a1793 = 1u64;
            nat_add(a1792, a1793)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f440(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1794 = f116(v0, v1)?;
    let a1795 = 48u64;
    if nat_eq(a1794, a1795) {
        Ok(Some({
            let a1796 = v1;
            let a1797 = 1u64;
            nat_add(a1796, a1797)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f441(v0: Bytes) -> R<u64> {
    let (h1798, _) = f330(Fn3::F515(v0.clone()), {
        let a1799 = v0;
        length_bytes(a1799)
    }, 0u64)?;
    Ok(h1798)
}

pub fn f442(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1803 = {
        let a1801 = v0;
        let a1802 = {
            let (h1800, _) = v1;
            h1800
        };
        index_bytes(a1801, a1802)
    };
    match m1803 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1805 = {
                    let (h1804, _) = v1;
                    h1804
                };
                let a1806 = 1u64;
                nat_add(a1805, a1806)?
            }, {
                let a1810 = {
                    let a1808 = {
                        let (_, h1807) = v1;
                        h1807
                    };
                    let a1809 = 10u64;
                    nat_mul(a1808, a1809)?
                };
                let a1811 = f516(v2)?;
                nat_add(a1810, a1811)?
            })))
        }
    }
}

pub fn f443(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f517(v0, v1)
}

pub fn f444(v0: (Bytes, Bytes), p1814: List<(Bytes, Bytes)>) -> bool {
    let m1813 = p1814;
    {
        let mut m1813 = m1813;
        loop {
            let step = {
                let v1 = m1813;
                let m1812 = v1;
                match m1812.uncons() {
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
                Ok(next) => { m1813 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1815 = v1;
    match m1815.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1816, _) = v2;
                h1816
            }, v0.clone()) {
                List::cons({
                    let (_, h1817) = v2;
                    h1817
                }, f445(v0, v3))
            } else {
                f445(v0, v3)
            }
        }
    }
}

pub fn f446() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f447(v0: Adt2, v1: Bytes, v2: Bytes) -> bool {
    let m1818 = f94(v0, v1);
    match m1818 {
        None => {
            false
        }
        Some(v3) => {
            f225(v3, v2)
        }
    }
}

pub fn f448() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f449(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f450(v0: Bytes) -> R<bool> {
    let v1: Bytes = f202(v0)?;
    let v2: u64 = if f86(v1.clone(), f8(Str::lit("if ! "))) {
        5u64
    } else {
        if f86(v1.clone(), f8(Str::lit("elif ! "))) {
            7u64
        } else {
            0u64
        }
    };
    let v3: Bytes = f46(v1.clone(), v2, {
        let a1820 = {
            let a1819 = v1;
            length_bytes(a1819)
        };
        let a1821 = v2;
        nat_sub(a1820, a1821)
    });
    let v5: Bytes = if f86(v3.clone(), f8(Str::lit("("))) {
        let m1822 = f349(v3.clone(), f8(Str::lit(")")))?;
        match m1822 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1823 = v4;
                    let a1824 = 1u64;
                    nat_add(a1823, a1824)?
                }, {
                    let a1828 = {
                        let a1825 = v3;
                        length_bytes(a1825)
                    };
                    let a1829 = {
                        let a1826 = v4;
                        let a1827 = 1u64;
                        nat_add(a1826, a1827)?
                    };
                    nat_sub(a1828, a1829)
                })
            }
        }
    } else {
        v3
    };
    let a1830 = 0u64;
    let a1831 = v2;
    if nat_lt(a1830, a1831) {
        f519(v5)
    } else {
        Ok(false)
    }
}

pub fn f451(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f452(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1834 = {
            let a1832 = v4;
            let a1833 = v1;
            nat_add(a1832, a1833)?
        };
        let a1835 = v3;
        if nat_lt(a1834, a1835) {
            Ok(Some({
                let a1836 = v4;
                let a1837 = 1u64;
                nat_add(a1836, a1837)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f452(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1842 = {
        let a1839 = v0;
        let a1840 = v2;
        let a1841 = {
            let a1838 = v1.clone();
            length_bytes(a1838)
        };
        slice_bytes(a1839, a1840, a1841)
    };
    match m1842 {
        None => {
            false
        }
        Some(v3) => {
            let a1843 = v3;
            let a1844 = v1;
            equal(a1843, a1844)
        }
    }
}

pub fn f453(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1845 = v0;
    match m1845.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f453(v3, {
                let a1846 = v1;
                let a1847 = 1u64;
                nat_add(a1846, a1847)?
            })?))
        }
    }
}

pub fn f454(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1848 = v2;
    match m1848 {
        Adt21::C0(v3, v4) => {
            if f460(v3, match v0.clone() {
                Adt18::C0(h1849, _, _, _) => {
                    h1849
                }
            }) {
                f520(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f460(v5, match v0 {
                Adt18::C0(_, _, h1850, _) => {
                    h1850
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f456(v7, v1) {
                f460(v8, match v0 {
                    Adt18::C0(_, _, _, h1851) => {
                        h1851
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f455(v0: Adt15, v1: Adt2) -> R<Adt21> {
    let m1852 = f94(v1.clone(), f8(Str::lit("std")));
    match m1852 {
        None => {
            let m1857 = f93(f94(v1.clone(), f8(Str::lit("req"))));
            match m1857 {
                None => {
                    let m1859 = f94(v1, f8(Str::lit("der")));
                    match m1859 {
                        None => {
                            Ok(f521(v0))
                        }
                        Some(v5) => {
                            let m1860 = f522(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1860 {
                                None => {
                                    Ok(f521(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f357(f219(f192(f94(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1861) => {
                                            h1861
                                        }
                                    })?, f458(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1862, _) => {
                                            h1862
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f458(v4, match v0 {
                        Adt15::C0(_, _, h1858, _, _) => {
                            h1858
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1853 = f93(f94(v2.clone(), f8(Str::lit("row"))));
            match m1853 {
                None => {
                    Ok(Adt21::C0(f458(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1854, _, _, _, _) => {
                            h1854
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f458(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1855, _, _, _, _) => {
                            h1855
                        }
                    })?, Some(f458(v3, match v0 {
                        Adt15::C0(_, h1856, _, _, _) => {
                            h1856
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f456(p1865: List<u64>, v1: List<u64>) -> bool {
    let m1864 = p1865;
    {
        let mut m1864 = m1864;
        loop {
            let step = {
                let v0 = m1864;
                let m1863 = v0;
                match m1863.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f460(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1864 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(p1870: List<u64>) -> bool {
    let m1869 = p1870;
    {
        let mut m1869 = m1869;
        loop {
            let step = {
                let v0 = m1869;
                let m1866 = v0;
                match m1866.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1867 = v1;
                        let a1868 = 7u64;
                        if nat_le(a1867, a1868) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1871 = f523(v0, v1.clone(), 0u64)?;
    match m1871 {
        None => {
            let a1872 = v1;
            Ok(length_list(a1872))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f459(v0: Adt2) -> u64 {
    let m1873 = f524(f94(v0, f8(Str::lit("stratum"))));
    match m1873 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f460(v0: u64, p1878: List<u64>) -> bool {
    let m1877 = p1878;
    {
        let mut m1877 = m1877;
        loop {
            let step = {
                let v1 = m1877;
                let m1874 = v1;
                match m1874.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1875 = v2;
                        let a1876 = v0;
                        if nat_eq(a1875, a1876) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1877 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: Fn7, p1882: List<Bytes>, p1883: List<(Bytes, ())>) -> List<Bytes> {
    let m1881 = (p1882, p1883);
    {
        let mut m1881 = m1881;
        loop {
            let step = {
                let (v1, v2) = m1881;
                let m1879 = v2;
                match m1879.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1880: Fn7 = v0.clone();
                            c1880.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1884, _) = v1;
        h1884
    }, v0)
}

pub fn f463(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f525(Fn12::F526, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f464(v0: Fn8, p1888: List<List<(Bytes, ())>>, p1889: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1887 = (p1888, p1889);
    {
        let mut m1887 = m1887;
        loop {
            let step = {
                let (v1, v2) = m1887;
                let m1885 = v2;
                match m1885.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1886: Fn8 = v0.clone();
                            c1886.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f466(v0: Fn9, p1894: u64, p1895: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1893 = (p1894, p1895);
    {
        let mut m1893 = m1893;
        loop {
            let step = {
                let (v1, v2) = m1893;
                let m1890 = v1;
                if m1890 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1890 - 1;
                    let m1892 = {
                        let c1891: Fn9 = v0.clone();
                        c1891.apply(v2.clone())?
                    };
                    match m1892 {
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
                Ok(next) => { m1893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f527(v0)
}

pub fn f468(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f528(Fn13::F529, List::<(Bytes, ())>::nil(), v0)
}

pub fn f469(v0: Fn10, p1899: List<(Bytes, ())>, p1900: List<Bytes>) -> List<(Bytes, ())> {
    let m1898 = (p1899, p1900);
    {
        let mut m1898 = m1898;
        loop {
            let step = {
                let (v1, v2) = m1898;
                let m1896 = v2;
                match m1896.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1897: Fn10 = v0.clone();
                            c1897.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1898 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f470(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f471(p1905: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1904 = p1905;
    {
        let mut m1904 = m1904;
        loop {
            let step = {
                let v0 = m1904;
                let m1901 = v0;
                match m1901.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1902, _) = v2;
                            h1902
                        }) {
                            let (_, h1903) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1903)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1904 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Bytes, p1931: u64, p1932: u64, p1933: Bytes) -> R<Bytes> {
    let m1930 = (p1931, (p1932, p1933));
    {
        let mut m1930 = m1930;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1930;
                let m1906 = v2;
                if m1906 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1906 - 1;
                    let m1909 = {
                        let a1907 = v0.clone();
                        let a1908 = v1;
                        index_bytes(a1907, a1908)
                    };
                    match m1909 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f73(v5)?;
                            if f207({
                                let a1910 = v6;
                                let a1911 = 34u64;
                                nat_eq(a1910, a1911)
                            }, {
                                let a1912 = v6;
                                let a1913 = 92u64;
                                nat_eq(a1912, a1913)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1914 = v1;
                                    let a1915 = 1u64;
                                    nat_add(a1914, a1915)?
                                }, (v4, {
                                    let a1918 = v3;
                                    let a1919 = {
                                        let a1916 = f8(Str::lit("\\"));
                                        let a1917 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1916, a1917)
                                    };
                                    append_bytes(a1918, a1919)
                                })))
                            } else {
                                let a1920 = v6;
                                let a1921 = 10u64;
                                if nat_eq(a1920, a1921) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1922 = v1;
                                        let a1923 = 1u64;
                                        nat_add(a1922, a1923)?
                                    }, (v4, {
                                        let a1924 = v3;
                                        let a1925 = f8(Str::lit("\\n"));
                                        append_bytes(a1924, a1925)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1926 = v1;
                                        let a1927 = 1u64;
                                        nat_add(a1926, a1927)?
                                    }, (v4, {
                                        let a1928 = v3;
                                        let a1929 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1928, a1929)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1930 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: Option<Adt2>, v1: Bytes) -> Bytes {
    let m1934 = v0;
    match m1934 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f474(v0: List<Adt2>) -> List<Bytes> {
    let m1935 = v0;
    match m1935.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f530(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f531(f58(v1, f8(Str::lit("authority")))), f474(v2)))
        }
    }
}

pub fn f475(v0: List<Bytes>) -> R<List<Adt2>> {
    let m1936 = f305(v0.clone())?;
    match m1936 {
        None => {
            Ok(f294(f293(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt2>::nil())
        }
    }
}

pub fn f476(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
    let m1937 = f305(v3.clone())?;
    match m1937 {
        None => {
            let m1938 = f293(v3.clone())?;
            match m1938 {
                None => {
                    f534(v0, v1, v3)
                }
                Some(v5) => {
                    f533(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f532(v0, v1, v4)
        }
    }
}

pub fn f477(v0: List<Bytes>) -> List<Bytes> {
    let m1939 = v0;
    match m1939.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1941 = {
                    let a1940 = v1.clone();
                    length_bytes(a1940)
                };
                let a1942 = 0u64;
                if nat_eq(a1941, a1942) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f477(v2))
        }
    }
}

pub fn f478(v0: Bytes, v1: u64) -> R<u64> {
    let m1945 = {
        let a1943 = v0;
        let a1944 = v1;
        index_bytes(a1943, a1944)
    };
    match m1945 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f73(v2)
        }
    }
}

pub fn f479(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1949 = {
        let a1946 = v1;
        let a1947 = v2;
        let a1948 = 1u64;
        slice_bytes(a1946, a1947, a1948)
    };
    match m1949 {
        None => {
            v0
        }
        Some(v3) => {
            let a1950 = v0;
            let a1951 = v3;
            append_bytes(a1950, a1951)
        }
    }
}

pub fn f480(p1954: List<Bytes>, p1955: List<Bytes>) -> List<Bytes> {
    let m1953 = (p1954, p1955);
    {
        let mut m1953 = m1953;
        loop {
            let step = {
                let (v0, v1) = m1953;
                let m1952 = v0;
                match m1952.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: List<Bytes>) -> Bytes {
    let m1956 = v0;
    match m1956.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f482(p1959: List<List<Bytes>>, v1: Bytes, v2: Bytes) -> R<bool> {
    let m1958 = p1959;
    {
        let mut m1958 = m1958;
        loop {
            let step = {
                let v0 = m1958;
                let m1957 = v0;
                match m1957.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        if f118(f86(f535(v3.clone(), 0u64), v1.clone()), f52(v2.clone(), f44(f535(v3, 4u64), 32u8)?)) {
                            Err::<List<List<Bytes>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<Bytes>>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1958 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f483(v0: Bytes) -> R<u64> {
    let m1962 = {
        let a1960 = v0;
        let a1961 = 0u64;
        index_bytes(a1960, a1961)
    };
    match m1962 {
        None => {
            Ok(256u64)
        }
        Some(v1) => {
            f73(v1)
        }
    }
}

pub fn f484(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1963 = v0;
    match m1963.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f318(f536(v1.clone())?)?, List::cons(f8(Str::lit(":")), List::cons(f537(v1), List::<Bytes>::nil())))), f484(v2)?))
        }
    }
}

pub fn f485(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1964 = v0;
    match m1964.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f86(v1.clone(), f8(Str::lit("["))) {
                f485(v2)
            } else {
                Ok(List::cons(f318(f536(v1)?)?, f485(v2)?))
            }
        }
    }
}

pub fn f486(v0: Adt2) -> Bytes {
    if f225(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f225(v0.clone(), f8(Str::lit("id"))) {
            if f225(v0.clone(), f8(Str::lit("statement"))) {
                if f225(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f412(v0, f8(Str::lit("evidence")))
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

pub fn f487(v0: Bytes) -> R<bool> {
    let m1966 = {
        let a1965 = v0.clone();
        length_bytes(a1965)
    };
    if m1966 == 0 {
        Ok(true)
    } else {
        let v1 = m1966 - 1;
        let m1969 = {
            let a1967 = v0;
            let a1968 = v1;
            index_bytes(a1967, a1968)
        };
        match m1969 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f538({
                    let v3: u8 = v2;
                    let a2014 = {
                        let a1990 = {
                            let a1978 = {
                                let a1972 = {
                                    let a1970 = v3;
                                    let a1971 = 128u8;
                                    fixed_u8::bit_and(a1970, a1971)
                                };
                                let a1973 = 128u8;
                                if equal(a1972, a1973) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1979 = {
                                let a1976 = {
                                    let a1974 = v3;
                                    let a1975 = 64u8;
                                    fixed_u8::bit_and(a1974, a1975)
                                };
                                let a1977 = 64u8;
                                if equal(a1976, a1977) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1978, a1979)?
                        };
                        let a1991 = {
                            let a1988 = {
                                let a1982 = {
                                    let a1980 = v3;
                                    let a1981 = 32u8;
                                    fixed_u8::bit_and(a1980, a1981)
                                };
                                let a1983 = 32u8;
                                if equal(a1982, a1983) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1989 = {
                                let a1986 = {
                                    let a1984 = v3;
                                    let a1985 = 16u8;
                                    fixed_u8::bit_and(a1984, a1985)
                                };
                                let a1987 = 16u8;
                                if equal(a1986, a1987) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1988, a1989)?
                        };
                        nat_add(a1990, a1991)?
                    };
                    let a2015 = {
                        let a2012 = {
                            let a2000 = {
                                let a1994 = {
                                    let a1992 = v3;
                                    let a1993 = 8u8;
                                    fixed_u8::bit_and(a1992, a1993)
                                };
                                let a1995 = 8u8;
                                if equal(a1994, a1995) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a2001 = {
                                let a1998 = {
                                    let a1996 = v3;
                                    let a1997 = 4u8;
                                    fixed_u8::bit_and(a1996, a1997)
                                };
                                let a1999 = 4u8;
                                if equal(a1998, a1999) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2000, a2001)?
                        };
                        let a2013 = {
                            let a2010 = {
                                let a2004 = {
                                    let a2002 = v3;
                                    let a2003 = 2u8;
                                    fixed_u8::bit_and(a2002, a2003)
                                };
                                let a2005 = 2u8;
                                if equal(a2004, a2005) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2011 = {
                                let a2008 = {
                                    let a2006 = v3;
                                    let a2007 = 1u8;
                                    fixed_u8::bit_and(a2006, a2007)
                                };
                                let a2009 = 1u8;
                                if equal(a2008, a2009) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2010, a2011)?
                        };
                        nat_add(a2012, a2013)?
                    };
                    nat_add(a2014, a2015)?
                }))
            }
        }
    }
}

pub fn f488(v0: Adt2, v1: Bytes) -> bool {
    let a2017 = {
        let a2016 = f58(v0, v1);
        length_bytes(a2016)
    };
    let a2018 = 0u64;
    !nat_eq(a2017, a2018)
}

pub fn f489(v0: Adt2, v1: Bytes) -> bool {
    let m2019 = f94(v0, v1);
    match m2019 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f490(v0: List<Bytes>) -> Option<Adt2> {
    let m2020 = v0;
    match m2020.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m2021 = v2;
                match m2021.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m2022 = v2;
                    match m2022.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m2023 = v6;
                                match m2023.uncons() {
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
                        let m2024 = v2;
                        match m2024.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m2025 = v10;
                                match m2025.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f410(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
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

pub fn f491(v0: Bytes, p2046: u64, p2047: u64, p2048: bool, p2049: Bytes) -> R<Bytes> {
    let m2045 = (p2046, (p2047, (p2048, p2049)));
    {
        let mut m2045 = m2045;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2045;
                let m2026 = v2;
                if m2026 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m2026 - 1;
                    let m2029 = {
                        let a2027 = v0.clone();
                        let a2028 = v1;
                        index_bytes(a2027, a2028)
                    };
                    match m2029 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f73(v6)?;
                            if f539(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2030 = v1;
                                    let a2031 = 1u64;
                                    nat_add(a2030, a2031)?
                                }, (v5, (true, v4))))
                            } else {
                                if f118(f540(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2032 = v1;
                                        let a2033 = 1u64;
                                        nat_add(a2032, a2033)?
                                    }, (v5, (false, {
                                        let a2037 = v4;
                                        let a2038 = f46({
                                            let a2034 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2034)
                                        }, {
                                            let a2035 = v7;
                                            let a2036 = 97u64;
                                            nat_sub(a2035, a2036)
                                        }, 1u64);
                                        append_bytes(a2037, a2038)
                                    }))))
                                } else {
                                    if f207(f540(v7), f207(f541(v7), f542(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2039 = v1;
                                            let a2040 = 1u64;
                                            nat_add(a2039, a2040)?
                                        }, (v5, (false, {
                                            let a2041 = v4;
                                            let a2042 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a2041, a2042)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2043 = v1;
                                            let a2044 = 1u64;
                                            nat_add(a2043, a2044)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2045 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f409(f543(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f493(v0: u32) -> Bytes {
    let a2078 = f544({
        let a2050 = f545(v0, 28u32);
        let a2051 = 15u32;
        fixed_u32::bit_and(a2050, a2051)
    });
    let a2079 = {
        let a2076 = f544({
            let a2052 = f545(v0, 24u32);
            let a2053 = 15u32;
            fixed_u32::bit_and(a2052, a2053)
        });
        let a2077 = {
            let a2074 = f544({
                let a2054 = f545(v0, 20u32);
                let a2055 = 15u32;
                fixed_u32::bit_and(a2054, a2055)
            });
            let a2075 = {
                let a2072 = f544({
                    let a2056 = f545(v0, 16u32);
                    let a2057 = 15u32;
                    fixed_u32::bit_and(a2056, a2057)
                });
                let a2073 = {
                    let a2070 = f544({
                        let a2058 = f545(v0, 12u32);
                        let a2059 = 15u32;
                        fixed_u32::bit_and(a2058, a2059)
                    });
                    let a2071 = {
                        let a2068 = f544({
                            let a2060 = f545(v0, 8u32);
                            let a2061 = 15u32;
                            fixed_u32::bit_and(a2060, a2061)
                        });
                        let a2069 = {
                            let a2066 = f544({
                                let a2062 = f545(v0, 4u32);
                                let a2063 = 15u32;
                                fixed_u32::bit_and(a2062, a2063)
                            });
                            let a2067 = f544({
                                let a2064 = v0;
                                let a2065 = 15u32;
                                fixed_u32::bit_and(a2064, a2065)
                            });
                            append_bytes(a2066, a2067)
                        };
                        append_bytes(a2068, a2069)
                    };
                    append_bytes(a2070, a2071)
                };
                append_bytes(a2072, a2073)
            };
            append_bytes(a2074, a2075)
        };
        append_bytes(a2076, a2077)
    };
    append_bytes(a2078, a2079)
}

pub fn f494(v0: Fn11, p2083: u64, p2084: (u64, Adt23)) -> R<(u64, Adt23)> {
    let m2082 = (p2083, p2084);
    {
        let mut m2082 = m2082;
        loop {
            let step = {
                let (v1, v2) = m2082;
                let m2080 = v1;
                if m2080 == 0 {
                    Err::<(u64, (u64, Adt23)), R<(u64, Adt23)>>(Ok(v2))
                } else {
                    let v3 = m2080 - 1;
                    Ok::<(u64, (u64, Adt23)), R<(u64, Adt23)>>((v3, {
                        let c2081: Fn11 = v0.clone();
                        c2081.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2082 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt23)) -> R<(u64, Adt23)> {
    Ok(({
        let a2086 = {
            let (h2085, _) = v3.clone();
            h2085
        };
        let a2087 = 64u64;
        nat_add(a2086, a2087)?
    }, f546({
        let (_, h2088) = v3;
        h2088
    }, f547(v0, v1, v2, {
        let (h2089, _) = v3;
        h2089
    })?)?))
}

pub fn f496() -> Adt23 {
    Adt23::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f497(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2090 = f44(v1, 34u8)?;
    match m2090.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2091 = v3;
            match m2091.uncons() {
                None => {
                    Ok(f548(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f498(v0: List<Bytes>) -> List<Bytes> {
    let m2092 = v0;
    match m2092.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f498(v2)
            } else {
                List::cons(v1, f498(v2))
            }
        }
    }
}

pub fn f499(p2095: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2094 = p2095;
    {
        let mut m2094 = m2094;
        loop {
            let step = {
                let v0 = m2094;
                let m2093 = v0;
                match m2093.uncons() {
                    None => {
                        Err::<List<Adt13>, Bytes>(v1.clone())
                    }
                    Some((v3, v4)) => {
                        if f86(v1.clone(), f103(v3.clone())) {
                            Err::<List<Adt13>, Bytes>(f69(v3, v2.clone()))
                        } else {
                            Ok::<List<Adt13>, Bytes>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2094 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f500(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2096 = v2;
    let a2097 = 0u64;
    if nat_eq(a2096, a2097) {
        Ok(f549(v0, 0u64))
    } else {
        let (_, h2098) = {
            let (h2099, _) = f550(Fn14::F551(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2099
        };
        Ok(h2098)
    }
}

pub fn f501() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f502(v0: List<u64>) -> R<Bytes> {
    let m2100 = f552(v0, List::<u64>::nil());
    match m2100.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2101 = f415(v1)?;
            let a2102 = f553(v2)?;
            Ok(append_bytes(a2101, a2102))
        }
    }
}

pub fn f503(p2105: u64, p2106: List<u64>, v2: u64) -> R<List<u64>> {
    let m2104 = (p2105, p2106);
    {
        let mut m2104 = m2104;
        loop {
            let step = {
                let (v0, v1) = m2104;
                let m2103 = v0;
                if m2103 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2103 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f554(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: u64, v1: u64) -> List<u64> {
    let m2107 = v0;
    if m2107 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2107 - 1;
        let a2108 = v1;
        let a2109 = 0u64;
        if nat_eq(a2108, a2109) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2110 = v1;
                let a2111 = f555();
                let a2112 = 0u64;
                nat_rem(a2110, a2111, a2112)
            }, f504(v2, {
                let a2113 = v1;
                let a2114 = f555();
                let a2115 = 0u64;
                nat_quot(a2113, a2114, a2115)
            }))
        }
    }
}

pub fn f505(p2126: Bytes, p2127: u64) -> R<Bytes> {
    let m2125 = (p2126, p2127);
    {
        let mut m2125 = m2125;
        loop {
            let step = {
                let (v0, v1) = m2125;
                let m2116 = v1;
                if m2116 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2116 - 1;
                    let a2120 = f116(v0.clone(), {
                        let a2118 = {
                            let a2117 = v0.clone();
                            length_bytes(a2117)
                        };
                        let a2119 = 1u64;
                        nat_sub(a2118, a2119)
                    })?;
                    let a2121 = 48u64;
                    if nat_eq(a2120, a2121) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2123 = {
                                let a2122 = v0;
                                length_bytes(a2122)
                            };
                            let a2124 = 1u64;
                            nat_sub(a2123, a2124)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(v0: u64) -> Bytes {
    let m2128 = v0;
    if m2128 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2128 - 1;
        let a2129 = Bytes::lit(&[0x30]);
        let a2130 = f506(v1);
        append_bytes(a2129, a2130)
    }
}

pub fn f507(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2175 = {
            let a2151 = {
                let a2139 = {
                    let a2133 = {
                        let a2131 = v3;
                        let a2132 = 128u8;
                        fixed_u8::bit_and(a2131, a2132)
                    };
                    let a2134 = 128u8;
                    if equal(a2133, a2134) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2140 = {
                    let a2137 = {
                        let a2135 = v3;
                        let a2136 = 64u8;
                        fixed_u8::bit_and(a2135, a2136)
                    };
                    let a2138 = 64u8;
                    if equal(a2137, a2138) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2139, a2140)?
            };
            let a2152 = {
                let a2149 = {
                    let a2143 = {
                        let a2141 = v3;
                        let a2142 = 32u8;
                        fixed_u8::bit_and(a2141, a2142)
                    };
                    let a2144 = 32u8;
                    if equal(a2143, a2144) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2150 = {
                    let a2147 = {
                        let a2145 = v3;
                        let a2146 = 16u8;
                        fixed_u8::bit_and(a2145, a2146)
                    };
                    let a2148 = 16u8;
                    if equal(a2147, a2148) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2149, a2150)?
            };
            nat_add(a2151, a2152)?
        };
        let a2176 = {
            let a2173 = {
                let a2161 = {
                    let a2155 = {
                        let a2153 = v3;
                        let a2154 = 8u8;
                        fixed_u8::bit_and(a2153, a2154)
                    };
                    let a2156 = 8u8;
                    if equal(a2155, a2156) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2162 = {
                    let a2159 = {
                        let a2157 = v3;
                        let a2158 = 4u8;
                        fixed_u8::bit_and(a2157, a2158)
                    };
                    let a2160 = 4u8;
                    if equal(a2159, a2160) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2161, a2162)?
            };
            let a2174 = {
                let a2171 = {
                    let a2165 = {
                        let a2163 = v3;
                        let a2164 = 2u8;
                        fixed_u8::bit_and(a2163, a2164)
                    };
                    let a2166 = 2u8;
                    if equal(a2165, a2166) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2172 = {
                    let a2169 = {
                        let a2167 = v3;
                        let a2168 = 1u8;
                        fixed_u8::bit_and(a2167, a2168)
                    };
                    let a2170 = 1u8;
                    if equal(a2169, a2170) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2171, a2172)?
            };
            nat_add(a2173, a2174)?
        };
        nat_add(a2175, a2176)?
    };
    let a2177 = v4;
    let a2178 = 34u64;
    if nat_eq(a2177, a2178) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2179 = v4;
        let a2180 = 92u64;
        if nat_eq(a2179, a2180) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2181 = v4;
            let a2182 = 8u64;
            if nat_eq(a2181, a2182) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2183 = v4;
                let a2184 = 12u64;
                if nat_eq(a2183, a2184) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2185 = v4;
                    let a2186 = 10u64;
                    if nat_eq(a2185, a2186) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2187 = v4;
                        let a2188 = 13u64;
                        if nat_eq(a2187, a2188) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2189 = v4;
                            let a2190 = 9u64;
                            if nat_eq(a2189, a2190) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2191 = v4;
                                let a2192 = 32u64;
                                if nat_lt(a2191, a2192) {
                                    let a2193 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2194 = f556(v2)?;
                                    Ok(append_bytes(a2193, a2194))
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

pub fn f508(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2195 = v0.clone();
    match m2195.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2198 = {
                let (h2196, _) = v2;
                h2196
            };
            let a2199 = {
                let (h2197, _) = v1.clone();
                h2197
            };
            if equal(a2198, a2199) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f509(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f557(f558(Fn15::F559, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2204 = {
        let (h2200, _) = f560(Fn16::F561, {
            let a2202 = {
                let a2201 = v0;
                length_list(a2201)
            };
            let a2203 = 1u64;
            nat_add(a2202, a2203)?
        }, v1)?;
        h2200
    };
    match m2204.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f510(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2209 = {
        let a2206 = f318({
            let (h2205, _) = v0;
            h2205
        })?;
        let a2207 = Bytes::lit(&[0x3a]);
        append_bytes(a2206, a2207)
    };
    let a2210 = {
        let (_, h2208) = v0;
        h2208
    };
    Ok(append_bytes(a2209, a2210))
}

pub fn f511(v0: Bytes) -> R<Bytes> {
    let a2216 = {
        let a2214 = Bytes::lit(&[0x22]);
        let a2215 = {
            let (_, h2211) = {
                let (h2212, _) = f550(Fn14::F562(v0.clone()), {
                    let a2213 = v0;
                    length_bytes(a2213)
                }, (0u64, Bytes::lit(&[])))?;
                h2212
            };
            h2211
        };
        append_bytes(a2214, a2215)
    };
    let a2217 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2216, a2217))
}

pub fn f512(v0: Bytes, v1: u64) -> bool {
    let m2218 = f212(v0, v1);
    match m2218 {
        None => {
            false
        }
        Some(v2) => {
            f563(v2)
        }
    }
}

pub fn f513(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2219 = v0.clone();
        length_bytes(a2219)
    };
    let a2223 = {
        let a2220 = v1;
        let a2221 = 2u64;
        let a2222 = 0u64;
        nat_rem(a2220, a2221, a2222)
    };
    let a2224 = 0u64;
    if nat_eq(a2223, a2224) {
        let (_, h2225) = {
            let (h2226, _) = f564(Fn17::F565(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2226
        };
        Ok(h2225)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f514(v0: u64) -> bool {
    let a2227 = v0;
    let a2228 = 8u64;
    if nat_eq(a2227, a2228) {
        true
    } else {
        let a2229 = v0;
        let a2230 = 9u64;
        if nat_eq(a2229, a2230) {
            true
        } else {
            let a2231 = v0;
            let a2232 = 10u64;
            if nat_eq(a2231, a2232) {
                true
            } else {
                let a2233 = v0;
                let a2234 = 12u64;
                if nat_eq(a2233, a2234) {
                    true
                } else {
                    let a2235 = v0;
                    let a2236 = 13u64;
                    nat_eq(a2235, a2236)
                }
            }
        }
    }
}

pub fn f515(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2239 = {
        let a2237 = v0;
        let a2238 = v1;
        index_bytes(a2237, a2238)
    };
    match m2239 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f566(v2)? {
                Ok(Some({
                    let a2240 = v1;
                    let a2241 = 1u64;
                    nat_add(a2240, a2241)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f516(v0: u8) -> R<u64> {
    let a2288 = {
        let v1: u8 = v0;
        let a2286 = {
            let a2262 = {
                let a2250 = {
                    let a2244 = {
                        let a2242 = v1;
                        let a2243 = 128u8;
                        fixed_u8::bit_and(a2242, a2243)
                    };
                    let a2245 = 128u8;
                    if equal(a2244, a2245) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2251 = {
                    let a2248 = {
                        let a2246 = v1;
                        let a2247 = 64u8;
                        fixed_u8::bit_and(a2246, a2247)
                    };
                    let a2249 = 64u8;
                    if equal(a2248, a2249) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2250, a2251)?
            };
            let a2263 = {
                let a2260 = {
                    let a2254 = {
                        let a2252 = v1;
                        let a2253 = 32u8;
                        fixed_u8::bit_and(a2252, a2253)
                    };
                    let a2255 = 32u8;
                    if equal(a2254, a2255) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2261 = {
                    let a2258 = {
                        let a2256 = v1;
                        let a2257 = 16u8;
                        fixed_u8::bit_and(a2256, a2257)
                    };
                    let a2259 = 16u8;
                    if equal(a2258, a2259) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2260, a2261)?
            };
            nat_add(a2262, a2263)?
        };
        let a2287 = {
            let a2284 = {
                let a2272 = {
                    let a2266 = {
                        let a2264 = v1;
                        let a2265 = 8u8;
                        fixed_u8::bit_and(a2264, a2265)
                    };
                    let a2267 = 8u8;
                    if equal(a2266, a2267) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2273 = {
                    let a2270 = {
                        let a2268 = v1;
                        let a2269 = 4u8;
                        fixed_u8::bit_and(a2268, a2269)
                    };
                    let a2271 = 4u8;
                    if equal(a2270, a2271) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2272, a2273)?
            };
            let a2285 = {
                let a2282 = {
                    let a2276 = {
                        let a2274 = v1;
                        let a2275 = 2u8;
                        fixed_u8::bit_and(a2274, a2275)
                    };
                    let a2277 = 2u8;
                    if equal(a2276, a2277) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2283 = {
                    let a2280 = {
                        let a2278 = v1;
                        let a2279 = 1u8;
                        fixed_u8::bit_and(a2278, a2279)
                    };
                    let a2281 = 1u8;
                    if equal(a2280, a2281) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2282, a2283)?
            };
            nat_add(a2284, a2285)?
        };
        nat_add(a2286, a2287)?
    };
    let a2289 = 48u64;
    Ok(nat_sub(a2288, a2289))
}

pub fn f517(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2290 = v0.clone();
        length_bytes(a2290)
    };
    let a2291 = v1;
    let a2292 = v2;
    if if nat_lt(a2291, a2292) {
        let a2297 = f116(v0, {
            let a2295 = v2;
            let a2296 = {
                let a2293 = v1;
                let a2294 = 1u64;
                nat_add(a2293, a2294)?
            };
            nat_sub(a2295, a2296)
        })?;
        let a2298 = 48u64;
        nat_eq(a2297, a2298)
    } else {
        false
    } {
        Ok(Some({
            let a2299 = v1;
            let a2300 = 1u64;
            nat_add(a2299, a2300)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f518(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f118(f32({
        let (h2301, _) = v0;
        h2301
    }, {
        let (h2302, _) = v1;
        h2302
    }), f32({
        let (_, h2303) = v0;
        h2303
    }, {
        let (_, h2304) = v1;
        h2304
    }))
}

pub fn f519(v0: Bytes) -> R<bool> {
    let m2305 = f349(v0.clone(), f8(Str::lit("&&")))?;
    if match m2305 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2306 = f349(v0, f8(Str::lit("||")))?;
        match m2306 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f520(v0: Adt18, v1: Option<u64>) -> bool {
    let m2307 = v1;
    match m2307 {
        None => {
            true
        }
        Some(v2) => {
            f460(v2, match v0 {
                Adt18::C0(_, h2308, _, _) => {
                    h2308
                }
            })
        }
    }
}

pub fn f521(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a2310 = match v0 {
            Adt15::C0(_, _, h2309, _, _) => {
                h2309
            }
        };
        length_list(a2310)
    })
}

pub fn f522(v0: Bytes) -> Option<Adt22> {
    if f32(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt22::C0)
    } else {
        if f32(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt22::C1)
        } else {
            if f32(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt22::C2)
            } else {
                None::<Adt22>
            }
        }
    }
}

pub fn f523(v0: Bytes, p2315: List<Bytes>, p2316: u64) -> R<Option<u64>> {
    let m2314 = (p2315, p2316);
    {
        let mut m2314 = m2314;
        loop {
            let step = {
                let (v1, v2) = m2314;
                let m2311 = v1;
                match m2311.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2312 = v2;
                                let a2313 = 1u64;
                                nat_add(a2312, a2313)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(v0: Option<Adt2>) -> Option<u64> {
    let m2317 = v0;
    match m2317 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2318 = v1;
            match m2318 {
                Adt2::C0 => {
                    None::<u64>
                }
                Adt2::C1(_, _) => {
                    None::<u64>
                }
                Adt2::C2(_) => {
                    None::<u64>
                }
                Adt2::C3(v7, v8) => {
                    if v7 {
                        None::<u64>
                    } else {
                        Some(v8)
                    }
                }
                Adt2::C4(_, _, _) => {
                    None::<u64>
                }
                Adt2::C5(_, _) => {
                    None::<u64>
                }
                Adt2::C6(_) => {
                    None::<u64>
                }
                Adt2::C7(_) => {
                    None::<u64>
                }
                Adt2::C8(_) => {
                    None::<u64>
                }
                Adt2::C9 => {
                    None::<u64>
                }
                Adt2::C10(_, _) => {
                    None::<u64>
                }
                Adt2::C11(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f525(v0: Fn12, p2322: List<List<(Bytes, ())>>, p2323: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2321 = (p2322, p2323);
    {
        let mut m2321 = m2321;
        loop {
            let step = {
                let (v1, v2) = m2321;
                let m2319 = v2;
                match m2319.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2320: Fn12 = v0.clone();
                            c2320.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f527(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2325 = {
        let a2324 = v0.clone();
        length_list(a2324)
    };
    let a2326 = 1u64;
    if nat_le(a2325, a2326) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f567(v0)?))
    }
}

pub fn f528(v0: Fn13, p2330: List<(Bytes, ())>, p2331: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2329 = (p2330, p2331);
    {
        let mut m2329 = m2329;
        loop {
            let step = {
                let (v1, v2) = m2329;
                let m2327 = v2;
                match m2327.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2328: Fn13 = v0.clone();
                            c2328.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f530(v0: Bytes) -> Bytes {
    f7(List::cons(f140(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f531(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f532(v0: Adt1, v1: Bytes, v2: List<Adt2>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f143(v2.clone(), f8(Str::lit("at")));
    let v4: List<Bytes> = f62(f306(v2), f306(f97(v0, f154())?));
    let a2333 = {
        let a2332 = v4.clone();
        length_list(a2332)
    };
    let a2334 = 0u64;
    if nat_eq(a2333, a2334) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2337 = {
            let a2335 = v4;
            length_list(a2335)
        };
        let a2338 = {
            let a2336 = v3;
            length_list(a2336)
        };
        if nat_eq(a2337, a2338) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f568(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f533(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt2>) -> R<(bool, Bytes)> {
    let m2339 = v3;
    match m2339.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f207(f569(v0.clone(), v6.clone())?, f52(v6.clone(), v2.clone())) {
                f533(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f531(v6.clone()))) {
                    let v9: (bool, Bytes) = f533(v0, v1.clone(), v2, v5)?;
                    let (h2340, _) = v9.clone();
                    if h2340 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2342 = f568(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2343 = {
                                let (_, h2341) = v9;
                                h2341
                            };
                            append_bytes(a2342, a2343)
                        }))
                    }
                } else {
                    if f118(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f570(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2344 = f8(Str::lit("Catalog."));
                        let a2345 = v8.clone();
                        append_bytes(a2344, a2345)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f533(v0, v1.clone(), v2, v5)?;
                        let (h2346, _) = v10.clone();
                        if h2346 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2348 = f568(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2349 = {
                                    let (_, h2347) = v10;
                                    h2347
                                };
                                append_bytes(a2348, a2349)
                            }))
                        }
                    } else {
                        Ok((true, f7(List::cons(f8(Str::lit("admit-registry ")), List::cons(v6, List::cons(f8(Str::lit(" ")), List::cons(v7, List::cons(f8(Str::lit(" ")), List::cons(v8, List::cons(f8(Str::lit(" ")), List::cons(f58(v4.clone(), f8(Str::lit("reader"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v4, f8(Str::lit("artifact"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))))))
                    }
                }
            }
        }
    }
}

pub fn f534(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt2> = f294(f396(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt2> = f294(f396(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f126(f143(v3.clone(), f8(Str::lit("id"))), f143(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f143(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f126(v6.clone(), f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2351 = {
        let a2350 = v7.clone();
        length_list(a2350)
    };
    let a2352 = 0u64;
    if nat_eq(a2351, a2352) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2355 = {
                let a2353 = v7;
                length_list(a2353)
            };
            let a2356 = {
                let a2354 = v5;
                length_list(a2354)
            };
            nat_eq(a2355, a2356)
        }) {
            Ok((false, f568(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f146(f126(f62(f571(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f572(v4), f126(v6, f143(v3, f8(Str::lit("id")))))))?;
            let a2358 = {
                let a2357 = v8.clone();
                length_list(a2357)
            };
            let a2359 = 0u64;
            if nat_eq(a2358, a2359) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f568(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f123(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f535(p2366: List<Bytes>, p2367: u64) -> Bytes {
    let m2365 = (p2366, p2367);
    {
        let mut m2365 = m2365;
        loop {
            let step = {
                let (v0, v1) = m2365;
                let m2360 = v0;
                match m2360.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a2361 = v1;
                        let a2362 = 0u64;
                        if nat_eq(a2361, a2362) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a2363 = v1;
                                let a2364 = 1u64;
                                nat_sub(a2363, a2364)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f536(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = if f86(v0.clone(), f8(Str::lit("["))) {
        f46(v0.clone(), 1u64, {
            let a2369 = {
                let a2368 = v0;
                length_bytes(a2368)
            };
            let a2370 = 2u64;
            nat_sub(a2369, a2370)
        })
    } else {
        v0
    };
    let v2: Bytes = if f41(v1.clone(), f8(Str::lit("..."))) {
        f46(v1.clone(), 0u64, {
            let a2372 = {
                let a2371 = v1;
                length_bytes(a2371)
            };
            let a2373 = 3u64;
            nat_sub(a2372, a2373)
        })
    } else {
        v1
    };
    let m2374 = f44(v2.clone(), 124u8)?;
    match m2374.uncons() {
        None => {
            Ok(v2)
        }
        Some((v3, _)) => {
            Ok(v3)
        }
    }
}

pub fn f537(v0: Bytes) -> Bytes {
    if if f41(v0.clone(), f8(Str::lit("..."))) {
        true
    } else {
        f41(v0, f8(Str::lit("...]")))
    } {
        f8(Str::lit("{\"type\":\"array\",\"items\":{\"type\":\"string\"}}"))
    } else {
        f8(Str::lit("{\"type\":\"string\"}"))
    }
}

pub fn f538(v0: u64) -> bool {
    let a2375 = v0;
    let a2376 = 47u64;
    nat_eq(a2375, a2376)
}

pub fn f539(v0: u64) -> bool {
    f207({
        let a2377 = v0;
        let a2378 = 95u64;
        nat_eq(a2377, a2378)
    }, f207({
        let a2379 = v0;
        let a2380 = 45u64;
        nat_eq(a2379, a2380)
    }, f207({
        let a2381 = v0;
        let a2382 = 32u64;
        nat_eq(a2381, a2382)
    }, f207({
        let a2383 = v0;
        let a2384 = 46u64;
        nat_eq(a2383, a2384)
    }, {
        let a2385 = v0;
        let a2386 = 47u64;
        nat_eq(a2385, a2386)
    }))))
}

pub fn f540(v0: u64) -> bool {
    f118({
        let a2387 = 97u64;
        let a2388 = v0;
        nat_le(a2387, a2388)
    }, {
        let a2389 = v0;
        let a2390 = 122u64;
        nat_le(a2389, a2390)
    })
}

pub fn f541(v0: u64) -> bool {
    f118({
        let a2391 = 65u64;
        let a2392 = v0;
        nat_le(a2391, a2392)
    }, {
        let a2393 = v0;
        let a2394 = 90u64;
        nat_le(a2393, a2394)
    })
}

pub fn f542(v0: u64) -> bool {
    f118({
        let a2395 = 48u64;
        let a2396 = v0;
        nat_le(a2395, a2396)
    }, {
        let a2397 = v0;
        let a2398 = 57u64;
        nat_le(a2397, a2398)
    })
}

pub fn f543(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2399 = v0;
    match m2399.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f368(v1)?, f543(v2)?))
        }
    }
}

pub fn f544(v0: u32) -> Bytes {
    let a2402 = {
        let a2400 = v0;
        let a2401 = 8u32;
        fixed_u32::bit_and(a2400, a2401)
    };
    let a2403 = 0u32;
    if equal(a2402, a2403) {
        let a2406 = {
            let a2404 = v0;
            let a2405 = 4u32;
            fixed_u32::bit_and(a2404, a2405)
        };
        let a2407 = 0u32;
        if equal(a2406, a2407) {
            let a2410 = {
                let a2408 = v0;
                let a2409 = 2u32;
                fixed_u32::bit_and(a2408, a2409)
            };
            let a2411 = 0u32;
            if equal(a2410, a2411) {
                let a2414 = {
                    let a2412 = v0;
                    let a2413 = 1u32;
                    fixed_u32::bit_and(a2412, a2413)
                };
                let a2415 = 0u32;
                if equal(a2414, a2415) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2418 = {
                    let a2416 = v0;
                    let a2417 = 1u32;
                    fixed_u32::bit_and(a2416, a2417)
                };
                let a2419 = 0u32;
                if equal(a2418, a2419) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2422 = {
                let a2420 = v0;
                let a2421 = 2u32;
                fixed_u32::bit_and(a2420, a2421)
            };
            let a2423 = 0u32;
            if equal(a2422, a2423) {
                let a2426 = {
                    let a2424 = v0;
                    let a2425 = 1u32;
                    fixed_u32::bit_and(a2424, a2425)
                };
                let a2427 = 0u32;
                if equal(a2426, a2427) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2430 = {
                    let a2428 = v0;
                    let a2429 = 1u32;
                    fixed_u32::bit_and(a2428, a2429)
                };
                let a2431 = 0u32;
                if equal(a2430, a2431) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2434 = {
            let a2432 = v0;
            let a2433 = 4u32;
            fixed_u32::bit_and(a2432, a2433)
        };
        let a2435 = 0u32;
        if equal(a2434, a2435) {
            let a2438 = {
                let a2436 = v0;
                let a2437 = 2u32;
                fixed_u32::bit_and(a2436, a2437)
            };
            let a2439 = 0u32;
            if equal(a2438, a2439) {
                let a2442 = {
                    let a2440 = v0;
                    let a2441 = 1u32;
                    fixed_u32::bit_and(a2440, a2441)
                };
                let a2443 = 0u32;
                if equal(a2442, a2443) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2446 = {
                    let a2444 = v0;
                    let a2445 = 1u32;
                    fixed_u32::bit_and(a2444, a2445)
                };
                let a2447 = 0u32;
                if equal(a2446, a2447) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2450 = {
                let a2448 = v0;
                let a2449 = 2u32;
                fixed_u32::bit_and(a2448, a2449)
            };
            let a2451 = 0u32;
            if equal(a2450, a2451) {
                let a2454 = {
                    let a2452 = v0;
                    let a2453 = 1u32;
                    fixed_u32::bit_and(a2452, a2453)
                };
                let a2455 = 0u32;
                if equal(a2454, a2455) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2458 = {
                    let a2456 = v0;
                    let a2457 = 1u32;
                    fixed_u32::bit_and(a2456, a2457)
                };
                let a2459 = 0u32;
                if equal(a2458, a2459) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f545(v0: u32, v1: u32) -> u32 {
    let m2462 = {
        let a2460 = v0;
        let a2461 = v1;
        fixed_u32::shift_right(a2460, a2461)
    };
    match m2462 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f546(v0: Adt23, v1: Adt24) -> R<Adt23> {
    let v2: Adt25 = f573(Fn18::F574, 64u64, Adt25::C0(match v0.clone() {
        Adt23::C0(h2463, _, _, _, _, _, _, _) => {
            h2463
        }
    }, match v0.clone() {
        Adt23::C0(_, h2464, _, _, _, _, _, _) => {
            h2464
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2465, _, _, _, _, _) => {
            h2465
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h2466, _, _, _, _) => {
            h2466
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, h2467, _, _, _) => {
            h2467
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2468, _, _) => {
            h2468
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2469, _) => {
            h2469
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h2470) => {
            h2470
        }
    }, v1, 0u64))?;
    Ok(Adt23::C0(f575(match v0.clone() {
        Adt23::C0(h2471, _, _, _, _, _, _, _) => {
            h2471
        }
    }, match v2.clone() {
        Adt25::C0(h2472, _, _, _, _, _, _, _, _, _) => {
            h2472
        }
    }), f575(match v0.clone() {
        Adt23::C0(_, h2473, _, _, _, _, _, _) => {
            h2473
        }
    }, match v2.clone() {
        Adt25::C0(_, h2474, _, _, _, _, _, _, _, _) => {
            h2474
        }
    }), f575(match v0.clone() {
        Adt23::C0(_, _, h2475, _, _, _, _, _) => {
            h2475
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h2476, _, _, _, _, _, _, _) => {
            h2476
        }
    }), f575(match v0.clone() {
        Adt23::C0(_, _, _, h2477, _, _, _, _) => {
            h2477
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h2478, _, _, _, _, _, _) => {
            h2478
        }
    }), f575(match v0.clone() {
        Adt23::C0(_, _, _, _, h2479, _, _, _) => {
            h2479
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h2480, _, _, _, _, _) => {
            h2480
        }
    }), f575(match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2481, _, _) => {
            h2481
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h2482, _, _, _, _) => {
            h2482
        }
    }), f575(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2483, _) => {
            h2483
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h2484, _, _, _) => {
            h2484
        }
    }), f575(match v0 {
        Adt23::C0(_, _, _, _, _, _, _, h2485) => {
            h2485
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h2486, _, _) => {
            h2486
        }
    })))
}

pub fn f547(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f576(v0.clone(), v1, v2, v3)?, f576(v0.clone(), v1, v2, {
        let a2487 = v3;
        let a2488 = 4u64;
        nat_add(a2487, a2488)?
    })?, f576(v0.clone(), v1, v2, {
        let a2489 = v3;
        let a2490 = 8u64;
        nat_add(a2489, a2490)?
    })?, f576(v0.clone(), v1, v2, {
        let a2491 = v3;
        let a2492 = 12u64;
        nat_add(a2491, a2492)?
    })?, f576(v0.clone(), v1, v2, {
        let a2493 = v3;
        let a2494 = 16u64;
        nat_add(a2493, a2494)?
    })?, f576(v0.clone(), v1, v2, {
        let a2495 = v3;
        let a2496 = 20u64;
        nat_add(a2495, a2496)?
    })?, f576(v0.clone(), v1, v2, {
        let a2497 = v3;
        let a2498 = 24u64;
        nat_add(a2497, a2498)?
    })?, f576(v0.clone(), v1, v2, {
        let a2499 = v3;
        let a2500 = 28u64;
        nat_add(a2499, a2500)?
    })?, f576(v0.clone(), v1, v2, {
        let a2501 = v3;
        let a2502 = 32u64;
        nat_add(a2501, a2502)?
    })?, f576(v0.clone(), v1, v2, {
        let a2503 = v3;
        let a2504 = 36u64;
        nat_add(a2503, a2504)?
    })?, f576(v0.clone(), v1, v2, {
        let a2505 = v3;
        let a2506 = 40u64;
        nat_add(a2505, a2506)?
    })?, f576(v0.clone(), v1, v2, {
        let a2507 = v3;
        let a2508 = 44u64;
        nat_add(a2507, a2508)?
    })?, f576(v0.clone(), v1, v2, {
        let a2509 = v3;
        let a2510 = 48u64;
        nat_add(a2509, a2510)?
    })?, f576(v0.clone(), v1, v2, {
        let a2511 = v3;
        let a2512 = 52u64;
        nat_add(a2511, a2512)?
    })?, f576(v0.clone(), v1, v2, {
        let a2513 = v3;
        let a2514 = 56u64;
        nat_add(a2513, a2514)?
    })?, f576(v0, v1, v2, {
        let a2515 = v3;
        let a2516 = 60u64;
        nat_add(a2515, a2516)?
    })?))
}

pub fn f548(p2519: List<Bytes>, v1: Bytes) -> bool {
    let m2518 = p2519;
    {
        let mut m2518 = m2518;
        loop {
            let step = {
                let v0 = m2518;
                let m2517 = v0;
                match m2517.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f86(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549(v0: Bytes, v1: u64) -> Bytes {
    let m2523 = {
        let a2520 = v0;
        let a2521 = v1;
        let a2522 = 1u64;
        slice_bytes(a2520, a2521, a2522)
    };
    match m2523 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f550(v0: Fn14, p2528: u64, p2529: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m2527 = (p2528, p2529);
    {
        let mut m2527 = m2527;
        loop {
            let step = {
                let (v1, v2) = m2527;
                let m2524 = v1;
                if m2524 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2524 - 1;
                    let m2526 = {
                        let c2525: Fn14 = v0.clone();
                        c2525.apply(v2.clone())?
                    };
                    match m2526 {
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
                Ok(next) => { m2527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f551(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f577(v1, v0, v2)
}

pub fn f552(p2532: List<u64>, p2533: List<u64>) -> List<u64> {
    let m2531 = (p2532, p2533);
    {
        let mut m2531 = m2531;
        loop {
            let step = {
                let (v0, v1) = m2531;
                let m2530 = v0;
                match m2530.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f553(v0: List<u64>) -> R<Bytes> {
    let m2534 = v0;
    match m2534.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a2535 = f578(v1)?;
            let a2536 = f553(v2)?;
            Ok(append_bytes(a2535, a2536))
        }
    }
}

pub fn f554(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m2537 = v0;
    match m2537.uncons() {
        None => {
            let a2538 = v2;
            let a2539 = 0u64;
            if nat_eq(a2538, a2539) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a2542 = {
                    let a2540 = v3;
                    let a2541 = v1;
                    nat_mul(a2540, a2541)?
                };
                let a2543 = v2;
                nat_add(a2542, a2543)?
            };
            Ok(List::cons({
                let a2544 = v5;
                let a2545 = f555();
                let a2546 = 0u64;
                nat_rem(a2544, a2545, a2546)
            }, f554(v4, v1, {
                let a2547 = v5;
                let a2548 = f555();
                let a2549 = 0u64;
                nat_quot(a2547, a2548, a2549)
            })?))
        }
    }
}

pub fn f555() -> u64 {
    1000000000u64
}

pub fn f556(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2594 = {
            let a2570 = {
                let a2558 = {
                    let a2552 = {
                        let a2550 = v1;
                        let a2551 = 128u8;
                        fixed_u8::bit_and(a2550, a2551)
                    };
                    let a2553 = 128u8;
                    if equal(a2552, a2553) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2559 = {
                    let a2556 = {
                        let a2554 = v1;
                        let a2555 = 64u8;
                        fixed_u8::bit_and(a2554, a2555)
                    };
                    let a2557 = 64u8;
                    if equal(a2556, a2557) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2558, a2559)?
            };
            let a2571 = {
                let a2568 = {
                    let a2562 = {
                        let a2560 = v1;
                        let a2561 = 32u8;
                        fixed_u8::bit_and(a2560, a2561)
                    };
                    let a2563 = 32u8;
                    if equal(a2562, a2563) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2569 = {
                    let a2566 = {
                        let a2564 = v1;
                        let a2565 = 16u8;
                        fixed_u8::bit_and(a2564, a2565)
                    };
                    let a2567 = 16u8;
                    if equal(a2566, a2567) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2568, a2569)?
            };
            nat_add(a2570, a2571)?
        };
        let a2595 = {
            let a2592 = {
                let a2580 = {
                    let a2574 = {
                        let a2572 = v1;
                        let a2573 = 8u8;
                        fixed_u8::bit_and(a2572, a2573)
                    };
                    let a2575 = 8u8;
                    if equal(a2574, a2575) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2581 = {
                    let a2578 = {
                        let a2576 = v1;
                        let a2577 = 4u8;
                        fixed_u8::bit_and(a2576, a2577)
                    };
                    let a2579 = 4u8;
                    if equal(a2578, a2579) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2580, a2581)?
            };
            let a2593 = {
                let a2590 = {
                    let a2584 = {
                        let a2582 = v1;
                        let a2583 = 2u8;
                        fixed_u8::bit_and(a2582, a2583)
                    };
                    let a2585 = 2u8;
                    if equal(a2584, a2585) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2591 = {
                    let a2588 = {
                        let a2586 = v1;
                        let a2587 = 1u8;
                        fixed_u8::bit_and(a2586, a2587)
                    };
                    let a2589 = 1u8;
                    if equal(a2588, a2589) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2590, a2591)?
            };
            nat_add(a2592, a2593)?
        };
        nat_add(a2594, a2595)?
    };
    let a2602 = f549(f579(), {
        let a2596 = v2;
        let a2597 = 16u64;
        let a2598 = 0u64;
        nat_quot(a2596, a2597, a2598)
    });
    let a2603 = f549(f579(), {
        let a2599 = v2;
        let a2600 = 16u64;
        let a2601 = 0u64;
        nat_rem(a2599, a2600, a2601)
    });
    Ok(append_bytes(a2602, a2603))
}

pub fn f557(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f580(Fn19::F581, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f558(v0: Fn15, p2607: List<List<(Bytes, Bytes)>>, p2608: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2606 = (p2607, p2608);
    {
        let mut m2606 = m2606;
        loop {
            let step = {
                let (v1, v2) = m2606;
                let m2604 = v2;
                match m2604.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2605: Fn15 = v0.clone();
                            c2605.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f559(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f560(v0: Fn16, p2613: u64, p2614: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2612 = (p2613, p2614);
    {
        let mut m2612 = m2612;
        loop {
            let step = {
                let (v1, v2) = m2612;
                let m2609 = v1;
                if m2609 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2609 - 1;
                    let m2611 = {
                        let c2610: Fn16 = v0.clone();
                        c2610.apply(v2.clone())?
                    };
                    match m2611 {
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
                Ok(next) => { m2612 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f582(v0)
}

pub fn f562(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f583(v0, v1)
}

pub fn f563(v0: u8) -> bool {
    let a2615 = v0;
    let a2616 = 32u8;
    if if if equal(a2615, a2616) {
        true
    } else {
        let a2617 = v0;
        let a2618 = 9u8;
        equal(a2617, a2618)
    } {
        true
    } else {
        let a2619 = v0;
        let a2620 = 10u8;
        equal(a2619, a2620)
    } {
        true
    } else {
        let a2621 = v0;
        let a2622 = 12u8;
        if equal(a2621, a2622) {
            true
        } else {
            let a2623 = v0;
            let a2624 = 13u8;
            equal(a2623, a2624)
        }
    }
}

pub fn f564(v0: Fn17, p2629: u64, p2630: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2628 = (p2629, p2630);
    {
        let mut m2628 = m2628;
        loop {
            let step = {
                let (v1, v2) = m2628;
                let m2625 = v1;
                if m2625 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2625 - 1;
                    let m2627 = {
                        let c2626: Fn17 = v0.clone();
                        c2626.apply(v2.clone())?
                    };
                    match m2627 {
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
                Ok(next) => { m2628 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f565(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f584(v0, v1)
}

pub fn f566(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2675 = {
            let a2651 = {
                let a2639 = {
                    let a2633 = {
                        let a2631 = v1;
                        let a2632 = 128u8;
                        fixed_u8::bit_and(a2631, a2632)
                    };
                    let a2634 = 128u8;
                    if equal(a2633, a2634) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2640 = {
                    let a2637 = {
                        let a2635 = v1;
                        let a2636 = 64u8;
                        fixed_u8::bit_and(a2635, a2636)
                    };
                    let a2638 = 64u8;
                    if equal(a2637, a2638) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2639, a2640)?
            };
            let a2652 = {
                let a2649 = {
                    let a2643 = {
                        let a2641 = v1;
                        let a2642 = 32u8;
                        fixed_u8::bit_and(a2641, a2642)
                    };
                    let a2644 = 32u8;
                    if equal(a2643, a2644) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2650 = {
                    let a2647 = {
                        let a2645 = v1;
                        let a2646 = 16u8;
                        fixed_u8::bit_and(a2645, a2646)
                    };
                    let a2648 = 16u8;
                    if equal(a2647, a2648) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2649, a2650)?
            };
            nat_add(a2651, a2652)?
        };
        let a2676 = {
            let a2673 = {
                let a2661 = {
                    let a2655 = {
                        let a2653 = v1;
                        let a2654 = 8u8;
                        fixed_u8::bit_and(a2653, a2654)
                    };
                    let a2656 = 8u8;
                    if equal(a2655, a2656) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2662 = {
                    let a2659 = {
                        let a2657 = v1;
                        let a2658 = 4u8;
                        fixed_u8::bit_and(a2657, a2658)
                    };
                    let a2660 = 4u8;
                    if equal(a2659, a2660) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2661, a2662)?
            };
            let a2674 = {
                let a2671 = {
                    let a2665 = {
                        let a2663 = v1;
                        let a2664 = 2u8;
                        fixed_u8::bit_and(a2663, a2664)
                    };
                    let a2666 = 2u8;
                    if equal(a2665, a2666) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2672 = {
                    let a2669 = {
                        let a2667 = v1;
                        let a2668 = 1u8;
                        fixed_u8::bit_and(a2667, a2668)
                    };
                    let a2670 = 1u8;
                    if equal(a2669, a2670) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2671, a2672)?
            };
            nat_add(a2673, a2674)?
        };
        nat_add(a2675, a2676)?
    };
    let a2677 = 48u64;
    let a2678 = v2;
    if nat_le(a2677, a2678) {
        let a2679 = v2;
        let a2680 = 57u64;
        Ok(nat_le(a2679, a2680))
    } else {
        Ok(false)
    }
}

pub fn f567(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f463({
        let (_, h2681) = {
            let (h2682, _) = f585(Fn20::F586, {
                let a2684 = {
                    let a2683 = v0.clone();
                    length_list(a2683)
                };
                let a2685 = 1u64;
                nat_add(a2684, a2685)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2682
        };
        h2681
    }))
}

pub fn f568(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f569(v0: Adt1, v1: Bytes) -> R<bool> {
    if f67(v0.clone(), f530(v1.clone())) {
        let m2686 = f96(f64(v0, f530(v1)))?;
        match m2686.uncons() {
            None => {
                Ok(false)
            }
            Some((v2, _)) => {
                let m2687 = f43(v2)?;
                match m2687 {
                    None => {
                        Ok(false)
                    }
                    Some(v4) => {
                        let m2688 = f94(v4, f8(Str::lit("reader")));
                        match m2688 {
                            None => {
                                Ok(false)
                            }
                            Some(_) => {
                                Ok(true)
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f570(p2692: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m2691 = p2692;
    {
        let mut m2691 = m2691;
        loop {
            let step = {
                let v0 = m2691;
                let m2689 = v0;
                match m2689.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f118(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2690 = f94(v3, f8(Str::lit("std")));
                            match m2690 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    f32(f58(v5, f8(Str::lit("row"))), v2.clone())
                                }
                            }
                        }) {
                            Err::<List<Adt2>, bool>(true)
                        } else {
                            Ok::<List<Adt2>, bool>(v4)
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

pub fn f571(v0: List<Adt2>) -> List<Bytes> {
    let m2693 = v0;
    match m2693.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2694 = f94(v1, f8(Str::lit("std")));
            match m2694 {
                None => {
                    f571(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f571(v2))
                }
            }
        }
    }
}

pub fn f572(v0: List<Adt2>) -> List<Bytes> {
    let m2695 = v0;
    match m2695.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2696 = f93(f94(v1, f8(Str::lit("req"))));
            match m2696 {
                None => {
                    f572(v2)
                }
                Some(v3) => {
                    List::cons(v3, f572(v2))
                }
            }
        }
    }
}

pub fn f573(v0: Fn18, p2700: u64, p2701: Adt25) -> R<Adt25> {
    let m2699 = (p2700, p2701);
    {
        let mut m2699 = m2699;
        loop {
            let step = {
                let (v1, v2) = m2699;
                let m2697 = v1;
                if m2697 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m2697 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c2698: Fn18 = v0.clone();
                        c2698.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f574(v0: Adt25) -> R<Adt25> {
    f587(v0)
}

pub fn f575(v0: u32, v1: u32) -> u32 {
    let m2704 = {
        let a2702 = f588(v0);
        let a2703 = f588(v1);
        fixed_u64::checked_add(a2702, a2703)
    };
    match m2704 {
        None => {
            0u32
        }
        Some(v2) => {
            f589(v2)
        }
    }
}

pub fn f576(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2707 = {
        let a2705 = v3;
        let a2706 = 8u64;
        nat_add(a2705, a2706)?
    };
    let a2708 = v2;
    if nat_eq(a2707, a2708) {
        Ok(f590({
            let a2711 = {
                let a2709 = v1;
                let a2710 = 8u64;
                nat_mul(a2709, a2710)?
            };
            let a2712 = 4294967296u64;
            let a2713 = 0u64;
            nat_quot(a2711, a2712, a2713)
        }))
    } else {
        let a2716 = {
            let a2714 = v3;
            let a2715 = 4u64;
            nat_add(a2714, a2715)?
        };
        let a2717 = v2;
        if nat_eq(a2716, a2717) {
            Ok(f590({
                let a2720 = {
                    let a2718 = v1;
                    let a2719 = 8u64;
                    nat_mul(a2718, a2719)?
                };
                let a2721 = 4294967296u64;
                let a2722 = 0u64;
                nat_rem(a2720, a2721, a2722)
            }))
        } else {
            Ok(f591(f592(v0.clone(), v1, v3), f592(v0.clone(), v1, {
                let a2723 = v3;
                let a2724 = 1u64;
                nat_add(a2723, a2724)?
            }), f592(v0.clone(), v1, {
                let a2725 = v3;
                let a2726 = 2u64;
                nat_add(a2725, a2726)?
            }), f592(v0, v1, {
                let a2727 = v3;
                let a2728 = 3u64;
                nat_add(a2727, a2728)?
            })))
        }
    }
}

pub fn f577(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2729, _) = v2;
        h2729
    };
    let a2730 = v3;
    let a2731 = 0u64;
    if nat_eq(a2730, a2731) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2732 = v3;
            let a2733 = v1;
            let a2734 = 0u64;
            nat_quot(a2732, a2733, a2734)
        }, {
            let a2739 = f549(v0, {
                let a2735 = v3;
                let a2736 = v1;
                let a2737 = 0u64;
                nat_rem(a2735, a2736, a2737)
            });
            let a2740 = {
                let (_, h2738) = v2;
                h2738
            };
            append_bytes(a2739, a2740)
        }))
    }
}

pub fn f578(v0: u64) -> R<Bytes> {
    let v1: Bytes = f415(v0)?;
    let a2744 = f506({
        let a2742 = 9u64;
        let a2743 = {
            let a2741 = v1.clone();
            length_bytes(a2741)
        };
        nat_sub(a2742, a2743)
    });
    let a2745 = v1;
    Ok(append_bytes(a2744, a2745))
}

pub fn f579() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f580(v0: Fn19, p2749: List<List<(Bytes, Bytes)>>, p2750: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2748 = (p2749, p2750);
    {
        let mut m2748 = m2748;
        loop {
            let step = {
                let (v1, v2) = m2748;
                let m2746 = v2;
                match m2746.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2747: Fn19 = v0.clone();
                            c2747.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f582(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2752 = {
        let a2751 = v0.clone();
        length_list(a2751)
    };
    let a2753 = 1u64;
    if nat_le(a2752, a2753) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f593(v0)?))
    }
}

pub fn f583(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2757 = {
        let a2755 = v0.clone();
        let a2756 = {
            let (h2754, _) = v1.clone();
            h2754
        };
        index_bytes(a2755, a2756)
    };
    match m2757 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2759 = {
                    let (h2758, _) = v1.clone();
                    h2758
                };
                let a2760 = 1u64;
                nat_add(a2759, a2760)?
            }, {
                let a2763 = {
                    let (_, h2761) = v1;
                    h2761
                };
                let a2764 = f507(v0, {
                    let (h2762, _) = v1;
                    h2762
                }, v2)?;
                append_bytes(a2763, a2764)
            })))
        }
    }
}

pub fn f584(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2765, _) = v1;
        h2765
    };
    let m2767 = {
        let (_, h2766) = v1;
        h2766
    };
    match m2767 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2770 = {
                let a2768 = v0.clone();
                let a2769 = v2;
                index_bytes(a2768, a2769)
            };
            match m2770 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2775 = {
                        let a2773 = v0;
                        let a2774 = {
                            let a2771 = v2;
                            let a2772 = 1u64;
                            nat_add(a2771, a2772)?
                        };
                        index_bytes(a2773, a2774)
                    };
                    match m2775 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f594(v4)? {
                                f594(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2776 = v2;
                                    let a2777 = 2u64;
                                    nat_add(a2776, a2777)?
                                }, Some({
                                    let a2782 = v3;
                                    let a2783 = f549(f595(), {
                                        let a2780 = {
                                            let a2778 = f596(v4)?;
                                            let a2779 = 16u64;
                                            nat_mul(a2778, a2779)?
                                        };
                                        let a2781 = f596(v5)?;
                                        nat_add(a2780, a2781)?
                                    });
                                    append_bytes(a2782, a2783)
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

pub fn f585(v0: Fn20, p2788: u64, p2789: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2787 = (p2788, p2789);
    {
        let mut m2787 = m2787;
        loop {
            let step = {
                let (v1, v2) = m2787;
                let m2784 = v1;
                if m2784 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2784 - 1;
                    let m2786 = {
                        let c2785: Fn20 = v0.clone();
                        c2785.apply(v2.clone())?
                    };
                    match m2786 {
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
                Ok(next) => { m2787 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f597(v0)
}

pub fn f587(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f598(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h2790) => {
            h2790
        }
    });
    let v2: u32 = f575(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h2791, _, _) => {
            h2791
        }
    }, f575(f599(match v0.clone() {
        Adt25::C0(_, _, _, _, h2792, _, _, _, _, _) => {
            h2792
        }
    }), f575(f600(match v0.clone() {
        Adt25::C0(_, _, _, _, h2793, _, _, _, _, _) => {
            h2793
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2794, _, _, _, _) => {
            h2794
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2795, _, _, _) => {
            h2795
        }
    }), f575(v1, {
        let m2798 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h2797, _) => {
                h2797
            }
        };
        match m2798 {
            Adt24::C0(h2796, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2796
            }
        }
    }))));
    let v3: u32 = f575(f601(match v0.clone() {
        Adt25::C0(h2799, _, _, _, _, _, _, _, _, _) => {
            h2799
        }
    }), f602(match v0.clone() {
        Adt25::C0(h2800, _, _, _, _, _, _, _, _, _) => {
            h2800
        }
    }, match v0.clone() {
        Adt25::C0(_, h2801, _, _, _, _, _, _, _, _) => {
            h2801
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2802, _, _, _, _, _, _, _) => {
            h2802
        }
    }));
    Ok(Adt25::C0(f575(v2, v3), match v0.clone() {
        Adt25::C0(h2803, _, _, _, _, _, _, _, _, _) => {
            h2803
        }
    }, match v0.clone() {
        Adt25::C0(_, h2804, _, _, _, _, _, _, _, _) => {
            h2804
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2805, _, _, _, _, _, _, _) => {
            h2805
        }
    }, f575(match v0.clone() {
        Adt25::C0(_, _, _, h2806, _, _, _, _, _, _) => {
            h2806
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h2807, _, _, _, _, _) => {
            h2807
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2808, _, _, _, _) => {
            h2808
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2809, _, _, _) => {
            h2809
        }
    }, f603(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h2810, _) => {
            h2810
        }
    }), {
        let a2812 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h2811) => {
                h2811
            }
        };
        let a2813 = 1u64;
        nat_add(a2812, a2813)?
    }))
}

pub fn f588(v0: u32) -> u64 {
    let m2815 = {
        let a2814 = v0;
        fixed_u64::convert(i128::from(a2814))
    };
    match m2815 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f589(v0: u64) -> u32 {
    let m2819 = {
        let a2818 = {
            let a2816 = v0;
            let a2817 = 4294967295u64;
            fixed_u64::bit_and(a2816, a2817)
        };
        fixed_u32::convert(i128::from(a2818))
    };
    match m2819 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f590(v0: u64) -> u32 {
    let (_, h2820) = {
        let (_, h2821) = f604(Fn21::F605, 32u64, (v0, (1u32, 0u32)));
        h2821
    };
    h2820
}

pub fn f591(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2826 = f606(v0, 24u32);
    let a2827 = {
        let a2824 = f606(v1, 16u32);
        let a2825 = {
            let a2822 = f606(v2, 8u32);
            let a2823 = v3;
            fixed_u32::bit_or(a2822, a2823)
        };
        fixed_u32::bit_or(a2824, a2825)
    };
    fixed_u32::bit_or(a2826, a2827)
}

pub fn f592(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2828 = v2;
    let a2829 = v1;
    if nat_lt(a2828, a2829) {
        let m2832 = {
            let a2830 = v0;
            let a2831 = v2;
            index_bytes(a2830, a2831)
        };
        match m2832 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2834 = {
                    let a2833 = v3;
                    fixed_u32::convert(i128::from(a2833))
                };
                match m2834 {
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
        let a2835 = v2;
        let a2836 = v1;
        if nat_eq(a2835, a2836) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f593(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f557({
        let (_, h2837) = {
            let (h2838, _) = f607(Fn22::F608, {
                let a2840 = {
                    let a2839 = v0.clone();
                    length_list(a2839)
                };
                let a2841 = 1u64;
                nat_add(a2840, a2841)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2838
        };
        h2837
    }))
}

pub fn f594(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2886 = {
            let a2862 = {
                let a2850 = {
                    let a2844 = {
                        let a2842 = v1;
                        let a2843 = 128u8;
                        fixed_u8::bit_and(a2842, a2843)
                    };
                    let a2845 = 128u8;
                    if equal(a2844, a2845) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2851 = {
                    let a2848 = {
                        let a2846 = v1;
                        let a2847 = 64u8;
                        fixed_u8::bit_and(a2846, a2847)
                    };
                    let a2849 = 64u8;
                    if equal(a2848, a2849) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2850, a2851)?
            };
            let a2863 = {
                let a2860 = {
                    let a2854 = {
                        let a2852 = v1;
                        let a2853 = 32u8;
                        fixed_u8::bit_and(a2852, a2853)
                    };
                    let a2855 = 32u8;
                    if equal(a2854, a2855) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2861 = {
                    let a2858 = {
                        let a2856 = v1;
                        let a2857 = 16u8;
                        fixed_u8::bit_and(a2856, a2857)
                    };
                    let a2859 = 16u8;
                    if equal(a2858, a2859) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2860, a2861)?
            };
            nat_add(a2862, a2863)?
        };
        let a2887 = {
            let a2884 = {
                let a2872 = {
                    let a2866 = {
                        let a2864 = v1;
                        let a2865 = 8u8;
                        fixed_u8::bit_and(a2864, a2865)
                    };
                    let a2867 = 8u8;
                    if equal(a2866, a2867) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2873 = {
                    let a2870 = {
                        let a2868 = v1;
                        let a2869 = 4u8;
                        fixed_u8::bit_and(a2868, a2869)
                    };
                    let a2871 = 4u8;
                    if equal(a2870, a2871) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2872, a2873)?
            };
            let a2885 = {
                let a2882 = {
                    let a2876 = {
                        let a2874 = v1;
                        let a2875 = 2u8;
                        fixed_u8::bit_and(a2874, a2875)
                    };
                    let a2877 = 2u8;
                    if equal(a2876, a2877) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2883 = {
                    let a2880 = {
                        let a2878 = v1;
                        let a2879 = 1u8;
                        fixed_u8::bit_and(a2878, a2879)
                    };
                    let a2881 = 1u8;
                    if equal(a2880, a2881) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2882, a2883)?
            };
            nat_add(a2884, a2885)?
        };
        nat_add(a2886, a2887)?
    };
    let a2888 = 48u64;
    let a2889 = v2;
    if if nat_le(a2888, a2889) {
        let a2890 = v2;
        let a2891 = 57u64;
        nat_le(a2890, a2891)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2892 = 97u64;
        let a2893 = v2;
        if nat_le(a2892, a2893) {
            let a2894 = v2;
            let a2895 = 102u64;
            Ok(nat_le(a2894, a2895))
        } else {
            Ok(false)
        }
    }
}

pub fn f595() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f596(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2940 = {
            let a2916 = {
                let a2904 = {
                    let a2898 = {
                        let a2896 = v1;
                        let a2897 = 128u8;
                        fixed_u8::bit_and(a2896, a2897)
                    };
                    let a2899 = 128u8;
                    if equal(a2898, a2899) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2905 = {
                    let a2902 = {
                        let a2900 = v1;
                        let a2901 = 64u8;
                        fixed_u8::bit_and(a2900, a2901)
                    };
                    let a2903 = 64u8;
                    if equal(a2902, a2903) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2904, a2905)?
            };
            let a2917 = {
                let a2914 = {
                    let a2908 = {
                        let a2906 = v1;
                        let a2907 = 32u8;
                        fixed_u8::bit_and(a2906, a2907)
                    };
                    let a2909 = 32u8;
                    if equal(a2908, a2909) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2915 = {
                    let a2912 = {
                        let a2910 = v1;
                        let a2911 = 16u8;
                        fixed_u8::bit_and(a2910, a2911)
                    };
                    let a2913 = 16u8;
                    if equal(a2912, a2913) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2914, a2915)?
            };
            nat_add(a2916, a2917)?
        };
        let a2941 = {
            let a2938 = {
                let a2926 = {
                    let a2920 = {
                        let a2918 = v1;
                        let a2919 = 8u8;
                        fixed_u8::bit_and(a2918, a2919)
                    };
                    let a2921 = 8u8;
                    if equal(a2920, a2921) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2927 = {
                    let a2924 = {
                        let a2922 = v1;
                        let a2923 = 4u8;
                        fixed_u8::bit_and(a2922, a2923)
                    };
                    let a2925 = 4u8;
                    if equal(a2924, a2925) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2926, a2927)?
            };
            let a2939 = {
                let a2936 = {
                    let a2930 = {
                        let a2928 = v1;
                        let a2929 = 2u8;
                        fixed_u8::bit_and(a2928, a2929)
                    };
                    let a2931 = 2u8;
                    if equal(a2930, a2931) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2937 = {
                    let a2934 = {
                        let a2932 = v1;
                        let a2933 = 1u8;
                        fixed_u8::bit_and(a2932, a2933)
                    };
                    let a2935 = 1u8;
                    if equal(a2934, a2935) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2936, a2937)?
            };
            nat_add(a2938, a2939)?
        };
        nat_add(a2940, a2941)?
    };
    let a2942 = v2;
    let a2943 = 57u64;
    if nat_le(a2942, a2943) {
        let a2944 = v2;
        let a2945 = 48u64;
        Ok(nat_sub(a2944, a2945))
    } else {
        let a2946 = v2;
        let a2947 = 87u64;
        Ok(nat_sub(a2946, a2947))
    }
}

pub fn f597(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2948, _) = v0;
        h2948
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2949) = v0;
        h2949
    };
    let m2950 = v1;
    match m2950.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2951 = v4;
            match m2951.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f609(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f598(v0: u64) -> u32 {
    let a2952 = v0;
    let a2953 = 32u64;
    if nat_lt(a2952, a2953) {
        let a2954 = v0;
        let a2955 = 16u64;
        if nat_lt(a2954, a2955) {
            let a2956 = v0;
            let a2957 = 8u64;
            if nat_lt(a2956, a2957) {
                let a2958 = v0;
                let a2959 = 4u64;
                if nat_lt(a2958, a2959) {
                    let a2960 = v0;
                    let a2961 = 2u64;
                    if nat_lt(a2960, a2961) {
                        let a2962 = v0;
                        let a2963 = 1u64;
                        if nat_lt(a2962, a2963) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2964 = v0;
                        let a2965 = 3u64;
                        if nat_lt(a2964, a2965) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2966 = v0;
                    let a2967 = 6u64;
                    if nat_lt(a2966, a2967) {
                        let a2968 = v0;
                        let a2969 = 5u64;
                        if nat_lt(a2968, a2969) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2970 = v0;
                        let a2971 = 7u64;
                        if nat_lt(a2970, a2971) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2972 = v0;
                let a2973 = 12u64;
                if nat_lt(a2972, a2973) {
                    let a2974 = v0;
                    let a2975 = 10u64;
                    if nat_lt(a2974, a2975) {
                        let a2976 = v0;
                        let a2977 = 9u64;
                        if nat_lt(a2976, a2977) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2978 = v0;
                        let a2979 = 11u64;
                        if nat_lt(a2978, a2979) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2980 = v0;
                    let a2981 = 14u64;
                    if nat_lt(a2980, a2981) {
                        let a2982 = v0;
                        let a2983 = 13u64;
                        if nat_lt(a2982, a2983) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2984 = v0;
                        let a2985 = 15u64;
                        if nat_lt(a2984, a2985) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2986 = v0;
            let a2987 = 24u64;
            if nat_lt(a2986, a2987) {
                let a2988 = v0;
                let a2989 = 20u64;
                if nat_lt(a2988, a2989) {
                    let a2990 = v0;
                    let a2991 = 18u64;
                    if nat_lt(a2990, a2991) {
                        let a2992 = v0;
                        let a2993 = 17u64;
                        if nat_lt(a2992, a2993) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2994 = v0;
                        let a2995 = 19u64;
                        if nat_lt(a2994, a2995) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2996 = v0;
                    let a2997 = 22u64;
                    if nat_lt(a2996, a2997) {
                        let a2998 = v0;
                        let a2999 = 21u64;
                        if nat_lt(a2998, a2999) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3000 = v0;
                        let a3001 = 23u64;
                        if nat_lt(a3000, a3001) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3002 = v0;
                let a3003 = 28u64;
                if nat_lt(a3002, a3003) {
                    let a3004 = v0;
                    let a3005 = 26u64;
                    if nat_lt(a3004, a3005) {
                        let a3006 = v0;
                        let a3007 = 25u64;
                        if nat_lt(a3006, a3007) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3008 = v0;
                        let a3009 = 27u64;
                        if nat_lt(a3008, a3009) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3010 = v0;
                    let a3011 = 30u64;
                    if nat_lt(a3010, a3011) {
                        let a3012 = v0;
                        let a3013 = 29u64;
                        if nat_lt(a3012, a3013) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3014 = v0;
                        let a3015 = 31u64;
                        if nat_lt(a3014, a3015) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3016 = v0;
        let a3017 = 48u64;
        if nat_lt(a3016, a3017) {
            let a3018 = v0;
            let a3019 = 40u64;
            if nat_lt(a3018, a3019) {
                let a3020 = v0;
                let a3021 = 36u64;
                if nat_lt(a3020, a3021) {
                    let a3022 = v0;
                    let a3023 = 34u64;
                    if nat_lt(a3022, a3023) {
                        let a3024 = v0;
                        let a3025 = 33u64;
                        if nat_lt(a3024, a3025) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3026 = v0;
                        let a3027 = 35u64;
                        if nat_lt(a3026, a3027) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3028 = v0;
                    let a3029 = 38u64;
                    if nat_lt(a3028, a3029) {
                        let a3030 = v0;
                        let a3031 = 37u64;
                        if nat_lt(a3030, a3031) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3032 = v0;
                        let a3033 = 39u64;
                        if nat_lt(a3032, a3033) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3034 = v0;
                let a3035 = 44u64;
                if nat_lt(a3034, a3035) {
                    let a3036 = v0;
                    let a3037 = 42u64;
                    if nat_lt(a3036, a3037) {
                        let a3038 = v0;
                        let a3039 = 41u64;
                        if nat_lt(a3038, a3039) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3040 = v0;
                        let a3041 = 43u64;
                        if nat_lt(a3040, a3041) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3042 = v0;
                    let a3043 = 46u64;
                    if nat_lt(a3042, a3043) {
                        let a3044 = v0;
                        let a3045 = 45u64;
                        if nat_lt(a3044, a3045) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3046 = v0;
                        let a3047 = 47u64;
                        if nat_lt(a3046, a3047) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3048 = v0;
            let a3049 = 56u64;
            if nat_lt(a3048, a3049) {
                let a3050 = v0;
                let a3051 = 52u64;
                if nat_lt(a3050, a3051) {
                    let a3052 = v0;
                    let a3053 = 50u64;
                    if nat_lt(a3052, a3053) {
                        let a3054 = v0;
                        let a3055 = 49u64;
                        if nat_lt(a3054, a3055) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3056 = v0;
                        let a3057 = 51u64;
                        if nat_lt(a3056, a3057) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3058 = v0;
                    let a3059 = 54u64;
                    if nat_lt(a3058, a3059) {
                        let a3060 = v0;
                        let a3061 = 53u64;
                        if nat_lt(a3060, a3061) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3062 = v0;
                        let a3063 = 55u64;
                        if nat_lt(a3062, a3063) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3064 = v0;
                let a3065 = 60u64;
                if nat_lt(a3064, a3065) {
                    let a3066 = v0;
                    let a3067 = 58u64;
                    if nat_lt(a3066, a3067) {
                        let a3068 = v0;
                        let a3069 = 57u64;
                        if nat_lt(a3068, a3069) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3070 = v0;
                        let a3071 = 59u64;
                        if nat_lt(a3070, a3071) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3072 = v0;
                    let a3073 = 62u64;
                    if nat_lt(a3072, a3073) {
                        let a3074 = v0;
                        let a3075 = 61u64;
                        if nat_lt(a3074, a3075) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3076 = v0;
                        let a3077 = 63u64;
                        if nat_lt(a3076, a3077) {
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

pub fn f599(v0: u32) -> u32 {
    let a3080 = {
        let a3078 = f610(v0, 6u32, 26u32);
        let a3079 = f610(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3078, a3079)
    };
    let a3081 = f610(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3080, a3081)
}

pub fn f600(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3087 = {
        let a3082 = v0;
        let a3083 = v1;
        fixed_u32::bit_and(a3082, a3083)
    };
    let a3088 = {
        let a3085 = {
            let a3084 = v0;
            fixed_u32::bit_not(a3084)
        };
        let a3086 = v2;
        fixed_u32::bit_and(a3085, a3086)
    };
    fixed_u32::bit_xor(a3087, a3088)
}

pub fn f601(v0: u32) -> u32 {
    let a3091 = {
        let a3089 = f610(v0, 2u32, 30u32);
        let a3090 = f610(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3089, a3090)
    };
    let a3092 = f610(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3091, a3092)
}

pub fn f602(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3101 = {
        let a3097 = {
            let a3093 = v0;
            let a3094 = v1;
            fixed_u32::bit_and(a3093, a3094)
        };
        let a3098 = {
            let a3095 = v0;
            let a3096 = v2;
            fixed_u32::bit_and(a3095, a3096)
        };
        fixed_u32::bit_xor(a3097, a3098)
    };
    let a3102 = {
        let a3099 = v1;
        let a3100 = v2;
        fixed_u32::bit_and(a3099, a3100)
    };
    fixed_u32::bit_xor(a3101, a3102)
}

pub fn f603(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h3103, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3103
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h3104, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3104
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h3105, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3105
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h3106, _, _, _, _, _, _, _, _, _, _, _) => {
            h3106
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h3107, _, _, _, _, _, _, _, _, _, _) => {
            h3107
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h3108, _, _, _, _, _, _, _, _, _) => {
            h3108
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h3109, _, _, _, _, _, _, _, _) => {
            h3109
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h3110, _, _, _, _, _, _, _) => {
            h3110
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3111, _, _, _, _, _, _) => {
            h3111
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h3112, _, _, _, _, _) => {
            h3112
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h3113, _, _, _, _) => {
            h3113
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3114, _, _, _) => {
            h3114
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3115, _, _) => {
            h3115
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3116, _) => {
            h3116
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3117) => {
            h3117
        }
    }, f575(f611(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3118, _) => {
            h3118
        }
    }), f575(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3119, _, _, _, _, _, _) => {
            h3119
        }
    }, f575(f612(match v0.clone() {
        Adt24::C0(_, h3120, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3120
        }
    }), match v0 {
        Adt24::C0(h3121, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3121
        }
    }))))
}

pub fn f604(v0: Fn21, p3125: u64, p3126: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3124 = (p3125, p3126);
    {
        let mut m3124 = m3124;
        loop {
            let step = {
                let (v1, v2) = m3124;
                let m3122 = v1;
                if m3122 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3122 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3123: Fn21 = v0.clone();
                        c3123.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3124 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f605(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3127, _) = v0;
        h3127
    };
    let v2: u32 = {
        let (h3128, _) = {
            let (_, h3129) = v0;
            h3129
        };
        h3128
    };
    let v3: u32 = {
        let (_, h3130) = {
            let (_, h3131) = v0;
            h3131
        };
        h3130
    };
    ({
        let a3132 = v1;
        let a3133 = 2u64;
        let a3134 = 0u64;
        nat_quot(a3132, a3133, a3134)
    }, (f606(v2, 1u32), {
        let a3138 = {
            let a3135 = v1;
            let a3136 = 2u64;
            let a3137 = 0u64;
            nat_rem(a3135, a3136, a3137)
        };
        let a3139 = 1u64;
        if nat_eq(a3138, a3139) {
            let a3140 = v3;
            let a3141 = v2;
            fixed_u32::bit_or(a3140, a3141)
        } else {
            v3
        }
    }))
}

pub fn f606(v0: u32, v1: u32) -> u32 {
    let m3144 = {
        let a3142 = v0;
        let a3143 = v1;
        fixed_u32::shift_left(a3142, a3143)
    };
    match m3144 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f607(v0: Fn22, p3149: u64, p3150: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3148 = (p3149, p3150);
    {
        let mut m3148 = m3148;
        loop {
            let step = {
                let (v1, v2) = m3148;
                let m3145 = v1;
                if m3145 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3145 - 1;
                    let m3147 = {
                        let c3146: Fn22 = v0.clone();
                        c3146.apply(v2.clone())?
                    };
                    match m3147 {
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
                Ok(next) => { m3148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f608(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f613(v0)
}

pub fn f609(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3155 = {
            let a3153 = {
                let a3151 = v0.clone();
                length_list(a3151)
            };
            let a3154 = {
                let a3152 = v1.clone();
                length_list(a3152)
            };
            nat_add(a3153, a3154)?
        };
        let a3156 = 1u64;
        nat_add(a3155, a3156)?
    };
    Ok(f468({
        let (_, h3157) = {
            let (_, h3158) = {
                let (h3159, _) = f614(Fn23::F615, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3159
            };
            h3158
        };
        h3157
    }))
}

pub fn f610(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3160 = f545(v0, v1);
    let a3161 = f606(v0, v2);
    fixed_u32::bit_or(a3160, a3161)
}

pub fn f611(v0: u32) -> u32 {
    let a3164 = {
        let a3162 = f610(v0, 17u32, 15u32);
        let a3163 = f610(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3162, a3163)
    };
    let a3165 = f545(v0, 10u32);
    fixed_u32::bit_xor(a3164, a3165)
}

pub fn f612(v0: u32) -> u32 {
    let a3168 = {
        let a3166 = f610(v0, 7u32, 25u32);
        let a3167 = f610(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3166, a3167)
    };
    let a3169 = f545(v0, 3u32);
    fixed_u32::bit_xor(a3168, a3169)
}

pub fn f613(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3170, _) = v0;
        h3170
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3171) = v0;
        h3171
    };
    let m3172 = v1;
    match m3172.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3173 = v4;
            match m3173.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f616(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f614(v0: Fn23, p3178: u64, p3179: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3177 = (p3178, p3179);
    {
        let mut m3177 = m3177;
        loop {
            let step = {
                let (v1, v2) = m3177;
                let m3174 = v1;
                if m3174 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3174 - 1;
                    let m3176 = {
                        let c3175: Fn23 = v0.clone();
                        c3175.apply(v2.clone())
                    };
                    match m3176 {
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
                Ok(next) => { m3177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f615(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f617(v0)
}

pub fn f616(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3184 = {
            let a3182 = {
                let a3180 = v0.clone();
                length_list(a3180)
            };
            let a3183 = {
                let a3181 = v1.clone();
                length_list(a3181)
            };
            nat_add(a3182, a3183)?
        };
        let a3185 = 1u64;
        nat_add(a3184, a3185)?
    };
    Ok(f618({
        let (_, h3186) = {
            let (_, h3187) = {
                let (h3188, _) = f619(Fn24::F620, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3188
            };
            h3187
        };
        h3186
    }))
}

pub fn f617(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3189, _) = v0.clone();
        h3189
    };
    let v2: List<(Bytes, ())> = {
        let (h3190, _) = {
            let (_, h3191) = v0.clone();
            h3191
        };
        h3190
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3192) = {
            let (_, h3193) = v0;
            h3193
        };
        h3192
    };
    let m3194 = v1.clone();
    match m3194.uncons() {
        None => {
            let m3195 = v2;
            match m3195.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3196 = v2.clone();
            match m3196.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f621({
                        let (h3197, _) = v8.clone();
                        h3197
                    }, {
                        let (h3198, _) = v6.clone();
                        h3198
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

pub fn f618(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f426(Fn5::F622, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f619(v0: Fn24, p3203: u64, p3204: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3202 = (p3203, p3204);
    {
        let mut m3202 = m3202;
        loop {
            let step = {
                let (v1, v2) = m3202;
                let m3199 = v1;
                if m3199 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3199 - 1;
                    let m3201 = {
                        let c3200: Fn24 = v0.clone();
                        c3200.apply(v2.clone())
                    };
                    match m3201 {
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
                Ok(next) => { m3202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f623(v0)
}

pub fn f621(v0: Bytes, v1: Bytes) -> bool {
    let a3209 = {
        let a3205 = v0;
        let a3206 = v1;
        compare_bytes(a3205, a3206)
    };
    let a3210 = {
        let a3207 = Bytes::lit(&[]);
        let a3208 = Bytes::lit(&[0x01]);
        compare_bytes(a3207, a3208)
    };
    equal(a3209, a3210)
}

pub fn f622(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f623(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3211, _) = v0.clone();
        h3211
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3212, _) = {
            let (_, h3213) = v0.clone();
            h3213
        };
        h3212
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3214) = {
            let (_, h3215) = v0;
            h3215
        };
        h3214
    };
    let m3216 = v1.clone();
    match m3216.uncons() {
        None => {
            let m3217 = v2;
            match m3217.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3218 = v2.clone();
            match m3218.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f621({
                        let (h3219, _) = v8.clone();
                        h3219
                    }, {
                        let (h3220, _) = v6.clone();
                        h3220
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
