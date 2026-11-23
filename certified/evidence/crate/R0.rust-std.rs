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
    C0(List<Bytes>, List<(Bytes, Bytes)>, Adt1, u64, Bytes, Bytes, List<Bytes>, u64, List<(Bytes, Bytes)>, Bytes),
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
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(bool, bool, Bytes, List<Bytes>, List<List<Bytes>>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, Adt14),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64, Adt19),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64, Option<u64>),
    C1(u64),
    C2(Adt20, List<u64>, u64),
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
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
pub enum Fn0 {
    F108(u8, Bytes),
    F147(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F108(k0, k1) => f108(*k0, k1.clone(), p0),
            Fn0::F147(k0) => f147(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F126,
    F161,
    F172,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F126 => f126(p0, p1),
            Fn1::F161 => f161(p0, p1),
            Fn1::F172 => f172(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F143(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F143(k0) => f143(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F185(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt21)) -> R<(u64, Adt21)> {
        match self {
            Fn3::F185(k0, k1, k2) => f185(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F220,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn4::F220 => f220(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F223,
}

impl Fn5 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn5::F223 => f223(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F225,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn6::F225 => f225(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F228,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn7::F228 => f228(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F234(Bytes),
    F235(u64, Bytes),
    F286(Bytes),
    F329(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn8::F234(k0) => Ok(f234(k0.clone(), p0)),
            Fn8::F235(k0, k1) => f235(*k0, k1.clone(), p0),
            Fn8::F286(k0) => f286(k0.clone(), p0),
            Fn8::F329(k0) => f329(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F247(Bytes),
    F294(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F247(k0) => f247(k0.clone(), p0),
            Fn9::F294(k0, k1) => Ok(f294(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F249,
    F359,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F249 => f249(p0, p1),
            Fn10::F359 => f359(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F252,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F252 => f252(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F254,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F254 => f254(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F257,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F257 => f257(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F259,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn14::F259 => f259(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F264(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F264(k0) => f264(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F289(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F289(k0) => f289(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F298,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F298 => f298(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F300,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F300 => f300(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F302,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F302 => f302(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F310,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F310 => f310(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F321,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F321 => f321(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F343,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F343 => f343(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F345,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F345 => f345(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F347(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F347(k0) => f347(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F358,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F358 => f358(p0),
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
            f2(Adt0::C0(match v4.clone() {
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
                Adt0::C0(_, _, _, h5, _, _, _, _, _, _) => {
                    h5
                }
            };
            let a7 = 7u64;
            if nat_eq(a6, a7) {
                Ok((Adt0::C0(match v3.clone() {
                    Adt0::C0(h8, _, _, _, _, _, _, _, _, _) => {
                        h8
                    }
                }, match v3.clone() {
                    Adt0::C0(_, h9, _, _, _, _, _, _, _, _) => {
                        h9
                    }
                }, match v3.clone() {
                    Adt0::C0(_, _, h10, _, _, _, _, _, _, _) => {
                        h10
                    }
                }, 8u64, match v3.clone() {
                    Adt0::C0(_, _, _, _, h11, _, _, _, _, _) => {
                        h11
                    }
                }, match v3.clone() {
                    Adt0::C0(_, _, _, _, _, h12, _, _, _, _) => {
                        h12
                    }
                }, match v3.clone() {
                    Adt0::C0(_, _, _, _, _, _, h13, _, _, _) => {
                        h13
                    }
                }, match v3.clone() {
                    Adt0::C0(_, _, _, _, _, _, _, h14, _, _) => {
                        h14
                    }
                }, match v3 {
                    Adt0::C0(_, _, _, _, _, _, _, _, h15, _) => {
                        h15
                    }
                }, {
                    let m16 = v5;
                    match m16 {
                        Ok(v6) => {
                            v6
                        }
                        Err(_) => {
                            Bytes::lit(&[])
                        }
                    }
                }), Adt9::C0(f4(f5()))))
            } else {
                let a18 = match v3.clone() {
                    Adt0::C0(_, _, _, h17, _, _, _, _, _, _) => {
                        h17
                    }
                };
                let a19 = 8u64;
                if nat_eq(a18, a19) {
                    Ok(f6(v3.clone(), 1u8, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, _, _, _, _, h20) => {
                            h20
                        }
                    }, f7(List::cons({
                        let m21 = v5;
                        match m21 {
                            Ok(v8) => {
                                v8
                            }
                            Err(_) => {
                                Bytes::lit(&[])
                            }
                        }
                    }, List::cons(f8(Str::lit("command failed: ")), List::cons(match v3 {
                        Adt0::C0(_, _, _, _, h22, _, _, _, _, _) => {
                            h22
                        }
                    }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))
                } else {
                    let m23 = v5;
                    match m23 {
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
            let m25 = v15;
            match m25 {
                Ok(_) => {
                    let a27 = match v3.clone() {
                        Adt0::C0(_, _, _, h26, _, _, _, _, _, _) => {
                            h26
                        }
                    };
                    let a28 = 3u64;
                    if nat_eq(a27, a28) {
                        Ok(f12(v3))
                    } else {
                        let a30 = match v3.clone() {
                            Adt0::C0(_, _, _, h29, _, _, _, _, _, _) => {
                                h29
                            }
                        };
                        let a31 = 5u64;
                        if nat_eq(a30, a31) {
                            let m33 = match v3.clone() {
                                Adt0::C0(_, _, _, _, _, _, _, _, h32, _) => {
                                    h32
                                }
                            };
                            match m33.uncons() {
                                None => {
                                    f13(v3)
                                }
                                Some((v18, _)) => {
                                    Ok((f14(v3, 6u64, {
                                        let (h34, _) = v18.clone();
                                        h34
                                    }), Adt9::C1(f4({
                                        let (h35, _) = v18;
                                        h35
                                    }), {
                                        let (_, h36) = v18;
                                        h36
                                    })))
                                }
                            }
                        } else {
                            let m38 = match v3.clone() {
                                Adt0::C0(_, _, _, _, _, _, _, _, h37, _) => {
                                    h37
                                }
                            };
                            match m38.uncons() {
                                None => {
                                    f13(v3)
                                }
                                Some((_, v21)) => {
                                    f13(Adt0::C0(match v3.clone() {
                                        Adt0::C0(h39, _, _, _, _, _, _, _, _, _) => {
                                            h39
                                        }
                                    }, match v3.clone() {
                                        Adt0::C0(_, h40, _, _, _, _, _, _, _, _) => {
                                            h40
                                        }
                                    }, match v3.clone() {
                                        Adt0::C0(_, _, h41, _, _, _, _, _, _, _) => {
                                            h41
                                        }
                                    }, 5u64, Bytes::lit(&[]), match v3.clone() {
                                        Adt0::C0(_, _, _, _, _, h42, _, _, _, _) => {
                                            h42
                                        }
                                    }, match v3.clone() {
                                        Adt0::C0(_, _, _, _, _, _, h43, _, _, _) => {
                                            h43
                                        }
                                    }, match v3.clone() {
                                        Adt0::C0(_, _, _, _, _, _, _, h44, _, _) => {
                                            h44
                                        }
                                    }, v21, match v3 {
                                        Adt0::C0(_, _, _, _, _, _, _, _, _, h45) => {
                                            h45
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
            let m24 = v12;
            match m24 {
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
            let m46 = v22;
            match m46 {
                Ok(v24) => {
                    Ok(f15(Adt0::C0(match v3.clone() {
                        Adt0::C0(h47, _, _, _, _, _, _, _, _, _) => {
                            h47
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, h48, _, _, _, _, _, _, _, _) => {
                            h48
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, _, h49, _, _, _, _, _, _, _) => {
                            h49
                        }
                    }, 4u64, match v3.clone() {
                        Adt0::C0(_, _, _, _, h50, _, _, _, _, _) => {
                            h50
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, h51, _, _, _, _) => {
                            h51
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, _, h52, _, _, _) => {
                            h52
                        }
                    }, {
                        let (h53, _) = v24;
                        h53
                    }, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, _, _, _, h54, _) => {
                            h54
                        }
                    }, match v3 {
                        Adt0::C0(_, _, _, _, _, _, _, _, _, h55) => {
                            h55
                        }
                    })))
                }
                Err(_) => {
                    Ok(f11(v3, f8(Str::lit("could not start"))))
                }
            }
        }
        Adt2::C7(v25) => {
            let m56 = v25;
            match m56 {
                Ok(v27) => {
                    let m59 = f16(match v27 {
                        Adt8::C0(h57, _) => {
                            h57
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, _, _, h58, _, _) => {
                            h58
                        }
                    });
                    match m59 {
                        None => {
                            Ok(f15(v3))
                        }
                        Some(v28) => {
                            if f17(v28) {
                                f18(v3)
                            } else {
                                Ok((f14(v3.clone(), 7u64, match v3 {
                                    Adt0::C0(_, _, _, _, h60, _, _, _, _, _) => {
                                        h60
                                    }
                                }), Adt9::C0(f4(f19()))))
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
    Adt0::C0(List::<Bytes>::nil(), List::<(Bytes, Bytes)>::nil(), f3(), 0u64, Bytes::lit(&[]), Bytes::lit(&[]), List::<Bytes>::nil(), 0u64, List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[]))
}

pub fn f2(v0: Adt0) -> R<(Adt0, Adt9)> {
    let m63 = f20(match v0.clone() {
        Adt0::C0(h61, _, _, _, _, _, _, _, _, _) => {
            h61
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h62, _, _, _, _, _, _, _) => {
            h62
        }
    })?;
    match m63 {
        Adt11::C0(v1) => {
            let m64 = v1;
            match m64.uncons() {
                None => {
                    Ok(f6(v0, 1u8, Bytes::lit(&[]), f8(Str::lit("internal: an empty read plan\u{a}"))))
                }
                Some((v2, _)) => {
                    Ok((f14(v0, 1u64, v2.clone()), Adt9::C0(f4(v2))))
                }
            }
        }
        Adt11::C1(v4) => {
            let m65 = v4;
            match m65.uncons() {
                None => {
                    Ok(f6(v0, 1u8, Bytes::lit(&[]), f8(Str::lit("internal: an empty list plan\u{a}"))))
                }
                Some((v5, _)) => {
                    Ok((f14(v0, 2u64, v5.clone()), Adt9::C2(f4(v5))))
                }
            }
        }
        Adt11::C2(v7, v8, v9) => {
            Ok((Adt0::C0(match v0.clone() {
                Adt0::C0(h66, _, _, _, _, _, _, _, _, _) => {
                    h66
                }
            }, match v0.clone() {
                Adt0::C0(_, h67, _, _, _, _, _, _, _, _) => {
                    h67
                }
            }, match v0.clone() {
                Adt0::C0(_, _, h68, _, _, _, _, _, _, _) => {
                    h68
                }
            }, 3u64, v9, v7, v8, 0u64, match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h69, _) => {
                    h69
                }
            }, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, _, _, h70) => {
                    h70
                }
            }), Adt9::C4(Str::lit("target"))))
        }
        Adt11::C3(v10, v11) => {
            f13(Adt0::C0(match v0.clone() {
                Adt0::C0(h71, _, _, _, _, _, _, _, _, _) => {
                    h71
                }
            }, match v0.clone() {
                Adt0::C0(_, h72, _, _, _, _, _, _, _, _) => {
                    h72
                }
            }, match v0.clone() {
                Adt0::C0(_, _, h73, _, _, _, _, _, _, _) => {
                    h73
                }
            }, 5u64, Bytes::lit(&[]), match v0.clone() {
                Adt0::C0(_, _, _, _, _, h74, _, _, _, _) => {
                    h74
                }
            }, match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h75, _, _, _) => {
                    h75
                }
            }, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, h76, _, _) => {
                    h76
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

pub fn f4(v0: Bytes) -> Str {
    let m78 = {
        let a77 = v0;
        utf8_decode(a77)
    };
    match m78 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f5() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.err"))
}

pub fn f6(v0: Adt0, v1: u8, v2: Bytes, v3: Bytes) -> (Adt0, Adt9) {
    (f14(v0, 9u64, Bytes::lit(&[])), Adt9::C13(v1, v2, v3))
}

pub fn f7(v0: List<Bytes>) -> Bytes {
    let m79 = v0;
    match m79.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, v2)) => {
            let a80 = v1;
            let a81 = f7(v2);
            append_bytes(a80, a81)
        }
    }
}

pub fn f8(v0: Str) -> Bytes {
    let a82 = v0;
    utf8_encode(a82)
}

pub fn f9(v0: Adt0, v1: Option<Bytes>) -> R<(Adt0, Adt9)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, h83, _, _, _, _, _, _, _) => {
            h83
        }
    };
    f2(f21(v0.clone(), Adt1::C0(List::cons((match v0 {
        Adt0::C0(_, _, _, _, h84, _, _, _, _, _) => {
            h84
        }
    }, v1), match v2.clone() {
        Adt1::C0(h85, _, _) => {
            h85
        }
    }), match v2.clone() {
        Adt1::C0(_, h86, _) => {
            h86
        }
    }, match v2 {
        Adt1::C0(_, _, h87) => {
            h87
        }
    })))
}

pub fn f10(v0: Adt0, v1: List<Bytes>) -> R<(Adt0, Adt9)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, h88, _, _, _, _, _, _, _) => {
            h88
        }
    };
    f2(f21(v0.clone(), Adt1::C0(match v2.clone() {
        Adt1::C0(h89, _, _) => {
            h89
        }
    }, List::cons((match v0 {
        Adt0::C0(_, _, _, _, h90, _, _, _, _, _) => {
            h90
        }
    }, v1), match v2.clone() {
        Adt1::C0(_, h91, _) => {
            h91
        }
    }), match v2 {
        Adt1::C0(_, _, h92) => {
            h92
        }
    })))
}

pub fn f11(v0: Adt0, v1: Bytes) -> (Adt0, Adt9) {
    f6(v0.clone(), 1u8, Bytes::lit(&[]), f7(List::cons(v1, List::cons(f8(Str::lit(": ")), List::cons(match v0 {
        Adt0::C0(_, _, _, _, h93, _, _, _, _, _) => {
            h93
        }
    }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f12(v0: Adt0) -> (Adt0, Adt9) {
    (f14(v0.clone(), 4u64, match v0.clone() {
        Adt0::C0(_, _, _, _, h94, _, _, _, _, _) => {
            h94
        }
    }), Adt9::C10(Adt10::C0(f4(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h95, _, _, _, _) => {
            h95
        }
    }), f22(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h96, _, _, _) => {
            h96
        }
    }), Str::lit("."), f23(match v0 {
        Adt0::C0(_, h97, _, _, _, _, _, _, _, _) => {
            h97
        }
    }), f4(f19()), f4(f5()))))
}

pub fn f13(v0: Adt0) -> R<(Adt0, Adt9)> {
    let m99 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h98, _) => {
            h98
        }
    };
    match m99.uncons() {
        None => {
            Ok(f6(v0.clone(), 0u8, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, _, _, h100) => {
                    h100
                }
            }, Bytes::lit(&[])))
        }
        Some((v1, _)) => {
            Ok((f14(v0, 5u64, {
                let (h101, _) = v1.clone();
                h101
            }), Adt9::C4(f4(f24({
                let (h102, _) = v1;
                h102
            })?))))
        }
    }
}

pub fn f14(v0: Adt0, v1: u64, v2: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h103, _, _, _, _, _, _, _, _, _) => {
            h103
        }
    }, match v0.clone() {
        Adt0::C0(_, h104, _, _, _, _, _, _, _, _) => {
            h104
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h105, _, _, _, _, _, _, _) => {
            h105
        }
    }, v1, v2, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h106, _, _, _, _) => {
            h106
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h107, _, _, _) => {
            h107
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h108, _, _) => {
            h108
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h109, _) => {
            h109
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h110) => {
            h110
        }
    })
}

pub fn f15(v0: Adt0) -> (Adt0, Adt9) {
    (v0.clone(), Adt9::C11(List::cons(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h111, _, _) => {
            h111
        }
    }, List::<u64>::nil()), 600000u64))
}

pub fn f16(p118: List<(u64, Option<i32>)>, v1: u64) -> Option<Option<i32>> {
    let m117 = p118;
    {
        let mut m117 = m117;
        loop {
            let step = {
                let v0 = m117;
                let m112 = v0;
                match m112.uncons() {
                    None => {
                        Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(None::<Option<i32>>)
                    }
                    Some((v2, v3)) => {
                        let a114 = {
                            let (h113, _) = v2;
                            h113
                        };
                        let a115 = v1;
                        if nat_eq(a114, a115) {
                            Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(Some({
                                let (_, h116) = v2;
                                h116
                            }))
                        } else {
                            Ok::<List<(u64, Option<i32>)>, Option<Option<i32>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Option<i32>) -> bool {
    let m119 = v0;
    match m119 {
        None => {
            false
        }
        Some(v1) => {
            let a120 = v1;
            let a121 = 0i32;
            equal(a120, a121)
        }
    }
}

pub fn f18(v0: Adt0) -> R<(Adt0, Adt9)> {
    let v1: Adt1 = match v0.clone() {
        Adt0::C0(_, _, h122, _, _, _, _, _, _, _) => {
            h122
        }
    };
    f2(f21(v0.clone(), Adt1::C0(match v1.clone() {
        Adt1::C0(h123, _, _) => {
            h123
        }
    }, match v1.clone() {
        Adt1::C0(_, h124, _) => {
            h124
        }
    }, List::cons(match v0 {
        Adt0::C0(_, _, _, _, h125, _, _, _, _, _) => {
            h125
        }
    }, match v1 {
        Adt1::C0(_, _, h126) => {
            h126
        }
    }))))
}

pub fn f19() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f20(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m127 = v0;
    match m127.uncons() {
        None => {
            Ok(Adt11::C4(f25()))
        }
        Some((v2, v3)) => {
            if f26(v2.clone(), f8(Str::lit("check"))) {
                f27(v1)
            } else {
                if f26(v2.clone(), f8(Str::lit("next"))) {
                    f28(v1)
                } else {
                    if f26(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f29(v1))
                    } else {
                        if f26(v2.clone(), f8(Str::lit("publish"))) {
                            f30(v1)
                        } else {
                            if f26(v2.clone(), f8(Str::lit("agenda"))) {
                                f31(v1)
                            } else {
                                let m128 = v3;
                                match m128.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f25()))
                                    }
                                    Some((v4, v5)) => {
                                        if f26(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f32(v1, v4)
                                        } else {
                                            if f26(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                f33(v1, v4)
                                            } else {
                                                if f26(v2.clone(), f8(Str::lit("admit"))) {
                                                    f34(v1, v4)
                                                } else {
                                                    if f26(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                        f35(v1, v4)
                                                    } else {
                                                        if f26(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                            f36(v1, v4)
                                                        } else {
                                                            if f26(v2, f8(Str::lit("propose-encoding"))) {
                                                                let m129 = v5;
                                                                match m129.uncons() {
                                                                    None => {
                                                                        Ok(Adt11::C4(f25()))
                                                                    }
                                                                    Some((v6, v7)) => {
                                                                        f37(v1, v4, v6, v7)
                                                                    }
                                                                }
                                                            } else {
                                                                Ok(Adt11::C4(f25()))
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

pub fn f21(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h130, _, _, _, _, _, _, _, _, _) => {
            h130
        }
    }, match v0.clone() {
        Adt0::C0(_, h131, _, _, _, _, _, _, _, _) => {
            h131
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, h132, _, _, _, _, _) => {
            h132
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h133, _, _, _, _) => {
            h133
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h134, _, _, _) => {
            h134
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h135, _, _) => {
            h135
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h136, _) => {
            h136
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h137) => {
            h137
        }
    })
}

pub fn f22(v0: List<Bytes>) -> List<Str> {
    let m138 = v0;
    match m138.uncons() {
        None => {
            List::<Str>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f4(v1), f22(v2))
        }
    }
}

pub fn f23(v0: List<(Bytes, Bytes)>) -> List<(Str, Str)> {
    let m139 = v0;
    match m139.uncons() {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f4({
                let (h140, _) = v1;
                h140
            }), f4({
                let (_, h141) = v1;
                h141
            })), f23(v2))
        }
    }
}

pub fn f24(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f38(v0.clone(), 47u8)?;
    let v2: Bytes = f39(v1);
    let a144 = {
        let a142 = v0.clone();
        length_bytes(a142)
    };
    let a145 = {
        let a143 = v2.clone();
        length_bytes(a143)
    };
    if nat_eq(a144, a145) {
        Ok(f8(Str::lit(".")))
    } else {
        Ok(f40(v0.clone(), 0u64, {
            let a150 = {
                let a146 = v0;
                length_bytes(a146)
            };
            let a151 = {
                let a148 = {
                    let a147 = v2;
                    length_bytes(a147)
                };
                let a149 = 1u64;
                nat_add(a148, a149)?
            };
            nat_sub(a150, a151)
        }))
    }
}

pub fn f25() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next | agenda | stage-bindings JSONL | admit JSONL | merge-gate RULES | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f26(v0: Bytes, v1: Bytes) -> bool {
    let a152 = v0;
    let a153 = v1;
    equal(a152, a153)
}

pub fn f27(v0: Adt1) -> R<Adt11> {
    let m154 = f41(v0.clone(), f42());
    match m154.uncons() {
        None => {
            let m155 = f43(v0.clone(), f42())?;
            match m155.uncons() {
                None => {
                    let m158 = f44(match v0.clone() {
                        Adt1::C0(_, h156, _) => {
                            h156
                        }
                    }, f45(f46(), f47(match v0.clone() {
                        Adt1::C0(_, h157, _) => {
                            h157
                        }
                    }, false)));
                    match m158.uncons() {
                        None => {
                            let v7: (bool, Bytes) = f48(Adt13::C0(Adt12::C0(f49(f50(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f51(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f49(f50(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f52(f53(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f53(v0.clone(), f8(Str::lit("model/admissions.json")))?, f53(v0.clone(), f8(Str::lit("model/strata.json")))?, f54(f53(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f47(match v0 {
                                Adt1::C0(_, h159, _) => {
                                    h159
                                }
                            }, true)))?;
                            let (h160, _) = v7;
                            if h160 {
                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                    let (_, h161) = v7;
                                    h161
                                }))
                            } else {
                                Ok(Adt11::C4({
                                    let (_, h162) = v7;
                                    h162
                                }))
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

pub fn f28(v0: Adt1) -> R<Adt11> {
    if f55(f56(match v0.clone() {
        Adt1::C0(_, h163, _) => {
            h163
        }
    }, f57())) {
        Ok(Adt11::C1(List::cons(f57(), List::<Bytes>::nil())))
    } else {
        let v1: List<Bytes> = f58(f59(f57(), f60(match v0.clone() {
            Adt1::C0(_, h164, _) => {
                h164
            }
        }, f57())))?;
        let m165 = f41(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v1.clone()));
        match m165.uncons() {
            None => {
                let m166 = f61(v0, v1)?;
                match m166 {
                    None => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])))
                    }
                    Some(v4) => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("ingest-authority ")), List::cons(v4, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
            Some((v2, v3)) => {
                Ok(Adt11::C0(List::cons(v2, v3)))
            }
        }
    }
}

pub fn f29(v0: Adt1) -> Adt11 {
    if f55(f62(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h167) => {
            h167
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f63(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f55(f62(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h168) => {
                h168
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f63(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f30(v0: Adt1) -> R<Adt11> {
    let m169 = f41(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m169.uncons() {
        None => {
            Ok(Adt11::C3(List::cons((f64(), f65()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f66()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f67()), List::cons((f8(Str::lit("docs/model.md")), f68(v0)?), List::<(Bytes, Bytes)>::nil())))), f8(Str::lit("published .github/workflows/pages.yml, .github/workflows/operate.yml, .github/CODEOWNERS and docs/model.md\u{a}"))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f31(v0: Adt1) -> R<Adt11> {
    let m170 = f41(v0.clone(), List::cons(f69(), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m170.uncons() {
        None => {
            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f70(f71(f72(f73(f50(v0.clone(), f69()))?)), f74(f53(v0, f8(Str::lit("model/admissions.json")))?)))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f32(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m171 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m171.uncons() {
        None => {
            let v4: Bytes = f50(v0.clone(), v1);
            let v5: List<List<Bytes>> = f73(v4.clone())?;
            if f55(f75(f76(v5.clone()), f77())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f78(Str::lit("id"), v6.clone())?, List::cons(f78(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f78(Str::lit("artifact"), f69())?, List::cons(f78(Str::lit("checksum"), f79(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f69(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f80(f50(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f81(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f82(f71(f72(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f33(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m172 = f41(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m172.uncons() {
        None => {
            let m173 = f83(f84(f50(v0, v1))?)?;
            match m173 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f82(f85(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f34(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m174 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil())))));
    match m174.uncons() {
        None => {
            let v4: List<Bytes> = f84(f50(v0.clone(), v1))?;
            let m175 = f86(v4.clone(), f8(Str::lit("requirement")))?;
            match m175 {
                None => {
                    Ok(Adt11::C4(f87()))
                }
                Some(v5) => {
                    let m176 = f86(v4, f8(Str::lit("admission")))?;
                    match m176 {
                        None => {
                            Ok(Adt11::C4(f87()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f53(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f53(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f45(f52(v5.clone(), f8(Str::lit("id"))), f52(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f58(f45(f52(v7.clone(), f8(Str::lit("id"))), f52(v8.clone(), f8(Str::lit("construct")))))?;
                            let a178 = {
                                let a177 = v9.clone();
                                length_list(a177)
                            };
                            let a179 = 0u64;
                            if nat_eq(a178, a179) {
                                Ok(Adt11::C4(f87()))
                            } else {
                                if f55({
                                    let a182 = {
                                        let a180 = f58(f45(v9.clone(), v10.clone()))?;
                                        length_list(a180)
                                    };
                                    let a183 = {
                                        let a181 = f45(v9.clone(), v10);
                                        length_list(a181)
                                    };
                                    nat_eq(a182, a183)
                                }) {
                                    Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                } else {
                                    Ok(Adt11::C3(List::cons(f88(f8(Str::lit("model/requirements.json")), f89(v7, v5))?, List::cons(f88(f8(Str::lit("model/admissions.json")), f89(v8, v6))?, List::cons(f81(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f82(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f35(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m184 = f41(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m184.uncons() {
        None => {
            let m185 = f90(v0, v1)?;
            match m185 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f91(f92(Some(v4))) {
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

pub fn f36(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m186 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m186.uncons() {
        None => {
            let m187 = f90(v0.clone(), v1.clone())?;
            match m187 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f55(f26(f93(v4.clone(), f8(Str::lit("schema"))), f94())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f93(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m188 = f95(v4.clone(), f8(Str::lit("source")));
                            match m188 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f96(f95(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f97(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f97(v9.clone(), f8(Str::lit("binding.json"))), f50(v0.clone(), v1));
                        let m189 = v8;
                        match m189 {
                            None => {
                                f98(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f55(f26(f93(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f98(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f97(v9, f99(f93(v7.clone(), f8(Str::lit("url"))))?);
                                    if f55(f62(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h190) => {
                                            h190
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f93(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m191 = f41(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m191.uncons() {
                                            None => {
                                                if f26(f79(f50(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f98(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f37(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m192 = f41(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil()))))));
    match m192.uncons() {
        None => {
            let m193 = f100(v3);
            match m193 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v6) => {
                    let m194 = f90(v0.clone(), v1.clone())?;
                    match m194 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                        }
                        Some(v7) => {
                            let v8: Bytes = {
                                let a195 = f8(Str::lit("Catalog."));
                                let a196 = f101(v2.clone())?;
                                append_bytes(a195, a196)
                            };
                            let v9: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f101(v2.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                            let m197 = f102(v7, v8.clone(), f101(v2.clone())?)?;
                            match m197 {
                                Ok(v11) => {
                                    Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f101(v2)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), f50(v0.clone(), v1)), List::cons((v9.clone(), v11), List::cons((f8(Str::lit("model/admissions.json")), f103(Adt14::C3(std::rc::Rc::new(f104(f105(f53(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v8.clone())), std::rc::Rc::new(v6)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f103(Adt14::C3(std::rc::Rc::new(f104(f105(f53(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                        let a201 = f106(f50(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f40(v9.clone(), 4u64, {
                                            let a199 = {
                                                let a198 = v9;
                                                length_bytes(a198)
                                            };
                                            let a200 = 4u64;
                                            nat_sub(a199, a200)
                                        }))?;
                                        let a202 = Bytes::lit(&[0x0a]);
                                        append_bytes(a201, a202)
                                    }), List::cons(f81(v0, List::cons(v8.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v8, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                }
                                Err(v10) => {
                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v10, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                }
                            }
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

pub fn f38(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a203 = v0.clone();
        length_bytes(a203)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h204, _) = f107(Fn0::F108(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h204
    };
    let v4: u64 = {
        let (h205, _) = {
            let (_, h206) = v3.clone();
            h206
        };
        h205
    };
    Ok(f109(List::cons(f40(v0, v4, {
        let a207 = v2;
        let a208 = v4;
        nat_sub(a207, a208)
    }), {
        let (_, h209) = {
            let (_, h210) = v3;
            h210
        };
        h209
    })))
}

pub fn f39(p214: List<Bytes>) -> Bytes {
    let m213 = p214;
    {
        let mut m213 = m213;
        loop {
            let step = {
                let v0 = m213;
                let m211 = v0;
                match m211.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m212 = v2.clone();
                        match m212.uncons() {
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
                Ok(next) => { m213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m218 = {
        let a215 = v0;
        let a216 = v1;
        let a217 = v2;
        slice_bytes(a215, a216, a217)
    };
    match m218 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f41(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m219 = v1;
    match m219.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m221 = f110(match v0.clone() {
                Adt1::C0(h220, _, _) => {
                    h220
                }
            }, v2.clone());
            match m221 {
                None => {
                    List::cons(v2, f41(v0, v3))
                }
                Some(_) => {
                    f41(v0, v3)
                }
            }
        }
    }
}

pub fn f42() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f69(), List::<Bytes>::nil())))))))
}

pub fn f43(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m222 = v1;
    match m222.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f111(v0.clone(), v2.clone()) {
                f112(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m223 = f90(v0.clone(), v2.clone())?;
                match m223 {
                    None => {
                        Ok(List::cons(v2, f43(v0, v3)?))
                    }
                    Some(_) => {
                        f43(v0, v3)
                    }
                }
            } else {
                f43(v0, v3)
            }
        }
    }
}

pub fn f44(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m224 = v1;
    match m224.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f56(v0.clone(), v2.clone()) {
                f44(v0, v3)
            } else {
                List::cons(v2, f44(v0, v3))
            }
        }
    }
}

pub fn f45(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m225 = v0;
    match m225.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f45(v3, v1))
        }
    }
}

pub fn f46() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f47(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m226 = v0;
    match m226.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f45(f113({
                let (h227, _) = v2;
                h227
            }, {
                let (_, h228) = v2;
                h228
            }, v1), f47(v3, v1))
        }
    }
}

pub fn f48(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h229, _, _, _, _) => {
            h229
        }
    };
    let v2: Adt15 = f114(v1.clone())?;
    let v3: List<Bytes> = f115(v1.clone());
    let v4: List<Bytes> = f52(match v0.clone() {
        Adt13::C0(_, _, h230, _, _) => {
            h230
        }
    }, f8(Str::lit("path")));
    let m232 = f116(v2, List::<u64>::nil(), f117(v1.clone(), match v0.clone() {
        Adt13::C0(_, h231, _, _, _) => {
            h231
        }
    }, 0u64)?);
    match m232 {
        None => {
            if f119(f120(match v0.clone() {
                Adt13::C0(_, _, _, _, h234) => {
                    h234
                }
            }, f45(match v0.clone() {
                Adt13::C0(_, _, _, _, h235) => {
                    h235
                }
            }, v4.clone()))?, f120(v4.clone(), f45(match v0.clone() {
                Adt13::C0(_, _, _, _, h236) => {
                    h236
                }
            }, v4))?, f121(match v0.clone() {
                Adt13::C0(_, _, h237, _, _) => {
                    h237
                }
            })) {
                let m241 = f122(f120(match v0.clone() {
                    Adt13::C0(_, _, _, h238, _) => {
                        h238
                    }
                }, f45(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h239, _) => {
                        h239
                    }
                }))?, f120(v3.clone(), f45(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h240, _) => {
                        h240
                    }
                }))?);
                match m241 {
                    None => {
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f82(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f118(f45(v3, match v0 {
                            Adt13::C0(_, _, _, h242, _) => {
                                h242
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f118(match v1 {
                Adt12::C0(_, _, _, _, h233) => {
                    h233
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f49(v0: Bytes) -> R<List<Bytes>> {
    f123(f84(v0)?, f8(Str::lit("id = \"")))
}

pub fn f50(v0: Adt1, v1: Bytes) -> Bytes {
    let m244 = f110(match v0 {
        Adt1::C0(h243, _, _) => {
            h243
        }
    }, v1);
    match m244 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m245 = v2;
            match m245 {
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

pub fn f51(v0: Adt1) -> R<List<Bytes>> {
    let m246 = f73(f50(v0, f69()))?;
    match m246.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f71(v2))
        }
    }
}

pub fn f52(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m247 = v0;
    match m247.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f93(v2, v1.clone()), f52(v3, v1))
        }
    }
}

pub fn f53(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f92(f90(v0, v1)?))
}

pub fn f54(v0: List<Adt14>) -> List<Bytes> {
    let m248 = v0;
    match m248.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m249 = f96(Some(v1));
            match m249 {
                None => {
                    f54(v2)
                }
                Some(v3) => {
                    List::cons(v3, f54(v2))
                }
            }
        }
    }
}

pub fn f55(v0: bool) -> bool {
    !v0
}

pub fn f56(p253: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m252 = p253;
    {
        let mut m252 = m252;
        loop {
            let step = {
                let v0 = m252;
                let m250 = v0;
                match m250.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h251, _) = v2;
                            h251
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
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

pub fn f57() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f58(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f124(f125(Fn1::F126, List::<Bytes>::nil(), f127(v0)?)))
}

pub fn f59(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m254 = v1;
    match m254.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f112(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f97(v0.clone(), v2), f59(v0, v3))
            } else {
                f59(v0, v3)
            }
        }
    }
}

pub fn f60(p259: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m258 = p259;
    {
        let mut m258 = m258;
        loop {
            let step = {
                let v0 = m258;
                let m255 = v0;
                match m255.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h256, _) = v2;
                            h256
                        }, v1.clone()) {
                            let (_, h257) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h257)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt1, p262: List<Bytes>) -> R<Option<Bytes>> {
    let m261 = p262;
    {
        let mut m261 = m261;
        loop {
            let step = {
                let v1 = m261;
                let m260 = v1;
                match m260.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        if f128(v0.clone(), v2.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v2)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
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

pub fn f62(v0: Bytes, p265: List<Bytes>) -> bool {
    let m264 = p265;
    {
        let mut m264 = m264;
        loop {
            let step = {
                let v1 = m264;
                let m263 = v1;
                match m263.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f26(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f64() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f65() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f66() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: operate\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          { echo \"proposed a change to the API's sources, exported again as $(basename studio/.lexlean/production/*/)\"; git diff --stat -- studio/src README.md; } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          if [ \"$ARGS\" = advance ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '[.[] | select(.title == \"verdant_studio advance\")] | length')\" != 0 ]; then\u{a}            echo \"::notice::an advance pull request is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}            exit 0\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            while op=$(target/release/verdant_studio next) && [ -n \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              target/release/verdant_studio $op | tee -a target/operation.txt\u{a}              last=$op\u{a}            done\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          git push origin \"HEAD:refs/heads/${branch}\"\u{a}          gh pr create --head \"$branch\" --base main --title \"verdant_studio $ARGS\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f67() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f68(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f129(f53(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f130(f53(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f69() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f70(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m266 = v0;
    match m266.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f62(v2.clone(), v1.clone()) {
                f70(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f70(v3, v1))
            }
        }
    }
}

pub fn f71(v0: List<List<Bytes>>) -> List<Bytes> {
    let m267 = v0;
    match m267.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m268 = v1;
            match m268.uncons() {
                None => {
                    f71(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f71(v2))
                }
            }
        }
    }
}

pub fn f72(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m269 = v0;
    match m269.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f73(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt16 = f131(v0.clone(), 0u64, {
        let a270 = v0;
        length_bytes(a270)
    }, Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt16 = {
        let a273 = {
            let a272 = match v1.clone() {
                Adt16::C0(_, _, h271, _, _) => {
                    h271
                }
            };
            length_bytes(a272)
        };
        let a274 = 0u64;
        if if nat_eq(a273, a274) {
            let m276 = match v1.clone() {
                Adt16::C0(_, _, _, h275, _) => {
                    h275
                }
            };
            match m276.uncons() {
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
            f132(v1)
        }
    };
    Ok(f133(match v4 {
        Adt16::C0(_, _, _, _, h277) => {
            h277
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f74(v0: List<Adt14>) -> List<Bytes> {
    let m278 = v0;
    match m278.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m279 = f95(v1, f8(Str::lit("std")));
            match m279 {
                None => {
                    f74(v2)
                }
                Some(v3) => {
                    List::cons(f93(v3, f8(Str::lit("row"))), f74(v2))
                }
            }
        }
    }
}

pub fn f75(p284: List<Bytes>, p285: List<Bytes>) -> bool {
    let m283 = (p284, p285);
    {
        let mut m283 = m283;
        loop {
            let step = {
                let (v0, v1) = m283;
                let m280 = v0;
                match m280.uncons() {
                    None => {
                        let m281 = v1;
                        match m281.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m282 = v1;
                        match m282.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f26(v4, v6) {
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
                Ok(next) => { m283 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: List<List<Bytes>>) -> List<Bytes> {
    let m286 = v0;
    match m286.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f77() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f78(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f134(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f79(v0: Bytes) -> R<Bytes> {
    Ok(f135(f136(v0)?))
}

pub fn f80(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f137(f138(f84(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f139(f140(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f81(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f103(Adt14::C3(std::rc::Rc::new(f141(f58(f45(v1, f54(f53(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f82(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m287 = f142(Fn2::F143(v1), None::<Bytes>, v0);
    match m287 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f83(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m288 = v0;
    match m288.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f144(v1)?;
            let a290 = {
                let a289 = v3.clone();
                length_bytes(a289)
            };
            let a291 = 0u64;
            if nat_eq(a290, a291) {
                f83(v2)
            } else {
                let m292 = f145(v3.clone())?;
                match m292 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f93(v4.clone(), f8(Str::lit("id")));
                        if if f26(f93(v4, f8(Str::lit("schema"))), f94()) {
                            f55({
                                let a294 = {
                                    let a293 = v5.clone();
                                    length_bytes(a293)
                                };
                                let a295 = 0u64;
                                nat_eq(a294, a295)
                            })
                        } else {
                            false
                        } {
                            let m296 = f83(v2)?;
                            match m296 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f146(v5), {
                                        let a297 = v3;
                                        let a298 = Bytes::lit(&[0x0a]);
                                        append_bytes(a297, a298)
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

pub fn f84(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a299 = v0.clone();
        length_bytes(a299)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h300, _) = f107(Fn0::F147(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h300
    };
    let v3: u64 = {
        let (h301, _) = {
            let (_, h302) = v2.clone();
            h302
        };
        h301
    };
    let v4: List<Bytes> = {
        let (_, h303) = {
            let (_, h304) = v2;
            h304
        };
        h303
    };
    let a305 = v3;
    let a306 = v1;
    if nat_lt(a305, a306) {
        Ok(f109(List::cons(f40(v0, v3, {
            let a307 = v1;
            let a308 = v3;
            nat_sub(a307, a308)
        }), v4)))
    } else {
        Ok(f109(v4))
    }
}

pub fn f85(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m309 = v0;
    match m309.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h310, _) = v1;
                h310
            }, f85(v2))
        }
    }
}

pub fn f86(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m311 = v0;
    match m311.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f144(v2)?;
            let a313 = {
                let a312 = v4.clone();
                length_bytes(a312)
            };
            let a314 = 0u64;
            if nat_eq(a313, a314) {
                f86(v3, v1)
            } else {
                let m315 = f145(v4)?;
                match m315 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f148(v5.clone());
                        let a317 = {
                            let a316 = v6.clone();
                            length_bytes(a316)
                        };
                        let a318 = 0u64;
                        if nat_eq(a317, a318) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m319 = f86(v3, v1.clone())?;
                            match m319 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f26(v6, v1) {
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

pub fn f87() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f88(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f103(Adt14::C3(std::rc::Rc::new(f104(v1))))?))
}

pub fn f89(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m320 = v0;
    match m320.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f89(v3, v1))
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f145(f50(v0, v1))
}

pub fn f91(p323: List<Adt14>) -> bool {
    let m322 = p323;
    {
        let mut m322 = m322;
        loop {
            let step = {
                let v0 = m322;
                let m321 = v0;
                match m321.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f26(f93(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f62(f8(Str::lit("studio")), f52(f149(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m322 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Option<Adt14>) -> List<Adt14> {
    let m324 = v0;
    match m324 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f150(v1)
        }
    }
}

pub fn f93(v0: Adt14, v1: Bytes) -> Bytes {
    let m325 = f96(f95(v0, v1));
    match m325 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f94() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f95(p331: Adt14, v1: Bytes) -> Option<Adt14> {
    let m330 = p331;
    {
        let mut m330 = m330;
        loop {
            let step = {
                let v0 = m330;
                let m326 = v0;
                match m326 {
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
                    Adt14::C4(r327) => {
                        let v5 = (*r327).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r328, r329) => {
                        let v3 = (*r328).clone();
                        let v4 = (*r329).clone();
                        if f26(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: Option<Adt14>) -> Option<Bytes> {
    let m332 = v0;
    match m332 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m333 = v1;
            match m333 {
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

pub fn f97(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f98(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f80(f50(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f151(v1, v4)?)?), List::cons(f81(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f99(v0: Bytes) -> R<Bytes> {
    Ok(f39(f38(v0, 47u8)?))
}

pub fn f100(v0: List<Bytes>) -> Option<Adt14> {
    let m334 = v0;
    match m334.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("req"))) {
                let m335 = v2;
                match m335.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f26(v1.clone(), f8(Str::lit("std"))) {
                    let m336 = v2;
                    match m336.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m337 = v6;
                                match m337.uncons() {
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
                    if f26(v1, f8(Str::lit("der"))) {
                        let m338 = v2;
                        match m338.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m339 = v10;
                                match m339.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f141(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f101(v0: Bytes) -> R<Bytes> {
    f152(v0.clone(), 0u64, {
        let a340 = v0;
        length_bytes(a340)
    }, true, Bytes::lit(&[]))
}

pub fn f102(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m341 = f153(4096u64, f154(f155(f156(f95(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt17::C0(v2, v0), List::<Adt17>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m341 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f140(f157(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f103(v0: Adt14) -> R<Bytes> {
    let a342 = f158(v0)?;
    let a343 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a342, a343))
}

pub fn f104(v0: List<Adt14>) -> Adt14 {
    let m344 = v0;
    match m344.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f104(v2)))
        }
    }
}

pub fn f105(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m345 = v0;
    match m345.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f105(v3, v1))
        }
    }
}

pub fn f106(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f140(f159(f84(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f107(v0: Fn0, p350: u64, p351: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m349 = (p350, p351);
    {
        let mut m349 = m349;
        loop {
            let step = {
                let (v1, v2) = m349;
                let m346 = v1;
                if m346 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m346 - 1;
                    let m348 = {
                        let c347: Fn0 = v0.clone();
                        c347.apply(v2.clone())?
                    };
                    match m348 {
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
                Ok(next) => { m349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f160(v1, v0, v2)
}

pub fn f109(v0: List<Bytes>) -> List<Bytes> {
    f125(Fn1::F161, List::<Bytes>::nil(), v0)
}

pub fn f110(p356: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m355 = p356;
    {
        let mut m355 = m355;
        loop {
            let step = {
                let v0 = m355;
                let m352 = v0;
                match m352.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h353, _) = v2;
                            h353
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h354) = v2;
                                h354
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: Adt1, v1: Bytes) -> bool {
    let m358 = f110(match v0 {
        Adt1::C0(h357, _, _) => {
            h357
        }
    }, v1);
    match m358 {
        None => {
            false
        }
        Some(v2) => {
            let m359 = v2;
            match m359 {
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

pub fn f112(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a360 = v0.clone();
        length_bytes(a360)
    };
    let v3: u64 = {
        let a361 = v1.clone();
        length_bytes(a361)
    };
    let a362 = v3;
    let a363 = v2;
    if nat_le(a362, a363) {
        let m369 = {
            let a366 = v0;
            let a367 = {
                let a364 = v2;
                let a365 = v3;
                nat_sub(a364, a365)
            };
            let a368 = v3;
            slice_bytes(a366, a367, a368)
        };
        match m369 {
            None => {
                false
            }
            Some(v4) => {
                let a370 = v4;
                let a371 = v1;
                equal(a370, a371)
            }
        }
    } else {
        false
    }
}

pub fn f113(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m372 = v1;
    match m372.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f162(v3.clone())
            } else {
                f55(f162(v3.clone()))
            } {
                List::cons(f97(v0.clone(), v3), f113(v0, v4, v2))
            } else {
                f113(v0, v4, v2)
            }
        }
    }
}

pub fn f114(v0: Adt12) -> R<Adt15> {
    Ok(Adt15::C0(f163(match v0.clone() {
        Adt12::C0(h373, _, _, _, _) => {
            h373
        }
    }, 0u64)?, f163(match v0.clone() {
        Adt12::C0(_, h374, _, _, _) => {
            h374
        }
    }, 0u64)?, f163(match v0.clone() {
        Adt12::C0(_, _, h375, _, _) => {
            h375
        }
    }, 0u64)?, f163(match v0 {
        Adt12::C0(_, _, _, h376, _) => {
            h376
        }
    }, 0u64)?))
}

pub fn f115(v0: Adt12) -> List<Bytes> {
    f45(match v0.clone() {
        Adt12::C0(h377, _, _, _, _) => {
            h377
        }
    }, f45(match v0.clone() {
        Adt12::C0(_, _, h378, _, _) => {
            h378
        }
    }, f45(match v0.clone() {
        Adt12::C0(_, _, _, h379, _) => {
            h379
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h380) => {
            h380
        }
    })))
}

pub fn f116(v0: Adt15, p386: List<u64>, p387: List<Adt18>) -> Option<u64> {
    let m385 = (p386, p387);
    {
        let mut m385 = m385;
        loop {
            let step = {
                let (v1, v2) = m385;
                let m381 = v2;
                match m381.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f164(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h382) => {
                                h382
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h383, _) => {
                                    h383
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h384, _) => {
                                    h384
                                }
                            }))
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

pub fn f117(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m388 = v1;
    match m388.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f165(v0.clone(), v3)?), f117(v0, v4, {
                let a389 = v2;
                let a390 = 1u64;
                nat_add(a389, a390)?
            })?))
        }
    }
}

pub fn f118(p394: List<Bytes>, p395: u64) -> Bytes {
    let m393 = (p394, p395);
    {
        let mut m393 = m393;
        loop {
            let step = {
                let (v0, v1) = m393;
                let m391 = v0;
                match m391.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m392 = v1;
                        if m392 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m392 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f166(v0.clone(), v1.clone()) {
        if f166(v1, v0) {
            f167(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f120(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m396 = v0;
    match m396.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f168(v2, v1.clone())?, f120(v3, v1)?))
        }
    }
}

pub fn f121(v0: List<Adt14>) -> List<u64> {
    let m397 = v0;
    match m397.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f169(v1), f121(v2))
        }
    }
}

pub fn f122(p400: List<u64>, v1: List<u64>) -> Option<u64> {
    let m399 = p400;
    {
        let mut m399 = m399;
        loop {
            let step = {
                let v0 = m399;
                let m398 = v0;
                match m398.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f170(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m401 = v0;
    match m401.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m402 = f171(v2, v1.clone())?;
            match m402 {
                None => {
                    f123(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f123(v3, v1)?))
                }
            }
        }
    }
}

pub fn f124(v0: List<Bytes>) -> List<Bytes> {
    f125(Fn1::F172, List::<Bytes>::nil(), v0)
}

pub fn f125(v0: Fn1, p406: List<Bytes>, p407: List<Bytes>) -> List<Bytes> {
    let m405 = (p406, p407);
    {
        let mut m405 = m405;
        loop {
            let step = {
                let (v1, v2) = m405;
                let m403 = v2;
                match m403.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c404: Fn1 = v0.clone();
                            c404.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m405 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f173(v0, v1)
}

pub fn f127(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f174(f175(f176(v0))?))
}

pub fn f128(v0: Adt1, v1: Bytes) -> R<bool> {
    let m408 = f90(v0.clone(), v1)?;
    match m408 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let v3: List<Bytes> = f177(f138(f84(f50(v0, f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f93(v2.clone(), f8(Str::lit("id"))))?;
            let m409 = v3.clone();
            match m409.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m411 = f96(f95({
                        let m410 = f95(v2, f8(Str::lit("source")));
                        match m410 {
                            None => {
                                Adt14::C5
                            }
                            Some(v6) => {
                                v6
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m411 {
                        None => {
                            Ok(false)
                        }
                        Some(v7) => {
                            Ok(f55(f62(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f134(v7)?, List::<Bytes>::nil()))), v3)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f129(v0: List<Adt14>) -> List<Bytes> {
    let m412 = v0;
    match m412.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f93(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f93(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f93(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f129(v2))
        }
    }
}

pub fn f130(v0: List<Adt14>) -> List<Bytes> {
    let m413 = v0;
    match m413.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f93(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f178(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f130(v2))
        }
    }
}

pub fn f131(v0: Bytes, p471: u64, p472: u64, p473: Adt16) -> R<Adt16> {
    let m470 = (p471, (p472, p473));
    {
        let mut m470 = m470;
        loop {
            let step = {
                let (v1, (v2, v3)) = m470;
                let m414 = v2;
                if m414 == 0 {
                    Err::<(u64, (u64, Adt16)), R<Adt16>>(Ok(v3))
                } else {
                    let v4 = m414 - 1;
                    let v5: u64 = f179(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt16::C0(_, h415, _, _, _) => {
                            h415
                        }
                    } {
                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                            let a416 = v1;
                            let a417 = 1u64;
                            nat_add(a416, a417)?
                        }, (v4, Adt16::C0(true, false, match v3.clone() {
                            Adt16::C0(_, _, h418, _, _) => {
                                h418
                            }
                        }, match v3.clone() {
                            Adt16::C0(_, _, _, h419, _) => {
                                h419
                            }
                        }, match v3 {
                            Adt16::C0(_, _, _, _, h420) => {
                                h420
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt16::C0(h421, _, _, _, _) => {
                                h421
                            }
                        } {
                            let a422 = v5;
                            let a423 = 34u64;
                            if nat_eq(a422, a423) {
                                let a426 = f179(v0.clone(), {
                                    let a424 = v1;
                                    let a425 = 1u64;
                                    nat_add(a424, a425)?
                                })?;
                                let a427 = 34u64;
                                if nat_eq(a426, a427) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a428 = v1;
                                        let a429 = 1u64;
                                        nat_add(a428, a429)?
                                    }, (v4, Adt16::C0(true, true, f180(match v3.clone() {
                                        Adt16::C0(_, _, h430, _, _) => {
                                            h430
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt16::C0(_, _, _, h431, _) => {
                                            h431
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h432) => {
                                            h432
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a433 = v1;
                                        let a434 = 1u64;
                                        nat_add(a433, a434)?
                                    }, (v4, Adt16::C0(false, false, match v3.clone() {
                                        Adt16::C0(_, _, h435, _, _) => {
                                            h435
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h436, _) => {
                                            h436
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h437) => {
                                            h437
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a438 = v1;
                                    let a439 = 1u64;
                                    nat_add(a438, a439)?
                                }, (v4, Adt16::C0(true, false, f180(match v3.clone() {
                                    Adt16::C0(_, _, h440, _, _) => {
                                        h440
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt16::C0(_, _, _, h441, _) => {
                                        h441
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h442) => {
                                        h442
                                    }
                                }))))
                            }
                        } else {
                            let a443 = v5;
                            let a444 = 34u64;
                            if nat_eq(a443, a444) {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a445 = v1;
                                    let a446 = 1u64;
                                    nat_add(a445, a446)?
                                }, (v4, Adt16::C0(true, false, match v3.clone() {
                                    Adt16::C0(_, _, h447, _, _) => {
                                        h447
                                    }
                                }, match v3.clone() {
                                    Adt16::C0(_, _, _, h448, _) => {
                                        h448
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h449) => {
                                        h449
                                    }
                                }))))
                            } else {
                                let a450 = v5;
                                let a451 = 44u64;
                                if nat_eq(a450, a451) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a452 = v1;
                                        let a453 = 1u64;
                                        nat_add(a452, a453)?
                                    }, (v4, Adt16::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt16::C0(_, _, h454, _, _) => {
                                            h454
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h455, _) => {
                                            h455
                                        }
                                    }), match v3 {
                                        Adt16::C0(_, _, _, _, h456) => {
                                            h456
                                        }
                                    }))))
                                } else {
                                    let a457 = v5;
                                    let a458 = 13u64;
                                    if nat_eq(a457, a458) {
                                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                            let a459 = v1;
                                            let a460 = 1u64;
                                            nat_add(a459, a460)?
                                        }, (v4, v3)))
                                    } else {
                                        let a461 = v5;
                                        let a462 = 10u64;
                                        if nat_eq(a461, a462) {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a463 = v1;
                                                let a464 = 1u64;
                                                nat_add(a463, a464)?
                                            }, (v4, f132(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a465 = v1;
                                                let a466 = 1u64;
                                                nat_add(a465, a466)?
                                            }, (v4, Adt16::C0(false, false, f180(match v3.clone() {
                                                Adt16::C0(_, _, h467, _, _) => {
                                                    h467
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt16::C0(_, _, _, h468, _) => {
                                                    h468
                                                }
                                            }, match v3 {
                                                Adt16::C0(_, _, _, _, h469) => {
                                                    h469
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
                Ok(next) => { m470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: Adt16) -> Adt16 {
    Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f181(List::cons(match v0.clone() {
        Adt16::C0(_, _, h474, _, _) => {
            h474
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h475, _) => {
            h475
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt16::C0(_, _, _, _, h476) => {
            h476
        }
    }))
}

pub fn f133(p479: List<List<Bytes>>, p480: List<List<Bytes>>) -> List<List<Bytes>> {
    let m478 = (p479, p480);
    {
        let mut m478 = m478;
        loop {
            let step = {
                let (v0, v1) = m478;
                let m477 = v0;
                match m477.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m478 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f182(v0.clone(), 0u64, {
        let a481 = v0;
        length_bytes(a481)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f135(v0: Adt21) -> Bytes {
    let a502 = f183(match v0.clone() {
        Adt21::C0(h482, _, _, _, _, _, _, _) => {
            h482
        }
    });
    let a503 = {
        let a500 = f183(match v0.clone() {
            Adt21::C0(_, h483, _, _, _, _, _, _) => {
                h483
            }
        });
        let a501 = {
            let a498 = f183(match v0.clone() {
                Adt21::C0(_, _, h484, _, _, _, _, _) => {
                    h484
                }
            });
            let a499 = {
                let a496 = f183(match v0.clone() {
                    Adt21::C0(_, _, _, h485, _, _, _, _) => {
                        h485
                    }
                });
                let a497 = {
                    let a494 = f183(match v0.clone() {
                        Adt21::C0(_, _, _, _, h486, _, _, _) => {
                            h486
                        }
                    });
                    let a495 = {
                        let a492 = f183(match v0.clone() {
                            Adt21::C0(_, _, _, _, _, h487, _, _) => {
                                h487
                            }
                        });
                        let a493 = {
                            let a490 = f183(match v0.clone() {
                                Adt21::C0(_, _, _, _, _, _, h488, _) => {
                                    h488
                                }
                            });
                            let a491 = f183(match v0 {
                                Adt21::C0(_, _, _, _, _, _, _, h489) => {
                                    h489
                                }
                            });
                            append_bytes(a490, a491)
                        };
                        append_bytes(a492, a493)
                    };
                    append_bytes(a494, a495)
                };
                append_bytes(a496, a497)
            };
            append_bytes(a498, a499)
        };
        append_bytes(a500, a501)
    };
    append_bytes(a502, a503)
}

pub fn f136(v0: Bytes) -> R<Adt21> {
    let v1: u64 = {
        let a504 = v0.clone();
        length_bytes(a504)
    };
    let v2: u64 = {
        let a510 = {
            let a507 = {
                let a505 = v1;
                let a506 = 72u64;
                nat_add(a505, a506)?
            };
            let a508 = 64u64;
            let a509 = 0u64;
            nat_quot(a507, a508, a509)
        };
        let a511 = 64u64;
        nat_mul(a510, a511)?
    };
    let (_, h512) = f184(Fn3::F185(v0, v1, v2), {
        let a513 = v2;
        let a514 = 64u64;
        let a515 = 0u64;
        nat_quot(a513, a514, a515)
    }, (0u64, f186()))?;
    Ok(h512)
}

pub fn f137(p518: List<List<Bytes>>, v1: Bytes, p519: List<Bytes>) -> R<List<Bytes>> {
    let m517 = (p518, p519);
    {
        let mut m517 = m517;
        loop {
            let step = {
                let (v0, v2) = m517;
                let m516 = v0;
                match m516.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f187(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f140(f188(v3), Bytes::lit(&[0x0a])), v2)))
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

pub fn f138(p522: List<Bytes>, p523: List<Bytes>, p524: List<List<Bytes>>) -> List<List<Bytes>> {
    let m521 = (p522, (p523, p524));
    {
        let mut m521 = m521;
        loop {
            let step = {
                let (v0, (v1, v2)) = m521;
                let m520 = v0;
                match m520.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f157(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f26(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f157(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m521 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Bytes) -> R<Bytes> {
    Ok(f40(v0.clone(), 0u64, f189(v0)?))
}

pub fn f140(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m525 = v0;
    match m525.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m526 = v3.clone();
            match m526.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a529 = v2;
                    let a530 = {
                        let a527 = v1.clone();
                        let a528 = f140(v3, v1);
                        append_bytes(a527, a528)
                    };
                    append_bytes(a529, a530)
                }
            }
        }
    }
}

pub fn f141(v0: List<Bytes>) -> Adt14 {
    let m531 = v0;
    match m531.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f141(v2)))
        }
    }
}

pub fn f142(v0: Fn2, p535: Option<Bytes>, p536: List<Bytes>) -> Option<Bytes> {
    let m534 = (p535, p536);
    {
        let mut m534 = m534;
        loop {
            let step = {
                let (v1, v2) = m534;
                let m532 = v2;
                match m532.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c533: Fn2 = v0.clone();
                            c533.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m534 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m537 = v1;
    match m537 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a540 = {
                    let a538 = v3;
                    let a539 = v0;
                    append_bytes(a538, a539)
                };
                let a541 = v2;
                append_bytes(a540, a541)
            })
        }
    }
}

pub fn f144(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f189(v0.clone())?;
    let v2: u64 = f190(v0.clone(), v1)?;
    Ok(f40(v0, v2, {
        let a542 = v1;
        let a543 = v2;
        nat_sub(a542, a543)
    }))
}

pub fn f145(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f191(v0)?;
    let m547 = f192(v1.clone(), 0u64, {
        let a545 = {
            let a544 = v1.clone();
            length_bytes(a544)
        };
        let a546 = 2u64;
        nat_add(a545, a546)?
    }, 0u64)?;
    match m547 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a550 = {
                let (_, h548) = v2;
                h548
            };
            let a551 = {
                let a549 = v1;
                length_bytes(a549)
            };
            if nat_eq(a550, a551) {
                Ok(Some({
                    let (h552, _) = v2;
                    h552
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f146(v0: Bytes) -> Bytes {
    f7(List::cons(f57(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f147(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f193(v0, v1)
}

pub fn f148(v0: Adt14) -> Bytes {
    if f194(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f194(v0.clone(), f8(Str::lit("id"))) {
            if f194(v0.clone(), f8(Str::lit("statement"))) {
                if f194(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f195(v0, f8(Str::lit("evidence")))
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

pub fn f149(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m553 = f95(v0, v1);
    match m553 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f92(f95(v3, v2))
        }
    }
}

pub fn f150(v0: Adt14) -> List<Adt14> {
    let m554 = v0;
    match m554 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r555) => {
            let v3 = (*r555).clone();
            f150(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r556, r557) => {
            let v1 = (*r556).clone();
            let v2 = (*r557).clone();
            List::cons(v1, f150(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f151(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f95(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f78(Str::lit("id"), f93(v0.clone(), f8(Str::lit("id"))))?, List::cons(f78(Str::lit("name"), f93(v0.clone(), f8(Str::lit("name"))))?, List::cons(f78(Str::lit("issuer"), f93(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f78(Str::lit("edition"), f93(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f78(Str::lit("canonical_id"), f93(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f78(Str::lit("source_role"), f93(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f78(Str::lit("retrieval_date"), f93(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f78(Str::lit("url"), f196(v2.clone(), f8(Str::lit("url"))))?, List::cons(f78(Str::lit("media_type"), f196(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f78(Str::lit("license"), f196(v2.clone(), f8(Str::lit("license"))))?, List::cons(f78(Str::lit("redistribution"), f196(v2, f8(Str::lit("redistribution"))))?, List::cons(f78(Str::lit("statement"), f93(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m558 = v1;
        match m558 {
            None => {
                f7(List::cons(f78(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f78(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f78(Str::lit("artifact"), {
                    let (h559, _) = v3;
                    h559
                })?, List::cons(f78(Str::lit("checksum"), {
                    let (_, h560) = v3;
                    h560
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f197(f54(f92(f95(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f152(v0: Bytes, p627: u64, p628: u64, p629: bool, p630: Bytes) -> R<Bytes> {
    let m626 = (p627, (p628, (p629, p630)));
    {
        let mut m626 = m626;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m626;
                let m561 = v2;
                if m561 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m561 - 1;
                    let m564 = {
                        let a562 = v0.clone();
                        let a563 = v1;
                        index_bytes(a562, a563)
                    };
                    match m564 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a609 = {
                                    let a585 = {
                                        let a573 = {
                                            let a567 = {
                                                let a565 = v7;
                                                let a566 = 128u8;
                                                fixed_u8::bit_and(a565, a566)
                                            };
                                            let a568 = 128u8;
                                            if equal(a567, a568) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a574 = {
                                            let a571 = {
                                                let a569 = v7;
                                                let a570 = 64u8;
                                                fixed_u8::bit_and(a569, a570)
                                            };
                                            let a572 = 64u8;
                                            if equal(a571, a572) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a573, a574)?
                                    };
                                    let a586 = {
                                        let a583 = {
                                            let a577 = {
                                                let a575 = v7;
                                                let a576 = 32u8;
                                                fixed_u8::bit_and(a575, a576)
                                            };
                                            let a578 = 32u8;
                                            if equal(a577, a578) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a584 = {
                                            let a581 = {
                                                let a579 = v7;
                                                let a580 = 16u8;
                                                fixed_u8::bit_and(a579, a580)
                                            };
                                            let a582 = 16u8;
                                            if equal(a581, a582) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a583, a584)?
                                    };
                                    nat_add(a585, a586)?
                                };
                                let a610 = {
                                    let a607 = {
                                        let a595 = {
                                            let a589 = {
                                                let a587 = v7;
                                                let a588 = 8u8;
                                                fixed_u8::bit_and(a587, a588)
                                            };
                                            let a590 = 8u8;
                                            if equal(a589, a590) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a596 = {
                                            let a593 = {
                                                let a591 = v7;
                                                let a592 = 4u8;
                                                fixed_u8::bit_and(a591, a592)
                                            };
                                            let a594 = 4u8;
                                            if equal(a593, a594) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a595, a596)?
                                    };
                                    let a608 = {
                                        let a605 = {
                                            let a599 = {
                                                let a597 = v7;
                                                let a598 = 2u8;
                                                fixed_u8::bit_and(a597, a598)
                                            };
                                            let a600 = 2u8;
                                            if equal(a599, a600) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a606 = {
                                            let a603 = {
                                                let a601 = v7;
                                                let a602 = 1u8;
                                                fixed_u8::bit_and(a601, a602)
                                            };
                                            let a604 = 1u8;
                                            if equal(a603, a604) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a605, a606)?
                                    };
                                    nat_add(a607, a608)?
                                };
                                nat_add(a609, a610)?
                            };
                            if f198(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a611 = v1;
                                    let a612 = 1u64;
                                    nat_add(a611, a612)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f199(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a613 = v1;
                                        let a614 = 1u64;
                                        nat_add(a613, a614)?
                                    }, (v5, (false, {
                                        let a618 = v4;
                                        let a619 = f40({
                                            let a615 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a615)
                                        }, {
                                            let a616 = v8;
                                            let a617 = 97u64;
                                            nat_sub(a616, a617)
                                        }, 1u64);
                                        append_bytes(a618, a619)
                                    }))))
                                } else {
                                    if if f199(v8) {
                                        true
                                    } else {
                                        if f200(v8) {
                                            true
                                        } else {
                                            f201(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a620 = v1;
                                            let a621 = 1u64;
                                            nat_add(a620, a621)?
                                        }, (v5, (false, {
                                            let a622 = v4;
                                            let a623 = f40(v0.clone(), v1, 1u64);
                                            append_bytes(a622, a623)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a624 = v1;
                                            let a625 = 1u64;
                                            nat_add(a624, a625)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m626 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(p638: u64, p639: List<Adt17>, p640: List<Bytes>, p641: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m637 = (p638, (p639, (p640, p641)));
    {
        let mut m637 = m637;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m637;
                let m631 = v0;
                if m631 == 0 {
                    Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m631 - 1;
                    let m632 = v1;
                    match m632.uncons() {
                        None => {
                            Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m633 = v5;
                            match m633 {
                                Adt17::C0(v8, v9) => {
                                    if f62(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m634 = f202(v8.clone(), v9)?;
                                        match m634 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f154({
                                                    let (_, h635) = v11;
                                                    h635
                                                }, List::cons(Adt17::C1({
                                                    let (h636, _) = v11;
                                                    h636
                                                }), v6)), (List::cons(v8, v2), v3))))
                                            }
                                            Err(v10) => {
                                                Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                            }
                                        }
                                    }
                                }
                                Adt17::C1(v7) => {
                                    Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m637 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: List<Adt17>, v1: List<Adt17>) -> List<Adt17> {
    let m642 = v0;
    match m642.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f154(v3, v1))
        }
    }
}

pub fn f155(v0: List<(Bytes, Adt14)>) -> R<List<Adt17>> {
    let m643 = v0;
    match m643.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt17::C0(f101({
                let (h644, _) = v1;
                h644
            })?, {
                let (_, h645) = v1;
                h645
            }), f155(v2)?))
        }
    }
}

pub fn f156(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m646 = v0;
    match m646 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f203(v1)
        }
    }
}

pub fn f157(p649: List<Bytes>, p650: List<Bytes>) -> List<Bytes> {
    let m648 = (p649, p650);
    {
        let mut m648 = m648;
        loop {
            let step = {
                let (v0, v1) = m648;
                let m647 = v0;
                match m647.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m648 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: Adt14) -> R<Bytes> {
    let m651 = v0;
    match m651 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f204(v2, v3)
        }
        Adt14::C2(v4) => {
            f205(v4)
        }
        Adt14::C3(r652) => {
            let v5 = (*r652).clone();
            Ok(f206(f207(v5)?))
        }
        Adt14::C4(r653) => {
            let v6 = (*r653).clone();
            f208(f209(v6)?)
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

pub fn f159(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m654 = v0;
    match m654.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f210(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f140(f211(f58(List::cons(v1, f212(f38(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f159(v3, v1)?))
            }
        }
    }
}

pub fn f160(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h655, _) = v2.clone();
        h655
    };
    let v4: u64 = {
        let (h656, _) = {
            let (_, h657) = v2.clone();
            h657
        };
        h656
    };
    let v5: List<Bytes> = {
        let (_, h658) = {
            let (_, h659) = v2;
            h659
        };
        h658
    };
    let m660 = f213(v0.clone(), v3);
    match m660 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a661 = v6;
            let a662 = v1;
            if equal(a661, a662) {
                Ok(Some(({
                    let a663 = v3;
                    let a664 = 1u64;
                    nat_add(a663, a664)?
                }, ({
                    let a665 = v3;
                    let a666 = 1u64;
                    nat_add(a665, a666)?
                }, List::cons(f40(v0, v4, {
                    let a667 = v3;
                    let a668 = v4;
                    nat_sub(a667, a668)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a669 = v3;
                    let a670 = 1u64;
                    nat_add(a669, a670)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f161(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f162(v0: Bytes) -> bool {
    f112(v0, f8(Str::lit(".lex.tex")))
}

pub fn f163(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m671 = v0;
    match m671.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f163(v3, {
                let a672 = v1;
                let a673 = 1u64;
                nat_add(a672, a673)?
            })?))
        }
    }
}

pub fn f164(v0: Adt15, v1: List<u64>, v2: Adt19) -> bool {
    let m674 = v2;
    match m674 {
        Adt19::C0(v3, v4) => {
            if f170(v3, match v0.clone() {
                Adt15::C0(h675, _, _, _) => {
                    h675
                }
            }) {
                f214(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f170(v5, match v0 {
                Adt15::C0(_, _, h676, _) => {
                    h676
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f166(v7, v1) {
                f170(v8, match v0 {
                    Adt15::C0(_, _, _, h677) => {
                        h677
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f165(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m678 = f95(v1.clone(), f8(Str::lit("std")));
    match m678 {
        None => {
            let m683 = f96(f95(v1.clone(), f8(Str::lit("req"))));
            match m683 {
                None => {
                    let m685 = f95(v1, f8(Str::lit("der")));
                    match m685 {
                        None => {
                            Ok(f215(v0))
                        }
                        Some(v5) => {
                            let m686 = f216(f93(v5.clone(), f8(Str::lit("rule"))));
                            match m686 {
                                None => {
                                    Ok(f215(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f120(f54(f92(f95(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h687) => {
                                            h687
                                        }
                                    })?, f168(f93(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h688, _) => {
                                            h688
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f168(v4, match v0 {
                        Adt12::C0(_, _, h684, _, _) => {
                            h684
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m679 = f96(f95(v2.clone(), f8(Str::lit("row"))));
            match m679 {
                None => {
                    Ok(Adt19::C0(f168(f93(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h680, _, _, _, _) => {
                            h680
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f168(f93(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h681, _, _, _, _) => {
                            h681
                        }
                    })?, Some(f168(v3, match v0 {
                        Adt12::C0(_, h682, _, _, _) => {
                            h682
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f166(p691: List<u64>, v1: List<u64>) -> bool {
    let m690 = p691;
    {
        let mut m690 = m690;
        loop {
            let step = {
                let v0 = m690;
                let m689 = v0;
                match m689.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f170(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f167(p696: List<u64>) -> bool {
    let m695 = p696;
    {
        let mut m695 = m695;
        loop {
            let step = {
                let v0 = m695;
                let m692 = v0;
                match m692.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a693 = v1;
                        let a694 = 7u64;
                        if nat_le(a693, a694) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f168(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m697 = f217(v0, v1.clone(), 0u64)?;
    match m697 {
        None => {
            let a698 = v1;
            Ok(length_list(a698))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f169(v0: Adt14) -> u64 {
    let m699 = f218(f95(v0, f8(Str::lit("stratum"))));
    match m699 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f170(v0: u64, p704: List<u64>) -> bool {
    let m703 = p704;
    {
        let mut m703 = m703;
        loop {
            let step = {
                let v1 = m703;
                let m700 = v1;
                match m700.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a701 = v2;
                        let a702 = v0;
                        if nat_eq(a701, a702) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f210(v0.clone(), v1) {
        let m705 = f38(v0, 34u8)?;
        match m705.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m706 = v3;
                match m706.uncons() {
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

pub fn f172(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f173(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m707 = v0.clone();
    match m707.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a708 = v2;
            let a709 = v1.clone();
            if equal(a708, a709) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f174(v0: List<(Bytes, ())>) -> List<Bytes> {
    f124(f219(Fn4::F220, List::<Bytes>::nil(), v0))
}

pub fn f175(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f221(f222(Fn5::F223, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m714 = {
        let (h710, _) = f224(Fn6::F225, {
            let a712 = {
                let a711 = v0;
                length_list(a711)
            };
            let a713 = 1u64;
            nat_add(a712, a713)?
        }, v1)?;
        h710
    };
    match m714.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f176(v0: List<Bytes>) -> List<(Bytes, ())> {
    f226(f227(Fn7::F228, List::<(Bytes, ())>::nil(), v0))
}

pub fn f177(p717: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m716 = p717;
    {
        let mut m716 = m716;
        loop {
            let step = {
                let v0 = m716;
                let m715 = v0;
                match m715.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f187(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Adt14) -> Bytes {
    if f194(v0.clone(), f8(Str::lit("req"))) {
        let a718 = f8(Str::lit("req "));
        let a719 = f93(v0, f8(Str::lit("req")));
        append_bytes(a718, a719)
    } else {
        let m720 = f95(v0.clone(), f8(Str::lit("std")));
        match m720 {
            None => {
                let m723 = f95(v0, f8(Str::lit("der")));
                match m723 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f93(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f82(f54(f92(f95(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f93(v1.clone(), f8(Str::lit("authority"))), List::cons(if f194(v1.clone(), f8(Str::lit("row"))) {
                    let a721 = f8(Str::lit(" row "));
                    let a722 = f93(v1, f8(Str::lit("row")));
                    append_bytes(a721, a722)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f179(v0: Bytes, v1: u64) -> R<u64> {
    let m726 = {
        let a724 = v0;
        let a725 = v1;
        index_bytes(a724, a725)
    };
    match m726 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a771 = {
                let a747 = {
                    let a735 = {
                        let a729 = {
                            let a727 = v3;
                            let a728 = 128u8;
                            fixed_u8::bit_and(a727, a728)
                        };
                        let a730 = 128u8;
                        if equal(a729, a730) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a736 = {
                        let a733 = {
                            let a731 = v3;
                            let a732 = 64u8;
                            fixed_u8::bit_and(a731, a732)
                        };
                        let a734 = 64u8;
                        if equal(a733, a734) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a735, a736)?
                };
                let a748 = {
                    let a745 = {
                        let a739 = {
                            let a737 = v3;
                            let a738 = 32u8;
                            fixed_u8::bit_and(a737, a738)
                        };
                        let a740 = 32u8;
                        if equal(a739, a740) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a746 = {
                        let a743 = {
                            let a741 = v3;
                            let a742 = 16u8;
                            fixed_u8::bit_and(a741, a742)
                        };
                        let a744 = 16u8;
                        if equal(a743, a744) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a745, a746)?
                };
                nat_add(a747, a748)?
            };
            let a772 = {
                let a769 = {
                    let a757 = {
                        let a751 = {
                            let a749 = v3;
                            let a750 = 8u8;
                            fixed_u8::bit_and(a749, a750)
                        };
                        let a752 = 8u8;
                        if equal(a751, a752) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a758 = {
                        let a755 = {
                            let a753 = v3;
                            let a754 = 4u8;
                            fixed_u8::bit_and(a753, a754)
                        };
                        let a756 = 4u8;
                        if equal(a755, a756) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a757, a758)?
                };
                let a770 = {
                    let a767 = {
                        let a761 = {
                            let a759 = v3;
                            let a760 = 2u8;
                            fixed_u8::bit_and(a759, a760)
                        };
                        let a762 = 2u8;
                        if equal(a761, a762) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a768 = {
                        let a765 = {
                            let a763 = v3;
                            let a764 = 1u8;
                            fixed_u8::bit_and(a763, a764)
                        };
                        let a766 = 1u8;
                        if equal(a765, a766) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a767, a768)?
                };
                nat_add(a769, a770)?
            };
            nat_add(a771, a772)
        }
    }
}

pub fn f180(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m776 = {
        let a773 = v1;
        let a774 = v2;
        let a775 = 1u64;
        slice_bytes(a773, a774, a775)
    };
    match m776 {
        None => {
            v0
        }
        Some(v3) => {
            let a777 = v0;
            let a778 = v3;
            append_bytes(a777, a778)
        }
    }
}

pub fn f181(p781: List<Bytes>, p782: List<Bytes>) -> List<Bytes> {
    let m780 = (p781, p782);
    {
        let mut m780 = m780;
        loop {
            let step = {
                let (v0, v1) = m780;
                let m779 = v0;
                match m779.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes, p854: u64, p855: u64, p856: Bytes) -> R<Bytes> {
    let m853 = (p854, (p855, p856));
    {
        let mut m853 = m853;
        loop {
            let step = {
                let (v1, (v2, v3)) = m853;
                let m783 = v2;
                if m783 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m783 - 1;
                    let m786 = {
                        let a784 = v0.clone();
                        let a785 = v1;
                        index_bytes(a784, a785)
                    };
                    match m786 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a831 = {
                                    let a807 = {
                                        let a795 = {
                                            let a789 = {
                                                let a787 = v6;
                                                let a788 = 128u8;
                                                fixed_u8::bit_and(a787, a788)
                                            };
                                            let a790 = 128u8;
                                            if equal(a789, a790) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a796 = {
                                            let a793 = {
                                                let a791 = v6;
                                                let a792 = 64u8;
                                                fixed_u8::bit_and(a791, a792)
                                            };
                                            let a794 = 64u8;
                                            if equal(a793, a794) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a795, a796)?
                                    };
                                    let a808 = {
                                        let a805 = {
                                            let a799 = {
                                                let a797 = v6;
                                                let a798 = 32u8;
                                                fixed_u8::bit_and(a797, a798)
                                            };
                                            let a800 = 32u8;
                                            if equal(a799, a800) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a806 = {
                                            let a803 = {
                                                let a801 = v6;
                                                let a802 = 16u8;
                                                fixed_u8::bit_and(a801, a802)
                                            };
                                            let a804 = 16u8;
                                            if equal(a803, a804) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a805, a806)?
                                    };
                                    nat_add(a807, a808)?
                                };
                                let a832 = {
                                    let a829 = {
                                        let a817 = {
                                            let a811 = {
                                                let a809 = v6;
                                                let a810 = 8u8;
                                                fixed_u8::bit_and(a809, a810)
                                            };
                                            let a812 = 8u8;
                                            if equal(a811, a812) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a818 = {
                                            let a815 = {
                                                let a813 = v6;
                                                let a814 = 4u8;
                                                fixed_u8::bit_and(a813, a814)
                                            };
                                            let a816 = 4u8;
                                            if equal(a815, a816) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a817, a818)?
                                    };
                                    let a830 = {
                                        let a827 = {
                                            let a821 = {
                                                let a819 = v6;
                                                let a820 = 2u8;
                                                fixed_u8::bit_and(a819, a820)
                                            };
                                            let a822 = 2u8;
                                            if equal(a821, a822) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a828 = {
                                            let a825 = {
                                                let a823 = v6;
                                                let a824 = 1u8;
                                                fixed_u8::bit_and(a823, a824)
                                            };
                                            let a826 = 1u8;
                                            if equal(a825, a826) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a827, a828)?
                                    };
                                    nat_add(a829, a830)?
                                };
                                nat_add(a831, a832)?
                            };
                            let a833 = v7;
                            let a834 = 34u64;
                            if if nat_eq(a833, a834) {
                                true
                            } else {
                                let a835 = v7;
                                let a836 = 92u64;
                                nat_eq(a835, a836)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a837 = v1;
                                    let a838 = 1u64;
                                    nat_add(a837, a838)?
                                }, (v4, {
                                    let a841 = v3;
                                    let a842 = {
                                        let a839 = Bytes::lit(&[0x5c]);
                                        let a840 = f40(v0.clone(), v1, 1u64);
                                        append_bytes(a839, a840)
                                    };
                                    append_bytes(a841, a842)
                                })))
                            } else {
                                let a843 = v7;
                                let a844 = 10u64;
                                if nat_eq(a843, a844) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a845 = v1;
                                        let a846 = 1u64;
                                        nat_add(a845, a846)?
                                    }, (v4, {
                                        let a847 = v3;
                                        let a848 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a847, a848)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a849 = v1;
                                        let a850 = 1u64;
                                        nat_add(a849, a850)?
                                    }, (v4, {
                                        let a851 = v3;
                                        let a852 = f40(v0.clone(), v1, 1u64);
                                        append_bytes(a851, a852)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: u32) -> Bytes {
    let a885 = f229({
        let a857 = f230(v0, 28u32);
        let a858 = 15u32;
        fixed_u32::bit_and(a857, a858)
    });
    let a886 = {
        let a883 = f229({
            let a859 = f230(v0, 24u32);
            let a860 = 15u32;
            fixed_u32::bit_and(a859, a860)
        });
        let a884 = {
            let a881 = f229({
                let a861 = f230(v0, 20u32);
                let a862 = 15u32;
                fixed_u32::bit_and(a861, a862)
            });
            let a882 = {
                let a879 = f229({
                    let a863 = f230(v0, 16u32);
                    let a864 = 15u32;
                    fixed_u32::bit_and(a863, a864)
                });
                let a880 = {
                    let a877 = f229({
                        let a865 = f230(v0, 12u32);
                        let a866 = 15u32;
                        fixed_u32::bit_and(a865, a866)
                    });
                    let a878 = {
                        let a875 = f229({
                            let a867 = f230(v0, 8u32);
                            let a868 = 15u32;
                            fixed_u32::bit_and(a867, a868)
                        });
                        let a876 = {
                            let a873 = f229({
                                let a869 = f230(v0, 4u32);
                                let a870 = 15u32;
                                fixed_u32::bit_and(a869, a870)
                            });
                            let a874 = f229({
                                let a871 = v0;
                                let a872 = 15u32;
                                fixed_u32::bit_and(a871, a872)
                            });
                            append_bytes(a873, a874)
                        };
                        append_bytes(a875, a876)
                    };
                    append_bytes(a877, a878)
                };
                append_bytes(a879, a880)
            };
            append_bytes(a881, a882)
        };
        append_bytes(a883, a884)
    };
    append_bytes(a885, a886)
}

pub fn f184(v0: Fn3, p890: u64, p891: (u64, Adt21)) -> R<(u64, Adt21)> {
    let m889 = (p890, p891);
    {
        let mut m889 = m889;
        loop {
            let step = {
                let (v1, v2) = m889;
                let m887 = v1;
                if m887 == 0 {
                    Err::<(u64, (u64, Adt21)), R<(u64, Adt21)>>(Ok(v2))
                } else {
                    let v3 = m887 - 1;
                    Ok::<(u64, (u64, Adt21)), R<(u64, Adt21)>>((v3, {
                        let c888: Fn3 = v0.clone();
                        c888.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt21)) -> R<(u64, Adt21)> {
    Ok(({
        let a893 = {
            let (h892, _) = v3.clone();
            h892
        };
        let a894 = 64u64;
        nat_add(a893, a894)?
    }, f231({
        let (_, h895) = v3;
        h895
    }, f232(v0, v1, v2, {
        let (h896, _) = v3;
        h896
    })?)?))
}

pub fn f186() -> Adt21 {
    Adt21::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f187(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f62(v1, f49(f140(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f188(v0: List<Bytes>) -> List<Bytes> {
    let m897 = v0;
    match m897.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("authority = []"))) {
                f188(v2)
            } else {
                List::cons(v1, f188(v2))
            }
        }
    }
}

pub fn f189(v0: Bytes) -> R<u64> {
    let (h898, _) = f233(Fn8::F234(v0.clone()), {
        let a899 = v0.clone();
        length_bytes(a899)
    }, {
        let a900 = v0;
        length_bytes(a900)
    })?;
    Ok(h898)
}

pub fn f190(v0: Bytes, v1: u64) -> R<u64> {
    let (h901, _) = f233(Fn8::F235(v1, v0), v1, 0u64)?;
    Ok(h901)
}

pub fn f191(v0: Bytes) -> R<Bytes> {
    f236(v0.clone(), 0u64, {
        let a902 = v0;
        length_bytes(a902)
    }, false, false, Bytes::lit(&[]))
}

pub fn f192(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m903 = v2;
    if m903 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m903 - 1;
        let v5: u64 = f237(v0.clone(), v1)?;
        let a904 = v3;
        let a905 = 1u64;
        if nat_eq(a904, a905) {
            let m906 = f192(v0.clone(), v1, v4, 0u64)?;
            match m906 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f237(v0.clone(), {
                        let (_, h907) = v6.clone();
                        h907
                    })?;
                    let a908 = v7;
                    let a909 = 44u64;
                    if nat_eq(a908, a909) {
                        let m913 = f192(v0, {
                            let a911 = {
                                let (_, h910) = v6;
                                h910
                            };
                            let a912 = 1u64;
                            nat_add(a911, a912)?
                        }, v4, 1u64)?;
                        match m913 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h914, _) = v6;
                                    h914
                                }), std::rc::Rc::new({
                                    let (h915, _) = v8;
                                    h915
                                })), {
                                    let (_, h916) = v8;
                                    h916
                                })))
                            }
                        }
                    } else {
                        let a917 = v7;
                        let a918 = 93u64;
                        if nat_eq(a917, a918) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h919, _) = v6;
                                h919
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a921 = {
                                    let (_, h920) = v6;
                                    h920
                                };
                                let a922 = 1u64;
                                nat_add(a921, a922)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a923 = v3;
            let a924 = 2u64;
            if nat_eq(a923, a924) {
                let m927 = f238(v0.clone(), {
                    let a925 = v1;
                    let a926 = 1u64;
                    nat_add(a925, a926)?
                })?;
                match m927 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a928 = v5;
                        let a929 = 34u64;
                        if if nat_eq(a928, a929) {
                            let a931 = f237(v0.clone(), {
                                let (h930, _) = v9.clone();
                                h930
                            })?;
                            let a932 = 58u64;
                            nat_eq(a931, a932)
                        } else {
                            false
                        } {
                            let m936 = f192(v0.clone(), {
                                let a934 = {
                                    let (h933, _) = v9;
                                    h933
                                };
                                let a935 = 1u64;
                                nat_add(a934, a935)?
                            }, v4, 0u64)?;
                            match m936 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f237(v0.clone(), {
                                        let (_, h937) = v10.clone();
                                        h937
                                    })?;
                                    let a938 = v11;
                                    let a939 = 44u64;
                                    if nat_eq(a938, a939) {
                                        let m943 = f192(v0, {
                                            let a941 = {
                                                let (_, h940) = v10;
                                                h940
                                            };
                                            let a942 = 1u64;
                                            nat_add(a941, a942)?
                                        }, v4, 2u64)?;
                                        match m943 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h944) = v9;
                                                    h944
                                                }, std::rc::Rc::new({
                                                    let (h945, _) = v10;
                                                    h945
                                                }), std::rc::Rc::new({
                                                    let (h946, _) = v12;
                                                    h946
                                                })), {
                                                    let (_, h947) = v12;
                                                    h947
                                                })))
                                            }
                                        }
                                    } else {
                                        let a948 = v11;
                                        let a949 = 125u64;
                                        if nat_eq(a948, a949) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h950) = v9;
                                                h950
                                            }, std::rc::Rc::new({
                                                let (h951, _) = v10;
                                                h951
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a953 = {
                                                    let (_, h952) = v10;
                                                    h952
                                                };
                                                let a954 = 1u64;
                                                nat_add(a953, a954)?
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
                let a955 = v5;
                let a956 = 34u64;
                if nat_eq(a955, a956) {
                    let m959 = f238(v0, {
                        let a957 = v1;
                        let a958 = 1u64;
                        nat_add(a957, a958)?
                    })?;
                    match m959 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h960) = v13;
                                h960
                            }), {
                                let (h961, _) = v13;
                                h961
                            })))
                        }
                    }
                } else {
                    if f26(f40(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a962 = v1;
                            let a963 = 4u64;
                            nat_add(a962, a963)?
                        })))
                    } else {
                        if f26(f40(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a964 = v1;
                                let a965 = 5u64;
                                nat_add(a964, a965)?
                            })))
                        } else {
                            let a966 = v5;
                            let a967 = 91u64;
                            if nat_eq(a966, a967) {
                                let a970 = f237(v0.clone(), {
                                    let a968 = v1;
                                    let a969 = 1u64;
                                    nat_add(a968, a969)?
                                })?;
                                let a971 = 93u64;
                                if nat_eq(a970, a971) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a972 = v1;
                                        let a973 = 2u64;
                                        nat_add(a972, a973)?
                                    })))
                                } else {
                                    let m976 = f192(v0, {
                                        let a974 = v1;
                                        let a975 = 1u64;
                                        nat_add(a974, a975)?
                                    }, v4, 1u64)?;
                                    match m976 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h977, _) = v14;
                                                h977
                                            })), {
                                                let (_, h978) = v14;
                                                h978
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a979 = v5;
                                let a980 = 123u64;
                                if nat_eq(a979, a980) {
                                    let a983 = f237(v0.clone(), {
                                        let a981 = v1;
                                        let a982 = 1u64;
                                        nat_add(a981, a982)?
                                    })?;
                                    let a984 = 125u64;
                                    if nat_eq(a983, a984) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a985 = v1;
                                            let a986 = 2u64;
                                            nat_add(a985, a986)?
                                        })))
                                    } else {
                                        let m989 = f192(v0, {
                                            let a987 = v1;
                                            let a988 = 1u64;
                                            nat_add(a987, a988)?
                                        }, v4, 2u64)?;
                                        match m989 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h990, _) = v15;
                                                    h990
                                                })), {
                                                    let (_, h991) = v15;
                                                    h991
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m992 = f239(v0, v1)?;
                                    match m992 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h993) = v16;
                                                h993
                                            }, {
                                                let (h994, _) = v16;
                                                h994
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

pub fn f193(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h995, _) = v1.clone();
        h995
    };
    let v3: u64 = {
        let (h996, _) = {
            let (_, h997) = v1.clone();
            h997
        };
        h996
    };
    let v4: List<Bytes> = {
        let (_, h998) = {
            let (_, h999) = v1;
            h999
        };
        h998
    };
    let m1000 = f213(v0.clone(), v2);
    match m1000 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1001 = v5;
            let a1002 = 10u8;
            if equal(a1001, a1002) {
                Ok(Some(({
                    let a1003 = v2;
                    let a1004 = 1u64;
                    nat_add(a1003, a1004)?
                }, ({
                    let a1005 = v2;
                    let a1006 = 1u64;
                    nat_add(a1005, a1006)?
                }, List::cons(f240(f40(v0, v3, {
                    let a1007 = v2;
                    let a1008 = v3;
                    nat_sub(a1007, a1008)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1009 = v2;
                    let a1010 = 1u64;
                    nat_add(a1009, a1010)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f194(v0: Adt14, v1: Bytes) -> bool {
    f55({
        let a1012 = {
            let a1011 = f93(v0, v1);
            length_bytes(a1011)
        };
        let a1013 = 0u64;
        nat_eq(a1012, a1013)
    })
}

pub fn f195(v0: Adt14, v1: Bytes) -> bool {
    let m1014 = f95(v0, v1);
    match m1014 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f196(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1015 = v0;
    match m1015 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f93(v2, v1)
        }
    }
}

pub fn f197(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f140(f211(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f198(v0: u64) -> bool {
    let a1016 = v0;
    let a1017 = 95u64;
    if nat_eq(a1016, a1017) {
        true
    } else {
        let a1018 = v0;
        let a1019 = 45u64;
        if nat_eq(a1018, a1019) {
            true
        } else {
            let a1020 = v0;
            let a1021 = 32u64;
            if nat_eq(a1020, a1021) {
                true
            } else {
                let a1022 = v0;
                let a1023 = 46u64;
                if nat_eq(a1022, a1023) {
                    true
                } else {
                    let a1024 = v0;
                    let a1025 = 47u64;
                    nat_eq(a1024, a1025)
                }
            }
        }
    }
}

pub fn f199(v0: u64) -> bool {
    let a1026 = 97u64;
    let a1027 = v0;
    if nat_le(a1026, a1027) {
        let a1028 = v0;
        let a1029 = 122u64;
        nat_le(a1028, a1029)
    } else {
        false
    }
}

pub fn f200(v0: u64) -> bool {
    let a1030 = 65u64;
    let a1031 = v0;
    if nat_le(a1030, a1031) {
        let a1032 = v0;
        let a1033 = 90u64;
        nat_le(a1032, a1033)
    } else {
        false
    }
}

pub fn f201(v0: u64) -> bool {
    let a1034 = 48u64;
    let a1035 = v0;
    if nat_le(a1034, a1035) {
        let a1036 = v0;
        let a1037 = 57u64;
        nat_le(a1036, a1037)
    } else {
        false
    }
}

pub fn f202(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1038 = f95(v1.clone(), f8(Str::lit("enum")));
    match m1038 {
        None => {
            let m1040 = f244(v0.clone(), f156(f95(v1.clone(), f8(Str::lit("properties")))), f54(f92(f95(v1, f8(Str::lit("required"))))))?;
            match m1040 {
                Ok(v6) => {
                    Ok(f243(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1042 = {
                            let (h1041, _) = v6.clone();
                            h1041
                        };
                        match m1042.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f140({
                                    let (h1043, _) = v6;
                                    h1043
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1044) = v6;
                        h1044
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt17>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1039 = f241(f54(f150(v2)))?;
            match m1039 {
                Ok(v4) => {
                    Ok(f243(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f140(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt17>::nil()))
                }
                Err(v3) => {
                    Ok(f242(v3, v0))
                }
            }
        }
    }
}

pub fn f203(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1045 = v0;
    match m1045 {
        Adt14::C0(_) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C1(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C2(_) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C3(_) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C4(r1046) => {
            let v4 = (*r1046).clone();
            f203(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1047, r1048) => {
            let v2 = (*r1047).clone();
            let v3 = (*r1048).clone();
            List::cons((v1, v2), f203(v3))
        }
    }
}

pub fn f204(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1049 = 0u64;
        let a1050 = v1;
        nat_lt(a1049, a1050)
    } else {
        false
    } {
        let a1051 = Bytes::lit(&[0x2d]);
        let a1052 = f245(v1)?;
        Ok(append_bytes(a1051, a1052))
    } else {
        f245(v1)
    }
}

pub fn f205(v0: Bytes) -> R<Bytes> {
    let a1058 = {
        let a1056 = Bytes::lit(&[0x22]);
        let a1057 = {
            let (_, h1053) = {
                let (h1054, _) = f246(Fn9::F247(v0.clone()), {
                    let a1055 = v0;
                    length_bytes(a1055)
                }, (0u64, Bytes::lit(&[])))?;
                h1054
            };
            h1053
        };
        append_bytes(a1056, a1057)
    };
    let a1059 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1058, a1059))
}

pub fn f206(v0: List<Bytes>) -> Bytes {
    let a1062 = {
        let a1060 = Bytes::lit(&[0x5b]);
        let a1061 = f82(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1060, a1061)
    };
    let a1063 = Bytes::lit(&[0x5d]);
    append_bytes(a1062, a1063)
}

pub fn f207(v0: Adt14) -> R<List<Bytes>> {
    let m1064 = v0;
    match m1064 {
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
        Adt14::C6(r1065, r1066) => {
            let v7 = (*r1065).clone();
            let v8 = (*r1066).clone();
            Ok(List::cons(f158(v7)?, f207(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f208(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f248(Fn10::F249, List::<(Bytes, Bytes)>::nil(), f250(v0)?);
    let v2: List<Bytes> = f251(Fn11::F252, List::<Bytes>::nil(), v1)?;
    let a1069 = {
        let a1067 = Bytes::lit(&[0x7b]);
        let a1068 = f82(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1067, a1068)
    };
    let a1070 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1069, a1070))
}

pub fn f209(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1071 = v0;
    match m1071 {
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
        Adt14::C7(v9, r1072, r1073) => {
            let v10 = (*r1072).clone();
            let v11 = (*r1073).clone();
            Ok(List::cons((v9, f158(v10)?), f209(v11)?))
        }
    }
}

pub fn f210(v0: Bytes, v1: Bytes) -> bool {
    let m1078 = {
        let a1075 = v0;
        let a1076 = 0u64;
        let a1077 = {
            let a1074 = v1.clone();
            length_bytes(a1074)
        };
        slice_bytes(a1075, a1076, a1077)
    };
    match m1078 {
        None => {
            false
        }
        Some(v2) => {
            let a1079 = v2;
            let a1080 = v1;
            equal(a1079, a1080)
        }
    }
}

pub fn f211(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1081 = v0;
    match m1081.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f134(v1)?, f211(v2)?))
        }
    }
}

pub fn f212(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1082 = v0;
    match m1082.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f212(v3, false))
            } else {
                f212(v3, true)
            }
        }
    }
}

pub fn f213(v0: Bytes, v1: u64) -> Option<u8> {
    let a1083 = v0;
    let a1084 = v1;
    index_bytes(a1083, a1084)
}

pub fn f214(v0: Adt15, v1: Option<u64>) -> bool {
    let m1085 = v1;
    match m1085 {
        None => {
            true
        }
        Some(v2) => {
            f170(v2, match v0 {
                Adt15::C0(_, h1086, _, _) => {
                    h1086
                }
            })
        }
    }
}

pub fn f215(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1088 = match v0 {
            Adt12::C0(_, _, h1087, _, _) => {
                h1087
            }
        };
        length_list(a1088)
    })
}

pub fn f216(v0: Bytes) -> Option<Adt20> {
    if f26(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt20::C0)
    } else {
        if f26(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt20::C1)
        } else {
            if f26(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt20::C2)
            } else {
                None::<Adt20>
            }
        }
    }
}

pub fn f217(v0: Bytes, p1093: List<Bytes>, p1094: u64) -> R<Option<u64>> {
    let m1092 = (p1093, p1094);
    {
        let mut m1092 = m1092;
        loop {
            let step = {
                let (v1, v2) = m1092;
                let m1089 = v1;
                match m1089.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f26(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1090 = v2;
                                let a1091 = 1u64;
                                nat_add(a1090, a1091)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1092 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f218(v0: Option<Adt14>) -> Option<u64> {
    let m1095 = v0;
    match m1095 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1096 = v1;
            match m1096 {
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

pub fn f219(v0: Fn4, p1100: List<Bytes>, p1101: List<(Bytes, ())>) -> List<Bytes> {
    let m1099 = (p1100, p1101);
    {
        let mut m1099 = m1099;
        loop {
            let step = {
                let (v1, v2) = m1099;
                let m1097 = v2;
                match m1097.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1098: Fn4 = v0.clone();
                            c1098.apply(v1, v3)
                        }, v4))
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

pub fn f220(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1102, _) = v1;
        h1102
    }, v0)
}

pub fn f221(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f253(Fn12::F254, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f222(v0: Fn5, p1106: List<List<(Bytes, ())>>, p1107: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1105 = (p1106, p1107);
    {
        let mut m1105 = m1105;
        loop {
            let step = {
                let (v1, v2) = m1105;
                let m1103 = v2;
                match m1103.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1104: Fn5 = v0.clone();
                            c1104.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1105 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f224(v0: Fn6, p1112: u64, p1113: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1111 = (p1112, p1113);
    {
        let mut m1111 = m1111;
        loop {
            let step = {
                let (v1, v2) = m1111;
                let m1108 = v1;
                if m1108 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1108 - 1;
                    let m1110 = {
                        let c1109: Fn6 = v0.clone();
                        c1109.apply(v2.clone())?
                    };
                    match m1110 {
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
                Ok(next) => { m1111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f255(v0)
}

pub fn f226(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f256(Fn13::F257, List::<(Bytes, ())>::nil(), v0)
}

pub fn f227(v0: Fn7, p1117: List<(Bytes, ())>, p1118: List<Bytes>) -> List<(Bytes, ())> {
    let m1116 = (p1117, p1118);
    {
        let mut m1116 = m1116;
        loop {
            let step = {
                let (v1, v2) = m1116;
                let m1114 = v2;
                match m1114.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1115: Fn7 = v0.clone();
                            c1115.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f228(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f229(v0: u32) -> Bytes {
    let a1121 = {
        let a1119 = v0;
        let a1120 = 8u32;
        fixed_u32::bit_and(a1119, a1120)
    };
    let a1122 = 0u32;
    if equal(a1121, a1122) {
        let a1125 = {
            let a1123 = v0;
            let a1124 = 4u32;
            fixed_u32::bit_and(a1123, a1124)
        };
        let a1126 = 0u32;
        if equal(a1125, a1126) {
            let a1129 = {
                let a1127 = v0;
                let a1128 = 2u32;
                fixed_u32::bit_and(a1127, a1128)
            };
            let a1130 = 0u32;
            if equal(a1129, a1130) {
                let a1133 = {
                    let a1131 = v0;
                    let a1132 = 1u32;
                    fixed_u32::bit_and(a1131, a1132)
                };
                let a1134 = 0u32;
                if equal(a1133, a1134) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1137 = {
                    let a1135 = v0;
                    let a1136 = 1u32;
                    fixed_u32::bit_and(a1135, a1136)
                };
                let a1138 = 0u32;
                if equal(a1137, a1138) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1141 = {
                let a1139 = v0;
                let a1140 = 2u32;
                fixed_u32::bit_and(a1139, a1140)
            };
            let a1142 = 0u32;
            if equal(a1141, a1142) {
                let a1145 = {
                    let a1143 = v0;
                    let a1144 = 1u32;
                    fixed_u32::bit_and(a1143, a1144)
                };
                let a1146 = 0u32;
                if equal(a1145, a1146) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1149 = {
                    let a1147 = v0;
                    let a1148 = 1u32;
                    fixed_u32::bit_and(a1147, a1148)
                };
                let a1150 = 0u32;
                if equal(a1149, a1150) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1153 = {
            let a1151 = v0;
            let a1152 = 4u32;
            fixed_u32::bit_and(a1151, a1152)
        };
        let a1154 = 0u32;
        if equal(a1153, a1154) {
            let a1157 = {
                let a1155 = v0;
                let a1156 = 2u32;
                fixed_u32::bit_and(a1155, a1156)
            };
            let a1158 = 0u32;
            if equal(a1157, a1158) {
                let a1161 = {
                    let a1159 = v0;
                    let a1160 = 1u32;
                    fixed_u32::bit_and(a1159, a1160)
                };
                let a1162 = 0u32;
                if equal(a1161, a1162) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1165 = {
                    let a1163 = v0;
                    let a1164 = 1u32;
                    fixed_u32::bit_and(a1163, a1164)
                };
                let a1166 = 0u32;
                if equal(a1165, a1166) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1169 = {
                let a1167 = v0;
                let a1168 = 2u32;
                fixed_u32::bit_and(a1167, a1168)
            };
            let a1170 = 0u32;
            if equal(a1169, a1170) {
                let a1173 = {
                    let a1171 = v0;
                    let a1172 = 1u32;
                    fixed_u32::bit_and(a1171, a1172)
                };
                let a1174 = 0u32;
                if equal(a1173, a1174) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1177 = {
                    let a1175 = v0;
                    let a1176 = 1u32;
                    fixed_u32::bit_and(a1175, a1176)
                };
                let a1178 = 0u32;
                if equal(a1177, a1178) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f230(v0: u32, v1: u32) -> u32 {
    let m1181 = {
        let a1179 = v0;
        let a1180 = v1;
        fixed_u32::shift_right(a1179, a1180)
    };
    match m1181 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f231(v0: Adt21, v1: Adt22) -> R<Adt21> {
    let v2: Adt23 = f258(Fn14::F259, 64u64, Adt23::C0(match v0.clone() {
        Adt21::C0(h1182, _, _, _, _, _, _, _) => {
            h1182
        }
    }, match v0.clone() {
        Adt21::C0(_, h1183, _, _, _, _, _, _) => {
            h1183
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h1184, _, _, _, _, _) => {
            h1184
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, h1185, _, _, _, _) => {
            h1185
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, h1186, _, _, _) => {
            h1186
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1187, _, _) => {
            h1187
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1188, _) => {
            h1188
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, h1189) => {
            h1189
        }
    }, v1, 0u64))?;
    Ok(Adt21::C0(f260(match v0.clone() {
        Adt21::C0(h1190, _, _, _, _, _, _, _) => {
            h1190
        }
    }, match v2.clone() {
        Adt23::C0(h1191, _, _, _, _, _, _, _, _, _) => {
            h1191
        }
    }), f260(match v0.clone() {
        Adt21::C0(_, h1192, _, _, _, _, _, _) => {
            h1192
        }
    }, match v2.clone() {
        Adt23::C0(_, h1193, _, _, _, _, _, _, _, _) => {
            h1193
        }
    }), f260(match v0.clone() {
        Adt21::C0(_, _, h1194, _, _, _, _, _) => {
            h1194
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1195, _, _, _, _, _, _, _) => {
            h1195
        }
    }), f260(match v0.clone() {
        Adt21::C0(_, _, _, h1196, _, _, _, _) => {
            h1196
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1197, _, _, _, _, _, _) => {
            h1197
        }
    }), f260(match v0.clone() {
        Adt21::C0(_, _, _, _, h1198, _, _, _) => {
            h1198
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1199, _, _, _, _, _) => {
            h1199
        }
    }), f260(match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1200, _, _) => {
            h1200
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1201, _, _, _, _) => {
            h1201
        }
    }), f260(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1202, _) => {
            h1202
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1203, _, _, _) => {
            h1203
        }
    }), f260(match v0 {
        Adt21::C0(_, _, _, _, _, _, _, h1204) => {
            h1204
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1205, _, _) => {
            h1205
        }
    })))
}

pub fn f232(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f261(v0.clone(), v1, v2, v3)?, f261(v0.clone(), v1, v2, {
        let a1206 = v3;
        let a1207 = 4u64;
        nat_add(a1206, a1207)?
    })?, f261(v0.clone(), v1, v2, {
        let a1208 = v3;
        let a1209 = 8u64;
        nat_add(a1208, a1209)?
    })?, f261(v0.clone(), v1, v2, {
        let a1210 = v3;
        let a1211 = 12u64;
        nat_add(a1210, a1211)?
    })?, f261(v0.clone(), v1, v2, {
        let a1212 = v3;
        let a1213 = 16u64;
        nat_add(a1212, a1213)?
    })?, f261(v0.clone(), v1, v2, {
        let a1214 = v3;
        let a1215 = 20u64;
        nat_add(a1214, a1215)?
    })?, f261(v0.clone(), v1, v2, {
        let a1216 = v3;
        let a1217 = 24u64;
        nat_add(a1216, a1217)?
    })?, f261(v0.clone(), v1, v2, {
        let a1218 = v3;
        let a1219 = 28u64;
        nat_add(a1218, a1219)?
    })?, f261(v0.clone(), v1, v2, {
        let a1220 = v3;
        let a1221 = 32u64;
        nat_add(a1220, a1221)?
    })?, f261(v0.clone(), v1, v2, {
        let a1222 = v3;
        let a1223 = 36u64;
        nat_add(a1222, a1223)?
    })?, f261(v0.clone(), v1, v2, {
        let a1224 = v3;
        let a1225 = 40u64;
        nat_add(a1224, a1225)?
    })?, f261(v0.clone(), v1, v2, {
        let a1226 = v3;
        let a1227 = 44u64;
        nat_add(a1226, a1227)?
    })?, f261(v0.clone(), v1, v2, {
        let a1228 = v3;
        let a1229 = 48u64;
        nat_add(a1228, a1229)?
    })?, f261(v0.clone(), v1, v2, {
        let a1230 = v3;
        let a1231 = 52u64;
        nat_add(a1230, a1231)?
    })?, f261(v0.clone(), v1, v2, {
        let a1232 = v3;
        let a1233 = 56u64;
        nat_add(a1232, a1233)?
    })?, f261(v0, v1, v2, {
        let a1234 = v3;
        let a1235 = 60u64;
        nat_add(a1234, a1235)?
    })?))
}

pub fn f233(v0: Fn8, p1240: u64, p1241: u64) -> R<(u64, bool)> {
    let m1239 = (p1240, p1241);
    {
        let mut m1239 = m1239;
        loop {
            let step = {
                let (v1, v2) = m1239;
                let m1236 = v1;
                if m1236 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1236 - 1;
                    let m1238 = {
                        let c1237: Fn8 = v0.clone();
                        c1237.apply(v2)?
                    };
                    match m1238 {
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
                Ok(next) => { m1239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: Bytes, v1: u64) -> Option<u64> {
    let a1242 = 0u64;
    let a1243 = v1;
    if nat_lt(a1242, a1243) {
        if f262(v0, {
            let a1244 = v1;
            let a1245 = 1u64;
            nat_sub(a1244, a1245)
        }) {
            Some({
                let a1246 = v1;
                let a1247 = 1u64;
                nat_sub(a1246, a1247)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f235(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1248 = v2;
    let a1249 = v0;
    if nat_lt(a1248, a1249) {
        if f262(v1, v2) {
            Ok(Some({
                let a1250 = v2;
                let a1251 = 1u64;
                nat_add(a1250, a1251)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f236(v0: Bytes, p1331: u64, p1332: u64, p1333: bool, p1334: bool, p1335: Bytes) -> R<Bytes> {
    let m1330 = (p1331, (p1332, (p1333, (p1334, p1335))));
    {
        let mut m1330 = m1330;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1330;
                let m1252 = v2;
                if m1252 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1252 - 1;
                    let m1255 = {
                        let a1253 = v0.clone();
                        let a1254 = v1;
                        index_bytes(a1253, a1254)
                    };
                    match m1255 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1300 = {
                                    let a1276 = {
                                        let a1264 = {
                                            let a1258 = {
                                                let a1256 = v8;
                                                let a1257 = 128u8;
                                                fixed_u8::bit_and(a1256, a1257)
                                            };
                                            let a1259 = 128u8;
                                            if equal(a1258, a1259) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1265 = {
                                            let a1262 = {
                                                let a1260 = v8;
                                                let a1261 = 64u8;
                                                fixed_u8::bit_and(a1260, a1261)
                                            };
                                            let a1263 = 64u8;
                                            if equal(a1262, a1263) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1264, a1265)?
                                    };
                                    let a1277 = {
                                        let a1274 = {
                                            let a1268 = {
                                                let a1266 = v8;
                                                let a1267 = 32u8;
                                                fixed_u8::bit_and(a1266, a1267)
                                            };
                                            let a1269 = 32u8;
                                            if equal(a1268, a1269) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1275 = {
                                            let a1272 = {
                                                let a1270 = v8;
                                                let a1271 = 16u8;
                                                fixed_u8::bit_and(a1270, a1271)
                                            };
                                            let a1273 = 16u8;
                                            if equal(a1272, a1273) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1274, a1275)?
                                    };
                                    nat_add(a1276, a1277)?
                                };
                                let a1301 = {
                                    let a1298 = {
                                        let a1286 = {
                                            let a1280 = {
                                                let a1278 = v8;
                                                let a1279 = 8u8;
                                                fixed_u8::bit_and(a1278, a1279)
                                            };
                                            let a1281 = 8u8;
                                            if equal(a1280, a1281) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1287 = {
                                            let a1284 = {
                                                let a1282 = v8;
                                                let a1283 = 4u8;
                                                fixed_u8::bit_and(a1282, a1283)
                                            };
                                            let a1285 = 4u8;
                                            if equal(a1284, a1285) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1286, a1287)?
                                    };
                                    let a1299 = {
                                        let a1296 = {
                                            let a1290 = {
                                                let a1288 = v8;
                                                let a1289 = 2u8;
                                                fixed_u8::bit_and(a1288, a1289)
                                            };
                                            let a1291 = 2u8;
                                            if equal(a1290, a1291) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1297 = {
                                            let a1294 = {
                                                let a1292 = v8;
                                                let a1293 = 1u8;
                                                fixed_u8::bit_and(a1292, a1293)
                                            };
                                            let a1295 = 1u8;
                                            if equal(a1294, a1295) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1296, a1297)?
                                    };
                                    nat_add(a1298, a1299)?
                                };
                                nat_add(a1300, a1301)?
                            };
                            let v10: Bytes = f40(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1302 = v1;
                                    let a1303 = 1u64;
                                    nat_add(a1302, a1303)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1304 = v9;
                                    let a1305 = 34u64;
                                    !nat_eq(a1304, a1305)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1306 = v9;
                                    let a1307 = 92u64;
                                    nat_eq(a1306, a1307)
                                }, {
                                    let a1308 = v5;
                                    let a1309 = v10;
                                    append_bytes(a1308, a1309)
                                })))))
                            } else {
                                let a1310 = v9;
                                let a1311 = 32u64;
                                if if if if nat_eq(a1310, a1311) {
                                    true
                                } else {
                                    let a1312 = v9;
                                    let a1313 = 9u64;
                                    nat_eq(a1312, a1313)
                                } {
                                    true
                                } else {
                                    let a1314 = v9;
                                    let a1315 = 10u64;
                                    nat_eq(a1314, a1315)
                                } {
                                    true
                                } else {
                                    let a1316 = v9;
                                    let a1317 = 13u64;
                                    nat_eq(a1316, a1317)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1318 = v1;
                                        let a1319 = 1u64;
                                        nat_add(a1318, a1319)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f26(f40(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1320 = v1;
                                            let a1321 = 4u64;
                                            nat_add(a1320, a1321)?
                                        }, (v6, (false, (false, {
                                            let a1322 = v5;
                                            let a1323 = f8(Str::lit("[]"));
                                            append_bytes(a1322, a1323)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1324 = v1;
                                            let a1325 = 1u64;
                                            nat_add(a1324, a1325)?
                                        }, (v6, ({
                                            let a1326 = v9;
                                            let a1327 = 34u64;
                                            nat_eq(a1326, a1327)
                                        }, (false, {
                                            let a1328 = v5;
                                            let a1329 = v10;
                                            append_bytes(a1328, a1329)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Bytes, v1: u64) -> R<u64> {
    let m1338 = {
        let a1336 = v0;
        let a1337 = v1;
        index_bytes(a1336, a1337)
    };
    match m1338 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1383 = {
                let a1359 = {
                    let a1347 = {
                        let a1341 = {
                            let a1339 = v3;
                            let a1340 = 128u8;
                            fixed_u8::bit_and(a1339, a1340)
                        };
                        let a1342 = 128u8;
                        if equal(a1341, a1342) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1348 = {
                        let a1345 = {
                            let a1343 = v3;
                            let a1344 = 64u8;
                            fixed_u8::bit_and(a1343, a1344)
                        };
                        let a1346 = 64u8;
                        if equal(a1345, a1346) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1347, a1348)?
                };
                let a1360 = {
                    let a1357 = {
                        let a1351 = {
                            let a1349 = v3;
                            let a1350 = 32u8;
                            fixed_u8::bit_and(a1349, a1350)
                        };
                        let a1352 = 32u8;
                        if equal(a1351, a1352) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1358 = {
                        let a1355 = {
                            let a1353 = v3;
                            let a1354 = 16u8;
                            fixed_u8::bit_and(a1353, a1354)
                        };
                        let a1356 = 16u8;
                        if equal(a1355, a1356) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1357, a1358)?
                };
                nat_add(a1359, a1360)?
            };
            let a1384 = {
                let a1381 = {
                    let a1369 = {
                        let a1363 = {
                            let a1361 = v3;
                            let a1362 = 8u8;
                            fixed_u8::bit_and(a1361, a1362)
                        };
                        let a1364 = 8u8;
                        if equal(a1363, a1364) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1370 = {
                        let a1367 = {
                            let a1365 = v3;
                            let a1366 = 4u8;
                            fixed_u8::bit_and(a1365, a1366)
                        };
                        let a1368 = 4u8;
                        if equal(a1367, a1368) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1369, a1370)?
                };
                let a1382 = {
                    let a1379 = {
                        let a1373 = {
                            let a1371 = v3;
                            let a1372 = 2u8;
                            fixed_u8::bit_and(a1371, a1372)
                        };
                        let a1374 = 2u8;
                        if equal(a1373, a1374) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1380 = {
                        let a1377 = {
                            let a1375 = v3;
                            let a1376 = 1u8;
                            fixed_u8::bit_and(a1375, a1376)
                        };
                        let a1378 = 1u8;
                        if equal(a1377, a1378) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1379, a1380)?
                };
                nat_add(a1381, a1382)?
            };
            nat_add(a1383, a1384)
        }
    }
}

pub fn f238(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f263(Fn15::F264(v0.clone()), {
        let a1386 = {
            let a1385 = v0;
            length_bytes(a1385)
        };
        let a1387 = 1u64;
        nat_add(a1386, a1387)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1391 = {
        let (_, h1388) = {
            let (_, h1389) = {
                let (h1390, _) = v2.clone();
                h1390
            };
            h1389
        };
        h1388
    };
    let a1392 = 1u64;
    if nat_eq(a1391, a1392) {
        Ok(Some(({
            let (h1393, _) = {
                let (h1394, _) = v2.clone();
                h1394
            };
            h1393
        }, {
            let (h1395, _) = {
                let (_, h1396) = {
                    let (h1397, _) = v2;
                    h1397
                };
                h1396
            };
            h1395
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f239(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1398 = f237(v0.clone(), v1)?;
        let a1399 = 45u64;
        nat_eq(a1398, a1399)
    };
    let v3: u64 = if v2 {
        let a1400 = v1;
        let a1401 = 1u64;
        nat_add(a1400, a1401)?
    } else {
        v1
    };
    let v4: u64 = f265(v0.clone(), v3)?;
    let a1404 = 19u64;
    let a1405 = {
        let a1402 = v4;
        let a1403 = v3;
        nat_sub(a1402, a1403)
    };
    if nat_lt(a1404, a1405) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1408 = f266(f40(v0, v3, {
            let a1406 = v4;
            let a1407 = v3;
            nat_sub(a1406, a1407)
        }))?;
        match m1408 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f267(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f240(v0: Bytes) -> Bytes {
    if f112(v0.clone(), Bytes::lit(&[0x0d])) {
        f40(v0.clone(), 0u64, {
            let a1410 = {
                let a1409 = v0;
                length_bytes(a1409)
            };
            let a1411 = 1u64;
            nat_sub(a1410, a1411)
        })
    } else {
        v0
    }
}

pub fn f241(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1412 = v0;
    match m1412.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f268(v1.clone())?;
            if f269(v3.clone())? {
                let m1413 = f241(v2)?;
                match m1413 {
                    Ok(v5) => {
                        Ok(Ok::<List<Bytes>, Bytes>(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v3, List::cons(f8(Str::lit(", fields: []}")), List::<Bytes>::nil())))), v5)))
                    }
                    Err(v4) => {
                        Ok(Err::<List<Bytes>, Bytes>(v4))
                    }
                }
            } else {
                Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(v1, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f242(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt17>), Bytes> {
    Err::<(Bytes, List<Adt17>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f243(v0: Bytes, v1: List<Adt17>) -> Result<(Bytes, List<Adt17>), Bytes> {
    Ok::<(Bytes, List<Adt17>), Bytes>((v0, v1))
}

pub fn f244(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt17>), Bytes>> {
    let m1414 = v1;
    match m1414.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::<Bytes>::nil(), List::<Adt17>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f268({
                let (h1415, _) = v3.clone();
                h1415
            })?;
            if f269(v5.clone())? {
                let m1420 = f270({
                    let (_, h1416) = v3.clone();
                    h1416
                }, {
                    let a1418 = v0.clone();
                    let a1419 = f101({
                        let (h1417, _) = v3.clone();
                        h1417
                    })?;
                    append_bytes(a1418, a1419)
                })?;
                match m1420 {
                    Ok(v7) => {
                        let v8: Bytes = if f62({
                            let (h1421, _) = v3;
                            h1421
                        }, v2.clone()) {
                            let (h1422, _) = v7;
                            h1422
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1423, _) = v7;
                                h1423
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1424 = f244(v0, v4, v2)?;
                        match m1424 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1425, _) = v10;
                                    h1425
                                }), f154({
                                    let (_, h1426) = v7;
                                    h1426
                                }, {
                                    let (_, h1427) = v10;
                                    h1427
                                }))))
                            }
                            Err(v9) => {
                                Ok(Err::<(List<Bytes>, List<Adt17>), Bytes>(v9))
                            }
                        }
                    }
                    Err(v6) => {
                        Ok(Err::<(List<Bytes>, List<Adt17>), Bytes>(v6))
                    }
                }
            } else {
                Ok(Err::<(List<Bytes>, List<Adt17>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                    let (h1428, _) = v3;
                    h1428
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f245(v0: u64) -> R<Bytes> {
    f271(f272(), 10u64, v0)
}

pub fn f246(v0: Fn9, p1433: u64, p1434: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1432 = (p1433, p1434);
    {
        let mut m1432 = m1432;
        loop {
            let step = {
                let (v1, v2) = m1432;
                let m1429 = v1;
                if m1429 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1429 - 1;
                    let m1431 = {
                        let c1430: Fn9 = v0.clone();
                        c1430.apply(v2.clone())?
                    };
                    match m1431 {
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
                Ok(next) => { m1432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f247(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f273(v0, v1)
}

pub fn f248(v0: Fn10, p1438: List<(Bytes, Bytes)>, p1439: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1437 = (p1438, p1439);
    {
        let mut m1437 = m1437;
        loop {
            let step = {
                let (v1, v2) = m1437;
                let m1435 = v2;
                match m1435.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1436: Fn10 = v0.clone();
                            c1436.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f249(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f274(v0, v1)
}

pub fn f250(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f275(v0)
}

pub fn f251(v0: Fn11, p1443: List<Bytes>, p1444: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1442 = (p1443, p1444);
    {
        let mut m1442 = m1442;
        loop {
            let step = {
                let (v1, v2) = m1442;
                let m1440 = v2;
                match m1440.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1441: Fn11 = v0.clone();
                            c1441.apply(v1, v3)?
                        }, v4))
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

pub fn f252(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f276(v1)?, v0))
}

pub fn f253(v0: Fn12, p1448: List<List<(Bytes, ())>>, p1449: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1447 = (p1448, p1449);
    {
        let mut m1447 = m1447;
        loop {
            let step = {
                let (v1, v2) = m1447;
                let m1445 = v2;
                match m1445.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1446: Fn12 = v0.clone();
                            c1446.apply(v1, v3)
                        }, v4))
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

pub fn f254(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f255(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1451 = {
        let a1450 = v0.clone();
        length_list(a1450)
    };
    let a1452 = 1u64;
    if nat_le(a1451, a1452) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f277(v0)?))
    }
}

pub fn f256(v0: Fn13, p1456: List<(Bytes, ())>, p1457: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1455 = (p1456, p1457);
    {
        let mut m1455 = m1455;
        loop {
            let step = {
                let (v1, v2) = m1455;
                let m1453 = v2;
                match m1453.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1454: Fn13 = v0.clone();
                            c1454.apply(v1, v3)
                        }, v4))
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

pub fn f257(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f258(v0: Fn14, p1461: u64, p1462: Adt23) -> R<Adt23> {
    let m1460 = (p1461, p1462);
    {
        let mut m1460 = m1460;
        loop {
            let step = {
                let (v1, v2) = m1460;
                let m1458 = v1;
                if m1458 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1458 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1459: Fn14 = v0.clone();
                        c1459.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1460 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: Adt23) -> R<Adt23> {
    f278(v0)
}

pub fn f260(v0: u32, v1: u32) -> u32 {
    let m1465 = {
        let a1463 = f279(v0);
        let a1464 = f279(v1);
        fixed_u64::checked_add(a1463, a1464)
    };
    match m1465 {
        None => {
            0u32
        }
        Some(v2) => {
            f280(v2)
        }
    }
}

pub fn f261(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1468 = {
        let a1466 = v3;
        let a1467 = 8u64;
        nat_add(a1466, a1467)?
    };
    let a1469 = v2;
    if nat_eq(a1468, a1469) {
        Ok(f281({
            let a1472 = {
                let a1470 = v1;
                let a1471 = 8u64;
                nat_mul(a1470, a1471)?
            };
            let a1473 = 4294967296u64;
            let a1474 = 0u64;
            nat_quot(a1472, a1473, a1474)
        }))
    } else {
        let a1477 = {
            let a1475 = v3;
            let a1476 = 4u64;
            nat_add(a1475, a1476)?
        };
        let a1478 = v2;
        if nat_eq(a1477, a1478) {
            Ok(f281({
                let a1481 = {
                    let a1479 = v1;
                    let a1480 = 8u64;
                    nat_mul(a1479, a1480)?
                };
                let a1482 = 4294967296u64;
                let a1483 = 0u64;
                nat_rem(a1481, a1482, a1483)
            }))
        } else {
            Ok(f282(f283(v0.clone(), v1, v3), f283(v0.clone(), v1, {
                let a1484 = v3;
                let a1485 = 1u64;
                nat_add(a1484, a1485)?
            }), f283(v0.clone(), v1, {
                let a1486 = v3;
                let a1487 = 2u64;
                nat_add(a1486, a1487)?
            }), f283(v0, v1, {
                let a1488 = v3;
                let a1489 = 3u64;
                nat_add(a1488, a1489)?
            })))
        }
    }
}

pub fn f262(v0: Bytes, v1: u64) -> bool {
    let m1490 = f213(v0, v1);
    match m1490 {
        None => {
            false
        }
        Some(v2) => {
            f284(v2)
        }
    }
}

pub fn f263(v0: Fn15, p1495: u64, p1496: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1494 = (p1495, p1496);
    {
        let mut m1494 = m1494;
        loop {
            let step = {
                let (v1, v2) = m1494;
                let m1491 = v1;
                if m1491 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1491 - 1;
                    let m1493 = {
                        let c1492: Fn15 = v0.clone();
                        c1492.apply(v2.clone())?
                    };
                    match m1493 {
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
                Ok(next) => { m1494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f285(v0, v1)
}

pub fn f265(v0: Bytes, v1: u64) -> R<u64> {
    let (h1497, _) = f233(Fn8::F286(v0.clone()), {
        let a1499 = {
            let a1498 = v0;
            length_bytes(a1498)
        };
        let a1500 = 1u64;
        nat_add(a1499, a1500)?
    }, v1)?;
    Ok(h1497)
}

pub fn f266(v0: Bytes) -> R<Option<u64>> {
    if f287(v0.clone())? {
        Ok(Some({
            let (_, h1501) = {
                let (h1502, _) = f288(Fn16::F289(v0.clone()), {
                    let a1503 = v0;
                    length_bytes(a1503)
                }, (0u64, 0u64))?;
                h1502
            };
            h1501
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f267(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1504 = 0u64;
        let a1505 = v1;
        if nat_lt(a1504, a1505) {
            let a1506 = v1;
            let a1507 = 9223372036854775808u64;
            nat_le(a1506, a1507)
        } else {
            false
        }
    } else {
        let a1508 = v1;
        let a1509 = 9223372036854775807u64;
        nat_le(a1508, a1509)
    }
}

pub fn f268(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f152(v0.clone(), 0u64, {
        let a1510 = v0;
        length_bytes(a1510)
    }, false, Bytes::lit(&[]))?;
    let m1513 = {
        let a1511 = v1.clone();
        let a1512 = 0u64;
        index_bytes(a1511, a1512)
    };
    match m1513 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1558 = {
                    let a1534 = {
                        let a1522 = {
                            let a1516 = {
                                let a1514 = v3;
                                let a1515 = 128u8;
                                fixed_u8::bit_and(a1514, a1515)
                            };
                            let a1517 = 128u8;
                            if equal(a1516, a1517) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1523 = {
                            let a1520 = {
                                let a1518 = v3;
                                let a1519 = 64u8;
                                fixed_u8::bit_and(a1518, a1519)
                            };
                            let a1521 = 64u8;
                            if equal(a1520, a1521) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1522, a1523)?
                    };
                    let a1535 = {
                        let a1532 = {
                            let a1526 = {
                                let a1524 = v3;
                                let a1525 = 32u8;
                                fixed_u8::bit_and(a1524, a1525)
                            };
                            let a1527 = 32u8;
                            if equal(a1526, a1527) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1533 = {
                            let a1530 = {
                                let a1528 = v3;
                                let a1529 = 16u8;
                                fixed_u8::bit_and(a1528, a1529)
                            };
                            let a1531 = 16u8;
                            if equal(a1530, a1531) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1532, a1533)?
                    };
                    nat_add(a1534, a1535)?
                };
                let a1559 = {
                    let a1556 = {
                        let a1544 = {
                            let a1538 = {
                                let a1536 = v3;
                                let a1537 = 8u8;
                                fixed_u8::bit_and(a1536, a1537)
                            };
                            let a1539 = 8u8;
                            if equal(a1538, a1539) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1545 = {
                            let a1542 = {
                                let a1540 = v3;
                                let a1541 = 4u8;
                                fixed_u8::bit_and(a1540, a1541)
                            };
                            let a1543 = 4u8;
                            if equal(a1542, a1543) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1544, a1545)?
                    };
                    let a1557 = {
                        let a1554 = {
                            let a1548 = {
                                let a1546 = v3;
                                let a1547 = 2u8;
                                fixed_u8::bit_and(a1546, a1547)
                            };
                            let a1549 = 2u8;
                            if equal(a1548, a1549) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1555 = {
                            let a1552 = {
                                let a1550 = v3;
                                let a1551 = 1u8;
                                fixed_u8::bit_and(a1550, a1551)
                            };
                            let a1553 = 1u8;
                            if equal(a1552, a1553) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1554, a1555)?
                    };
                    nat_add(a1556, a1557)?
                };
                nat_add(a1558, a1559)?
            };
            if f200(v4) {
                let a1566 = f40({
                    let a1560 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a1560)
                }, {
                    let a1561 = v4;
                    let a1562 = 65u64;
                    nat_sub(a1561, a1562)
                }, 1u64);
                let a1567 = f40(v1.clone(), 1u64, {
                    let a1564 = {
                        let a1563 = v1;
                        length_bytes(a1563)
                    };
                    let a1565 = 1u64;
                    nat_sub(a1564, a1565)
                });
                Ok(append_bytes(a1566, a1567))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f269(v0: Bytes) -> R<bool> {
    let m1570 = {
        let a1568 = v0;
        let a1569 = 0u64;
        index_bytes(a1568, a1569)
    };
    match m1570 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f199({
                let v2: u8 = v1;
                let a1615 = {
                    let a1591 = {
                        let a1579 = {
                            let a1573 = {
                                let a1571 = v2;
                                let a1572 = 128u8;
                                fixed_u8::bit_and(a1571, a1572)
                            };
                            let a1574 = 128u8;
                            if equal(a1573, a1574) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1580 = {
                            let a1577 = {
                                let a1575 = v2;
                                let a1576 = 64u8;
                                fixed_u8::bit_and(a1575, a1576)
                            };
                            let a1578 = 64u8;
                            if equal(a1577, a1578) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1579, a1580)?
                    };
                    let a1592 = {
                        let a1589 = {
                            let a1583 = {
                                let a1581 = v2;
                                let a1582 = 32u8;
                                fixed_u8::bit_and(a1581, a1582)
                            };
                            let a1584 = 32u8;
                            if equal(a1583, a1584) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1590 = {
                            let a1587 = {
                                let a1585 = v2;
                                let a1586 = 16u8;
                                fixed_u8::bit_and(a1585, a1586)
                            };
                            let a1588 = 16u8;
                            if equal(a1587, a1588) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1589, a1590)?
                    };
                    nat_add(a1591, a1592)?
                };
                let a1616 = {
                    let a1613 = {
                        let a1601 = {
                            let a1595 = {
                                let a1593 = v2;
                                let a1594 = 8u8;
                                fixed_u8::bit_and(a1593, a1594)
                            };
                            let a1596 = 8u8;
                            if equal(a1595, a1596) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1602 = {
                            let a1599 = {
                                let a1597 = v2;
                                let a1598 = 4u8;
                                fixed_u8::bit_and(a1597, a1598)
                            };
                            let a1600 = 4u8;
                            if equal(a1599, a1600) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1601, a1602)?
                    };
                    let a1614 = {
                        let a1611 = {
                            let a1605 = {
                                let a1603 = v2;
                                let a1604 = 2u8;
                                fixed_u8::bit_and(a1603, a1604)
                            };
                            let a1606 = 2u8;
                            if equal(a1605, a1606) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1612 = {
                            let a1609 = {
                                let a1607 = v2;
                                let a1608 = 1u8;
                                fixed_u8::bit_and(a1607, a1608)
                            };
                            let a1610 = 1u8;
                            if equal(a1609, a1610) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1611, a1612)?
                    };
                    nat_add(a1613, a1614)?
                };
                nat_add(a1615, a1616)?
            }) {
                Ok(true)
            } else {
                Ok(f200({
                    let v3: u8 = v1;
                    let a1661 = {
                        let a1637 = {
                            let a1625 = {
                                let a1619 = {
                                    let a1617 = v3;
                                    let a1618 = 128u8;
                                    fixed_u8::bit_and(a1617, a1618)
                                };
                                let a1620 = 128u8;
                                if equal(a1619, a1620) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1626 = {
                                let a1623 = {
                                    let a1621 = v3;
                                    let a1622 = 64u8;
                                    fixed_u8::bit_and(a1621, a1622)
                                };
                                let a1624 = 64u8;
                                if equal(a1623, a1624) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1625, a1626)?
                        };
                        let a1638 = {
                            let a1635 = {
                                let a1629 = {
                                    let a1627 = v3;
                                    let a1628 = 32u8;
                                    fixed_u8::bit_and(a1627, a1628)
                                };
                                let a1630 = 32u8;
                                if equal(a1629, a1630) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1636 = {
                                let a1633 = {
                                    let a1631 = v3;
                                    let a1632 = 16u8;
                                    fixed_u8::bit_and(a1631, a1632)
                                };
                                let a1634 = 16u8;
                                if equal(a1633, a1634) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1635, a1636)?
                        };
                        nat_add(a1637, a1638)?
                    };
                    let a1662 = {
                        let a1659 = {
                            let a1647 = {
                                let a1641 = {
                                    let a1639 = v3;
                                    let a1640 = 8u8;
                                    fixed_u8::bit_and(a1639, a1640)
                                };
                                let a1642 = 8u8;
                                if equal(a1641, a1642) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1648 = {
                                let a1645 = {
                                    let a1643 = v3;
                                    let a1644 = 4u8;
                                    fixed_u8::bit_and(a1643, a1644)
                                };
                                let a1646 = 4u8;
                                if equal(a1645, a1646) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1647, a1648)?
                        };
                        let a1660 = {
                            let a1657 = {
                                let a1651 = {
                                    let a1649 = v3;
                                    let a1650 = 2u8;
                                    fixed_u8::bit_and(a1649, a1650)
                                };
                                let a1652 = 2u8;
                                if equal(a1651, a1652) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1658 = {
                                let a1655 = {
                                    let a1653 = v3;
                                    let a1654 = 1u8;
                                    fixed_u8::bit_and(a1653, a1654)
                                };
                                let a1656 = 1u8;
                                if equal(a1655, a1656) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1657, a1658)?
                        };
                        nat_add(a1659, a1660)?
                    };
                    nat_add(a1661, a1662)?
                }))
            }
        }
    }
}

pub fn f270(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1663 = f290(v0.clone());
    match m1663 {
        None => {
            if f26(f93(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1664 = f95(v0, f8(Str::lit("items")));
                match m1664 {
                    None => {
                        Ok(f242(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f291(f8(Str::lit("List")), f292(v3, {
                            let a1665 = v1;
                            let a1666 = f8(Str::lit("Item"));
                            append_bytes(a1665, a1666)
                        })?))
                    }
                }
            } else {
                f292(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f291(f8(Str::lit("Option")), f292(v2, v1)?))
        }
    }
}

pub fn f271(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1667 = v2;
    let a1668 = 0u64;
    if nat_eq(a1667, a1668) {
        Ok(f293(v0, 0u64))
    } else {
        let (_, h1669) = {
            let (h1670, _) = f246(Fn9::F294(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1670
        };
        Ok(h1669)
    }
}

pub fn f272() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f273(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1674 = {
        let a1672 = v0.clone();
        let a1673 = {
            let (h1671, _) = v1.clone();
            h1671
        };
        index_bytes(a1672, a1673)
    };
    match m1674 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1676 = {
                    let (h1675, _) = v1.clone();
                    h1675
                };
                let a1677 = 1u64;
                nat_add(a1676, a1677)?
            }, {
                let a1680 = {
                    let (_, h1678) = v1;
                    h1678
                };
                let a1681 = f295(v0, {
                    let (h1679, _) = v1;
                    h1679
                }, v2)?;
                append_bytes(a1680, a1681)
            })))
        }
    }
}

pub fn f274(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1682 = v0.clone();
    match m1682.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1685 = {
                let (h1683, _) = v2;
                h1683
            };
            let a1686 = {
                let (h1684, _) = v1.clone();
                h1684
            };
            if equal(a1685, a1686) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f275(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f296(f297(Fn17::F298, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1691 = {
        let (h1687, _) = f299(Fn18::F300, {
            let a1689 = {
                let a1688 = v0;
                length_list(a1688)
            };
            let a1690 = 1u64;
            nat_add(a1689, a1690)?
        }, v1)?;
        h1687
    };
    match m1691.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f276(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1696 = {
        let a1693 = f205({
            let (h1692, _) = v0;
            h1692
        })?;
        let a1694 = Bytes::lit(&[0x3a]);
        append_bytes(a1693, a1694)
    };
    let a1697 = {
        let (_, h1695) = v0;
        h1695
    };
    Ok(append_bytes(a1696, a1697))
}

pub fn f277(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f221({
        let (_, h1698) = {
            let (h1699, _) = f301(Fn19::F302, {
                let a1701 = {
                    let a1700 = v0.clone();
                    length_list(a1700)
                };
                let a1702 = 1u64;
                nat_add(a1701, a1702)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1699
        };
        h1698
    }))
}

pub fn f278(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f303(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h1703) => {
            h1703
        }
    });
    let v2: u32 = f260(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1704, _, _) => {
            h1704
        }
    }, f260(f304(match v0.clone() {
        Adt23::C0(_, _, _, _, h1705, _, _, _, _, _) => {
            h1705
        }
    }), f260(f305(match v0.clone() {
        Adt23::C0(_, _, _, _, h1706, _, _, _, _, _) => {
            h1706
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1707, _, _, _, _) => {
            h1707
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1708, _, _, _) => {
            h1708
        }
    }), f260(v1, {
        let m1711 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h1710, _) => {
                h1710
            }
        };
        match m1711 {
            Adt22::C0(h1709, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1709
            }
        }
    }))));
    let v3: u32 = f260(f306(match v0.clone() {
        Adt23::C0(h1712, _, _, _, _, _, _, _, _, _) => {
            h1712
        }
    }), f307(match v0.clone() {
        Adt23::C0(h1713, _, _, _, _, _, _, _, _, _) => {
            h1713
        }
    }, match v0.clone() {
        Adt23::C0(_, h1714, _, _, _, _, _, _, _, _) => {
            h1714
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1715, _, _, _, _, _, _, _) => {
            h1715
        }
    }));
    Ok(Adt23::C0(f260(v2, v3), match v0.clone() {
        Adt23::C0(h1716, _, _, _, _, _, _, _, _, _) => {
            h1716
        }
    }, match v0.clone() {
        Adt23::C0(_, h1717, _, _, _, _, _, _, _, _) => {
            h1717
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1718, _, _, _, _, _, _, _) => {
            h1718
        }
    }, f260(match v0.clone() {
        Adt23::C0(_, _, _, h1719, _, _, _, _, _, _) => {
            h1719
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h1720, _, _, _, _, _) => {
            h1720
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1721, _, _, _, _) => {
            h1721
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1722, _, _, _) => {
            h1722
        }
    }, f308(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h1723, _) => {
            h1723
        }
    }), {
        let a1725 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h1724) => {
                h1724
            }
        };
        let a1726 = 1u64;
        nat_add(a1725, a1726)?
    }))
}

pub fn f279(v0: u32) -> u64 {
    let m1728 = {
        let a1727 = v0;
        fixed_u64::convert(i128::from(a1727))
    };
    match m1728 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f280(v0: u64) -> u32 {
    let m1732 = {
        let a1731 = {
            let a1729 = v0;
            let a1730 = 4294967295u64;
            fixed_u64::bit_and(a1729, a1730)
        };
        fixed_u32::convert(i128::from(a1731))
    };
    match m1732 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f281(v0: u64) -> u32 {
    let (_, h1733) = {
        let (_, h1734) = f309(Fn20::F310, 32u64, (v0, (1u32, 0u32)));
        h1734
    };
    h1733
}

pub fn f282(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1739 = f311(v0, 24u32);
    let a1740 = {
        let a1737 = f311(v1, 16u32);
        let a1738 = {
            let a1735 = f311(v2, 8u32);
            let a1736 = v3;
            fixed_u32::bit_or(a1735, a1736)
        };
        fixed_u32::bit_or(a1737, a1738)
    };
    fixed_u32::bit_or(a1739, a1740)
}

pub fn f283(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1741 = v2;
    let a1742 = v1;
    if nat_lt(a1741, a1742) {
        let m1745 = {
            let a1743 = v0;
            let a1744 = v2;
            index_bytes(a1743, a1744)
        };
        match m1745 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1747 = {
                    let a1746 = v3;
                    fixed_u32::convert(i128::from(a1746))
                };
                match m1747 {
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
        let a1748 = v2;
        let a1749 = v1;
        if nat_eq(a1748, a1749) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f284(v0: u8) -> bool {
    let a1750 = v0;
    let a1751 = 32u8;
    if if if equal(a1750, a1751) {
        true
    } else {
        let a1752 = v0;
        let a1753 = 9u8;
        equal(a1752, a1753)
    } {
        true
    } else {
        let a1754 = v0;
        let a1755 = 10u8;
        equal(a1754, a1755)
    } {
        true
    } else {
        let a1756 = v0;
        let a1757 = 12u8;
        if equal(a1756, a1757) {
            true
        } else {
            let a1758 = v0;
            let a1759 = 13u8;
            equal(a1758, a1759)
        }
    }
}

pub fn f285(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1760, _) = v1.clone();
        h1760
    };
    let v3: Bytes = {
        let (h1761, _) = {
            let (_, h1762) = v1.clone();
            h1762
        };
        h1761
    };
    let a1765 = {
        let (_, h1763) = {
            let (_, h1764) = v1;
            h1764
        };
        h1763
    };
    let a1766 = 0u64;
    if nat_eq(a1765, a1766) {
        let v4: u64 = f237(v0.clone(), v2)?;
        let a1767 = v4;
        let a1768 = 34u64;
        if nat_eq(a1767, a1768) {
            Ok(Some(({
                let a1769 = v2;
                let a1770 = 1u64;
                nat_add(a1769, a1770)?
            }, (v3, 1u64))))
        } else {
            let a1771 = v4;
            let a1772 = 92u64;
            if nat_eq(a1771, a1772) {
                f312(v0, v2, v3)
            } else {
                let a1773 = v4;
                let a1774 = 32u64;
                if if nat_lt(a1773, a1774) {
                    true
                } else {
                    let a1775 = v4;
                    let a1776 = 256u64;
                    nat_eq(a1775, a1776)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1777 = v2;
                        let a1778 = 1u64;
                        nat_add(a1777, a1778)?
                    }, ({
                        let a1779 = v3;
                        let a1780 = f40(v0, v2, 1u64);
                        append_bytes(a1779, a1780)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f286(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f313(v0, v1)
}

pub fn f287(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1781 = v0.clone();
        length_bytes(a1781)
    };
    let a1782 = 0u64;
    let a1783 = v1;
    if nat_lt(a1782, a1783) {
        let a1784 = f314(v0.clone())?;
        let a1785 = v1;
        if nat_eq(a1784, a1785) {
            let a1786 = 1u64;
            let a1787 = v1;
            if nat_lt(a1786, a1787) {
                let m1790 = {
                    let a1788 = v0;
                    let a1789 = 0u64;
                    index_bytes(a1788, a1789)
                };
                match m1790 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1791 = v2;
                        let a1792 = 48u8;
                        Ok(!equal(a1791, a1792))
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

pub fn f288(v0: Fn16, p1797: u64, p1798: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1796 = (p1797, p1798);
    {
        let mut m1796 = m1796;
        loop {
            let step = {
                let (v1, v2) = m1796;
                let m1793 = v1;
                if m1793 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1793 - 1;
                    let m1795 = {
                        let c1794: Fn16 = v0.clone();
                        c1794.apply(v2)?
                    };
                    match m1795 {
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
                Ok(next) => { m1796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f289(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f315(v0, v1)
}

pub fn f290(v0: Adt14) -> Option<Adt14> {
    let m1799 = f92(f95(v0, f8(Str::lit("oneOf"))));
    match m1799.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m1800 = v2;
            match m1800.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m1801 = v4;
                    match m1801.uncons() {
                        None => {
                            if f316(v3.clone()) {
                                Some(v1)
                            } else {
                                if f316(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt14>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt14>
                        }
                    }
                }
            }
        }
    }
}

pub fn f291(v0: Bytes, v1: Result<(Bytes, List<Adt17>), Bytes>) -> Result<(Bytes, List<Adt17>), Bytes> {
    let m1802 = v1;
    match m1802 {
        Ok(v3) => {
            f243(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h1803, _) = v3;
                h1803
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h1804) = v3;
                h1804
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt17>), Bytes>(v2)
        }
    }
}

pub fn f292(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1805 = f290(v0.clone());
    match m1805 {
        None => {
            if f26(f93(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1806 = f95(v0, f8(Str::lit("items")));
                match m1806 {
                    None => {
                        Ok(f242(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f291(f8(Str::lit("List")), f317(v3, {
                            let a1807 = v1;
                            let a1808 = f8(Str::lit("Item"));
                            append_bytes(a1807, a1808)
                        })?))
                    }
                }
            } else {
                f317(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f291(f8(Str::lit("Option")), f317(v2, v1)?))
        }
    }
}

pub fn f293(v0: Bytes, v1: u64) -> Bytes {
    let m1812 = {
        let a1809 = v0;
        let a1810 = v1;
        let a1811 = 1u64;
        slice_bytes(a1809, a1810, a1811)
    };
    match m1812 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f294(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f318(v1, v0, v2)
}

pub fn f295(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a1857 = {
            let a1833 = {
                let a1821 = {
                    let a1815 = {
                        let a1813 = v3;
                        let a1814 = 128u8;
                        fixed_u8::bit_and(a1813, a1814)
                    };
                    let a1816 = 128u8;
                    if equal(a1815, a1816) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1822 = {
                    let a1819 = {
                        let a1817 = v3;
                        let a1818 = 64u8;
                        fixed_u8::bit_and(a1817, a1818)
                    };
                    let a1820 = 64u8;
                    if equal(a1819, a1820) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1821, a1822)?
            };
            let a1834 = {
                let a1831 = {
                    let a1825 = {
                        let a1823 = v3;
                        let a1824 = 32u8;
                        fixed_u8::bit_and(a1823, a1824)
                    };
                    let a1826 = 32u8;
                    if equal(a1825, a1826) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1832 = {
                    let a1829 = {
                        let a1827 = v3;
                        let a1828 = 16u8;
                        fixed_u8::bit_and(a1827, a1828)
                    };
                    let a1830 = 16u8;
                    if equal(a1829, a1830) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1831, a1832)?
            };
            nat_add(a1833, a1834)?
        };
        let a1858 = {
            let a1855 = {
                let a1843 = {
                    let a1837 = {
                        let a1835 = v3;
                        let a1836 = 8u8;
                        fixed_u8::bit_and(a1835, a1836)
                    };
                    let a1838 = 8u8;
                    if equal(a1837, a1838) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1844 = {
                    let a1841 = {
                        let a1839 = v3;
                        let a1840 = 4u8;
                        fixed_u8::bit_and(a1839, a1840)
                    };
                    let a1842 = 4u8;
                    if equal(a1841, a1842) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1843, a1844)?
            };
            let a1856 = {
                let a1853 = {
                    let a1847 = {
                        let a1845 = v3;
                        let a1846 = 2u8;
                        fixed_u8::bit_and(a1845, a1846)
                    };
                    let a1848 = 2u8;
                    if equal(a1847, a1848) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1854 = {
                    let a1851 = {
                        let a1849 = v3;
                        let a1850 = 1u8;
                        fixed_u8::bit_and(a1849, a1850)
                    };
                    let a1852 = 1u8;
                    if equal(a1851, a1852) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1853, a1854)?
            };
            nat_add(a1855, a1856)?
        };
        nat_add(a1857, a1858)?
    };
    let a1859 = v4;
    let a1860 = 34u64;
    if nat_eq(a1859, a1860) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a1861 = v4;
        let a1862 = 92u64;
        if nat_eq(a1861, a1862) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a1863 = v4;
            let a1864 = 8u64;
            if nat_eq(a1863, a1864) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a1865 = v4;
                let a1866 = 12u64;
                if nat_eq(a1865, a1866) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a1867 = v4;
                    let a1868 = 10u64;
                    if nat_eq(a1867, a1868) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a1869 = v4;
                        let a1870 = 13u64;
                        if nat_eq(a1869, a1870) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a1871 = v4;
                            let a1872 = 9u64;
                            if nat_eq(a1871, a1872) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a1873 = v4;
                                let a1874 = 32u64;
                                if nat_lt(a1873, a1874) {
                                    let a1875 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a1876 = f319(v2)?;
                                    Ok(append_bytes(a1875, a1876))
                                } else {
                                    Ok(f40(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f296(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f320(Fn21::F321, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f297(v0: Fn17, p1880: List<List<(Bytes, Bytes)>>, p1881: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m1879 = (p1880, p1881);
    {
        let mut m1879 = m1879;
        loop {
            let step = {
                let (v1, v2) = m1879;
                let m1877 = v2;
                match m1877.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c1878: Fn17 = v0.clone();
                            c1878.apply(v1, v3)
                        }, v4))
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

pub fn f298(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f299(v0: Fn18, p1886: u64, p1887: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m1885 = (p1886, p1887);
    {
        let mut m1885 = m1885;
        loop {
            let step = {
                let (v1, v2) = m1885;
                let m1882 = v1;
                if m1882 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1882 - 1;
                    let m1884 = {
                        let c1883: Fn18 = v0.clone();
                        c1883.apply(v2.clone())?
                    };
                    match m1884 {
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
                Ok(next) => { m1885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f322(v0)
}

pub fn f301(v0: Fn19, p1892: u64, p1893: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m1891 = (p1892, p1893);
    {
        let mut m1891 = m1891;
        loop {
            let step = {
                let (v1, v2) = m1891;
                let m1888 = v1;
                if m1888 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1888 - 1;
                    let m1890 = {
                        let c1889: Fn19 = v0.clone();
                        c1889.apply(v2.clone())?
                    };
                    match m1890 {
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
                Ok(next) => { m1891 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f323(v0)
}

pub fn f303(v0: u64) -> u32 {
    let a1894 = v0;
    let a1895 = 32u64;
    if nat_lt(a1894, a1895) {
        let a1896 = v0;
        let a1897 = 16u64;
        if nat_lt(a1896, a1897) {
            let a1898 = v0;
            let a1899 = 8u64;
            if nat_lt(a1898, a1899) {
                let a1900 = v0;
                let a1901 = 4u64;
                if nat_lt(a1900, a1901) {
                    let a1902 = v0;
                    let a1903 = 2u64;
                    if nat_lt(a1902, a1903) {
                        let a1904 = v0;
                        let a1905 = 1u64;
                        if nat_lt(a1904, a1905) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1906 = v0;
                        let a1907 = 3u64;
                        if nat_lt(a1906, a1907) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1908 = v0;
                    let a1909 = 6u64;
                    if nat_lt(a1908, a1909) {
                        let a1910 = v0;
                        let a1911 = 5u64;
                        if nat_lt(a1910, a1911) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1912 = v0;
                        let a1913 = 7u64;
                        if nat_lt(a1912, a1913) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1914 = v0;
                let a1915 = 12u64;
                if nat_lt(a1914, a1915) {
                    let a1916 = v0;
                    let a1917 = 10u64;
                    if nat_lt(a1916, a1917) {
                        let a1918 = v0;
                        let a1919 = 9u64;
                        if nat_lt(a1918, a1919) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1920 = v0;
                        let a1921 = 11u64;
                        if nat_lt(a1920, a1921) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1922 = v0;
                    let a1923 = 14u64;
                    if nat_lt(a1922, a1923) {
                        let a1924 = v0;
                        let a1925 = 13u64;
                        if nat_lt(a1924, a1925) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1926 = v0;
                        let a1927 = 15u64;
                        if nat_lt(a1926, a1927) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1928 = v0;
            let a1929 = 24u64;
            if nat_lt(a1928, a1929) {
                let a1930 = v0;
                let a1931 = 20u64;
                if nat_lt(a1930, a1931) {
                    let a1932 = v0;
                    let a1933 = 18u64;
                    if nat_lt(a1932, a1933) {
                        let a1934 = v0;
                        let a1935 = 17u64;
                        if nat_lt(a1934, a1935) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1936 = v0;
                        let a1937 = 19u64;
                        if nat_lt(a1936, a1937) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1938 = v0;
                    let a1939 = 22u64;
                    if nat_lt(a1938, a1939) {
                        let a1940 = v0;
                        let a1941 = 21u64;
                        if nat_lt(a1940, a1941) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1942 = v0;
                        let a1943 = 23u64;
                        if nat_lt(a1942, a1943) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1944 = v0;
                let a1945 = 28u64;
                if nat_lt(a1944, a1945) {
                    let a1946 = v0;
                    let a1947 = 26u64;
                    if nat_lt(a1946, a1947) {
                        let a1948 = v0;
                        let a1949 = 25u64;
                        if nat_lt(a1948, a1949) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1950 = v0;
                        let a1951 = 27u64;
                        if nat_lt(a1950, a1951) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1952 = v0;
                    let a1953 = 30u64;
                    if nat_lt(a1952, a1953) {
                        let a1954 = v0;
                        let a1955 = 29u64;
                        if nat_lt(a1954, a1955) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1956 = v0;
                        let a1957 = 31u64;
                        if nat_lt(a1956, a1957) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1958 = v0;
        let a1959 = 48u64;
        if nat_lt(a1958, a1959) {
            let a1960 = v0;
            let a1961 = 40u64;
            if nat_lt(a1960, a1961) {
                let a1962 = v0;
                let a1963 = 36u64;
                if nat_lt(a1962, a1963) {
                    let a1964 = v0;
                    let a1965 = 34u64;
                    if nat_lt(a1964, a1965) {
                        let a1966 = v0;
                        let a1967 = 33u64;
                        if nat_lt(a1966, a1967) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1968 = v0;
                        let a1969 = 35u64;
                        if nat_lt(a1968, a1969) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1970 = v0;
                    let a1971 = 38u64;
                    if nat_lt(a1970, a1971) {
                        let a1972 = v0;
                        let a1973 = 37u64;
                        if nat_lt(a1972, a1973) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1974 = v0;
                        let a1975 = 39u64;
                        if nat_lt(a1974, a1975) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1976 = v0;
                let a1977 = 44u64;
                if nat_lt(a1976, a1977) {
                    let a1978 = v0;
                    let a1979 = 42u64;
                    if nat_lt(a1978, a1979) {
                        let a1980 = v0;
                        let a1981 = 41u64;
                        if nat_lt(a1980, a1981) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1982 = v0;
                        let a1983 = 43u64;
                        if nat_lt(a1982, a1983) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1984 = v0;
                    let a1985 = 46u64;
                    if nat_lt(a1984, a1985) {
                        let a1986 = v0;
                        let a1987 = 45u64;
                        if nat_lt(a1986, a1987) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1988 = v0;
                        let a1989 = 47u64;
                        if nat_lt(a1988, a1989) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1990 = v0;
            let a1991 = 56u64;
            if nat_lt(a1990, a1991) {
                let a1992 = v0;
                let a1993 = 52u64;
                if nat_lt(a1992, a1993) {
                    let a1994 = v0;
                    let a1995 = 50u64;
                    if nat_lt(a1994, a1995) {
                        let a1996 = v0;
                        let a1997 = 49u64;
                        if nat_lt(a1996, a1997) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1998 = v0;
                        let a1999 = 51u64;
                        if nat_lt(a1998, a1999) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2000 = v0;
                    let a2001 = 54u64;
                    if nat_lt(a2000, a2001) {
                        let a2002 = v0;
                        let a2003 = 53u64;
                        if nat_lt(a2002, a2003) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2004 = v0;
                        let a2005 = 55u64;
                        if nat_lt(a2004, a2005) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2006 = v0;
                let a2007 = 60u64;
                if nat_lt(a2006, a2007) {
                    let a2008 = v0;
                    let a2009 = 58u64;
                    if nat_lt(a2008, a2009) {
                        let a2010 = v0;
                        let a2011 = 57u64;
                        if nat_lt(a2010, a2011) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2012 = v0;
                        let a2013 = 59u64;
                        if nat_lt(a2012, a2013) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2014 = v0;
                    let a2015 = 62u64;
                    if nat_lt(a2014, a2015) {
                        let a2016 = v0;
                        let a2017 = 61u64;
                        if nat_lt(a2016, a2017) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2018 = v0;
                        let a2019 = 63u64;
                        if nat_lt(a2018, a2019) {
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

pub fn f304(v0: u32) -> u32 {
    let a2022 = {
        let a2020 = f324(v0, 6u32, 26u32);
        let a2021 = f324(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2020, a2021)
    };
    let a2023 = f324(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2022, a2023)
}

pub fn f305(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2029 = {
        let a2024 = v0;
        let a2025 = v1;
        fixed_u32::bit_and(a2024, a2025)
    };
    let a2030 = {
        let a2027 = {
            let a2026 = v0;
            fixed_u32::bit_not(a2026)
        };
        let a2028 = v2;
        fixed_u32::bit_and(a2027, a2028)
    };
    fixed_u32::bit_xor(a2029, a2030)
}

pub fn f306(v0: u32) -> u32 {
    let a2033 = {
        let a2031 = f324(v0, 2u32, 30u32);
        let a2032 = f324(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2031, a2032)
    };
    let a2034 = f324(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2033, a2034)
}

pub fn f307(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2043 = {
        let a2039 = {
            let a2035 = v0;
            let a2036 = v1;
            fixed_u32::bit_and(a2035, a2036)
        };
        let a2040 = {
            let a2037 = v0;
            let a2038 = v2;
            fixed_u32::bit_and(a2037, a2038)
        };
        fixed_u32::bit_xor(a2039, a2040)
    };
    let a2044 = {
        let a2041 = v1;
        let a2042 = v2;
        fixed_u32::bit_and(a2041, a2042)
    };
    fixed_u32::bit_xor(a2043, a2044)
}

pub fn f308(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h2045, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2045
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2046, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2046
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h2047, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2047
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h2048, _, _, _, _, _, _, _, _, _, _, _) => {
            h2048
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2049, _, _, _, _, _, _, _, _, _, _) => {
            h2049
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2050, _, _, _, _, _, _, _, _, _) => {
            h2050
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2051, _, _, _, _, _, _, _, _) => {
            h2051
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2052, _, _, _, _, _, _, _) => {
            h2052
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2053, _, _, _, _, _, _) => {
            h2053
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h2054, _, _, _, _, _) => {
            h2054
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h2055, _, _, _, _) => {
            h2055
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2056, _, _, _) => {
            h2056
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2057, _, _) => {
            h2057
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2058, _) => {
            h2058
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2059) => {
            h2059
        }
    }, f260(f325(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2060, _) => {
            h2060
        }
    }), f260(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2061, _, _, _, _, _, _) => {
            h2061
        }
    }, f260(f326(match v0.clone() {
        Adt22::C0(_, h2062, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2062
        }
    }), match v0 {
        Adt22::C0(h2063, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2063
        }
    }))))
}

pub fn f309(v0: Fn20, p2067: u64, p2068: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2066 = (p2067, p2068);
    {
        let mut m2066 = m2066;
        loop {
            let step = {
                let (v1, v2) = m2066;
                let m2064 = v1;
                if m2064 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2064 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2065: Fn20 = v0.clone();
                        c2065.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2069, _) = v0;
        h2069
    };
    let v2: u32 = {
        let (h2070, _) = {
            let (_, h2071) = v0;
            h2071
        };
        h2070
    };
    let v3: u32 = {
        let (_, h2072) = {
            let (_, h2073) = v0;
            h2073
        };
        h2072
    };
    ({
        let a2074 = v1;
        let a2075 = 2u64;
        let a2076 = 0u64;
        nat_quot(a2074, a2075, a2076)
    }, (f311(v2, 1u32), {
        let a2080 = {
            let a2077 = v1;
            let a2078 = 2u64;
            let a2079 = 0u64;
            nat_rem(a2077, a2078, a2079)
        };
        let a2081 = 1u64;
        if nat_eq(a2080, a2081) {
            let a2082 = v3;
            let a2083 = v2;
            fixed_u32::bit_or(a2082, a2083)
        } else {
            v3
        }
    }))
}

pub fn f311(v0: u32, v1: u32) -> u32 {
    let m2086 = {
        let a2084 = v0;
        let a2085 = v1;
        fixed_u32::shift_left(a2084, a2085)
    };
    match m2086 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f312(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f237(v0.clone(), {
        let a2087 = v1;
        let a2088 = 1u64;
        nat_add(a2087, a2088)?
    })?;
    let v4: Bytes = {
        let a2089 = v3;
        let a2090 = 117u64;
        if nat_eq(a2089, a2090) {
            f327(v0, v1)?
        } else {
            f328(v3)
        }
    };
    let a2091 = v4.clone();
    let a2092 = Bytes::lit(&[]);
    if equal(a2091, a2092) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2093 = v3;
            let a2094 = 117u64;
            if nat_eq(a2093, a2094) {
                let a2095 = v1;
                let a2096 = 6u64;
                nat_add(a2095, a2096)?
            } else {
                let a2097 = v1;
                let a2098 = 2u64;
                nat_add(a2097, a2098)?
            }
        }, ({
            let a2099 = v2;
            let a2100 = v4;
            append_bytes(a2099, a2100)
        }, 0u64))))
    }
}

pub fn f313(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f237(v0, v1)?;
    let a2101 = 47u64;
    let a2102 = v2;
    if if nat_lt(a2101, a2102) {
        let a2103 = v2;
        let a2104 = 58u64;
        nat_lt(a2103, a2104)
    } else {
        false
    } {
        Ok(Some({
            let a2105 = v1;
            let a2106 = 1u64;
            nat_add(a2105, a2106)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f314(v0: Bytes) -> R<u64> {
    let (h2107, _) = f233(Fn8::F329(v0.clone()), {
        let a2108 = v0;
        length_bytes(a2108)
    }, 0u64)?;
    Ok(h2107)
}

pub fn f315(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2112 = {
        let a2110 = v0;
        let a2111 = {
            let (h2109, _) = v1;
            h2109
        };
        index_bytes(a2110, a2111)
    };
    match m2112 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2114 = {
                    let (h2113, _) = v1;
                    h2113
                };
                let a2115 = 1u64;
                nat_add(a2114, a2115)?
            }, {
                let a2119 = {
                    let a2117 = {
                        let (_, h2116) = v1;
                        h2116
                    };
                    let a2118 = 10u64;
                    nat_mul(a2117, a2118)?
                };
                let a2120 = f330(v2)?;
                nat_add(a2119, a2120)?
            })))
        }
    }
}

pub fn f316(v0: Adt14) -> bool {
    f26(f93(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f317(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m2121 = f331(f203(v0.clone()));
    match m2121 {
        None => {
            let m2122 = f96(f95(v0.clone(), f8(Str::lit("$ref"))));
            match m2122 {
                None => {
                    let m2124 = f95(v0.clone(), f8(Str::lit("enum")));
                    match m2124 {
                        None => {
                            let m2125 = f95(v0.clone(), f8(Str::lit("const")));
                            match m2125 {
                                None => {
                                    let v8: Bytes = f93(v0.clone(), f8(Str::lit("type")));
                                    if f26(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f243(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                    } else {
                                        if f26(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f243(f333(v0), List::<Adt17>::nil()))
                                        } else {
                                            if f26(v8.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f243(f8(Str::lit("Bool")), List::<Adt17>::nil()))
                                            } else {
                                                if f26(v8.clone(), f8(Str::lit("object"))) {
                                                    Ok(f243(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                                                } else {
                                                    Ok(f242(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2126 = f96(Some(v6));
                                    match m2126 {
                                        None => {
                                            Ok(f242(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f243(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f243(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2123 = f332(v3)?;
                    match m2123 {
                        None => {
                            Ok(f242(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f243(v4, List::<Adt17>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f242(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f318(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2127, _) = v2;
        h2127
    };
    let a2128 = v3;
    let a2129 = 0u64;
    if nat_eq(a2128, a2129) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2130 = v3;
            let a2131 = v1;
            let a2132 = 0u64;
            nat_quot(a2130, a2131, a2132)
        }, {
            let a2137 = f293(v0, {
                let a2133 = v3;
                let a2134 = v1;
                let a2135 = 0u64;
                nat_rem(a2133, a2134, a2135)
            });
            let a2138 = {
                let (_, h2136) = v2;
                h2136
            };
            append_bytes(a2137, a2138)
        }))
    }
}

pub fn f319(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2183 = {
            let a2159 = {
                let a2147 = {
                    let a2141 = {
                        let a2139 = v1;
                        let a2140 = 128u8;
                        fixed_u8::bit_and(a2139, a2140)
                    };
                    let a2142 = 128u8;
                    if equal(a2141, a2142) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2148 = {
                    let a2145 = {
                        let a2143 = v1;
                        let a2144 = 64u8;
                        fixed_u8::bit_and(a2143, a2144)
                    };
                    let a2146 = 64u8;
                    if equal(a2145, a2146) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2147, a2148)?
            };
            let a2160 = {
                let a2157 = {
                    let a2151 = {
                        let a2149 = v1;
                        let a2150 = 32u8;
                        fixed_u8::bit_and(a2149, a2150)
                    };
                    let a2152 = 32u8;
                    if equal(a2151, a2152) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2158 = {
                    let a2155 = {
                        let a2153 = v1;
                        let a2154 = 16u8;
                        fixed_u8::bit_and(a2153, a2154)
                    };
                    let a2156 = 16u8;
                    if equal(a2155, a2156) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2157, a2158)?
            };
            nat_add(a2159, a2160)?
        };
        let a2184 = {
            let a2181 = {
                let a2169 = {
                    let a2163 = {
                        let a2161 = v1;
                        let a2162 = 8u8;
                        fixed_u8::bit_and(a2161, a2162)
                    };
                    let a2164 = 8u8;
                    if equal(a2163, a2164) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2170 = {
                    let a2167 = {
                        let a2165 = v1;
                        let a2166 = 4u8;
                        fixed_u8::bit_and(a2165, a2166)
                    };
                    let a2168 = 4u8;
                    if equal(a2167, a2168) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2169, a2170)?
            };
            let a2182 = {
                let a2179 = {
                    let a2173 = {
                        let a2171 = v1;
                        let a2172 = 2u8;
                        fixed_u8::bit_and(a2171, a2172)
                    };
                    let a2174 = 2u8;
                    if equal(a2173, a2174) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2180 = {
                    let a2177 = {
                        let a2175 = v1;
                        let a2176 = 1u8;
                        fixed_u8::bit_and(a2175, a2176)
                    };
                    let a2178 = 1u8;
                    if equal(a2177, a2178) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2179, a2180)?
            };
            nat_add(a2181, a2182)?
        };
        nat_add(a2183, a2184)?
    };
    let a2191 = f293(f334(), {
        let a2185 = v2;
        let a2186 = 16u64;
        let a2187 = 0u64;
        nat_quot(a2185, a2186, a2187)
    });
    let a2192 = f293(f334(), {
        let a2188 = v2;
        let a2189 = 16u64;
        let a2190 = 0u64;
        nat_rem(a2188, a2189, a2190)
    });
    Ok(append_bytes(a2191, a2192))
}

pub fn f320(v0: Fn21, p2196: List<List<(Bytes, Bytes)>>, p2197: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2195 = (p2196, p2197);
    {
        let mut m2195 = m2195;
        loop {
            let step = {
                let (v1, v2) = m2195;
                let m2193 = v2;
                match m2193.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2194: Fn21 = v0.clone();
                            c2194.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f321(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f322(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2199 = {
        let a2198 = v0.clone();
        length_list(a2198)
    };
    let a2200 = 1u64;
    if nat_le(a2199, a2200) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f335(v0)?))
    }
}

pub fn f323(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2201, _) = v0;
        h2201
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2202) = v0;
        h2202
    };
    let m2203 = v1;
    match m2203.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2204 = v4;
            match m2204.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f336(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f324(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2205 = f230(v0, v1);
    let a2206 = f311(v0, v2);
    fixed_u32::bit_or(a2205, a2206)
}

pub fn f325(v0: u32) -> u32 {
    let a2209 = {
        let a2207 = f324(v0, 17u32, 15u32);
        let a2208 = f324(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2207, a2208)
    };
    let a2210 = f230(v0, 10u32);
    fixed_u32::bit_xor(a2209, a2210)
}

pub fn f326(v0: u32) -> u32 {
    let a2213 = {
        let a2211 = f324(v0, 7u32, 25u32);
        let a2212 = f324(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2211, a2212)
    };
    let a2214 = f230(v0, 3u32);
    fixed_u32::bit_xor(a2213, a2214)
}

pub fn f327(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2217 = f40(v0.clone(), {
        let a2215 = v1;
        let a2216 = 2u64;
        nat_add(a2215, a2216)?
    }, 2u64);
    let a2218 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2217, a2218) {
        let m2221 = f337(f40(v0, {
            let a2219 = v1;
            let a2220 = 4u64;
            nat_add(a2219, a2220)?
        }, 2u64))?;
        match m2221 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f237(v2.clone(), 0u64)?;
                let a2222 = v3;
                let a2223 = 32u64;
                if nat_lt(a2222, a2223) {
                    if f338(v3) {
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

pub fn f328(v0: u64) -> Bytes {
    let a2224 = v0;
    let a2225 = 34u64;
    if nat_eq(a2224, a2225) {
        Bytes::lit(&[0x22])
    } else {
        let a2226 = v0;
        let a2227 = 92u64;
        if nat_eq(a2226, a2227) {
            Bytes::lit(&[0x5c])
        } else {
            let a2228 = v0;
            let a2229 = 98u64;
            if nat_eq(a2228, a2229) {
                Bytes::lit(&[0x08])
            } else {
                let a2230 = v0;
                let a2231 = 102u64;
                if nat_eq(a2230, a2231) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2232 = v0;
                    let a2233 = 110u64;
                    if nat_eq(a2232, a2233) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2234 = v0;
                        let a2235 = 114u64;
                        if nat_eq(a2234, a2235) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2236 = v0;
                            let a2237 = 116u64;
                            if nat_eq(a2236, a2237) {
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

pub fn f329(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2240 = {
        let a2238 = v0;
        let a2239 = v1;
        index_bytes(a2238, a2239)
    };
    match m2240 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f339(v2)? {
                Ok(Some({
                    let a2241 = v1;
                    let a2242 = 1u64;
                    nat_add(a2241, a2242)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f330(v0: u8) -> R<u64> {
    let a2289 = {
        let v1: u8 = v0;
        let a2287 = {
            let a2263 = {
                let a2251 = {
                    let a2245 = {
                        let a2243 = v1;
                        let a2244 = 128u8;
                        fixed_u8::bit_and(a2243, a2244)
                    };
                    let a2246 = 128u8;
                    if equal(a2245, a2246) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2252 = {
                    let a2249 = {
                        let a2247 = v1;
                        let a2248 = 64u8;
                        fixed_u8::bit_and(a2247, a2248)
                    };
                    let a2250 = 64u8;
                    if equal(a2249, a2250) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2251, a2252)?
            };
            let a2264 = {
                let a2261 = {
                    let a2255 = {
                        let a2253 = v1;
                        let a2254 = 32u8;
                        fixed_u8::bit_and(a2253, a2254)
                    };
                    let a2256 = 32u8;
                    if equal(a2255, a2256) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2262 = {
                    let a2259 = {
                        let a2257 = v1;
                        let a2258 = 16u8;
                        fixed_u8::bit_and(a2257, a2258)
                    };
                    let a2260 = 16u8;
                    if equal(a2259, a2260) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2261, a2262)?
            };
            nat_add(a2263, a2264)?
        };
        let a2288 = {
            let a2285 = {
                let a2273 = {
                    let a2267 = {
                        let a2265 = v1;
                        let a2266 = 8u8;
                        fixed_u8::bit_and(a2265, a2266)
                    };
                    let a2268 = 8u8;
                    if equal(a2267, a2268) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2274 = {
                    let a2271 = {
                        let a2269 = v1;
                        let a2270 = 4u8;
                        fixed_u8::bit_and(a2269, a2270)
                    };
                    let a2272 = 4u8;
                    if equal(a2271, a2272) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2273, a2274)?
            };
            let a2286 = {
                let a2283 = {
                    let a2277 = {
                        let a2275 = v1;
                        let a2276 = 2u8;
                        fixed_u8::bit_and(a2275, a2276)
                    };
                    let a2278 = 2u8;
                    if equal(a2277, a2278) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2284 = {
                    let a2281 = {
                        let a2279 = v1;
                        let a2280 = 1u8;
                        fixed_u8::bit_and(a2279, a2280)
                    };
                    let a2282 = 1u8;
                    if equal(a2281, a2282) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2283, a2284)?
            };
            nat_add(a2285, a2286)?
        };
        nat_add(a2287, a2288)?
    };
    let a2290 = 48u64;
    Ok(nat_sub(a2289, a2290))
}

pub fn f331(p2296: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2295 = p2296;
    {
        let mut m2295 = m2295;
        loop {
            let step = {
                let v0 = m2295;
                let m2291 = v0;
                match m2291.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f62({
                            let (h2292, _) = v1.clone();
                            h2292
                        }, f340()) {
                            true
                        } else {
                            f62({
                                let (h2293, _) = v1.clone();
                                h2293
                            }, f341())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2294, _) = v1;
                                h2294
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: Bytes) -> R<Option<Bytes>> {
    if f210(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f101(f40(v0.clone(), 8u64, {
            let a2298 = {
                let a2297 = v0;
                length_bytes(a2297)
            };
            let a2299 = 8u64;
            nat_sub(a2298, a2299)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f333(v0: Adt14) -> Bytes {
    let m2300 = f218(f95(v0, f8(Str::lit("minimum"))));
    match m2300 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f334() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f335(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f296({
        let (_, h2301) = {
            let (h2302, _) = f342(Fn22::F343, {
                let a2304 = {
                    let a2303 = v0.clone();
                    length_list(a2303)
                };
                let a2305 = 1u64;
                nat_add(a2304, a2305)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2302
        };
        h2301
    }))
}

pub fn f336(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2310 = {
            let a2308 = {
                let a2306 = v0.clone();
                length_list(a2306)
            };
            let a2309 = {
                let a2307 = v1.clone();
                length_list(a2307)
            };
            nat_add(a2308, a2309)?
        };
        let a2311 = 1u64;
        nat_add(a2310, a2311)?
    };
    Ok(f226({
        let (_, h2312) = {
            let (_, h2313) = {
                let (h2314, _) = f344(Fn23::F345, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2314
            };
            h2313
        };
        h2312
    }))
}

pub fn f337(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2315 = v0.clone();
        length_bytes(a2315)
    };
    let a2319 = {
        let a2316 = v1;
        let a2317 = 2u64;
        let a2318 = 0u64;
        nat_rem(a2316, a2317, a2318)
    };
    let a2320 = 0u64;
    if nat_eq(a2319, a2320) {
        let (_, h2321) = {
            let (h2322, _) = f346(Fn24::F347(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2322
        };
        Ok(h2321)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f338(v0: u64) -> bool {
    let a2323 = v0;
    let a2324 = 8u64;
    if nat_eq(a2323, a2324) {
        true
    } else {
        let a2325 = v0;
        let a2326 = 9u64;
        if nat_eq(a2325, a2326) {
            true
        } else {
            let a2327 = v0;
            let a2328 = 10u64;
            if nat_eq(a2327, a2328) {
                true
            } else {
                let a2329 = v0;
                let a2330 = 12u64;
                if nat_eq(a2329, a2330) {
                    true
                } else {
                    let a2331 = v0;
                    let a2332 = 13u64;
                    nat_eq(a2331, a2332)
                }
            }
        }
    }
}

pub fn f339(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2377 = {
            let a2353 = {
                let a2341 = {
                    let a2335 = {
                        let a2333 = v1;
                        let a2334 = 128u8;
                        fixed_u8::bit_and(a2333, a2334)
                    };
                    let a2336 = 128u8;
                    if equal(a2335, a2336) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2342 = {
                    let a2339 = {
                        let a2337 = v1;
                        let a2338 = 64u8;
                        fixed_u8::bit_and(a2337, a2338)
                    };
                    let a2340 = 64u8;
                    if equal(a2339, a2340) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2341, a2342)?
            };
            let a2354 = {
                let a2351 = {
                    let a2345 = {
                        let a2343 = v1;
                        let a2344 = 32u8;
                        fixed_u8::bit_and(a2343, a2344)
                    };
                    let a2346 = 32u8;
                    if equal(a2345, a2346) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2352 = {
                    let a2349 = {
                        let a2347 = v1;
                        let a2348 = 16u8;
                        fixed_u8::bit_and(a2347, a2348)
                    };
                    let a2350 = 16u8;
                    if equal(a2349, a2350) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2351, a2352)?
            };
            nat_add(a2353, a2354)?
        };
        let a2378 = {
            let a2375 = {
                let a2363 = {
                    let a2357 = {
                        let a2355 = v1;
                        let a2356 = 8u8;
                        fixed_u8::bit_and(a2355, a2356)
                    };
                    let a2358 = 8u8;
                    if equal(a2357, a2358) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2364 = {
                    let a2361 = {
                        let a2359 = v1;
                        let a2360 = 4u8;
                        fixed_u8::bit_and(a2359, a2360)
                    };
                    let a2362 = 4u8;
                    if equal(a2361, a2362) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2363, a2364)?
            };
            let a2376 = {
                let a2373 = {
                    let a2367 = {
                        let a2365 = v1;
                        let a2366 = 2u8;
                        fixed_u8::bit_and(a2365, a2366)
                    };
                    let a2368 = 2u8;
                    if equal(a2367, a2368) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2374 = {
                    let a2371 = {
                        let a2369 = v1;
                        let a2370 = 1u8;
                        fixed_u8::bit_and(a2369, a2370)
                    };
                    let a2372 = 1u8;
                    if equal(a2371, a2372) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2373, a2374)?
            };
            nat_add(a2375, a2376)?
        };
        nat_add(a2377, a2378)?
    };
    let a2379 = 48u64;
    let a2380 = v2;
    if nat_le(a2379, a2380) {
        let a2381 = v2;
        let a2382 = 57u64;
        Ok(nat_le(a2381, a2382))
    } else {
        Ok(false)
    }
}

pub fn f340() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f341() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f342(v0: Fn22, p2387: u64, p2388: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2386 = (p2387, p2388);
    {
        let mut m2386 = m2386;
        loop {
            let step = {
                let (v1, v2) = m2386;
                let m2383 = v1;
                if m2383 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2383 - 1;
                    let m2385 = {
                        let c2384: Fn22 = v0.clone();
                        c2384.apply(v2.clone())?
                    };
                    match m2385 {
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
                Ok(next) => { m2386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f348(v0)
}

pub fn f344(v0: Fn23, p2393: u64, p2394: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2392 = (p2393, p2394);
    {
        let mut m2392 = m2392;
        loop {
            let step = {
                let (v1, v2) = m2392;
                let m2389 = v1;
                if m2389 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2389 - 1;
                    let m2391 = {
                        let c2390: Fn23 = v0.clone();
                        c2390.apply(v2.clone())
                    };
                    match m2391 {
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
                Ok(next) => { m2392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f349(v0)
}

pub fn f346(v0: Fn24, p2399: u64, p2400: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2398 = (p2399, p2400);
    {
        let mut m2398 = m2398;
        loop {
            let step = {
                let (v1, v2) = m2398;
                let m2395 = v1;
                if m2395 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2395 - 1;
                    let m2397 = {
                        let c2396: Fn24 = v0.clone();
                        c2396.apply(v2.clone())?
                    };
                    match m2397 {
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
                Ok(next) => { m2398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f350(v0, v1)
}

pub fn f348(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2401, _) = v0;
        h2401
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2402) = v0;
        h2402
    };
    let m2403 = v1;
    match m2403.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2404 = v4;
            match m2404.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f351(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f349(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2405, _) = v0.clone();
        h2405
    };
    let v2: List<(Bytes, ())> = {
        let (h2406, _) = {
            let (_, h2407) = v0.clone();
            h2407
        };
        h2406
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2408) = {
            let (_, h2409) = v0;
            h2409
        };
        h2408
    };
    let m2410 = v1.clone();
    match m2410.uncons() {
        None => {
            let m2411 = v2;
            match m2411.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2412 = v2.clone();
            match m2412.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f352({
                        let (h2413, _) = v8.clone();
                        h2413
                    }, {
                        let (h2414, _) = v6.clone();
                        h2414
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

pub fn f350(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2415, _) = v1;
        h2415
    };
    let m2417 = {
        let (_, h2416) = v1;
        h2416
    };
    match m2417 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2420 = {
                let a2418 = v0.clone();
                let a2419 = v2;
                index_bytes(a2418, a2419)
            };
            match m2420 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2425 = {
                        let a2423 = v0;
                        let a2424 = {
                            let a2421 = v2;
                            let a2422 = 1u64;
                            nat_add(a2421, a2422)?
                        };
                        index_bytes(a2423, a2424)
                    };
                    match m2425 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f353(v4)? {
                                f353(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2426 = v2;
                                    let a2427 = 2u64;
                                    nat_add(a2426, a2427)?
                                }, Some({
                                    let a2432 = v3;
                                    let a2433 = f293(f354(), {
                                        let a2430 = {
                                            let a2428 = f355(v4)?;
                                            let a2429 = 16u64;
                                            nat_mul(a2428, a2429)?
                                        };
                                        let a2431 = f355(v5)?;
                                        nat_add(a2430, a2431)?
                                    });
                                    append_bytes(a2432, a2433)
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

pub fn f351(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2438 = {
            let a2436 = {
                let a2434 = v0.clone();
                length_list(a2434)
            };
            let a2437 = {
                let a2435 = v1.clone();
                length_list(a2435)
            };
            nat_add(a2436, a2437)?
        };
        let a2439 = 1u64;
        nat_add(a2438, a2439)?
    };
    Ok(f356({
        let (_, h2440) = {
            let (_, h2441) = {
                let (h2442, _) = f357(Fn25::F358, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2442
            };
            h2441
        };
        h2440
    }))
}

pub fn f352(v0: Bytes, v1: Bytes) -> bool {
    let a2447 = {
        let a2443 = v0;
        let a2444 = v1;
        compare_bytes(a2443, a2444)
    };
    let a2448 = {
        let a2445 = Bytes::lit(&[]);
        let a2446 = Bytes::lit(&[0x01]);
        compare_bytes(a2445, a2446)
    };
    equal(a2447, a2448)
}

pub fn f353(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2493 = {
            let a2469 = {
                let a2457 = {
                    let a2451 = {
                        let a2449 = v1;
                        let a2450 = 128u8;
                        fixed_u8::bit_and(a2449, a2450)
                    };
                    let a2452 = 128u8;
                    if equal(a2451, a2452) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2458 = {
                    let a2455 = {
                        let a2453 = v1;
                        let a2454 = 64u8;
                        fixed_u8::bit_and(a2453, a2454)
                    };
                    let a2456 = 64u8;
                    if equal(a2455, a2456) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2457, a2458)?
            };
            let a2470 = {
                let a2467 = {
                    let a2461 = {
                        let a2459 = v1;
                        let a2460 = 32u8;
                        fixed_u8::bit_and(a2459, a2460)
                    };
                    let a2462 = 32u8;
                    if equal(a2461, a2462) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2468 = {
                    let a2465 = {
                        let a2463 = v1;
                        let a2464 = 16u8;
                        fixed_u8::bit_and(a2463, a2464)
                    };
                    let a2466 = 16u8;
                    if equal(a2465, a2466) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2467, a2468)?
            };
            nat_add(a2469, a2470)?
        };
        let a2494 = {
            let a2491 = {
                let a2479 = {
                    let a2473 = {
                        let a2471 = v1;
                        let a2472 = 8u8;
                        fixed_u8::bit_and(a2471, a2472)
                    };
                    let a2474 = 8u8;
                    if equal(a2473, a2474) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2480 = {
                    let a2477 = {
                        let a2475 = v1;
                        let a2476 = 4u8;
                        fixed_u8::bit_and(a2475, a2476)
                    };
                    let a2478 = 4u8;
                    if equal(a2477, a2478) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2479, a2480)?
            };
            let a2492 = {
                let a2489 = {
                    let a2483 = {
                        let a2481 = v1;
                        let a2482 = 2u8;
                        fixed_u8::bit_and(a2481, a2482)
                    };
                    let a2484 = 2u8;
                    if equal(a2483, a2484) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2490 = {
                    let a2487 = {
                        let a2485 = v1;
                        let a2486 = 1u8;
                        fixed_u8::bit_and(a2485, a2486)
                    };
                    let a2488 = 1u8;
                    if equal(a2487, a2488) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2489, a2490)?
            };
            nat_add(a2491, a2492)?
        };
        nat_add(a2493, a2494)?
    };
    let a2495 = 48u64;
    let a2496 = v2;
    if if nat_le(a2495, a2496) {
        let a2497 = v2;
        let a2498 = 57u64;
        nat_le(a2497, a2498)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2499 = 97u64;
        let a2500 = v2;
        if nat_le(a2499, a2500) {
            let a2501 = v2;
            let a2502 = 102u64;
            Ok(nat_le(a2501, a2502))
        } else {
            Ok(false)
        }
    }
}

pub fn f354() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f355(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2547 = {
            let a2523 = {
                let a2511 = {
                    let a2505 = {
                        let a2503 = v1;
                        let a2504 = 128u8;
                        fixed_u8::bit_and(a2503, a2504)
                    };
                    let a2506 = 128u8;
                    if equal(a2505, a2506) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2512 = {
                    let a2509 = {
                        let a2507 = v1;
                        let a2508 = 64u8;
                        fixed_u8::bit_and(a2507, a2508)
                    };
                    let a2510 = 64u8;
                    if equal(a2509, a2510) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2511, a2512)?
            };
            let a2524 = {
                let a2521 = {
                    let a2515 = {
                        let a2513 = v1;
                        let a2514 = 32u8;
                        fixed_u8::bit_and(a2513, a2514)
                    };
                    let a2516 = 32u8;
                    if equal(a2515, a2516) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2522 = {
                    let a2519 = {
                        let a2517 = v1;
                        let a2518 = 16u8;
                        fixed_u8::bit_and(a2517, a2518)
                    };
                    let a2520 = 16u8;
                    if equal(a2519, a2520) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2521, a2522)?
            };
            nat_add(a2523, a2524)?
        };
        let a2548 = {
            let a2545 = {
                let a2533 = {
                    let a2527 = {
                        let a2525 = v1;
                        let a2526 = 8u8;
                        fixed_u8::bit_and(a2525, a2526)
                    };
                    let a2528 = 8u8;
                    if equal(a2527, a2528) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2534 = {
                    let a2531 = {
                        let a2529 = v1;
                        let a2530 = 4u8;
                        fixed_u8::bit_and(a2529, a2530)
                    };
                    let a2532 = 4u8;
                    if equal(a2531, a2532) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2533, a2534)?
            };
            let a2546 = {
                let a2543 = {
                    let a2537 = {
                        let a2535 = v1;
                        let a2536 = 2u8;
                        fixed_u8::bit_and(a2535, a2536)
                    };
                    let a2538 = 2u8;
                    if equal(a2537, a2538) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2544 = {
                    let a2541 = {
                        let a2539 = v1;
                        let a2540 = 1u8;
                        fixed_u8::bit_and(a2539, a2540)
                    };
                    let a2542 = 1u8;
                    if equal(a2541, a2542) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2543, a2544)?
            };
            nat_add(a2545, a2546)?
        };
        nat_add(a2547, a2548)?
    };
    let a2549 = v2;
    let a2550 = 57u64;
    if nat_le(a2549, a2550) {
        let a2551 = v2;
        let a2552 = 48u64;
        Ok(nat_sub(a2551, a2552))
    } else {
        let a2553 = v2;
        let a2554 = 87u64;
        Ok(nat_sub(a2553, a2554))
    }
}

pub fn f356(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f248(Fn10::F359, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f357(v0: Fn25, p2559: u64, p2560: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2558 = (p2559, p2560);
    {
        let mut m2558 = m2558;
        loop {
            let step = {
                let (v1, v2) = m2558;
                let m2555 = v1;
                if m2555 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2555 - 1;
                    let m2557 = {
                        let c2556: Fn25 = v0.clone();
                        c2556.apply(v2.clone())
                    };
                    match m2557 {
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
                Ok(next) => { m2558 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f360(v0)
}

pub fn f359(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f360(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2561, _) = v0.clone();
        h2561
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2562, _) = {
            let (_, h2563) = v0.clone();
            h2563
        };
        h2562
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2564) = {
            let (_, h2565) = v0;
            h2565
        };
        h2564
    };
    let m2566 = v1.clone();
    match m2566.uncons() {
        None => {
            let m2567 = v2;
            match m2567.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2568 = v2.clone();
            match m2568.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f352({
                        let (h2569, _) = v8.clone();
                        h2569
                    }, {
                        let (h2570, _) = v6.clone();
                        h2570
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
