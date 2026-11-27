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
    C0(Adt15, List<Adt2>, List<Adt2>, List<Bytes>, List<Bytes>),
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
    C0(Bytes, Adt2),
    C1(Bytes),
    C2,
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt25, u64),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, List<Adt24>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(List<Bytes>, List<Adt24>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(bool, List<(u64, u64)>, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F77(u8, Bytes),
    F187(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F187(k0) => f187(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F121,
    F238,
    F354,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F121 => f121(p0, p1),
            Fn1::F238 => f238(p0, p1),
            Fn1::F354 => f354(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F210(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F210(k0) => f210(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F324(Bytes),
    F325(Bytes),
    F430(Bytes),
    F431(u64, Bytes),
    F438(Bytes),
    F446(Bytes, u64, Bytes, u64),
    F512(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F324(k0) => f324(k0.clone(), p0),
            Fn3::F325(k0) => f325(k0.clone(), p0),
            Fn3::F430(k0) => Ok(f430(k0.clone(), p0)),
            Fn3::F431(k0, k1) => f431(*k0, k1.clone(), p0),
            Fn3::F438(k0) => f438(k0.clone(), p0),
            Fn3::F446(k0, k1, k2, k3) => f446(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F512(k0) => f512(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F328(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F328(k0) => f328(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F422,
    F760,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F422 => f422(p0, p1),
            Fn5::F760 => f760(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F425,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F425 => f425(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F457,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F457 => f457(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F460,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F460 => f460(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F462,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F462 => f462(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F465,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F465 => f465(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F484(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt23)) -> R<(u64, Adt23)> {
        match self {
            Fn11::F484(k0, k1, k2) => f484(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F523,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F523 => f523(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F526,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F526 => f526(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F557(u64, Bytes),
    F568(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn14::F557(k0, k1) => Ok(f557(*k0, k1.clone(), p0)),
            Fn14::F568(k0) => f568(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F565,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F565 => f565(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F567,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F567 => f567(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F571(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F571(k0) => f571(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F588,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt26) -> R<Adt26> {
        match self {
            Fn18::F588 => f588(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F609,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F609 => f609(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F614,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F614 => f614(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F676,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F676 => f676(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F687,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F687 => f687(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F718,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F718 => f718(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F746,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F746 => f746(p0),
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
            Adt12::C3(List::cons((f8(Str::lit(".github/workflows/track.yml")), f71()), v1), v2)
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
    let v1: u64 = f72(v0)?;
    let m206 = {
        let a204 = f73({
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
    let v1: Bytes = f74(v0)?;
    let m236 = f75(v1.clone(), 0u64, {
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
        let (h243, _) = f76(Fn0::F77(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h243
    };
    let v4: u64 = {
        let (h244, _) = {
            let (_, h245) = v3.clone();
            h245
        };
        h244
    };
    Ok(f78(List::cons(f46(v0, v4, {
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
            Ok(Adt12::C4(f79()))
        }
        Some((v2, v3)) => {
            if f32(v2.clone(), f8(Str::lit("agenda"))) {
                f80(v1)
            } else {
                if f32(v2.clone(), f8(Str::lit("build"))) {
                    Ok(f81(v1))
                } else {
                    if f32(v2.clone(), f8(Str::lit("check"))) {
                        f82(v1)
                    } else {
                        if f32(v2.clone(), f8(Str::lit("next"))) {
                            f83(v1, v3)
                        } else {
                            if f32(v2.clone(), f8(Str::lit("publish"))) {
                                f84(v1)
                            } else {
                                let m259 = v3;
                                match m259.uncons() {
                                    None => {
                                        Ok(Adt12::C4(f79()))
                                    }
                                    Some((v4, v5)) => {
                                        if if f85(v2.clone(), f8(Str::lit("read-"))) {
                                            true
                                        } else {
                                            f32(v2.clone(), f8(Str::lit("check-yaml")))
                                        } {
                                            Ok(Adt12::C4(f8(Str::lit("check-yaml, read-webidl and read-yaml run in verdant_readers, the command of the readers project, and read-xml, read-xsd and read-xsd-components in verdant_xml, the command of the XML readers project\u{a}"))))
                                        } else {
                                            if f85(v2.clone(), f8(Str::lit("stage-"))) {
                                                f86(v2, v4, v5, v1)
                                            } else {
                                                f87(v2, v4, v5, v1)
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
                        if f88(v1.clone())? {
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
            let m275 = f89(f90(v0.clone(), v2.clone()), v2.clone());
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
    f91(v0, v1)
}

pub fn f58(v0: Adt2, v1: Bytes) -> Bytes {
    let m276 = f92(f93(v0, v1));
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
    if f85(v0.clone(), f8(Str::lit("v"))) {
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
    f94(f95(v0)?, f8(Str::lit("id = \"")))
}

pub fn f64(v0: Adt1, v1: Bytes) -> Bytes {
    let m282 = f89(f90(v0, v1.clone()), v1);
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
    let v2: List<Adt2> = f96(v0, f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt2> = f97(v2.clone(), f61(v1.clone()));
    if f98(f99(v2), f99(v3.clone())) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" is pinned, and every admission of its registers cites it\u{a}")), List::<Bytes>::nil()))))))
    } else {
        Ok(Adt12::C3(List::cons((f8(Str::lit("model/admissions.json")), f100(Adt2::C7(std::rc::Rc::new(f101(v3))))?), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("re-cited the admissions of the verdant-engine registers at ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m285 = f89(f90(v0, v1.clone()), v1);
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
    f7(List::cons(f102(v0), List::cons(v1, List::<Bytes>::nil())))
}

pub fn f70(v0: Adt1, v1: Bytes) -> R<Adt12> {
    if f51(f52(f103(), match v0.clone() {
        Adt1::C0(_, _, h287, _) => {
            h287
        }
    })) {
        Ok(f54(f103(), f55(f7(List::cons(f8(Str::lit("git/ref/tags/")), List::cons(v1, List::<Bytes>::nil()))))))
    } else {
        let m288 = f56(v0.clone(), List::cons(f103(), List::<Bytes>::nil()));
        match m288.uncons() {
            None => {
                let v6: Adt2 = {
                    let m290 = f93({
                        let m289 = f57(v0.clone(), f103())?;
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
                    if f51(f52(f104(), match v0.clone() {
                        Adt1::C0(_, _, h291, _) => {
                            h291
                        }
                    })) {
                        Ok(f54(f104(), f55(f7(List::cons(f8(Str::lit("git/tags/")), List::cons(f58(v6, f8(Str::lit("sha"))), List::<Bytes>::nil()))))))
                    } else {
                        let m292 = f56(v0.clone(), List::cons(f104(), List::<Bytes>::nil()));
                        match m292.uncons() {
                            None => {
                                f105(v0.clone(), v1, {
                                    let m294 = f93({
                                        let m293 = f57(v0, f104())?;
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
                    f105(v0, v1, f58(v6, f8(Str::lit("sha"))))
                }
            }
            Some((v2, v3)) => {
                Ok(Adt12::C0(List::cons(v2, v3)))
            }
        }
    }
}

pub fn f71() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Engine.lex.tex. Do not edit.\u{a}# REQ-ENGINE-TRACKING: Verdant Studio tracks Verdant Engine releases. Every hour this runs\u{a}# operate stage-engine, which stages the latest release's four registers when they are new\u{a}# (advance then ingests them by sha256), and re-cites their admissions once they are ingested.\u{a}name: track\u{a}\u{a}on:\u{a}  schedule:\u{a}    - cron: '17 * * * *'\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}  actions: write\u{a}\u{a}jobs:\u{a}  track:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 5\u{a}    steps:\u{a}      - env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}        run: gh workflow run operate.yml --repo \"$GITHUB_REPOSITORY\" --ref main -f args=stage-engine\u{a}"))
}

pub fn f72(v0: u8) -> R<u64> {
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

pub fn f73(v0: u64) -> List<Str> {
    let a341 = v0;
    let a342 = 0u64;
    if nat_eq(a341, a342) {
        f106()
    } else {
        let a343 = v0;
        let a344 = 1u64;
        if nat_eq(a343, a344) {
            f107()
        } else {
            let a345 = v0;
            let a346 = 2u64;
            if nat_eq(a345, a346) {
                f108()
            } else {
                let a347 = v0;
                let a348 = 3u64;
                if nat_eq(a347, a348) {
                    f109()
                } else {
                    let a349 = v0;
                    let a350 = 4u64;
                    if nat_eq(a349, a350) {
                        f110()
                    } else {
                        let a351 = v0;
                        let a352 = 5u64;
                        if nat_eq(a351, a352) {
                            f111()
                        } else {
                            let a353 = v0;
                            let a354 = 6u64;
                            if nat_eq(a353, a354) {
                                f112()
                            } else {
                                let a355 = v0;
                                let a356 = 7u64;
                                if nat_eq(a355, a356) {
                                    f113()
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

pub fn f74(v0: Bytes) -> R<Bytes> {
    f114(v0.clone(), 0u64, {
        let a357 = v0;
        length_bytes(a357)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f75(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m358 = v2;
    if m358 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m358 - 1;
        let v5: u64 = f115(v0.clone(), v1)?;
        let a359 = v3;
        let a360 = 1u64;
        if nat_eq(a359, a360) {
            let m361 = f75(v0.clone(), v1, v4, 0u64)?;
            match m361 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f115(v0.clone(), {
                        let (_, h362) = v6.clone();
                        h362
                    })?;
                    let a363 = v7;
                    let a364 = 44u64;
                    if nat_eq(a363, a364) {
                        let m368 = f75(v0, {
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
                let m382 = f116(v0.clone(), {
                    let a380 = v1;
                    let a381 = 1u64;
                    nat_add(a380, a381)?
                })?;
                match m382 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f117({
                            let a383 = v5;
                            let a384 = 34u64;
                            nat_eq(a383, a384)
                        }, {
                            let a386 = f115(v0.clone(), {
                                let (h385, _) = v9.clone();
                                h385
                            })?;
                            let a387 = 58u64;
                            nat_eq(a386, a387)
                        }) {
                            let m391 = f75(v0.clone(), {
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
                                    let v11: u64 = f115(v0.clone(), {
                                        let (_, h392) = v10.clone();
                                        h392
                                    })?;
                                    let a393 = v11;
                                    let a394 = 44u64;
                                    if nat_eq(a393, a394) {
                                        let m398 = f75(v0, {
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
                    let m414 = f116(v0, {
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
                                    let a427 = f115(v0.clone(), {
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
                                        let m433 = f75(v0, {
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
                                        let a440 = f115(v0.clone(), {
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
                                            let m446 = f75(v0, {
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
                                        let m449 = f118(v0, v1)?;
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

pub fn f76(v0: Fn0, p456: u64, p457: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
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

pub fn f77(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f119(v1, v0, v2)
}

pub fn f78(v0: List<Bytes>) -> List<Bytes> {
    f120(Fn1::F121, List::<Bytes>::nil(), v0)
}

pub fn f79() -> Bytes {
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f122(f123(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f80(v0: Adt1) -> R<Adt12> {
    let m458 = f124(v0.clone());
    match m458.uncons() {
        None => {
            let m459 = f125(f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f126(v0.clone())?);
            match m459.uncons() {
                None => {
                    let v5: Adt14 = f127(v0.clone())?;
                    let v6: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(f125(f128(f129(match v5.clone() {
                        Adt14::C0(h460, _, _) => {
                            h460
                        }
                    }, f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f131(v6.clone())), f132(f129(match v5.clone() {
                        Adt14::C0(h461, _, _) => {
                            h461
                        }
                    }, f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f133(v5.clone(), v6, f134(v5, f135(v0.clone(), f136(v0)?)?))?)))))
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

pub fn f81(v0: Adt1) -> Adt12 {
    if f51(f52(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h462, _) => {
            h462
        }
    })) {
        Adt12::C2(f8(Str::lit("npx")), f137(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f51(f52(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h463, _) => {
                h463
            }
        })) {
            Adt12::C2(f8(Str::lit("npx")), f137(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f82(v0: Adt1) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h464, _, _) => {
            h464
        }
    }, f139())) {
        Ok(Adt12::C1(List::cons(f139(), List::<Bytes>::nil())))
    } else {
        let m465 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f140(v0.clone())?)));
        match m465.uncons() {
            None => {
                let m466 = f141(v0.clone(), f140(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m466.uncons() {
                    None => {
                        f143(v0)
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

pub fn f83(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h467, _, _) => {
            h467
        }
    }, f144())) {
        Ok(Adt12::C1(List::cons(f144(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f145(f146(f144(), f147(match v0.clone() {
            Adt1::C0(_, h468, _, _) => {
                h468
            }
        }, f144())))?;
        let m469 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m469.uncons() {
            None => {
                let m470 = f148(v0.clone(), f149(f150(f95(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?, f151(v2, v1.clone()))?;
                match m470 {
                    None => {
                        f152(v0, v1)
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

pub fn f84(v0: Adt1) -> R<Adt12> {
    let m471 = f124(v0.clone());
    match m471.uncons() {
        None => {
            let m472 = f125(f56(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f153(), List::<Bytes>::nil())))), f126(v0.clone())?);
            match m472.uncons() {
                None => {
                    Ok(Adt12::C3(f154(f155(v0.clone())?, List::cons((f156(), f157()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f158()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f159()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f160()), List::cons((f8(Str::lit(".github/scripts/readmit.sh")), f161()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f162()), List::cons((f8(Str::lit("docs/catalog.md")), f163(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f164(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f165(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f166()), List::<(Bytes, Bytes)>::nil()))))))))))), f8(Str::lit("published the workflows, the export and readmit scripts, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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

pub fn f85(v0: Bytes, v1: Bytes) -> bool {
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

pub fn f86(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f167(v3.clone(), v1.clone(), f168(v3, v1, false)?)
    } else {
        if f32(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f167(v3.clone(), v1.clone(), f169(v3, v1)?)
        } else {
            if f32(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f167(v3.clone(), v1.clone(), f168(v3, v1, true)?)
            } else {
                if f32(v0.clone(), f8(Str::lit("stage-registries"))) {
                    f167(v3.clone(), v1.clone(), f170(v3, v1)?)
                } else {
                    if f32(v0.clone(), f8(Str::lit("stage-encodings"))) {
                        f167(v3.clone(), v1.clone(), f171(v3, v1)?)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("stage-mappings"))) {
                            f167(v3.clone(), v1.clone(), f172(v3, v1, false)?)
                        } else {
                            if f32(v0, f8(Str::lit("stage-matrix"))) {
                                f167(v3.clone(), v1.clone(), f172(v3, v1, true)?)
                            } else {
                                Ok(Adt12::C4(f79()))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("admit"))) {
        f173(v3, v1)
    } else {
        if f32(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f174(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("admit-registry"))) {
                f175(v3, v1, v2)
            } else {
                if f32(v0.clone(), f8(Str::lit("ingest-authority"))) {
                    f176(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("ingest-index"))) {
                        f177(v3, v1)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("merge-gate"))) {
                            f178(v3, v1)
                        } else {
                            if f32(v0.clone(), f8(Str::lit("propose-encoding"))) {
                                let m480 = v2;
                                match m480.uncons() {
                                    None => {
                                        Ok(Adt12::C4(f79()))
                                    }
                                    Some((v4, v5)) => {
                                        f179(v3, v1, v4, v5)
                                    }
                                }
                            } else {
                                if f32(v0.clone(), f8(Str::lit("admit-translated"))) {
                                    let m481 = v2;
                                    match m481.uncons() {
                                        None => {
                                            Ok(Adt12::C4(f79()))
                                        }
                                        Some((v6, v7)) => {
                                            f180(v3, v1, v6, v7)
                                        }
                                    }
                                } else {
                                    if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                                        f181(v3, v1)
                                    } else {
                                        if f32(v0, f8(Str::lit("record-error"))) {
                                            f182(v3, v1, v2)
                                        } else {
                                            Ok(Adt12::C4(f79()))
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

pub fn f88(v0: Bytes) -> R<bool> {
    f183(v0.clone(), 0u64, {
        let a482 = v0;
        length_bytes(a482)
    })
}

pub fn f89(p487: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m486 = p487;
    {
        let mut m486 = m486;
        loop {
            let step = {
                let v0 = m486;
                let m483 = v0;
                match m483.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h484, _) = v2;
                            h484
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h485) = v2;
                                h485
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m491 = {
        let a489 = match v0 {
            Adt1::C0(h488, _, _, _) => {
                h488
            }
        };
        let a490 = f40(v1);
        index_list(a489, a490)
    };
    match m491 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m492 = f184(f185(v0.clone(), v1.clone()), v1.clone());
    match m492 {
        None => {
            f43(f64(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f92(v0: Option<Adt2>) -> Option<Bytes> {
    let m493 = v0;
    match m493 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m494 = v1;
            match m494 {
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

pub fn f93(p500: Adt2, v1: Bytes) -> Option<Adt2> {
    let m499 = p500;
    {
        let mut m499 = m499;
        loop {
            let step = {
                let v0 = m499;
                let m495 = v0;
                match m495 {
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
                    Adt2::C8(r496) => {
                        let v5 = (*r496).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C9 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(v2, r497, r498) => {
                        let v3 = (*r497).clone();
                        let v4 = (*r498).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m499 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m501 = v0;
    match m501.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m502 = f186(v2, v1.clone())?;
            match m502 {
                None => {
                    f94(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f94(v3, v1)?))
                }
            }
        }
    }
}

pub fn f95(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a503 = v0.clone();
        length_bytes(a503)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h504, _) = f76(Fn0::F187(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h504
    };
    let v3: u64 = {
        let (h505, _) = {
            let (_, h506) = v2.clone();
            h506
        };
        h505
    };
    let v4: List<Bytes> = {
        let (_, h507) = {
            let (_, h508) = v2;
            h508
        };
        h507
    };
    let a509 = v3;
    let a510 = v1;
    if nat_lt(a509, a510) {
        Ok(f78(List::cons(f46(v0, v3, {
            let a511 = v1;
            let a512 = v3;
            nat_sub(a511, a512)
        }), v4)))
    } else {
        Ok(f78(v4))
    }
}

pub fn f96(v0: Adt1, v1: Bytes) -> R<List<Adt2>> {
    Ok(f188(f57(v0, v1)?))
}

pub fn f97(v0: List<Adt2>, v1: Bytes) -> List<Adt2> {
    let m513 = v0;
    match m513.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f189(v2, v1.clone()), f97(v3, v1))
        }
    }
}

pub fn f98(p518: List<Bytes>, p519: List<Bytes>) -> bool {
    let m517 = (p518, p519);
    {
        let mut m517 = m517;
        loop {
            let step = {
                let (v0, v1) = m517;
                let m514 = v0;
                match m514.uncons() {
                    None => {
                        let m515 = v1;
                        match m515.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m516 = v1;
                        match m516.uncons() {
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
                Ok(next) => { m517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: List<Adt2>) -> List<Bytes> {
    let m520 = v0;
    match m520.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m521 = f93(v1, f8(Str::lit("std")));
            match m521 {
                None => {
                    f99(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f99(v2))
                }
            }
        }
    }
}

pub fn f100(v0: Adt2) -> R<Bytes> {
    let a522 = f190(v0)?;
    let a523 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a522, a523))
}

pub fn f101(v0: List<Adt2>) -> Adt2 {
    let m524 = v0;
    match m524.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(v1), std::rc::Rc::new(f101(v2)))
        }
    }
}

pub fn f102(v0: Adt13) -> Bytes {
    let m525 = v0;
    match m525 {
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

pub fn f103() -> Bytes {
    f8(Str::lit("target/engine/ref.json"))
}

pub fn f104() -> Bytes {
    f8(Str::lit("target/engine/tag.json"))
}

pub fn f105(v0: Adt1, v1: Bytes, v2: Bytes) -> R<Adt12> {
    let a527 = {
        let a526 = v2.clone();
        length_bytes(a526)
    };
    let a528 = 0u64;
    if nat_eq(a527, a528) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m529 = f191(v0.clone(), f60())?;
        match m529 {
            None => {
                if f51(f52(f194(), match v0.clone() {
                    Adt1::C0(_, _, h530, _) => {
                        h530
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f194()))
                } else {
                    let m531 = f56(v0.clone(), List::cons(f195(), f196(f60())?));
                    match m531.uncons() {
                        None => {
                            Ok(Adt12::C3(f197(v0.clone(), f60(), v1.clone(), v2.clone(), f198(f64(v0, f195()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f192(v3.clone())?, f193(v3, v2)))
            }
        }
    }
}

pub fn f106() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f107() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f108() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f109() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f110() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f111() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f112() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f113() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f114(v0: Bytes, p559: u64, p560: u64, p561: bool, p562: bool, p563: u64, p564: Bytes) -> R<Bytes> {
    let m558 = (p559, (p560, (p561, (p562, (p563, p564)))));
    {
        let mut m558 = m558;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m558;
                let m532 = v2;
                if m532 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f199(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m532 - 1;
                    let m535 = {
                        let a533 = v0.clone();
                        let a534 = v1;
                        index_bytes(a533, a534)
                    };
                    match m535 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f199(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f72(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a536 = v1;
                                    let a537 = 1u64;
                                    nat_add(a536, a537)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a538 = v9;
                                    let a539 = 34u64;
                                    !nat_eq(a538, a539)
                                }, (if v4 {
                                    false
                                } else {
                                    let a540 = v9;
                                    let a541 = 92u64;
                                    nat_eq(a540, a541)
                                }, (v5, v6))))))
                            } else {
                                let a542 = v9;
                                let a543 = 32u64;
                                if if if if nat_eq(a542, a543) {
                                    true
                                } else {
                                    let a544 = v9;
                                    let a545 = 9u64;
                                    nat_eq(a544, a545)
                                } {
                                    true
                                } else {
                                    let a546 = v9;
                                    let a547 = 10u64;
                                    nat_eq(a546, a547)
                                } {
                                    true
                                } else {
                                    let a548 = v9;
                                    let a549 = 13u64;
                                    nat_eq(a548, a549)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a550 = v1;
                                        let a551 = 1u64;
                                        nat_add(a550, a551)?
                                    }, (v7, (false, (false, ({
                                        let a552 = v1;
                                        let a553 = 1u64;
                                        nat_add(a552, a553)?
                                    }, f199(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a554 = v1;
                                        let a555 = 1u64;
                                        nat_add(a554, a555)?
                                    }, (v7, ({
                                        let a556 = v9;
                                        let a557 = 34u64;
                                        nat_eq(a556, a557)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m558 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: Bytes, v1: u64) -> R<u64> {
    let m567 = {
        let a565 = v0;
        let a566 = v1;
        index_bytes(a565, a566)
    };
    match m567 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a612 = {
                let a588 = {
                    let a576 = {
                        let a570 = {
                            let a568 = v3;
                            let a569 = 128u8;
                            fixed_u8::bit_and(a568, a569)
                        };
                        let a571 = 128u8;
                        if equal(a570, a571) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a577 = {
                        let a574 = {
                            let a572 = v3;
                            let a573 = 64u8;
                            fixed_u8::bit_and(a572, a573)
                        };
                        let a575 = 64u8;
                        if equal(a574, a575) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a576, a577)?
                };
                let a589 = {
                    let a586 = {
                        let a580 = {
                            let a578 = v3;
                            let a579 = 32u8;
                            fixed_u8::bit_and(a578, a579)
                        };
                        let a581 = 32u8;
                        if equal(a580, a581) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a587 = {
                        let a584 = {
                            let a582 = v3;
                            let a583 = 16u8;
                            fixed_u8::bit_and(a582, a583)
                        };
                        let a585 = 16u8;
                        if equal(a584, a585) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a586, a587)?
                };
                nat_add(a588, a589)?
            };
            let a613 = {
                let a610 = {
                    let a598 = {
                        let a592 = {
                            let a590 = v3;
                            let a591 = 8u8;
                            fixed_u8::bit_and(a590, a591)
                        };
                        let a593 = 8u8;
                        if equal(a592, a593) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a599 = {
                        let a596 = {
                            let a594 = v3;
                            let a595 = 4u8;
                            fixed_u8::bit_and(a594, a595)
                        };
                        let a597 = 4u8;
                        if equal(a596, a597) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a598, a599)?
                };
                let a611 = {
                    let a608 = {
                        let a602 = {
                            let a600 = v3;
                            let a601 = 2u8;
                            fixed_u8::bit_and(a600, a601)
                        };
                        let a603 = 2u8;
                        if equal(a602, a603) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a609 = {
                        let a606 = {
                            let a604 = v3;
                            let a605 = 1u8;
                            fixed_u8::bit_and(a604, a605)
                        };
                        let a607 = 1u8;
                        if equal(a606, a607) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a608, a609)?
                };
                nat_add(a610, a611)?
            };
            nat_add(a612, a613)
        }
    }
}

pub fn f116(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f200(v0.clone(), v1, {
        let a615 = {
            let a614 = v0.clone();
            length_bytes(a614)
        };
        let a616 = 1u64;
        nat_add(a615, a616)?
    })?;
    let a617 = f115(v0.clone(), v2)?;
    let a618 = 34u64;
    if nat_eq(a617, a618) {
        Ok(Some(({
            let a619 = v2;
            let a620 = 1u64;
            nat_add(a619, a620)?
        }, f46(v0, v1, {
            let a621 = v2;
            let a622 = v1;
            nat_sub(a621, a622)
        }))))
    } else {
        f201(v0.clone(), v1, {
            let a624 = {
                let a623 = v0;
                length_bytes(a623)
            };
            let a625 = 1u64;
            nat_add(a624, a625)?
        }, Bytes::lit(&[]))
    }
}

pub fn f117(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f118(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a626 = f115(v0.clone(), v1)?;
        let a627 = 45u64;
        nat_eq(a626, a627)
    };
    let v3: u64 = if v2 {
        let a628 = v1;
        let a629 = 1u64;
        nat_add(a628, a629)?
    } else {
        v1
    };
    let v4: u64 = f202(v0.clone(), v3)?;
    let v5: u64 = {
        let a630 = v4;
        let a631 = v3;
        nat_sub(a630, a631)
    };
    let v6: bool = {
        let a632 = f115(v0.clone(), v4)?;
        let a633 = 46u64;
        nat_eq(a632, a633)
    };
    let v7: u64 = if v6 {
        let a634 = v4;
        let a635 = 1u64;
        nat_add(a634, a635)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f202(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f115(v0.clone(), v8)?;
    let v10: bool = {
        let a636 = v9;
        let a637 = 101u64;
        if nat_eq(a636, a637) {
            true
        } else {
            let a638 = v9;
            let a639 = 69u64;
            nat_eq(a638, a639)
        }
    };
    let v11: u64 = f115(v0.clone(), {
        let a640 = v8;
        let a641 = 1u64;
        nat_add(a640, a641)?
    })?;
    let v12: bool = f117(v10, f203({
        let a642 = v11;
        let a643 = 43u64;
        nat_eq(a642, a643)
    }, {
        let a644 = v11;
        let a645 = 45u64;
        nat_eq(a644, a645)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a646 = v8;
            let a647 = 2u64;
            nat_add(a646, a647)?
        } else {
            let a648 = v8;
            let a649 = 1u64;
            nat_add(a648, a649)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f202(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a650 = v14;
        let a651 = v13;
        nat_sub(a650, a651)
    });
    let v16: u64 = f204(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a653 = {
            let a652 = v15;
            length_bytes(a652)
        };
        let a654 = v16;
        nat_sub(a653, a654)
    });
    let a655 = v5;
    let a656 = 0u64;
    if if nat_eq(a655, a656) {
        true
    } else {
        if f117({
            let a657 = 1u64;
            let a658 = v5;
            nat_lt(a657, a658)
        }, {
            let a659 = f115(v0.clone(), v3)?;
            let a660 = 48u64;
            nat_eq(a659, a660)
        }) {
            true
        } else {
            if f117(v6, {
                let a661 = v8;
                let a662 = v7;
                nat_eq(a661, a662)
            }) {
                true
            } else {
                f203(if v10 {
                    let a663 = v14;
                    let a664 = v13;
                    nat_eq(a663, a664)
                } else {
                    false
                }, {
                    let a666 = 18u64;
                    let a667 = {
                        let a665 = v17.clone();
                        length_bytes(a665)
                    };
                    nat_lt(a666, a667)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m671 = {
            let a669 = {
                let a668 = v17.clone();
                length_bytes(a668)
            };
            let a670 = 0u64;
            if nat_eq(a669, a670) {
                Some(0u64)
            } else {
                f205(v17)?
            }
        };
        match m671 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m676 = f206(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a672 = v8;
                    let a673 = v7;
                    nat_sub(a672, a673)
                }), f117(v12, {
                    let a674 = v11;
                    let a675 = 45u64;
                    nat_eq(a674, a675)
                }), v18)?;
                match m676 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a679 = 19u64;
                        let a680 = {
                            let a678 = {
                                let (h677, _) = v19.clone();
                                h677
                            };
                            length_bytes(a678)
                        };
                        if nat_lt(a679, a680) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m682 = f205({
                                let (h681, _) = v19.clone();
                                h681
                            })?;
                            match m682 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f117(v2, {
                                        let a683 = 0u64;
                                        let a684 = v20;
                                        nat_lt(a683, a684)
                                    });
                                    if f207(v21, v20) {
                                        Ok(Some((v14, {
                                            let a686 = {
                                                let (_, h685) = v19.clone();
                                                h685
                                            };
                                            let a687 = 0u64;
                                            if nat_eq(a686, a687) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h688) = v19;
                                                    h688
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

pub fn f119(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h689, _) = v2.clone();
        h689
    };
    let v4: u64 = {
        let (h690, _) = {
            let (_, h691) = v2.clone();
            h691
        };
        h690
    };
    let v5: List<Bytes> = {
        let (_, h692) = {
            let (_, h693) = v2;
            h693
        };
        h692
    };
    let m694 = f208(v0.clone(), v3);
    match m694 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a695 = v6;
            let a696 = v1;
            if equal(a695, a696) {
                Ok(Some(({
                    let a697 = v3;
                    let a698 = 1u64;
                    nat_add(a697, a698)?
                }, ({
                    let a699 = v3;
                    let a700 = 1u64;
                    nat_add(a699, a700)?
                }, List::cons(f46(v0, v4, {
                    let a701 = v3;
                    let a702 = v4;
                    nat_sub(a701, a702)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a703 = v3;
                    let a704 = 1u64;
                    nat_add(a703, a704)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f120(v0: Fn1, p708: List<Bytes>, p709: List<Bytes>) -> List<Bytes> {
    let m707 = (p708, p709);
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v1, v2) = m707;
                let m705 = v2;
                match m705.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c706: Fn1 = v0.clone();
                            c706.apply(v1, v3)
                        }, v4))
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

pub fn f121(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f122(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m710 = f209(Fn2::F210(v1), None::<Bytes>, v0);
    match m710 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f123() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP|check-yaml ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("check-yaml FILE")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-admit NAME read-OP FILE ARG... CITATION...")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::cons(f8(Str::lit("verify-translations ADMISSIONS")), List::<Bytes>::nil())))))))))))))))))))))))))))))))
}

pub fn f124(v0: Adt1) -> List<Bytes> {
    if f138(match v0.clone() {
        Adt1::C0(_, h711, _, _) => {
            h711
        }
    }, f211()) {
        if f138(match v0 {
            Adt1::C0(_, h712, _, _) => {
                h712
            }
        }, f212()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f212(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f211(), List::<Bytes>::nil())
    }
}

pub fn f125(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m713 = v0;
    match m713.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f125(v3, v1))
        }
    }
}

pub fn f126(v0: Adt1) -> R<List<Bytes>> {
    Ok(f56(v0.clone(), List::cons(f213(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f136(v0)?)))))
}

pub fn f127(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f214(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f142(f96(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f128(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m714 = v0;
    match m714.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f128(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f128(v3, v1))
            }
        }
    }
}

pub fn f129(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m715 = v0;
    match m715.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f129(v3, v1)
            } else {
                List::cons(v2, f129(v3, v1))
            }
        }
    }
}

pub fn f130(v0: List<Adt2>) -> List<Bytes> {
    let m716 = v0;
    match m716.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f125(f215(f188(f93(v1, f8(Str::lit("defers"))))), f130(v2))
        }
    }
}

pub fn f131(v0: List<Adt2>) -> List<Bytes> {
    let m717 = v0;
    match m717.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m718 = f93(v1, f8(Str::lit("std")));
            match m718 {
                None => {
                    f131(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("row"))), f131(v2))
                }
            }
        }
    }
}

pub fn f132(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m719 = v0;
    match m719.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f132(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f132(v3, v1))
            }
        }
    }
}

pub fn f133(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f154(f216(v2.clone()), f217(f218(v0, v1.clone(), v2)?));
    f219(f145(f220(v1))?, v3.clone(), {
        let a721 = {
            let a720 = v3;
            length_list(a720)
        };
        let a722 = 1u64;
        nat_add(a721, a722)?
    })
}

pub fn f134(v0: Adt14, v1: List<Adt2>) -> List<Adt2> {
    let m723 = v1;
    match m723.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            if if f221(v2.clone(), f8(Str::lit("subject"))) {
                f222(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f134(v0, v3))
            } else {
                f134(v0, v3)
            }
        }
    }
}

pub fn f135(v0: Adt1, v1: List<Bytes>) -> R<List<Adt2>> {
    let m724 = v1;
    match m724.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let m725 = f223(f95(f64(v0.clone(), v2.clone()))?, f85(v2, f212()))?;
            match m725 {
                None => {
                    f135(v0, v3)
                }
                Some(v4) => {
                    Ok(f224(v4, f135(v0, v3)?))
                }
            }
        }
    }
}

pub fn f136(v0: Adt1) -> R<List<Bytes>> {
    Ok(f125(f145(f225(f211(), f147(match v0.clone() {
        Adt1::C0(_, h726, _, _) => {
            h726
        }
    }, f211())))?, f145(f225(f212(), f147(match v0 {
        Adt1::C0(_, h727, _, _) => {
            h727
        }
    }, f212())))?))
}

pub fn f137(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f138(p731: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m730 = p731;
    {
        let mut m730 = m730;
        loop {
            let step = {
                let v0 = m730;
                let m728 = v0;
                match m728.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h729, _) = v2;
                            h729
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m730 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f140(v0: Adt1) -> R<List<Bytes>> {
    f145(f225(f139(), f147(match v0 {
        Adt1::C0(_, h732, _, _) => {
            h732
        }
    }, f139())))
}

pub fn f141(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m733 = v1;
    match m733.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f125(if f52(v4.clone(), v3.clone()) {
                f226(v4.clone(), f95(f64(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f227(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
            }, f141(v0, v5, v2, v3)?))
        }
    }
}

pub fn f142(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m734 = v0;
    match m734.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f142(v3, v1))
        }
    }
}

pub fn f143(v0: Adt1) -> R<Adt12> {
    let m735 = f56(v0.clone(), f228());
    match m735.uncons() {
        None => {
            let m736 = f229(v0.clone(), f228())?;
            match m736.uncons() {
                None => {
                    let m739 = f230(match v0.clone() {
                        Adt1::C0(_, h737, _, _) => {
                            h737
                        }
                    }, f125(f231(), f232(match v0.clone() {
                        Adt1::C0(_, h738, _, _) => {
                            h738
                        }
                    }, false)));
                    match m739.uncons() {
                        None => {
                            let m741 = f56(v0.clone(), f232(match v0.clone() {
                                Adt1::C0(_, h740, _, _) => {
                                    h740
                                }
                            }, true));
                            match m741.uncons() {
                                None => {
                                    let m742 = f56(v0.clone(), f233());
                                    match m742.uncons() {
                                        None => {
                                            let m743 = f234(v0.clone())?;
                                            match m743 {
                                                None => {
                                                    let m745 = f235(v0.clone(), f232(match v0.clone() {
                                                        Adt1::C0(_, h744, _, _) => {
                                                            h744
                                                        }
                                                    }, true))?;
                                                    match m745 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f236(Adt16::C0(Adt15::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f214(v0.clone())?, f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f96(v0.clone(), f8(Str::lit("model/strata.json")))?, f215(f96(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f232(match v0 {
                                                                Adt1::C0(_, h746, _, _) => {
                                                                    h746
                                                                }
                                                            }, true)))?;
                                                            let (h747, _) = v13;
                                                            if h747 {
                                                                Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h748) = v13;
                                                                    h748
                                                                }))
                                                            } else {
                                                                Ok(Adt12::C4({
                                                                    let (_, h749) = v13;
                                                                    h749
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

pub fn f144() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f145(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f237(f120(Fn1::F238, List::<Bytes>::nil(), f239(v0)?)))
}

pub fn f146(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m750 = v1;
    match m750.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f240(v0.clone(), v2), f146(v0, v3))
            } else {
                f146(v0, v3)
            }
        }
    }
}

pub fn f147(p755: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m754 = p755;
    {
        let mut m754 = m754;
        loop {
            let step = {
                let v0 = m754;
                let m751 = v0;
                match m751.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h752, _) = v2;
                            h752
                        }, v1.clone()) {
                            let (_, h753) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h753)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m754 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p758: List<Bytes>) -> R<Option<Bytes>> {
    let m757 = p758;
    {
        let mut m757 = m757;
        loop {
            let step = {
                let v2 = m757;
                let m756 = v2;
                match m756.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f241(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m757 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m759 = v0;
    match m759.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f242(v1.clone())?, v1), f149(v2)?))
        }
    }
}

pub fn f150(p762: List<Bytes>, p763: List<Bytes>, p764: List<List<Bytes>>) -> List<List<Bytes>> {
    let m761 = (p762, (p763, p764));
    {
        let mut m761 = m761;
        loop {
            let step = {
                let (v0, (v1, v2)) = m761;
                let m760 = v0;
                match m760.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f243(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f243(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m765 = v0;
    match m765.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f151(v3, v1)
            } else {
                List::cons(v2, f151(v3, v1))
            }
        }
    }
}

pub fn f152(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h766, _, _) => {
            h766
        }
    }, f244())) {
        Ok(Adt12::C1(List::cons(f244(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f146(f244(), f147(match v0.clone() {
            Adt1::C0(_, h767, _, _) => {
                h767
            }
        }, f244())))?, v1.clone());
        let m768 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m768.uncons() {
            None => {
                let v5: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m769 = f56(v0.clone(), f245(v0.clone(), v2.clone(), v5.clone())?);
                match m769.uncons() {
                    None => {
                        let m770 = f246(v0.clone(), v2, v5)?;
                        match m770 {
                            None => {
                                f247(v0, v1)
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

pub fn f153() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f154(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m771 = v0;
    match m771.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f154(v3, v1))
        }
    }
}

pub fn f155(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m772 = f96(v0.clone(), f153())?;
    match m772.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f248(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f156() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f157() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}            target/release/verdant_xml\u{a}            target/release/verdant_admit\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      # Restored on every run, not only when certified/ changed: the end-to-end propose-command step below\u{a}      # exports with the attested lexlean binary, and a fresh cargo install does not reproduce its hash.\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3-rust-1.97.0\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          [ \"$(sha256sum target/lexlean/bin/lexlean 2>/dev/null | cut -c1-64)\" = \"$(jq -r .lexlean.executable_sha256 certified/evidence/attestation.json)\" ] || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked --version 0.4.3 lexlean --root target/lexlean; }\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers certified-xml certified-admit)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/, certified-readers/, certified-xml/ or certified-admit/:\"\u{a}            git status --short -- certified certified-readers certified-xml certified-admit | head -20\u{a}            exit 1\u{a}          fi\u{a}      # verify-artifact replays a bundle but does not compare it with the sources. So each LexLean project's\u{a}      # semantic id must equal the one its bundle records. Lock-level ids move with every admission to app/,\u{a}      # which readers depends on without using it, so they are not compared.\u{a}      - name: the certified bundles were exported from these sources (semantic ids, no Lean)\u{a}        run: |\u{a}          set -euo pipefail\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          for pair in studio:certified readers:certified-readers readers/xml:certified-xml; do\u{a}            project=$(cut -d : -f 1 <<< \"$pair\")\u{a}            bundle=$(cut -d : -f 2 <<< \"$pair\")\u{a}            if [ ! -f \"$project/lexlean.toml\" ]; then continue; fi\u{a}            if [ ! -f \"$bundle/result.json\" ]; then echo \"::warning::$project has no committed bundle $bundle, so nothing records what it was exported from\"; continue; fi\u{a}            now=$(cd \"$project\" && \"$ll\" lock >/dev/null && \"$ll\" check | sed -n 's/.*semantic \\([0-9a-f]*\\)).*/\\1/p')\u{a}            git checkout -q -- \"$project/lexlean.lock\"\u{a}            was=$(jq -r .source.semantic_id \"$bundle/result.json\")\u{a}            if [ \"$now\" != \"$was\" ]; then echo \"::error::$project now has semantic id $now, but $bundle was exported from $was: export it again through propose-command\"; exit 1; fi\u{a}            echo \"$project matches $bundle (semantic $now)\"\u{a}          done\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          # Each encoding admitted through admit-translated recorded the reader command that printed it: run it again and fail if the output differs.\u{a}          target/release/verdant_readers verify-translations model/admissions.json\u{a}          if [ -x target/release/verdant_xml ]; then target/release/verdant_xml verify-translations model/admissions.json; fi\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          # certified-xml/ and certified-admit/ join the comparison once main holds them: the export that first writes it is the one that adds it.\u{a}          bundles=\"certified certified-readers\"\u{a}          if [ -n \"$(git ls-files certified-xml | head -n 1)\" ]; then bundles=\"$bundles certified-xml\"; fi\u{a}          if [ -n \"$(git ls-files certified-admit | head -n 1)\" ]; then bundles=\"$bundles certified-admit\"; fi\u{a}          if [ -n \"$(git status --porcelain -- $bundles)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- $bundles | head -20\u{a}            attested=$(git show HEAD:certified/evidence/attestation.json | jq -r .lexlean.executable_sha256); used=$(sha256sum target/lexlean/bin/lexlean | cut -c1-64)\u{a}            [ \"$attested\" = \"$used\" ] || echo \"::error::the export ran with lexlean $used, but certified/ attests lexlean $attested. lexlean is built with rust 1.97.0 and its cargo home remapped, so a different hash means the toolchain or the runner image changed\"\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f158() -> Bytes {
    f7(List::cons(f249(), List::cons(f250(), List::cons(f251(), List::cons(f252(), List::cons(f253(), List::cons(f254(), List::<Bytes>::nil())))))))
}

pub fn f159() -> Bytes {
    f7(List::cons(f255(), List::cons(f256(), List::cons(f257(), List::cons(f258(), List::<Bytes>::nil())))))
}

pub fn f160() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, readers, readers/xml and admit when they exist.\u{a}# Lean's proof of a project grows with the square of its function count, so the readers are split by family and admission is its own project.\u{a}pids=\"\"\u{a}for project in studio readers readers/xml admit; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.${project//\\//-}\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}if [ -f \"$log.readers-xml\" ]; then cat \"$log.readers-xml\" >> \"$log\"; fi\u{a}if [ -f \"$log.admit\" ]; then cat \"$log.admit\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}if [ -f readers/xml/lexlean.toml ]; then\u{a}  rm -rf certified-xml\u{a}  cp -r readers/xml/.lexlean/production/*/ certified-xml\u{a}  rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-xml\u{a}fi\u{a}if [ -f admit/lexlean.toml ]; then\u{a}  rm -rf certified-admit\u{a}  cp -r admit/.lexlean/production/*/ certified-admit\u{a}  rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-admit\u{a}fi\u{a}"))
}

pub fn f161() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env bash\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# A change to a reader can change what it prints for an encoding already admitted, and the studio check\u{a}")), List::cons(f8(Str::lit("# replays every admitted reader command (verify-translations). So the change re-derives those encodings\u{a}")), List::cons(f8(Str::lit("# itself: each Catalog encoding whose replay now differs is admitted again through read-admit, with the\u{a}")), List::cons(f8(Str::lit("# command and citation its admission row records, and the pull request carries both (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("# It prints each construct it admitted again, one per line.\u{a}")), List::cons(f8(Str::lit("set -euo pipefail\u{a}")), List::cons(f8(Str::lit("readers=$1\u{a}")), List::cons(f8(Str::lit("mkdir -p target\u{a}")), List::cons(f8(Str::lit("rows=$(jq length model/admissions.json)\u{a}")), List::cons(f8(Str::lit("for try in $(seq 1 \"$rows\"); do\u{a}")), List::cons(f8(Str::lit("  if \"$readers\" verify-translations model/admissions.json > target/replay.txt 2>&1; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("  construct=$(sed -n 's/^\\([^:]*\\): its reader command now prints .*/\\1/p' target/replay.txt | head -n 1)\u{a}")), List::cons(f8(Str::lit("  case \"$construct\" in\u{a}")), List::cons(f8(Str::lit("    Catalog.*) ;;\u{a}")), List::cons(f8(Str::lit("    *) cat target/replay.txt >&2; echo \"readmit: only a Catalog encoding whose reader now prints other output is admitted again\" >&2; exit 1;;\u{a}")), List::cons(f8(Str::lit("  esac\u{a}")), List::cons(f8(Str::lit("  row=$(jq -c --arg construct \"$construct\" '[.[] | select(.construct == $construct and .translation != null)][0]' model/admissions.json)\u{a}")), List::cons(f8(Str::lit("  authority=$(jq -r '.std.authority // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  cited=$(jq -r '.std.row // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  if [ -z \"$authority\" ]; then echo \"readmit: $construct cites no std authority, so read-admit cannot cite it again\" >&2; exit 1; fi\u{a}")), List::cons(f8(Str::lit("  mapfile -t command < <(jq -r '.translation.command[]' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  \"$readers\" read-admit \"${construct#Catalog.}\" \"${command[@]}\" std \"$authority\" $cited >/dev/null\u{a}")), List::cons(f8(Str::lit("  echo \"$construct\"\u{a}")), List::cons(f8(Str::lit("done\u{a}")), List::cons(f8(Str::lit("echo \"readmit: the replay did not settle after $rows admissions\" >&2\u{a}")), List::cons(f8(Str::lit("exit 1\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))
}

pub fn f162() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f163(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f127(v0.clone())?;
    let v2: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt2> = f134(v1.clone(), f135(v0.clone(), f136(v0.clone())?)?);
    let v4: List<List<Bytes>> = f259(f260(f64(v0, f213()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f261(v4.clone(), f262(v4), f218(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f133(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f164(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f263(f127(v0.clone())?, f135(v0.clone(), f136(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f165(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f264(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f265(f96(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f166() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f167(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m773 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m773.uncons() {
        None => {
            if f266(f95(f64(v0, v1))?)? {
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

pub fn f168(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m774 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m774.uncons() {
        None => {
            let v5: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f267(v5)?
                } else {
                    f268(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f240(f269(), f270(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f240(f269(), f270(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f271()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f169(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m775 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m775.uncons() {
        None => {
            let m776 = f272(f95(f64(v0.clone(), v1.clone()))?)?;
            match m776 {
                None => {
                    Ok(Adt12::C4(f273()))
                }
                Some(v4) => {
                    let m777 = f274(f95(f64(v0, v1))?)?;
                    match m777 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f275(v5)))
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

pub fn f170(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let v2: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f117(f276(f277(v2.clone())?), f51({
            let a779 = {
                let a778 = f278(f277(v2)?);
                length_list(a778)
            };
            let a780 = 0u64;
            nat_eq(a779, a780)
        })) {
            Ok(Adt12::C3(List::cons((f240(f269(), f270(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f240(f269(), f270(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f171(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m781 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m781.uncons() {
        None => {
            let m782 = f279(f95(f64(v0, v1))?)?;
            match m782 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f172(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m783 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m783.uncons() {
        None => {
            let v5: Bytes = f240(if v2 {
                f212()
            } else {
                f211()
            }, f270(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m784 = f223(f95(f64(v0.clone(), v1.clone()))?, v2)?;
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

pub fn f173(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m788 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m788.uncons() {
        None => {
            let v4: List<Bytes> = f95(f64(v0.clone(), v1))?;
            let m789 = f280(v4.clone(), f8(Str::lit("requirement")))?;
            match m789 {
                None => {
                    Ok(Adt12::C4(f271()))
                }
                Some(v5) => {
                    let m790 = f280(v4, f8(Str::lit("admission")))?;
                    match m790 {
                        None => {
                            Ok(Adt12::C4(f271()))
                        }
                        Some(v6) => {
                            let v7: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f125(f142(v5.clone(), f8(Str::lit("id"))), f142(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f145(f125(f142(v7.clone(), f8(Str::lit("id"))), f142(v8.clone(), f8(Str::lit("construct")))))?;
                            let m791 = f281(f131(v6.clone()), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m791 {
                                None => {
                                    let a793 = {
                                        let a792 = v9.clone();
                                        length_list(a792)
                                    };
                                    let a794 = 0u64;
                                    if nat_eq(a793, a794) {
                                        Ok(Adt12::C4(f271()))
                                    } else {
                                        if f51({
                                            let a797 = {
                                                let a795 = f145(f125(v9.clone(), v10.clone()))?;
                                                length_list(a795)
                                            };
                                            let a798 = {
                                                let a796 = f125(v9.clone(), v10);
                                                length_list(a796)
                                            };
                                            nat_eq(a797, a798)
                                        }) {
                                            Ok(Adt12::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt12::C3(List::cons(f283(f8(Str::lit("model/requirements.json")), f224(v7, v5))?, List::cons(f283(f8(Str::lit("model/admissions.json")), f224(v8, v6))?, List::cons(f284(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f122(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f282(v11))
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

pub fn f174(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m799 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m799.uncons() {
        None => {
            let m800 = f91(v0.clone(), v1.clone())?;
            match m800 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m801 = f93(v4.clone(), f8(Str::lit("schema")));
                    match m801 {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f285(v0, f100(v5.clone())?, v5, f58(v4.clone(), f8(Str::lit("name"))), f215(f188(f93(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f175(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m802 = v2;
    match m802.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m803 = v4;
            match m803.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m804 = v6.clone();
                    match m804.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((v7, v8)) => {
                            let m805 = v8;
                            match m805.uncons() {
                                None => {
                                    Ok(Adt12::C4(f79()))
                                }
                                Some((v9, _)) => {
                                    if f51(f286(v7, v5.clone())) {
                                        Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that reads the artifact, a read-OP, or check-yaml when the construct is none: admit-registry AUTHORITY ROW CONSTRUCT|none read-OP|check-yaml ARTIFACT\u{a}"))))
                                    } else {
                                        let m806 = f56(v0.clone(), List::cons(f287(v1.clone()), List::cons(v9.clone(), List::cons(f8(Str::lit("model/admissions.json")), if f32(v5.clone(), f8(Str::lit("none"))) {
                                            List::<Bytes>::nil()
                                        } else {
                                            List::cons(f7(List::cons(f8(Str::lit("encodings/")), List::cons(v5.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), List::<Bytes>::nil())
                                        }))));
                                        match m806.uncons() {
                                            None => {
                                                if f51(f67(v0.clone(), f287(v1.clone()))) {
                                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v1.clone(), List::cons(f8(Str::lit(" is not a bound authority, because ")), List::cons(f287(v1), List::cons(f8(Str::lit(" is absent, and every admitted value cites a bound standard (REQ-ADMISSION)\u{a}")), List::<Bytes>::nil()))))))))
                                                } else {
                                                    let m807 = f57(v0.clone(), f287(v1.clone()))?;
                                                    match m807 {
                                                        None => {
                                                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(f287(v1), List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                        Some(v13) => {
                                                            let v15: Bytes = {
                                                                let m808 = f93(v13, f8(Str::lit("source")));
                                                                match m808 {
                                                                    None => {
                                                                        Bytes::lit(&[])
                                                                    }
                                                                    Some(v14) => {
                                                                        f58(v14, f8(Str::lit("sha256")))
                                                                    }
                                                                }
                                                            };
                                                            let v16: Bytes = f288(f64(v0.clone(), v9.clone()))?;
                                                            if f51(f32(v16.clone(), v15.clone())) {
                                                                Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v9, List::cons(f8(Str::lit(" is not the artifact ")), List::cons(v1, List::cons(f8(Str::lit(" binds: its SHA-256 is ")), List::cons(v16, List::cons(f8(Str::lit(" and the binding's source.sha256 is ")), List::cons(v15, List::cons(f8(Str::lit(" (FIPS 180-4 section 6.2)\u{a}")), List::<Bytes>::nil()))))))))))))
                                                            } else {
                                                                f289(v0, v1, v3, v5, v6, v9)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Some((v11, v12)) => {
                                                Ok(Adt12::C0(List::cons(v11, v12)))
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

pub fn f176(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m809 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m809.uncons() {
        None => {
            let m810 = f57(v0.clone(), v1.clone())?;
            match m810 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f290())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt2 = {
                            let m811 = f93(v4.clone(), f8(Str::lit("source")));
                            match m811 {
                                None => {
                                    Adt2::C9
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f92(f93(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f240(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f240(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m812 = v8;
                        match m812 {
                            None => {
                                f291(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f291(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f240(v9, f270(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h813, _) => {
                                            h813
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m814 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m814.uncons() {
                                            None => {
                                                if f32(f288(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f291(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f177(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m815 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m815.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f260(v4.clone())?;
            if f51(f98(f292(v5.clone()), f293())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f294(Str::lit("id"), v6.clone())?, List::cons(f294(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f294(Str::lit("artifact"), f213())?, List::cons(f294(Str::lit("checksum"), f288(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f213(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f295(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f284(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f122(f262(f259(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f178(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m816 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m816.uncons() {
        None => {
            let m817 = f57(v0, v1)?;
            match m817 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f296(f188(Some(v4))) {
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

pub fn f179(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m818 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m818.uncons() {
        None => {
            let m819 = f91(v0.clone(), v1.clone())?;
            match m819 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f285(v0.clone(), f64(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f180(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m820 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil())))))));
    match m820.uncons() {
        None => {
            let m821 = f297(v3.clone());
            match m821.uncons() {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                }
                Some((v6, v7)) => {
                    if f85(v6.clone(), f8(Str::lit("read-"))) {
                        f298(v0.clone(), f64(v0, v1), v2, List::cons(v6, v7), f299(v3))
                    } else {
                        Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                    }
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f181(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m822 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f153(), List::<Bytes>::nil())));
    match m822.uncons() {
        None => {
            let m823 = f300(f95(f64(v0.clone(), v1))?)?;
            match m823 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt2> = f96(v0, f153())?;
                    let v6: List<Bytes> = f142(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f145(f142(v5.clone(), f8(Str::lit("at"))))?;
                    let a825 = {
                        let a824 = v6.clone();
                        length_list(a824)
                    };
                    let a826 = 0u64;
                    if if nat_eq(a825, a826) {
                        true
                    } else {
                        f51({
                            let a829 = {
                                let a827 = f145(f125(v6.clone(), v7.clone()))?;
                                length_list(a827)
                            };
                            let a830 = {
                                let a828 = f125(v6.clone(), v7);
                                length_list(a828)
                            };
                            nat_eq(a829, a830)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f283(f153(), f224(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f122(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f182(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m831 = v2;
    match m831.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m832 = v4;
            match m832.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m833 = v6.clone();
                    match m833.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((_, _)) => {
                            let m834 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f301(), List::<Bytes>::nil())));
                            match m834.uncons() {
                                None => {
                                    let v11: Bytes = f122(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt2> = f96(v0, f301())?;
                                    if f302(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f283(f301(), f224(v13, List::cons(f303(v11.clone(), v12, v5, v3), List::<Adt2>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f183(v0: Bytes, p888: u64, p889: u64) -> R<bool> {
    let m887 = (p888, p889);
    {
        let mut m887 = m887;
        loop {
            let step = {
                let (v1, v2) = m887;
                let m835 = v2;
                if m835 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m835 - 1;
                    let m838 = {
                        let a836 = v0.clone();
                        let a837 = v1;
                        index_bytes(a836, a837)
                    };
                    match m838 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f304({
                                let v5: u8 = v4;
                                let a883 = {
                                    let a859 = {
                                        let a847 = {
                                            let a841 = {
                                                let a839 = v5;
                                                let a840 = 128u8;
                                                fixed_u8::bit_and(a839, a840)
                                            };
                                            let a842 = 128u8;
                                            if equal(a841, a842) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a848 = {
                                            let a845 = {
                                                let a843 = v5;
                                                let a844 = 64u8;
                                                fixed_u8::bit_and(a843, a844)
                                            };
                                            let a846 = 64u8;
                                            if equal(a845, a846) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a847, a848)?
                                    };
                                    let a860 = {
                                        let a857 = {
                                            let a851 = {
                                                let a849 = v5;
                                                let a850 = 32u8;
                                                fixed_u8::bit_and(a849, a850)
                                            };
                                            let a852 = 32u8;
                                            if equal(a851, a852) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a858 = {
                                            let a855 = {
                                                let a853 = v5;
                                                let a854 = 16u8;
                                                fixed_u8::bit_and(a853, a854)
                                            };
                                            let a856 = 16u8;
                                            if equal(a855, a856) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a857, a858)?
                                    };
                                    nat_add(a859, a860)?
                                };
                                let a884 = {
                                    let a881 = {
                                        let a869 = {
                                            let a863 = {
                                                let a861 = v5;
                                                let a862 = 8u8;
                                                fixed_u8::bit_and(a861, a862)
                                            };
                                            let a864 = 8u8;
                                            if equal(a863, a864) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a870 = {
                                            let a867 = {
                                                let a865 = v5;
                                                let a866 = 4u8;
                                                fixed_u8::bit_and(a865, a866)
                                            };
                                            let a868 = 4u8;
                                            if equal(a867, a868) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a869, a870)?
                                    };
                                    let a882 = {
                                        let a879 = {
                                            let a873 = {
                                                let a871 = v5;
                                                let a872 = 2u8;
                                                fixed_u8::bit_and(a871, a872)
                                            };
                                            let a874 = 2u8;
                                            if equal(a873, a874) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a880 = {
                                            let a877 = {
                                                let a875 = v5;
                                                let a876 = 1u8;
                                                fixed_u8::bit_and(a875, a876)
                                            };
                                            let a878 = 1u8;
                                            if equal(a877, a878) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a879, a880)?
                                    };
                                    nat_add(a881, a882)?
                                };
                                nat_add(a883, a884)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a885 = v1;
                                    let a886 = 1u64;
                                    nat_add(a885, a886)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(p894: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m893 = p894;
    {
        let mut m893 = m893;
        loop {
            let step = {
                let v0 = m893;
                let m890 = v0;
                match m890.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h891, _) = v2;
                            h891
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h892) = v2;
                                h892
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
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

pub fn f185(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m898 = {
        let a896 = match v0 {
            Adt1::C0(_, _, _, h895) => {
                h895
            }
        };
        let a897 = f40(v1);
        index_list(a896, a897)
    };
    match m898 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f186(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m899 = f44(v0, 34u8)?;
        match m899.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m900 = v3;
                match m900.uncons() {
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

pub fn f187(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f305(v0, v1)
}

pub fn f188(v0: Option<Adt2>) -> List<Adt2> {
    let m901 = v0;
    match m901 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f306(v1)
        }
    }
}

pub fn f189(v0: Adt2, v1: Bytes) -> Adt2 {
    let m902 = v0;
    match m902 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r903) => {
            let v12 = (*r903).clone();
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
        Adt2::C7(r904) => {
            let v10 = (*r904).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r905) => {
            let v5 = (*r905).clone();
            Adt2::C8(std::rc::Rc::new(f189(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r906, r907) => {
            let v18 = (*r906).clone();
            let v19 = (*r907).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r908, r909) => {
            let v3 = (*r908).clone();
            let v4 = (*r909).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt2::C11(v2, std::rc::Rc::new(f307(v3, v1.clone())), std::rc::Rc::new(f189(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f189(v4, v1)))
            }
        }
    }
}

pub fn f190(v0: Adt2) -> R<Bytes> {
    let m910 = v0;
    match m910 {
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
            f308(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f309(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f310(v9, v10)
        }
        Adt2::C6(v11) => {
            f311(v11)
        }
        Adt2::C7(r911) => {
            let v12 = (*r911).clone();
            Ok(f312(f313(v12)?))
        }
        Adt2::C8(r912) => {
            let v13 = (*r912).clone();
            f314(f315(v13)?)
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

pub fn f191(v0: Adt1, p916: List<Adt13>) -> R<Option<Adt13>> {
    let m915 = p916;
    {
        let mut m915 = m915;
        loop {
            let step = {
                let v1 = m915;
                let m913 = v1;
                match m913.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f192(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h914, _) => {
                                h914
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
                Ok(next) => { m915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Adt13) -> R<Bytes> {
    Ok(f240(f316(), f270(f317(v0))?))
}

pub fn f193(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f317(v0), List::<Bytes>::nil())))))
}

pub fn f194() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f195() -> Bytes {
    f318()
}

pub fn f196(v0: List<Adt13>) -> R<List<Bytes>> {
    let m917 = v0;
    match m917.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f192(v1)?, f196(v2)?))
        }
    }
}

pub fn f197(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m918 = v1;
    match m918.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f319(v5.clone(), v2.clone(), v3.clone(), f288(f64(v0.clone(), f192(v5)?))?, v4.clone())?), f197(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f198(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f320(v0.clone())?;
    let v2: u64 = f321(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a919 = v1;
        let a920 = v2;
        nat_sub(a919, a920)
    }))
}

pub fn f199(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a923 = v3;
    let a924 = f46(v0, v1, {
        let a921 = v2;
        let a922 = v1;
        nat_sub(a921, a922)
    });
    append_bytes(a923, a924)
}

pub fn f200(v0: Bytes, p937: u64, p938: u64) -> R<u64> {
    let m936 = (p937, p938);
    {
        let mut m936 = m936;
        loop {
            let step = {
                let (v1, v2) = m936;
                let m925 = v2;
                if m925 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m925 - 1;
                    let v4: u64 = f115(v0.clone(), v1)?;
                    let a926 = v4;
                    let a927 = 34u64;
                    if nat_eq(a926, a927) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a928 = v4;
                        let a929 = 92u64;
                        if nat_eq(a928, a929) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a930 = v4;
                            let a931 = 32u64;
                            if nat_lt(a930, a931) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a932 = v4;
                                let a933 = 256u64;
                                if nat_eq(a932, a933) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a934 = v1;
                                        let a935 = 1u64;
                                        nat_add(a934, a935)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: Bytes, p964: u64, p965: u64, p966: Bytes) -> R<Option<(u64, Bytes)>> {
    let m963 = (p964, (p965, p966));
    {
        let mut m963 = m963;
        loop {
            let step = {
                let (v1, (v2, v3)) = m963;
                let m939 = v2;
                if m939 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m939 - 1;
                    let v5: u64 = f200(v0.clone(), v1, {
                        let a943 = {
                            let a941 = {
                                let a940 = v0.clone();
                                length_bytes(a940)
                            };
                            let a942 = v1;
                            nat_sub(a941, a942)
                        };
                        let a944 = 1u64;
                        nat_add(a943, a944)?
                    })?;
                    let v6: Bytes = {
                        let a947 = v3;
                        let a948 = f46(v0.clone(), v1, {
                            let a945 = v5;
                            let a946 = v1;
                            nat_sub(a945, a946)
                        });
                        append_bytes(a947, a948)
                    };
                    let v7: u64 = f115(v0.clone(), v5)?;
                    let a949 = v7;
                    let a950 = 34u64;
                    if nat_eq(a949, a950) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a951 = v5;
                            let a952 = 1u64;
                            nat_add(a951, a952)?
                        }, v6))))
                    } else {
                        let a953 = v7;
                        let a954 = 92u64;
                        if nat_eq(a953, a954) {
                            let m955 = f322(v0.clone(), v5, v6)?;
                            match m955 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a958 = {
                                        let (_, h956) = {
                                            let (_, h957) = v8.clone();
                                            h957
                                        };
                                        h956
                                    };
                                    let a959 = 0u64;
                                    if nat_eq(a958, a959) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h960, _) = v8;
                                            h960
                                        }, (v4, {
                                            let (h961, _) = {
                                                let (_, h962) = v8;
                                                h962
                                            };
                                            h961
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
                Ok(next) => { m963 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Bytes, v1: u64) -> R<u64> {
    let (h967, _) = f323(Fn3::F324(v0.clone()), {
        let a969 = {
            let a968 = v0;
            length_bytes(a968)
        };
        let a970 = 1u64;
        nat_add(a969, a970)?
    }, v1)?;
    Ok(h967)
}

pub fn f203(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f204(v0: Bytes) -> R<u64> {
    let (h971, _) = f323(Fn3::F325(v0.clone()), {
        let a973 = {
            let a972 = v0;
            length_bytes(a972)
        };
        let a974 = 1u64;
        nat_add(a973, a974)?
    }, 0u64)?;
    Ok(h971)
}

pub fn f205(v0: Bytes) -> R<Option<u64>> {
    if f326(v0.clone())? {
        Ok(Some({
            let (_, h975) = {
                let (h976, _) = f327(Fn4::F328(v0.clone()), {
                    let a977 = v0;
                    length_bytes(a977)
                }, (0u64, 0u64))?;
                h976
            };
            h975
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f206(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a979 = {
            let a978 = v1.clone();
            length_bytes(a978)
        };
        let a980 = f329(v1)?;
        nat_sub(a979, a980)
    });
    let v5: Bytes = {
        let a981 = v0;
        let a982 = v4.clone();
        append_bytes(a981, a982)
    };
    let v6: u64 = f204(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a984 = {
            let a983 = v5;
            length_bytes(a983)
        };
        let a985 = v6;
        nat_sub(a984, a985)
    });
    let v8: u64 = {
        let a986 = v4;
        length_bytes(a986)
    };
    let v9: u64 = {
        let a987 = v7.clone();
        length_bytes(a987)
    };
    let a988 = v9;
    let a989 = 0u64;
    if nat_eq(a988, a989) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f203(v2, {
            let a990 = v3;
            let a991 = v8;
            nat_lt(a990, a991)
        }) {
            let v10: u64 = if v2 {
                let a992 = v3;
                let a993 = v8;
                nat_add(a992, a993)?
            } else {
                let a994 = v8;
                let a995 = v3;
                nat_sub(a994, a995)
            };
            let v11: u64 = f329(v7.clone())?;
            let a996 = v10;
            let a997 = v11;
            if nat_le(a996, a997) {
                Ok(Some((f46(v7, 0u64, {
                    let a998 = v9;
                    let a999 = v10;
                    nat_sub(a998, a999)
                }), 0u64)))
            } else {
                let a1002 = {
                    let a1000 = v10;
                    let a1001 = v11;
                    nat_sub(a1000, a1001)
                };
                let a1003 = 19u64;
                if nat_le(a1002, a1003) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1004 = v9;
                        let a1005 = v11;
                        nat_sub(a1004, a1005)
                    }), {
                        let a1006 = v10;
                        let a1007 = v11;
                        nat_sub(a1006, a1007)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1008 = v3;
                let a1009 = v8;
                nat_sub(a1008, a1009)
            };
            let a1012 = {
                let a1010 = v9;
                let a1011 = v12;
                nat_add(a1010, a1011)?
            };
            let a1013 = 19u64;
            if nat_le(a1012, a1013) {
                Ok(Some(({
                    let a1014 = v7;
                    let a1015 = f330(v12);
                    append_bytes(a1014, a1015)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f207(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1016 = 0u64;
        let a1017 = v1;
        if nat_lt(a1016, a1017) {
            let a1018 = v1;
            let a1019 = 9223372036854775808u64;
            nat_le(a1018, a1019)
        } else {
            false
        }
    } else {
        let a1020 = v1;
        let a1021 = 9223372036854775807u64;
        nat_le(a1020, a1021)
    }
}

pub fn f208(v0: Bytes, v1: u64) -> Option<u8> {
    let a1022 = v0;
    let a1023 = v1;
    index_bytes(a1022, a1023)
}

pub fn f209(v0: Fn2, p1027: Option<Bytes>, p1028: List<Bytes>) -> Option<Bytes> {
    let m1026 = (p1027, p1028);
    {
        let mut m1026 = m1026;
        loop {
            let step = {
                let (v1, v2) = m1026;
                let m1024 = v2;
                match m1024.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1025: Fn2 = v0.clone();
                            c1025.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f210(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1029 = v1;
    match m1029 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1032 = {
                    let a1030 = v3;
                    let a1031 = v0;
                    append_bytes(a1030, a1031)
                };
                let a1033 = v2;
                append_bytes(a1032, a1033)
            })
        }
    }
}

pub fn f211() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f212() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f213() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f214(v0: Adt1) -> R<List<Bytes>> {
    let m1034 = f260(f64(v0, f213()))?;
    match m1034.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f262(v2))
        }
    }
}

pub fn f215(v0: List<Adt2>) -> List<Bytes> {
    let m1035 = v0;
    match m1035.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1036 = f92(Some(v1));
            match m1036 {
                None => {
                    f215(v2)
                }
                Some(v3) => {
                    List::cons(v3, f215(v2))
                }
            }
        }
    }
}

pub fn f216(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1037 = v0;
    match m1037.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f216(v2))
        }
    }
}

pub fn f217(v0: List<Adt17>) -> List<(Bytes, Bytes)> {
    let m1038 = v0;
    match m1038.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt17::C0(h1039, _, _) => {
                    h1039
                }
            }, match v1 {
                Adt17::C0(_, _, h1040) => {
                    h1040
                }
            }), f217(v2))
        }
    }
}

pub fn f218(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Adt17>> {
    let v3: List<(Bytes, Bytes)> = f331(v1);
    let v4: List<(Bytes, Bytes)> = f332(v0, v2);
    f333(v3.clone(), f154(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f219(p1047: List<Bytes>, v1: List<(Bytes, Bytes)>, p1048: u64) -> R<List<Bytes>> {
    let m1046 = (p1047, p1048);
    {
        let mut m1046 = m1046;
        loop {
            let step = {
                let (v0, v2) = m1046;
                let m1041 = v2;
                if m1041 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1041 - 1;
                    let v4: List<Bytes> = f145(f125(v0.clone(), f334(v0.clone(), v1.clone())))?;
                    let a1044 = {
                        let a1042 = v4.clone();
                        length_list(a1042)
                    };
                    let a1045 = {
                        let a1043 = v0.clone();
                        length_list(a1043)
                    };
                    if nat_eq(a1044, a1045) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
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

pub fn f220(v0: List<Adt2>) -> List<Bytes> {
    let m1049 = v0;
    match m1049.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1050 = f93(v1, f8(Str::lit("std")));
            match m1050 {
                None => {
                    f220(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f221(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f220(v2))
                    } else {
                        f220(v2)
                    })
                }
            }
        }
    }
}

pub fn f221(v0: Adt2, v1: Bytes) -> bool {
    f51({
        let a1052 = {
            let a1051 = f58(v0, v1);
            length_bytes(a1051)
        };
        let a1053 = 0u64;
        nat_eq(a1052, a1053)
    })
}

pub fn f222(v0: Adt14, v1: Adt2) -> bool {
    if f335(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f335(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f223(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt2>>> {
    let m1054 = v0;
    match m1054.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f198(v2)?;
            let a1056 = {
                let a1055 = v4.clone();
                length_bytes(a1055)
            };
            let a1057 = 0u64;
            if nat_eq(a1056, a1057) {
                f223(v3, v1)
            } else {
                let m1058 = f43(v4)?;
                match m1058 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f336(v5.clone())
                        } else {
                            f337(v5.clone())
                        } {
                            let m1059 = f223(v3, v1)?;
                            match m1059 {
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

pub fn f224(v0: List<Adt2>, v1: List<Adt2>) -> List<Adt2> {
    let m1060 = v0;
    match m1060.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f224(v3, v1))
        }
    }
}

pub fn f225(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1061 = v1;
    match m1061.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f240(v0.clone(), v2), f225(v0, v3))
            } else {
                f225(v0, v3)
            }
        }
    }
}

pub fn f226(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1062 = v1;
    match m1062.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f125({
                let a1064 = {
                    let a1063 = f198(v4.clone())?;
                    length_bytes(a1063)
                };
                let a1065 = 0u64;
                if nat_eq(a1064, a1065) {
                    List::<Bytes>::nil()
                } else {
                    f338(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f226(v0, v5, v2, v3)?))
        }
    }
}

pub fn f227(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f228() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f213(), List::<Bytes>::nil())))))))
}

pub fn f229(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1066 = v1;
    match m1066.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m1067 = f57(v0.clone(), v2.clone())?;
                match m1067 {
                    None => {
                        Ok(List::cons(v2, f229(v0, v3)?))
                    }
                    Some(_) => {
                        f229(v0, v3)
                    }
                }
            } else {
                f229(v0, v3)
            }
        }
    }
}

pub fn f230(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m1068 = v1;
    match m1068.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f138(v0.clone(), v2.clone()) {
                f230(v0, v3)
            } else {
                List::cons(v2, f230(v0, v3))
            }
        }
    }
}

pub fn f231() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f232(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m1069 = v0;
    match m1069.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f125(f339({
                let (h1070, _) = v2;
                h1070
            }, {
                let (_, h1071) = v2;
                h1071
            }, v1), f232(v3, v1))
        }
    }
}

pub fn f233() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::cons(f8(Str::lit(".github/scripts/readmit.sh")), List::<Bytes>::nil())))))
}

pub fn f234(v0: Adt1) -> R<Option<Bytes>> {
    f340(f341(v0, f233())?)
}

pub fn f235(v0: Adt1, p1075: List<Bytes>) -> R<Option<Bytes>> {
    let m1074 = p1075;
    {
        let mut m1074 = m1074;
        loop {
            let step = {
                let v1 = m1074;
                let m1072 = v1;
                match m1072.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1073 = f342(f64(v0.clone(), v2.clone()), f343())?;
                        match m1073 {
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
                Ok(next) => { m1074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h1076, _, _, _, _) => {
            h1076
        }
    };
    let v2: Adt18 = f344(v1.clone())?;
    let v3: List<Bytes> = f345(v1.clone());
    let v4: List<Bytes> = f142(match v0.clone() {
        Adt16::C0(_, _, h1077, _, _) => {
            h1077
        }
    }, f8(Str::lit("path")));
    let m1079 = f346(v2, List::<u64>::nil(), f347(v1.clone(), match v0.clone() {
        Adt16::C0(_, h1078, _, _, _) => {
            h1078
        }
    }, 0u64)?);
    match m1079 {
        None => {
            if f349(f350(match v0.clone() {
                Adt16::C0(_, _, _, _, h1081) => {
                    h1081
                }
            }, f125(match v0.clone() {
                Adt16::C0(_, _, _, _, h1082) => {
                    h1082
                }
            }, v4.clone()))?, f350(v4.clone(), f125(match v0.clone() {
                Adt16::C0(_, _, _, _, h1083) => {
                    h1083
                }
            }, v4.clone()))?, f351(match v0.clone() {
                Adt16::C0(_, _, h1084, _, _) => {
                    h1084
                }
            })) {
                let m1088 = f352(f350(match v0.clone() {
                    Adt16::C0(_, _, _, h1085, _) => {
                        h1085
                    }
                }, f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h1086, _) => {
                        h1086
                    }
                }))?, f350(v3.clone(), f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h1087, _) => {
                        h1087
                    }
                }))?);
                match m1088 {
                    None => {
                        let m1091 = f353(f142(match v0 {
                            Adt16::C0(_, h1090, _, _, _) => {
                                h1090
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1091 {
                            None => {
                                let m1092 = f353(v4, List::<Bytes>::nil());
                                match m1092 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f348(f125(v3, match v0 {
                            Adt16::C0(_, _, _, h1089, _) => {
                                h1089
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f348(match v1 {
                Adt15::C0(_, _, _, _, h1080) => {
                    h1080
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f237(v0: List<Bytes>) -> List<Bytes> {
    f120(Fn1::F354, List::<Bytes>::nil(), v0)
}

pub fn f238(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f355(v0, v1)
}

pub fn f239(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f356(f357(f358(v0))?))
}

pub fn f240(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f241(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1093 = f57(v0, v2)?;
    match m1093 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f359(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1094 = v4.clone();
            match m1094.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1096 = f92(f93({
                        let m1095 = f93(v3.clone(), f8(Str::lit("source")));
                        match m1095 {
                            None => {
                                Adt2::C9
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1096 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f360(f361(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f242(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1097 = v0;
    match m1097.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f362(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1101 = f44(f46(v1.clone(), 6u64, {
                    let a1099 = {
                        let a1098 = v1;
                        length_bytes(a1098)
                    };
                    let a1100 = 6u64;
                    nat_sub(a1099, a1100)
                }), 34u8)?;
                match m1101.uncons() {
                    None => {
                        f242(v2)
                    }
                    Some((v3, v4)) => {
                        let m1102 = v4;
                        match m1102.uncons() {
                            None => {
                                f242(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f242(v2)?))
                            }
                        }
                    }
                }
            } else {
                f242(v2)
            }
        }
    }
}

pub fn f243(p1105: List<Bytes>, p1106: List<Bytes>) -> List<Bytes> {
    let m1104 = (p1105, p1106);
    {
        let mut m1104 = m1104;
        loop {
            let step = {
                let (v0, v1) = m1104;
                let m1103 = v0;
                match m1103.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f245(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1107 = v1;
    match m1107.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f363(v0.clone(), v3)?;
            if f52(f364(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f365(v5)?, f245(v0, v4, v2)?))
            } else {
                f245(v0, v4, v2)
            }
        }
    }
}

pub fn f246(v0: Adt1, p1111: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1110 = p1111;
    {
        let mut m1110 = m1110;
        loop {
            let step = {
                let v1 = m1110;
                let m1108 = v1;
                match m1108.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f363(v0.clone(), v3.clone())?;
                        if if f52(f364(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f100({
                                let m1109 = f93(v5.clone(), f8(Str::lit("schema")));
                                match m1109 {
                                    None => {
                                        Adt2::C9
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f365(v5)?))
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
                Ok(next) => { m1110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f247(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h1112, _, _) => {
            h1112
        }
    }, f269())) {
        Ok(Adt12::C1(List::cons(f269(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f366(f147(match v0.clone() {
            Adt1::C0(_, h1113, _, _) => {
                h1113
            }
        }, f269())))?, v1.clone());
        let m1114 = f56(v0.clone(), List::cons(f153(), List::cons(f213(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1114.uncons() {
            None => {
                let m1115 = f56(v0.clone(), f367(v0.clone(), v2.clone())?);
                match m1115.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f368(v0.clone(), v2, v1)?;
                        let (h1116, _) = v7;
                        if h1116 {
                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h1117) = v7;
                                h1117
                            }))
                        } else {
                            let m1118 = f124(v0.clone());
                            match m1118.uncons() {
                                None => {
                                    let m1119 = f126(v0.clone())?;
                                    match m1119.uncons() {
                                        None => {
                                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h1120) = v7;
                                                h1120
                                            }, List::cons(f7(f145(f369(f127(v0.clone())?, f135(v0.clone(), f136(v0.clone())?)?))?), List::cons(f370(v0)?, List::<Bytes>::nil()))))))
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

pub fn f248(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f371(f96(v0, f153())?)?)))
}

pub fn f249() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The one way an agent changes this repository: dispatch this workflow with an\u{a}")), List::cons(f8(Str::lit("# operation of the API. It runs the certified command, and opens a pull request\u{a}")), List::cons(f8(Str::lit("# as the Verdant Studio GitHub App, which merges it once the studio check passes.\u{a}")), List::cons(f8(Str::lit("name: operate\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}")), List::cons(f8(Str::lit("# that defers to an open advance pull request always has a successor when that merges.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("    inputs:\u{a}")), List::cons(f8(Str::lit("      args:\u{a}")), List::cons(f8(Str::lit("        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}")), List::cons(f8(Str::lit("        required: true\u{a}")), List::cons(f8(Str::lit("        default: 'advance'\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input:\u{a}")), List::cons(f8(Str::lit("        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input_name:\u{a}")), List::cons(f8(Str::lit("        description: 'File name for input'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("        default: 'input'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Only operations that write the registers share one group. GitHub keeps one\u{a}")), List::cons(f8(Str::lit("# pending run per group, so a staging or propose-command run gets its own.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  operate:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 30\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      # Main moves about every 10 minutes, and refresh's regeneration of an API change plus its check takes\u{a}")), List::cons(f8(Str::lit("      # about 12, so an operation merged in that window pushes the change behind main again (#309, #315).\u{a}")), List::cons(f8(Str::lit("      # refresh keeps a regenerated change ready with its auto-merge set, so this run waits for every such\u{a}")), List::cons(f8(Str::lit("      # change whose studio check has not failed to merge first, then starts from the new main. It waits at\u{a}")), List::cons(f8(Str::lit("      # most 10 minutes, so neither queue stalls. A new propose-command waits too, so it never merges ahead of an\u{a}")), List::cons(f8(Str::lit("      # older change that is being regenerated; revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("      - name: wait for the API changes that are next to merge (every operation but revert-merge)\u{a}")), List::cons(f8(Str::lit("        if: ${{ !startsWith(inputs.args, 'revert-merge ') }}\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          for try in $(seq 1 20); do\u{a}")), List::cons(f8(Str::lit("            next=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio propose-command\")) and .auto_merge != null and (.draft | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then next=\"$next #${pr}\"; fi\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f250() -> Bytes {
    f7(List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$next\" ]; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("            echo \"waiting for${next} to merge (check $try of 20)\"\u{a}")), List::cons(f8(Str::lit("            sleep 30\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          echo \"::warning::${next# } still open after 10 minutes, so this run goes on\"\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-0.4.3-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}")), List::cons(f8(Str::lit("        if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          mkdir -p target/input\u{a}")), List::cons(f8(Str::lit("          cat > target/input/change.patch <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}")), List::cons(f8(Str::lit("          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}")), List::cons(f8(Str::lit("          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^admit/' -e '^lexlean-workspace.toml$' || true)\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers, admit and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          # readers/lexlean.lock, readers/xml/lexlean.lock and admit/lexlean.lock are generated, and the export below writes them again.\u{a}")), List::cons(f8(Str::lit("          git apply --exclude=readers/lexlean.lock --exclude=readers/xml/lexlean.lock --exclude=admit/lexlean.lock target/input/change.patch\u{a}")), List::cons(f8(Str::lit("          # The owner reads this pull request, so its body leads with the proposer's own account (the prose\u{a}")), List::cons(f8(Str::lit("          # before the first diff), then the definitions it changes. Checked before the export, which takes minutes.\u{a}")), List::cons(f8(Str::lit("          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}")), List::cons(f8(Str::lit("          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The export refuses Lean output holding an absolute path (LLV7006) only at its end, so refuse an added\u{a}")), List::cons(f8(Str::lit("          # string literal with a slash at a token boundary before a name character here, in seconds.\u{a}")), List::cons(f8(Str::lit("          if grep '^+[^+]' target/input/change.patch | grep '\"' | grep -E \"(^|[[:space:]\\\"'\\`([{<,;=])/[[:alnum:]_.]\" > target/absolute.txt; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::LLV7006: these added lines put a slash at a token boundary, which the export refuses as an absolute path (write >/dev/null with no space):\"\u{a}")), List::cons(f8(Str::lit("            cat target/absolute.txt\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          [ \"$(sha256sum target/lexlean/bin/lexlean 2>/dev/null | cut -c1-64)\" = \"$(jq -r .lexlean.executable_sha256 certified/evidence/attestation.json)\" ] || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked --version 0.4.3 lexlean --root target/lexlean; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          # The one export, shared with refresh and run end to end by the studio check.\u{a}")), List::cons(f8(Str::lit("          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f372(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("            tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}")), List::cons(f8(Str::lit("          {\u{a}")), List::cons(f8(Str::lit("            cat target/summary.md\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"### Definitions this adds or changes\"\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f251() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            {\u{a}")), List::cons(f8(Str::lit("              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}")), List::cons(f8(Str::lit("              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}")), List::cons(f8(Str::lit("            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            slash=/\u{a}")), List::cons(f8(Str::lit("            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            head -n 8 target/export.log.phases || true\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"<${slash}details>\"\u{a}")), List::cons(f8(Str::lit("          } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("      - id: command\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: |\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_readers\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_xml\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_admit\u{a}")), List::cons(f8(Str::lit("          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-0.4.3-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}")), List::cons(f8(Str::lit("        if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          [ \"$(sha256sum target/lexlean/bin/lexlean 2>/dev/null | cut -c1-64)\" = \"$(jq -r .lexlean.executable_sha256 certified/evidence/attestation.json)\" ] || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked --version 0.4.3 lexlean --root target/lexlean; }\u{a}")), List::cons(f8(Str::lit("          target/lexlean/bin/lexlean production verify-artifact certified\u{a}")), List::cons(f8(Str::lit("          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}")), List::cons(f8(Str::lit("          if [ -d certified-readers ]; then\u{a}")), List::cons(f8(Str::lit("            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}")), List::cons(f8(Str::lit("            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}")), List::cons(f8(Str::lit("      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("          INPUT_NAME: ${{ inputs.input_name }}\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail -f\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          # read-* operations run in their reader project's command when it is built (the XML ones in verdant_xml), the admission operations in verdant_admit, every other in the studio command.\u{a}")), List::cons(f8(Str::lit("          vs() { case \"$1\" in admit|admit-encoding|admit-registry|admit-translated|propose-encoding) if [ -x target/release/verdant_admit ]; then target/release/verdant_admit \"$@\"; return; fi;; read-xml|read-xsd|read-xsd-components) if [ -x target/release/verdant_xml ]; then target/release/verdant_xml \"$@\"; return; fi;; read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f252() -> Bytes {
    f7(List::cons(f8(Str::lit("          # An App pull request whose files already hold on main has nothing left to merge,\u{a}")), List::cons(f8(Str::lit("          # so it is closed; the propose-command ones change the API and stay open until they merge.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}")), List::cons(f8(Str::lit("              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}")), List::cons(f8(Str::lit("              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}")), List::cons(f8(Str::lit("              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}")), List::cons(f8(Str::lit("                exit 0\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            # A change to a reader re-derives the encodings it printed before the studio check replays them.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then\u{a}")), List::cons(f8(Str::lit("              readmitted=$(bash .github/scripts/readmit.sh target/release/verdant_readers; if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi)\u{a}")), List::cons(f8(Str::lit("              if [ -n \"$readmitted\" ]; then\u{a}")), List::cons(f8(Str::lit("                { echo; echo \"### Encodings admitted again\"; echo; echo \"This change alters what a reader prints for these admitted encodings, so each was admitted again through read-admit with the command and citation its row records:\"; echo; sed 's/^/- /' <<< \"$readmitted\"; } >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            cat target/operation.txt\u{a}")), List::cons(f8(Str::lit("          elif [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            : > target/operation.txt\u{a}")), List::cons(f8(Str::lit("            last=''\u{a}")), List::cons(f8(Str::lit("            skipped=''\u{a}")), List::cons(f8(Str::lit("            # next prints one operation it can dispatch, or else the lines that say why the\u{a}")), List::cons(f8(Str::lit("            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}")), List::cons(f8(Str::lit("            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}")), List::cons(f8(Str::lit("              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("              if vs $op > target/attempt.txt 2>&1; then\u{a}")), List::cons(f8(Str::lit("                tee -a target/operation.txt < target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                last=$op\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                cat target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                binding=${op#* }\u{a}")), List::cons(f8(Str::lit("                echo \"::warning::$op was refused; the other bindings continue\"\u{a}")), List::cons(f8(Str::lit("                echo \"refused: $op\" >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}")), List::cons(f8(Str::lit("                skipped=\"$skipped $binding\"\u{a}")), List::cons(f8(Str::lit("                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            t0=$EPOCHREALTIME\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}")), List::cons(f8(Str::lit("            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}")), List::cons(f8(Str::lit("            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}")), List::cons(f8(Str::lit("              echo \"waiting merge: #${pr} changes the API, and merges once the studio check passes\" >> target/waiting.txt\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}")), List::cons(f8(Str::lit("          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f253() -> Bytes {
    f7(List::cons(f8(Str::lit("            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}")), List::cons(f8(Str::lit("            # good main commit, it reverts the merge on main as the App, and merges once the studio check passes.\u{a}")), List::cons(f8(Str::lit("            merge=${ARGS#revert-merge }\u{a}")), List::cons(f8(Str::lit("            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B revert origin/main\u{a}")), List::cons(f8(Str::lit("            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("            git revert -m 1 --no-commit \"$merge\"\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::cons(f8(Str::lit("            vs $ARGS | tee target/operation.txt\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("      - name: open a pull request as the Verdant Studio app\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          git add -A\u{a}")), List::cons(f8(Str::lit("          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}")), List::cons(f8(Str::lit("          verdant_studio $ARGS\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("          $(cat target/operation.txt)\u{a}")), List::cons(f8(Str::lit("          MSG\u{a}")), List::cons(f8(Str::lit("          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}")), List::cons(f8(Str::lit("          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}")), List::cons(f8(Str::lit("            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          title=\"verdant_studio $ARGS\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}")), List::cons(f8(Str::lit("          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}")), List::cons(f8(Str::lit("          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}")), List::cons(f8(Str::lit("          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}")), List::cons(f8(Str::lit("          pr=\"\"\u{a}")), List::cons(f8(Str::lit("          for try in 1 2 3 4; do\u{a}")), List::cons(f8(Str::lit("            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            pr=\"\"\u{a}")), List::cons(f8(Str::lit("            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}")), List::cons(f8(Str::lit("            sleep $((2 ** try))\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          echo \"opened #${pr}\"\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("          # A change to the API itself (propose-command, revert-merge) merges as every App pull request does:\u{a}")), List::cons(f8(Str::lit("          # the owner automated these approvals (2026-11-26), so the studio check, which runs the API's own\u{a}")), List::cons(f8(Str::lit("          # end-to-end propose run, is its gate. refresh keeps its auto-merge while it regenerates it.\u{a}")), List::cons(f8(Str::lit("          if [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}")), List::cons(f8(Str::lit("            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f254() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::<Bytes>::nil())))
}

pub fn f255() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# When main moves, every open propose-command pull request that no longer merges\u{a}")), List::cons(f8(Str::lit("# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}")), List::cons(f8(Str::lit("# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}")), List::cons(f8(Str::lit("# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}")), List::cons(f8(Str::lit("# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}")), List::cons(f8(Str::lit("# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}")), List::cons(f8(Str::lit("# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("# The exception is one whose auto-merge is set, the next change to merge: it stays ready and keeps its\u{a}")), List::cons(f8(Str::lit("# auto-merge, so it merges as soon as its regeneration passes the studio check. It is\u{a}")), List::cons(f8(Str::lit("# regenerated first, and only the oldest such one at a time, since two changes to the\u{a}")), List::cons(f8(Str::lit("# API conflict on their export (#309 and #315 were pushed behind main this way).\u{a}")), List::cons(f8(Str::lit("name: refresh\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}")), List::cons(f8(Str::lit("# operate run completes, which needs no token that may dispatch workflows, so every\u{a}")), List::cons(f8(Str::lit("# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_run:\u{a}")), List::cons(f8(Str::lit("    workflows: [operate]\u{a}")), List::cons(f8(Str::lit("    types: [completed]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A running refresh always finishes, because an export takes minutes and main moves more\u{a}")), List::cons(f8(Str::lit("# often than that; GitHub keeps only the newest pending run behind it.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: refresh\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  find:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 15\u{a}")), List::cons(f8(Str::lit("    outputs:\u{a}")), List::cons(f8(Str::lit("      stale: ${{ steps.stale.outputs.stale }}\u{a}")), List::cons(f8(Str::lit("      matrix: ${{ steps.stale.outputs.matrix }}\u{a}")), List::cons(f8(Str::lit("      auto: ${{ steps.stale.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - id: stale\u{a}")), List::cons(f8(Str::lit("        name: find the propose-command pull requests that conflict with main\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          stale=''\u{a}")), List::cons(f8(Str::lit("          auto=''\u{a}")), List::cons(f8(Str::lit("          # Oldest first, so the oldest one whose auto-merge is set is the one regenerated now.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}")), List::cons(f8(Str::lit("            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}")), List::cons(f8(Str::lit("            # which a cancelled run may have left behind.\u{a}")), List::cons(f8(Str::lit("            mergeable=null\u{a}")), List::cons(f8(Str::lit("            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}")), List::cons(f8(Str::lit("              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f256() -> Bytes {
    f7(List::cons(f8(Str::lit("              if [ \"$mergeable\" != null ]; then break; fi\u{a}")), List::cons(f8(Str::lit("              sleep 10\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            # One that changes .github is exported again by the studio check, byte for byte, so it also fails\u{a}")), List::cons(f8(Str::lit("            # when main changed its export inputs since it was exported, even without a conflict: readers\u{a}")), List::cons(f8(Str::lit("            # depends on app, so admissions merged during its export (#357 and #361 under #362) turn it red.\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = true ] && [ \"$(gh api --paginate \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files\" --jq '.[].filename' | grep -c '^\\.github/' || true)\" != 0 ]; then\u{a}")), List::cons(f8(Str::lit("              base=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/main...$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\" --jq .merge_base_commit.sha)\u{a}")), List::cons(f8(Str::lit("              moved=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/${base}...main\" --jq '[.files[]?.filename | select(test(\"^(app|stdlib|studio|readers)/\"))] | length')\u{a}")), List::cons(f8(Str::lit("              if [ \"$moved\" != 0 ]; then echo \"#${pr}: main changed ${moved} of its export inputs since it was exported\"; mergeable=false; fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = false ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              # Drafting would drop its auto-merge, so it stays ready. A later one waits: it is\u{a}")), List::cons(f8(Str::lit("              # regenerated once this one merges, when it would conflict again anyway.\u{a}")), List::cons(f8(Str::lit("              if [ -z \"$auto\" ]; then auto=$pr; else echo \"#${pr} waits for #${auto} to merge\"; fi\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = false ]; then\u{a}")), List::cons(f8(Str::lit("              stale=\"$stale $pr\"\u{a}")), List::cons(f8(Str::lit("              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}")), List::cons(f8(Str::lit("              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$auto\" ]; then stale=\" $auto$stale\"; fi\u{a}")), List::cons(f8(Str::lit("          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"auto=$auto\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("  regenerate:\u{a}")), List::cons(f8(Str::lit("    needs: find\u{a}")), List::cons(f8(Str::lit("    if: needs.find.outputs.stale != ''\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 60\u{a}")), List::cons(f8(Str::lit("    strategy:\u{a}")), List::cons(f8(Str::lit("      fail-fast: false\u{a}")), List::cons(f8(Str::lit("      matrix:\u{a}")), List::cons(f8(Str::lit("        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          fetch-depth: 0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-0.4.3-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          STALE: ${{ matrix.pr }}\u{a}")), List::cons(f8(Str::lit("          AUTO: ${{ needs.find.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A regeneration of the one whose auto-merge is set that fails leaves it a draft, which drops\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f257() -> Bytes {
    f7(List::cons(f8(Str::lit("          # its auto-merge, so it is no longer the next change to merge and nothing is handed over red.\u{a}")), List::cons(f8(Str::lit("          trap 'rc=$?; if [ \"$rc\" != 0 ] && [ \"$STALE\" = \"$AUTO\" ]; then gh pr ready --undo \"$STALE\" || true; fi' EXIT\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            mkdir -p target\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          [ \"$(sha256sum target/lexlean/bin/lexlean 2>/dev/null | cut -c1-64)\" = \"$(jq -r .lexlean.executable_sha256 certified/evidence/attestation.json)\" ] || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked --version 0.4.3 lexlean --root target/lexlean; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          failed=''\u{a}")), List::cons(f8(Str::lit("          for pr in $STALE; do\u{a}")), List::cons(f8(Str::lit("            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}")), List::cons(f8(Str::lit("            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}")), List::cons(f8(Str::lit("            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("            # The one whose auto-merge is set stays ready, so that it keeps its auto-merge.\u{a}")), List::cons(f8(Str::lit("            if [ \"$pr\" != \"$AUTO\" ]; then gh pr ready --undo \"$pr\" || true; fi\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin \"$branch\" main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B refresh origin/main\u{a}")), List::cons(f8(Str::lit("            git clean -fdq\u{a}")), List::cons(f8(Str::lit("            # readers/lexlean.lock and readers/xml/lexlean.lock are generated: the export below writes them again, so they are left out of the\u{a}")), List::cons(f8(Str::lit("            # change, and two reader pull requests never conflict on it.\u{a}")), List::cons(f8(Str::lit("            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml ':!readers/lexlean.lock' ':!readers/xml/lexlean.lock' ':!admit/lexlean.lock' > target/change.patch\u{a}")), List::cons(f8(Str::lit("            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}")), List::cons(f8(Str::lit("            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}")), List::cons(f8(Str::lit("            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}")), List::cons(f8(Str::lit("            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}")), List::cons(f8(Str::lit("              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}")), List::cons(f8(Str::lit("                base=$(git merge-base origin/main \"origin/$branch\")\u{a}")), List::cons(f8(Str::lit("                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}")), List::cons(f8(Str::lit("                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}")), List::cons(f8(Str::lit("                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}")), List::cons(f8(Str::lit("                git add model/strata.json\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}")), List::cons(f8(Str::lit("                git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("                failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}")), List::cons(f8(Str::lit("                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("                continue\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f372(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("              tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("              git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("              failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("              continue\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            rm -rf studio/.lexlean/production readers/.lexlean/production readers/xml/.lexlean/production admit/.lexlean/production\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            # The regenerated change re-derives the encodings whose reader output it changes, as propose-command does.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then bash .github/scripts/readmit.sh target/release/verdant_readers; fi\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("            git add -A\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f258() -> Bytes {
    f7(List::cons(f8(Str::lit("            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml; } > target/message.txt\u{a}")), List::cons(f8(Str::lit("            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}")), List::cons(f8(Str::lit("            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}")), List::cons(f8(Str::lit("            gh pr ready \"$pr\" || true\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::regenerated #${pr} on the new main\"\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))))))
}

pub fn f259(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1121 = v0;
    match m1121.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f260(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt19 = f373(v0.clone(), 0u64, {
        let a1122 = v0;
        length_bytes(a1122)
    }, Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt19 = {
        let a1125 = {
            let a1124 = match v1.clone() {
                Adt19::C0(_, _, h1123, _, _) => {
                    h1123
                }
            };
            length_bytes(a1124)
        };
        let a1126 = 0u64;
        if if nat_eq(a1125, a1126) {
            let m1128 = match v1.clone() {
                Adt19::C0(_, _, _, h1127, _) => {
                    h1127
                }
            };
            match m1128.uncons() {
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
            f374(v1)
        }
    };
    Ok(f375(match v4 {
        Adt19::C0(_, _, _, _, h1129) => {
            h1129
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f261(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt17>, v3: List<Adt2>, v4: List<Bytes>) -> List<Bytes> {
    let m1130 = v1;
    match m1130.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f125(f376(v5.clone(), v2.clone()), f377(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f378(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1132 = {
                    let a1131 = v7.clone();
                    length_list(a1131)
                };
                let a1133 = 0u64;
                if nat_eq(a1132, a1133) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f122(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f261(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f262(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1134 = v0;
    match m1134.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1135 = v1;
            match m1135.uncons() {
                None => {
                    f262(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f262(v2))
                }
            }
        }
    }
}

pub fn f263(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1136 = v1;
    match m1136.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f221(v2.clone(), f8(Str::lit("domain"))) {
                f379(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1137 = f93(v2.clone(), f8(Str::lit("role")));
                    match m1137 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f380(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1138 = f93(v2.clone(), f8(Str::lit("origin")));
                    match m1138 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1139 = f93(v2, f8(Str::lit("evidence")));
                    match m1139 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f263(v0, v3))
            } else {
                f263(v0, v3)
            }
        }
    }
}

pub fn f264(v0: List<Adt2>) -> List<Bytes> {
    let m1140 = v0;
    match m1140.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f264(v2))
        }
    }
}

pub fn f265(v0: List<Adt2>) -> List<Bytes> {
    let m1141 = v0;
    match m1141.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f381(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f265(v2))
        }
    }
}

pub fn f266(p1147: List<Bytes>) -> R<bool> {
    let m1146 = p1147;
    {
        let mut m1146 = m1146;
        loop {
            let step = {
                let v0 = m1146;
                let m1142 = v0;
                match m1142.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1144 = {
                            let a1143 = f198(v1)?;
                            length_bytes(a1143)
                        };
                        let a1145 = 0u64;
                        if nat_eq(a1144, a1145) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(v0: List<Bytes>) -> R<bool> {
    if f276(f300(v0.clone())?) {
        Ok(f51({
            let a1149 = {
                let a1148 = f278(f300(v0)?);
                length_list(a1148)
            };
            let a1150 = 0u64;
            nat_eq(a1149, a1150)
        }))
    } else {
        Ok(false)
    }
}

pub fn f268(v0: List<Bytes>) -> R<bool> {
    if f276(f280(v0.clone(), f8(Str::lit("requirement")))?) {
        if f276(f280(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1152 = {
                    let a1151 = f224(f278(f280(v0.clone(), f8(Str::lit("requirement")))?), f278(f280(v0, f8(Str::lit("admission")))?));
                    length_list(a1151)
                };
                let a1153 = 0u64;
                nat_eq(a1152, a1153)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f269() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f270(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f271() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f272(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1154 = v0;
    match m1154.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1156 = {
                let a1155 = v3.clone();
                length_bytes(a1155)
            };
            let a1157 = 0u64;
            if nat_eq(a1156, a1157) {
                f272(v2)
            } else {
                let m1158 = f43(v3.clone())?;
                match m1158 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f382(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1159 = f272(v2)?;
                            match m1159 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f383(v5), {
                                        let a1160 = v3;
                                        let a1161 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1160, a1161)
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

pub fn f273() -> Bytes {
    let a1162 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1162)
}

pub fn f274(p1167: List<Bytes>) -> R<Option<Bytes>> {
    let m1166 = p1167;
    {
        let mut m1166 = m1166;
        loop {
            let step = {
                let v0 = m1166;
                let m1163 = v0;
                match m1163.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1164 = f43(f198(v1)?)?;
                        match m1164 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1165 = f93(v3.clone(), f8(Str::lit("source")));
                                match m1165 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f384(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes) -> Bytes {
    let a1172 = {
        let a1168 = Str::lit("binding ");
        utf8_encode(a1168)
    };
    let a1173 = {
        let a1170 = v0;
        let a1171 = {
            let a1169 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1169)
        };
        append_bytes(a1170, a1171)
    };
    append_bytes(a1172, a1173)
}

pub fn f276(v0: Option<List<Adt2>>) -> bool {
    let m1174 = v0;
    match m1174 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f277(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1175 = v0;
    match m1175.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1177 = {
                let a1176 = v3.clone();
                length_bytes(a1176)
            };
            let a1178 = 0u64;
            if nat_eq(a1177, a1178) {
                f277(v2)
            } else {
                let m1179 = f43(v3)?;
                match m1179 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if f117(f117(f221(v4.clone(), f8(Str::lit("authority"))), f221(v4.clone(), f8(Str::lit("row")))), f117(f117(f221(v4.clone(), f8(Str::lit("construct"))), f221(v4.clone(), f8(Str::lit("artifact")))), f286(f58(v4.clone(), f8(Str::lit("reader"))), f58(v4.clone(), f8(Str::lit("construct")))))) {
                            let m1180 = f277(v2)?;
                            match m1180 {
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

pub fn f278(v0: Option<List<Adt2>>) -> List<Adt2> {
    let m1181 = v0;
    match m1181 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f279(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1182 = v0;
    match m1182.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1184 = {
                let a1183 = v3.clone();
                length_bytes(a1183)
            };
            let a1185 = 0u64;
            if nat_eq(a1184, a1185) {
                f279(v2)
            } else {
                let m1186 = f43(v3.clone())?;
                match m1186 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f385(v4.clone()) {
                            let m1187 = f279(v2)?;
                            match m1187 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f244(), List::cons(f8(Str::lit("/")), List::cons(f386(f58(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1188 = v3;
                                        let a1189 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1188, a1189)
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

pub fn f280(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt2>>> {
    let m1190 = v0;
    match m1190.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f198(v2)?;
            let a1192 = {
                let a1191 = v4.clone();
                length_bytes(a1191)
            };
            let a1193 = 0u64;
            if nat_eq(a1192, a1193) {
                f280(v3, v1)
            } else {
                let m1194 = f43(v4)?;
                match m1194 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f387(v5.clone());
                        let a1196 = {
                            let a1195 = v6.clone();
                            length_bytes(a1195)
                        };
                        let a1197 = 0u64;
                        if nat_eq(a1196, a1197) {
                            Ok(None::<List<Adt2>>)
                        } else {
                            let m1198 = f280(v3, v1.clone())?;
                            match m1198 {
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

pub fn f281(p1201: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1200 = p1201;
    {
        let mut m1200 = m1200;
        loop {
            let step = {
                let v0 = m1200;
                let m1199 = v0;
                match m1199.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f52(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f283(v0: Bytes, v1: List<Adt2>) -> R<(Bytes, Bytes)> {
    Ok((v0, f100(Adt2::C7(std::rc::Rc::new(f101(v1))))?))
}

pub fn f284(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f100(Adt2::C7(std::rc::Rc::new(f388(f145(f125(v1, f215(f96(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f285(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1202 = f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m1202.uncons() {
        None => {
            let m1204 = f281(f131(List::cons(Adt2::C8(std::rc::Rc::new({
                let m1203 = f389(v4.clone());
                match m1203 {
                    None => {
                        Adt2::C9
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt2>::nil())), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1204 {
                None => {
                    f390(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f282(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f286(v0: Bytes, v1: Bytes) -> bool {
    f203(f85(v0.clone(), f8(Str::lit("read-"))), f117(f32(v0, f8(Str::lit("check-yaml"))), f32(v1, f8(Str::lit("none")))))
}

pub fn f287(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f288(v0: Bytes) -> R<Bytes> {
    Ok(f391(f392(v0)?))
}

pub fn f289(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: List<Bytes>, v5: Bytes) -> R<Adt12> {
    let v6: Bytes = {
        let a1205 = f8(Str::lit("registry-read "));
        let a1206 = v5.clone();
        append_bytes(a1205, a1206)
    };
    if f51(f52(v6.clone(), match v0.clone() {
        Adt1::C0(_, _, h1207, _) => {
            h1207
        }
    })) {
        Ok(Adt12::C2({
            let m1208 = v4.clone();
            match m1208.uncons() {
                None => {
                    f393()
                }
                Some((v7, _)) => {
                    if f394(v7) {
                        f395()
                    } else {
                        f393()
                    }
                }
            }
        }, v4, v6))
    } else {
        let m1209 = f56(v0.clone(), List::cons(f318(), List::<Bytes>::nil()));
        match m1209.uncons() {
            None => {
                f396(v0, v1, f318(), v2, v3, v4, v5)
            }
            Some((v9, v10)) => {
                Ok(Adt12::C0(List::cons(v9, v10)))
            }
        }
    }
}

pub fn f290() -> Bytes {
    f397()
}

pub fn f291(v0: Adt1, v1: Adt2, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f295(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f398(v1, v4)?)?), List::cons(f284(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f292(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1210 = v0;
    match m1210.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f293() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f294(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f360(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f295(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f399(f150(f95(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f400(f401(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f296(p1213: List<Adt2>) -> bool {
    let m1212 = p1213;
    {
        let mut m1212 = m1212;
        loop {
            let step = {
                let v0 = m1212;
                let m1211 = v0;
                match m1211.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f142(f402(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1212 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: List<Bytes>) -> List<Bytes> {
    let m1214 = v0;
    match m1214.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f403(v1.clone()) {
                List::<Bytes>::nil()
            } else {
                List::cons(v1, f297(v2))
            }
        }
    }
}

pub fn f298(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: List<Bytes>) -> R<Adt12> {
    let m1215 = f389(v4);
    match m1215 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m1216 = f281(f131(List::cons(Adt2::C8(std::rc::Rc::new(v5.clone())), List::<Adt2>::nil())), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1216 {
                None => {
                    f404(v0, v2.clone(), Adt2::C11(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f388(v3)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C6(f288(v1.clone())?)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(v5)), f405(v2, f406(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f282(v6))
                }
            }
        }
    }
}

pub fn f299(p1219: List<Bytes>) -> List<Bytes> {
    let m1218 = p1219;
    {
        let mut m1218 = m1218;
        loop {
            let step = {
                let v0 = m1218;
                let m1217 = v0;
                match m1217.uncons() {
                    None => {
                        Err::<List<Bytes>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v1, v2)) => {
                        if f403(v1.clone()) {
                            Err::<List<Bytes>, List<Bytes>>(List::cons(v1, v2))
                        } else {
                            Ok::<List<Bytes>, List<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1220 = v0;
    match m1220.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1222 = {
                let a1221 = v3.clone();
                length_bytes(a1221)
            };
            let a1223 = 0u64;
            if nat_eq(a1222, a1223) {
                f300(v2)
            } else {
                let m1224 = f43(v3)?;
                match m1224 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if if f221(v4.clone(), f8(Str::lit("agent"))) {
                            if f221(v4.clone(), f8(Str::lit("at"))) {
                                if f221(v4.clone(), f8(Str::lit("quote"))) {
                                    f407(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1225 = f300(v2)?;
                            match m1225 {
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

pub fn f301() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f302(p1228: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m1227 = p1228;
    {
        let mut m1227 = m1227;
        loop {
            let step = {
                let v0 = m1227;
                let m1226 = v0;
                match m1226.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f117(f32(f58(v3.clone(), f8(Str::lit("observedProperty"))), v1.clone()), f32(f58(v3, f8(Str::lit("hasSimpleResult"))), v2.clone())) {
                            Err::<List<Adt2>, bool>(true)
                        } else {
                            Ok::<List<Adt2>, bool>(v4)
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

pub fn f303(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt2::C6(v0)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))))))))
}

pub fn f304(v0: u64) -> bool {
    let a1229 = v0;
    let a1230 = 127u64;
    nat_le(a1229, a1230)
}

pub fn f305(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1231, _) = v1.clone();
        h1231
    };
    let v3: u64 = {
        let (h1232, _) = {
            let (_, h1233) = v1.clone();
            h1233
        };
        h1232
    };
    let v4: List<Bytes> = {
        let (_, h1234) = {
            let (_, h1235) = v1;
            h1235
        };
        h1234
    };
    let m1236 = f208(v0.clone(), v2);
    match m1236 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1237 = v5;
            let a1238 = 10u8;
            if equal(a1237, a1238) {
                Ok(Some(({
                    let a1239 = v2;
                    let a1240 = 1u64;
                    nat_add(a1239, a1240)?
                }, ({
                    let a1241 = v2;
                    let a1242 = 1u64;
                    nat_add(a1241, a1242)?
                }, List::cons(f408(f46(v0, v3, {
                    let a1243 = v2;
                    let a1244 = v3;
                    nat_sub(a1243, a1244)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1245 = v2;
                    let a1246 = 1u64;
                    nat_add(a1245, a1246)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f306(v0: Adt2) -> List<Adt2> {
    let m1247 = v0;
    match m1247 {
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
        Adt2::C7(r1248) => {
            let v3 = (*r1248).clone();
            f306(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r1249, r1250) => {
            let v1 = (*r1249).clone();
            let v2 = (*r1250).clone();
            List::cons(v1, f306(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f307(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1251 = v0;
    match m1251 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r1252) => {
            let v12 = (*r1252).clone();
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
        Adt2::C7(r1253) => {
            let v10 = (*r1253).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r1254) => {
            let v5 = (*r1254).clone();
            Adt2::C8(std::rc::Rc::new(f307(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1255, r1256) => {
            let v18 = (*r1255).clone();
            let v19 = (*r1256).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r1257, r1258) => {
            let v3 = (*r1257).clone();
            let v4 = (*r1258).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt2::C11(v2, std::rc::Rc::new(f409(v3, v1.clone())), std::rc::Rc::new(f307(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f307(v4, v1)))
            }
        }
    }
}

pub fn f308(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1259 = 0u64;
        let a1260 = v1;
        nat_lt(a1259, a1260)
    } else {
        false
    } {
        let a1261 = Bytes::lit(&[0x2d]);
        let a1262 = f410(v1)?;
        Ok(append_bytes(a1261, a1262))
    } else {
        f410(v1)
    }
}

pub fn f309(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f410(v1)?;
    let v4: u64 = {
        let a1263 = v3.clone();
        length_bytes(a1263)
    };
    let v5: Bytes = {
        let a1264 = v2;
        let a1265 = v4;
        if nat_lt(a1264, a1265) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1266 = v4;
                let a1267 = v2;
                nat_sub(a1266, a1267)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1268 = v4;
                let a1269 = v2;
                nat_sub(a1268, a1269)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f330({
                let a1270 = v2;
                let a1271 = v4;
                nat_sub(a1270, a1271)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1272 = f8(Str::lit("-"));
        let a1273 = v5;
        Ok(append_bytes(a1272, a1273))
    } else {
        Ok(v5)
    }
}

pub fn f310(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f411(v0)?;
    let v3: bool = {
        let a1274 = f412(v0)?;
        let a1275 = v1;
        nat_le(a1274, a1275)
    };
    let v4: u64 = {
        let a1279 = {
            let a1276 = v1;
            let a1277 = v2;
            let a1278 = 0u64;
            nat_quot(a1276, a1277, a1278)
        };
        let a1280 = f413(f414(v0))?;
        let a1281 = 0u64;
        nat_rem(a1279, a1280, a1281)
    };
    let v5: u64 = {
        let a1282 = v1;
        let a1283 = v2;
        let a1284 = 0u64;
        nat_rem(a1282, a1283, a1284)
    };
    let a1285 = v4;
    let a1286 = f415(v0)?;
    if nat_eq(a1285, a1286) {
        let a1287 = v5;
        let a1288 = 0u64;
        if nat_eq(a1287, a1288) {
            Ok(f416(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f416(v3, {
                let a1292 = {
                    let a1289 = v2;
                    let a1290 = 2u64;
                    let a1291 = 0u64;
                    nat_quot(a1289, a1290, a1291)
                };
                let a1293 = v5;
                if nat_le(a1292, a1293) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f117({
            let a1294 = v4;
            let a1295 = 0u64;
            nat_eq(a1294, a1295)
        }, {
            let a1296 = v5;
            let a1297 = 0u64;
            nat_eq(a1296, a1297)
        }) {
            Ok(f416(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1298 = v4;
                let a1299 = 0u64;
                if nat_eq(a1298, a1299) {
                    v5
                } else {
                    let a1300 = v2;
                    let a1301 = v5;
                    nat_add(a1300, a1301)?
                }
            };
            let v7: u64 = {
                let a1302 = v4;
                let a1303 = 0u64;
                if nat_eq(a1302, a1303) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1310 = {
                    let a1306 = {
                        let a1304 = v7;
                        let a1305 = 2000u64;
                        nat_add(a1304, a1305)?
                    };
                    let a1307 = f417(v0);
                    nat_sub(a1306, a1307)
                };
                let a1311 = {
                    let a1308 = f418(v0);
                    let a1309 = 1u64;
                    nat_sub(a1308, a1309)
                };
                nat_sub(a1310, a1311)
            };
            Ok(f416(v3, {
                let a1312 = 2000u64;
                let a1313 = v8;
                if nat_le(a1312, a1313) {
                    f419(v6, true, {
                        let a1314 = v8;
                        let a1315 = 2000u64;
                        nat_sub(a1314, a1315)
                    })?
                } else {
                    f419(v6, false, {
                        let a1316 = 2000u64;
                        let a1317 = v8;
                        nat_sub(a1316, a1317)
                    })?
                }
            }))
        }
    }
}

pub fn f311(v0: Bytes) -> R<Bytes> {
    let a1322 = f200(v0.clone(), 0u64, {
        let a1319 = {
            let a1318 = v0.clone();
            length_bytes(a1318)
        };
        let a1320 = 1u64;
        nat_add(a1319, a1320)?
    })?;
    let a1323 = {
        let a1321 = v0.clone();
        length_bytes(a1321)
    };
    if nat_eq(a1322, a1323) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1329 = {
            let a1327 = Bytes::lit(&[0x22]);
            let a1328 = f420(v0.clone(), 0u64, {
                let a1325 = {
                    let a1324 = v0;
                    length_bytes(a1324)
                };
                let a1326 = 1u64;
                nat_add(a1325, a1326)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1327, a1328)
        };
        let a1330 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1329, a1330))
    }
}

pub fn f312(v0: List<Bytes>) -> Bytes {
    let a1333 = {
        let a1331 = Bytes::lit(&[0x5b]);
        let a1332 = f122(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1331, a1332)
    };
    let a1334 = Bytes::lit(&[0x5d]);
    append_bytes(a1333, a1334)
}

pub fn f313(v0: Adt2) -> R<List<Bytes>> {
    let m1335 = v0;
    match m1335 {
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
        Adt2::C10(r1336, r1337) => {
            let v14 = (*r1336).clone();
            let v15 = (*r1337).clone();
            Ok(List::cons(f190(v14)?, f313(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f314(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f421(Fn5::F422, List::<(Bytes, Bytes)>::nil(), f423(v0)?);
    let v2: List<Bytes> = f424(Fn6::F425, List::<Bytes>::nil(), v1)?;
    let a1340 = {
        let a1338 = Bytes::lit(&[0x7b]);
        let a1339 = f122(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1338, a1339)
    };
    let a1341 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1340, a1341))
}

pub fn f315(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1342 = v0;
    match m1342 {
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
        Adt2::C11(v16, r1343, r1344) => {
            let v17 = (*r1343).clone();
            let v18 = (*r1344).clone();
            Ok(List::cons((v16, f190(v17)?), f315(v18)?))
        }
    }
}

pub fn f316() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f317(v0: Adt13) -> Bytes {
    let m1345 = v0;
    match m1345 {
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

pub fn f318() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f319(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f426(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f426(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f426(f8(Str::lit("name")), f427(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f426(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f426(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f426(f8(Str::lit("statement")), f428(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f426(f8(Str::lit("media_type")), f429(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f426(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f426(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f426(f8(Str::lit("url")), f193(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f320(v0: Bytes) -> R<u64> {
    let (h1346, _) = f323(Fn3::F430(v0.clone()), {
        let a1347 = v0.clone();
        length_bytes(a1347)
    }, {
        let a1348 = v0;
        length_bytes(a1348)
    })?;
    Ok(h1346)
}

pub fn f321(v0: Bytes, v1: u64) -> R<u64> {
    let (h1349, _) = f323(Fn3::F431(v1, v0), v1, 0u64)?;
    Ok(h1349)
}

pub fn f322(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f115(v0.clone(), {
        let a1350 = v1;
        let a1351 = 1u64;
        nat_add(a1350, a1351)?
    })?;
    let v4: Bytes = {
        let a1352 = v3;
        let a1353 = 117u64;
        if nat_eq(a1352, a1353) {
            f432(v0, v1)?
        } else {
            f433(v3)
        }
    };
    let a1354 = v4.clone();
    let a1355 = Bytes::lit(&[]);
    if equal(a1354, a1355) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1356 = v3;
            let a1357 = 117u64;
            if nat_eq(a1356, a1357) {
                let a1358 = v1;
                let a1359 = 6u64;
                nat_add(a1358, a1359)?
            } else {
                let a1360 = v1;
                let a1361 = 2u64;
                nat_add(a1360, a1361)?
            }
        }, ({
            let a1362 = v2;
            let a1363 = v4;
            append_bytes(a1362, a1363)
        }, 0u64))))
    }
}

pub fn f323(v0: Fn3, p1368: u64, p1369: u64) -> R<(u64, bool)> {
    let m1367 = (p1368, p1369);
    {
        let mut m1367 = m1367;
        loop {
            let step = {
                let (v1, v2) = m1367;
                let m1364 = v1;
                if m1364 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1364 - 1;
                    let m1366 = {
                        let c1365: Fn3 = v0.clone();
                        c1365.apply(v2)?
                    };
                    match m1366 {
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
                Ok(next) => { m1367 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f434(v0, v1)
}

pub fn f325(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f435(v0, v1)
}

pub fn f326(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1370 = v0.clone();
        length_bytes(a1370)
    };
    let a1371 = 0u64;
    let a1372 = v1;
    if nat_lt(a1371, a1372) {
        let a1373 = f436(v0.clone())?;
        let a1374 = v1;
        if nat_eq(a1373, a1374) {
            let a1375 = 1u64;
            let a1376 = v1;
            if nat_lt(a1375, a1376) {
                let m1379 = {
                    let a1377 = v0;
                    let a1378 = 0u64;
                    index_bytes(a1377, a1378)
                };
                match m1379 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1380 = v2;
                        let a1381 = 48u8;
                        Ok(!equal(a1380, a1381))
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

pub fn f327(v0: Fn4, p1386: u64, p1387: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1385 = (p1386, p1387);
    {
        let mut m1385 = m1385;
        loop {
            let step = {
                let (v1, v2) = m1385;
                let m1382 = v1;
                if m1382 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1382 - 1;
                    let m1384 = {
                        let c1383: Fn4 = v0.clone();
                        c1383.apply(v2)?
                    };
                    match m1384 {
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
                Ok(next) => { m1385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f437(v0, v1)
}

pub fn f329(v0: Bytes) -> R<u64> {
    let (h1388, _) = f323(Fn3::F438(v0.clone()), {
        let a1390 = {
            let a1389 = v0;
            length_bytes(a1389)
        };
        let a1391 = 1u64;
        nat_add(a1390, a1391)?
    }, 0u64)?;
    Ok(h1388)
}

pub fn f330(v0: u64) -> Bytes {
    let m1392 = v0;
    if m1392 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1392 - 1;
        let a1393 = f8(Str::lit("0"));
        let a1394 = f330(v1);
        append_bytes(a1393, a1394)
    }
}

pub fn f331(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1395 = v0;
    match m1395.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1396 = f93(v1, f8(Str::lit("std")));
            match m1396 {
                None => {
                    f331(v2)
                }
                Some(v3) => {
                    if f221(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f331(v2))
                    } else {
                        f331(v2)
                    }
                }
            }
        }
    }
}

pub fn f332(v0: Adt14, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1397 = v1;
    match m1397.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt14::C0(h1398, _, _) => {
                    h1398
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1399, _) => {
                        h1399
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f332(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt14::C0(h1400, _, _) => {
                        h1400
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1401, _) => {
                            h1401
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f332(v0, v3))
                } else {
                    f332(v0, v3)
                }
            }
        }
    }
}

pub fn f333(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt17>> {
    let m1402 = v0;
    match m1402.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v4, v5)) => {
            if if f439(v4.clone(), v2.clone()) {
                true
            } else {
                f439(v4.clone(), v3.clone())
            } {
                f333(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt17::C0({
                    let (h1403, _) = v4.clone();
                    h1403
                }, {
                    let a1406 = {
                        let a1405 = f145(f440({
                            let (h1404, _) = v4.clone();
                            h1404
                        }, v1.clone()))?;
                        length_list(a1405)
                    };
                    let a1407 = 1u64;
                    if nat_eq(a1406, a1407) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1408) = v4.clone();
                    h1408
                }), f333(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f334(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1409 = v1;
    match m1409.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1410, _) = v2.clone();
                h1410
            }, v0.clone()) {
                List::cons({
                    let (_, h1411) = v2;
                    h1411
                }, f334(v0, v3))
            } else {
                if f52({
                    let (_, h1412) = v2;
                    h1412
                }, v0.clone()) {
                    List::cons({
                        let (h1413, _) = v2;
                        h1413
                    }, f334(v0, v3))
                } else {
                    f334(v0, v3)
                }
            }
        }
    }
}

pub fn f335(v0: Adt14, v1: Adt2, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt14::C0(h1414, _, _) => {
            h1414
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1415, _) => {
                h1415
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt14::C0(_, _, h1416) => {
                    h1416
                }
            }) {
                true
            } else {
                if f221(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1417, _) => {
                            h1417
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f336(v0: Adt2) -> bool {
    if f221(v0.clone(), f8(Str::lit("domain"))) {
        if f221(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f441()) {
                if f442(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f442(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f442(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f442(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f337(v0: Adt2) -> bool {
    if f221(v0.clone(), f8(Str::lit("subject"))) {
        if f221(v0.clone(), f8(Str::lit("object"))) {
            if f221(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f443())
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

pub fn f338(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1418 = f43(v1)?;
    match m1418 {
        None => {
            Ok(f227(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f215(f188(f93(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1419 = f92(f93(v4.clone(), f8(Str::lit("construct"))));
                match m1419 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1420 = v5;
            match m1420.uncons() {
                None => {
                    Ok(f227(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1421 = v9;
                    match m1421.uncons() {
                        None => {
                            Ok(f227(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f51(f32(v8, f8(Str::lit("std")))) {
                                Ok(f227(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f51(f52(v10.clone(), v2)) {
                                    Ok(f227(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f117(f51(f32(v7.clone(), f8(Str::lit("none")))), f51(f52(v7.clone(), v3))) {
                                        Ok(f227(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1422 = f92(f93(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1422 {
                                            None => {
                                                Ok(f227(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1423 = f93(v4.clone(), f8(Str::lit("reader")));
                                                match m1423 {
                                                    None => {
                                                        let m1424 = f93(v4, f8(Str::lit("value")));
                                                        match m1424 {
                                                            None => {
                                                                Ok(f227(v0, f8(Str::lit("a row names neither the reader of its artifact nor a value"))))
                                                            }
                                                            Some(_) => {
                                                                Ok(List::<Bytes>::nil())
                                                            }
                                                        }
                                                    }
                                                    Some(_) => {
                                                        if f221(v4, f8(Str::lit("artifact"))) {
                                                            Ok(List::<Bytes>::nil())
                                                        } else {
                                                            Ok(f227(v0, f8(Str::lit("a row names its reader but not its artifact"))))
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

pub fn f339(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1425 = v1;
    match m1425.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f444(v3.clone())
            } else {
                f51(f444(v3.clone()))
            } {
                List::cons(f240(v0.clone(), v3), f339(v0, v4, v2))
            } else {
                f339(v0, v4, v2)
            }
        }
    }
}

pub fn f340(p1428: List<Bytes>) -> R<Option<Bytes>> {
    let m1427 = p1428;
    {
        let mut m1427 = m1427;
        loop {
            let step = {
                let v0 = m1427;
                let m1426 = v0;
                match m1426.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f445(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f198(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1429 = v1;
    match m1429.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f95(f64(v0.clone(), v2))?, f341(v0, v3)?))
        }
    }
}

pub fn f342(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1430 = v0.clone();
        length_bytes(a1430)
    };
    let v3: u64 = {
        let a1431 = v1.clone();
        length_bytes(a1431)
    };
    let v4: u64 = {
        let (h1432, _) = f323(Fn3::F446(v1.clone(), v3, v0.clone(), v2), {
            let a1433 = v2;
            let a1434 = 1u64;
            nat_add(a1433, a1434)?
        }, 0u64)?;
        h1432
    };
    if f447(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f343() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f344(v0: Adt15) -> R<Adt18> {
    Ok(Adt18::C0(f448(match v0.clone() {
        Adt15::C0(h1435, _, _, _, _) => {
            h1435
        }
    }, 0u64)?, f448(match v0.clone() {
        Adt15::C0(_, h1436, _, _, _) => {
            h1436
        }
    }, 0u64)?, f448(match v0.clone() {
        Adt15::C0(_, _, h1437, _, _) => {
            h1437
        }
    }, 0u64)?, f448(match v0 {
        Adt15::C0(_, _, _, h1438, _) => {
            h1438
        }
    }, 0u64)?))
}

pub fn f345(v0: Adt15) -> List<Bytes> {
    f125(match v0.clone() {
        Adt15::C0(h1439, _, _, _, _) => {
            h1439
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, h1440, _, _) => {
            h1440
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, _, h1441, _) => {
            h1441
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1442) => {
            h1442
        }
    })))
}

pub fn f346(v0: Adt18, p1448: List<u64>, p1449: List<Adt20>) -> Option<u64> {
    let m1447 = (p1448, p1449);
    {
        let mut m1447 = m1447;
        loop {
            let step = {
                let (v1, v2) = m1447;
                let m1443 = v2;
                match m1443.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f449(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1444) => {
                                h1444
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1445, _) => {
                                    h1445
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1446, _) => {
                                    h1446
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: Adt15, v1: List<Adt2>, v2: u64) -> R<List<Adt20>> {
    let m1450 = v1;
    match m1450.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f450(v0.clone(), v3)?), f347(v0, v4, {
                let a1451 = v2;
                let a1452 = 1u64;
                nat_add(a1451, a1452)?
            })?))
        }
    }
}

pub fn f348(p1456: List<Bytes>, p1457: u64) -> Bytes {
    let m1455 = (p1456, p1457);
    {
        let mut m1455 = m1455;
        loop {
            let step = {
                let (v0, v1) = m1455;
                let m1453 = v0;
                match m1453.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1454 = v1;
                        if m1454 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1454 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
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

pub fn f349(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f451(v0.clone(), v1.clone()) {
        if f451(v1, v0) {
            f452(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f350(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1458 = v0;
    match m1458.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f453(v2, v1.clone())?, f350(v3, v1)?))
        }
    }
}

pub fn f351(v0: List<Adt2>) -> List<u64> {
    let m1459 = v0;
    match m1459.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f454(v1), f351(v2))
        }
    }
}

pub fn f352(p1462: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1461 = p1462;
    {
        let mut m1461 = m1461;
        loop {
            let step = {
                let v0 = m1461;
                let m1460 = v0;
                match m1460.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f455(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f353(p1465: List<Bytes>, p1466: List<Bytes>) -> Option<Bytes> {
    let m1464 = (p1465, p1466);
    {
        let mut m1464 = m1464;
        loop {
            let step = {
                let (v0, v1) = m1464;
                let m1463 = v0;
                match m1463.uncons() {
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
                Ok(next) => { m1464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f355(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1467 = v0.clone();
    match m1467.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1468 = v2;
            let a1469 = v1.clone();
            if equal(a1468, a1469) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f356(v0: List<(Bytes, ())>) -> List<Bytes> {
    f237(f456(Fn7::F457, List::<Bytes>::nil(), v0))
}

pub fn f357(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f458(f459(Fn8::F460, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1474 = {
        let (h1470, _) = f461(Fn9::F462, {
            let a1472 = {
                let a1471 = v0;
                length_list(a1471)
            };
            let a1473 = 1u64;
            nat_add(a1472, a1473)?
        }, v1)?;
        h1470
    };
    match m1474.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f358(v0: List<Bytes>) -> List<(Bytes, ())> {
    f463(f464(Fn10::F465, List::<(Bytes, ())>::nil(), v0))
}

pub fn f359(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1475 = f44(v1, 34u8)?;
    match m1475.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1476 = v3;
            match m1476.uncons() {
                None => {
                    Ok(f466(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f360(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f467(v0.clone(), 0u64, {
        let a1477 = v0;
        length_bytes(a1477)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f361(v0: Adt2, v1: Bytes) -> Bytes {
    if f32(f468(f93(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f362(v0: Bytes, v1: Bytes, p1490: u64, p1491: u64) -> R<bool> {
    let m1489 = (p1490, p1491);
    {
        let mut m1489 = m1489;
        loop {
            let step = {
                let (v2, v3) = m1489;
                let m1478 = v3;
                if m1478 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1478 - 1;
                    let m1481 = {
                        let a1479 = v1.clone();
                        let a1480 = v2;
                        index_bytes(a1479, a1480)
                    };
                    match m1481 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1484 = {
                                let a1482 = v0.clone();
                                let a1483 = v2;
                                index_bytes(a1482, a1483)
                            };
                            match m1484 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1485 = v5;
                                    let a1486 = v6;
                                    if equal(a1485, a1486) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1487 = v2;
                                            let a1488 = 1u64;
                                            nat_add(a1487, a1488)?
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
                Ok(next) => { m1489 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1492 = f91(v0, v1)?;
    match m1492 {
        None => {
            Ok(Adt2::C9)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f364(v0: Bytes) -> R<Bytes> {
    let a1493 = f8(Str::lit("Catalog."));
    let a1494 = f386(v0)?;
    Ok(append_bytes(a1493, a1494))
}

pub fn f365(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f386(f58(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f366(v0: List<Bytes>) -> List<Bytes> {
    let m1495 = v0;
    match m1495.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f240(f269(), v1), f366(v2))
            } else {
                f366(v2)
            }
        }
    }
}

pub fn f367(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1496 = v1;
    match m1496.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f469(f470(f95(f64(v0.clone(), v2))?)?), f367(v0, v3)?))
        }
    }
}

pub fn f368(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1497 = v1;
    match m1497.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f471(v0.clone(), v3, v2.clone())?;
            let (h1498, _) = v5.clone();
            if h1498 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f368(v0, v4, v2)?;
                let (h1499, _) = v6.clone();
                if h1499 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1502 = {
                            let (_, h1500) = v5;
                            h1500
                        };
                        let a1503 = {
                            let (_, h1501) = v6;
                            h1501
                        };
                        append_bytes(a1502, a1503)
                    }))
                }
            }
        }
    }
}

pub fn f369(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1504 = v1;
    match m1504.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f221(v2.clone(), f8(Str::lit("subject"))) {
                if f222(v0.clone(), v2.clone()) {
                    f369(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f369(v0, v3))
                }
            } else {
                if f379(v0.clone(), v2.clone()) {
                    f369(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f380(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f369(v0, v3))
                }
            }
        }
    }
}

pub fn f370(v0: Adt1) -> R<Bytes> {
    let a1506 = {
        let a1505 = f128(f129(f262(f259(f260(f64(v0.clone(), f213()))?)), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f131(f96(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1505)
    };
    let a1507 = 0u64;
    if nat_eq(a1506, a1507) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f371(v0: List<Adt2>) -> R<List<Bytes>> {
    let m1508 = v0;
    match m1508.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f122(f215(f188(f93(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f472(f95(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f371(v2)?))
        }
    }
}

pub fn f372() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f373(v0: Bytes, p1566: u64, p1567: u64, p1568: Adt19) -> R<Adt19> {
    let m1565 = (p1566, (p1567, p1568));
    {
        let mut m1565 = m1565;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1565;
                let m1509 = v2;
                if m1509 == 0 {
                    Err::<(u64, (u64, Adt19)), R<Adt19>>(Ok(v3))
                } else {
                    let v4 = m1509 - 1;
                    let v5: u64 = f473(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt19::C0(_, h1510, _, _, _) => {
                            h1510
                        }
                    } {
                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                            let a1511 = v1;
                            let a1512 = 1u64;
                            nat_add(a1511, a1512)?
                        }, (v4, Adt19::C0(true, false, match v3.clone() {
                            Adt19::C0(_, _, h1513, _, _) => {
                                h1513
                            }
                        }, match v3.clone() {
                            Adt19::C0(_, _, _, h1514, _) => {
                                h1514
                            }
                        }, match v3 {
                            Adt19::C0(_, _, _, _, h1515) => {
                                h1515
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt19::C0(h1516, _, _, _, _) => {
                                h1516
                            }
                        } {
                            let a1517 = v5;
                            let a1518 = 34u64;
                            if nat_eq(a1517, a1518) {
                                let a1521 = f473(v0.clone(), {
                                    let a1519 = v1;
                                    let a1520 = 1u64;
                                    nat_add(a1519, a1520)?
                                })?;
                                let a1522 = 34u64;
                                if nat_eq(a1521, a1522) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1523 = v1;
                                        let a1524 = 1u64;
                                        nat_add(a1523, a1524)?
                                    }, (v4, Adt19::C0(true, true, f474(match v3.clone() {
                                        Adt19::C0(_, _, h1525, _, _) => {
                                            h1525
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt19::C0(_, _, _, h1526, _) => {
                                            h1526
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1527) => {
                                            h1527
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1528 = v1;
                                        let a1529 = 1u64;
                                        nat_add(a1528, a1529)?
                                    }, (v4, Adt19::C0(false, false, match v3.clone() {
                                        Adt19::C0(_, _, h1530, _, _) => {
                                            h1530
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1531, _) => {
                                            h1531
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1532) => {
                                            h1532
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1533 = v1;
                                    let a1534 = 1u64;
                                    nat_add(a1533, a1534)?
                                }, (v4, Adt19::C0(true, false, f474(match v3.clone() {
                                    Adt19::C0(_, _, h1535, _, _) => {
                                        h1535
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt19::C0(_, _, _, h1536, _) => {
                                        h1536
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1537) => {
                                        h1537
                                    }
                                }))))
                            }
                        } else {
                            let a1538 = v5;
                            let a1539 = 34u64;
                            if nat_eq(a1538, a1539) {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1540 = v1;
                                    let a1541 = 1u64;
                                    nat_add(a1540, a1541)?
                                }, (v4, Adt19::C0(true, false, match v3.clone() {
                                    Adt19::C0(_, _, h1542, _, _) => {
                                        h1542
                                    }
                                }, match v3.clone() {
                                    Adt19::C0(_, _, _, h1543, _) => {
                                        h1543
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1544) => {
                                        h1544
                                    }
                                }))))
                            } else {
                                let a1545 = v5;
                                let a1546 = 44u64;
                                if nat_eq(a1545, a1546) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1547 = v1;
                                        let a1548 = 1u64;
                                        nat_add(a1547, a1548)?
                                    }, (v4, Adt19::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt19::C0(_, _, h1549, _, _) => {
                                            h1549
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1550, _) => {
                                            h1550
                                        }
                                    }), match v3 {
                                        Adt19::C0(_, _, _, _, h1551) => {
                                            h1551
                                        }
                                    }))))
                                } else {
                                    let a1552 = v5;
                                    let a1553 = 13u64;
                                    if nat_eq(a1552, a1553) {
                                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                            let a1554 = v1;
                                            let a1555 = 1u64;
                                            nat_add(a1554, a1555)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1556 = v5;
                                        let a1557 = 10u64;
                                        if nat_eq(a1556, a1557) {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1558 = v1;
                                                let a1559 = 1u64;
                                                nat_add(a1558, a1559)?
                                            }, (v4, f374(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1560 = v1;
                                                let a1561 = 1u64;
                                                nat_add(a1560, a1561)?
                                            }, (v4, Adt19::C0(false, false, f474(match v3.clone() {
                                                Adt19::C0(_, _, h1562, _, _) => {
                                                    h1562
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt19::C0(_, _, _, h1563, _) => {
                                                    h1563
                                                }
                                            }, match v3 {
                                                Adt19::C0(_, _, _, _, h1564) => {
                                                    h1564
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
                Ok(next) => { m1565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374(v0: Adt19) -> Adt19 {
    Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f475(List::cons(match v0.clone() {
        Adt19::C0(_, _, h1569, _, _) => {
            h1569
        }
    }, match v0.clone() {
        Adt19::C0(_, _, _, h1570, _) => {
            h1570
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt19::C0(_, _, _, _, h1571) => {
            h1571
        }
    }))
}

pub fn f375(p1574: List<List<Bytes>>, p1575: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1573 = (p1574, p1575);
    {
        let mut m1573 = m1573;
        loop {
            let step = {
                let (v0, v1) = m1573;
                let m1572 = v0;
                match m1572.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: Bytes, v1: List<Adt17>) -> List<Bytes> {
    let m1576 = v1;
    match m1576.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt17::C0(h1577, _, _) => {
                    h1577
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt17::C0(_, h1578, _) => {
                        h1578
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt17::C0(_, _, h1579) => {
                        h1579
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f376(v0, v3))
            } else {
                f376(v0, v3)
            }
        }
    }
}

pub fn f377(v0: Bytes, v1: List<Adt2>) -> List<Bytes> {
    let m1580 = v1;
    match m1580.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f377(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f377(v0, v3))
                } else {
                    f377(v0, v3)
                }
            }
        }
    }
}

pub fn f378(p1585: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1584 = p1585;
    {
        let mut m1584 = m1584;
        loop {
            let step = {
                let v0 = m1584;
                let m1581 = v0;
                match m1581.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1582 = v2;
                        match m1582.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1583 = v5;
                                    match m1583.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f476(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1584 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(v0: Adt14, v1: Adt2) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1586, _, _) => {
            h1586
        }
    }) {
        f52(f380(v1), match v0 {
            Adt14::C0(_, h1587, _) => {
                h1587
            }
        })
    } else {
        false
    }
}

pub fn f380(v0: Adt2) -> Bytes {
    let m1588 = f93(v0, f8(Str::lit("evidence")));
    match m1588 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f381(v0: Adt2) -> Bytes {
    if f221(v0.clone(), f8(Str::lit("req"))) {
        let a1589 = f8(Str::lit("req "));
        let a1590 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1589, a1590)
    } else {
        let m1591 = f93(v0.clone(), f8(Str::lit("std")));
        match m1591 {
            None => {
                let m1594 = f93(v0, f8(Str::lit("der")));
                match m1594 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f122(f215(f188(f93(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f221(v1.clone(), f8(Str::lit("row"))) {
                    let a1592 = f8(Str::lit(" row "));
                    let a1593 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1592, a1593)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f382(v0: Bytes, v1: Bytes) -> bool {
    let a1595 = v0;
    let a1596 = f397();
    if equal(a1595, a1596) {
        let a1600 = {
            let a1598 = {
                let a1597 = v1;
                length_bytes(a1597)
            };
            let a1599 = 0u64;
            nat_eq(a1598, a1599)
        };
        bool_not(a1600)
    } else {
        false
    }
}

pub fn f383(v0: Bytes) -> Bytes {
    f7(List::cons(f144(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f384(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1602 = v0;
    let a1603 = {
        let a1601 = Str::lit("redistributable");
        utf8_encode(a1601)
    };
    if equal(a1602, a1603) {
        f477(v1)
    } else {
        Ok(false)
    }
}

pub fn f385(v0: Adt2) -> bool {
    if f478(v0.clone(), f8(Str::lit("name"))) {
        if f479(v0.clone(), f8(Str::lit("schema"))) {
            let m1604 = f389(f215(f188(f93(v0, f8(Str::lit("citation"))))));
            match m1604 {
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

pub fn f386(v0: Bytes) -> R<Bytes> {
    f480(v0.clone(), 0u64, {
        let a1605 = v0;
        length_bytes(a1605)
    }, true, Bytes::lit(&[]))
}

pub fn f387(v0: Adt2) -> Bytes {
    if f221(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f221(v0.clone(), f8(Str::lit("id"))) {
            if f221(v0.clone(), f8(Str::lit("statement"))) {
                if f221(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f407(v0, f8(Str::lit("evidence")))
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

pub fn f388(v0: List<Bytes>) -> Adt2 {
    let m1606 = v0;
    match m1606.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f388(v2)))
        }
    }
}

pub fn f389(v0: List<Bytes>) -> Option<Adt2> {
    let m1607 = v0;
    match m1607.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1608 = v2;
                match m1608.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1609 = v2;
                    match m1609.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m1610 = v6;
                                match m1610.uncons() {
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
                        let m1611 = v2;
                        match m1611.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m1612 = v10;
                                match m1612.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f388(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
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

pub fn f390(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1613 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1613.uncons() {
        None => {
            let m1614 = f389(v4);
            match m1614 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1615 = f8(Str::lit("Catalog."));
                        let a1616 = f386(v3.clone())?;
                        append_bytes(a1615, a1616)
                    };
                    let m1617 = f481(v2, v9, f386(v3.clone())?)?;
                    match m1617 {
                        Ok(v11) => {
                            f404(v0, v3.clone(), v8, v11, f8(Str::lit("Encoding entered through propose-encoding.")), List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f386(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::<(Bytes, Bytes)>::nil()))
                        }
                        Err(v10) => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v10, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                    }
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f391(v0: Adt23) -> Bytes {
    let a1638 = f482(match v0.clone() {
        Adt23::C0(h1618, _, _, _, _, _, _, _) => {
            h1618
        }
    });
    let a1639 = {
        let a1636 = f482(match v0.clone() {
            Adt23::C0(_, h1619, _, _, _, _, _, _) => {
                h1619
            }
        });
        let a1637 = {
            let a1634 = f482(match v0.clone() {
                Adt23::C0(_, _, h1620, _, _, _, _, _) => {
                    h1620
                }
            });
            let a1635 = {
                let a1632 = f482(match v0.clone() {
                    Adt23::C0(_, _, _, h1621, _, _, _, _) => {
                        h1621
                    }
                });
                let a1633 = {
                    let a1630 = f482(match v0.clone() {
                        Adt23::C0(_, _, _, _, h1622, _, _, _) => {
                            h1622
                        }
                    });
                    let a1631 = {
                        let a1628 = f482(match v0.clone() {
                            Adt23::C0(_, _, _, _, _, h1623, _, _) => {
                                h1623
                            }
                        });
                        let a1629 = {
                            let a1626 = f482(match v0.clone() {
                                Adt23::C0(_, _, _, _, _, _, h1624, _) => {
                                    h1624
                                }
                            });
                            let a1627 = f482(match v0 {
                                Adt23::C0(_, _, _, _, _, _, _, h1625) => {
                                    h1625
                                }
                            });
                            append_bytes(a1626, a1627)
                        };
                        append_bytes(a1628, a1629)
                    };
                    append_bytes(a1630, a1631)
                };
                append_bytes(a1632, a1633)
            };
            append_bytes(a1634, a1635)
        };
        append_bytes(a1636, a1637)
    };
    append_bytes(a1638, a1639)
}

pub fn f392(v0: Bytes) -> R<Adt23> {
    let v1: u64 = {
        let a1640 = v0.clone();
        length_bytes(a1640)
    };
    let v2: u64 = {
        let a1646 = {
            let a1643 = {
                let a1641 = v1;
                let a1642 = 72u64;
                nat_add(a1641, a1642)?
            };
            let a1644 = 64u64;
            let a1645 = 0u64;
            nat_quot(a1643, a1644, a1645)
        };
        let a1647 = 64u64;
        nat_mul(a1646, a1647)?
    };
    let (_, h1648) = f483(Fn11::F484(v0, v1, v2), {
        let a1649 = v2;
        let a1650 = 64u64;
        let a1651 = 0u64;
        nat_quot(a1649, a1650, a1651)
    }, (0u64, f485()))?;
    Ok(h1648)
}

pub fn f393() -> Bytes {
    f8(Str::lit("target/release/verdant_readers"))
}

pub fn f394(v0: Bytes) -> bool {
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

pub fn f395() -> Bytes {
    f8(Str::lit("target/release/verdant_xml"))
}

pub fn f396(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Bytes) -> R<Adt12> {
    if f32(v4.clone(), f8(Str::lit("none"))) {
        f486(v0, v1, v2, v3, v4, v5, v6)
    } else {
        let m1652 = f91(v0.clone(), v2.clone())?;
        match m1652 {
            None => {
                Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
            }
            Some(v7) => {
                let v8: Bytes = {
                    let a1653 = f8(Str::lit("Catalog."));
                    let a1654 = v4.clone();
                    append_bytes(a1653, a1654)
                };
                if f51(f487(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, v8.clone(), v3.clone())) {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v3, List::cons(f8(Str::lit(" in model/admissions.json, so it cannot check the entries\u{a}")), List::<Bytes>::nil()))))))))
                } else {
                    let m1655 = f91(v0.clone(), f7(List::cons(f8(Str::lit("encodings/")), List::cons(v4.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))?;
                    match m1655 {
                        None => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: encodings/")), List::cons(v4, List::cons(f8(Str::lit(".schema.json is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v9) => {
                            let m1656 = f488(v9, v7)?;
                            match m1656 {
                                None => {
                                    f486(v0, v1, v2, v3, {
                                        let a1657 = f8(Str::lit("Catalog."));
                                        let a1658 = v4;
                                        append_bytes(a1657, a1658)
                                    }, v5, v6)
                                }
                                Some(v10) => {
                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" does not conform to ")), List::cons(v8, List::cons(f8(Str::lit(": ")), List::cons(v10, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f397() -> Bytes {
    let a1659 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1659)
}

pub fn f398(v0: Adt2, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt2> = f93(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f294(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f294(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f294(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f294(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f294(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f294(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f294(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f294(Str::lit("url"), f468(v2.clone(), f8(Str::lit("url"))))?, List::cons(f294(Str::lit("media_type"), f468(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f294(Str::lit("license"), f468(v2.clone(), f8(Str::lit("license"))))?, List::cons(f294(Str::lit("redistribution"), f468(v2, f8(Str::lit("redistribution"))))?, List::cons(f294(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1660 = v1;
        match m1660 {
            None => {
                f7(List::cons(f294(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f294(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f294(Str::lit("artifact"), {
                    let (h1661, _) = v3;
                    h1661
                })?, List::cons(f294(Str::lit("checksum"), {
                    let (_, h1662) = v3;
                    h1662
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f489(f215(f188(f93(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f399(p1665: List<List<Bytes>>, v1: Bytes, p1666: List<Bytes>) -> R<List<Bytes>> {
    let m1664 = (p1665, p1666);
    {
        let mut m1664 = m1664;
        loop {
            let step = {
                let (v0, v2) = m1664;
                let m1663 = v0;
                match m1663.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f490(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f401(f491(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f320(v0)?))
}

pub fn f401(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1667 = v0;
    match m1667.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1668 = v3.clone();
            match m1668.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1671 = v2;
                    let a1672 = {
                        let a1669 = v1.clone();
                        let a1670 = f401(v3, v1);
                        append_bytes(a1669, a1670)
                    };
                    append_bytes(a1671, a1672)
                }
            }
        }
    }
}

pub fn f402(v0: Adt2, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1673 = f93(v0, v1);
    match m1673 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v3) => {
            f188(f93(v3, v2))
        }
    }
}

pub fn f403(v0: Bytes) -> bool {
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

pub fn f404(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a1674 = f8(Str::lit("Catalog."));
        let a1675 = f386(v1.clone())?;
        append_bytes(a1674, a1675)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f386(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f492(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f100(Adt2::C7(std::rc::Rc::new(f101(f493(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f100(Adt2::C7(std::rc::Rc::new(f101(f493(f96(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("path")), std::rc::Rc::new(Adt2::C6(v7.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("role")), std::rc::Rc::new(Adt2::C6(v4)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("stratum")), std::rc::Rc::new(Adt2::C3(false, 0u64)), std::rc::Rc::new(Adt2::C9)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a1679 = f494(f64(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a1677 = {
                let a1676 = v7;
                length_bytes(a1676)
            };
            let a1678 = 4u64;
            nat_sub(a1677, a1678)
        }))?;
        let a1680 = f8(Str::lit("\u{a}"));
        append_bytes(a1679, a1680)
    }), List::cons(f284(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f405(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f386(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f406(v0: Bytes) -> R<Bytes> {
    f495({
        let a1682 = {
            let a1681 = v0.clone();
            length_bytes(a1681)
        };
        let a1683 = 1u64;
        nat_add(a1682, a1683)?
    }, v0)
}

pub fn f407(v0: Adt2, v1: Bytes) -> bool {
    let m1684 = f93(v0, v1);
    match m1684 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f408(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1686 = {
                let a1685 = v0;
                length_bytes(a1685)
            };
            let a1687 = 1u64;
            nat_sub(a1686, a1687)
        })
    } else {
        v0
    }
}

pub fn f409(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1688 = v0;
    match m1688 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v8, r1689) => {
            let v9 = (*r1689).clone();
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
            Adt2::C6(f496(f60(), v2, v1))
        }
        Adt2::C7(r1690) => {
            let v6 = (*r1690).clone();
            Adt2::C7(std::rc::Rc::new(v6))
        }
        Adt2::C8(r1691) => {
            let v7 = (*r1691).clone();
            Adt2::C8(std::rc::Rc::new(v7))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1692, r1693) => {
            let v15 = (*r1692).clone();
            let v16 = (*r1693).clone();
            Adt2::C10(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt2::C11(v17, r1694, r1695) => {
            let v18 = (*r1694).clone();
            let v19 = (*r1695).clone();
            Adt2::C11(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
    }
}

pub fn f410(v0: u64) -> R<Bytes> {
    f497(f498(), 10u64, v0)
}

pub fn f411(v0: bool) -> R<u64> {
    f413({
        let a1696 = f418(v0);
        let a1697 = 1u64;
        nat_sub(a1696, a1697)
    })
}

pub fn f412(v0: bool) -> R<u64> {
    f413(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f413(v0: u64) -> R<u64> {
    let m1698 = v0;
    if m1698 == 0 {
        Ok(1u64)
    } else {
        let v1 = m1698 - 1;
        let a1699 = 2u64;
        let a1700 = f413(v1)?;
        nat_mul(a1699, a1700)
    }
}

pub fn f414(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f415(v0: bool) -> R<u64> {
    let a1701 = f413(f414(v0))?;
    let a1702 = 1u64;
    Ok(nat_sub(a1701, a1702))
}

pub fn f416(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a1703 = Bytes::lit(&[0x2d]);
        let a1704 = v1;
        append_bytes(a1703, a1704)
    } else {
        v1
    }
}

pub fn f417(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f418(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f419(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f499(f500(v2, f501(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f499(f500(v2, f501(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a1705 = v3.clone();
            length_bytes(a1705)
        };
        let v5: Bytes = {
            let a1706 = v2;
            let a1707 = v4;
            if nat_lt(a1706, a1707) {
                f46(v3.clone(), 0u64, {
                    let a1708 = v4;
                    let a1709 = v2;
                    nat_sub(a1708, a1709)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f502({
            let a1710 = v2;
            let a1711 = v4;
            if nat_lt(a1710, a1711) {
                f46(v3, {
                    let a1712 = v4;
                    let a1713 = v2;
                    nat_sub(a1712, a1713)
                }, v2)
            } else {
                let a1716 = f503({
                    let a1714 = v2;
                    let a1715 = v4;
                    nat_sub(a1714, a1715)
                });
                let a1717 = v3;
                append_bytes(a1716, a1717)
            }
        }, {
            let a1718 = v2;
            let a1719 = 1u64;
            nat_add(a1718, a1719)?
        })?;
        let a1721 = {
            let a1720 = v6.clone();
            length_bytes(a1720)
        };
        let a1722 = 0u64;
        if nat_eq(a1721, a1722) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f420(v0: Bytes, p1741: u64, p1742: u64, p1743: Bytes) -> R<Bytes> {
    let m1740 = (p1741, (p1742, p1743));
    {
        let mut m1740 = m1740;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1740;
                let m1723 = v2;
                if m1723 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1723 - 1;
                    let v5: u64 = f200(v0.clone(), v1, {
                        let a1727 = {
                            let a1725 = {
                                let a1724 = v0.clone();
                                length_bytes(a1724)
                            };
                            let a1726 = v1;
                            nat_sub(a1725, a1726)
                        };
                        let a1728 = 1u64;
                        nat_add(a1727, a1728)?
                    })?;
                    let v6: Bytes = {
                        let a1731 = v3;
                        let a1732 = f46(v0.clone(), v1, {
                            let a1729 = v5;
                            let a1730 = v1;
                            nat_sub(a1729, a1730)
                        });
                        append_bytes(a1731, a1732)
                    };
                    let m1735 = {
                        let a1733 = v0.clone();
                        let a1734 = v5;
                        index_bytes(a1733, a1734)
                    };
                    match m1735 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a1736 = v5;
                                let a1737 = 1u64;
                                nat_add(a1736, a1737)?
                            }, (v4, {
                                let a1738 = v6;
                                let a1739 = f504(v0.clone(), v5, v7)?;
                                append_bytes(a1738, a1739)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1740 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: Fn5, p1747: List<(Bytes, Bytes)>, p1748: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1746 = (p1747, p1748);
    {
        let mut m1746 = m1746;
        loop {
            let step = {
                let (v1, v2) = m1746;
                let m1744 = v2;
                match m1744.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1745: Fn5 = v0.clone();
                            c1745.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1746 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f422(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f505(v0, v1)
}

pub fn f423(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f506(v0)
}

pub fn f424(v0: Fn6, p1752: List<Bytes>, p1753: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1751 = (p1752, p1753);
    {
        let mut m1751 = m1751;
        loop {
            let step = {
                let (v1, v2) = m1751;
                let m1749 = v2;
                match m1749.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1750: Fn6 = v0.clone();
                            c1750.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1751 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f507(v1)?, v0))
}

pub fn f426(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f508(v0)?, List::cons(f8(Str::lit(":")), List::cons(f508(v1)?, List::<Bytes>::nil())))))
}

pub fn f427(v0: Adt13) -> Bytes {
    let m1754 = v0;
    match m1754 {
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

pub fn f428(v0: Adt13) -> Bytes {
    let m1755 = v0;
    match m1755 {
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

pub fn f429(v0: Adt13) -> Bytes {
    let m1756 = v0;
    match m1756 {
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

pub fn f430(v0: Bytes, v1: u64) -> Option<u64> {
    let a1757 = 0u64;
    let a1758 = v1;
    if nat_lt(a1757, a1758) {
        if f509(v0, {
            let a1759 = v1;
            let a1760 = 1u64;
            nat_sub(a1759, a1760)
        }) {
            Some({
                let a1761 = v1;
                let a1762 = 1u64;
                nat_sub(a1761, a1762)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f431(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1763 = v2;
    let a1764 = v0;
    if nat_lt(a1763, a1764) {
        if f509(v1, v2) {
            Ok(Some({
                let a1765 = v2;
                let a1766 = 1u64;
                nat_add(a1765, a1766)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f432(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1769 = f46(v0.clone(), {
        let a1767 = v1;
        let a1768 = 2u64;
        nat_add(a1767, a1768)?
    }, 2u64);
    let a1770 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1769, a1770) {
        let m1773 = f510(f46(v0, {
            let a1771 = v1;
            let a1772 = 4u64;
            nat_add(a1771, a1772)?
        }, 2u64))?;
        match m1773 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f115(v2.clone(), 0u64)?;
                let a1774 = v3;
                let a1775 = 32u64;
                if nat_lt(a1774, a1775) {
                    if f511(v3) {
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

pub fn f433(v0: u64) -> Bytes {
    let a1776 = v0;
    let a1777 = 34u64;
    if nat_eq(a1776, a1777) {
        Bytes::lit(&[0x22])
    } else {
        let a1778 = v0;
        let a1779 = 92u64;
        if nat_eq(a1778, a1779) {
            Bytes::lit(&[0x5c])
        } else {
            let a1780 = v0;
            let a1781 = 98u64;
            if nat_eq(a1780, a1781) {
                Bytes::lit(&[0x08])
            } else {
                let a1782 = v0;
                let a1783 = 102u64;
                if nat_eq(a1782, a1783) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1784 = v0;
                    let a1785 = 110u64;
                    if nat_eq(a1784, a1785) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1786 = v0;
                        let a1787 = 114u64;
                        if nat_eq(a1786, a1787) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1788 = v0;
                            let a1789 = 116u64;
                            if nat_eq(a1788, a1789) {
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

pub fn f434(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f115(v0, v1)?;
    let a1790 = 47u64;
    let a1791 = v2;
    if if nat_lt(a1790, a1791) {
        let a1792 = v2;
        let a1793 = 58u64;
        nat_lt(a1792, a1793)
    } else {
        false
    } {
        Ok(Some({
            let a1794 = v1;
            let a1795 = 1u64;
            nat_add(a1794, a1795)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f435(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1796 = f115(v0, v1)?;
    let a1797 = 48u64;
    if nat_eq(a1796, a1797) {
        Ok(Some({
            let a1798 = v1;
            let a1799 = 1u64;
            nat_add(a1798, a1799)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f436(v0: Bytes) -> R<u64> {
    let (h1800, _) = f323(Fn3::F512(v0.clone()), {
        let a1801 = v0;
        length_bytes(a1801)
    }, 0u64)?;
    Ok(h1800)
}

pub fn f437(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1805 = {
        let a1803 = v0;
        let a1804 = {
            let (h1802, _) = v1;
            h1802
        };
        index_bytes(a1803, a1804)
    };
    match m1805 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1807 = {
                    let (h1806, _) = v1;
                    h1806
                };
                let a1808 = 1u64;
                nat_add(a1807, a1808)?
            }, {
                let a1812 = {
                    let a1810 = {
                        let (_, h1809) = v1;
                        h1809
                    };
                    let a1811 = 10u64;
                    nat_mul(a1810, a1811)?
                };
                let a1813 = f513(v2)?;
                nat_add(a1812, a1813)?
            })))
        }
    }
}

pub fn f438(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f514(v0, v1)
}

pub fn f439(v0: (Bytes, Bytes), p1816: List<(Bytes, Bytes)>) -> bool {
    let m1815 = p1816;
    {
        let mut m1815 = m1815;
        loop {
            let step = {
                let v1 = m1815;
                let m1814 = v1;
                match m1814.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f515(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1817 = v1;
    match m1817.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1818, _) = v2;
                h1818
            }, v0.clone()) {
                List::cons({
                    let (_, h1819) = v2;
                    h1819
                }, f440(v0, v3))
            } else {
                f440(v0, v3)
            }
        }
    }
}

pub fn f441() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f442(v0: Adt2, v1: Bytes, v2: Bytes) -> bool {
    let m1820 = f93(v0, v1);
    match m1820 {
        None => {
            false
        }
        Some(v3) => {
            f221(v3, v2)
        }
    }
}

pub fn f443() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f444(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f445(v0: Bytes) -> R<bool> {
    let v1: Bytes = f198(v0)?;
    let v2: u64 = if f85(v1.clone(), f8(Str::lit("if ! "))) {
        5u64
    } else {
        if f85(v1.clone(), f8(Str::lit("elif ! "))) {
            7u64
        } else {
            0u64
        }
    };
    let v3: Bytes = f46(v1.clone(), v2, {
        let a1822 = {
            let a1821 = v1;
            length_bytes(a1821)
        };
        let a1823 = v2;
        nat_sub(a1822, a1823)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1824 = f342(v3.clone(), f8(Str::lit(")")))?;
        match m1824 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1825 = v4;
                    let a1826 = 1u64;
                    nat_add(a1825, a1826)?
                }, {
                    let a1830 = {
                        let a1827 = v3;
                        length_bytes(a1827)
                    };
                    let a1831 = {
                        let a1828 = v4;
                        let a1829 = 1u64;
                        nat_add(a1828, a1829)?
                    };
                    nat_sub(a1830, a1831)
                })
            }
        }
    } else {
        v3
    };
    let a1832 = 0u64;
    let a1833 = v2;
    if nat_lt(a1832, a1833) {
        f516(v5)
    } else {
        Ok(false)
    }
}

pub fn f446(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f447(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1836 = {
            let a1834 = v4;
            let a1835 = v1;
            nat_add(a1834, a1835)?
        };
        let a1837 = v3;
        if nat_lt(a1836, a1837) {
            Ok(Some({
                let a1838 = v4;
                let a1839 = 1u64;
                nat_add(a1838, a1839)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f447(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1844 = {
        let a1841 = v0;
        let a1842 = v2;
        let a1843 = {
            let a1840 = v1.clone();
            length_bytes(a1840)
        };
        slice_bytes(a1841, a1842, a1843)
    };
    match m1844 {
        None => {
            false
        }
        Some(v3) => {
            let a1845 = v3;
            let a1846 = v1;
            equal(a1845, a1846)
        }
    }
}

pub fn f448(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1847 = v0;
    match m1847.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f448(v3, {
                let a1848 = v1;
                let a1849 = 1u64;
                nat_add(a1848, a1849)?
            })?))
        }
    }
}

pub fn f449(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1850 = v2;
    match m1850 {
        Adt21::C0(v3, v4) => {
            if f455(v3, match v0.clone() {
                Adt18::C0(h1851, _, _, _) => {
                    h1851
                }
            }) {
                f517(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f455(v5, match v0 {
                Adt18::C0(_, _, h1852, _) => {
                    h1852
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f451(v7, v1) {
                f455(v8, match v0 {
                    Adt18::C0(_, _, _, h1853) => {
                        h1853
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f450(v0: Adt15, v1: Adt2) -> R<Adt21> {
    let m1854 = f93(v1.clone(), f8(Str::lit("std")));
    match m1854 {
        None => {
            let m1859 = f92(f93(v1.clone(), f8(Str::lit("req"))));
            match m1859 {
                None => {
                    let m1861 = f93(v1, f8(Str::lit("der")));
                    match m1861 {
                        None => {
                            Ok(f518(v0))
                        }
                        Some(v5) => {
                            let m1862 = f519(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1862 {
                                None => {
                                    Ok(f518(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f350(f215(f188(f93(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1863) => {
                                            h1863
                                        }
                                    })?, f453(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1864, _) => {
                                            h1864
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f453(v4, match v0 {
                        Adt15::C0(_, _, h1860, _, _) => {
                            h1860
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1855 = f92(f93(v2.clone(), f8(Str::lit("row"))));
            match m1855 {
                None => {
                    Ok(Adt21::C0(f453(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1856, _, _, _, _) => {
                            h1856
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f453(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1857, _, _, _, _) => {
                            h1857
                        }
                    })?, Some(f453(v3, match v0 {
                        Adt15::C0(_, h1858, _, _, _) => {
                            h1858
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f451(p1867: List<u64>, v1: List<u64>) -> bool {
    let m1866 = p1867;
    {
        let mut m1866 = m1866;
        loop {
            let step = {
                let v0 = m1866;
                let m1865 = v0;
                match m1865.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f455(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1866 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(p1872: List<u64>) -> bool {
    let m1871 = p1872;
    {
        let mut m1871 = m1871;
        loop {
            let step = {
                let v0 = m1871;
                let m1868 = v0;
                match m1868.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1869 = v1;
                        let a1870 = 7u64;
                        if nat_le(a1869, a1870) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1873 = f520(v0, v1.clone(), 0u64)?;
    match m1873 {
        None => {
            let a1874 = v1;
            Ok(length_list(a1874))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f454(v0: Adt2) -> u64 {
    let m1875 = f521(f93(v0, f8(Str::lit("stratum"))));
    match m1875 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f455(v0: u64, p1880: List<u64>) -> bool {
    let m1879 = p1880;
    {
        let mut m1879 = m1879;
        loop {
            let step = {
                let v1 = m1879;
                let m1876 = v1;
                match m1876.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1877 = v2;
                        let a1878 = v0;
                        if nat_eq(a1877, a1878) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1879 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: Fn7, p1884: List<Bytes>, p1885: List<(Bytes, ())>) -> List<Bytes> {
    let m1883 = (p1884, p1885);
    {
        let mut m1883 = m1883;
        loop {
            let step = {
                let (v1, v2) = m1883;
                let m1881 = v2;
                match m1881.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1882: Fn7 = v0.clone();
                            c1882.apply(v1, v3)
                        }, v4))
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

pub fn f457(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1886, _) = v1;
        h1886
    }, v0)
}

pub fn f458(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f522(Fn12::F523, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f459(v0: Fn8, p1890: List<List<(Bytes, ())>>, p1891: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1889 = (p1890, p1891);
    {
        let mut m1889 = m1889;
        loop {
            let step = {
                let (v1, v2) = m1889;
                let m1887 = v2;
                match m1887.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1888: Fn8 = v0.clone();
                            c1888.apply(v1, v3)
                        }, v4))
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

pub fn f460(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f461(v0: Fn9, p1896: u64, p1897: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1895 = (p1896, p1897);
    {
        let mut m1895 = m1895;
        loop {
            let step = {
                let (v1, v2) = m1895;
                let m1892 = v1;
                if m1892 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1892 - 1;
                    let m1894 = {
                        let c1893: Fn9 = v0.clone();
                        c1893.apply(v2.clone())?
                    };
                    match m1894 {
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
                Ok(next) => { m1895 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f524(v0)
}

pub fn f463(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f525(Fn13::F526, List::<(Bytes, ())>::nil(), v0)
}

pub fn f464(v0: Fn10, p1901: List<(Bytes, ())>, p1902: List<Bytes>) -> List<(Bytes, ())> {
    let m1900 = (p1901, p1902);
    {
        let mut m1900 = m1900;
        loop {
            let step = {
                let (v1, v2) = m1900;
                let m1898 = v2;
                match m1898.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1899: Fn10 = v0.clone();
                            c1899.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f466(p1907: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1906 = p1907;
    {
        let mut m1906 = m1906;
        loop {
            let step = {
                let v0 = m1906;
                let m1903 = v0;
                match m1903.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1904, _) = v2;
                            h1904
                        }) {
                            let (_, h1905) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1905)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
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

pub fn f467(v0: Bytes, p1933: u64, p1934: u64, p1935: Bytes) -> R<Bytes> {
    let m1932 = (p1933, (p1934, p1935));
    {
        let mut m1932 = m1932;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1932;
                let m1908 = v2;
                if m1908 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1908 - 1;
                    let m1911 = {
                        let a1909 = v0.clone();
                        let a1910 = v1;
                        index_bytes(a1909, a1910)
                    };
                    match m1911 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f203({
                                let a1912 = v6;
                                let a1913 = 34u64;
                                nat_eq(a1912, a1913)
                            }, {
                                let a1914 = v6;
                                let a1915 = 92u64;
                                nat_eq(a1914, a1915)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1916 = v1;
                                    let a1917 = 1u64;
                                    nat_add(a1916, a1917)?
                                }, (v4, {
                                    let a1920 = v3;
                                    let a1921 = {
                                        let a1918 = f8(Str::lit("\\"));
                                        let a1919 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1918, a1919)
                                    };
                                    append_bytes(a1920, a1921)
                                })))
                            } else {
                                let a1922 = v6;
                                let a1923 = 10u64;
                                if nat_eq(a1922, a1923) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1924 = v1;
                                        let a1925 = 1u64;
                                        nat_add(a1924, a1925)?
                                    }, (v4, {
                                        let a1926 = v3;
                                        let a1927 = f8(Str::lit("\\n"));
                                        append_bytes(a1926, a1927)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1928 = v1;
                                        let a1929 = 1u64;
                                        nat_add(a1928, a1929)?
                                    }, (v4, {
                                        let a1930 = v3;
                                        let a1931 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1930, a1931)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1932 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f468(v0: Option<Adt2>, v1: Bytes) -> Bytes {
    let m1936 = v0;
    match m1936 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f469(v0: List<Adt2>) -> List<Bytes> {
    let m1937 = v0;
    match m1937.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f527(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f287(f58(v1, f8(Str::lit("authority")))), f469(v2)))
        }
    }
}

pub fn f470(v0: List<Bytes>) -> R<List<Adt2>> {
    let m1938 = f300(v0.clone())?;
    match m1938 {
        None => {
            Ok(f278(f277(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt2>::nil())
        }
    }
}

pub fn f471(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    let m1939 = f300(v3.clone())?;
    match m1939 {
        None => {
            let m1940 = f277(v3.clone())?;
            match m1940 {
                None => {
                    f530(v0, v1, v3)
                }
                Some(v5) => {
                    f529(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f528(v0, v1, v4)
        }
    }
}

pub fn f472(v0: List<Bytes>) -> List<Bytes> {
    let m1941 = v0;
    match m1941.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1943 = {
                    let a1942 = v1.clone();
                    length_bytes(a1942)
                };
                let a1944 = 0u64;
                if nat_eq(a1943, a1944) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f472(v2))
        }
    }
}

pub fn f473(v0: Bytes, v1: u64) -> R<u64> {
    let m1947 = {
        let a1945 = v0;
        let a1946 = v1;
        index_bytes(a1945, a1946)
    };
    match m1947 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f474(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1951 = {
        let a1948 = v1;
        let a1949 = v2;
        let a1950 = 1u64;
        slice_bytes(a1948, a1949, a1950)
    };
    match m1951 {
        None => {
            v0
        }
        Some(v3) => {
            let a1952 = v0;
            let a1953 = v3;
            append_bytes(a1952, a1953)
        }
    }
}

pub fn f475(p1956: List<Bytes>, p1957: List<Bytes>) -> List<Bytes> {
    let m1955 = (p1956, p1957);
    {
        let mut m1955 = m1955;
        loop {
            let step = {
                let (v0, v1) = m1955;
                let m1954 = v0;
                match m1954.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(v0: List<Bytes>) -> Bytes {
    let m1958 = v0;
    match m1958.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f477(v0: Bytes) -> R<bool> {
    let m1960 = {
        let a1959 = v0.clone();
        length_bytes(a1959)
    };
    if m1960 == 0 {
        Ok(true)
    } else {
        let v1 = m1960 - 1;
        let m1963 = {
            let a1961 = v0;
            let a1962 = v1;
            index_bytes(a1961, a1962)
        };
        match m1963 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f531({
                    let v3: u8 = v2;
                    let a2008 = {
                        let a1984 = {
                            let a1972 = {
                                let a1966 = {
                                    let a1964 = v3;
                                    let a1965 = 128u8;
                                    fixed_u8::bit_and(a1964, a1965)
                                };
                                let a1967 = 128u8;
                                if equal(a1966, a1967) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1973 = {
                                let a1970 = {
                                    let a1968 = v3;
                                    let a1969 = 64u8;
                                    fixed_u8::bit_and(a1968, a1969)
                                };
                                let a1971 = 64u8;
                                if equal(a1970, a1971) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1972, a1973)?
                        };
                        let a1985 = {
                            let a1982 = {
                                let a1976 = {
                                    let a1974 = v3;
                                    let a1975 = 32u8;
                                    fixed_u8::bit_and(a1974, a1975)
                                };
                                let a1977 = 32u8;
                                if equal(a1976, a1977) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1983 = {
                                let a1980 = {
                                    let a1978 = v3;
                                    let a1979 = 16u8;
                                    fixed_u8::bit_and(a1978, a1979)
                                };
                                let a1981 = 16u8;
                                if equal(a1980, a1981) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1982, a1983)?
                        };
                        nat_add(a1984, a1985)?
                    };
                    let a2009 = {
                        let a2006 = {
                            let a1994 = {
                                let a1988 = {
                                    let a1986 = v3;
                                    let a1987 = 8u8;
                                    fixed_u8::bit_and(a1986, a1987)
                                };
                                let a1989 = 8u8;
                                if equal(a1988, a1989) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1995 = {
                                let a1992 = {
                                    let a1990 = v3;
                                    let a1991 = 4u8;
                                    fixed_u8::bit_and(a1990, a1991)
                                };
                                let a1993 = 4u8;
                                if equal(a1992, a1993) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1994, a1995)?
                        };
                        let a2007 = {
                            let a2004 = {
                                let a1998 = {
                                    let a1996 = v3;
                                    let a1997 = 2u8;
                                    fixed_u8::bit_and(a1996, a1997)
                                };
                                let a1999 = 2u8;
                                if equal(a1998, a1999) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2005 = {
                                let a2002 = {
                                    let a2000 = v3;
                                    let a2001 = 1u8;
                                    fixed_u8::bit_and(a2000, a2001)
                                };
                                let a2003 = 1u8;
                                if equal(a2002, a2003) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2004, a2005)?
                        };
                        nat_add(a2006, a2007)?
                    };
                    nat_add(a2008, a2009)?
                }))
            }
        }
    }
}

pub fn f478(v0: Adt2, v1: Bytes) -> bool {
    let a2011 = {
        let a2010 = f58(v0, v1);
        length_bytes(a2010)
    };
    let a2012 = 0u64;
    !nat_eq(a2011, a2012)
}

pub fn f479(v0: Adt2, v1: Bytes) -> bool {
    let m2013 = f93(v0, v1);
    match m2013 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f480(v0: Bytes, p2034: u64, p2035: u64, p2036: bool, p2037: Bytes) -> R<Bytes> {
    let m2033 = (p2034, (p2035, (p2036, p2037)));
    {
        let mut m2033 = m2033;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2033;
                let m2014 = v2;
                if m2014 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m2014 - 1;
                    let m2017 = {
                        let a2015 = v0.clone();
                        let a2016 = v1;
                        index_bytes(a2015, a2016)
                    };
                    match m2017 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f532(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2018 = v1;
                                    let a2019 = 1u64;
                                    nat_add(a2018, a2019)?
                                }, (v5, (true, v4))))
                            } else {
                                if f117(f533(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2020 = v1;
                                        let a2021 = 1u64;
                                        nat_add(a2020, a2021)?
                                    }, (v5, (false, {
                                        let a2025 = v4;
                                        let a2026 = f46({
                                            let a2022 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2022)
                                        }, {
                                            let a2023 = v7;
                                            let a2024 = 97u64;
                                            nat_sub(a2023, a2024)
                                        }, 1u64);
                                        append_bytes(a2025, a2026)
                                    }))))
                                } else {
                                    if f203(f533(v7), f203(f534(v7), f535(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2027 = v1;
                                            let a2028 = 1u64;
                                            nat_add(a2027, a2028)?
                                        }, (v5, (false, {
                                            let a2029 = v4;
                                            let a2030 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a2029, a2030)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2031 = v1;
                                            let a2032 = 1u64;
                                            nat_add(a2031, a2032)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2033 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f536(f537(f93(v0.clone(), f8(Str::lit("$defs")))), f537(f93(v0.clone(), f8(Str::lit("definitions")))));
    let m2038 = f538(4096u64, v3.clone(), List::<Bytes>::nil(), f539(v3)?)?;
    match m2038 {
        Ok(v5) => {
            let m2039 = f540(4096u64, f541(f542(v5)?, List::cons(Adt24::C0(v2, v0), List::<Adt24>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m2039 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f401(f243(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f482(v0: u32) -> Bytes {
    let a2068 = f543({
        let a2040 = f544(v0, 28u32);
        let a2041 = 15u32;
        fixed_u32::bit_and(a2040, a2041)
    });
    let a2069 = {
        let a2066 = f543({
            let a2042 = f544(v0, 24u32);
            let a2043 = 15u32;
            fixed_u32::bit_and(a2042, a2043)
        });
        let a2067 = {
            let a2064 = f543({
                let a2044 = f544(v0, 20u32);
                let a2045 = 15u32;
                fixed_u32::bit_and(a2044, a2045)
            });
            let a2065 = {
                let a2062 = f543({
                    let a2046 = f544(v0, 16u32);
                    let a2047 = 15u32;
                    fixed_u32::bit_and(a2046, a2047)
                });
                let a2063 = {
                    let a2060 = f543({
                        let a2048 = f544(v0, 12u32);
                        let a2049 = 15u32;
                        fixed_u32::bit_and(a2048, a2049)
                    });
                    let a2061 = {
                        let a2058 = f543({
                            let a2050 = f544(v0, 8u32);
                            let a2051 = 15u32;
                            fixed_u32::bit_and(a2050, a2051)
                        });
                        let a2059 = {
                            let a2056 = f543({
                                let a2052 = f544(v0, 4u32);
                                let a2053 = 15u32;
                                fixed_u32::bit_and(a2052, a2053)
                            });
                            let a2057 = f543({
                                let a2054 = v0;
                                let a2055 = 15u32;
                                fixed_u32::bit_and(a2054, a2055)
                            });
                            append_bytes(a2056, a2057)
                        };
                        append_bytes(a2058, a2059)
                    };
                    append_bytes(a2060, a2061)
                };
                append_bytes(a2062, a2063)
            };
            append_bytes(a2064, a2065)
        };
        append_bytes(a2066, a2067)
    };
    append_bytes(a2068, a2069)
}

pub fn f483(v0: Fn11, p2073: u64, p2074: (u64, Adt23)) -> R<(u64, Adt23)> {
    let m2072 = (p2073, p2074);
    {
        let mut m2072 = m2072;
        loop {
            let step = {
                let (v1, v2) = m2072;
                let m2070 = v1;
                if m2070 == 0 {
                    Err::<(u64, (u64, Adt23)), R<(u64, Adt23)>>(Ok(v2))
                } else {
                    let v3 = m2070 - 1;
                    Ok::<(u64, (u64, Adt23)), R<(u64, Adt23)>>((v3, {
                        let c2071: Fn11 = v0.clone();
                        c2071.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2072 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt23)) -> R<(u64, Adt23)> {
    Ok(({
        let a2076 = {
            let (h2075, _) = v3.clone();
            h2075
        };
        let a2077 = 64u64;
        nat_add(a2076, a2077)?
    }, f545({
        let (_, h2078) = v3;
        h2078
    }, f546(v0, v1, v2, {
        let (h2079, _) = v3;
        h2079
    })?)?))
}

pub fn f485() -> Adt23 {
    Adt23::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f486(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Bytes) -> R<Adt12> {
    let m2080 = f547(f548(v1.clone(), v3.clone(), v4, v5.clone(), v6))?;
    match m2080 {
        Ok(v8) => {
            let v9: Bytes = f527(v1.clone());
            Ok(Adt12::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f100(Adt2::C7(std::rc::Rc::new(f101(f493(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v9.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("row")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f388(v5)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C6(f288(f64(v0, v2))?)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(Adt2::C9)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f487(p2084: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m2083 = p2084;
    {
        let mut m2083 = m2083;
        loop {
            let step = {
                let v0 = m2083;
                let m2081 = v0;
                match m2081.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f117(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2082 = f93(v3, f8(Str::lit("std")));
                            match m2082 {
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
                Ok(next) => { m2083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: Adt2, v1: Adt2) -> R<Option<Bytes>> {
    f549({
        let a2091 = {
            let a2086 = {
                let a2085 = f550(v0.clone())?;
                length_bytes(a2085)
            };
            let a2087 = 2u64;
            nat_add(a2086, a2087)?
        };
        let a2092 = {
            let a2089 = {
                let a2088 = f550(v1.clone())?;
                length_bytes(a2088)
            };
            let a2090 = 2u64;
            nat_add(a2089, a2090)?
        };
        nat_mul(a2091, a2092)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f489(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f401(f551(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f490(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2093 = f44(v1, 34u8)?;
    match m2093.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2094 = v3;
            match m2094.uncons() {
                None => {
                    Ok(f552(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f491(v0: List<Bytes>) -> List<Bytes> {
    let m2095 = v0;
    match m2095.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f491(v2)
            } else {
                List::cons(v1, f491(v2))
            }
        }
    }
}

pub fn f492(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2096 = v0;
    match m2096.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f492(v3, v1))
        }
    }
}

pub fn f493(v0: List<Adt2>, v1: Bytes, v2: Bytes, v3: Adt2) -> List<Adt2> {
    let m2097 = v0;
    match m2097.uncons() {
        None => {
            List::cons(v3, List::<Adt2>::nil())
        }
        Some((v4, v5)) => {
            if f32(f58(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f553(v5, v1, v2))
            } else {
                List::cons(v4, f493(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f494(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f401(f554(f95(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f495(p2108: u64, p2109: Bytes) -> R<Bytes> {
    let m2107 = (p2108, p2109);
    {
        let mut m2107 = m2107;
        loop {
            let step = {
                let (v0, v1) = m2107;
                let m2098 = v0;
                if m2098 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m2098 - 1;
                    if f85(v1.clone(), f8(Str::lit("-- "))) {
                        let m2099 = f342(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m2099 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f46(v1.clone(), {
                                    let a2100 = v3;
                                    let a2101 = 1u64;
                                    nat_add(a2100, a2101)?
                                }, {
                                    let a2105 = {
                                        let a2102 = v1;
                                        length_bytes(a2102)
                                    };
                                    let a2106 = {
                                        let a2103 = v3;
                                        let a2104 = 1u64;
                                        nat_add(a2103, a2104)?
                                    };
                                    nat_sub(a2105, a2106)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m2107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f496(p2112: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2111 = p2112;
    {
        let mut m2111 = m2111;
        loop {
            let step = {
                let v0 = m2111;
                let m2110 = v0;
                match m2110.uncons() {
                    None => {
                        Err::<List<Adt13>, Bytes>(v1.clone())
                    }
                    Some((v3, v4)) => {
                        if f85(v1.clone(), f102(v3.clone())) {
                            Err::<List<Adt13>, Bytes>(f69(v3, v2.clone()))
                        } else {
                            Ok::<List<Adt13>, Bytes>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2113 = v2;
    let a2114 = 0u64;
    if nat_eq(a2113, a2114) {
        Ok(f555(v0, 0u64))
    } else {
        let (_, h2115) = {
            let (h2116, _) = f556(Fn14::F557(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2116
        };
        Ok(h2115)
    }
}

pub fn f498() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f499(v0: List<u64>) -> R<Bytes> {
    let m2117 = f558(v0, List::<u64>::nil());
    match m2117.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2118 = f410(v1)?;
            let a2119 = f559(v2)?;
            Ok(append_bytes(a2118, a2119))
        }
    }
}

pub fn f500(p2122: u64, p2123: List<u64>, v2: u64) -> R<List<u64>> {
    let m2121 = (p2122, p2123);
    {
        let mut m2121 = m2121;
        loop {
            let step = {
                let (v0, v1) = m2121;
                let m2120 = v0;
                if m2120 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2120 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f560(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(v0: u64, v1: u64) -> List<u64> {
    let m2124 = v0;
    if m2124 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2124 - 1;
        let a2125 = v1;
        let a2126 = 0u64;
        if nat_eq(a2125, a2126) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2127 = v1;
                let a2128 = f561();
                let a2129 = 0u64;
                nat_rem(a2127, a2128, a2129)
            }, f501(v2, {
                let a2130 = v1;
                let a2131 = f561();
                let a2132 = 0u64;
                nat_quot(a2130, a2131, a2132)
            }))
        }
    }
}

pub fn f502(p2143: Bytes, p2144: u64) -> R<Bytes> {
    let m2142 = (p2143, p2144);
    {
        let mut m2142 = m2142;
        loop {
            let step = {
                let (v0, v1) = m2142;
                let m2133 = v1;
                if m2133 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2133 - 1;
                    let a2137 = f115(v0.clone(), {
                        let a2135 = {
                            let a2134 = v0.clone();
                            length_bytes(a2134)
                        };
                        let a2136 = 1u64;
                        nat_sub(a2135, a2136)
                    })?;
                    let a2138 = 48u64;
                    if nat_eq(a2137, a2138) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2140 = {
                                let a2139 = v0;
                                length_bytes(a2139)
                            };
                            let a2141 = 1u64;
                            nat_sub(a2140, a2141)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: u64) -> Bytes {
    let m2145 = v0;
    if m2145 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2145 - 1;
        let a2146 = Bytes::lit(&[0x30]);
        let a2147 = f503(v1);
        append_bytes(a2146, a2147)
    }
}

pub fn f504(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2192 = {
            let a2168 = {
                let a2156 = {
                    let a2150 = {
                        let a2148 = v3;
                        let a2149 = 128u8;
                        fixed_u8::bit_and(a2148, a2149)
                    };
                    let a2151 = 128u8;
                    if equal(a2150, a2151) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2157 = {
                    let a2154 = {
                        let a2152 = v3;
                        let a2153 = 64u8;
                        fixed_u8::bit_and(a2152, a2153)
                    };
                    let a2155 = 64u8;
                    if equal(a2154, a2155) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2156, a2157)?
            };
            let a2169 = {
                let a2166 = {
                    let a2160 = {
                        let a2158 = v3;
                        let a2159 = 32u8;
                        fixed_u8::bit_and(a2158, a2159)
                    };
                    let a2161 = 32u8;
                    if equal(a2160, a2161) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2167 = {
                    let a2164 = {
                        let a2162 = v3;
                        let a2163 = 16u8;
                        fixed_u8::bit_and(a2162, a2163)
                    };
                    let a2165 = 16u8;
                    if equal(a2164, a2165) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2166, a2167)?
            };
            nat_add(a2168, a2169)?
        };
        let a2193 = {
            let a2190 = {
                let a2178 = {
                    let a2172 = {
                        let a2170 = v3;
                        let a2171 = 8u8;
                        fixed_u8::bit_and(a2170, a2171)
                    };
                    let a2173 = 8u8;
                    if equal(a2172, a2173) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2179 = {
                    let a2176 = {
                        let a2174 = v3;
                        let a2175 = 4u8;
                        fixed_u8::bit_and(a2174, a2175)
                    };
                    let a2177 = 4u8;
                    if equal(a2176, a2177) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2178, a2179)?
            };
            let a2191 = {
                let a2188 = {
                    let a2182 = {
                        let a2180 = v3;
                        let a2181 = 2u8;
                        fixed_u8::bit_and(a2180, a2181)
                    };
                    let a2183 = 2u8;
                    if equal(a2182, a2183) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2189 = {
                    let a2186 = {
                        let a2184 = v3;
                        let a2185 = 1u8;
                        fixed_u8::bit_and(a2184, a2185)
                    };
                    let a2187 = 1u8;
                    if equal(a2186, a2187) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2188, a2189)?
            };
            nat_add(a2190, a2191)?
        };
        nat_add(a2192, a2193)?
    };
    let a2194 = v4;
    let a2195 = 34u64;
    if nat_eq(a2194, a2195) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2196 = v4;
        let a2197 = 92u64;
        if nat_eq(a2196, a2197) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2198 = v4;
            let a2199 = 8u64;
            if nat_eq(a2198, a2199) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2200 = v4;
                let a2201 = 12u64;
                if nat_eq(a2200, a2201) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2202 = v4;
                    let a2203 = 10u64;
                    if nat_eq(a2202, a2203) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2204 = v4;
                        let a2205 = 13u64;
                        if nat_eq(a2204, a2205) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2206 = v4;
                            let a2207 = 9u64;
                            if nat_eq(a2206, a2207) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2208 = v4;
                                let a2209 = 32u64;
                                if nat_lt(a2208, a2209) {
                                    let a2210 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2211 = f562(v2)?;
                                    Ok(append_bytes(a2210, a2211))
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

pub fn f505(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2212 = v0.clone();
    match m2212.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2215 = {
                let (h2213, _) = v2;
                h2213
            };
            let a2216 = {
                let (h2214, _) = v1.clone();
                h2214
            };
            if equal(a2215, a2216) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f506(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f563(f564(Fn15::F565, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2221 = {
        let (h2217, _) = f566(Fn16::F567, {
            let a2219 = {
                let a2218 = v0;
                length_list(a2218)
            };
            let a2220 = 1u64;
            nat_add(a2219, a2220)?
        }, v1)?;
        h2217
    };
    match m2221.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f507(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2226 = {
        let a2223 = f311({
            let (h2222, _) = v0;
            h2222
        })?;
        let a2224 = Bytes::lit(&[0x3a]);
        append_bytes(a2223, a2224)
    };
    let a2227 = {
        let (_, h2225) = v0;
        h2225
    };
    Ok(append_bytes(a2226, a2227))
}

pub fn f508(v0: Bytes) -> R<Bytes> {
    let a2233 = {
        let a2231 = Bytes::lit(&[0x22]);
        let a2232 = {
            let (_, h2228) = {
                let (h2229, _) = f556(Fn14::F568(v0.clone()), {
                    let a2230 = v0;
                    length_bytes(a2230)
                }, (0u64, Bytes::lit(&[])))?;
                h2229
            };
            h2228
        };
        append_bytes(a2231, a2232)
    };
    let a2234 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2233, a2234))
}

pub fn f509(v0: Bytes, v1: u64) -> bool {
    let m2235 = f208(v0, v1);
    match m2235 {
        None => {
            false
        }
        Some(v2) => {
            f569(v2)
        }
    }
}

pub fn f510(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2236 = v0.clone();
        length_bytes(a2236)
    };
    let a2240 = {
        let a2237 = v1;
        let a2238 = 2u64;
        let a2239 = 0u64;
        nat_rem(a2237, a2238, a2239)
    };
    let a2241 = 0u64;
    if nat_eq(a2240, a2241) {
        let (_, h2242) = {
            let (h2243, _) = f570(Fn17::F571(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2243
        };
        Ok(h2242)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f511(v0: u64) -> bool {
    let a2244 = v0;
    let a2245 = 8u64;
    if nat_eq(a2244, a2245) {
        true
    } else {
        let a2246 = v0;
        let a2247 = 9u64;
        if nat_eq(a2246, a2247) {
            true
        } else {
            let a2248 = v0;
            let a2249 = 10u64;
            if nat_eq(a2248, a2249) {
                true
            } else {
                let a2250 = v0;
                let a2251 = 12u64;
                if nat_eq(a2250, a2251) {
                    true
                } else {
                    let a2252 = v0;
                    let a2253 = 13u64;
                    nat_eq(a2252, a2253)
                }
            }
        }
    }
}

pub fn f512(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2256 = {
        let a2254 = v0;
        let a2255 = v1;
        index_bytes(a2254, a2255)
    };
    match m2256 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f572(v2)? {
                Ok(Some({
                    let a2257 = v1;
                    let a2258 = 1u64;
                    nat_add(a2257, a2258)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f513(v0: u8) -> R<u64> {
    let a2305 = {
        let v1: u8 = v0;
        let a2303 = {
            let a2279 = {
                let a2267 = {
                    let a2261 = {
                        let a2259 = v1;
                        let a2260 = 128u8;
                        fixed_u8::bit_and(a2259, a2260)
                    };
                    let a2262 = 128u8;
                    if equal(a2261, a2262) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2268 = {
                    let a2265 = {
                        let a2263 = v1;
                        let a2264 = 64u8;
                        fixed_u8::bit_and(a2263, a2264)
                    };
                    let a2266 = 64u8;
                    if equal(a2265, a2266) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2267, a2268)?
            };
            let a2280 = {
                let a2277 = {
                    let a2271 = {
                        let a2269 = v1;
                        let a2270 = 32u8;
                        fixed_u8::bit_and(a2269, a2270)
                    };
                    let a2272 = 32u8;
                    if equal(a2271, a2272) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2278 = {
                    let a2275 = {
                        let a2273 = v1;
                        let a2274 = 16u8;
                        fixed_u8::bit_and(a2273, a2274)
                    };
                    let a2276 = 16u8;
                    if equal(a2275, a2276) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2277, a2278)?
            };
            nat_add(a2279, a2280)?
        };
        let a2304 = {
            let a2301 = {
                let a2289 = {
                    let a2283 = {
                        let a2281 = v1;
                        let a2282 = 8u8;
                        fixed_u8::bit_and(a2281, a2282)
                    };
                    let a2284 = 8u8;
                    if equal(a2283, a2284) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2290 = {
                    let a2287 = {
                        let a2285 = v1;
                        let a2286 = 4u8;
                        fixed_u8::bit_and(a2285, a2286)
                    };
                    let a2288 = 4u8;
                    if equal(a2287, a2288) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2289, a2290)?
            };
            let a2302 = {
                let a2299 = {
                    let a2293 = {
                        let a2291 = v1;
                        let a2292 = 2u8;
                        fixed_u8::bit_and(a2291, a2292)
                    };
                    let a2294 = 2u8;
                    if equal(a2293, a2294) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2300 = {
                    let a2297 = {
                        let a2295 = v1;
                        let a2296 = 1u8;
                        fixed_u8::bit_and(a2295, a2296)
                    };
                    let a2298 = 1u8;
                    if equal(a2297, a2298) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2299, a2300)?
            };
            nat_add(a2301, a2302)?
        };
        nat_add(a2303, a2304)?
    };
    let a2306 = 48u64;
    Ok(nat_sub(a2305, a2306))
}

pub fn f514(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2307 = v0.clone();
        length_bytes(a2307)
    };
    let a2308 = v1;
    let a2309 = v2;
    if if nat_lt(a2308, a2309) {
        let a2314 = f115(v0, {
            let a2312 = v2;
            let a2313 = {
                let a2310 = v1;
                let a2311 = 1u64;
                nat_add(a2310, a2311)?
            };
            nat_sub(a2312, a2313)
        })?;
        let a2315 = 48u64;
        nat_eq(a2314, a2315)
    } else {
        false
    } {
        Ok(Some({
            let a2316 = v1;
            let a2317 = 1u64;
            nat_add(a2316, a2317)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f515(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f117(f32({
        let (h2318, _) = v0;
        h2318
    }, {
        let (h2319, _) = v1;
        h2319
    }), f32({
        let (_, h2320) = v0;
        h2320
    }, {
        let (_, h2321) = v1;
        h2321
    }))
}

pub fn f516(v0: Bytes) -> R<bool> {
    let m2322 = f342(v0.clone(), f8(Str::lit("&&")))?;
    if match m2322 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2323 = f342(v0, f8(Str::lit("||")))?;
        match m2323 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f517(v0: Adt18, v1: Option<u64>) -> bool {
    let m2324 = v1;
    match m2324 {
        None => {
            true
        }
        Some(v2) => {
            f455(v2, match v0 {
                Adt18::C0(_, h2325, _, _) => {
                    h2325
                }
            })
        }
    }
}

pub fn f518(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a2327 = match v0 {
            Adt15::C0(_, _, h2326, _, _) => {
                h2326
            }
        };
        length_list(a2327)
    })
}

pub fn f519(v0: Bytes) -> Option<Adt22> {
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

pub fn f520(v0: Bytes, p2332: List<Bytes>, p2333: u64) -> R<Option<u64>> {
    let m2331 = (p2332, p2333);
    {
        let mut m2331 = m2331;
        loop {
            let step = {
                let (v1, v2) = m2331;
                let m2328 = v1;
                match m2328.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2329 = v2;
                                let a2330 = 1u64;
                                nat_add(a2329, a2330)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: Option<Adt2>) -> Option<u64> {
    let m2334 = v0;
    match m2334 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2335 = v1;
            match m2335 {
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

pub fn f522(v0: Fn12, p2339: List<List<(Bytes, ())>>, p2340: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2338 = (p2339, p2340);
    {
        let mut m2338 = m2338;
        loop {
            let step = {
                let (v1, v2) = m2338;
                let m2336 = v2;
                match m2336.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2337: Fn12 = v0.clone();
                            c2337.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f524(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2342 = {
        let a2341 = v0.clone();
        length_list(a2341)
    };
    let a2343 = 1u64;
    if nat_le(a2342, a2343) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f573(v0)?))
    }
}

pub fn f525(v0: Fn13, p2347: List<(Bytes, ())>, p2348: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2346 = (p2347, p2348);
    {
        let mut m2346 = m2346;
        loop {
            let step = {
                let (v1, v2) = m2346;
                let m2344 = v2;
                match m2344.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2345: Fn13 = v0.clone();
                            c2345.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f527(v0: Bytes) -> Bytes {
    f7(List::cons(f139(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f528(v0: Adt1, v1: Bytes, v2: List<Adt2>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f142(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f142(f96(v0, f153())?, f8(Str::lit("at"))));
    let a2350 = {
        let a2349 = v4.clone();
        length_list(a2349)
    };
    let a2351 = 0u64;
    if nat_eq(a2350, a2351) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2354 = {
            let a2352 = v4;
            length_list(a2352)
        };
        let a2355 = {
            let a2353 = v3;
            length_list(a2353)
        };
        if nat_eq(a2354, a2355) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f574(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f529(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt2>) -> R<(bool, Bytes)> {
    let m2356 = v3;
    match m2356.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f203(f575(v0.clone(), v6.clone())?, f52(v6.clone(), v2.clone())) {
                f529(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f287(v6.clone()))) {
                    let v9: (bool, Bytes) = f529(v0, v1.clone(), v2, v5)?;
                    let (h2357, _) = v9.clone();
                    if h2357 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2359 = f574(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2360 = {
                                let (_, h2358) = v9;
                                h2358
                            };
                            append_bytes(a2359, a2360)
                        }))
                    }
                } else {
                    if f117(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f487(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2361 = f8(Str::lit("Catalog."));
                        let a2362 = v8.clone();
                        append_bytes(a2361, a2362)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f529(v0, v1.clone(), v2, v5)?;
                        let (h2363, _) = v10.clone();
                        if h2363 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2365 = f574(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2366 = {
                                    let (_, h2364) = v10;
                                    h2364
                                };
                                append_bytes(a2365, a2366)
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

pub fn f530(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt2> = f278(f280(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt2> = f278(f280(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f125(f142(v3.clone(), f8(Str::lit("id"))), f142(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f125(v6.clone(), f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2368 = {
        let a2367 = v7.clone();
        length_list(a2367)
    };
    let a2369 = 0u64;
    if nat_eq(a2368, a2369) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2372 = {
                let a2370 = v7;
                length_list(a2370)
            };
            let a2373 = {
                let a2371 = v5;
                length_list(a2371)
            };
            nat_eq(a2372, a2373)
        }) {
            Ok((false, f574(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f145(f125(f62(f576(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f577(v4), f125(v6, f142(v3, f8(Str::lit("id")))))))?;
            let a2375 = {
                let a2374 = v8.clone();
                length_list(a2374)
            };
            let a2376 = 0u64;
            if nat_eq(a2375, a2376) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f574(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f122(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f531(v0: u64) -> bool {
    let a2377 = v0;
    let a2378 = 47u64;
    nat_eq(a2377, a2378)
}

pub fn f532(v0: u64) -> bool {
    f203({
        let a2379 = v0;
        let a2380 = 95u64;
        nat_eq(a2379, a2380)
    }, f203({
        let a2381 = v0;
        let a2382 = 45u64;
        nat_eq(a2381, a2382)
    }, f203({
        let a2383 = v0;
        let a2384 = 32u64;
        nat_eq(a2383, a2384)
    }, f203({
        let a2385 = v0;
        let a2386 = 46u64;
        nat_eq(a2385, a2386)
    }, {
        let a2387 = v0;
        let a2388 = 47u64;
        nat_eq(a2387, a2388)
    }))))
}

pub fn f533(v0: u64) -> bool {
    f117({
        let a2389 = 97u64;
        let a2390 = v0;
        nat_le(a2389, a2390)
    }, {
        let a2391 = v0;
        let a2392 = 122u64;
        nat_le(a2391, a2392)
    })
}

pub fn f534(v0: u64) -> bool {
    f117({
        let a2393 = 65u64;
        let a2394 = v0;
        nat_le(a2393, a2394)
    }, {
        let a2395 = v0;
        let a2396 = 90u64;
        nat_le(a2395, a2396)
    })
}

pub fn f535(v0: u64) -> bool {
    f117({
        let a2397 = 48u64;
        let a2398 = v0;
        nat_le(a2397, a2398)
    }, {
        let a2399 = v0;
        let a2400 = 57u64;
        nat_le(a2399, a2400)
    })
}

pub fn f536(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2401 = v0;
    match m2401.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f536(v3, v1))
        }
    }
}

pub fn f537(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2402 = v0;
    match m2402 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f578(v1)
        }
    }
}

pub fn f538(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2403 = v0;
    if m2403 == 0 {
        let m2404 = v1;
        match m2404.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2403 - 1;
        let m2405 = v1.clone();
        match m2405.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2406 = f579(v1.clone(), v2.clone(), v3.clone())?;
                match m2406 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f401(f539(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2409 = f538(v6, f580(v1, {
                            let (h2407, _) = v9.clone();
                            h2407
                        }), List::cons(f581({
                            let (h2408, _) = v9.clone();
                            h2408
                        })?, v2), v3)?;
                        match m2409 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f539(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2410 = v0;
    match m2410.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f581({
                let (h2411, _) = v1;
                h2411
            })?, f539(v2)?))
        }
    }
}

pub fn f540(p2417: u64, p2418: List<Adt24>, p2419: List<Bytes>, p2420: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2416 = (p2417, (p2418, (p2419, p2420)));
    {
        let mut m2416 = m2416;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2416;
                let m2412 = v0;
                if m2412 == 0 {
                    Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2412 - 1;
                    let m2413 = v1;
                    match m2413.uncons() {
                        None => {
                            Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2414 = v5;
                            match m2414 {
                                Adt24::C0(v8, v9) => {
                                    if f32(v8.clone(), f582()) {
                                        Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f52(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2415 = f584(v8.clone(), v9)?;
                                            match m2415 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f541(f585(v11.clone()), List::cons(Adt24::C1(f586(v11)), v6)), (List::cons(v8, v2), v3))))
                                                }
                                                Err(v10) => {
                                                    Err::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt24::C1(v7) => {
                                    Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                                Adt24::C2 => {
                                    if f52(f582(), v2.clone()) {
                                        Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt24>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f582(), v2), List::cons(f583(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2416 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f541(v0: List<Adt24>, v1: List<Adt24>) -> List<Adt24> {
    let m2421 = v0;
    match m2421.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f541(v3, v1))
        }
    }
}

pub fn f542(v0: List<(Bytes, Adt2)>) -> R<List<Adt24>> {
    let m2422 = v0;
    match m2422.uncons() {
        None => {
            Ok(List::<Adt24>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt24::C0(f581({
                let (h2423, _) = v1;
                h2423
            })?, {
                let (_, h2424) = v1;
                h2424
            }), f542(v2)?))
        }
    }
}

pub fn f543(v0: u32) -> Bytes {
    let a2427 = {
        let a2425 = v0;
        let a2426 = 8u32;
        fixed_u32::bit_and(a2425, a2426)
    };
    let a2428 = 0u32;
    if equal(a2427, a2428) {
        let a2431 = {
            let a2429 = v0;
            let a2430 = 4u32;
            fixed_u32::bit_and(a2429, a2430)
        };
        let a2432 = 0u32;
        if equal(a2431, a2432) {
            let a2435 = {
                let a2433 = v0;
                let a2434 = 2u32;
                fixed_u32::bit_and(a2433, a2434)
            };
            let a2436 = 0u32;
            if equal(a2435, a2436) {
                let a2439 = {
                    let a2437 = v0;
                    let a2438 = 1u32;
                    fixed_u32::bit_and(a2437, a2438)
                };
                let a2440 = 0u32;
                if equal(a2439, a2440) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2443 = {
                    let a2441 = v0;
                    let a2442 = 1u32;
                    fixed_u32::bit_and(a2441, a2442)
                };
                let a2444 = 0u32;
                if equal(a2443, a2444) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2447 = {
                let a2445 = v0;
                let a2446 = 2u32;
                fixed_u32::bit_and(a2445, a2446)
            };
            let a2448 = 0u32;
            if equal(a2447, a2448) {
                let a2451 = {
                    let a2449 = v0;
                    let a2450 = 1u32;
                    fixed_u32::bit_and(a2449, a2450)
                };
                let a2452 = 0u32;
                if equal(a2451, a2452) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2455 = {
                    let a2453 = v0;
                    let a2454 = 1u32;
                    fixed_u32::bit_and(a2453, a2454)
                };
                let a2456 = 0u32;
                if equal(a2455, a2456) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2459 = {
            let a2457 = v0;
            let a2458 = 4u32;
            fixed_u32::bit_and(a2457, a2458)
        };
        let a2460 = 0u32;
        if equal(a2459, a2460) {
            let a2463 = {
                let a2461 = v0;
                let a2462 = 2u32;
                fixed_u32::bit_and(a2461, a2462)
            };
            let a2464 = 0u32;
            if equal(a2463, a2464) {
                let a2467 = {
                    let a2465 = v0;
                    let a2466 = 1u32;
                    fixed_u32::bit_and(a2465, a2466)
                };
                let a2468 = 0u32;
                if equal(a2467, a2468) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2471 = {
                    let a2469 = v0;
                    let a2470 = 1u32;
                    fixed_u32::bit_and(a2469, a2470)
                };
                let a2472 = 0u32;
                if equal(a2471, a2472) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2475 = {
                let a2473 = v0;
                let a2474 = 2u32;
                fixed_u32::bit_and(a2473, a2474)
            };
            let a2476 = 0u32;
            if equal(a2475, a2476) {
                let a2479 = {
                    let a2477 = v0;
                    let a2478 = 1u32;
                    fixed_u32::bit_and(a2477, a2478)
                };
                let a2480 = 0u32;
                if equal(a2479, a2480) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2483 = {
                    let a2481 = v0;
                    let a2482 = 1u32;
                    fixed_u32::bit_and(a2481, a2482)
                };
                let a2484 = 0u32;
                if equal(a2483, a2484) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f544(v0: u32, v1: u32) -> u32 {
    let m2487 = {
        let a2485 = v0;
        let a2486 = v1;
        fixed_u32::shift_right(a2485, a2486)
    };
    match m2487 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f545(v0: Adt23, v1: Adt25) -> R<Adt23> {
    let v2: Adt26 = f587(Fn18::F588, 64u64, Adt26::C0(match v0.clone() {
        Adt23::C0(h2488, _, _, _, _, _, _, _) => {
            h2488
        }
    }, match v0.clone() {
        Adt23::C0(_, h2489, _, _, _, _, _, _) => {
            h2489
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2490, _, _, _, _, _) => {
            h2490
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h2491, _, _, _, _) => {
            h2491
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, h2492, _, _, _) => {
            h2492
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2493, _, _) => {
            h2493
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2494, _) => {
            h2494
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h2495) => {
            h2495
        }
    }, v1, 0u64))?;
    Ok(Adt23::C0(f589(match v0.clone() {
        Adt23::C0(h2496, _, _, _, _, _, _, _) => {
            h2496
        }
    }, match v2.clone() {
        Adt26::C0(h2497, _, _, _, _, _, _, _, _, _) => {
            h2497
        }
    }), f589(match v0.clone() {
        Adt23::C0(_, h2498, _, _, _, _, _, _) => {
            h2498
        }
    }, match v2.clone() {
        Adt26::C0(_, h2499, _, _, _, _, _, _, _, _) => {
            h2499
        }
    }), f589(match v0.clone() {
        Adt23::C0(_, _, h2500, _, _, _, _, _) => {
            h2500
        }
    }, match v2.clone() {
        Adt26::C0(_, _, h2501, _, _, _, _, _, _, _) => {
            h2501
        }
    }), f589(match v0.clone() {
        Adt23::C0(_, _, _, h2502, _, _, _, _) => {
            h2502
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, h2503, _, _, _, _, _, _) => {
            h2503
        }
    }), f589(match v0.clone() {
        Adt23::C0(_, _, _, _, h2504, _, _, _) => {
            h2504
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, h2505, _, _, _, _, _) => {
            h2505
        }
    }), f589(match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2506, _, _) => {
            h2506
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, _, h2507, _, _, _, _) => {
            h2507
        }
    }), f589(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2508, _) => {
            h2508
        }
    }, match v2.clone() {
        Adt26::C0(_, _, _, _, _, _, h2509, _, _, _) => {
            h2509
        }
    }), f589(match v0 {
        Adt23::C0(_, _, _, _, _, _, _, h2510) => {
            h2510
        }
    }, match v2 {
        Adt26::C0(_, _, _, _, _, _, _, h2511, _, _) => {
            h2511
        }
    })))
}

pub fn f546(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt25> {
    Ok(Adt25::C0(f590(v0.clone(), v1, v2, v3)?, f590(v0.clone(), v1, v2, {
        let a2512 = v3;
        let a2513 = 4u64;
        nat_add(a2512, a2513)?
    })?, f590(v0.clone(), v1, v2, {
        let a2514 = v3;
        let a2515 = 8u64;
        nat_add(a2514, a2515)?
    })?, f590(v0.clone(), v1, v2, {
        let a2516 = v3;
        let a2517 = 12u64;
        nat_add(a2516, a2517)?
    })?, f590(v0.clone(), v1, v2, {
        let a2518 = v3;
        let a2519 = 16u64;
        nat_add(a2518, a2519)?
    })?, f590(v0.clone(), v1, v2, {
        let a2520 = v3;
        let a2521 = 20u64;
        nat_add(a2520, a2521)?
    })?, f590(v0.clone(), v1, v2, {
        let a2522 = v3;
        let a2523 = 24u64;
        nat_add(a2522, a2523)?
    })?, f590(v0.clone(), v1, v2, {
        let a2524 = v3;
        let a2525 = 28u64;
        nat_add(a2524, a2525)?
    })?, f590(v0.clone(), v1, v2, {
        let a2526 = v3;
        let a2527 = 32u64;
        nat_add(a2526, a2527)?
    })?, f590(v0.clone(), v1, v2, {
        let a2528 = v3;
        let a2529 = 36u64;
        nat_add(a2528, a2529)?
    })?, f590(v0.clone(), v1, v2, {
        let a2530 = v3;
        let a2531 = 40u64;
        nat_add(a2530, a2531)?
    })?, f590(v0.clone(), v1, v2, {
        let a2532 = v3;
        let a2533 = 44u64;
        nat_add(a2532, a2533)?
    })?, f590(v0.clone(), v1, v2, {
        let a2534 = v3;
        let a2535 = 48u64;
        nat_add(a2534, a2535)?
    })?, f590(v0.clone(), v1, v2, {
        let a2536 = v3;
        let a2537 = 52u64;
        nat_add(a2536, a2537)?
    })?, f590(v0.clone(), v1, v2, {
        let a2538 = v3;
        let a2539 = 56u64;
        nat_add(a2538, a2539)?
    })?, f590(v0, v1, v2, {
        let a2540 = v3;
        let a2541 = 60u64;
        nat_add(a2540, a2541)?
    })?))
}

pub fn f547(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m2542 = f591(v0.clone());
    match m2542 {
        None => {
            let m2543 = f592(v0.clone())?;
            match m2543 {
                None => {
                    Ok(Ok::<Bytes, Bytes>(f190(v0)?))
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

pub fn f548(v0: Bytes, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: Bytes) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("artifact")), std::rc::Rc::new(Adt2::C6(v4)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v0.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("citation")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f593(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("pointer")), std::rc::Rc::new(Adt2::C6(Bytes::lit(&[]))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("reader")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f593(v3)))), std::rc::Rc::new(Adt2::C9))))))))))))))
}

pub fn f549(v0: u64, v1: Adt2, v2: Adt2, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2544 = v0;
    if m2544 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m2544 - 1;
        let m2545 = v2.clone();
        match m2545 {
            Adt2::C0 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C1(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(None::<Bytes>)
                } else {
                    Ok(f594(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
                }
            }
            Adt2::C3(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C4(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C5(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C6(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C7(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C8(r2559) => {
                let v7 = (*r2559).clone();
                let m2546 = f595(f578(v7));
                match m2546 {
                    None => {
                        Ok(f596(f597(v2.clone(), v3.clone(), v4.clone())?, f596({
                            let m2547 = f92(f93(v2.clone(), f8(Str::lit("$ref"))));
                            match m2547 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m2548 = f598(v1.clone(), v9)?;
                                    match m2548 {
                                        Ok(v11) => {
                                            f549(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f596(f599(v5, v1.clone(), f188(f93(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f596({
                            let m2549 = f93(v2.clone(), f8(Str::lit("anyOf")));
                            match m2549 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a2550 = 0u64;
                                    let a2551 = f600(v5, v1.clone(), f188(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a2550, a2551) {
                                        None::<Bytes>
                                    } else {
                                        f594(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f596({
                            let m2552 = f93(v2.clone(), f8(Str::lit("oneOf")));
                            match m2552 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a2553 = f600(v5, v1.clone(), f188(Some(v13)), v3.clone(), v4.clone())?;
                                    let a2554 = 1u64;
                                    if nat_eq(a2553, a2554) {
                                        None::<Bytes>
                                    } else {
                                        f594(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m2555 = v3.clone();
                            match m2555 {
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
                                Adt2::C6(_) => {
                                    None::<Bytes>
                                }
                                Adt2::C7(r2557) => {
                                    let v15 = (*r2557).clone();
                                    let m2556 = f93(v2, f8(Str::lit("items")));
                                    match m2556 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f603(v5, v1, v16, f306(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt2::C8(r2558) => {
                                    let v14 = (*r2558).clone();
                                    f596(f601(v5, v1.clone(), f537(f93(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f602(v5, v1, v2, f578(v14), v4)?)
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
                        }))))))
                    }
                    Some(v8) => {
                        Ok(Some(f7(List::cons(f8(Str::lit("the keyword ")), List::cons(v8, List::cons(f8(Str::lit(" is not checked by conforms, so the instance is not admitted (JSON Schema 2020-12 Core section 4.3.1)")), List::<Bytes>::nil()))))))
                    }
                }
            }
            Adt2::C9 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C10(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C11(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
        }
    }
}

pub fn f550(v0: Adt2) -> R<Bytes> {
    f190(v0)
}

pub fn f551(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2560 = v0;
    match m2560.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f360(v1)?, f551(v2)?))
        }
    }
}

pub fn f552(p2563: List<Bytes>, v1: Bytes) -> bool {
    let m2562 = p2563;
    {
        let mut m2562 = m2562;
        loop {
            let step = {
                let v0 = m2562;
                let m2561 = v0;
                match m2561.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f85(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m2564 = v0;
    match m2564.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f32(f58(v3.clone(), v1.clone()), v2.clone()) {
                f553(v4, v1, v2)
            } else {
                List::cons(v3, f553(v4, v1, v2))
            }
        }
    }
}

pub fn f554(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2565 = v0;
    match m2565.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f401(f551(f145(List::cons(v1, f604(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f554(v3, v1)?))
            }
        }
    }
}

pub fn f555(v0: Bytes, v1: u64) -> Bytes {
    let m2569 = {
        let a2566 = v0;
        let a2567 = v1;
        let a2568 = 1u64;
        slice_bytes(a2566, a2567, a2568)
    };
    match m2569 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f556(v0: Fn14, p2574: u64, p2575: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m2573 = (p2574, p2575);
    {
        let mut m2573 = m2573;
        loop {
            let step = {
                let (v1, v2) = m2573;
                let m2570 = v1;
                if m2570 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2570 - 1;
                    let m2572 = {
                        let c2571: Fn14 = v0.clone();
                        c2571.apply(v2.clone())?
                    };
                    match m2572 {
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
                Ok(next) => { m2573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f557(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f605(v1, v0, v2)
}

pub fn f558(p2578: List<u64>, p2579: List<u64>) -> List<u64> {
    let m2577 = (p2578, p2579);
    {
        let mut m2577 = m2577;
        loop {
            let step = {
                let (v0, v1) = m2577;
                let m2576 = v0;
                match m2576.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f559(v0: List<u64>) -> R<Bytes> {
    let m2580 = v0;
    match m2580.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a2581 = f606(v1)?;
            let a2582 = f559(v2)?;
            Ok(append_bytes(a2581, a2582))
        }
    }
}

pub fn f560(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m2583 = v0;
    match m2583.uncons() {
        None => {
            let a2584 = v2;
            let a2585 = 0u64;
            if nat_eq(a2584, a2585) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a2588 = {
                    let a2586 = v3;
                    let a2587 = v1;
                    nat_mul(a2586, a2587)?
                };
                let a2589 = v2;
                nat_add(a2588, a2589)?
            };
            Ok(List::cons({
                let a2590 = v5;
                let a2591 = f561();
                let a2592 = 0u64;
                nat_rem(a2590, a2591, a2592)
            }, f560(v4, v1, {
                let a2593 = v5;
                let a2594 = f561();
                let a2595 = 0u64;
                nat_quot(a2593, a2594, a2595)
            })?))
        }
    }
}

pub fn f561() -> u64 {
    1000000000u64
}

pub fn f562(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2640 = {
            let a2616 = {
                let a2604 = {
                    let a2598 = {
                        let a2596 = v1;
                        let a2597 = 128u8;
                        fixed_u8::bit_and(a2596, a2597)
                    };
                    let a2599 = 128u8;
                    if equal(a2598, a2599) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2605 = {
                    let a2602 = {
                        let a2600 = v1;
                        let a2601 = 64u8;
                        fixed_u8::bit_and(a2600, a2601)
                    };
                    let a2603 = 64u8;
                    if equal(a2602, a2603) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2604, a2605)?
            };
            let a2617 = {
                let a2614 = {
                    let a2608 = {
                        let a2606 = v1;
                        let a2607 = 32u8;
                        fixed_u8::bit_and(a2606, a2607)
                    };
                    let a2609 = 32u8;
                    if equal(a2608, a2609) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2615 = {
                    let a2612 = {
                        let a2610 = v1;
                        let a2611 = 16u8;
                        fixed_u8::bit_and(a2610, a2611)
                    };
                    let a2613 = 16u8;
                    if equal(a2612, a2613) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2614, a2615)?
            };
            nat_add(a2616, a2617)?
        };
        let a2641 = {
            let a2638 = {
                let a2626 = {
                    let a2620 = {
                        let a2618 = v1;
                        let a2619 = 8u8;
                        fixed_u8::bit_and(a2618, a2619)
                    };
                    let a2621 = 8u8;
                    if equal(a2620, a2621) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2627 = {
                    let a2624 = {
                        let a2622 = v1;
                        let a2623 = 4u8;
                        fixed_u8::bit_and(a2622, a2623)
                    };
                    let a2625 = 4u8;
                    if equal(a2624, a2625) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2626, a2627)?
            };
            let a2639 = {
                let a2636 = {
                    let a2630 = {
                        let a2628 = v1;
                        let a2629 = 2u8;
                        fixed_u8::bit_and(a2628, a2629)
                    };
                    let a2631 = 2u8;
                    if equal(a2630, a2631) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2637 = {
                    let a2634 = {
                        let a2632 = v1;
                        let a2633 = 1u8;
                        fixed_u8::bit_and(a2632, a2633)
                    };
                    let a2635 = 1u8;
                    if equal(a2634, a2635) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2636, a2637)?
            };
            nat_add(a2638, a2639)?
        };
        nat_add(a2640, a2641)?
    };
    let a2648 = f555(f607(), {
        let a2642 = v2;
        let a2643 = 16u64;
        let a2644 = 0u64;
        nat_quot(a2642, a2643, a2644)
    });
    let a2649 = f555(f607(), {
        let a2645 = v2;
        let a2646 = 16u64;
        let a2647 = 0u64;
        nat_rem(a2645, a2646, a2647)
    });
    Ok(append_bytes(a2648, a2649))
}

pub fn f563(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f608(Fn19::F609, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f564(v0: Fn15, p2653: List<List<(Bytes, Bytes)>>, p2654: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2652 = (p2653, p2654);
    {
        let mut m2652 = m2652;
        loop {
            let step = {
                let (v1, v2) = m2652;
                let m2650 = v2;
                match m2650.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2651: Fn15 = v0.clone();
                            c2651.apply(v1, v3)
                        }, v4))
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

pub fn f565(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f566(v0: Fn16, p2659: u64, p2660: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2658 = (p2659, p2660);
    {
        let mut m2658 = m2658;
        loop {
            let step = {
                let (v1, v2) = m2658;
                let m2655 = v1;
                if m2655 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2655 - 1;
                    let m2657 = {
                        let c2656: Fn16 = v0.clone();
                        c2656.apply(v2.clone())?
                    };
                    match m2657 {
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
                Ok(next) => { m2658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f567(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f610(v0)
}

pub fn f568(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f611(v0, v1)
}

pub fn f569(v0: u8) -> bool {
    let a2661 = v0;
    let a2662 = 32u8;
    if if if equal(a2661, a2662) {
        true
    } else {
        let a2663 = v0;
        let a2664 = 9u8;
        equal(a2663, a2664)
    } {
        true
    } else {
        let a2665 = v0;
        let a2666 = 10u8;
        equal(a2665, a2666)
    } {
        true
    } else {
        let a2667 = v0;
        let a2668 = 12u8;
        if equal(a2667, a2668) {
            true
        } else {
            let a2669 = v0;
            let a2670 = 13u8;
            equal(a2669, a2670)
        }
    }
}

pub fn f570(v0: Fn17, p2675: u64, p2676: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2674 = (p2675, p2676);
    {
        let mut m2674 = m2674;
        loop {
            let step = {
                let (v1, v2) = m2674;
                let m2671 = v1;
                if m2671 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2671 - 1;
                    let m2673 = {
                        let c2672: Fn17 = v0.clone();
                        c2672.apply(v2.clone())?
                    };
                    match m2673 {
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
                Ok(next) => { m2674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f571(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f612(v0, v1)
}

pub fn f572(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2721 = {
            let a2697 = {
                let a2685 = {
                    let a2679 = {
                        let a2677 = v1;
                        let a2678 = 128u8;
                        fixed_u8::bit_and(a2677, a2678)
                    };
                    let a2680 = 128u8;
                    if equal(a2679, a2680) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2686 = {
                    let a2683 = {
                        let a2681 = v1;
                        let a2682 = 64u8;
                        fixed_u8::bit_and(a2681, a2682)
                    };
                    let a2684 = 64u8;
                    if equal(a2683, a2684) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2685, a2686)?
            };
            let a2698 = {
                let a2695 = {
                    let a2689 = {
                        let a2687 = v1;
                        let a2688 = 32u8;
                        fixed_u8::bit_and(a2687, a2688)
                    };
                    let a2690 = 32u8;
                    if equal(a2689, a2690) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2696 = {
                    let a2693 = {
                        let a2691 = v1;
                        let a2692 = 16u8;
                        fixed_u8::bit_and(a2691, a2692)
                    };
                    let a2694 = 16u8;
                    if equal(a2693, a2694) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2695, a2696)?
            };
            nat_add(a2697, a2698)?
        };
        let a2722 = {
            let a2719 = {
                let a2707 = {
                    let a2701 = {
                        let a2699 = v1;
                        let a2700 = 8u8;
                        fixed_u8::bit_and(a2699, a2700)
                    };
                    let a2702 = 8u8;
                    if equal(a2701, a2702) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2708 = {
                    let a2705 = {
                        let a2703 = v1;
                        let a2704 = 4u8;
                        fixed_u8::bit_and(a2703, a2704)
                    };
                    let a2706 = 4u8;
                    if equal(a2705, a2706) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2707, a2708)?
            };
            let a2720 = {
                let a2717 = {
                    let a2711 = {
                        let a2709 = v1;
                        let a2710 = 2u8;
                        fixed_u8::bit_and(a2709, a2710)
                    };
                    let a2712 = 2u8;
                    if equal(a2711, a2712) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2718 = {
                    let a2715 = {
                        let a2713 = v1;
                        let a2714 = 1u8;
                        fixed_u8::bit_and(a2713, a2714)
                    };
                    let a2716 = 1u8;
                    if equal(a2715, a2716) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2717, a2718)?
            };
            nat_add(a2719, a2720)?
        };
        nat_add(a2721, a2722)?
    };
    let a2723 = 48u64;
    let a2724 = v2;
    if nat_le(a2723, a2724) {
        let a2725 = v2;
        let a2726 = 57u64;
        Ok(nat_le(a2725, a2726))
    } else {
        Ok(false)
    }
}

pub fn f573(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f458({
        let (_, h2727) = {
            let (h2728, _) = f613(Fn20::F614, {
                let a2730 = {
                    let a2729 = v0.clone();
                    length_list(a2729)
                };
                let a2731 = 1u64;
                nat_add(a2730, a2731)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2728
        };
        h2727
    }))
}

pub fn f574(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f575(v0: Adt1, v1: Bytes) -> R<bool> {
    if f67(v0.clone(), f527(v1.clone())) {
        let m2732 = f95(f64(v0, f527(v1)))?;
        match m2732.uncons() {
            None => {
                Ok(false)
            }
            Some((v2, _)) => {
                let m2733 = f43(v2)?;
                match m2733 {
                    None => {
                        Ok(false)
                    }
                    Some(v4) => {
                        let m2734 = f93(v4, f8(Str::lit("reader")));
                        match m2734 {
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

pub fn f576(v0: List<Adt2>) -> List<Bytes> {
    let m2735 = v0;
    match m2735.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2736 = f93(v1, f8(Str::lit("std")));
            match m2736 {
                None => {
                    f576(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f576(v2))
                }
            }
        }
    }
}

pub fn f577(v0: List<Adt2>) -> List<Bytes> {
    let m2737 = v0;
    match m2737.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2738 = f92(f93(v1, f8(Str::lit("req"))));
            match m2738 {
                None => {
                    f577(v2)
                }
                Some(v3) => {
                    List::cons(v3, f577(v2))
                }
            }
        }
    }
}

pub fn f578(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2739 = v0;
    match m2739 {
        Adt2::C0 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C1(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C2(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C3(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C4(_, _, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C5(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C6(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C7(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C8(r2740) => {
            let v4 = (*r2740).clone();
            f578(v4)
        }
        Adt2::C9 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C10(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C11(v1, r2741, r2742) => {
            let v2 = (*r2741).clone();
            let v3 = (*r2742).clone();
            List::cons((v1, v2), f578(v3))
        }
    }
}

pub fn f579(p2747: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2746 = p2747;
    {
        let mut m2746 = m2746;
        loop {
            let step = {
                let v0 = m2746;
                let m2743 = v0;
                match m2743.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f615(f616({
                            let (_, h2744) = v3.clone();
                            h2744
                        }, List::<Bytes>::nil())?, f581({
                            let (h2745, _) = v3.clone();
                            h2745
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
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

pub fn f580(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2748 = v0;
    match m2748.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2749, _) = v2.clone();
                h2749
            }, v1.clone()) {
                f580(v3, v1)
            } else {
                List::cons(v2, f580(v3, v1))
            }
        }
    }
}

pub fn f581(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f386(v0)?;
    if f52(v1.clone(), f617()) {
        f618(v1)
    } else {
        Ok(v1)
    }
}

pub fn f582() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f583() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f584(v0: Bytes, v1: Adt2) -> R<Result<Adt27, Bytes>> {
    f619(v0, f620(v1))
}

pub fn f585(v0: Adt27) -> List<Adt24> {
    let m2750 = v0;
    match m2750 {
        Adt27::C0(_, v2) => {
            v2
        }
    }
}

pub fn f586(v0: Adt27) -> Bytes {
    let m2751 = v0;
    match m2751 {
        Adt27::C0(v1, _) => {
            v1
        }
    }
}

pub fn f587(v0: Fn18, p2755: u64, p2756: Adt26) -> R<Adt26> {
    let m2754 = (p2755, p2756);
    {
        let mut m2754 = m2754;
        loop {
            let step = {
                let (v1, v2) = m2754;
                let m2752 = v1;
                if m2752 == 0 {
                    Err::<(u64, Adt26), R<Adt26>>(Ok(v2))
                } else {
                    let v3 = m2752 - 1;
                    Ok::<(u64, Adt26), R<Adt26>>((v3, {
                        let c2753: Fn18 = v0.clone();
                        c2753.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2754 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: Adt26) -> R<Adt26> {
    f621(v0)
}

pub fn f589(v0: u32, v1: u32) -> u32 {
    let m2759 = {
        let a2757 = f622(v0);
        let a2758 = f622(v1);
        fixed_u64::checked_add(a2757, a2758)
    };
    match m2759 {
        None => {
            0u32
        }
        Some(v2) => {
            f623(v2)
        }
    }
}

pub fn f590(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2762 = {
        let a2760 = v3;
        let a2761 = 8u64;
        nat_add(a2760, a2761)?
    };
    let a2763 = v2;
    if nat_eq(a2762, a2763) {
        Ok(f624({
            let a2766 = {
                let a2764 = v1;
                let a2765 = 8u64;
                nat_mul(a2764, a2765)?
            };
            let a2767 = 4294967296u64;
            let a2768 = 0u64;
            nat_quot(a2766, a2767, a2768)
        }))
    } else {
        let a2771 = {
            let a2769 = v3;
            let a2770 = 4u64;
            nat_add(a2769, a2770)?
        };
        let a2772 = v2;
        if nat_eq(a2771, a2772) {
            Ok(f624({
                let a2775 = {
                    let a2773 = v1;
                    let a2774 = 8u64;
                    nat_mul(a2773, a2774)?
                };
                let a2776 = 4294967296u64;
                let a2777 = 0u64;
                nat_rem(a2775, a2776, a2777)
            }))
        } else {
            Ok(f625(f626(v0.clone(), v1, v3), f626(v0.clone(), v1, {
                let a2778 = v3;
                let a2779 = 1u64;
                nat_add(a2778, a2779)?
            }), f626(v0.clone(), v1, {
                let a2780 = v3;
                let a2781 = 2u64;
                nat_add(a2780, a2781)?
            }), f626(v0, v1, {
                let a2782 = v3;
                let a2783 = 3u64;
                nat_add(a2782, a2783)?
            })))
        }
    }
}

pub fn f591(v0: Adt2) -> Option<Bytes> {
    let m2784 = v0;
    match m2784 {
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
        Adt2::C7(r2787) => {
            let v3 = (*r2787).clone();
            f591(v3)
        }
        Adt2::C8(r2788) => {
            let v4 = (*r2788).clone();
            f591(v4)
        }
        Adt2::C9 => {
            None::<Bytes>
        }
        Adt2::C10(r2789, r2790) => {
            let v5 = (*r2789).clone();
            let v6 = (*r2790).clone();
            let m2785 = f591(v5);
            match m2785 {
                None => {
                    f591(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C11(_, r2791, r2792) => {
            let v9 = (*r2791).clone();
            let v10 = (*r2792).clone();
            let m2786 = f591(v9);
            match m2786 {
                None => {
                    f591(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f592(v0: Adt2) -> R<Option<Bytes>> {
    let m2793 = v0;
    match m2793 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, r2796) => {
            let v2 = (*r2796).clone();
            f592(v2)
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
            if f627(v18, v19)? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f310(v18, v19)?))
            }
        }
        Adt2::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(r2797) => {
            let v3 = (*r2797).clone();
            f592(v3)
        }
        Adt2::C8(r2798) => {
            let v4 = (*r2798).clone();
            f592(v4)
        }
        Adt2::C9 => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(r2799, r2800) => {
            let v5 = (*r2799).clone();
            let v6 = (*r2800).clone();
            let m2794 = f592(v5)?;
            match m2794 {
                None => {
                    f592(v6)
                }
                Some(v7) => {
                    Ok(Some(v7))
                }
            }
        }
        Adt2::C11(_, r2801, r2802) => {
            let v9 = (*r2801).clone();
            let v10 = (*r2802).clone();
            let m2795 = f592(v9)?;
            match m2795 {
                None => {
                    f592(v10)
                }
                Some(v11) => {
                    Ok(Some(v11))
                }
            }
        }
    }
}

pub fn f593(v0: List<Bytes>) -> Adt2 {
    let m2803 = v0;
    match m2803.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f593(v2)))
        }
    }
}

pub fn f594(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f595(p2808: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m2807 = p2808;
    {
        let mut m2807 = m2807;
        loop {
            let step = {
                let v0 = m2807;
                let m2804 = v0;
                match m2804.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h2805, _) = v1.clone();
                            h2805
                        }, f628()) {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h2806, _) = v1;
                                h2806
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2807 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2809 = v0;
    match m2809 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f597(v0: Adt2, v1: Adt2, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f629(v1.clone());
    let v7: Option<Bytes> = {
        let m2810 = f93(v0.clone(), f8(Str::lit("type")));
        match m2810 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m2811 = f92(Some(v4.clone()));
                    match m2811 {
                        None => {
                            f215(f188(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f630(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f594(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f401(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m2812 = f93(v0.clone(), f8(Str::lit("enum")));
        match m2812 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f631(f188(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f594(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m2813 = f93(v0.clone(), f8(Str::lit("const")));
        match m2813 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f632(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f594(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v45: Option<Bytes> = {
        let m2814 = v1.clone();
        match m2814 {
            Adt2::C0 => {
                None::<Bytes>
            }
            Adt2::C1(v28, _) => {
                f594(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
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
            Adt2::C5(v36, v37) => {
                if f627(v36, v37)? {
                    let m2831 = f93(v0.clone(), f8(Str::lit("minimum")));
                    match m2831 {
                        None => {
                            let m2832 = f93(v0.clone(), f8(Str::lit("maximum")));
                            match m2832 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(_) => {
                                    f594(v2.clone(), f8(Str::lit("is a binary floating-point number, which maximum compares only as an exact decimal (JSON Schema 2020-12 Validation section 6.2.2)")))
                                }
                            }
                        }
                        Some(_) => {
                            f594(v2.clone(), f8(Str::lit("is a binary floating-point number, which minimum compares only as an exact decimal (JSON Schema 2020-12 Validation section 6.2.4)")))
                        }
                    }
                } else {
                    f594(v2.clone(), f8(Str::lit("is NaN or an infinity, which is not a JSON number (RFC 8259 section 6)")))
                }
            }
            Adt2::C6(v12) => {
                let m2815 = f633(v12.clone())?;
                match m2815 {
                    None => {
                        f594(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f634(v13)?;
                        f596({
                            let m2816 = f521(f93(v0.clone(), f8(Str::lit("minLength"))));
                            match m2816 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a2817 = v14;
                                    let a2818 = v15;
                                    if nat_lt(a2817, a2818) {
                                        f594(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f596({
                            let m2819 = f521(f93(v0.clone(), f8(Str::lit("maxLength"))));
                            match m2819 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a2820 = v16;
                                    let a2821 = v14;
                                    if nat_lt(a2820, a2821) {
                                        f594(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m2822 = f92(f93(v0.clone(), f8(Str::lit("pattern"))));
                            match m2822 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m2823 = f635(v17.clone(), v12)?;
                                    match m2823 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f594(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
                                            }
                                        }
                                        Err(v18) => {
                                            Some(v18)
                                        }
                                    }
                                }
                            }
                        }))
                    }
                }
            }
            Adt2::C7(r2833) => {
                let v20 = (*r2833).clone();
                let v21: u64 = f636(v20)?;
                f596({
                    let m2824 = f521(f93(v0.clone(), f8(Str::lit("minItems"))));
                    match m2824 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a2825 = v21;
                            let a2826 = v22;
                            if nat_lt(a2825, a2826) {
                                f594(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2827 = f521(f93(v0.clone(), f8(Str::lit("maxItems"))));
                    match m2827 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a2828 = v23;
                            let a2829 = v21;
                            if nat_lt(a2828, a2829) {
                                f594(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt2::C8(r2834) => {
                let v24 = (*r2834).clone();
                let v25: List<Bytes> = f637(f215(f188(f93(v0.clone(), f8(Str::lit("required"))))), f638(f578(v24)));
                let m2830 = v25;
                match m2830.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f594(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
                    }
                }
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
    };
    let v51: Option<Bytes> = {
        let m2835 = f639(v1);
        match m2835 {
            None => {
                None::<Bytes>
            }
            Some(v46) => {
                f596({
                    let m2837 = {
                        let m2836 = f93(v0.clone(), f8(Str::lit("minimum")));
                        match m2836 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v47) => {
                                f639(v47)
                            }
                        }
                    };
                    match m2837 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v48) => {
                            if f640(v46, v48)? {
                                f594(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2839 = {
                        let m2838 = f93(v0, f8(Str::lit("maximum")));
                        match m2838 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v49) => {
                                f639(v49)
                            }
                        }
                    };
                    match m2839 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v50) => {
                            if f640(v50, v46)? {
                                f594(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f596(v7, f596(v9, f596(v11, f596(v45, v51)))))
}

pub fn f598(v0: Adt2, v1: Bytes) -> R<Result<Adt2, Bytes>> {
    let v2: u64 = {
        let a2840 = v1.clone();
        length_bytes(a2840)
    };
    if f32(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt2, Bytes>(v0))
    } else {
        let m2845 = if f85(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f46(v1.clone(), 8u64, {
                let a2841 = v2;
                let a2842 = 8u64;
                nat_sub(a2841, a2842)
            })))
        } else {
            if f85(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f46(v1.clone(), 14u64, {
                    let a2843 = v2;
                    let a2844 = 14u64;
                    nat_sub(a2843, a2844)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m2845 {
            None => {
                Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f641({
                    let a2848 = {
                        let a2847 = {
                            let (_, h2846) = v3.clone();
                            h2846
                        };
                        length_bytes(a2847)
                    };
                    let a2849 = 1u64;
                    nat_add(a2848, a2849)?
                }, {
                    let (_, h2850) = v3;
                    h2850
                }, 0u64, Bytes::lit(&[]))?;
                let m2853 = {
                    let m2852 = f93(v0, {
                        let (h2851, _) = v3;
                        h2851
                    });
                    match m2852 {
                        None => {
                            None::<Adt2>
                        }
                        Some(v5) => {
                            f93(v5, v4)
                        }
                    }
                };
                match m2853 {
                    None => {
                        Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" names no definition of this schema (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok(Ok::<Adt2, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f599(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2854 = v0;
    if m2854 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m2854 - 1;
        let m2855 = v2;
        match m2855.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f596(f549(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f599(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f600(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<u64> {
    let m2856 = v0;
    if m2856 == 0 {
        Ok(0u64)
    } else {
        let v5 = m2856 - 1;
        let m2857 = v2;
        match m2857.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a2859 = {
                    let m2858 = f549(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m2858 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a2860 = f600(v5, v1, v7, v3, v4)?;
                nat_add(a2859, a2860)
            }
        }
    }
}

pub fn f601(v0: u64, v1: Adt2, v2: List<(Bytes, Adt2)>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2861 = v0;
    if m2861 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m2861 - 1;
        let m2862 = v2;
        match m2862.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f596({
                    let m2864 = f93(v3.clone(), {
                        let (h2863, _) = v6.clone();
                        h2863
                    });
                    match m2864 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f549(v5, v1.clone(), {
                                let (_, h2865) = v6;
                                h2865
                            }, v8, f642(v4.clone(), {
                                let (h2866, _) = v6;
                                h2866
                            })?)?
                        }
                    }
                }, f601(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f602(v0: u64, v1: Adt2, v2: Adt2, v3: List<(Bytes, Adt2)>, v4: Bytes) -> R<Option<Bytes>> {
    let m2867 = v0;
    if m2867 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m2867 - 1;
        let m2868 = v3;
        match m2868.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f642(v4.clone(), {
                    let (h2869, _) = v6.clone();
                    h2869
                })?;
                let m2871 = f643(f537(f93(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h2870, _) = v6.clone();
                    h2870
                })?;
                match m2871 {
                    Ok(v10) => {
                        Ok(f596(f599(v5, v1.clone(), v10.clone(), {
                            let (_, h2872) = v6.clone();
                            h2872
                        }, v8.clone())?, f596({
                            let m2873 = v10;
                            match m2873.uncons() {
                                None => {
                                    if f52({
                                        let (h2874, _) = v6;
                                        h2874
                                    }, f638(f578({
                                        let m2875 = f93(v2.clone(), f8(Str::lit("properties")));
                                        match m2875 {
                                            None => {
                                                Adt2::C8(std::rc::Rc::new(Adt2::C9))
                                            }
                                            Some(v13) => {
                                                v13
                                            }
                                        }
                                    }))) {
                                        None::<Bytes>
                                    } else {
                                        let m2876 = f93(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m2876 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f549(v5, v1.clone(), v14, {
                                                    let (_, h2877) = v6;
                                                    h2877
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f602(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f603(v0: u64, v1: Adt2, v2: Adt2, v3: List<Adt2>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m2878 = v0;
    if m2878 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m2878 - 1;
        let m2879 = v3;
        match m2879.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f596(f549(v6, v1.clone(), v2.clone(), v7, f642(v4.clone(), f410(v5)?)?)?, f603(v6, v1, v2, v8, v4, {
                    let a2880 = v5;
                    let a2881 = 1u64;
                    nat_add(a2880, a2881)?
                })?))
            }
        }
    }
}

pub fn f604(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2882 = v0;
    match m2882.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f604(v3, false))
            } else {
                f604(v3, true)
            }
        }
    }
}

pub fn f605(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2883, _) = v2;
        h2883
    };
    let a2884 = v3;
    let a2885 = 0u64;
    if nat_eq(a2884, a2885) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2886 = v3;
            let a2887 = v1;
            let a2888 = 0u64;
            nat_quot(a2886, a2887, a2888)
        }, {
            let a2893 = f555(v0, {
                let a2889 = v3;
                let a2890 = v1;
                let a2891 = 0u64;
                nat_rem(a2889, a2890, a2891)
            });
            let a2894 = {
                let (_, h2892) = v2;
                h2892
            };
            append_bytes(a2893, a2894)
        }))
    }
}

pub fn f606(v0: u64) -> R<Bytes> {
    let v1: Bytes = f410(v0)?;
    let a2898 = f503({
        let a2896 = 9u64;
        let a2897 = {
            let a2895 = v1.clone();
            length_bytes(a2895)
        };
        nat_sub(a2896, a2897)
    });
    let a2899 = v1;
    Ok(append_bytes(a2898, a2899))
}

pub fn f607() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f608(v0: Fn19, p2903: List<List<(Bytes, Bytes)>>, p2904: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2902 = (p2903, p2904);
    {
        let mut m2902 = m2902;
        loop {
            let step = {
                let (v1, v2) = m2902;
                let m2900 = v2;
                match m2900.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2901: Fn19 = v0.clone();
                            c2901.apply(v1, v3)
                        }, v4))
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

pub fn f609(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f610(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2906 = {
        let a2905 = v0.clone();
        length_list(a2905)
    };
    let a2907 = 1u64;
    if nat_le(a2906, a2907) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f644(v0)?))
    }
}

pub fn f611(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2911 = {
        let a2909 = v0.clone();
        let a2910 = {
            let (h2908, _) = v1.clone();
            h2908
        };
        index_bytes(a2909, a2910)
    };
    match m2911 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2913 = {
                    let (h2912, _) = v1.clone();
                    h2912
                };
                let a2914 = 1u64;
                nat_add(a2913, a2914)?
            }, {
                let a2917 = {
                    let (_, h2915) = v1;
                    h2915
                };
                let a2918 = f504(v0, {
                    let (h2916, _) = v1;
                    h2916
                }, v2)?;
                append_bytes(a2917, a2918)
            })))
        }
    }
}

pub fn f612(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2919, _) = v1;
        h2919
    };
    let m2921 = {
        let (_, h2920) = v1;
        h2920
    };
    match m2921 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2924 = {
                let a2922 = v0.clone();
                let a2923 = v2;
                index_bytes(a2922, a2923)
            };
            match m2924 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2929 = {
                        let a2927 = v0;
                        let a2928 = {
                            let a2925 = v2;
                            let a2926 = 1u64;
                            nat_add(a2925, a2926)?
                        };
                        index_bytes(a2927, a2928)
                    };
                    match m2929 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f645(v4)? {
                                f645(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2930 = v2;
                                    let a2931 = 2u64;
                                    nat_add(a2930, a2931)?
                                }, Some({
                                    let a2936 = v3;
                                    let a2937 = f555(f646(), {
                                        let a2934 = {
                                            let a2932 = f647(v4)?;
                                            let a2933 = 16u64;
                                            nat_mul(a2932, a2933)?
                                        };
                                        let a2935 = f647(v5)?;
                                        nat_add(a2934, a2935)?
                                    });
                                    append_bytes(a2936, a2937)
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

pub fn f613(v0: Fn20, p2942: u64, p2943: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2941 = (p2942, p2943);
    {
        let mut m2941 = m2941;
        loop {
            let step = {
                let (v1, v2) = m2941;
                let m2938 = v1;
                if m2938 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2938 - 1;
                    let m2940 = {
                        let c2939: Fn20 = v0.clone();
                        c2939.apply(v2.clone())?
                    };
                    match m2940 {
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
                Ok(next) => { m2941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f648(v0)
}

pub fn f615(p2946: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2945 = p2946;
    {
        let mut m2945 = m2945;
        loop {
            let step = {
                let v0 = m2945;
                let m2944 = v0;
                match m2944.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f32(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f52(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f52(v4, v3.clone())
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
                Ok(next) => { m2945 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2947 = v0;
    match m2947 {
        Adt2::C0 => {
            Ok(v1)
        }
        Adt2::C1(_, _) => {
            Ok(v1)
        }
        Adt2::C2(_) => {
            Ok(v1)
        }
        Adt2::C3(_, _) => {
            Ok(v1)
        }
        Adt2::C4(_, _, _) => {
            Ok(v1)
        }
        Adt2::C5(_, _) => {
            Ok(v1)
        }
        Adt2::C6(_) => {
            Ok(v1)
        }
        Adt2::C7(r2950) => {
            let v2 = (*r2950).clone();
            f616(v2, v1)
        }
        Adt2::C8(r2951) => {
            let v3 = (*r2951).clone();
            f616(v3, v1)
        }
        Adt2::C9 => {
            Ok(v1)
        }
        Adt2::C10(r2952, r2953) => {
            let v4 = (*r2952).clone();
            let v5 = (*r2953).clone();
            f616(v4, f616(v5, v1)?)
        }
        Adt2::C11(v6, r2954, r2955) => {
            let v7 = (*r2954).clone();
            let v8 = (*r2955).clone();
            let v9: List<Bytes> = f616(v7.clone(), f616(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2948 = f92(Some(v7));
                match m2948 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2949 = f649(v10)?;
                        match m2949 {
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

pub fn f617() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f618(v0: Bytes) -> R<Bytes> {
    let m2958 = {
        let a2956 = v0.clone();
        let a2957 = 0u64;
        index_bytes(a2956, a2957)
    };
    match m2958 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2964 = {
                let a2959 = f8(Str::lit("u"));
                let a2960 = f650(8u64, f72(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2959, a2960)
            };
            let a2965 = f46(v0.clone(), 1u64, {
                let a2962 = {
                    let a2961 = v0;
                    length_bytes(a2961)
                };
                let a2963 = 1u64;
                nat_sub(a2962, a2963)
            });
            Ok(append_bytes(a2964, a2965))
        }
    }
}

pub fn f619(v0: Bytes, v1: Adt2) -> R<Result<Adt27, Bytes>> {
    let m2966 = f93(v1.clone(), f8(Str::lit("enum")));
    match m2966 {
        None => {
            let m2968 = f93(v1.clone(), f8(Str::lit("oneOf")));
            match m2968 {
                None => {
                    if f655(v1.clone(), v0.clone())? {
                        let m2969 = f656(v0.clone(), f537(f93(v1.clone(), f8(Str::lit("properties")))), f215(f188(f93(v1, f8(Str::lit("required"))))), true)?;
                        match m2969 {
                            Ok(v7) => {
                                Ok(f653(f657(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f401(f658(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f659(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt27, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2970 = f537(f93(v1.clone(), f8(Str::lit("properties"))));
                        match m2970.uncons() {
                            None => {
                                if f660(v1.clone()) {
                                    Ok(f653(f661(v0), List::<Adt24>::nil()))
                                } else {
                                    let m2973 = f662(v1, {
                                        let a2971 = v0.clone();
                                        let a2972 = f8(Str::lit("Value"));
                                        append_bytes(a2971, a2972)
                                    })?;
                                    match m2973 {
                                        Ok(v9) => {
                                            Ok(f653(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f586(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f585(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt27, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2974 = f656(v0.clone(), f537(f93(v1.clone(), f8(Str::lit("properties")))), f215(f188(f93(v1, f8(Str::lit("required"))))), false)?;
                                match m2974 {
                                    Ok(v13) => {
                                        Ok(f653(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2975 = f658(v13.clone());
                                            match m2975.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f401(f658(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f659(v13)))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<Adt27, Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f654(v0, f306(v5))
                }
            }
        }
        Some(v2) => {
            let m2967 = f651(f306(v2), List::<Bytes>::nil())?;
            match m2967 {
                Ok(v4) => {
                    Ok(f653(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f401(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt24>::nil()))
                }
                Err(v3) => {
                    Ok(f652(v3, v0))
                }
            }
        }
    }
}

pub fn f620(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2976 = f93(v0.clone(), f8(Str::lit("anyOf")));
        match m2976 {
            None => {
                v0
            }
            Some(v1) => {
                if f663(f306(v1.clone())) {
                    f664(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f665(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f666(f306(v1.clone()))
                    } {
                        v0
                    } else {
                        f667(f664(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2977 = f93(v2.clone(), f8(Str::lit("oneOf")));
        match m2977 {
            None => {
                v2
            }
            Some(v3) => {
                if f663(f306(v3)) {
                    f664(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2978 = f93(v4.clone(), f8(Str::lit("allOf")));
    match m2978 {
        None => {
            v4
        }
        Some(v5) => {
            if f668(f306(v5)) {
                f664(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f621(v0: Adt26) -> R<Adt26> {
    let v1: u32 = f669(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2979) => {
            h2979
        }
    });
    let v2: u32 = f589(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h2980, _, _) => {
            h2980
        }
    }, f589(f670(match v0.clone() {
        Adt26::C0(_, _, _, _, h2981, _, _, _, _, _) => {
            h2981
        }
    }), f589(f671(match v0.clone() {
        Adt26::C0(_, _, _, _, h2982, _, _, _, _, _) => {
            h2982
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2983, _, _, _, _) => {
            h2983
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2984, _, _, _) => {
            h2984
        }
    }), f589(v1, {
        let m2987 = match v0.clone() {
            Adt26::C0(_, _, _, _, _, _, _, _, h2986, _) => {
                h2986
            }
        };
        match m2987 {
            Adt25::C0(h2985, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2985
            }
        }
    }))));
    let v3: u32 = f589(f672(match v0.clone() {
        Adt26::C0(h2988, _, _, _, _, _, _, _, _, _) => {
            h2988
        }
    }), f673(match v0.clone() {
        Adt26::C0(h2989, _, _, _, _, _, _, _, _, _) => {
            h2989
        }
    }, match v0.clone() {
        Adt26::C0(_, h2990, _, _, _, _, _, _, _, _) => {
            h2990
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2991, _, _, _, _, _, _, _) => {
            h2991
        }
    }));
    Ok(Adt26::C0(f589(v2, v3), match v0.clone() {
        Adt26::C0(h2992, _, _, _, _, _, _, _, _, _) => {
            h2992
        }
    }, match v0.clone() {
        Adt26::C0(_, h2993, _, _, _, _, _, _, _, _) => {
            h2993
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2994, _, _, _, _, _, _, _) => {
            h2994
        }
    }, f589(match v0.clone() {
        Adt26::C0(_, _, _, h2995, _, _, _, _, _, _) => {
            h2995
        }
    }, v2), match v0.clone() {
        Adt26::C0(_, _, _, _, h2996, _, _, _, _, _) => {
            h2996
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2997, _, _, _, _) => {
            h2997
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2998, _, _, _) => {
            h2998
        }
    }, f674(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h2999, _) => {
            h2999
        }
    }), {
        let a3001 = match v0 {
            Adt26::C0(_, _, _, _, _, _, _, _, _, h3000) => {
                h3000
            }
        };
        let a3002 = 1u64;
        nat_add(a3001, a3002)?
    }))
}

pub fn f622(v0: u32) -> u64 {
    let m3004 = {
        let a3003 = v0;
        fixed_u64::convert(i128::from(a3003))
    };
    match m3004 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f623(v0: u64) -> u32 {
    let m3008 = {
        let a3007 = {
            let a3005 = v0;
            let a3006 = 4294967295u64;
            fixed_u64::bit_and(a3005, a3006)
        };
        fixed_u32::convert(i128::from(a3007))
    };
    match m3008 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f624(v0: u64) -> u32 {
    let (_, h3009) = {
        let (_, h3010) = f675(Fn21::F676, 32u64, (v0, (1u32, 0u32)));
        h3010
    };
    h3009
}

pub fn f625(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3015 = f677(v0, 24u32);
    let a3016 = {
        let a3013 = f677(v1, 16u32);
        let a3014 = {
            let a3011 = f677(v2, 8u32);
            let a3012 = v3;
            fixed_u32::bit_or(a3011, a3012)
        };
        fixed_u32::bit_or(a3013, a3014)
    };
    fixed_u32::bit_or(a3015, a3016)
}

pub fn f626(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3017 = v2;
    let a3018 = v1;
    if nat_lt(a3017, a3018) {
        let m3021 = {
            let a3019 = v0;
            let a3020 = v2;
            index_bytes(a3019, a3020)
        };
        match m3021 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3023 = {
                    let a3022 = v3;
                    fixed_u32::convert(i128::from(a3022))
                };
                match m3023 {
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
        let a3024 = v2;
        let a3025 = v1;
        if nat_eq(a3024, a3025) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f627(v0: bool, v1: u64) -> R<bool> {
    let a3032 = {
        let a3029 = {
            let a3026 = v1;
            let a3027 = f411(v0)?;
            let a3028 = 0u64;
            nat_quot(a3026, a3027, a3028)
        };
        let a3030 = f413(f414(v0))?;
        let a3031 = 0u64;
        nat_rem(a3029, a3030, a3031)
    };
    let a3033 = f415(v0)?;
    Ok(!nat_eq(a3032, a3033))
}

pub fn f628() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f629(v0: Adt2) -> Bytes {
    let m3034 = v0;
    match m3034 {
        Adt2::C0 => {
            f8(Str::lit("null"))
        }
        Adt2::C1(_, _) => {
            f8(Str::lit("tagged"))
        }
        Adt2::C2(_) => {
            f8(Str::lit("boolean"))
        }
        Adt2::C3(_, _) => {
            f8(Str::lit("integer"))
        }
        Adt2::C4(_, _, _) => {
            f8(Str::lit("number"))
        }
        Adt2::C5(_, _) => {
            f8(Str::lit("number"))
        }
        Adt2::C6(_) => {
            f8(Str::lit("string"))
        }
        Adt2::C7(_) => {
            f8(Str::lit("array"))
        }
        Adt2::C8(_) => {
            f8(Str::lit("object"))
        }
        Adt2::C9 => {
            f8(Str::lit("spine"))
        }
        Adt2::C10(_, _) => {
            f8(Str::lit("spine"))
        }
        Adt2::C11(_, _, _) => {
            f8(Str::lit("spine"))
        }
    }
}

pub fn f630(p3037: List<Bytes>, v1: Adt2) -> bool {
    let m3036 = p3037;
    {
        let mut m3036 = m3036;
        loop {
            let step = {
                let v0 = m3036;
                let m3035 = v0;
                match m3035.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f678(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3036 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631(p3040: List<Adt2>, v1: Adt2) -> R<bool> {
    let m3039 = p3040;
    {
        let mut m3039 = m3039;
        loop {
            let step = {
                let v0 = m3039;
                let m3038 = v0;
                match m3038.uncons() {
                    None => {
                        Err::<List<Adt2>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f632(false, v2, v1.clone())? {
                            Err::<List<Adt2>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt2>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3039 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f632(v0: bool, v1: Adt2, v2: Adt2) -> R<bool> {
    let m3041 = v1;
    match m3041 {
        Adt2::C0 => {
            Ok(f32(f550(v2)?, f8(Str::lit("null"))))
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(v85) => {
            Ok(f32(f550(v2)?, f550(Adt2::C2(v85))?))
        }
        Adt2::C3(v86, v87) => {
            Ok(f32(f550(v2)?, f550(Adt2::C3(v86, v87))?))
        }
        Adt2::C4(v88, v89, v90) => {
            Ok(f32(f550(v2)?, f550(Adt2::C4(v88, v89, v90))?))
        }
        Adt2::C5(v91, v92) => {
            Ok(f32(f550(v2)?, f550(Adt2::C5(v91, v92))?))
        }
        Adt2::C6(v93) => {
            let m3053 = v2;
            match m3053 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(v94) => {
                    Ok(f32(v93, v94))
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8(_) => {
                    Ok(false)
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(_, _) => {
                    Ok(false)
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C7(r3054) => {
            let v64 = (*r3054).clone();
            let m3051 = v2;
            match m3051 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(r3052) => {
                    let v65 = (*r3052).clone();
                    f632(false, v64, v65)
                }
                Adt2::C8(_) => {
                    Ok(false)
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(_, _) => {
                    Ok(false)
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C8(r3055) => {
            let v45 = (*r3055).clone();
            let m3047 = v2;
            match m3047 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8(r3050) => {
                    let v46 = (*r3050).clone();
                    if f117({
                        let a3048 = f636(v45.clone())?;
                        let a3049 = f636(v46.clone())?;
                        nat_eq(a3048, a3049)
                    }, true) {
                        f632(true, v45, v46)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(_, _) => {
                    Ok(false)
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C9 => {
            if v0 {
                Ok(true)
            } else {
                let m3043 = v2;
                match m3043 {
                    Adt2::C0 => {
                        Ok(false)
                    }
                    Adt2::C1(_, _) => {
                        Ok(false)
                    }
                    Adt2::C2(_) => {
                        Ok(false)
                    }
                    Adt2::C3(_, _) => {
                        Ok(false)
                    }
                    Adt2::C4(_, _, _) => {
                        Ok(false)
                    }
                    Adt2::C5(_, _) => {
                        Ok(false)
                    }
                    Adt2::C6(_) => {
                        Ok(false)
                    }
                    Adt2::C7(_) => {
                        Ok(false)
                    }
                    Adt2::C8(_) => {
                        Ok(false)
                    }
                    Adt2::C9 => {
                        Ok(true)
                    }
                    Adt2::C10(_, _) => {
                        Ok(false)
                    }
                    Adt2::C11(_, _, _) => {
                        Ok(false)
                    }
                }
            }
        }
        Adt2::C10(r3056, r3057) => {
            let v25 = (*r3056).clone();
            let v26 = (*r3057).clone();
            let m3044 = v2;
            match m3044 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8(_) => {
                    Ok(false)
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(r3045, r3046) => {
                    let v27 = (*r3045).clone();
                    let v28 = (*r3046).clone();
                    if f632(false, v25, v27)? {
                        f632(false, v26, v28)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C11(v3, r3058, r3059) => {
            let v4 = (*r3058).clone();
            let v5 = (*r3059).clone();
            let m3042 = f93(v2.clone(), v3);
            if match m3042 {
                None => {
                    false
                }
                Some(v6) => {
                    f632(false, v4, v6)?
                }
            } {
                f632(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f633(v0: Bytes) -> R<Option<List<u64>>> {
    f679({
        let a3061 = {
            let a3060 = v0.clone();
            length_bytes(a3060)
        };
        let a3062 = 1u64;
        nat_add(a3061, a3062)?
    }, v0, 0u64)
}

pub fn f634(v0: List<u64>) -> R<u64> {
    let m3063 = v0;
    match m3063.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3064 = f634(v2)?;
            let a3065 = 1u64;
            nat_add(a3064, a3065)
        }
    }
}

pub fn f635(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m3066 = f680(v0)?;
    match m3066 {
        Ok(v3) => {
            let m3067 = f633(v1)?;
            match m3067 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h3068, _) = {
                            let (h3069, _) = v3.clone();
                            h3069
                        };
                        if h3068 {
                            f681({
                                let a3073 = {
                                    let a3071 = f682({
                                        let (_, h3070) = v3.clone();
                                        h3070
                                    })?;
                                    let a3072 = f634(v4.clone())?;
                                    nat_add(a3071, a3072)?
                                };
                                let a3074 = 2u64;
                                nat_add(a3073, a3074)?
                            }, {
                                let (_, h3075) = v3;
                                h3075
                            }, v4, {
                                let (_, h3076) = {
                                    let (h3077, _) = v3;
                                    h3077
                                };
                                h3076
                            })?
                        } else {
                            f683({
                                let (_, h3078) = v3;
                                h3078
                            }, v4, {
                                let (_, h3079) = {
                                    let (h3080, _) = v3;
                                    h3080
                                };
                                h3079
                            })?
                        }
                    }))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<bool, Bytes>(v2))
        }
    }
}

pub fn f636(v0: Adt2) -> R<u64> {
    let m3081 = v0;
    match m3081 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(_, _) => {
            Ok(0u64)
        }
        Adt2::C2(_) => {
            Ok(0u64)
        }
        Adt2::C3(_, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _) => {
            Ok(0u64)
        }
        Adt2::C6(_) => {
            Ok(0u64)
        }
        Adt2::C7(_) => {
            Ok(0u64)
        }
        Adt2::C8(_) => {
            Ok(0u64)
        }
        Adt2::C9 => {
            Ok(0u64)
        }
        Adt2::C10(_, r3086) => {
            let v5 = (*r3086).clone();
            let a3084 = f636(v5)?;
            let a3085 = 1u64;
            nat_add(a3084, a3085)
        }
        Adt2::C11(_, _, r3087) => {
            let v3 = (*r3087).clone();
            let a3082 = f636(v3)?;
            let a3083 = 1u64;
            nat_add(a3082, a3083)
        }
    }
}

pub fn f637(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3088 = v0;
    match m3088.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f637(v3, v1)
            } else {
                List::cons(v2, f637(v3, v1))
            }
        }
    }
}

pub fn f638(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m3089 = v0;
    match m3089.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3090, _) = v1;
                h3090
            }, f638(v2))
        }
    }
}

pub fn f639(v0: Adt2) -> Option<(bool, (u64, u64))> {
    let m3091 = v0;
    match m3091 {
        Adt2::C0 => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C1(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C2(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C3(v1, v2) => {
            Some((v1, (v2, 0u64)))
        }
        Adt2::C4(v3, v4, v5) => {
            Some((v3, (v4, v5)))
        }
        Adt2::C5(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C6(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C7(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C8(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C9 => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C10(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C11(_, _, _) => {
            None::<(bool, (u64, u64))>
        }
    }
}

pub fn f640(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a3096 = {
            let (h3092, _) = {
                let (_, h3093) = v0;
                h3093
            };
            h3092
        };
        let a3097 = f684({
            let (_, h3094) = {
                let (_, h3095) = v1;
                h3095
            };
            h3094
        })?;
        nat_mul(a3096, a3097)?
    };
    let v3: u64 = {
        let a3102 = {
            let (h3098, _) = {
                let (_, h3099) = v1;
                h3099
            };
            h3098
        };
        let a3103 = f684({
            let (_, h3100) = {
                let (_, h3101) = v0;
                h3101
            };
            h3100
        })?;
        nat_mul(a3102, a3103)?
    };
    let (h3104, _) = v0;
    if h3104 {
        let (h3105, _) = v1;
        if h3105 {
            let a3106 = v3;
            let a3107 = v2;
            Ok(nat_lt(a3106, a3107))
        } else {
            Ok(true)
        }
    } else {
        let (h3108, _) = v1;
        if h3108 {
            Ok(false)
        } else {
            let a3109 = v2;
            let a3110 = v3;
            Ok(nat_lt(a3109, a3110))
        }
    }
}

pub fn f641(p3140: u64, v1: Bytes, p3141: u64, p3142: Bytes) -> R<Bytes> {
    let m3139 = (p3140, (p3141, p3142));
    {
        let mut m3139 = m3139;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3139;
                let m3111 = v0;
                if m3111 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3111 - 1;
                    let a3113 = {
                        let a3112 = v1.clone();
                        length_bytes(a3112)
                    };
                    let a3114 = v2;
                    if nat_le(a3113, a3114) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f117({
                            let a3115 = f115(v1.clone(), v2)?;
                            let a3116 = 126u64;
                            nat_eq(a3115, a3116)
                        }, {
                            let a3119 = f115(v1.clone(), {
                                let a3117 = v2;
                                let a3118 = 1u64;
                                nat_add(a3117, a3118)?
                            })?;
                            let a3120 = 48u64;
                            nat_eq(a3119, a3120)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3121 = v2;
                                let a3122 = 2u64;
                                nat_add(a3121, a3122)?
                            }, {
                                let a3123 = v3;
                                let a3124 = f8(Str::lit("~"));
                                append_bytes(a3123, a3124)
                            })))
                        } else {
                            if f117({
                                let a3125 = f115(v1.clone(), v2)?;
                                let a3126 = 126u64;
                                nat_eq(a3125, a3126)
                            }, {
                                let a3129 = f115(v1.clone(), {
                                    let a3127 = v2;
                                    let a3128 = 1u64;
                                    nat_add(a3127, a3128)?
                                })?;
                                let a3130 = 49u64;
                                nat_eq(a3129, a3130)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3131 = v2;
                                    let a3132 = 2u64;
                                    nat_add(a3131, a3132)?
                                }, {
                                    let a3133 = v3;
                                    let a3134 = f8(Str::lit("/"));
                                    append_bytes(a3133, a3134)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3135 = v2;
                                    let a3136 = 1u64;
                                    nat_add(a3135, a3136)?
                                }, {
                                    let a3137 = v3;
                                    let a3138 = f46(v1.clone(), v2, 1u64);
                                    append_bytes(a3137, a3138)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f642(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f685({
        let a3144 = {
            let a3143 = v1.clone();
            length_bytes(a3143)
        };
        let a3145 = 1u64;
        nat_add(a3144, a3145)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f643(v0: List<(Bytes, Adt2)>, v1: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let m3146 = v0;
    match m3146.uncons() {
        None => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let m3148 = f635({
                let (h3147, _) = v2;
                h3147
            }, v1.clone())?;
            match m3148 {
                Ok(v5) => {
                    let m3149 = f643(v3, v1)?;
                    match m3149 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt2>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h3150) = v2;
                                    h3150
                                }, v7)
                            } else {
                                v7
                            }))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt2>, Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt2>, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f644(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f563({
        let (_, h3151) = {
            let (h3152, _) = f686(Fn22::F687, {
                let a3154 = {
                    let a3153 = v0.clone();
                    length_list(a3153)
                };
                let a3155 = 1u64;
                nat_add(a3154, a3155)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3152
        };
        h3151
    }))
}

pub fn f645(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3200 = {
            let a3176 = {
                let a3164 = {
                    let a3158 = {
                        let a3156 = v1;
                        let a3157 = 128u8;
                        fixed_u8::bit_and(a3156, a3157)
                    };
                    let a3159 = 128u8;
                    if equal(a3158, a3159) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3165 = {
                    let a3162 = {
                        let a3160 = v1;
                        let a3161 = 64u8;
                        fixed_u8::bit_and(a3160, a3161)
                    };
                    let a3163 = 64u8;
                    if equal(a3162, a3163) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3164, a3165)?
            };
            let a3177 = {
                let a3174 = {
                    let a3168 = {
                        let a3166 = v1;
                        let a3167 = 32u8;
                        fixed_u8::bit_and(a3166, a3167)
                    };
                    let a3169 = 32u8;
                    if equal(a3168, a3169) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3175 = {
                    let a3172 = {
                        let a3170 = v1;
                        let a3171 = 16u8;
                        fixed_u8::bit_and(a3170, a3171)
                    };
                    let a3173 = 16u8;
                    if equal(a3172, a3173) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3174, a3175)?
            };
            nat_add(a3176, a3177)?
        };
        let a3201 = {
            let a3198 = {
                let a3186 = {
                    let a3180 = {
                        let a3178 = v1;
                        let a3179 = 8u8;
                        fixed_u8::bit_and(a3178, a3179)
                    };
                    let a3181 = 8u8;
                    if equal(a3180, a3181) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3187 = {
                    let a3184 = {
                        let a3182 = v1;
                        let a3183 = 4u8;
                        fixed_u8::bit_and(a3182, a3183)
                    };
                    let a3185 = 4u8;
                    if equal(a3184, a3185) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3186, a3187)?
            };
            let a3199 = {
                let a3196 = {
                    let a3190 = {
                        let a3188 = v1;
                        let a3189 = 2u8;
                        fixed_u8::bit_and(a3188, a3189)
                    };
                    let a3191 = 2u8;
                    if equal(a3190, a3191) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3197 = {
                    let a3194 = {
                        let a3192 = v1;
                        let a3193 = 1u8;
                        fixed_u8::bit_and(a3192, a3193)
                    };
                    let a3195 = 1u8;
                    if equal(a3194, a3195) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3196, a3197)?
            };
            nat_add(a3198, a3199)?
        };
        nat_add(a3200, a3201)?
    };
    let a3202 = 48u64;
    let a3203 = v2;
    if if nat_le(a3202, a3203) {
        let a3204 = v2;
        let a3205 = 57u64;
        nat_le(a3204, a3205)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3206 = 97u64;
        let a3207 = v2;
        if nat_le(a3206, a3207) {
            let a3208 = v2;
            let a3209 = 102u64;
            Ok(nat_le(a3208, a3209))
        } else {
            Ok(false)
        }
    }
}

pub fn f646() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f647(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3254 = {
            let a3230 = {
                let a3218 = {
                    let a3212 = {
                        let a3210 = v1;
                        let a3211 = 128u8;
                        fixed_u8::bit_and(a3210, a3211)
                    };
                    let a3213 = 128u8;
                    if equal(a3212, a3213) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3219 = {
                    let a3216 = {
                        let a3214 = v1;
                        let a3215 = 64u8;
                        fixed_u8::bit_and(a3214, a3215)
                    };
                    let a3217 = 64u8;
                    if equal(a3216, a3217) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3218, a3219)?
            };
            let a3231 = {
                let a3228 = {
                    let a3222 = {
                        let a3220 = v1;
                        let a3221 = 32u8;
                        fixed_u8::bit_and(a3220, a3221)
                    };
                    let a3223 = 32u8;
                    if equal(a3222, a3223) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3229 = {
                    let a3226 = {
                        let a3224 = v1;
                        let a3225 = 16u8;
                        fixed_u8::bit_and(a3224, a3225)
                    };
                    let a3227 = 16u8;
                    if equal(a3226, a3227) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3228, a3229)?
            };
            nat_add(a3230, a3231)?
        };
        let a3255 = {
            let a3252 = {
                let a3240 = {
                    let a3234 = {
                        let a3232 = v1;
                        let a3233 = 8u8;
                        fixed_u8::bit_and(a3232, a3233)
                    };
                    let a3235 = 8u8;
                    if equal(a3234, a3235) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3241 = {
                    let a3238 = {
                        let a3236 = v1;
                        let a3237 = 4u8;
                        fixed_u8::bit_and(a3236, a3237)
                    };
                    let a3239 = 4u8;
                    if equal(a3238, a3239) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3240, a3241)?
            };
            let a3253 = {
                let a3250 = {
                    let a3244 = {
                        let a3242 = v1;
                        let a3243 = 2u8;
                        fixed_u8::bit_and(a3242, a3243)
                    };
                    let a3245 = 2u8;
                    if equal(a3244, a3245) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3251 = {
                    let a3248 = {
                        let a3246 = v1;
                        let a3247 = 1u8;
                        fixed_u8::bit_and(a3246, a3247)
                    };
                    let a3249 = 1u8;
                    if equal(a3248, a3249) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3250, a3251)?
            };
            nat_add(a3252, a3253)?
        };
        nat_add(a3254, a3255)?
    };
    let a3256 = v2;
    let a3257 = 57u64;
    if nat_le(a3256, a3257) {
        let a3258 = v2;
        let a3259 = 48u64;
        Ok(nat_sub(a3258, a3259))
    } else {
        let a3260 = v2;
        let a3261 = 87u64;
        Ok(nat_sub(a3260, a3261))
    }
}

pub fn f648(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3262, _) = v0;
        h3262
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3263) = v0;
        h3263
    };
    let m3264 = v1;
    match m3264.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3265 = v4;
            match m3265.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f688(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f649(v0: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f581(f46(v0.clone(), 8u64, {
            let a3267 = {
                let a3266 = v0;
                length_bytes(a3266)
            };
            let a3268 = 8u64;
            nat_sub(a3267, a3268)
        }))?))
    } else {
        if f85(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f581(f46(v0.clone(), 14u64, {
                let a3270 = {
                    let a3269 = v0;
                    length_bytes(a3269)
                };
                let a3271 = 14u64;
                nat_sub(a3270, a3271)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f650(p3288: u64, p3289: u64, p3290: u64, p3291: Bytes) -> Bytes {
    let m3287 = (p3288, (p3289, (p3290, p3291)));
    {
        let mut m3287 = m3287;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3287;
                let m3272 = v0;
                if m3272 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3272 - 1;
                    if f117({
                        let a3273 = v1;
                        let a3274 = 0u64;
                        nat_eq(a3273, a3274)
                    }, {
                        let a3275 = v2;
                        let a3276 = 0u64;
                        nat_eq(a3275, a3276)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3277 = v1;
                            let a3278 = 16u64;
                            let a3279 = 0u64;
                            nat_quot(a3277, a3278, a3279)
                        }, ({
                            let a3280 = v2;
                            let a3281 = 1u64;
                            nat_sub(a3280, a3281)
                        }, {
                            let a3285 = f689({
                                let a3282 = v1;
                                let a3283 = 16u64;
                                let a3284 = 0u64;
                                nat_rem(a3282, a3283, a3284)
                            });
                            let a3286 = v3;
                            append_bytes(a3285, a3286)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3287 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f651(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f690(f691(f215(v0.clone()))?, v0, v1)
}

pub fn f652(v0: Bytes, v1: Bytes) -> Result<Adt27, Bytes> {
    Err::<Adt27, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f653(v0: Bytes, v1: List<Adt24>) -> Result<Adt27, Bytes> {
    Ok::<Adt27, Bytes>(Adt27::C0(v0, v1))
}

pub fn f654(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt27, Bytes>> {
    let m3292 = v1.clone();
    match m3292.uncons() {
        None => {
            Ok(f652(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3293 = f692(f537(f93(v2, f8(Str::lit("properties")))), v1.clone());
            match m3293.uncons() {
                None => {
                    let m3294 = f693(f691(f694(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3294 {
                        Ok(v5) => {
                            Ok(f653(f657(v0, f658(v5.clone())), f659(v5)))
                        }
                        Err(v4) => {
                            Ok(f652(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3295 = f695(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3295 {
                        Ok(v9) => {
                            Ok(f653(f657(v0, f658(v9.clone())), f659(v9)))
                        }
                        Err(v8) => {
                            Ok(f652(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f655(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3296 = v0;
    match m3296 {
        Adt2::C0 => {
            Ok(false)
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(_) => {
            Ok(false)
        }
        Adt2::C3(_, _) => {
            Ok(false)
        }
        Adt2::C4(_, _, _) => {
            Ok(false)
        }
        Adt2::C5(_, _) => {
            Ok(false)
        }
        Adt2::C6(_) => {
            Ok(false)
        }
        Adt2::C7(r3297) => {
            let v2 = (*r3297).clone();
            f655(v2, v1)
        }
        Adt2::C8(r3298) => {
            let v3 = (*r3298).clone();
            f655(v3, v1)
        }
        Adt2::C9 => {
            Ok(false)
        }
        Adt2::C10(r3299, r3300) => {
            let v4 = (*r3299).clone();
            let v5 = (*r3300).clone();
            if f655(v4, v1.clone())? {
                Ok(true)
            } else {
                f655(v5, v1)
            }
        }
        Adt2::C11(v6, r3301, r3302) => {
            let v7 = (*r3301).clone();
            let v8 = (*r3302).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f696(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f655(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f655(v8, v1)
                }
            }
        }
    }
}

pub fn f656(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt28, Bytes>> {
    f697(f691(f638(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f657(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f401(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f658(v0: Adt28) -> List<Bytes> {
    let m3303 = v0;
    match m3303 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f659(v0: Adt28) -> List<Adt24> {
    let m3304 = v0;
    match m3304 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f660(v0: Adt2) -> bool {
    let m3305 = f537(f93(v0.clone(), f8(Str::lit("properties"))));
    match m3305.uncons() {
        None => {
            let m3306 = f698(v0);
            match m3306 {
                None => {
                    false
                }
                Some(v3) => {
                    f699(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f661(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f662(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3307 = f700(v0.clone());
    match m3307 {
        None => {
            if f32(f58(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3308 = f93(v0, f8(Str::lit("items")));
                match m3308 {
                    None => {
                        Ok(f701(f8(Str::lit("List")), f703()))
                    }
                    Some(v3) => {
                        Ok(f701(f8(Str::lit("List")), f702(v3, {
                            let a3309 = v1;
                            let a3310 = f8(Str::lit("Item"));
                            append_bytes(a3309, a3310)
                        })?))
                    }
                }
            } else {
                f702(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f701(f8(Str::lit("Option")), f702(v2, v1)?))
        }
    }
}

pub fn f663(p3313: List<Adt2>) -> bool {
    let m3312 = p3313;
    {
        let mut m3312 = m3312;
        loop {
            let step = {
                let v0 = m3312;
                let m3311 = v0;
                match m3311.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f704(f578(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f664(v0: Adt2, v1: Bytes) -> Adt2 {
    let m3314 = v0;
    match m3314 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v6, r3315) => {
            let v7 = (*r3315).clone();
            Adt2::C1(v6, std::rc::Rc::new(v7))
        }
        Adt2::C2(v8) => {
            Adt2::C2(v8)
        }
        Adt2::C3(v14, v15) => {
            Adt2::C3(v14, v15)
        }
        Adt2::C4(v9, v10, v11) => {
            Adt2::C4(v9, v10, v11)
        }
        Adt2::C5(v12, v13) => {
            Adt2::C5(v12, v13)
        }
        Adt2::C6(v16) => {
            Adt2::C6(v16)
        }
        Adt2::C7(r3316) => {
            let v17 = (*r3316).clone();
            Adt2::C7(std::rc::Rc::new(v17))
        }
        Adt2::C8(r3317) => {
            let v5 = (*r3317).clone();
            Adt2::C8(std::rc::Rc::new(f664(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r3318, r3319) => {
            let v18 = (*r3318).clone();
            let v19 = (*r3319).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r3320, r3321) => {
            let v3 = (*r3320).clone();
            let v4 = (*r3321).clone();
            if f32(v2.clone(), v1.clone()) {
                f664(v4, v1)
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f664(v4, v1)))
            }
        }
    }
}

pub fn f665(v0: Adt2, v1: Bytes) -> bool {
    let m3322 = f93(v0, v1);
    match m3322 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f666(v0: List<Adt2>) -> bool {
    let m3323 = v0.clone();
    match m3323.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3324 = f692(f537(f93(v1, f8(Str::lit("properties")))), v0.clone());
            match m3324.uncons() {
                None => {
                    f706(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f705(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f667(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m3325 = v0;
    match m3325 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v7, r3326) => {
            let v8 = (*r3326).clone();
            Adt2::C1(v7, std::rc::Rc::new(v8))
        }
        Adt2::C2(v9) => {
            Adt2::C2(v9)
        }
        Adt2::C3(v15, v16) => {
            Adt2::C3(v15, v16)
        }
        Adt2::C4(v10, v11, v12) => {
            Adt2::C4(v10, v11, v12)
        }
        Adt2::C5(v13, v14) => {
            Adt2::C5(v13, v14)
        }
        Adt2::C6(v17) => {
            Adt2::C6(v17)
        }
        Adt2::C7(r3327) => {
            let v18 = (*r3327).clone();
            Adt2::C7(std::rc::Rc::new(v18))
        }
        Adt2::C8(r3328) => {
            let v3 = (*r3328).clone();
            Adt2::C8(std::rc::Rc::new(Adt2::C11(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r3329, r3330) => {
            let v19 = (*r3329).clone();
            let v20 = (*r3330).clone();
            Adt2::C10(std::rc::Rc::new(v19), std::rc::Rc::new(v20))
        }
        Adt2::C11(v4, r3331, r3332) => {
            let v5 = (*r3331).clone();
            let v6 = (*r3332).clone();
            Adt2::C11(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f668(p3335: List<Adt2>) -> bool {
    let m3334 = p3335;
    {
        let mut m3334 = m3334;
        loop {
            let step = {
                let v0 = m3334;
                let m3333 = v0;
                match m3333.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f707(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f669(v0: u64) -> u32 {
    let a3336 = v0;
    let a3337 = 32u64;
    if nat_lt(a3336, a3337) {
        let a3338 = v0;
        let a3339 = 16u64;
        if nat_lt(a3338, a3339) {
            let a3340 = v0;
            let a3341 = 8u64;
            if nat_lt(a3340, a3341) {
                let a3342 = v0;
                let a3343 = 4u64;
                if nat_lt(a3342, a3343) {
                    let a3344 = v0;
                    let a3345 = 2u64;
                    if nat_lt(a3344, a3345) {
                        let a3346 = v0;
                        let a3347 = 1u64;
                        if nat_lt(a3346, a3347) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3348 = v0;
                        let a3349 = 3u64;
                        if nat_lt(a3348, a3349) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3350 = v0;
                    let a3351 = 6u64;
                    if nat_lt(a3350, a3351) {
                        let a3352 = v0;
                        let a3353 = 5u64;
                        if nat_lt(a3352, a3353) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3354 = v0;
                        let a3355 = 7u64;
                        if nat_lt(a3354, a3355) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3356 = v0;
                let a3357 = 12u64;
                if nat_lt(a3356, a3357) {
                    let a3358 = v0;
                    let a3359 = 10u64;
                    if nat_lt(a3358, a3359) {
                        let a3360 = v0;
                        let a3361 = 9u64;
                        if nat_lt(a3360, a3361) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3362 = v0;
                        let a3363 = 11u64;
                        if nat_lt(a3362, a3363) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3364 = v0;
                    let a3365 = 14u64;
                    if nat_lt(a3364, a3365) {
                        let a3366 = v0;
                        let a3367 = 13u64;
                        if nat_lt(a3366, a3367) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3368 = v0;
                        let a3369 = 15u64;
                        if nat_lt(a3368, a3369) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3370 = v0;
            let a3371 = 24u64;
            if nat_lt(a3370, a3371) {
                let a3372 = v0;
                let a3373 = 20u64;
                if nat_lt(a3372, a3373) {
                    let a3374 = v0;
                    let a3375 = 18u64;
                    if nat_lt(a3374, a3375) {
                        let a3376 = v0;
                        let a3377 = 17u64;
                        if nat_lt(a3376, a3377) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3378 = v0;
                        let a3379 = 19u64;
                        if nat_lt(a3378, a3379) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3380 = v0;
                    let a3381 = 22u64;
                    if nat_lt(a3380, a3381) {
                        let a3382 = v0;
                        let a3383 = 21u64;
                        if nat_lt(a3382, a3383) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3384 = v0;
                        let a3385 = 23u64;
                        if nat_lt(a3384, a3385) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3386 = v0;
                let a3387 = 28u64;
                if nat_lt(a3386, a3387) {
                    let a3388 = v0;
                    let a3389 = 26u64;
                    if nat_lt(a3388, a3389) {
                        let a3390 = v0;
                        let a3391 = 25u64;
                        if nat_lt(a3390, a3391) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3392 = v0;
                        let a3393 = 27u64;
                        if nat_lt(a3392, a3393) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3394 = v0;
                    let a3395 = 30u64;
                    if nat_lt(a3394, a3395) {
                        let a3396 = v0;
                        let a3397 = 29u64;
                        if nat_lt(a3396, a3397) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3398 = v0;
                        let a3399 = 31u64;
                        if nat_lt(a3398, a3399) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3400 = v0;
        let a3401 = 48u64;
        if nat_lt(a3400, a3401) {
            let a3402 = v0;
            let a3403 = 40u64;
            if nat_lt(a3402, a3403) {
                let a3404 = v0;
                let a3405 = 36u64;
                if nat_lt(a3404, a3405) {
                    let a3406 = v0;
                    let a3407 = 34u64;
                    if nat_lt(a3406, a3407) {
                        let a3408 = v0;
                        let a3409 = 33u64;
                        if nat_lt(a3408, a3409) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3410 = v0;
                        let a3411 = 35u64;
                        if nat_lt(a3410, a3411) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3412 = v0;
                    let a3413 = 38u64;
                    if nat_lt(a3412, a3413) {
                        let a3414 = v0;
                        let a3415 = 37u64;
                        if nat_lt(a3414, a3415) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3416 = v0;
                        let a3417 = 39u64;
                        if nat_lt(a3416, a3417) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3418 = v0;
                let a3419 = 44u64;
                if nat_lt(a3418, a3419) {
                    let a3420 = v0;
                    let a3421 = 42u64;
                    if nat_lt(a3420, a3421) {
                        let a3422 = v0;
                        let a3423 = 41u64;
                        if nat_lt(a3422, a3423) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3424 = v0;
                        let a3425 = 43u64;
                        if nat_lt(a3424, a3425) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3426 = v0;
                    let a3427 = 46u64;
                    if nat_lt(a3426, a3427) {
                        let a3428 = v0;
                        let a3429 = 45u64;
                        if nat_lt(a3428, a3429) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3430 = v0;
                        let a3431 = 47u64;
                        if nat_lt(a3430, a3431) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3432 = v0;
            let a3433 = 56u64;
            if nat_lt(a3432, a3433) {
                let a3434 = v0;
                let a3435 = 52u64;
                if nat_lt(a3434, a3435) {
                    let a3436 = v0;
                    let a3437 = 50u64;
                    if nat_lt(a3436, a3437) {
                        let a3438 = v0;
                        let a3439 = 49u64;
                        if nat_lt(a3438, a3439) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3440 = v0;
                        let a3441 = 51u64;
                        if nat_lt(a3440, a3441) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3442 = v0;
                    let a3443 = 54u64;
                    if nat_lt(a3442, a3443) {
                        let a3444 = v0;
                        let a3445 = 53u64;
                        if nat_lt(a3444, a3445) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3446 = v0;
                        let a3447 = 55u64;
                        if nat_lt(a3446, a3447) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3448 = v0;
                let a3449 = 60u64;
                if nat_lt(a3448, a3449) {
                    let a3450 = v0;
                    let a3451 = 58u64;
                    if nat_lt(a3450, a3451) {
                        let a3452 = v0;
                        let a3453 = 57u64;
                        if nat_lt(a3452, a3453) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3454 = v0;
                        let a3455 = 59u64;
                        if nat_lt(a3454, a3455) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3456 = v0;
                    let a3457 = 62u64;
                    if nat_lt(a3456, a3457) {
                        let a3458 = v0;
                        let a3459 = 61u64;
                        if nat_lt(a3458, a3459) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3460 = v0;
                        let a3461 = 63u64;
                        if nat_lt(a3460, a3461) {
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

pub fn f670(v0: u32) -> u32 {
    let a3464 = {
        let a3462 = f708(v0, 6u32, 26u32);
        let a3463 = f708(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3462, a3463)
    };
    let a3465 = f708(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3464, a3465)
}

pub fn f671(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3471 = {
        let a3466 = v0;
        let a3467 = v1;
        fixed_u32::bit_and(a3466, a3467)
    };
    let a3472 = {
        let a3469 = {
            let a3468 = v0;
            fixed_u32::bit_not(a3468)
        };
        let a3470 = v2;
        fixed_u32::bit_and(a3469, a3470)
    };
    fixed_u32::bit_xor(a3471, a3472)
}

pub fn f672(v0: u32) -> u32 {
    let a3475 = {
        let a3473 = f708(v0, 2u32, 30u32);
        let a3474 = f708(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3473, a3474)
    };
    let a3476 = f708(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3475, a3476)
}

pub fn f673(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3485 = {
        let a3481 = {
            let a3477 = v0;
            let a3478 = v1;
            fixed_u32::bit_and(a3477, a3478)
        };
        let a3482 = {
            let a3479 = v0;
            let a3480 = v2;
            fixed_u32::bit_and(a3479, a3480)
        };
        fixed_u32::bit_xor(a3481, a3482)
    };
    let a3486 = {
        let a3483 = v1;
        let a3484 = v2;
        fixed_u32::bit_and(a3483, a3484)
    };
    fixed_u32::bit_xor(a3485, a3486)
}

pub fn f674(v0: Adt25) -> Adt25 {
    Adt25::C0(match v0.clone() {
        Adt25::C0(_, h3487, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3487
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h3488, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3488
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, h3489, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3489
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, h3490, _, _, _, _, _, _, _, _, _, _, _) => {
            h3490
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h3491, _, _, _, _, _, _, _, _, _, _) => {
            h3491
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h3492, _, _, _, _, _, _, _, _, _) => {
            h3492
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h3493, _, _, _, _, _, _, _, _) => {
            h3493
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h3494, _, _, _, _, _, _, _) => {
            h3494
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h3495, _, _, _, _, _, _) => {
            h3495
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, h3496, _, _, _, _, _) => {
            h3496
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, h3497, _, _, _, _) => {
            h3497
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3498, _, _, _) => {
            h3498
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3499, _, _) => {
            h3499
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3500, _) => {
            h3500
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3501) => {
            h3501
        }
    }, f589(f709(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3502, _) => {
            h3502
        }
    }), f589(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h3503, _, _, _, _, _, _) => {
            h3503
        }
    }, f589(f710(match v0.clone() {
        Adt25::C0(_, h3504, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3504
        }
    }), match v0 {
        Adt25::C0(h3505, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3505
        }
    }))))
}

pub fn f675(v0: Fn21, p3509: u64, p3510: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3508 = (p3509, p3510);
    {
        let mut m3508 = m3508;
        loop {
            let step = {
                let (v1, v2) = m3508;
                let m3506 = v1;
                if m3506 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3506 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3507: Fn21 = v0.clone();
                        c3507.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f676(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3511, _) = v0;
        h3511
    };
    let v2: u32 = {
        let (h3512, _) = {
            let (_, h3513) = v0;
            h3513
        };
        h3512
    };
    let v3: u32 = {
        let (_, h3514) = {
            let (_, h3515) = v0;
            h3515
        };
        h3514
    };
    ({
        let a3516 = v1;
        let a3517 = 2u64;
        let a3518 = 0u64;
        nat_quot(a3516, a3517, a3518)
    }, (f677(v2, 1u32), {
        let a3522 = {
            let a3519 = v1;
            let a3520 = 2u64;
            let a3521 = 0u64;
            nat_rem(a3519, a3520, a3521)
        };
        let a3523 = 1u64;
        if nat_eq(a3522, a3523) {
            let a3524 = v3;
            let a3525 = v2;
            fixed_u32::bit_or(a3524, a3525)
        } else {
            v3
        }
    }))
}

pub fn f677(v0: u32, v1: u32) -> u32 {
    let m3528 = {
        let a3526 = v0;
        let a3527 = v1;
        fixed_u32::shift_left(a3526, a3527)
    };
    match m3528 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f678(v0: Bytes, v1: Adt2) -> bool {
    let v2: Bytes = f629(v1);
    if f32(v0.clone(), f8(Str::lit("number"))) {
        if f32(v2.clone(), f8(Str::lit("integer"))) {
            true
        } else {
            f32(v2, f8(Str::lit("number")))
        }
    } else {
        f32(v0, v2)
    }
}

pub fn f679(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m3529 = v0;
    if m3529 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m3529 - 1;
        let a3531 = {
            let a3530 = v1.clone();
            length_bytes(a3530)
        };
        let a3532 = v2;
        if nat_le(a3531, a3532) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m3533 = f711(v1.clone(), v2)?;
            match m3533 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m3537 = f679(v3, v1, {
                        let a3535 = v2;
                        let a3536 = {
                            let (_, h3534) = v4;
                            h3534
                        };
                        nat_add(a3535, a3536)?
                    })?;
                    match m3537 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h3538, _) = v4;
                                h3538
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f680(v0: Bytes) -> R<Result<((bool, bool), List<Adt29>), Bytes>> {
    let v1: u64 = {
        let a3539 = v0.clone();
        length_bytes(a3539)
    };
    let v2: bool = {
        let a3540 = 0u64;
        let a3541 = v1;
        if nat_lt(a3540, a3541) {
            let a3542 = f115(v0.clone(), 0u64)?;
            let a3543 = 94u64;
            nat_eq(a3542, a3543)
        } else {
            false
        }
    };
    let v3: bool = {
        let a3544 = 0u64;
        let a3545 = v1;
        if nat_lt(a3544, a3545) {
            f117({
                let a3548 = f115(v0.clone(), {
                    let a3546 = v1;
                    let a3547 = 1u64;
                    nat_sub(a3546, a3547)
                })?;
                let a3549 = 36u64;
                nat_eq(a3548, a3549)
            }, f712({
                let a3552 = f115(v0.clone(), {
                    let a3550 = v1;
                    let a3551 = 2u64;
                    nat_sub(a3550, a3551)
                })?;
                let a3553 = 92u64;
                nat_eq(a3552, a3553)
            }))
        } else {
            false
        }
    };
    let m3558 = f713({
        let a3554 = v1;
        let a3555 = 1u64;
        nat_add(a3554, a3555)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a3556 = v1;
        let a3557 = 1u64;
        nat_sub(a3556, a3557)
    } else {
        v1
    })?;
    match m3558 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt29>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt29>), Bytes>(v4))
        }
    }
}

pub fn f681(v0: u64, v1: List<Adt29>, v2: List<u64>, v3: bool) -> R<bool> {
    let m3559 = v0;
    if m3559 == 0 {
        Ok(false)
    } else {
        let v4 = m3559 - 1;
        let m3560 = v1.clone();
        match m3560.uncons() {
            None => {
                if v3 {
                    let a3561 = f634(v2)?;
                    let a3562 = 0u64;
                    Ok(nat_eq(a3561, a3562))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m3563 = v5.clone();
                match m3563 {
                    Adt29::C0(v7, v8, v9) => {
                        let a3564 = v9;
                        let a3565 = 1u64;
                        if nat_eq(a3564, a3565) {
                            if f681(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m3566 = v2;
                                match m3566.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f714(v5, v10) {
                                            f681(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a3567 = v9;
                            let a3568 = 2u64;
                            if nat_eq(a3567, a3568) {
                                let m3569 = v2;
                                match m3569.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f714(v5, v12) {
                                            f681(v4, List::cons(Adt29::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a3570 = v9;
                                let a3571 = 3u64;
                                if nat_eq(a3570, a3571) {
                                    if f681(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m3572 = v2;
                                        match m3572.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f714(v5, v14) {
                                                    f681(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m3573 = v2;
                                    match m3573.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f714(v5, v16) {
                                                f681(v4, v6, v17, v3)
                                            } else {
                                                Ok(false)
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

pub fn f682(v0: List<Adt29>) -> R<u64> {
    let m3574 = v0;
    match m3574.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3575 = f682(v2)?;
            let a3576 = 1u64;
            nat_add(a3575, a3576)
        }
    }
}

pub fn f683(v0: List<Adt29>, v1: List<u64>, v2: bool) -> R<bool> {
    if f681({
        let a3579 = {
            let a3577 = f682(v0.clone())?;
            let a3578 = f634(v1.clone())?;
            nat_add(a3577, a3578)?
        };
        let a3580 = 2u64;
        nat_add(a3579, a3580)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f715(v0, v1, v2)
    }
}

pub fn f684(v0: u64) -> R<u64> {
    let m3581 = v0;
    if m3581 == 0 {
        Ok(1u64)
    } else {
        let v1 = m3581 - 1;
        let a3582 = f684(v1)?;
        let a3583 = 10u64;
        nat_mul(a3582, a3583)
    }
}

pub fn f685(p3597: u64, v1: Bytes, p3598: u64, p3599: Bytes) -> R<Bytes> {
    let m3596 = (p3597, (p3598, p3599));
    {
        let mut m3596 = m3596;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3596;
                let m3584 = v0;
                if m3584 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3584 - 1;
                    let m3587 = {
                        let a3585 = v1.clone();
                        let a3586 = v2;
                        index_bytes(a3585, a3586)
                    };
                    match m3587 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3588 = v2;
                                let a3589 = 1u64;
                                nat_add(a3588, a3589)?
                            }, {
                                let a3594 = v3;
                                let a3595 = {
                                    let a3590 = v6;
                                    let a3591 = 126u64;
                                    if nat_eq(a3590, a3591) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a3592 = v6;
                                        let a3593 = 47u64;
                                        if nat_eq(a3592, a3593) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f46(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a3594, a3595)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: Fn22, p3604: u64, p3605: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3603 = (p3604, p3605);
    {
        let mut m3603 = m3603;
        loop {
            let step = {
                let (v1, v2) = m3603;
                let m3600 = v1;
                if m3600 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3600 - 1;
                    let m3602 = {
                        let c3601: Fn22 = v0.clone();
                        c3601.apply(v2.clone())?
                    };
                    match m3602 {
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
                Ok(next) => { m3603 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f687(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f716(v0)
}

pub fn f688(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3610 = {
            let a3608 = {
                let a3606 = v0.clone();
                length_list(a3606)
            };
            let a3609 = {
                let a3607 = v1.clone();
                length_list(a3607)
            };
            nat_add(a3608, a3609)?
        };
        let a3611 = 1u64;
        nat_add(a3610, a3611)?
    };
    Ok(f463({
        let (_, h3612) = {
            let (_, h3613) = {
                let (h3614, _) = f717(Fn23::F718, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3614
            };
            h3613
        };
        h3612
    }))
}

pub fn f689(v0: u64) -> Bytes {
    f46({
        let a3615 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3615)
    }, v0, 1u64)
}

pub fn f690(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3616 = v1;
    match m3616.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3617 = f719(v0, v3.clone())?;
            match m3617 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f550(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f52(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3618 = f690(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3618 {
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

pub fn f691(p3622: List<Bytes>) -> R<bool> {
    let m3621 = p3622;
    {
        let mut m3621 = m3621;
        loop {
            let step = {
                let v0 = m3621;
                let m3619 = v0;
                match m3619.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3620 = f720(v1)?;
                        match m3620 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f52(v3, f721(false, v2.clone())?) {
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
                Ok(next) => { m3621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f692(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m3623 = v0;
    match m3623.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f722(v1.clone(), {
                let (h3624, _) = v2.clone();
                h3624
            }) {
                List::cons({
                    let (h3625, _) = v2;
                    h3625
                }, f692(v3, v1))
            } else {
                f692(v3, v1)
            }
        }
    }
}

pub fn f693(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m3626 = v2;
    match m3626.uncons() {
        None => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v4, v5)) => {
            let m3627 = f723(v4.clone())?;
            match m3627 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3628 = f724(v0, v6.clone())?;
                    match m3628 {
                        None => {
                            Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f52(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3631 = f725(v4, {
                                    let a3629 = v1.clone();
                                    let a3630 = f386(v6)?;
                                    append_bytes(a3629, a3630)
                                })?;
                                match m3631 {
                                    Ok(v9) => {
                                        let m3632 = f693(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3632 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f586(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f658(v11.clone())), f541(f585(v9), f659(v11)))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<Adt28, Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<Adt28, Bytes>(v8))
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

pub fn f694(v0: List<Adt2>) -> R<List<Bytes>> {
    let m3633 = v0;
    match m3633.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3634 = f723(v1)?;
            match m3634 {
                None => {
                    f694(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f694(v2)?))
                }
            }
        }
    }
}

pub fn f695(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    f726(f691(f727(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f696(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3635 = f92(Some(v0));
    match m3635 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3636 = f649(v2)?;
            match m3636 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f32(v3, v1))
                }
            }
        }
    }
}

pub fn f697(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt28, Bytes>> {
    let m3637 = v2;
    match m3637.uncons() {
        None => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v5, v6)) => {
            let m3639 = f728(v0, {
                let (h3638, _) = v5.clone();
                h3638
            })?;
            match m3639 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3640, _) = v5;
                        h3640
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f52(v7.clone(), f729(v0, f638(v6.clone()))?) {
                        Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3645 = f662({
                            let (_, h3641) = v5.clone();
                            h3641
                        }, {
                            let a3643 = v1.clone();
                            let a3644 = f386({
                                let (h3642, _) = v5.clone();
                                h3642
                            })?;
                            append_bytes(a3643, a3644)
                        })?;
                        match m3645 {
                            Ok(v9) => {
                                let v10: Bytes = if f52({
                                    let (h3646, _) = v5;
                                    h3646
                                }, v3.clone()) {
                                    f586(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f586(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3647 = f697(v0, v1, v6, v3, v4)?;
                                match m3647 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f658(v12.clone())), f541(f585(v9), f659(v12)))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt28, Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<Adt28, Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f698(v0: Adt2) -> Option<Adt2> {
    let m3648 = f93(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3648 {
        None => {
            f730(v0)
        }
        Some(v1) => {
            if f699(v1.clone()) {
                let m3649 = f730(v0);
                match m3649 {
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

pub fn f699(v0: Adt2) -> bool {
    let m3650 = v0;
    match m3650 {
        Adt2::C0 => {
            false
        }
        Adt2::C1(_, _) => {
            false
        }
        Adt2::C2(v3) => {
            !v3
        }
        Adt2::C3(_, _) => {
            false
        }
        Adt2::C4(_, _, _) => {
            false
        }
        Adt2::C5(_, _) => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            false
        }
        Adt2::C9 => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11(_, _, _) => {
            false
        }
    }
}

pub fn f700(v0: Adt2) -> Option<Adt2> {
    let m3651 = f188(f93(v0, f8(Str::lit("oneOf"))));
    match m3651.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3652 = v2;
            match m3652.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m3653 = v4;
                    match m3653.uncons() {
                        None => {
                            if f731(v3.clone()) {
                                Some(v1)
                            } else {
                                if f731(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt2>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt2>
                        }
                    }
                }
            }
        }
    }
}

pub fn f701(v0: Bytes, v1: Result<Adt27, Bytes>) -> Result<Adt27, Bytes> {
    let m3654 = v1;
    match m3654 {
        Ok(v3) => {
            f653(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f586(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f585(v3))
        }
        Err(v2) => {
            Err::<Adt27, Bytes>(v2)
        }
    }
}

pub fn f702(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3655 = f700(v0.clone());
    match m3655 {
        None => {
            if f32(f58(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3656 = f93(v0, f8(Str::lit("items")));
                match m3656 {
                    None => {
                        Ok(f701(f8(Str::lit("List")), f703()))
                    }
                    Some(v3) => {
                        Ok(f701(f8(Str::lit("List")), f725(v3, {
                            let a3657 = v1;
                            let a3658 = f8(Str::lit("Item"));
                            append_bytes(a3657, a3658)
                        })?))
                    }
                }
            } else {
                f725(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f701(f8(Str::lit("Option")), f725(v2, v1)?))
        }
    }
}

pub fn f703() -> Result<Adt27, Bytes> {
    Ok::<Adt27, Bytes>(Adt27::C0(f582(), List::cons(Adt24::C2, List::<Adt24>::nil())))
}

pub fn f704(p3662: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m3661 = p3662;
    {
        let mut m3661 = m3661;
        loop {
            let step = {
                let v0 = m3661;
                let m3659 = v0;
                match m3659.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f52({
                            let (h3660, _) = v2;
                            h3660
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f705(p3666: List<Adt2>, v1: Bytes, p3667: List<Bytes>) -> bool {
    let m3665 = (p3666, p3667);
    {
        let mut m3665 = m3665;
        loop {
            let step = {
                let (v0, v2) = m3665;
                let m3663 = v0;
                match m3663.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3664 = f732(v3, v1.clone());
                        match m3664 {
                            None => {
                                Err::<(List<Adt2>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f52(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt2>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt2>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3665 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f706(p3670: List<Adt2>, p3671: List<Bytes>) -> bool {
    let m3669 = (p3670, p3671);
    {
        let mut m3669 = m3669;
        loop {
            let step = {
                let (v0, v1) = m3669;
                let m3668 = v0;
                match m3668.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f58(v2, f8(Str::lit("type")));
                        if !f32(v4.clone(), Bytes::lit(&[])) {
                            if !f52(v4.clone(), v1.clone()) {
                                if !if if f32(v4.clone(), f8(Str::lit("integer"))) {
                                    f52(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f32(v4.clone(), f8(Str::lit("number"))) {
                                        f52(f8(Str::lit("integer")), v1.clone())
                                    } else {
                                        false
                                    }
                                } {
                                    Ok::<(List<Adt2>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt2>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt2>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt2>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f707(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m3672 = f93(v0.clone(), f8(Str::lit("anyOf")));
        match m3672 {
            None => {
                Adt2::C9
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f704(f578(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f663(f306(v2))
    } else {
        false
    }
}

pub fn f708(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3673 = f544(v0, v1);
    let a3674 = f677(v0, v2);
    fixed_u32::bit_or(a3673, a3674)
}

pub fn f709(v0: u32) -> u32 {
    let a3677 = {
        let a3675 = f708(v0, 17u32, 15u32);
        let a3676 = f708(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3675, a3676)
    };
    let a3678 = f544(v0, 10u32);
    fixed_u32::bit_xor(a3677, a3678)
}

pub fn f710(v0: u32) -> u32 {
    let a3681 = {
        let a3679 = f708(v0, 7u32, 25u32);
        let a3680 = f708(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3679, a3680)
    };
    let a3682 = f544(v0, 3u32);
    fixed_u32::bit_xor(a3681, a3682)
}

pub fn f711(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3685 = {
        let a3683 = v0.clone();
        let a3684 = v1;
        index_bytes(a3683, a3684)
    };
    match m3685 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            let a3686 = v3;
            let a3687 = 128u64;
            if nat_lt(a3686, a3687) {
                Ok(Some((v3, 1u64)))
            } else {
                if f117({
                    let a3688 = 194u64;
                    let a3689 = v3;
                    nat_le(a3688, a3689)
                }, {
                    let a3690 = v3;
                    let a3691 = 223u64;
                    nat_le(a3690, a3691)
                }) {
                    let m3694 = f733(v0, {
                        let a3692 = v1;
                        let a3693 = 1u64;
                        nat_add(a3692, a3693)?
                    })?;
                    match m3694 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f734({
                                let a3699 = {
                                    let a3697 = {
                                        let a3695 = v3;
                                        let a3696 = 192u64;
                                        nat_sub(a3695, a3696)
                                    };
                                    let a3698 = 64u64;
                                    nat_mul(a3697, a3698)?
                                };
                                let a3700 = v4;
                                nat_add(a3699, a3700)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f117({
                        let a3701 = 224u64;
                        let a3702 = v3;
                        nat_le(a3701, a3702)
                    }, {
                        let a3703 = v3;
                        let a3704 = 239u64;
                        nat_le(a3703, a3704)
                    }) {
                        let m3707 = f733(v0.clone(), {
                            let a3705 = v1;
                            let a3706 = 1u64;
                            nat_add(a3705, a3706)?
                        })?;
                        match m3707 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3710 = f733(v0, {
                                    let a3708 = v1;
                                    let a3709 = 2u64;
                                    nat_add(a3708, a3709)?
                                })?;
                                match m3710 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f734({
                                            let a3719 = {
                                                let a3713 = {
                                                    let a3711 = v3;
                                                    let a3712 = 224u64;
                                                    nat_sub(a3711, a3712)
                                                };
                                                let a3714 = 4096u64;
                                                nat_mul(a3713, a3714)?
                                            };
                                            let a3720 = {
                                                let a3717 = {
                                                    let a3715 = v5;
                                                    let a3716 = 64u64;
                                                    nat_mul(a3715, a3716)?
                                                };
                                                let a3718 = v6;
                                                nat_add(a3717, a3718)?
                                            };
                                            nat_add(a3719, a3720)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f117({
                            let a3721 = 240u64;
                            let a3722 = v3;
                            nat_le(a3721, a3722)
                        }, {
                            let a3723 = v3;
                            let a3724 = 244u64;
                            nat_le(a3723, a3724)
                        }) {
                            let m3727 = f733(v0.clone(), {
                                let a3725 = v1;
                                let a3726 = 1u64;
                                nat_add(a3725, a3726)?
                            })?;
                            match m3727 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3730 = f733(v0.clone(), {
                                        let a3728 = v1;
                                        let a3729 = 2u64;
                                        nat_add(a3728, a3729)?
                                    })?;
                                    match m3730 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3733 = f733(v0, {
                                                let a3731 = v1;
                                                let a3732 = 3u64;
                                                nat_add(a3731, a3732)?
                                            })?;
                                            match m3733 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f734({
                                                        let a3746 = {
                                                            let a3736 = {
                                                                let a3734 = v3;
                                                                let a3735 = 240u64;
                                                                nat_sub(a3734, a3735)
                                                            };
                                                            let a3737 = 262144u64;
                                                            nat_mul(a3736, a3737)?
                                                        };
                                                        let a3747 = {
                                                            let a3744 = {
                                                                let a3738 = v7;
                                                                let a3739 = 4096u64;
                                                                nat_mul(a3738, a3739)?
                                                            };
                                                            let a3745 = {
                                                                let a3742 = {
                                                                    let a3740 = v8;
                                                                    let a3741 = 64u64;
                                                                    nat_mul(a3740, a3741)?
                                                                };
                                                                let a3743 = v9;
                                                                nat_add(a3742, a3743)?
                                                            };
                                                            nat_add(a3744, a3745)?
                                                        };
                                                        nat_add(a3746, a3747)?
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

pub fn f712(v0: bool) -> bool {
    !v0
}

pub fn f713(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt29>, Bytes>> {
    let m3748 = v0;
    if m3748 == 0 {
        Ok(Ok::<List<Adt29>, Bytes>(List::<Adt29>::nil()))
    } else {
        let v4 = m3748 - 1;
        let a3749 = v3;
        let a3750 = v2;
        if nat_le(a3749, a3750) {
            Ok(Ok::<List<Adt29>, Bytes>(List::<Adt29>::nil()))
        } else {
            let v5: u64 = f115(v1.clone(), v2)?;
            let m3799 = {
                let a3751 = v5;
                let a3752 = 91u64;
                if nat_eq(a3751, a3752) {
                    let a3755 = f115(v1.clone(), {
                        let a3753 = v2;
                        let a3754 = 1u64;
                        nat_add(a3753, a3754)?
                    })?;
                    let a3756 = 94u64;
                    if nat_eq(a3755, a3756) {
                        let m3760 = f735({
                            let a3757 = v1.clone();
                            length_bytes(a3757)
                        }, v1.clone(), {
                            let a3758 = v2;
                            let a3759 = 2u64;
                            nat_add(a3758, a3759)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3760 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h3761, _) = v7;
                                    h3761
                                }), {
                                    let (_, h3762) = v7;
                                    h3762
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m3766 = f735({
                            let a3763 = v1.clone();
                            length_bytes(a3763)
                        }, v1.clone(), {
                            let a3764 = v2;
                            let a3765 = 1u64;
                            nat_add(a3764, a3765)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3766 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h3767, _) = v9;
                                    h3767
                                }), {
                                    let (_, h3768) = v9;
                                    h3768
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a3769 = v5;
                    let a3770 = 46u64;
                    if nat_eq(a3769, a3770) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a3771 = v2;
                            let a3772 = 1u64;
                            nat_add(a3771, a3772)?
                        }))
                    } else {
                        let a3773 = v5;
                        let a3774 = 40u64;
                        if if if if if if nat_eq(a3773, a3774) {
                            true
                        } else {
                            let a3775 = v5;
                            let a3776 = 41u64;
                            nat_eq(a3775, a3776)
                        } {
                            true
                        } else {
                            let a3777 = v5;
                            let a3778 = 124u64;
                            if nat_eq(a3777, a3778) {
                                true
                            } else {
                                let a3779 = v5;
                                let a3780 = 123u64;
                                nat_eq(a3779, a3780)
                            }
                        } {
                            true
                        } else {
                            let a3781 = v5;
                            let a3782 = 125u64;
                            if if nat_eq(a3781, a3782) {
                                true
                            } else {
                                let a3783 = v5;
                                let a3784 = 92u64;
                                nat_eq(a3783, a3784)
                            } {
                                true
                            } else {
                                let a3785 = v5;
                                let a3786 = 94u64;
                                if nat_eq(a3785, a3786) {
                                    true
                                } else {
                                    let a3787 = v5;
                                    let a3788 = 36u64;
                                    nat_eq(a3787, a3788)
                                }
                            }
                        } {
                            true
                        } else {
                            let a3789 = v5;
                            let a3790 = 42u64;
                            if if nat_eq(a3789, a3790) {
                                true
                            } else {
                                let a3791 = v5;
                                let a3792 = 43u64;
                                nat_eq(a3791, a3792)
                            } {
                                true
                            } else {
                                let a3793 = v5;
                                let a3794 = 63u64;
                                nat_eq(a3793, a3794)
                            }
                        } {
                            true
                        } else {
                            let a3795 = 127u64;
                            let a3796 = v5;
                            nat_lt(a3795, a3796)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f46(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a3797 = v2;
                                let a3798 = 1u64;
                                nat_add(a3797, a3798)?
                            }))
                        }
                    }
                }
            };
            match m3799 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h3800) = v11.clone();
                        h3800
                    };
                    let v13: u64 = {
                        let a3801 = v12;
                        let a3802 = v3;
                        if nat_lt(a3801, a3802) {
                            f736(f115(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a3803 = v12;
                    let a3804 = v3;
                    if if nat_lt(a3803, a3804) {
                        let a3805 = f115(v1.clone(), v12)?;
                        let a3806 = 123u64;
                        nat_eq(a3805, a3806)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt29>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m3811 = f713(v4, v1, {
                            let a3807 = 0u64;
                            let a3808 = v13;
                            if nat_lt(a3807, a3808) {
                                let a3809 = v12;
                                let a3810 = 1u64;
                                nat_add(a3809, a3810)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m3811 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt29>, Bytes>(List::cons(Adt29::C0({
                                    let (h3812, _) = {
                                        let (h3813, _) = v11.clone();
                                        h3813
                                    };
                                    h3812
                                }, {
                                    let (_, h3814) = {
                                        let (h3815, _) = v11;
                                        h3815
                                    };
                                    h3814
                                }, v13), v15)))
                            }
                            Err(v14) => {
                                Ok(Err::<List<Adt29>, Bytes>(v14))
                            }
                        }
                    }
                }
                Err(v10) => {
                    Ok(Err::<List<Adt29>, Bytes>(v10))
                }
            }
        }
    }
}

pub fn f714(v0: Adt29, v1: u64) -> bool {
    let m3816 = v0;
    match m3816 {
        Adt29::C0(v2, v3, _) => {
            if v2 {
                f712(f737(v3, v1))
            } else {
                f737(v3, v1)
            }
        }
    }
}

pub fn f715(v0: List<Adt29>, p3823: List<u64>, v2: bool) -> R<bool> {
    let m3822 = p3823;
    {
        let mut m3822 = m3822;
        loop {
            let step = {
                let v1 = m3822;
                let m3817 = v1;
                match m3817.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f681({
                            let a3820 = {
                                let a3818 = f682(v0.clone())?;
                                let a3819 = f634(v4.clone())?;
                                nat_add(a3818, a3819)?
                            };
                            let a3821 = 2u64;
                            nat_add(a3820, a3821)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3822 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f716(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3824, _) = v0;
        h3824
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3825) = v0;
        h3825
    };
    let m3826 = v1;
    match m3826.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3827 = v4;
            match m3827.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f738(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f717(v0: Fn23, p3832: u64, p3833: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3831 = (p3832, p3833);
    {
        let mut m3831 = m3831;
        loop {
            let step = {
                let (v1, v2) = m3831;
                let m3828 = v1;
                if m3828 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3828 - 1;
                    let m3830 = {
                        let c3829: Fn23 = v0.clone();
                        c3829.apply(v2.clone())
                    };
                    match m3830 {
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
                Ok(next) => { m3831 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f718(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f739(v0)
}

pub fn f719(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3834 = v1;
    match m3834 {
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
                let a3845 = if f117(v9, {
                    let a3843 = 0u64;
                    let a3844 = v10;
                    nat_lt(a3843, a3844)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3846 = f308(false, v10)?;
                append_bytes(a3845, a3846)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3837 = f740({
                let a3835 = f8(Str::lit("n"));
                let a3836 = f309(false, v4, v5)?;
                append_bytes(a3835, a3836)
            })?;
            match m3837 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3841 = f8(Str::lit("nMinus"));
                        let a3842 = f46(v6.clone(), 1u64, {
                            let a3839 = {
                                let a3838 = v6;
                                length_bytes(a3838)
                            };
                            let a3840 = 1u64;
                            nat_sub(a3839, a3840)
                        });
                        append_bytes(a3841, a3842)
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
            f724(v0, v2)
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

pub fn f720(v0: Bytes) -> R<Option<Bytes>> {
    let m3847 = f741(v0)?;
    match m3847 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f742(v1)?))
        }
    }
}

pub fn f721(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3848 = v1;
    match m3848.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3849 = f724(v0, v2)?;
            match m3849 {
                None => {
                    f721(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f721(v0, v3)?))
                }
            }
        }
    }
}

pub fn f722(p3853: List<Adt2>, v1: Bytes) -> bool {
    let m3852 = p3853;
    {
        let mut m3852 = m3852;
        loop {
            let step = {
                let v0 = m3852;
                let m3850 = v0;
                match m3850.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3851 = f732(v2, v1.clone());
                        if match m3851 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt2>, bool>(v3)
                        } else {
                            Err::<List<Adt2>, bool>(false)
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

pub fn f723(v0: Adt2) -> R<Option<Bytes>> {
    let m3854 = f92(f93(v0.clone(), f8(Str::lit("$ref"))));
    match m3854 {
        None => {
            Ok(f92(f93(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f649(v1)
        }
    }
}

pub fn f724(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f740(v1)
    } else {
        f720(v1)
    }
}

pub fn f725(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    f743(f620(v0), v1)
}

pub fn f726(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m3855 = v2;
    match m3855.uncons() {
        None => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::<Bytes>::nil(), List::<Adt24>::nil())))
        }
        Some((v5, v6)) => {
            let m3856 = f732(v5.clone(), v3.clone());
            match m3856 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3857 = f724(v0, v7.clone())?;
                    match m3857 {
                        None => {
                            Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f52(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt28, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3860 = f656({
                                    let a3858 = v1.clone();
                                    let a3859 = f386(v7)?;
                                    append_bytes(a3858, a3859)
                                }, f580(f537(f93(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f215(f188(f93(v5, f8(Str::lit("required"))))), true)?;
                                match m3860 {
                                    Ok(v10) => {
                                        let m3861 = f726(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3861 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f401(f658(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f658(v12.clone())), f541(f659(v10), f659(v12)))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt28, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt28, Bytes>(v9))
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

pub fn f727(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3862 = v0;
    match m3862.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3863 = f732(v2, v1.clone());
            match m3863 {
                None => {
                    f727(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f727(v3, v1))
                }
            }
        }
    }
}

pub fn f728(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3864 = f724(v0, v1)?;
    match m3864 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f32(v2.clone(), f8(Str::lit("mk"))) {
                f618(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f729(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3865 = v1;
    match m3865.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3866 = f728(v0, v2)?;
            match m3866 {
                None => {
                    f729(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f729(v0, v3)?))
                }
            }
        }
    }
}

pub fn f730(v0: Adt2) -> Option<Adt2> {
    let m3867 = f537(f93(v0, f8(Str::lit("patternProperties"))));
    match m3867.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3868 = v2;
            match m3868.uncons() {
                None => {
                    Some({
                        let (_, h3869) = v1;
                        h3869
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f731(v0: Adt2) -> bool {
    f32(f58(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f732(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f52(v1.clone(), f215(f188(f93(v0.clone(), f8(Str::lit("required")))))) {
        let m3870 = f93(v0, f8(Str::lit("properties")));
        match m3870 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3871 = f93(v2, v1);
                match m3871 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f92(f93(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f733(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3874 = {
        let a3872 = v0;
        let a3873 = v1;
        index_bytes(a3872, a3873)
    };
    match m3874 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f117({
                let a3875 = 128u64;
                let a3876 = v3;
                nat_le(a3875, a3876)
            }, {
                let a3877 = v3;
                let a3878 = 191u64;
                nat_le(a3877, a3878)
            }) {
                Ok(Some({
                    let a3879 = v3;
                    let a3880 = 128u64;
                    nat_sub(a3879, a3880)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f734(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f117({
        let a3881 = v1;
        let a3882 = v0;
        nat_le(a3881, a3882)
    }, f117({
        let a3883 = v0;
        let a3884 = v2;
        nat_le(a3883, a3884)
    }, f203({
        let a3885 = v0;
        let a3886 = 55296u64;
        nat_lt(a3885, a3886)
    }, {
        let a3887 = 57343u64;
        let a3888 = v0;
        nat_lt(a3887, a3888)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f735(p3925: u64, v1: Bytes, p3926: u64, p3927: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3924 = (p3925, (p3926, p3927));
    {
        let mut m3924 = m3924;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3924;
                let m3889 = v0;
                if m3889 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3889 - 1;
                    let v5: u64 = f115(v1.clone(), v2)?;
                    let a3891 = {
                        let a3890 = v1.clone();
                        length_bytes(a3890)
                    };
                    let a3892 = v2;
                    if nat_le(a3891, a3892) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3893 = v5;
                        let a3894 = 93u64;
                        if nat_eq(a3893, a3894) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3895 = v2;
                                let a3896 = 1u64;
                                nat_add(a3895, a3896)?
                            }))))
                        } else {
                            let a3897 = v5;
                            let a3898 = 92u64;
                            if if nat_eq(a3897, a3898) {
                                true
                            } else {
                                let a3899 = 127u64;
                                let a3900 = v5;
                                nat_lt(a3899, a3900)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3903 = f115(v1.clone(), {
                                    let a3901 = v2;
                                    let a3902 = 1u64;
                                    nat_add(a3901, a3902)?
                                })?;
                                let a3904 = 45u64;
                                if if nat_eq(a3903, a3904) {
                                    f117({
                                        let a3908 = {
                                            let a3905 = v2;
                                            let a3906 = 2u64;
                                            nat_add(a3905, a3906)?
                                        };
                                        let a3909 = {
                                            let a3907 = v1.clone();
                                            length_bytes(a3907)
                                        };
                                        nat_lt(a3908, a3909)
                                    }, f117(f712({
                                        let a3912 = f115(v1.clone(), {
                                            let a3910 = v2;
                                            let a3911 = 2u64;
                                            nat_add(a3910, a3911)?
                                        })?;
                                        let a3913 = 93u64;
                                        nat_eq(a3912, a3913)
                                    }), f712({
                                        let a3916 = f115(v1.clone(), {
                                            let a3914 = v2;
                                            let a3915 = 2u64;
                                            nat_add(a3914, a3915)?
                                        })?;
                                        let a3917 = 92u64;
                                        nat_eq(a3916, a3917)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3918 = v2;
                                        let a3919 = 3u64;
                                        nat_add(a3918, a3919)?
                                    }, List::cons((v5, f115(v1.clone(), {
                                        let a3920 = v2;
                                        let a3921 = 2u64;
                                        nat_add(a3920, a3921)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3922 = v2;
                                        let a3923 = 1u64;
                                        nat_add(a3922, a3923)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3924 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f736(v0: u64) -> u64 {
    let a3928 = v0;
    let a3929 = 42u64;
    if nat_eq(a3928, a3929) {
        1u64
    } else {
        let a3930 = v0;
        let a3931 = 43u64;
        if nat_eq(a3930, a3931) {
            2u64
        } else {
            let a3932 = v0;
            let a3933 = 63u64;
            if nat_eq(a3932, a3933) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f737(p3942: List<(u64, u64)>, v1: u64) -> bool {
    let m3941 = p3942;
    {
        let mut m3941 = m3941;
        loop {
            let step = {
                let v0 = m3941;
                let m3934 = v0;
                match m3934.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3936 = {
                            let (h3935, _) = v2;
                            h3935
                        };
                        let a3937 = v1;
                        if if nat_le(a3936, a3937) {
                            let a3939 = v1;
                            let a3940 = {
                                let (_, h3938) = v2;
                                h3938
                            };
                            nat_le(a3939, a3940)
                        } else {
                            false
                        } {
                            Err::<List<(u64, u64)>, bool>(true)
                        } else {
                            Ok::<List<(u64, u64)>, bool>(v3)
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

pub fn f738(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3947 = {
            let a3945 = {
                let a3943 = v0.clone();
                length_list(a3943)
            };
            let a3946 = {
                let a3944 = v1.clone();
                length_list(a3944)
            };
            nat_add(a3945, a3946)?
        };
        let a3948 = 1u64;
        nat_add(a3947, a3948)?
    };
    Ok(f744({
        let (_, h3949) = {
            let (_, h3950) = {
                let (h3951, _) = f745(Fn24::F746, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3951
            };
            h3950
        };
        h3949
    }))
}

pub fn f739(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3952, _) = v0.clone();
        h3952
    };
    let v2: List<(Bytes, ())> = {
        let (h3953, _) = {
            let (_, h3954) = v0.clone();
            h3954
        };
        h3953
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3955) = {
            let (_, h3956) = v0;
            h3956
        };
        h3955
    };
    let m3957 = v1.clone();
    match m3957.uncons() {
        None => {
            let m3958 = v2;
            match m3958.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3959 = v2.clone();
            match m3959.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f747({
                        let (h3960, _) = v8.clone();
                        h3960
                    }, {
                        let (h3961, _) = v6.clone();
                        h3961
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

pub fn f740(v0: Bytes) -> R<Option<Bytes>> {
    let m3963 = f748({
        let a3962 = v0.clone();
        length_bytes(a3962)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3963 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3965 = {
                let a3964 = v1.clone();
                length_bytes(a3964)
            };
            let a3966 = 0u64;
            if nat_eq(a3965, a3966) {
                let a3968 = {
                    let a3967 = v0;
                    length_bytes(a3967)
                };
                let a3969 = 0u64;
                if nat_eq(a3968, a3969) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f742(v1)?))
            }
        }
    }
}

pub fn f741(v0: Bytes) -> R<Option<Bytes>> {
    if f749({
        let a3970 = v0.clone();
        length_bytes(a3970)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f750(v0)?;
        if f751(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3973 = {
                let a3971 = v1.clone();
                let a3972 = 0u64;
                index_bytes(a3971, a3972)
            };
            match m3973 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f535(f72(v2)?) {
                        Ok(Some({
                            let a3974 = f8(Str::lit("n"));
                            let a3975 = v1;
                            append_bytes(a3974, a3975)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f752({
            let a3976 = v0.clone();
            length_bytes(a3976)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f742(v0: Bytes) -> R<Bytes> {
    if f52(v0.clone(), f753()) {
        f618(v0)
    } else {
        Ok(v0)
    }
}

pub fn f743(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3977 = v0.clone();
    match m3977 {
        Adt2::C0 => {
            Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C1(_, _) => {
            Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(v4) => {
            if v4 {
                Ok(f703())
            } else {
                Ok(f652(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C3(_, _) => {
            Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_, _, _) => {
            Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_, _) => {
            Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C6(_) => {
            Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C7(_) => {
            Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C8(_) => {
            let m3978 = f754(f578(v0.clone()));
            match m3978 {
                None => {
                    let m3979 = f92(f93(v0.clone(), f8(Str::lit("$ref"))));
                    match m3979 {
                        None => {
                            let m3981 = f93(v0.clone(), f8(Str::lit("allOf")));
                            match m3981 {
                                None => {
                                    let m3986 = f93(v0.clone(), f8(Str::lit("enum")));
                                    match m3986 {
                                        None => {
                                            let m3987 = f93(v0.clone(), f8(Str::lit("const")));
                                            match m3987 {
                                                None => {
                                                    let m3989 = f93(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3989 {
                                                        None => {
                                                            let v34: Bytes = f58(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v34.clone(), Bytes::lit(&[])) {
                                                                !f755(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f703())
                                                            } else {
                                                                if f32(v34.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f653(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                                                } else {
                                                                    if f32(v34.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f653(f756(v0), List::<Adt24>::nil()))
                                                                    } else {
                                                                        if f32(v34.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f653(f757(), List::<Adt24>::nil()))
                                                                        } else {
                                                                            if f32(v34.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f653(f8(Str::lit("Bool")), List::<Adt24>::nil()))
                                                                            } else {
                                                                                if f32(v34.clone(), f8(Str::lit("object"))) {
                                                                                    let m3990 = f537(f93(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3990.uncons() {
                                                                                        None => {
                                                                                            if f660(v0.clone()) {
                                                                                                Ok(f653(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                                                            } else {
                                                                                                let m3991 = f698(v0);
                                                                                                match m3991 {
                                                                                                    None => {
                                                                                                        Ok(f701(f8(Str::lit("List")), f758(f703())))
                                                                                                    }
                                                                                                    Some(v37) => {
                                                                                                        Ok(f701(f8(Str::lit("List")), f758(f759(v37, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f653(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f652(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v34, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f653(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v31) => {
                                                    let m3988 = f92(Some(v31));
                                                    match m3988 {
                                                        None => {
                                                            Ok(f652(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f653(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f653(v1.clone(), List::cons(Adt24::C0(v1, v0), List::<Adt24>::nil())))
                                        }
                                    }
                                }
                                Some(v23) => {
                                    let m3982 = f306(v23);
                                    match m3982.uncons() {
                                        None => {
                                            Ok(f652(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v24, v25)) => {
                                            let m3983 = v25;
                                            match m3983.uncons() {
                                                None => {
                                                    let m3984 = f92(f93(v24, f8(Str::lit("$ref"))));
                                                    match m3984 {
                                                        None => {
                                                            Ok(f652(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v28) => {
                                                            let m3985 = f649(v28)?;
                                                            match m3985 {
                                                                None => {
                                                                    Ok(f652(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v29) => {
                                                                    Ok(f653(v29, List::<Adt24>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f652(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v21) => {
                            let m3980 = f649(v21)?;
                            match m3980 {
                                None => {
                                    Ok(f652(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v22) => {
                                    Ok(f653(v22, List::<Adt24>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v20) => {
                    Ok(f652(if f32(v20.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v20, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C9 => {
            Ok(f703())
        }
        Adt2::C10(_, _) => {
            Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C11(_, _, _) => {
            Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f744(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f421(Fn5::F760, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f745(v0: Fn24, p3996: u64, p3997: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3995 = (p3996, p3997);
    {
        let mut m3995 = m3995;
        loop {
            let step = {
                let (v1, v2) = m3995;
                let m3992 = v1;
                if m3992 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3992 - 1;
                    let m3994 = {
                        let c3993: Fn24 = v0.clone();
                        c3993.apply(v2.clone())
                    };
                    match m3994 {
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
                Ok(next) => { m3995 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f746(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f761(v0)
}

pub fn f747(v0: Bytes, v1: Bytes) -> bool {
    let a4002 = {
        let a3998 = v0;
        let a3999 = v1;
        compare_bytes(a3998, a3999)
    };
    let a4003 = {
        let a4000 = Bytes::lit(&[]);
        let a4001 = Bytes::lit(&[0x01]);
        compare_bytes(a4000, a4001)
    };
    equal(a4002, a4003)
}

pub fn f748(p4016: u64, v1: Bytes, p4017: u64, p4018: Bytes) -> R<Option<Bytes>> {
    let m4015 = (p4016, (p4017, p4018));
    {
        let mut m4015 = m4015;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4015;
                let m4004 = v0;
                if m4004 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4004 - 1;
                    let a4006 = {
                        let a4005 = v1.clone();
                        length_bytes(a4005)
                    };
                    let a4007 = v2;
                    if nat_le(a4006, a4007) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4008 = f711(v1.clone(), v2)?;
                        match m4008 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4010 = v2;
                                    let a4011 = {
                                        let (_, h4009) = v5;
                                        h4009
                                    };
                                    nat_add(a4010, a4011)?
                                }, {
                                    let a4013 = v3;
                                    let a4014 = f762(v1.clone(), v2, {
                                        let (h4012, _) = v5;
                                        h4012
                                    });
                                    append_bytes(a4013, a4014)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4015 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f749(p4028: u64, v1: Bytes, p4029: u64) -> R<bool> {
    let m4027 = (p4028, p4029);
    {
        let mut m4027 = m4027;
        loop {
            let step = {
                let (v0, v2) = m4027;
                let m4019 = v0;
                if m4019 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4019 - 1;
                    let m4022 = {
                        let a4020 = v1.clone();
                        let a4021 = v2;
                        index_bytes(a4020, a4021)
                    };
                    match m4022 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4023 = f72(v4)?;
                            let a4024 = 128u64;
                            if nat_lt(a4023, a4024) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4025 = v2;
                                    let a4026 = 1u64;
                                    nat_add(a4025, a4026)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4027 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f750(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f480(v0.clone(), 0u64, {
        let a4030 = v0;
        length_bytes(a4030)
    }, false, Bytes::lit(&[]))?;
    let m4033 = {
        let a4031 = v1.clone();
        let a4032 = 0u64;
        index_bytes(a4031, a4032)
    };
    match m4033 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f534(v3) {
                let a4040 = f46({
                    let a4034 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4034)
                }, {
                    let a4035 = v3;
                    let a4036 = 65u64;
                    nat_sub(a4035, a4036)
                }, 1u64);
                let a4041 = f46(v1.clone(), 1u64, {
                    let a4038 = {
                        let a4037 = v1;
                        length_bytes(a4037)
                    };
                    let a4039 = 1u64;
                    nat_sub(a4038, a4039)
                });
                Ok(append_bytes(a4040, a4041))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f751(v0: Bytes) -> R<bool> {
    let m4044 = {
        let a4042 = v0;
        let a4043 = 0u64;
        index_bytes(a4042, a4043)
    };
    match m4044 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f203(f533(f72(v1)?), f534(f72(v1)?)))
        }
    }
}

pub fn f752(p4057: u64, v1: Bytes, p4058: u64, p4059: Bytes) -> R<Option<Bytes>> {
    let m4056 = (p4057, (p4058, p4059));
    {
        let mut m4056 = m4056;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4056;
                let m4045 = v0;
                if m4045 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4045 - 1;
                    let a4047 = {
                        let a4046 = v1.clone();
                        length_bytes(a4046)
                    };
                    let a4048 = v2;
                    if nat_le(a4047, a4048) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4049 = f711(v1.clone(), v2)?;
                        match m4049 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4051 = v2;
                                    let a4052 = {
                                        let (_, h4050) = v5;
                                        h4050
                                    };
                                    nat_add(a4051, a4052)?
                                }, {
                                    let a4054 = v3;
                                    let a4055 = f763({
                                        let (h4053, _) = v5;
                                        h4053
                                    });
                                    append_bytes(a4054, a4055)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f753() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f754(p4064: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m4063 = p4064;
    {
        let mut m4063 = m4063;
        loop {
            let step = {
                let v0 = m4063;
                let m4060 = v0;
                match m4060.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h4061, _) = v1.clone();
                            h4061
                        }, f764()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h4062, _) = v1;
                                h4062
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f755(v0: Adt2) -> bool {
    if f665(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f665(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f665(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f665(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f665(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f665(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f756(v0: Adt2) -> Bytes {
    let m4065 = f521(f93(v0, f8(Str::lit("minimum"))));
    match m4065 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f757() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f758(v0: Result<Adt27, Bytes>) -> Result<Adt27, Bytes> {
    let m4066 = v0;
    match m4066 {
        Ok(v2) => {
            f653(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f586(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f585(v2))
        }
        Err(v1) => {
            Err::<Adt27, Bytes>(v1)
        }
    }
}

pub fn f759(v0: Adt2, v1: Bytes) -> R<Result<Adt27, Bytes>> {
    f765(16u64, v0, v1)
}

pub fn f760(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f761(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4067, _) = v0.clone();
        h4067
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4068, _) = {
            let (_, h4069) = v0.clone();
            h4069
        };
        h4068
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4070) = {
            let (_, h4071) = v0;
            h4071
        };
        h4070
    };
    let m4072 = v1.clone();
    match m4072.uncons() {
        None => {
            let m4073 = v2;
            match m4073.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4074 = v2.clone();
            match m4074.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f747({
                        let (h4075, _) = v8.clone();
                        h4075
                    }, {
                        let (h4076, _) = v6.clone();
                        h4076
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

pub fn f762(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f203(f117(f203(f533(v2), f534(v2)), f117({
        let a4077 = v2;
        let a4078 = 117u64;
        !nat_eq(a4077, a4078)
    }, {
        let a4079 = v2;
        let a4080 = 85u64;
        !nat_eq(a4079, a4080)
    })), f117(f535(v2), {
        let a4081 = 0u64;
        let a4082 = v1;
        nat_lt(a4081, a4082)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a4083 = v2;
        let a4084 = 65536u64;
        if nat_lt(a4083, a4084) {
            let a4085 = f8(Str::lit("u"));
            let a4086 = f650(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4085, a4086)
        } else {
            let a4087 = f8(Str::lit("U"));
            let a4088 = f650(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4087, a4088)
        }
    }
}

pub fn f763(v0: u64) -> Bytes {
    let a4089 = f8(Str::lit("u"));
    let a4090 = f650(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4089, a4090)
}

pub fn f764() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f765(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt27, Bytes>> {
    let m4091 = v0;
    if m4091 == 0 {
        Ok(f652(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4091 - 1;
        let m4092 = v1.clone();
        match m4092 {
            Adt2::C0 => {
                Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C1(_, _) => {
                Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(f703())
                } else {
                    Ok(f652(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C3(_, _) => {
                Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_, _, _) => {
                Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_, _) => {
                Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C6(_) => {
                Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C7(_) => {
                Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C8(_) => {
                let m4093 = f92(f93(v1.clone(), f8(Str::lit("$ref"))));
                match m4093 {
                    None => {
                        if if f665(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f665(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f665(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f653(v2.clone(), List::cons(Adt24::C0(v2, v1), List::<Adt24>::nil())))
                        } else {
                            let v24: Bytes = f58(v1.clone(), f8(Str::lit("type")));
                            if if f32(v24.clone(), Bytes::lit(&[])) {
                                !f755(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f703())
                            } else {
                                if f32(v24.clone(), f8(Str::lit("string"))) {
                                    Ok(f653(f8(Str::lit("Bytes")), List::<Adt24>::nil()))
                                } else {
                                    if f32(v24.clone(), f8(Str::lit("integer"))) {
                                        Ok(f653(f756(v1), List::<Adt24>::nil()))
                                    } else {
                                        if f32(v24.clone(), f8(Str::lit("number"))) {
                                            Ok(f653(f757(), List::<Adt24>::nil()))
                                        } else {
                                            if f32(v24.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f653(f8(Str::lit("Bool")), List::<Adt24>::nil()))
                                            } else {
                                                if f32(v24.clone(), f8(Str::lit("array"))) {
                                                    let m4095 = f93(v1, f8(Str::lit("items")));
                                                    match m4095 {
                                                        None => {
                                                            Ok(f701(f8(Str::lit("List")), f703()))
                                                        }
                                                        Some(v25) => {
                                                            Ok(f701(f8(Str::lit("List")), f765(v3, v25, {
                                                                let a4096 = v2;
                                                                let a4097 = f8(Str::lit("Item"));
                                                                append_bytes(a4096, a4097)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v24.clone(), f8(Str::lit("object"))) {
                                                        if f660(v1.clone()) {
                                                            Ok(f653(v2.clone(), List::cons(Adt24::C0(v2, v1), List::<Adt24>::nil())))
                                                        } else {
                                                            let m4098 = f698(v1);
                                                            match m4098 {
                                                                None => {
                                                                    Ok(f701(f8(Str::lit("List")), f758(f703())))
                                                                }
                                                                Some(v26) => {
                                                                    Ok(f701(f8(Str::lit("List")), f758(f765(v3, v26, {
                                                                        let a4099 = v2;
                                                                        let a4100 = f8(Str::lit("Value"));
                                                                        append_bytes(a4099, a4100)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f652(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v24, List::<Bytes>::nil()))), v2))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(v22) => {
                        let m4094 = f649(v22)?;
                        match m4094 {
                            None => {
                                Ok(f652(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v23) => {
                                Ok(f653(v23, List::<Adt24>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C9 => {
                Ok(f703())
            }
            Adt2::C10(_, _) => {
                Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C11(_, _, _) => {
                Ok(f652(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}
