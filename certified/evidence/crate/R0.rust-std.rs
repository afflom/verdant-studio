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
    F189(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F58(k0, k1) => f58(*k0, k1.clone(), p0),
            Fn0::F189(k0) => f189(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F167,
    F185,
    F277,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F167 => f167(p0, p1),
            Fn1::F185 => f185(p0, p1),
            Fn1::F277 => f277(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F221(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F221(k0) => f221(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F267(Bytes, u64, Bytes, u64),
    F318(Bytes),
    F388(Bytes),
    F393(Bytes, Bytes),
    F394(Bytes),
    F395(Bytes, u64),
    F409(u64, Bytes),
    F455(bool, u64, Bytes),
    F509(Bytes),
    F510(Bytes),
    F546(Bytes),
    F589(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F267(k0, k1, k2, k3) => f267(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F318(k0) => f318(k0.clone(), p0),
            Fn3::F388(k0) => Ok(f388(k0.clone(), p0)),
            Fn3::F393(k0, k1) => f393(k0.clone(), k1.clone(), p0),
            Fn3::F394(k0) => f394(k0.clone(), p0),
            Fn3::F395(k0, k1) => f395(k0.clone(), *k1, p0),
            Fn3::F409(k0, k1) => f409(*k0, k1.clone(), p0),
            Fn3::F455(k0, k1, k2) => f455(*k0, *k1, k2.clone(), p0),
            Fn3::F509(k0) => f509(k0.clone(), p0),
            Fn3::F510(k0) => f510(k0.clone(), p0),
            Fn3::F546(k0) => f546(k0.clone(), p0),
            Fn3::F589(k0) => f589(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F310(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn4::F310(k0, k1, k2) => f310(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F317(Bytes),
    F330(u64, Bytes),
    F428(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F317(k0) => f317(k0.clone(), p0),
            Fn5::F330(k0, k1) => Ok(f330(*k0, k1.clone(), p0)),
            Fn5::F428(k0) => f428(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F368,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn6::F368 => f368(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F371,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn7::F371 => f371(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F373,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn8::F373 => f373(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F376,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn9::F376 => f376(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F430,
    F647,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F430 => f430(p0, p1),
            Fn10::F647 => f647(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F433,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F433 => f433(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F436,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F436 => f436(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F439,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F439 => f439(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F444,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn14::F444 => f444(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F462(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F462(k0) => f462(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F497(bool, Bytes),
    F512(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F497(k0, k1) => f497(*k0, k1.clone(), p0),
            Fn16::F512(k0) => f512(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F521,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F521 => f521(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F523,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F523 => f523(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F525,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F525 => f525(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F533,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F533 => f533(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F563,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F563 => f563(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F621,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F621 => f621(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F623,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F623 => f623(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F625(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F625(k0) => f625(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F646,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F646 => f646(p0),
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
            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ")), List::cons(v9, List::cons(f8(Str::lit(" is not\u{a}")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f28(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m177 = v0;
    match m177.uncons() {
        None => {
            Ok(Adt11::C4(f35()))
        }
        Some((v2, v3)) => {
            if f36(v2.clone(), f8(Str::lit("check"))) {
                f37(v1)
            } else {
                if f36(v2.clone(), f8(Str::lit("next"))) {
                    f38(v1, v3)
                } else {
                    if f36(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f39(v1))
                    } else {
                        if f36(v2.clone(), f8(Str::lit("publish"))) {
                            f40(v1)
                        } else {
                            if f36(v2.clone(), f8(Str::lit("agenda"))) {
                                f41(v1)
                            } else {
                                let m178 = v3;
                                match m178.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f35()))
                                    }
                                    Some((v4, v5)) => {
                                        if f36(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f42(v1, v4)
                                        } else {
                                            if f36(v2.clone(), f8(Str::lit("read-xml"))) {
                                                f43(v1, v4)
                                            } else {
                                                if f36(v2.clone(), f8(Str::lit("read-xsd"))) {
                                                    f44(v1, v4)
                                                } else {
                                                    if f36(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                        f45(v1.clone(), v4.clone(), f46(v1, v4)?)
                                                    } else {
                                                        if f36(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                            f45(v1.clone(), v4.clone(), f47(v1, v4, false)?)
                                                        } else {
                                                            if f36(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                                f45(v1.clone(), v4.clone(), f47(v1, v4, true)?)
                                                            } else {
                                                                if f36(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                                    f45(v1.clone(), v4.clone(), f48(v1, v4, false)?)
                                                                } else {
                                                                    if f36(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                        f45(v1.clone(), v4.clone(), f48(v1, v4, true)?)
                                                                    } else {
                                                                        if f36(v2.clone(), f8(Str::lit("admit"))) {
                                                                            f49(v1, v4)
                                                                        } else {
                                                                            if f36(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                                f45(v1.clone(), v4.clone(), f50(v1, v4)?)
                                                                            } else {
                                                                                if f36(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                                    f51(v1, v4)
                                                                                } else {
                                                                                    if f36(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                        f52(v1, v4)
                                                                                    } else {
                                                                                        if f36(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                            f53(v1, v4)
                                                                                        } else {
                                                                                            if f36(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                                f54(v1, v4)
                                                                                            } else {
                                                                                                if f36(v2, f8(Str::lit("propose-encoding"))) {
                                                                                                    let m179 = v5;
                                                                                                    match m179.uncons() {
                                                                                                        None => {
                                                                                                            Ok(Adt11::C4(f35()))
                                                                                                        }
                                                                                                        Some((v6, v7)) => {
                                                                                                            f55(v1, v4, v6, v7)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    Ok(Adt11::C4(f35()))
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

pub fn f33(p248: List<Bytes>) -> R<Option<Bytes>> {
    let m247 = p248;
    {
        let mut m247 = m247;
        loop {
            let step = {
                let v0 = m247;
                let m245 = v0;
                match m245.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f60(v1.clone(), 0u64, {
                            let a246 = v1.clone();
                            length_bytes(a246)
                        })? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m249 = v0;
    match m249.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h250, _) = v1;
                h250
            }, f34(v2))
        }
    }
}

pub fn f35() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | read-xml FILE | read-xsd FILE | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f36(v0: Bytes, v1: Bytes) -> bool {
    let a251 = v0;
    let a252 = v1;
    equal(a251, a252)
}

pub fn f37(v0: Adt1) -> R<Adt11> {
    let m253 = f61(v0.clone(), f62());
    match m253.uncons() {
        None => {
            let m254 = f63(v0.clone(), f62())?;
            match m254.uncons() {
                None => {
                    let m257 = f64(match v0.clone() {
                        Adt1::C0(_, h255, _) => {
                            h255
                        }
                    }, f65(f66(), f67(match v0.clone() {
                        Adt1::C0(_, h256, _) => {
                            h256
                        }
                    }, false)));
                    match m257.uncons() {
                        None => {
                            let m259 = f61(v0.clone(), f67(match v0.clone() {
                                Adt1::C0(_, h258, _) => {
                                    h258
                                }
                            }, true));
                            match m259.uncons() {
                                None => {
                                    let m261 = f68(v0.clone(), f67(match v0.clone() {
                                        Adt1::C0(_, h260, _) => {
                                            h260
                                        }
                                    }, true))?;
                                    match m261 {
                                        None => {
                                            let v10: (bool, Bytes) = f69(Adt13::C0(Adt12::C0(f70(f71(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f72(v0.clone())?, f73(f74(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f70(f71(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f73(f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f74(v0.clone(), f8(Str::lit("model/strata.json")))?, f75(f74(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f67(match v0 {
                                                Adt1::C0(_, h262, _) => {
                                                    h262
                                                }
                                            }, true)))?;
                                            let (h263, _) = v10;
                                            if h263 {
                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                    let (_, h264) = v10;
                                                    h264
                                                }))
                                            } else {
                                                Ok(Adt11::C4({
                                                    let (_, h265) = v10;
                                                    h265
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

pub fn f38(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f76(f77(match v0.clone() {
        Adt1::C0(_, h266, _) => {
            h266
        }
    }, f78())) {
        Ok(Adt11::C1(List::cons(f78(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f79(f80(f78(), f81(match v0.clone() {
            Adt1::C0(_, h267, _) => {
                h267
            }
        }, f78())))?;
        let m268 = f61(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m268.uncons() {
            None => {
                let m269 = f82(v0.clone(), f83(f84(f71(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f85(v2, v1.clone()))?;
                match m269 {
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

pub fn f39(v0: Adt1) -> Adt11 {
    if f76(f87(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h270) => {
            h270
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f88(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f76(f87(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h271) => {
                h271
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f88(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f40(v0: Adt1) -> R<Adt11> {
    let m272 = f89(v0.clone());
    match m272.uncons() {
        None => {
            let m273 = f65(f61(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f90(), List::<Bytes>::nil())))), f91(v0.clone())?);
            match m273.uncons() {
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

pub fn f41(v0: Adt1) -> R<Adt11> {
    let m274 = f89(v0.clone());
    match m274.uncons() {
        None => {
            let m275 = f65(f61(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f91(v0.clone())?);
            match m275.uncons() {
                None => {
                    let v5: Adt15 = f103(v0.clone())?;
                    let v6: List<Adt14> = f74(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f65(f104(f105(match v5.clone() {
                        Adt15::C0(h276, _, _) => {
                            h276
                        }
                    }, f106(f74(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f107(v6.clone())), f108(f105(match v5.clone() {
                        Adt15::C0(h277, _, _) => {
                            h277
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

pub fn f42(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m278 = f61(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m278.uncons() {
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

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m279 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m279.uncons() {
        None => {
            let m280 = f125(f71(v0, v1))?;
            match m280 {
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

pub fn f44(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m281 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m281.uncons() {
        None => {
            let m282 = f129(f71(v0, v1))?;
            match m282 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f127({
                        let (h283, _) = v5;
                        h283
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h284) = v5;
                        h284
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

pub fn f45(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m285 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m285.uncons() {
        None => {
            if f130(f84(f71(v0, v1))?)? {
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
    let m286 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m286.uncons() {
        None => {
            let m287 = f131(f84(f71(v0.clone(), v1.clone()))?)?;
            match m287 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    let m288 = f132(f84(f71(v0, v1))?)?;
                    match m288 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("binding ")), List::cons(v5, List::cons(f8(Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f47(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m289 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m289.uncons() {
        None => {
            let v5: Bytes = f133(if v2 {
                f134()
            } else {
                f135()
            }, f136(v1.clone())?);
            if f76(f137(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m290 = f138(f84(f71(v0.clone(), v1.clone()))?, v2)?;
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
            if f76(f137(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f139(v5)?
                } else {
                    f140(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f133(f141(), f136(v1.clone())?), f71(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f133(f141(), f136(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f142()
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
            let m296 = f143(v4.clone(), f8(Str::lit("requirement")))?;
            match m296 {
                None => {
                    Ok(Adt11::C4(f142()))
                }
                Some(v5) => {
                    let m297 = f143(v4, f8(Str::lit("admission")))?;
                    match m297 {
                        None => {
                            Ok(Adt11::C4(f142()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f74(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f74(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f65(f73(v5.clone(), f8(Str::lit("id"))), f73(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f79(f65(f73(v7.clone(), f8(Str::lit("id"))), f73(v8.clone(), f8(Str::lit("construct")))))?;
                            let m298 = f144(f107(v6.clone()), f106(f74(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m298 {
                                None => {
                                    let a300 = {
                                        let a299 = v9.clone();
                                        length_list(a299)
                                    };
                                    let a301 = 0u64;
                                    if nat_eq(a300, a301) {
                                        Ok(Adt11::C4(f142()))
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
                                            Ok(Adt11::C3(List::cons(f146(f8(Str::lit("model/requirements.json")), f147(v7, v5))?, List::cons(f146(f8(Str::lit("model/admissions.json")), f147(v8, v6))?, List::cons(f121(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f122(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f145(v11))
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
            let m307 = f148(f84(f71(v0, v1))?)?;
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
            let m309 = f149(v0.clone(), v1.clone())?;
            match m309 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m310 = f150(v4.clone(), f8(Str::lit("schema")));
                    match m310 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f151(v0, f152(v5.clone())?, v5, f153(v4.clone(), f8(Str::lit("name"))), f154(f155(f150(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
            let m312 = f156(f84(f71(v0.clone(), v1))?)?;
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
                        Ok(Adt11::C3(List::cons(f146(f90(), f147(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f122(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
            let m321 = f157(v0, v1)?;
            match m321 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f158(f159(Some(v4))) {
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
            let m323 = f157(v0.clone(), v1.clone())?;
            match m323 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f76(f36(f160(v4.clone(), f8(Str::lit("schema"))), f161())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f160(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m324 = f162(v4.clone(), f8(Str::lit("source")));
                            match m324 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f163(f162(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f133(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f133(v9.clone(), f8(Str::lit("binding.json"))), f71(v0.clone(), v1));
                        let m325 = v8;
                        match m325 {
                            None => {
                                f164(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f76(f36(f160(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f164(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f133(v9, f136(f160(v7.clone(), f8(Str::lit("url"))))?);
                                    if f76(f87(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h326) => {
                                            h326
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f160(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m327 = f61(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m327.uncons() {
                                            None => {
                                                if f36(f119(f71(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f164(v0, v4, v5, v10, Some((v12, v11)))
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
            let m329 = f149(v0.clone(), v1.clone())?;
            match m329 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f151(v0.clone(), f71(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
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
    f165(v1, v0, v2)
}

pub fn f59(v0: List<Bytes>) -> List<Bytes> {
    f166(Fn1::F167, List::<Bytes>::nil(), v0)
}

pub fn f60(v0: Bytes, p391: u64, p392: u64) -> R<bool> {
    let m390 = (p391, p392);
    {
        let mut m390 = m390;
        loop {
            let step = {
                let (v1, v2) = m390;
                let m336 = v2;
                if m336 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m336 - 1;
                    let m339 = {
                        let a337 = v0.clone();
                        let a338 = v1;
                        index_bytes(a337, a338)
                    };
                    match m339 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a386 = {
                                let v5: u8 = v4;
                                let a384 = {
                                    let a360 = {
                                        let a348 = {
                                            let a342 = {
                                                let a340 = v5;
                                                let a341 = 128u8;
                                                fixed_u8::bit_and(a340, a341)
                                            };
                                            let a343 = 128u8;
                                            if equal(a342, a343) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a349 = {
                                            let a346 = {
                                                let a344 = v5;
                                                let a345 = 64u8;
                                                fixed_u8::bit_and(a344, a345)
                                            };
                                            let a347 = 64u8;
                                            if equal(a346, a347) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a348, a349)?
                                    };
                                    let a361 = {
                                        let a358 = {
                                            let a352 = {
                                                let a350 = v5;
                                                let a351 = 32u8;
                                                fixed_u8::bit_and(a350, a351)
                                            };
                                            let a353 = 32u8;
                                            if equal(a352, a353) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a359 = {
                                            let a356 = {
                                                let a354 = v5;
                                                let a355 = 16u8;
                                                fixed_u8::bit_and(a354, a355)
                                            };
                                            let a357 = 16u8;
                                            if equal(a356, a357) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a358, a359)?
                                    };
                                    nat_add(a360, a361)?
                                };
                                let a385 = {
                                    let a382 = {
                                        let a370 = {
                                            let a364 = {
                                                let a362 = v5;
                                                let a363 = 8u8;
                                                fixed_u8::bit_and(a362, a363)
                                            };
                                            let a365 = 8u8;
                                            if equal(a364, a365) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a371 = {
                                            let a368 = {
                                                let a366 = v5;
                                                let a367 = 4u8;
                                                fixed_u8::bit_and(a366, a367)
                                            };
                                            let a369 = 4u8;
                                            if equal(a368, a369) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a370, a371)?
                                    };
                                    let a383 = {
                                        let a380 = {
                                            let a374 = {
                                                let a372 = v5;
                                                let a373 = 2u8;
                                                fixed_u8::bit_and(a372, a373)
                                            };
                                            let a375 = 2u8;
                                            if equal(a374, a375) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a381 = {
                                            let a378 = {
                                                let a376 = v5;
                                                let a377 = 1u8;
                                                fixed_u8::bit_and(a376, a377)
                                            };
                                            let a379 = 1u8;
                                            if equal(a378, a379) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a380, a381)?
                                    };
                                    nat_add(a382, a383)?
                                };
                                nat_add(a384, a385)?
                            };
                            let a387 = 127u64;
                            if nat_le(a386, a387) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a388 = v1;
                                    let a389 = 1u64;
                                    nat_add(a388, a389)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m393 = v1;
    match m393.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m395 = f168(match v0.clone() {
                Adt1::C0(h394, _, _) => {
                    h394
                }
            }, v2.clone());
            match m395 {
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
    let m396 = v1;
    match m396.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f169(v0.clone(), v2.clone()) {
                f137(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m397 = f157(v0.clone(), v2.clone())?;
                match m397 {
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
    let m398 = v1;
    match m398.uncons() {
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
    let m399 = v0;
    match m399.uncons() {
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
    let m400 = v0;
    match m400.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f65(f170({
                let (h401, _) = v2;
                h401
            }, {
                let (_, h402) = v2;
                h402
            }, v1), f67(v3, v1))
        }
    }
}

pub fn f68(v0: Adt1, p406: List<Bytes>) -> R<Option<Bytes>> {
    let m405 = p406;
    {
        let mut m405 = m405;
        loop {
            let step = {
                let v1 = m405;
                let m403 = v1;
                match m403.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m404 = f171(f71(v0.clone(), v2.clone()), f172())?;
                        match m404 {
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
                Ok(next) => { m405 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h407, _, _, _, _) => {
            h407
        }
    };
    let v2: Adt16 = f173(v1.clone())?;
    let v3: List<Bytes> = f174(v1.clone());
    let v4: List<Bytes> = f73(match v0.clone() {
        Adt13::C0(_, _, h408, _, _) => {
            h408
        }
    }, f8(Str::lit("path")));
    let m410 = f175(v2, List::<u64>::nil(), f176(v1.clone(), match v0.clone() {
        Adt13::C0(_, h409, _, _, _) => {
            h409
        }
    }, 0u64)?);
    match m410 {
        None => {
            if f178(f179(match v0.clone() {
                Adt13::C0(_, _, _, _, h412) => {
                    h412
                }
            }, f65(match v0.clone() {
                Adt13::C0(_, _, _, _, h413) => {
                    h413
                }
            }, v4.clone()))?, f179(v4.clone(), f65(match v0.clone() {
                Adt13::C0(_, _, _, _, h414) => {
                    h414
                }
            }, v4.clone()))?, f180(match v0.clone() {
                Adt13::C0(_, _, h415, _, _) => {
                    h415
                }
            })) {
                let m419 = f181(f179(match v0.clone() {
                    Adt13::C0(_, _, _, h416, _) => {
                        h416
                    }
                }, f65(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h417, _) => {
                        h417
                    }
                }))?, f179(v3.clone(), f65(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h418, _) => {
                        h418
                    }
                }))?);
                match m419 {
                    None => {
                        let m422 = f182(f73(match v0 {
                            Adt13::C0(_, h421, _, _, _) => {
                                h421
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m422 {
                            None => {
                                let m423 = f182(v4, List::<Bytes>::nil());
                                match m423 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f177(f65(v3, match v0 {
                            Adt13::C0(_, _, _, h420, _) => {
                                h420
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f177(match v1 {
                Adt12::C0(_, _, _, _, h411) => {
                    h411
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f70(v0: Bytes) -> R<List<Bytes>> {
    f183(f84(v0)?, f8(Str::lit("id = \"")))
}

pub fn f71(v0: Adt1, v1: Bytes) -> Bytes {
    let m425 = f168(match v0 {
        Adt1::C0(h424, _, _) => {
            h424
        }
    }, v1);
    match m425 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m426 = v2;
            match m426 {
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
    let m427 = f113(f71(v0, f118()))?;
    match m427.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f123(v2))
        }
    }
}

pub fn f73(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m428 = v0;
    match m428.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f160(v2, v1.clone()), f73(v3, v1))
        }
    }
}

pub fn f74(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f159(f157(v0, v1)?))
}

pub fn f75(v0: List<Adt14>) -> List<Bytes> {
    let m429 = v0;
    match m429.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m430 = f163(Some(v1));
            match m430 {
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

pub fn f77(p434: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m433 = p434;
    {
        let mut m433 = m433;
        loop {
            let step = {
                let v0 = m433;
                let m431 = v0;
                match m431.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h432, _) = v2;
                            h432
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f79(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f184(f166(Fn1::F185, List::<Bytes>::nil(), f186(v0)?)))
}

pub fn f80(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m435 = v1;
    match m435.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f137(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f133(v0.clone(), v2), f80(v0, v3))
            } else {
                f80(v0, v3)
            }
        }
    }
}

pub fn f81(p440: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m439 = p440;
    {
        let mut m439 = m439;
        loop {
            let step = {
                let v0 = m439;
                let m436 = v0;
                match m436.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h437, _) = v2;
                            h437
                        }, v1.clone()) {
                            let (_, h438) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h438)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: Adt1, v1: List<List<Bytes>>, p443: List<Bytes>) -> R<Option<Bytes>> {
    let m442 = p443;
    {
        let mut m442 = m442;
        loop {
            let step = {
                let v2 = m442;
                let m441 = v2;
                match m441.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f187(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(p446: List<Bytes>, p447: List<Bytes>, p448: List<List<Bytes>>) -> List<List<Bytes>> {
    let m445 = (p446, (p447, p448));
    {
        let mut m445 = m445;
        loop {
            let step = {
                let (v0, (v1, v2)) = m445;
                let m444 = v0;
                match m444.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f188(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f188(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f84(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a449 = v0.clone();
        length_bytes(a449)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h450, _) = f57(Fn0::F189(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h450
    };
    let v3: u64 = {
        let (h451, _) = {
            let (_, h452) = v2.clone();
            h452
        };
        h451
    };
    let v4: List<Bytes> = {
        let (_, h453) = {
            let (_, h454) = v2;
            h454
        };
        h453
    };
    let a455 = v3;
    let a456 = v1;
    if nat_lt(a455, a456) {
        Ok(f59(List::cons(f32(v0, v3, {
            let a457 = v1;
            let a458 = v3;
            nat_sub(a457, a458)
        }), v4)))
    } else {
        Ok(f59(v4))
    }
}

pub fn f85(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m459 = v0;
    match m459.uncons() {
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
        Adt1::C0(_, h460, _) => {
            h460
        }
    }, f190())) {
        Ok(Adt11::C1(List::cons(f190(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f85(f79(f80(f190(), f81(match v0.clone() {
            Adt1::C0(_, h461, _) => {
                h461
            }
        }, f190())))?, v1.clone());
        let m462 = f61(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m462.uncons() {
            None => {
                let v5: List<Bytes> = f73(f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m463 = f61(v0.clone(), f191(v0.clone(), v2.clone(), v5.clone())?);
                match m463.uncons() {
                    None => {
                        let m464 = f192(v0.clone(), v2, v5)?;
                        match m464 {
                            None => {
                                f193(v0, v1)
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

pub fn f87(v0: Bytes, p467: List<Bytes>) -> bool {
    let m466 = p467;
    {
        let mut m466 = m466;
        loop {
            let step = {
                let v1 = m466;
                let m465 = v1;
                match m465.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m466 = next; }
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
        Adt1::C0(_, h468, _) => {
            h468
        }
    }, f135()) {
        if f77(match v0 {
            Adt1::C0(_, h469, _) => {
                h469
            }
        }, f134()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f134(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f135(), List::<Bytes>::nil())
    }
}

pub fn f90() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f91(v0: Adt1) -> R<List<Bytes>> {
    Ok(f61(v0.clone(), List::cons(f118(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f112(v0)?)))))
}

pub fn f92(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m470 = v0;
    match m470.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f92(v3, v1))
        }
    }
}

pub fn f93(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m471 = f74(v0.clone(), f90())?;
    match m471.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f194(v0)?), List::<(Bytes, Bytes)>::nil()))
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
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f195(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f97() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught when it opens, too.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  pull_request:\u{a}    types: [opened]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}              { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f195(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f98() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f99(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f103(v0.clone())?;
    let v2: List<Adt14> = f74(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f110(v1.clone(), f111(v0.clone(), f112(v0.clone())?)?);
    let v4: List<List<Bytes>> = f124(f113(f71(v0, f118()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f196(v4.clone(), f123(v4), f197(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f109(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f100(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f198(f103(v0.clone())?, f111(v0.clone(), f112(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f101(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f199(f74(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f200(f74(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f102() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f103(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f72(v0.clone())?, f70(f71(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f73(f74(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f104(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m472 = v0;
    match m472.uncons() {
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
    let m473 = v0;
    match m473.uncons() {
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
    let m474 = v0;
    match m474.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f65(f75(f159(f162(v1, f8(Str::lit("defers"))))), f106(v2))
        }
    }
}

pub fn f107(v0: List<Adt14>) -> List<Bytes> {
    let m475 = v0;
    match m475.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m476 = f162(v1, f8(Str::lit("std")));
            match m476 {
                None => {
                    f107(v2)
                }
                Some(v3) => {
                    List::cons(f160(v3, f8(Str::lit("row"))), f107(v2))
                }
            }
        }
    }
}

pub fn f108(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m477 = v0;
    match m477.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f92(f201(v2.clone()), f202(f197(v0, v1.clone(), v2)?));
    f203(f79(f204(v1))?, v3.clone(), {
        let a479 = {
            let a478 = v3;
            length_list(a478)
        };
        let a480 = 1u64;
        nat_add(a479, a480)?
    })
}

pub fn f110(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m481 = v1;
    match m481.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f205(v2.clone(), f8(Str::lit("subject"))) {
                f206(v0.clone(), v2.clone())
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
    let m482 = v1;
    match m482.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m483 = f138(f84(f71(v0.clone(), v2.clone()))?, f207(v2, f134()))?;
            match m483 {
                None => {
                    f111(v0, v3)
                }
                Some(v4) => {
                    Ok(f147(v4, f111(v0, v3)?))
                }
            }
        }
    }
}

pub fn f112(v0: Adt1) -> R<List<Bytes>> {
    Ok(f65(f79(f208(f135(), f81(match v0.clone() {
        Adt1::C0(_, h484, _) => {
            h484
        }
    }, f135())))?, f79(f208(f134(), f81(match v0 {
        Adt1::C0(_, h485, _) => {
            h485
        }
    }, f134())))?))
}

pub fn f113(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f209(v0.clone(), 0u64, {
        let a486 = v0;
        length_bytes(a486)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a489 = {
            let a488 = match v1.clone() {
                Adt17::C0(_, _, h487, _, _) => {
                    h487
                }
            };
            length_bytes(a488)
        };
        let a490 = 0u64;
        if if nat_eq(a489, a490) {
            let m492 = match v1.clone() {
                Adt17::C0(_, _, _, h491, _) => {
                    h491
                }
            };
            match m492.uncons() {
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
            f210(v1)
        }
    };
    Ok(f211(match v4 {
        Adt17::C0(_, _, _, _, h493) => {
            h493
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f114(p498: List<Bytes>, p499: List<Bytes>) -> bool {
    let m497 = (p498, p499);
    {
        let mut m497 = m497;
        loop {
            let step = {
                let (v0, v1) = m497;
                let m494 = v0;
                match m494.uncons() {
                    None => {
                        let m495 = v1;
                        match m495.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m496 = v1;
                        match m496.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f36(v4, v6) {
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
                Ok(next) => { m497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<List<Bytes>>) -> List<Bytes> {
    let m500 = v0;
    match m500.uncons() {
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
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f212(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f118() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f119(v0: Bytes) -> R<Bytes> {
    Ok(f213(f214(v0)?))
}

pub fn f120(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f215(f83(f84(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f216(f217(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f121(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f218(Adt14::C3(std::rc::Rc::new(f219(f79(f65(v1, f75(f74(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f122(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m501 = f220(Fn2::F221(v1), None::<Bytes>, v0);
    match m501 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f123(v0: List<List<Bytes>>) -> List<Bytes> {
    let m502 = v0;
    match m502.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m503 = v1;
            match m503.uncons() {
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
    let m504 = v0;
    match m504.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f125(v0: Bytes) -> R<Result<Adt18, Bytes>> {
    let v1: Bytes = if f222(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
            let a506 = {
                let a505 = v0;
                length_bytes(a505)
            };
            let a507 = 3u64;
            nat_sub(a506, a507)
        })
    } else {
        v0
    };
    let v2: Bytes = f223(v1)?;
    let v3: u64 = {
        let a508 = v2.clone();
        length_bytes(a508)
    };
    let v4: u64 = f224(v2.clone())?;
    let a509 = v4;
    let a510 = v3;
    if nat_lt(a509, a510) {
        Ok(Err::<Adt18, Bytes>(f225(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m511 = f226(v2.clone())?;
        match m511 {
            Ok(v6) => {
                let m514 = f227(v2.clone(), v6, {
                    let a512 = v3;
                    let a513 = 2u64;
                    nat_add(a512, a513)?
                })?;
                match m514 {
                    Ok(v8) => {
                        if f228(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(Err::<Adt18, Bytes>(f225(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a515 = f229(v2.clone(), v8)?;
                            let a516 = 60u64;
                            if !nat_eq(a515, a516) {
                                Ok(Err::<Adt18, Bytes>(f225(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            } else {
                                let m519 = f230(v2.clone(), v8, {
                                    let a517 = v3;
                                    let a518 = 2u64;
                                    nat_add(a517, a518)?
                                }, 0u64)?;
                                match m519 {
                                    Ok(v10) => {
                                        let m523 = f227(v2, {
                                            let (_, h520) = v10;
                                            h520
                                        }, {
                                            let a521 = v3;
                                            let a522 = 2u64;
                                            nat_add(a521, a522)?
                                        })?;
                                        match m523 {
                                            Ok(v12) => {
                                                let a524 = v12;
                                                let a525 = v3;
                                                if nat_eq(a524, a525) {
                                                    Ok(Ok::<Adt18, Bytes>({
                                                        let (h526, _) = v10;
                                                        h526
                                                    }))
                                                } else {
                                                    Ok(Err::<Adt18, Bytes>(f225(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
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
    let m527 = v0;
    match m527 {
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
    f231(f232(), 10u64, v0)
}

pub fn f128(v0: Adt18) -> R<u64> {
    let m528 = v0;
    match m528 {
        Adt18::C0(_, _, r533) => {
            let v3 = (*r533).clone();
            let a529 = 1u64;
            let a530 = f128(v3)?;
            nat_add(a529, a530)
        }
        Adt18::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt18::C2(_) => {
            Ok(0u64)
        }
        Adt18::C3(r534, r535) => {
            let v8 = (*r534).clone();
            let v9 = (*r535).clone();
            let a531 = f128(v8)?;
            let a532 = f128(v9)?;
            nat_add(a531, a532)
        }
        Adt18::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f129(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m536 = f125(v0)?;
    match m536 {
        Ok(v2) => {
            let m537 = v2;
            match m537 {
                Adt18::C0(v3, r549, r550) => {
                    let v4 = (*r549).clone();
                    let v5 = (*r550).clone();
                    let m538 = f233(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m538 {
                        Ok(v7) => {
                            let m539 = f234(v7.clone(), v3)?;
                            match m539 {
                                Ok(v9) => {
                                    if f235(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m540 = f236(v4, f8(Str::lit("targetNamespace")));
                                            match m540 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m541 = f237(v5, v7, v11)?;
                                        match m541 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f238(v13.clone());
                                                let m542 = f239(v14.clone());
                                                match m542 {
                                                    None => {
                                                        let m543 = f240(v13.clone(), v14.clone());
                                                        match m543 {
                                                            None => {
                                                                let m547 = f241({
                                                                    let a545 = {
                                                                        let a544 = v14.clone();
                                                                        length_list(a544)
                                                                    };
                                                                    let a546 = 1u64;
                                                                    nat_add(a545, a546)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m547 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a548 = v14;
                                                                            length_list(a548)
                                                                        }, f217(f242(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
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
                Adt18::C1(_, _, _) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C2(_) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C3(_, _) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C4 => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f130(p556: List<Bytes>) -> R<bool> {
    let m555 = p556;
    {
        let mut m555 = m555;
        loop {
            let step = {
                let v0 = m555;
                let m551 = v0;
                match m551.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a553 = {
                            let a552 = f243(v1)?;
                            length_bytes(a552)
                        };
                        let a554 = 0u64;
                        if nat_eq(a553, a554) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m557 = v0;
    match m557.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f243(v1)?;
            let a559 = {
                let a558 = v3.clone();
                length_bytes(a558)
            };
            let a560 = 0u64;
            if nat_eq(a559, a560) {
                f131(v2)
            } else {
                let m561 = f244(v3.clone())?;
                match m561 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f160(v4.clone(), f8(Str::lit("id")));
                        if if f36(f160(v4, f8(Str::lit("schema"))), f161()) {
                            f76({
                                let a563 = {
                                    let a562 = v5.clone();
                                    length_bytes(a562)
                                };
                                let a564 = 0u64;
                                nat_eq(a563, a564)
                            })
                        } else {
                            false
                        } {
                            let m565 = f131(v2)?;
                            match m565 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f245(v5), {
                                        let a566 = v3;
                                        let a567 = Bytes::lit(&[0x0a]);
                                        append_bytes(a566, a567)
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

pub fn f132(p575: List<Bytes>) -> R<Option<Bytes>> {
    let m574 = p575;
    {
        let mut m574 = m574;
        loop {
            let step = {
                let v0 = m574;
                let m568 = v0;
                match m568.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m569 = f244(f243(v1)?)?;
                        match m569 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m570 = f162(v3.clone(), f8(Str::lit("source")));
                                match m570 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if if f36(f160(v4.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
                                            let a572 = {
                                                let a571 = f136(f160(v4, f8(Str::lit("url"))))?;
                                                length_bytes(a571)
                                            };
                                            let a573 = 0u64;
                                            nat_eq(a572, a573)
                                        } else {
                                            false
                                        } {
                                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f160(v3, f8(Str::lit("id"))))))
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
                Ok(next) => { m574 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f134() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f135() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f136(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f137(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a576 = v0.clone();
        length_bytes(a576)
    };
    let v3: u64 = {
        let a577 = v1.clone();
        length_bytes(a577)
    };
    let a578 = v3;
    let a579 = v2;
    if nat_le(a578, a579) {
        let m585 = {
            let a582 = v0;
            let a583 = {
                let a580 = v2;
                let a581 = v3;
                nat_sub(a580, a581)
            };
            let a584 = v3;
            slice_bytes(a582, a583, a584)
        };
        match m585 {
            None => {
                false
            }
            Some(v4) => {
                let a586 = v4;
                let a587 = v1;
                equal(a586, a587)
            }
        }
    } else {
        false
    }
}

pub fn f138(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m588 = v0;
    match m588.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f243(v2)?;
            let a590 = {
                let a589 = v4.clone();
                length_bytes(a589)
            };
            let a591 = 0u64;
            if nat_eq(a590, a591) {
                f138(v3, v1)
            } else {
                let m592 = f244(v4)?;
                match m592 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f246(v5.clone())
                        } else {
                            f247(v5.clone())
                        } {
                            let m593 = f138(v3, v1)?;
                            match m593 {
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

pub fn f139(v0: List<Bytes>) -> R<bool> {
    if f248(f156(v0.clone())?) {
        Ok(f76({
            let a595 = {
                let a594 = f249(f156(v0)?);
                length_list(a594)
            };
            let a596 = 0u64;
            nat_eq(a595, a596)
        }))
    } else {
        Ok(false)
    }
}

pub fn f140(v0: List<Bytes>) -> R<bool> {
    if f248(f143(v0.clone(), f8(Str::lit("requirement")))?) {
        if f248(f143(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f76({
                let a598 = {
                    let a597 = f147(f249(f143(v0.clone(), f8(Str::lit("requirement")))?), f249(f143(v0, f8(Str::lit("admission")))?));
                    length_list(a597)
                };
                let a599 = 0u64;
                nat_eq(a598, a599)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f141() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f142() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f143(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m600 = v0;
    match m600.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f243(v2)?;
            let a602 = {
                let a601 = v4.clone();
                length_bytes(a601)
            };
            let a603 = 0u64;
            if nat_eq(a602, a603) {
                f143(v3, v1)
            } else {
                let m604 = f244(v4)?;
                match m604 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f250(v5.clone());
                        let a606 = {
                            let a605 = v6.clone();
                            length_bytes(a605)
                        };
                        let a607 = 0u64;
                        if nat_eq(a606, a607) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m608 = f143(v3, v1.clone())?;
                            match m608 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f36(v6, v1) {
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

pub fn f144(p611: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m610 = p611;
    {
        let mut m610 = m610;
        loop {
            let step = {
                let v0 = m610;
                let m609 = v0;
                match m609.uncons() {
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
                Ok(next) => { m610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f146(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f218(Adt14::C3(std::rc::Rc::new(f251(v1))))?))
}

pub fn f147(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m612 = v0;
    match m612.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f147(v3, v1))
        }
    }
}

pub fn f148(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m613 = v0;
    match m613.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f243(v1)?;
            let a615 = {
                let a614 = v3.clone();
                length_bytes(a614)
            };
            let a616 = 0u64;
            if nat_eq(a615, a616) {
                f148(v2)
            } else {
                let m617 = f252(v3.clone())?;
                match m617 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f253(v4.clone()) {
                            let m618 = f148(v2)?;
                            match m618 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f190(), List::cons(f8(Str::lit("/")), List::cons(f254(f153(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a619 = v3;
                                        let a620 = Bytes::lit(&[0x0a]);
                                        append_bytes(a619, a620)
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

pub fn f149(v0: Adt1, v1: Bytes) -> R<Option<Adt19>> {
    f252(f71(v0, v1))
}

pub fn f150(p626: Adt19, v1: Bytes) -> Option<Adt19> {
    let m625 = p626;
    {
        let mut m625 = m625;
        loop {
            let step = {
                let v0 = m625;
                let m621 = v0;
                match m621 {
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
                    Adt19::C5(r622) => {
                        let v5 = (*r622).clone();
                        Ok::<Adt19, Option<Adt19>>(v5)
                    }
                    Adt19::C6 => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C7(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C8(v2, r623, r624) => {
                        let v3 = (*r623).clone();
                        let v4 = (*r624).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt19, Option<Adt19>>(Some(v3))
                        } else {
                            Ok::<Adt19, Option<Adt19>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m627 = f61(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m627.uncons() {
        None => {
            let m629 = f144(f107(List::cons(Adt14::C4(std::rc::Rc::new({
                let m628 = f255(v4.clone());
                match m628 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f106(f74(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m629 {
                None => {
                    f256(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f145(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f152(v0: Adt19) -> R<Bytes> {
    let a630 = f257(v0)?;
    let a631 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a630, a631))
}

pub fn f153(v0: Adt19, v1: Bytes) -> Bytes {
    let m632 = f258(f150(v0, v1));
    match m632 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f154(v0: List<Adt19>) -> List<Bytes> {
    let m633 = v0;
    match m633.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m634 = f258(Some(v1));
            match m634 {
                None => {
                    f154(v2)
                }
                Some(v3) => {
                    List::cons(v3, f154(v2))
                }
            }
        }
    }
}

pub fn f155(v0: Option<Adt19>) -> List<Adt19> {
    let m635 = v0;
    match m635 {
        None => {
            List::<Adt19>::nil()
        }
        Some(v1) => {
            f259(v1)
        }
    }
}

pub fn f156(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m636 = v0;
    match m636.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f243(v1)?;
            let a638 = {
                let a637 = v3.clone();
                length_bytes(a637)
            };
            let a639 = 0u64;
            if nat_eq(a638, a639) {
                f156(v2)
            } else {
                let m640 = f244(v3)?;
                match m640 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f205(v4.clone(), f8(Str::lit("agent"))) {
                            if f205(v4.clone(), f8(Str::lit("at"))) {
                                if f205(v4.clone(), f8(Str::lit("quote"))) {
                                    f260(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m641 = f156(v2)?;
                            match m641 {
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

pub fn f157(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f244(f71(v0, v1))
}

pub fn f158(p644: List<Adt14>) -> bool {
    let m643 = p644;
    {
        let mut m643 = m643;
        loop {
            let step = {
                let v0 = m643;
                let m642 = v0;
                match m642.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f160(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f87(f8(Str::lit("studio")), f73(f261(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m643 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: Option<Adt14>) -> List<Adt14> {
    let m645 = v0;
    match m645 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f262(v1)
        }
    }
}

pub fn f160(v0: Adt14, v1: Bytes) -> Bytes {
    let m646 = f163(f162(v0, v1));
    match m646 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f161() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f162(p652: Adt14, v1: Bytes) -> Option<Adt14> {
    let m651 = p652;
    {
        let mut m651 = m651;
        loop {
            let step = {
                let v0 = m651;
                let m647 = v0;
                match m647 {
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
                    Adt14::C4(r648) => {
                        let v5 = (*r648).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r649, r650) => {
                        let v3 = (*r649).clone();
                        let v4 = (*r650).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m651 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: Option<Adt14>) -> Option<Bytes> {
    let m653 = v0;
    match m653 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m654 = v1;
            match m654 {
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

pub fn f164(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f120(f71(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f263(v1, v4)?)?), List::cons(f121(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f165(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
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
    let m660 = f264(v0.clone(), v3);
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
                }, List::cons(f32(v0, v4, {
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

pub fn f166(v0: Fn1, p674: List<Bytes>, p675: List<Bytes>) -> List<Bytes> {
    let m673 = (p674, p675);
    {
        let mut m673 = m673;
        loop {
            let step = {
                let (v1, v2) = m673;
                let m671 = v2;
                match m671.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c672: Fn1 = v0.clone();
                            c672.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m673 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f168(p680: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m679 = p680;
    {
        let mut m679 = m679;
        loop {
            let step = {
                let v0 = m679;
                let m676 = v0;
                match m676.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h677, _) = v2;
                            h677
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h678) = v2;
                                h678
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Adt1, v1: Bytes) -> bool {
    let m682 = f168(match v0 {
        Adt1::C0(h681, _, _) => {
            h681
        }
    }, v1);
    match m682 {
        None => {
            false
        }
        Some(v2) => {
            let m683 = v2;
            match m683 {
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

pub fn f170(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m684 = v1;
    match m684.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f265(v3.clone())
            } else {
                f76(f265(v3.clone()))
            } {
                List::cons(f133(v0.clone(), v3), f170(v0, v4, v2))
            } else {
                f170(v0, v4, v2)
            }
        }
    }
}

pub fn f171(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a685 = v0.clone();
        length_bytes(a685)
    };
    let v3: u64 = {
        let a686 = v1.clone();
        length_bytes(a686)
    };
    let v4: u64 = {
        let (h687, _) = f266(Fn3::F267(v1.clone(), v3, v0.clone(), v2), {
            let a688 = v2;
            let a689 = 1u64;
            nat_add(a688, a689)?
        }, 0u64)?;
        h687
    };
    if f222(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f172() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f173(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f268(match v0.clone() {
        Adt12::C0(h690, _, _, _, _) => {
            h690
        }
    }, 0u64)?, f268(match v0.clone() {
        Adt12::C0(_, h691, _, _, _) => {
            h691
        }
    }, 0u64)?, f268(match v0.clone() {
        Adt12::C0(_, _, h692, _, _) => {
            h692
        }
    }, 0u64)?, f268(match v0 {
        Adt12::C0(_, _, _, h693, _) => {
            h693
        }
    }, 0u64)?))
}

pub fn f174(v0: Adt12) -> List<Bytes> {
    f65(match v0.clone() {
        Adt12::C0(h694, _, _, _, _) => {
            h694
        }
    }, f65(match v0.clone() {
        Adt12::C0(_, _, h695, _, _) => {
            h695
        }
    }, f65(match v0.clone() {
        Adt12::C0(_, _, _, h696, _) => {
            h696
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h697) => {
            h697
        }
    })))
}

pub fn f175(v0: Adt16, p703: List<u64>, p704: List<Adt20>) -> Option<u64> {
    let m702 = (p703, p704);
    {
        let mut m702 = m702;
        loop {
            let step = {
                let (v1, v2) = m702;
                let m698 = v2;
                match m698.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f269(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h699) => {
                                h699
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h700, _) => {
                                    h700
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h701, _) => {
                                    h701
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m705 = v1;
    match m705.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f270(v0.clone(), v3)?), f176(v0, v4, {
                let a706 = v2;
                let a707 = 1u64;
                nat_add(a706, a707)?
            })?))
        }
    }
}

pub fn f177(p711: List<Bytes>, p712: u64) -> Bytes {
    let m710 = (p711, p712);
    {
        let mut m710 = m710;
        loop {
            let step = {
                let (v0, v1) = m710;
                let m708 = v0;
                match m708.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m709 = v1;
                        if m709 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m709 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m710 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f271(v0.clone(), v1.clone()) {
        if f271(v1, v0) {
            f272(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f179(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m713 = v0;
    match m713.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f273(v2, v1.clone())?, f179(v3, v1)?))
        }
    }
}

pub fn f180(v0: List<Adt14>) -> List<u64> {
    let m714 = v0;
    match m714.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f274(v1), f180(v2))
        }
    }
}

pub fn f181(p717: List<u64>, v1: List<u64>) -> Option<u64> {
    let m716 = p717;
    {
        let mut m716 = m716;
        loop {
            let step = {
                let v0 = m716;
                let m715 = v0;
                match m715.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f275(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f182(p720: List<Bytes>, p721: List<Bytes>) -> Option<Bytes> {
    let m719 = (p720, p721);
    {
        let mut m719 = m719;
        loop {
            let step = {
                let (v0, v1) = m719;
                let m718 = v0;
                match m718.uncons() {
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
                Ok(next) => { m719 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m722 = v0;
    match m722.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m723 = f276(v2, v1.clone())?;
            match m723 {
                None => {
                    f183(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f183(v3, v1)?))
                }
            }
        }
    }
}

pub fn f184(v0: List<Bytes>) -> List<Bytes> {
    f166(Fn1::F277, List::<Bytes>::nil(), v0)
}

pub fn f185(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f278(v0, v1)
}

pub fn f186(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f279(f280(f281(v0))?))
}

pub fn f187(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m724 = f157(v0, v2)?;
    match m724 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f282(v1, f160(v3.clone(), f8(Str::lit("id"))))?;
            let m725 = v4.clone();
            match m725.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m727 = f163(f162({
                        let m726 = f162(v3.clone(), f8(Str::lit("source")));
                        match m726 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m727 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f76(f87(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f212(f283(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f188(p730: List<Bytes>, p731: List<Bytes>) -> List<Bytes> {
    let m729 = (p730, p731);
    {
        let mut m729 = m729;
        loop {
            let step = {
                let (v0, v1) = m729;
                let m728 = v0;
                match m728.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f284(v0, v1)
}

pub fn f190() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f191(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m732 = v1;
    match m732.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt19 = f285(v0.clone(), v3)?;
            if f87(f286(f153(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f287(v5)?, f191(v0, v4, v2)?))
            } else {
                f191(v0, v4, v2)
            }
        }
    }
}

pub fn f192(v0: Adt1, p736: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m735 = p736;
    {
        let mut m735 = m735;
        loop {
            let step = {
                let v1 = m735;
                let m733 = v1;
                match m733.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt19 = f285(v0.clone(), v3.clone())?;
                        if if f87(f286(f153(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f36(f152({
                                let m734 = f150(v5.clone(), f8(Str::lit("schema")));
                                match m734 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f71(v0.clone(), f287(v5)?))
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
                Ok(next) => { m735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f76(f77(match v0.clone() {
        Adt1::C0(_, h737, _) => {
            h737
        }
    }, f141())) {
        Ok(Adt11::C1(List::cons(f141(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f85(f79(f288(f81(match v0.clone() {
            Adt1::C0(_, h738, _) => {
                h738
            }
        }, f141())))?, v1);
        let m739 = f61(v0.clone(), List::cons(f90(), List::cons(f118(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m739.uncons() {
            None => {
                let v5: (bool, Bytes) = f289(v0.clone(), v2)?;
                let (h740, _) = v5;
                if h740 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h741) = v5;
                        h741
                    }))
                } else {
                    let m742 = f89(v0.clone());
                    match m742.uncons() {
                        None => {
                            let m743 = f91(v0.clone())?;
                            match m743.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h744) = v5;
                                        h744
                                    }, List::cons(f7(f79(f290(f103(v0.clone())?, f111(v0.clone(), f112(v0.clone())?)?))?), List::cons(f291(v0)?, List::<Bytes>::nil()))))))
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

pub fn f194(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f292(f74(v0, f90())?)?)))
}

pub fn f195() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f196(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt23>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m745 = v1;
    match m745.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f65(f293(v5.clone(), v2.clone()), f294(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f295(v0.clone(), v5.clone()), List::cons(if f87(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a747 = {
                    let a746 = v7.clone();
                    length_list(a746)
                };
                let a748 = 0u64;
                if nat_eq(a747, a748) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f122(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f196(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f197(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt23>> {
    let v3: List<(Bytes, Bytes)> = f296(v1);
    let v4: List<(Bytes, Bytes)> = f297(v0, v2);
    f298(v3.clone(), f92(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f198(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m749 = v1;
    match m749.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f205(v2.clone(), f8(Str::lit("domain"))) {
                f299(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f160(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f160(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m750 = f162(v2.clone(), f8(Str::lit("role")));
                    match m750 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f160(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f160(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f160(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f300(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m751 = f162(v2.clone(), f8(Str::lit("origin")));
                    match m751 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f160(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m752 = f162(v2, f8(Str::lit("evidence")));
                    match m752 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f160(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f198(v0, v3))
            } else {
                f198(v0, v3)
            }
        }
    }
}

pub fn f199(v0: List<Adt14>) -> List<Bytes> {
    let m753 = v0;
    match m753.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f160(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f160(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f160(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f199(v2))
        }
    }
}

pub fn f200(v0: List<Adt14>) -> List<Bytes> {
    let m754 = v0;
    match m754.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f160(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f301(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f200(v2))
        }
    }
}

pub fn f201(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m755 = v0;
    match m755.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f160(v1.clone(), f8(Str::lit("subject"))), f160(v1, f8(Str::lit("object")))), f201(v2))
        }
    }
}

pub fn f202(v0: List<Adt23>) -> List<(Bytes, Bytes)> {
    let m756 = v0;
    match m756.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt23::C0(h757, _, _) => {
                    h757
                }
            }, match v1 {
                Adt23::C0(_, _, h758) => {
                    h758
                }
            }), f202(v2))
        }
    }
}

pub fn f203(p765: List<Bytes>, v1: List<(Bytes, Bytes)>, p766: u64) -> R<List<Bytes>> {
    let m764 = (p765, p766);
    {
        let mut m764 = m764;
        loop {
            let step = {
                let (v0, v2) = m764;
                let m759 = v2;
                if m759 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m759 - 1;
                    let v4: List<Bytes> = f79(f65(v0.clone(), f302(v0.clone(), v1.clone())))?;
                    let a762 = {
                        let a760 = v4.clone();
                        length_list(a760)
                    };
                    let a763 = {
                        let a761 = v0.clone();
                        length_list(a761)
                    };
                    if nat_eq(a762, a763) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m764 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: List<Adt14>) -> List<Bytes> {
    let m767 = v0;
    match m767.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m768 = f162(v1, f8(Str::lit("std")));
            match m768 {
                None => {
                    f204(v2)
                }
                Some(v3) => {
                    List::cons(f160(v3.clone(), f8(Str::lit("authority"))), if f205(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f160(v3, f8(Str::lit("row"))), f204(v2))
                    } else {
                        f204(v2)
                    })
                }
            }
        }
    }
}

pub fn f205(v0: Adt14, v1: Bytes) -> bool {
    f76({
        let a770 = {
            let a769 = f160(v0, v1);
            length_bytes(a769)
        };
        let a771 = 0u64;
        nat_eq(a770, a771)
    })
}

pub fn f206(v0: Adt15, v1: Adt14) -> bool {
    if f303(v0.clone(), v1.clone(), f160(v1.clone(), f8(Str::lit("subject")))) {
        f303(v0, v1.clone(), f160(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f207(v0: Bytes, v1: Bytes) -> bool {
    let m776 = {
        let a773 = v0;
        let a774 = 0u64;
        let a775 = {
            let a772 = v1.clone();
            length_bytes(a772)
        };
        slice_bytes(a773, a774, a775)
    };
    match m776 {
        None => {
            false
        }
        Some(v2) => {
            let a777 = v2;
            let a778 = v1;
            equal(a777, a778)
        }
    }
}

pub fn f208(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m779 = v1;
    match m779.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f137(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f133(v0.clone(), v2), f208(v0, v3))
            } else {
                f208(v0, v3)
            }
        }
    }
}

pub fn f209(v0: Bytes, p837: u64, p838: u64, p839: Adt17) -> R<Adt17> {
    let m836 = (p837, (p838, p839));
    {
        let mut m836 = m836;
        loop {
            let step = {
                let (v1, (v2, v3)) = m836;
                let m780 = v2;
                if m780 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m780 - 1;
                    let v5: u64 = f304(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h781, _, _, _) => {
                            h781
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a782 = v1;
                            let a783 = 1u64;
                            nat_add(a782, a783)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h784, _, _) => {
                                h784
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h785, _) => {
                                h785
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h786) => {
                                h786
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h787, _, _, _, _) => {
                                h787
                            }
                        } {
                            let a788 = v5;
                            let a789 = 34u64;
                            if nat_eq(a788, a789) {
                                let a792 = f304(v0.clone(), {
                                    let a790 = v1;
                                    let a791 = 1u64;
                                    nat_add(a790, a791)?
                                })?;
                                let a793 = 34u64;
                                if nat_eq(a792, a793) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a794 = v1;
                                        let a795 = 1u64;
                                        nat_add(a794, a795)?
                                    }, (v4, Adt17::C0(true, true, f305(match v3.clone() {
                                        Adt17::C0(_, _, h796, _, _) => {
                                            h796
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h797, _) => {
                                            h797
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h798) => {
                                            h798
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a799 = v1;
                                        let a800 = 1u64;
                                        nat_add(a799, a800)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h801, _, _) => {
                                            h801
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h802, _) => {
                                            h802
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h803) => {
                                            h803
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a804 = v1;
                                    let a805 = 1u64;
                                    nat_add(a804, a805)?
                                }, (v4, Adt17::C0(true, false, f305(match v3.clone() {
                                    Adt17::C0(_, _, h806, _, _) => {
                                        h806
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h807, _) => {
                                        h807
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h808) => {
                                        h808
                                    }
                                }))))
                            }
                        } else {
                            let a809 = v5;
                            let a810 = 34u64;
                            if nat_eq(a809, a810) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a811 = v1;
                                    let a812 = 1u64;
                                    nat_add(a811, a812)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h813, _, _) => {
                                        h813
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h814, _) => {
                                        h814
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h815) => {
                                        h815
                                    }
                                }))))
                            } else {
                                let a816 = v5;
                                let a817 = 44u64;
                                if nat_eq(a816, a817) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a818 = v1;
                                        let a819 = 1u64;
                                        nat_add(a818, a819)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h820, _, _) => {
                                            h820
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h821, _) => {
                                            h821
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h822) => {
                                            h822
                                        }
                                    }))))
                                } else {
                                    let a823 = v5;
                                    let a824 = 13u64;
                                    if nat_eq(a823, a824) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a825 = v1;
                                            let a826 = 1u64;
                                            nat_add(a825, a826)?
                                        }, (v4, v3)))
                                    } else {
                                        let a827 = v5;
                                        let a828 = 10u64;
                                        if nat_eq(a827, a828) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a829 = v1;
                                                let a830 = 1u64;
                                                nat_add(a829, a830)?
                                            }, (v4, f210(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a831 = v1;
                                                let a832 = 1u64;
                                                nat_add(a831, a832)?
                                            }, (v4, Adt17::C0(false, false, f305(match v3.clone() {
                                                Adt17::C0(_, _, h833, _, _) => {
                                                    h833
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h834, _) => {
                                                    h834
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h835) => {
                                                    h835
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
                Ok(next) => { m836 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f210(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f306(List::cons(match v0.clone() {
        Adt17::C0(_, _, h840, _, _) => {
            h840
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h841, _) => {
            h841
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h842) => {
            h842
        }
    }))
}

pub fn f211(p845: List<List<Bytes>>, p846: List<List<Bytes>>) -> List<List<Bytes>> {
    let m844 = (p845, p846);
    {
        let mut m844 = m844;
        loop {
            let step = {
                let (v0, v1) = m844;
                let m843 = v0;
                match m843.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m844 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f307(v0.clone(), 0u64, {
        let a847 = v0;
        length_bytes(a847)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f213(v0: Adt24) -> Bytes {
    let a868 = f308(match v0.clone() {
        Adt24::C0(h848, _, _, _, _, _, _, _) => {
            h848
        }
    });
    let a869 = {
        let a866 = f308(match v0.clone() {
            Adt24::C0(_, h849, _, _, _, _, _, _) => {
                h849
            }
        });
        let a867 = {
            let a864 = f308(match v0.clone() {
                Adt24::C0(_, _, h850, _, _, _, _, _) => {
                    h850
                }
            });
            let a865 = {
                let a862 = f308(match v0.clone() {
                    Adt24::C0(_, _, _, h851, _, _, _, _) => {
                        h851
                    }
                });
                let a863 = {
                    let a860 = f308(match v0.clone() {
                        Adt24::C0(_, _, _, _, h852, _, _, _) => {
                            h852
                        }
                    });
                    let a861 = {
                        let a858 = f308(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h853, _, _) => {
                                h853
                            }
                        });
                        let a859 = {
                            let a856 = f308(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h854, _) => {
                                    h854
                                }
                            });
                            let a857 = f308(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h855) => {
                                    h855
                                }
                            });
                            append_bytes(a856, a857)
                        };
                        append_bytes(a858, a859)
                    };
                    append_bytes(a860, a861)
                };
                append_bytes(a862, a863)
            };
            append_bytes(a864, a865)
        };
        append_bytes(a866, a867)
    };
    append_bytes(a868, a869)
}

pub fn f214(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a870 = v0.clone();
        length_bytes(a870)
    };
    let v2: u64 = {
        let a876 = {
            let a873 = {
                let a871 = v1;
                let a872 = 72u64;
                nat_add(a871, a872)?
            };
            let a874 = 64u64;
            let a875 = 0u64;
            nat_quot(a873, a874, a875)
        };
        let a877 = 64u64;
        nat_mul(a876, a877)?
    };
    let (_, h878) = f309(Fn4::F310(v0, v1, v2), {
        let a879 = v2;
        let a880 = 64u64;
        let a881 = 0u64;
        nat_quot(a879, a880, a881)
    }, (0u64, f311()))?;
    Ok(h878)
}

pub fn f215(p884: List<List<Bytes>>, v1: Bytes, p885: List<Bytes>) -> R<List<Bytes>> {
    let m883 = (p884, p885);
    {
        let mut m883 = m883;
        loop {
            let step = {
                let (v0, v2) = m883;
                let m882 = v0;
                match m882.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f312(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f217(f313(v3), Bytes::lit(&[0x0a])), v2)))
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

pub fn f216(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f314(v0)?))
}

pub fn f217(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m886 = v0;
    match m886.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m887 = v3.clone();
            match m887.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a890 = v2;
                    let a891 = {
                        let a888 = v1.clone();
                        let a889 = f217(v3, v1);
                        append_bytes(a888, a889)
                    };
                    append_bytes(a890, a891)
                }
            }
        }
    }
}

pub fn f218(v0: Adt14) -> R<Bytes> {
    let a892 = f315(v0)?;
    let a893 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a892, a893))
}

pub fn f219(v0: List<Bytes>) -> Adt14 {
    let m894 = v0;
    match m894.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f219(v2)))
        }
    }
}

pub fn f220(v0: Fn2, p898: Option<Bytes>, p899: List<Bytes>) -> Option<Bytes> {
    let m897 = (p898, p899);
    {
        let mut m897 = m897;
        loop {
            let step = {
                let (v1, v2) = m897;
                let m895 = v2;
                match m895.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c896: Fn2 = v0.clone();
                            c896.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m897 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m900 = v1;
    match m900 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a903 = {
                    let a901 = v3;
                    let a902 = v0;
                    append_bytes(a901, a902)
                };
                let a904 = v2;
                append_bytes(a903, a904)
            })
        }
    }
}

pub fn f222(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m909 = {
        let a906 = v0;
        let a907 = v2;
        let a908 = {
            let a905 = v1.clone();
            length_bytes(a905)
        };
        slice_bytes(a906, a907, a908)
    };
    match m909 {
        None => {
            false
        }
        Some(v3) => {
            let a910 = v3;
            let a911 = v1;
            equal(a910, a911)
        }
    }
}

pub fn f223(v0: Bytes) -> R<Bytes> {
    let (_, h912) = {
        let (h913, _) = f316(Fn5::F317(v0.clone()), {
            let a915 = {
                let a914 = v0;
                length_bytes(a914)
            };
            let a916 = 1u64;
            nat_add(a915, a916)?
        }, (0u64, Bytes::lit(&[])))?;
        h913
    };
    Ok(h912)
}

pub fn f224(v0: Bytes) -> R<u64> {
    let (h917, _) = f266(Fn3::F318(v0.clone()), {
        let a919 = {
            let a918 = v0;
            length_bytes(a918)
        };
        let a920 = 1u64;
        nat_add(a919, a920)?
    }, 0u64)?;
    Ok(h917)
}

pub fn f225(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a921 = v0;
        utf8_encode(a921)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f127(v1)?, List::<Bytes>::nil())))))
}

pub fn f226(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f228(v0.clone(), 0u64, Str::lit("<?xml")) {
        f319(f229(v0.clone(), 5u64)?)
    } else {
        false
    } {
        let m923 = f320(v0.clone(), {
            let a922 = Str::lit("?>");
            utf8_encode(a922)
        }, 5u64)?;
        match m923 {
            None => {
                Ok(Err::<u64, Bytes>(f225(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m925 = f320(f32(v0.clone(), 0u64, v1), {
                    let a924 = Str::lit("encoding");
                    utf8_encode(a924)
                }, 5u64)?;
                match m925 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a926 = v1;
                            let a927 = 2u64;
                            nat_add(a926, a927)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f321(v0.clone(), {
                            let a928 = v2;
                            let a929 = 8u64;
                            nat_add(a928, a929)?
                        })?;
                        let v4: u64 = f321(v0.clone(), {
                            let a930 = v3;
                            let a931 = 1u64;
                            nat_add(a930, a931)?
                        })?;
                        let _: u64 = f229(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m934 = f320(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a932 = v4;
                                let a933 = 1u64;
                                nat_add(a932, a933)?
                            })?;
                            match m934 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f322(f32(v0, {
                            let a935 = v4;
                            let a936 = 1u64;
                            nat_add(a935, a936)?
                        }, {
                            let a939 = v7;
                            let a940 = {
                                let a937 = v4;
                                let a938 = 1u64;
                                nat_add(a937, a938)?
                            };
                            nat_sub(a939, a940)
                        }))? {
                            Ok(Ok::<u64, Bytes>({
                                let a941 = v1;
                                let a942 = 2u64;
                                nat_add(a941, a942)?
                            }))
                        } else {
                            Ok(Err::<u64, Bytes>(f225(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f227(v0: Bytes, p947: u64, p948: u64) -> R<Result<u64, Bytes>> {
    let m946 = (p947, p948);
    {
        let mut m946 = m946;
        loop {
            let step = {
                let (v1, v2) = m946;
                let m943 = v2;
                if m943 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(f225(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m943 - 1;
                    let v4: u64 = f321(v0.clone(), v1)?;
                    if f228(v0.clone(), v4, Str::lit("<!--")) {
                        let m944 = f323(v0.clone(), v4)?;
                        match m944 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v5)))
                            }
                        }
                    } else {
                        if f228(v0.clone(), v4, Str::lit("<?")) {
                            let m945 = f324(v0.clone(), v4)?;
                            match m945 {
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
                Ok(next) => { m946 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f228(v0: Bytes, v1: u64, v2: Str) -> bool {
    f222(v0, {
        let a949 = v2;
        utf8_encode(a949)
    }, v1)
}

pub fn f229(v0: Bytes, v1: u64) -> R<u64> {
    f325(v0, v1)
}

pub fn f230(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt18, u64), Bytes>> {
    let m950 = v2;
    if m950 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m950 - 1;
        let a951 = v3;
        let a952 = 0u64;
        if nat_eq(a951, a952) {
            let v5: u64 = f326(v0.clone(), {
                let a953 = v1;
                let a954 = 1u64;
                nat_add(a953, a954)?
            })?;
            let a957 = v5;
            let a958 = {
                let a955 = v1;
                let a956 = 1u64;
                nat_add(a955, a956)?
            };
            if nat_eq(a957, a958) {
                Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a959 = v1;
                    let a960 = 1u64;
                    nat_add(a959, a960)?
                }, {
                    let a963 = v5;
                    let a964 = {
                        let a961 = v1;
                        let a962 = 1u64;
                        nat_add(a961, a962)?
                    };
                    nat_sub(a963, a964)
                });
                let m965 = f327(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m965 {
                    Ok(v8) => {
                        if f228(v0.clone(), {
                            let (_, h966) = v8.clone();
                            h966
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                let (h967, _) = v8;
                                h967
                            }), std::rc::Rc::new(Adt18::C4)), {
                                let a969 = {
                                    let (_, h968) = v8;
                                    h968
                                };
                                let a970 = 2u64;
                                nat_add(a969, a970)?
                            })))
                        } else {
                            let m974 = f230(v0.clone(), {
                                let a972 = {
                                    let (_, h971) = v8;
                                    h971
                                };
                                let a973 = 1u64;
                                nat_add(a972, a973)?
                            }, v4, 1u64)?;
                            match m974 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a976 = {
                                            let (_, h975) = v10.clone();
                                            h975
                                        };
                                        let a977 = 2u64;
                                        nat_add(a976, a977)?
                                    };
                                    let v12: u64 = f326(v0.clone(), v11)?;
                                    if !f36(f32(v0.clone(), v11, {
                                        let a978 = v12;
                                        let a979 = v11;
                                        nat_sub(a978, a979)
                                    }), v6.clone()) {
                                        Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h980) = v10;
                                            h980
                                        })?))
                                    } else {
                                        let v13: u64 = f321(v0.clone(), v12)?;
                                        let a981 = f229(v0, v13)?;
                                        let a982 = 62u64;
                                        if nat_eq(a981, a982) {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                                let (h983, _) = v8;
                                                h983
                                            }), std::rc::Rc::new({
                                                let (h984, _) = v10;
                                                h984
                                            })), {
                                                let a985 = v13;
                                                let a986 = 1u64;
                                                nat_add(a985, a986)?
                                            })))
                                        } else {
                                            Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
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
            let a987 = f229(v0.clone(), v1)?;
            let a988 = 256u64;
            if nat_eq(a987, a988) {
                Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f228(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v1)))
                } else {
                    if f228(v0.clone(), v1, Str::lit("<!--")) {
                        let m989 = f323(v0.clone(), v1)?;
                        match m989 {
                            Ok(v15) => {
                                f230(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(Err::<(Adt18, u64), Bytes>(v14))
                            }
                        }
                    } else {
                        if f228(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m993 = f320(v0.clone(), {
                                let a990 = Str::lit("]]>");
                                utf8_encode(a990)
                            }, {
                                let a991 = v1;
                                let a992 = 9u64;
                                nat_add(a991, a992)?
                            })?;
                            match m993 {
                                None => {
                                    Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m996 = f230(v0.clone(), {
                                        let a994 = v16;
                                        let a995 = 3u64;
                                        nat_add(a994, a995)?
                                    }, v4, 1u64)?;
                                    match m996 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2(f32(v0, {
                                                let a997 = v1;
                                                let a998 = 9u64;
                                                nat_add(a997, a998)?
                                            }, {
                                                let a1001 = v16;
                                                let a1002 = {
                                                    let a999 = v1;
                                                    let a1000 = 9u64;
                                                    nat_add(a999, a1000)?
                                                };
                                                nat_sub(a1001, a1002)
                                            }))), std::rc::Rc::new({
                                                let (h1003, _) = v18;
                                                h1003
                                            })), {
                                                let (_, h1004) = v18;
                                                h1004
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(Err::<(Adt18, u64), Bytes>(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f228(v0.clone(), v1, Str::lit("<?")) {
                                let m1005 = f324(v0.clone(), v1)?;
                                match m1005 {
                                    Ok(v20) => {
                                        f230(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v19))
                                    }
                                }
                            } else {
                                if f228(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a1006 = f229(v0.clone(), v1)?;
                                    let a1007 = 60u64;
                                    if nat_eq(a1006, a1007) {
                                        let m1008 = f230(v0.clone(), v1, v4, 0u64)?;
                                        match m1008 {
                                            Ok(v22) => {
                                                let m1010 = f230(v0, {
                                                    let (_, h1009) = v22;
                                                    h1009
                                                }, v4, 1u64)?;
                                                match m1010 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new({
                                                            let (h1011, _) = v22;
                                                            h1011
                                                        }), std::rc::Rc::new({
                                                            let (h1012, _) = v24;
                                                            h1012
                                                        })), {
                                                            let (_, h1013) = v24;
                                                            h1013
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
                                        let m1014 = f328(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m1014 {
                                            Ok(v26) => {
                                                let m1016 = f230(v0, {
                                                    let (_, h1015) = v26;
                                                    h1015
                                                }, v4, 1u64)?;
                                                match m1016 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2({
                                                            let (h1017, _) = v26;
                                                            h1017
                                                        })), std::rc::Rc::new({
                                                            let (h1018, _) = v28;
                                                            h1018
                                                        })), {
                                                            let (_, h1019) = v28;
                                                            h1019
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

pub fn f231(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1020 = v2;
    let a1021 = 0u64;
    if nat_eq(a1020, a1021) {
        Ok(f329(v0, 0u64))
    } else {
        let (_, h1022) = {
            let (h1023, _) = f316(Fn5::F330(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1023
        };
        Ok(h1022)
    }
}

pub fn f232() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f233(v0: Adt18, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m1024 = v0;
    match m1024 {
        Adt18::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C1(v2, v3, r1029) => {
            let v4 = (*r1029).clone();
            let m1025 = f233(v4, v1);
            match m1025 {
                Ok(v6) => {
                    if f36(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f36(v3.clone(), f331()) {
                            true
                        } else {
                            f36(v3.clone(), f332())
                        } {
                            Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f207(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a1027 = {
                                    let a1026 = v2;
                                    length_bytes(a1026)
                                };
                                let a1028 = 6u64;
                                nat_sub(a1027, a1028)
                            });
                            if f36(v7.clone(), f8(Str::lit("xmlns"))) {
                                Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f36(v7.clone(), f8(Str::lit("xml"))) {
                                    !f36(v3.clone(), f331())
                                } else {
                                    false
                                } {
                                    Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f36(v3.clone(), f331()) {
                                        true
                                    } else {
                                        f36(v3.clone(), f332())
                                    } {
                                        !f36(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f36(v3.clone(), Bytes::lit(&[])) {
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
        Adt18::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f234(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f333(v1.clone())?;
    if f36(v2.clone(), Bytes::lit(&[])) {
        let m1030 = f334(v0, Bytes::lit(&[]));
        match m1030 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f36(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f331(), f335(v1)?)))
        } else {
            let m1031 = f334(v0, v2.clone());
            match m1031 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f335(v1)?)))
                }
            }
        }
    }
}

pub fn f235(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f36({
        let (h1032, _) = v0;
        h1032
    }, f336()) {
        f36({
            let (_, h1033) = v0;
            h1033
        }, {
            let a1034 = v1;
            utf8_encode(a1034)
        })
    } else {
        false
    }
}

pub fn f236(p1038: Adt18, v1: Bytes) -> Option<Bytes> {
    let m1037 = p1038;
    {
        let mut m1037 = m1037;
        loop {
            let step = {
                let v0 = m1037;
                let m1035 = v0;
                match m1035 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C1(v2, v3, r1036) => {
                        let v4 = (*r1036).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt18, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt18, Option<Bytes>>(v4)
                        }
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C3(_, _) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C4 => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m1037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Adt18, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m1039 = v0;
    match m1039 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C3(r1050, r1051) => {
            let v3 = (*r1050).clone();
            let v4 = (*r1051).clone();
            let m1040 = f237(v4, v1.clone(), v2.clone())?;
            match m1040 {
                Ok(v6) => {
                    let m1041 = v3;
                    match m1041 {
                        Adt18::C0(v7, r1048, r1049) => {
                            let v8 = (*r1048).clone();
                            let v9 = (*r1049).clone();
                            let m1042 = f233(v8.clone(), v1);
                            match m1042 {
                                Ok(v11) => {
                                    let m1043 = f234(v11.clone(), v7)?;
                                    match m1043 {
                                        Ok(v13) => {
                                            if f235(v13, Str::lit("simpleType")) {
                                                let m1044 = f236(v8, f8(Str::lit("name")));
                                                match m1044 {
                                                    None => {
                                                        Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m1045 = f337(v14.clone(), v9, v11, v2)?;
                                                        match m1045 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f254(v14)?, ({
                                                                    let (_, h1046) = v16;
                                                                    h1046
                                                                }, {
                                                                    let (h1047, _) = v16;
                                                                    h1047
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
                        Adt18::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v5))
                }
            }
        }
        Adt18::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f238(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m1052 = v0;
    match m1052.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1053, _) = v1;
                h1053
            }, f238(v2))
        }
    }
}

pub fn f239(p1056: List<Bytes>) -> Option<Bytes> {
    let m1055 = p1056;
    {
        let mut m1055 = m1055;
        loop {
            let step = {
                let v0 = m1055;
                let m1054 = v0;
                match m1054.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f87(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1055 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(p1062: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m1061 = p1062;
    {
        let mut m1061 = m1061;
        loop {
            let step = {
                let v0 = m1061;
                let m1057 = v0;
                match m1057.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1060 = f338({
                            let (h1058, _) = {
                                let (_, h1059) = v2;
                                h1059
                            };
                            h1058
                        }, v1.clone());
                        match m1060 {
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
                Ok(next) => { m1061 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(p1073: u64, p1074: List<(Bytes, (List<Bytes>, Bytes))>, p1075: List<Bytes>, p1076: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1072 = (p1073, (p1074, (p1075, p1076)));
    {
        let mut m1072 = m1072;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1072;
                let m1063 = v0;
                if m1063 == 0 {
                    let m1064 = v1;
                    match m1064.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m1063 - 1;
                    let m1065 = v1.clone();
                    match m1065.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m1066 = f339(v1, v2.clone());
                            match m1066 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h1067) = v9.clone();
                                        h1067
                                    }, (List::cons({
                                        let (h1068, _) = {
                                            let (h1069, _) = v9.clone();
                                            h1069
                                        };
                                        h1068
                                    }, v2), List::cons({
                                        let (_, h1070) = {
                                            let (h1071, _) = v9;
                                            h1071
                                        };
                                        h1070
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1072 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(p1079: List<Bytes>, p1080: List<Bytes>) -> List<Bytes> {
    let m1078 = (p1079, p1080);
    {
        let mut m1078 = m1078;
        loop {
            let step = {
                let (v0, v1) = m1078;
                let m1077 = v0;
                match m1077.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f314(v0.clone())?;
    let v2: u64 = f340(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a1081 = v1;
        let a1082 = v2;
        nat_sub(a1081, a1082)
    }))
}

pub fn f244(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f341(v0)?;
    let m1086 = f342(v1.clone(), 0u64, {
        let a1084 = {
            let a1083 = v1.clone();
            length_bytes(a1083)
        };
        let a1085 = 2u64;
        nat_add(a1084, a1085)?
    }, 0u64)?;
    match m1086 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a1089 = {
                let (_, h1087) = v2;
                h1087
            };
            let a1090 = {
                let a1088 = v1;
                length_bytes(a1088)
            };
            if nat_eq(a1089, a1090) {
                Ok(Some({
                    let (h1091, _) = v2;
                    h1091
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f245(v0: Bytes) -> Bytes {
    f7(List::cons(f78(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f246(v0: Adt14) -> bool {
    if f205(v0.clone(), f8(Str::lit("domain"))) {
        if f205(v0.clone(), f8(Str::lit("row"))) {
            if f87(f160(v0.clone(), f8(Str::lit("relation"))), f343()) {
                if f344(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f344(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f344(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f344(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f247(v0: Adt14) -> bool {
    if f205(v0.clone(), f8(Str::lit("subject"))) {
        if f205(v0.clone(), f8(Str::lit("object"))) {
            if f205(v0.clone(), f8(Str::lit("evidence"))) {
                f87(f160(v0, f8(Str::lit("relation"))), f345())
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

pub fn f248(v0: Option<List<Adt14>>) -> bool {
    let m1092 = v0;
    match m1092 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f249(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1093 = v0;
    match m1093 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f250(v0: Adt14) -> Bytes {
    if f205(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f205(v0.clone(), f8(Str::lit("id"))) {
            if f205(v0.clone(), f8(Str::lit("statement"))) {
                if f205(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f260(v0, f8(Str::lit("evidence")))
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

pub fn f251(v0: List<Adt14>) -> Adt14 {
    let m1094 = v0;
    match m1094.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f251(v2)))
        }
    }
}

pub fn f252(v0: Bytes) -> R<Option<Adt19>> {
    let v1: Bytes = f341(v0)?;
    let m1098 = f346(v1.clone(), 0u64, {
        let a1096 = {
            let a1095 = v1.clone();
            length_bytes(a1095)
        };
        let a1097 = 2u64;
        nat_add(a1096, a1097)?
    }, 0u64)?;
    match m1098 {
        None => {
            Ok(None::<Adt19>)
        }
        Some(v2) => {
            let a1101 = {
                let (_, h1099) = v2;
                h1099
            };
            let a1102 = {
                let a1100 = v1;
                length_bytes(a1100)
            };
            if nat_eq(a1101, a1102) {
                Ok(Some({
                    let (h1103, _) = v2;
                    h1103
                }))
            } else {
                Ok(None::<Adt19>)
            }
        }
    }
}

pub fn f253(v0: Adt19) -> bool {
    if f347(v0.clone(), f8(Str::lit("name"))) {
        if f348(v0.clone(), f8(Str::lit("schema"))) {
            let m1104 = f255(f154(f155(f150(v0, f8(Str::lit("citation"))))));
            match m1104 {
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

pub fn f254(v0: Bytes) -> R<Bytes> {
    f349(v0.clone(), 0u64, {
        let a1105 = v0;
        length_bytes(a1105)
    }, true, Bytes::lit(&[]))
}

pub fn f255(v0: List<Bytes>) -> Option<Adt14> {
    let m1106 = v0;
    match m1106.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m1107 = v2;
                match m1107.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m1108 = v2;
                    match m1108.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1109 = v6;
                                match m1109.uncons() {
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
                    if f36(v1, f8(Str::lit("der"))) {
                        let m1110 = v2;
                        match m1110.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1111 = v10;
                                match m1111.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f219(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f256(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1112 = f61(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1112.uncons() {
        None => {
            let m1113 = f255(v4);
            match m1113 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1114 = f8(Str::lit("Catalog."));
                        let a1115 = f254(v3.clone())?;
                        append_bytes(a1114, a1115)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f254(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1116 = f350(v2, v9.clone(), f254(v3.clone())?)?;
                    match m1116 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f254(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f218(Adt14::C3(std::rc::Rc::new(f251(f351(f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f218(Adt14::C3(std::rc::Rc::new(f251(f351(f74(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1120 = f352(f71(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a1118 = {
                                        let a1117 = v10;
                                        length_bytes(a1117)
                                    };
                                    let a1119 = 4u64;
                                    nat_sub(a1118, a1119)
                                }))?;
                                let a1121 = Bytes::lit(&[0x0a]);
                                append_bytes(a1120, a1121)
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

pub fn f257(v0: Adt19) -> R<Bytes> {
    let m1122 = v0;
    match m1122 {
        Adt19::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C1(v2, v3) => {
            f353(v2, v3)
        }
        Adt19::C2(v4, v5, v6) => {
            f354(v4, v5, v6)
        }
        Adt19::C3(v7) => {
            f355(v7)
        }
        Adt19::C4(r1123) => {
            let v8 = (*r1123).clone();
            Ok(f356(f357(v8)?))
        }
        Adt19::C5(r1124) => {
            let v9 = (*r1124).clone();
            f358(f359(v9)?)
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

pub fn f258(v0: Option<Adt19>) -> Option<Bytes> {
    let m1125 = v0;
    match m1125 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1126 = v1;
            match m1126 {
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

pub fn f259(v0: Adt19) -> List<Adt19> {
    let m1127 = v0;
    match m1127 {
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
        Adt19::C4(r1128) => {
            let v3 = (*r1128).clone();
            f259(v3)
        }
        Adt19::C5(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C6 => {
            List::<Adt19>::nil()
        }
        Adt19::C7(r1129, r1130) => {
            let v1 = (*r1129).clone();
            let v2 = (*r1130).clone();
            List::cons(v1, f259(v2))
        }
        Adt19::C8(_, _, _) => {
            List::<Adt19>::nil()
        }
    }
}

pub fn f260(v0: Adt14, v1: Bytes) -> bool {
    let m1131 = f162(v0, v1);
    match m1131 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f261(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1132 = f162(v0, v1);
    match m1132 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f159(f162(v3, v2))
        }
    }
}

pub fn f262(v0: Adt14) -> List<Adt14> {
    let m1133 = v0;
    match m1133 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1134) => {
            let v3 = (*r1134).clone();
            f262(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1135, r1136) => {
            let v1 = (*r1135).clone();
            let v2 = (*r1136).clone();
            List::cons(v1, f262(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f263(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f162(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f117(Str::lit("id"), f160(v0.clone(), f8(Str::lit("id"))))?, List::cons(f117(Str::lit("name"), f160(v0.clone(), f8(Str::lit("name"))))?, List::cons(f117(Str::lit("issuer"), f160(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f117(Str::lit("edition"), f160(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f117(Str::lit("canonical_id"), f160(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f117(Str::lit("source_role"), f160(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f117(Str::lit("retrieval_date"), f160(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f117(Str::lit("url"), f360(v2.clone(), f8(Str::lit("url"))))?, List::cons(f117(Str::lit("media_type"), f360(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f117(Str::lit("license"), f360(v2.clone(), f8(Str::lit("license"))))?, List::cons(f117(Str::lit("redistribution"), f360(v2, f8(Str::lit("redistribution"))))?, List::cons(f117(Str::lit("statement"), f160(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1137 = v1;
        match m1137 {
            None => {
                f7(List::cons(f117(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f117(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f117(Str::lit("artifact"), {
                    let (h1138, _) = v3;
                    h1138
                })?, List::cons(f117(Str::lit("checksum"), {
                    let (_, h1139) = v3;
                    h1139
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f361(f75(f159(f162(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f264(v0: Bytes, v1: u64) -> Option<u8> {
    let a1140 = v0;
    let a1141 = v1;
    index_bytes(a1140, a1141)
}

pub fn f265(v0: Bytes) -> bool {
    f137(v0, f8(Str::lit(".lex.tex")))
}

pub fn f266(v0: Fn3, p1146: u64, p1147: u64) -> R<(u64, bool)> {
    let m1145 = (p1146, p1147);
    {
        let mut m1145 = m1145;
        loop {
            let step = {
                let (v1, v2) = m1145;
                let m1142 = v1;
                if m1142 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1142 - 1;
                    let m1144 = {
                        let c1143: Fn3 = v0.clone();
                        c1143.apply(v2)?
                    };
                    match m1144 {
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
                Ok(next) => { m1145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f222(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1150 = {
            let a1148 = v4;
            let a1149 = v1;
            nat_add(a1148, a1149)?
        };
        let a1151 = v3;
        if nat_lt(a1150, a1151) {
            Ok(Some({
                let a1152 = v4;
                let a1153 = 1u64;
                nat_add(a1152, a1153)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f268(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1154 = v0;
    match m1154.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f268(v3, {
                let a1155 = v1;
                let a1156 = 1u64;
                nat_add(a1155, a1156)?
            })?))
        }
    }
}

pub fn f269(v0: Adt16, v1: List<u64>, v2: Adt21) -> bool {
    let m1157 = v2;
    match m1157 {
        Adt21::C0(v3, v4) => {
            if f275(v3, match v0.clone() {
                Adt16::C0(h1158, _, _, _) => {
                    h1158
                }
            }) {
                f362(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f275(v5, match v0 {
                Adt16::C0(_, _, h1159, _) => {
                    h1159
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f271(v7, v1) {
                f275(v8, match v0 {
                    Adt16::C0(_, _, _, h1160) => {
                        h1160
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f270(v0: Adt12, v1: Adt14) -> R<Adt21> {
    let m1161 = f162(v1.clone(), f8(Str::lit("std")));
    match m1161 {
        None => {
            let m1166 = f163(f162(v1.clone(), f8(Str::lit("req"))));
            match m1166 {
                None => {
                    let m1168 = f162(v1, f8(Str::lit("der")));
                    match m1168 {
                        None => {
                            Ok(f363(v0))
                        }
                        Some(v5) => {
                            let m1169 = f364(f160(v5.clone(), f8(Str::lit("rule"))));
                            match m1169 {
                                None => {
                                    Ok(f363(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f179(f75(f159(f162(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h1170) => {
                                            h1170
                                        }
                                    })?, f273(f160(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h1171, _) => {
                                            h1171
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f273(v4, match v0 {
                        Adt12::C0(_, _, h1167, _, _) => {
                            h1167
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1162 = f163(f162(v2.clone(), f8(Str::lit("row"))));
            match m1162 {
                None => {
                    Ok(Adt21::C0(f273(f160(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h1163, _, _, _, _) => {
                            h1163
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f273(f160(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h1164, _, _, _, _) => {
                            h1164
                        }
                    })?, Some(f273(v3, match v0 {
                        Adt12::C0(_, h1165, _, _, _) => {
                            h1165
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f271(p1174: List<u64>, v1: List<u64>) -> bool {
    let m1173 = p1174;
    {
        let mut m1173 = m1173;
        loop {
            let step = {
                let v0 = m1173;
                let m1172 = v0;
                match m1172.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f275(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f272(p1179: List<u64>) -> bool {
    let m1178 = p1179;
    {
        let mut m1178 = m1178;
        loop {
            let step = {
                let v0 = m1178;
                let m1175 = v0;
                match m1175.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1176 = v1;
                        let a1177 = 7u64;
                        if nat_le(a1176, a1177) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1180 = f365(v0, v1.clone(), 0u64)?;
    match m1180 {
        None => {
            let a1181 = v1;
            Ok(length_list(a1181))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f274(v0: Adt14) -> u64 {
    let m1182 = f366(f162(v0, f8(Str::lit("stratum"))));
    match m1182 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f275(v0: u64, p1187: List<u64>) -> bool {
    let m1186 = p1187;
    {
        let mut m1186 = m1186;
        loop {
            let step = {
                let v1 = m1186;
                let m1183 = v1;
                match m1183.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1184 = v2;
                        let a1185 = v0;
                        if nat_eq(a1184, a1185) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f207(v0.clone(), v1) {
        let m1188 = f30(v0, 34u8)?;
        match m1188.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1189 = v3;
                match m1189.uncons() {
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

pub fn f277(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f278(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1190 = v0.clone();
    match m1190.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1191 = v2;
            let a1192 = v1.clone();
            if equal(a1191, a1192) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f279(v0: List<(Bytes, ())>) -> List<Bytes> {
    f184(f367(Fn6::F368, List::<Bytes>::nil(), v0))
}

pub fn f280(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f369(f370(Fn7::F371, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1197 = {
        let (h1193, _) = f372(Fn8::F373, {
            let a1195 = {
                let a1194 = v0;
                length_list(a1194)
            };
            let a1196 = 1u64;
            nat_add(a1195, a1196)?
        }, v1)?;
        h1193
    };
    match m1197.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f281(v0: List<Bytes>) -> List<(Bytes, ())> {
    f374(f375(Fn9::F376, List::<(Bytes, ())>::nil(), v0))
}

pub fn f282(p1200: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1199 = p1200;
    {
        let mut m1199 = m1199;
        loop {
            let step = {
                let v0 = m1199;
                let m1198 = v0;
                match m1198.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f312(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f360(f162(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f284(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1201, _) = v1.clone();
        h1201
    };
    let v3: u64 = {
        let (h1202, _) = {
            let (_, h1203) = v1.clone();
            h1203
        };
        h1202
    };
    let v4: List<Bytes> = {
        let (_, h1204) = {
            let (_, h1205) = v1;
            h1205
        };
        h1204
    };
    let m1206 = f264(v0.clone(), v2);
    match m1206 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1207 = v5;
            let a1208 = 10u8;
            if equal(a1207, a1208) {
                Ok(Some(({
                    let a1209 = v2;
                    let a1210 = 1u64;
                    nat_add(a1209, a1210)?
                }, ({
                    let a1211 = v2;
                    let a1212 = 1u64;
                    nat_add(a1211, a1212)?
                }, List::cons(f377(f32(v0, v3, {
                    let a1213 = v2;
                    let a1214 = v3;
                    nat_sub(a1213, a1214)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1215 = v2;
                    let a1216 = 1u64;
                    nat_add(a1215, a1216)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f285(v0: Adt1, v1: Bytes) -> R<Adt19> {
    let m1217 = f149(v0, v1)?;
    match m1217 {
        None => {
            Ok(Adt19::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f286(v0: Bytes) -> R<Bytes> {
    let a1218 = f8(Str::lit("Catalog."));
    let a1219 = f254(v0)?;
    Ok(append_bytes(a1218, a1219))
}

pub fn f287(v0: Adt19) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f254(f153(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f288(v0: List<Bytes>) -> List<Bytes> {
    let m1220 = v0;
    match m1220.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f137(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f133(f141(), v1), f288(v2))
            } else {
                f288(v2)
            }
        }
    }
}

pub fn f289(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1221 = v1;
    match m1221.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f378(v0.clone(), v2)?;
            let (h1222, _) = v4.clone();
            if h1222 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f289(v0, v3)?;
                let (h1223, _) = v5.clone();
                if h1223 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1226 = {
                            let (_, h1224) = v4;
                            h1224
                        };
                        let a1227 = {
                            let (_, h1225) = v5;
                            h1225
                        };
                        append_bytes(a1226, a1227)
                    }))
                }
            }
        }
    }
}

pub fn f290(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1228 = v1;
    match m1228.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f205(v2.clone(), f8(Str::lit("subject"))) {
                if f206(v0.clone(), v2.clone()) {
                    f290(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f160(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f160(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f160(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f290(v0, v3))
                }
            } else {
                if f299(v0.clone(), v2.clone()) {
                    f290(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f160(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f300(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f290(v0, v3))
                }
            }
        }
    }
}

pub fn f291(v0: Adt1) -> R<Bytes> {
    let a1230 = {
        let a1229 = f104(f105(f123(f124(f113(f71(v0.clone(), f118()))?)), f106(f74(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f107(f74(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1229)
    };
    let a1231 = 0u64;
    if nat_eq(a1230, a1231) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f292(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1232 = v0;
    match m1232.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f122(f75(f159(f162(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f160(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f160(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f379(f84(f160(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f292(v2)?))
        }
    }
}

pub fn f293(v0: Bytes, v1: List<Adt23>) -> List<Bytes> {
    let m1233 = v1;
    match m1233.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt23::C0(h1234, _, _) => {
                    h1234
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt23::C0(_, h1235, _) => {
                        h1235
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt23::C0(_, _, h1236) => {
                        h1236
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f293(v0, v3))
            } else {
                f293(v0, v3)
            }
        }
    }
}

pub fn f294(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1237 = v1;
    match m1237.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f160(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f160(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f160(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f294(v0, v3))
            } else {
                if f36(f160(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f160(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f160(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f294(v0, v3))
                } else {
                    f294(v0, v3)
                }
            }
        }
    }
}

pub fn f295(p1242: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1241 = p1242;
    {
        let mut m1241 = m1241;
        loop {
            let step = {
                let v0 = m1241;
                let m1238 = v0;
                match m1238.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1239 = v2;
                        match m1239.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m1240 = v5;
                                    match m1240.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f380(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1243 = v0;
    match m1243.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1244 = f162(v1, f8(Str::lit("std")));
            match m1244 {
                None => {
                    f296(v2)
                }
                Some(v3) => {
                    if f205(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f160(v3.clone(), f8(Str::lit("row"))), f160(v3, f8(Str::lit("authority")))), f296(v2))
                    } else {
                        f296(v2)
                    }
                }
            }
        }
    }
}

pub fn f297(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1245 = v1;
    match m1245.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f160(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f160(v2, f8(Str::lit("object")));
            if if f87(v4.clone(), match v0.clone() {
                Adt15::C0(h1246, _, _) => {
                    h1246
                }
            }) {
                f87(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1247, _) => {
                        h1247
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f297(v0, v3))
            } else {
                if if f87(v5.clone(), match v0.clone() {
                    Adt15::C0(h1248, _, _) => {
                        h1248
                    }
                }) {
                    f87(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1249, _) => {
                            h1249
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f297(v0, v3))
                } else {
                    f297(v0, v3)
                }
            }
        }
    }
}

pub fn f298(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt23>> {
    let m1250 = v0;
    match m1250.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v4, v5)) => {
            if if f381(v4.clone(), v2.clone()) {
                true
            } else {
                f381(v4.clone(), v3.clone())
            } {
                f298(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt23::C0({
                    let (h1251, _) = v4.clone();
                    h1251
                }, {
                    let a1254 = {
                        let a1253 = f79(f382({
                            let (h1252, _) = v4.clone();
                            h1252
                        }, v1.clone()))?;
                        length_list(a1253)
                    };
                    let a1255 = 1u64;
                    if nat_eq(a1254, a1255) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1256) = v4.clone();
                    h1256
                }), f298(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f299(v0: Adt15, v1: Adt14) -> bool {
    if f87(f160(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1257, _, _) => {
            h1257
        }
    }) {
        f87(f300(v1), match v0 {
            Adt15::C0(_, h1258, _) => {
                h1258
            }
        })
    } else {
        false
    }
}

pub fn f300(v0: Adt14) -> Bytes {
    let m1259 = f162(v0, f8(Str::lit("evidence")));
    match m1259 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f160(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f301(v0: Adt14) -> Bytes {
    if f205(v0.clone(), f8(Str::lit("req"))) {
        let a1260 = f8(Str::lit("req "));
        let a1261 = f160(v0, f8(Str::lit("req")));
        append_bytes(a1260, a1261)
    } else {
        let m1262 = f162(v0.clone(), f8(Str::lit("std")));
        match m1262 {
            None => {
                let m1265 = f162(v0, f8(Str::lit("der")));
                match m1265 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f160(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f122(f75(f159(f162(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f160(v1.clone(), f8(Str::lit("authority"))), List::cons(if f205(v1.clone(), f8(Str::lit("row"))) {
                    let a1263 = f8(Str::lit(" row "));
                    let a1264 = f160(v1, f8(Str::lit("row")));
                    append_bytes(a1263, a1264)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f302(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1266 = v1;
    match m1266.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f87({
                let (h1267, _) = v2.clone();
                h1267
            }, v0.clone()) {
                List::cons({
                    let (_, h1268) = v2;
                    h1268
                }, f302(v0, v3))
            } else {
                if f87({
                    let (_, h1269) = v2;
                    h1269
                }, v0.clone()) {
                    List::cons({
                        let (h1270, _) = v2;
                        h1270
                    }, f302(v0, v3))
                } else {
                    f302(v0, v3)
                }
            }
        }
    }
}

pub fn f303(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f87(v2.clone(), match v0.clone() {
        Adt15::C0(h1271, _, _) => {
            h1271
        }
    }) {
        true
    } else {
        if f87(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1272, _) => {
                h1272
            }
        }) {
            true
        } else {
            if f87(v2, match v0.clone() {
                Adt15::C0(_, _, h1273) => {
                    h1273
                }
            }) {
                true
            } else {
                if f205(v1.clone(), f8(Str::lit("scheme"))) {
                    f87(f160(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1274, _) => {
                            h1274
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f304(v0: Bytes, v1: u64) -> R<u64> {
    let m1277 = {
        let a1275 = v0;
        let a1276 = v1;
        index_bytes(a1275, a1276)
    };
    match m1277 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1322 = {
                let a1298 = {
                    let a1286 = {
                        let a1280 = {
                            let a1278 = v3;
                            let a1279 = 128u8;
                            fixed_u8::bit_and(a1278, a1279)
                        };
                        let a1281 = 128u8;
                        if equal(a1280, a1281) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1287 = {
                        let a1284 = {
                            let a1282 = v3;
                            let a1283 = 64u8;
                            fixed_u8::bit_and(a1282, a1283)
                        };
                        let a1285 = 64u8;
                        if equal(a1284, a1285) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1286, a1287)?
                };
                let a1299 = {
                    let a1296 = {
                        let a1290 = {
                            let a1288 = v3;
                            let a1289 = 32u8;
                            fixed_u8::bit_and(a1288, a1289)
                        };
                        let a1291 = 32u8;
                        if equal(a1290, a1291) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1297 = {
                        let a1294 = {
                            let a1292 = v3;
                            let a1293 = 16u8;
                            fixed_u8::bit_and(a1292, a1293)
                        };
                        let a1295 = 16u8;
                        if equal(a1294, a1295) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1296, a1297)?
                };
                nat_add(a1298, a1299)?
            };
            let a1323 = {
                let a1320 = {
                    let a1308 = {
                        let a1302 = {
                            let a1300 = v3;
                            let a1301 = 8u8;
                            fixed_u8::bit_and(a1300, a1301)
                        };
                        let a1303 = 8u8;
                        if equal(a1302, a1303) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1309 = {
                        let a1306 = {
                            let a1304 = v3;
                            let a1305 = 4u8;
                            fixed_u8::bit_and(a1304, a1305)
                        };
                        let a1307 = 4u8;
                        if equal(a1306, a1307) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1308, a1309)?
                };
                let a1321 = {
                    let a1318 = {
                        let a1312 = {
                            let a1310 = v3;
                            let a1311 = 2u8;
                            fixed_u8::bit_and(a1310, a1311)
                        };
                        let a1313 = 2u8;
                        if equal(a1312, a1313) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1319 = {
                        let a1316 = {
                            let a1314 = v3;
                            let a1315 = 1u8;
                            fixed_u8::bit_and(a1314, a1315)
                        };
                        let a1317 = 1u8;
                        if equal(a1316, a1317) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1318, a1319)?
                };
                nat_add(a1320, a1321)?
            };
            nat_add(a1322, a1323)
        }
    }
}

pub fn f305(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1327 = {
        let a1324 = v1;
        let a1325 = v2;
        let a1326 = 1u64;
        slice_bytes(a1324, a1325, a1326)
    };
    match m1327 {
        None => {
            v0
        }
        Some(v3) => {
            let a1328 = v0;
            let a1329 = v3;
            append_bytes(a1328, a1329)
        }
    }
}

pub fn f306(p1332: List<Bytes>, p1333: List<Bytes>) -> List<Bytes> {
    let m1331 = (p1332, p1333);
    {
        let mut m1331 = m1331;
        loop {
            let step = {
                let (v0, v1) = m1331;
                let m1330 = v0;
                match m1330.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f307(v0: Bytes, p1405: u64, p1406: u64, p1407: Bytes) -> R<Bytes> {
    let m1404 = (p1405, (p1406, p1407));
    {
        let mut m1404 = m1404;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1404;
                let m1334 = v2;
                if m1334 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1334 - 1;
                    let m1337 = {
                        let a1335 = v0.clone();
                        let a1336 = v1;
                        index_bytes(a1335, a1336)
                    };
                    match m1337 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1382 = {
                                    let a1358 = {
                                        let a1346 = {
                                            let a1340 = {
                                                let a1338 = v6;
                                                let a1339 = 128u8;
                                                fixed_u8::bit_and(a1338, a1339)
                                            };
                                            let a1341 = 128u8;
                                            if equal(a1340, a1341) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1347 = {
                                            let a1344 = {
                                                let a1342 = v6;
                                                let a1343 = 64u8;
                                                fixed_u8::bit_and(a1342, a1343)
                                            };
                                            let a1345 = 64u8;
                                            if equal(a1344, a1345) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1346, a1347)?
                                    };
                                    let a1359 = {
                                        let a1356 = {
                                            let a1350 = {
                                                let a1348 = v6;
                                                let a1349 = 32u8;
                                                fixed_u8::bit_and(a1348, a1349)
                                            };
                                            let a1351 = 32u8;
                                            if equal(a1350, a1351) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1357 = {
                                            let a1354 = {
                                                let a1352 = v6;
                                                let a1353 = 16u8;
                                                fixed_u8::bit_and(a1352, a1353)
                                            };
                                            let a1355 = 16u8;
                                            if equal(a1354, a1355) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1356, a1357)?
                                    };
                                    nat_add(a1358, a1359)?
                                };
                                let a1383 = {
                                    let a1380 = {
                                        let a1368 = {
                                            let a1362 = {
                                                let a1360 = v6;
                                                let a1361 = 8u8;
                                                fixed_u8::bit_and(a1360, a1361)
                                            };
                                            let a1363 = 8u8;
                                            if equal(a1362, a1363) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1369 = {
                                            let a1366 = {
                                                let a1364 = v6;
                                                let a1365 = 4u8;
                                                fixed_u8::bit_and(a1364, a1365)
                                            };
                                            let a1367 = 4u8;
                                            if equal(a1366, a1367) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1368, a1369)?
                                    };
                                    let a1381 = {
                                        let a1378 = {
                                            let a1372 = {
                                                let a1370 = v6;
                                                let a1371 = 2u8;
                                                fixed_u8::bit_and(a1370, a1371)
                                            };
                                            let a1373 = 2u8;
                                            if equal(a1372, a1373) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1379 = {
                                            let a1376 = {
                                                let a1374 = v6;
                                                let a1375 = 1u8;
                                                fixed_u8::bit_and(a1374, a1375)
                                            };
                                            let a1377 = 1u8;
                                            if equal(a1376, a1377) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1378, a1379)?
                                    };
                                    nat_add(a1380, a1381)?
                                };
                                nat_add(a1382, a1383)?
                            };
                            let a1384 = v7;
                            let a1385 = 34u64;
                            if if nat_eq(a1384, a1385) {
                                true
                            } else {
                                let a1386 = v7;
                                let a1387 = 92u64;
                                nat_eq(a1386, a1387)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1388 = v1;
                                    let a1389 = 1u64;
                                    nat_add(a1388, a1389)?
                                }, (v4, {
                                    let a1392 = v3;
                                    let a1393 = {
                                        let a1390 = Bytes::lit(&[0x5c]);
                                        let a1391 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1390, a1391)
                                    };
                                    append_bytes(a1392, a1393)
                                })))
                            } else {
                                let a1394 = v7;
                                let a1395 = 10u64;
                                if nat_eq(a1394, a1395) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1396 = v1;
                                        let a1397 = 1u64;
                                        nat_add(a1396, a1397)?
                                    }, (v4, {
                                        let a1398 = v3;
                                        let a1399 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1398, a1399)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1400 = v1;
                                        let a1401 = 1u64;
                                        nat_add(a1400, a1401)?
                                    }, (v4, {
                                        let a1402 = v3;
                                        let a1403 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1402, a1403)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f308(v0: u32) -> Bytes {
    let a1436 = f383({
        let a1408 = f384(v0, 28u32);
        let a1409 = 15u32;
        fixed_u32::bit_and(a1408, a1409)
    });
    let a1437 = {
        let a1434 = f383({
            let a1410 = f384(v0, 24u32);
            let a1411 = 15u32;
            fixed_u32::bit_and(a1410, a1411)
        });
        let a1435 = {
            let a1432 = f383({
                let a1412 = f384(v0, 20u32);
                let a1413 = 15u32;
                fixed_u32::bit_and(a1412, a1413)
            });
            let a1433 = {
                let a1430 = f383({
                    let a1414 = f384(v0, 16u32);
                    let a1415 = 15u32;
                    fixed_u32::bit_and(a1414, a1415)
                });
                let a1431 = {
                    let a1428 = f383({
                        let a1416 = f384(v0, 12u32);
                        let a1417 = 15u32;
                        fixed_u32::bit_and(a1416, a1417)
                    });
                    let a1429 = {
                        let a1426 = f383({
                            let a1418 = f384(v0, 8u32);
                            let a1419 = 15u32;
                            fixed_u32::bit_and(a1418, a1419)
                        });
                        let a1427 = {
                            let a1424 = f383({
                                let a1420 = f384(v0, 4u32);
                                let a1421 = 15u32;
                                fixed_u32::bit_and(a1420, a1421)
                            });
                            let a1425 = f383({
                                let a1422 = v0;
                                let a1423 = 15u32;
                                fixed_u32::bit_and(a1422, a1423)
                            });
                            append_bytes(a1424, a1425)
                        };
                        append_bytes(a1426, a1427)
                    };
                    append_bytes(a1428, a1429)
                };
                append_bytes(a1430, a1431)
            };
            append_bytes(a1432, a1433)
        };
        append_bytes(a1434, a1435)
    };
    append_bytes(a1436, a1437)
}

pub fn f309(v0: Fn4, p1441: u64, p1442: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1440 = (p1441, p1442);
    {
        let mut m1440 = m1440;
        loop {
            let step = {
                let (v1, v2) = m1440;
                let m1438 = v1;
                if m1438 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1438 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1439: Fn4 = v0.clone();
                        c1439.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1444 = {
            let (h1443, _) = v3.clone();
            h1443
        };
        let a1445 = 64u64;
        nat_add(a1444, a1445)?
    }, f385({
        let (_, h1446) = v3;
        h1446
    }, f386(v0, v1, v2, {
        let (h1447, _) = v3;
        h1447
    })?)?))
}

pub fn f311() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f312(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1448 = f30(v1, 34u8)?;
    match m1448.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1449 = v3;
            match m1449.uncons() {
                None => {
                    Ok(f387(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f313(v0: List<Bytes>) -> List<Bytes> {
    let m1450 = v0;
    match m1450.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f313(v2)
            } else {
                List::cons(v1, f313(v2))
            }
        }
    }
}

pub fn f314(v0: Bytes) -> R<u64> {
    let (h1451, _) = f266(Fn3::F388(v0.clone()), {
        let a1452 = v0.clone();
        length_bytes(a1452)
    }, {
        let a1453 = v0;
        length_bytes(a1453)
    })?;
    Ok(h1451)
}

pub fn f315(v0: Adt14) -> R<Bytes> {
    let m1454 = v0;
    match m1454 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f353(v2, v3)
        }
        Adt14::C2(v4) => {
            f355(v4)
        }
        Adt14::C3(r1455) => {
            let v5 = (*r1455).clone();
            Ok(f356(f389(v5)?))
        }
        Adt14::C4(r1456) => {
            let v6 = (*r1456).clone();
            f358(f390(v6)?)
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

pub fn f316(v0: Fn5, p1461: u64, p1462: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1460 = (p1461, p1462);
    {
        let mut m1460 = m1460;
        loop {
            let step = {
                let (v1, v2) = m1460;
                let m1457 = v1;
                if m1457 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1457 - 1;
                    let m1459 = {
                        let c1458: Fn5 = v0.clone();
                        c1458.apply(v2.clone())?
                    };
                    match m1459 {
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
                Ok(next) => { m1460 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f391(v0, v1)
}

pub fn f318(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f392(v0, v1)
}

pub fn f319(v0: u64) -> bool {
    let a1463 = v0;
    let a1464 = 32u64;
    if if nat_eq(a1463, a1464) {
        true
    } else {
        let a1465 = v0;
        let a1466 = 9u64;
        nat_eq(a1465, a1466)
    } {
        true
    } else {
        let a1467 = v0;
        let a1468 = 10u64;
        if nat_eq(a1467, a1468) {
            true
        } else {
            let a1469 = v0;
            let a1470 = 13u64;
            nat_eq(a1469, a1470)
        }
    }
}

pub fn f320(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1471, _) = f266(Fn3::F393(v1.clone(), v0.clone()), {
            let a1473 = {
                let a1472 = v0.clone();
                length_bytes(a1472)
            };
            let a1474 = 1u64;
            nat_add(a1473, a1474)?
        }, v2)?;
        h1471
    };
    if f222(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f321(v0: Bytes, v1: u64) -> R<u64> {
    let (h1475, _) = f266(Fn3::F394(v0.clone()), {
        let a1477 = {
            let a1476 = v0;
            length_bytes(a1476)
        };
        let a1478 = 1u64;
        nat_add(a1477, a1478)?
    }, v1)?;
    Ok(h1475)
}

pub fn f322(v0: Bytes) -> R<bool> {
    let a1480 = {
        let a1479 = v0.clone();
        length_bytes(a1479)
    };
    let a1481 = 5u64;
    if nat_eq(a1480, a1481) {
        let a1482 = f325(v0.clone(), 0u64)?;
        let a1483 = 85u64;
        if if nat_eq(a1482, a1483) {
            true
        } else {
            let a1484 = f325(v0.clone(), 0u64)?;
            let a1485 = 117u64;
            nat_eq(a1484, a1485)
        } {
            let a1486 = f325(v0.clone(), 1u64)?;
            let a1487 = 84u64;
            if if nat_eq(a1486, a1487) {
                true
            } else {
                let a1488 = f325(v0.clone(), 1u64)?;
                let a1489 = 116u64;
                nat_eq(a1488, a1489)
            } {
                let a1490 = f325(v0.clone(), 2u64)?;
                let a1491 = 70u64;
                if if nat_eq(a1490, a1491) {
                    true
                } else {
                    let a1492 = f325(v0.clone(), 2u64)?;
                    let a1493 = 102u64;
                    nat_eq(a1492, a1493)
                } {
                    let a1494 = f325(v0.clone(), 3u64)?;
                    let a1495 = 45u64;
                    if nat_eq(a1494, a1495) {
                        let a1496 = f325(v0, 4u64)?;
                        let a1497 = 56u64;
                        Ok(nat_eq(a1496, a1497))
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

pub fn f323(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1501 = f320(v0.clone(), {
        let a1498 = Str::lit("--");
        utf8_encode(a1498)
    }, {
        let a1499 = v1;
        let a1500 = 4u64;
        nat_add(a1499, a1500)?
    })?;
    match m1501 {
        None => {
            Ok(Err::<u64, Bytes>(f225(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1504 = f229(v0, {
                let a1502 = v2;
                let a1503 = 2u64;
                nat_add(a1502, a1503)?
            })?;
            let a1505 = 62u64;
            if nat_eq(a1504, a1505) {
                Ok(Ok::<u64, Bytes>({
                    let a1506 = v2;
                    let a1507 = 3u64;
                    nat_add(a1506, a1507)?
                }))
            } else {
                Ok(Err::<u64, Bytes>(f225(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f324(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f326(v0.clone(), {
        let a1508 = v1;
        let a1509 = 2u64;
        nat_add(a1508, a1509)?
    })?;
    let a1512 = v2;
    let a1513 = {
        let a1510 = v1;
        let a1511 = 2u64;
        nat_add(a1510, a1511)?
    };
    if nat_eq(a1512, a1513) {
        Ok(Err::<u64, Bytes>(f225(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1516 = v2;
        let a1517 = {
            let a1514 = v1;
            let a1515 = 5u64;
            nat_add(a1514, a1515)?
        };
        if if nat_eq(a1516, a1517) {
            let a1520 = f229(v0.clone(), {
                let a1518 = v1;
                let a1519 = 2u64;
                nat_add(a1518, a1519)?
            })?;
            let a1521 = 120u64;
            if if nat_eq(a1520, a1521) {
                true
            } else {
                let a1524 = f229(v0.clone(), {
                    let a1522 = v1;
                    let a1523 = 2u64;
                    nat_add(a1522, a1523)?
                })?;
                let a1525 = 88u64;
                nat_eq(a1524, a1525)
            } {
                let a1528 = f229(v0.clone(), {
                    let a1526 = v1;
                    let a1527 = 3u64;
                    nat_add(a1526, a1527)?
                })?;
                let a1529 = 109u64;
                if if nat_eq(a1528, a1529) {
                    true
                } else {
                    let a1532 = f229(v0.clone(), {
                        let a1530 = v1;
                        let a1531 = 3u64;
                        nat_add(a1530, a1531)?
                    })?;
                    let a1533 = 77u64;
                    nat_eq(a1532, a1533)
                } {
                    let a1536 = f229(v0.clone(), {
                        let a1534 = v1;
                        let a1535 = 4u64;
                        nat_add(a1534, a1535)?
                    })?;
                    let a1537 = 108u64;
                    if nat_eq(a1536, a1537) {
                        true
                    } else {
                        let a1540 = f229(v0.clone(), {
                            let a1538 = v1;
                            let a1539 = 4u64;
                            nat_add(a1538, a1539)?
                        })?;
                        let a1541 = 76u64;
                        nat_eq(a1540, a1541)
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
            Ok(Err::<u64, Bytes>(f225(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1543 = f320(v0, {
                let a1542 = Str::lit("?>");
                utf8_encode(a1542)
            }, v2)?;
            match m1543 {
                None => {
                    Ok(Err::<u64, Bytes>(f225(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1544 = v3;
                        let a1545 = 2u64;
                        nat_add(a1544, a1545)?
                    }))
                }
            }
        }
    }
}

pub fn f325(v0: Bytes, v1: u64) -> R<u64> {
    let m1548 = {
        let a1546 = v0;
        let a1547 = v1;
        index_bytes(a1546, a1547)
    };
    match m1548 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1593 = {
                let a1569 = {
                    let a1557 = {
                        let a1551 = {
                            let a1549 = v3;
                            let a1550 = 128u8;
                            fixed_u8::bit_and(a1549, a1550)
                        };
                        let a1552 = 128u8;
                        if equal(a1551, a1552) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1558 = {
                        let a1555 = {
                            let a1553 = v3;
                            let a1554 = 64u8;
                            fixed_u8::bit_and(a1553, a1554)
                        };
                        let a1556 = 64u8;
                        if equal(a1555, a1556) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1557, a1558)?
                };
                let a1570 = {
                    let a1567 = {
                        let a1561 = {
                            let a1559 = v3;
                            let a1560 = 32u8;
                            fixed_u8::bit_and(a1559, a1560)
                        };
                        let a1562 = 32u8;
                        if equal(a1561, a1562) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1568 = {
                        let a1565 = {
                            let a1563 = v3;
                            let a1564 = 16u8;
                            fixed_u8::bit_and(a1563, a1564)
                        };
                        let a1566 = 16u8;
                        if equal(a1565, a1566) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1567, a1568)?
                };
                nat_add(a1569, a1570)?
            };
            let a1594 = {
                let a1591 = {
                    let a1579 = {
                        let a1573 = {
                            let a1571 = v3;
                            let a1572 = 8u8;
                            fixed_u8::bit_and(a1571, a1572)
                        };
                        let a1574 = 8u8;
                        if equal(a1573, a1574) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1580 = {
                        let a1577 = {
                            let a1575 = v3;
                            let a1576 = 4u8;
                            fixed_u8::bit_and(a1575, a1576)
                        };
                        let a1578 = 4u8;
                        if equal(a1577, a1578) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1579, a1580)?
                };
                let a1592 = {
                    let a1589 = {
                        let a1583 = {
                            let a1581 = v3;
                            let a1582 = 2u8;
                            fixed_u8::bit_and(a1581, a1582)
                        };
                        let a1584 = 2u8;
                        if equal(a1583, a1584) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1590 = {
                        let a1587 = {
                            let a1585 = v3;
                            let a1586 = 1u8;
                            fixed_u8::bit_and(a1585, a1586)
                        };
                        let a1588 = 1u8;
                        if equal(a1587, a1588) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1589, a1590)?
                };
                nat_add(a1591, a1592)?
            };
            nat_add(a1593, a1594)
        }
    }
}

pub fn f326(v0: Bytes, v1: u64) -> R<u64> {
    let (h1595, _) = f266(Fn3::F395(v0.clone(), v1), {
        let a1597 = {
            let a1596 = v0;
            length_bytes(a1596)
        };
        let a1598 = 1u64;
        nat_add(a1597, a1598)?
    }, v1)?;
    Ok(h1595)
}

pub fn f327(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt18, u64), Bytes>> {
    let m1599 = v2;
    if m1599 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1599 - 1;
        let v5: u64 = f321(v0.clone(), v1)?;
        let a1600 = f229(v0.clone(), v5)?;
        let a1601 = 62u64;
        if if nat_eq(a1600, a1601) {
            true
        } else {
            f228(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v5)))
        } else {
            let a1602 = v5;
            let a1603 = v1;
            if nat_eq(a1602, a1603) {
                Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f326(v0.clone(), v5)?;
                let a1604 = v6;
                let a1605 = v5;
                if nat_eq(a1604, a1605) {
                    Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a1606 = v6;
                        let a1607 = v5;
                        nat_sub(a1606, a1607)
                    });
                    if f396(v7.clone(), v3.clone()) {
                        Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f321(v0.clone(), v6)?;
                        let a1608 = f229(v0.clone(), v8)?;
                        let a1609 = 61u64;
                        if !nat_eq(a1608, a1609) {
                            Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        } else {
                            let v9: u64 = f321(v0.clone(), {
                                let a1610 = v8;
                                let a1611 = 1u64;
                                nat_add(a1610, a1611)?
                            })?;
                            let v10: u64 = f229(v0.clone(), v9)?;
                            let a1612 = v10;
                            let a1613 = 34u64;
                            if !if nat_eq(a1612, a1613) {
                                true
                            } else {
                                let a1614 = v10;
                                let a1615 = 39u64;
                                nat_eq(a1614, a1615)
                            } {
                                Ok(Err::<(Adt18, u64), Bytes>(f225(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            } else {
                                let m1618 = f328(v0.clone(), {
                                    let a1616 = v9;
                                    let a1617 = 1u64;
                                    nat_add(a1616, a1617)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1618 {
                                    Ok(v12) => {
                                        let m1620 = f327(v0, {
                                            let (_, h1619) = v12;
                                            h1619
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1620 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C1(v7, {
                                                    let (h1621, _) = v12;
                                                    h1621
                                                }, std::rc::Rc::new({
                                                    let (h1622, _) = v14;
                                                    h1622
                                                })), {
                                                    let (_, h1623) = v14;
                                                    h1623
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

pub fn f328(v0: Bytes, p1663: u64, p1664: u64, p1665: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1662 = (p1663, (p1664, p1665));
    {
        let mut m1662 = m1662;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1662;
                let m1624 = v2;
                if m1624 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f225(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1624 - 1;
                    let v7: u64 = f229(v0.clone(), v1)?;
                    if if v4 {
                        let a1625 = v7;
                        let a1626 = v5;
                        nat_eq(a1625, a1626)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1627 = v1;
                            let a1628 = 1u64;
                            nat_add(a1627, a1628)?
                        }))))
                    } else {
                        let a1629 = v7;
                        let a1630 = 256u64;
                        if nat_eq(a1629, a1630) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f225(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1631 = v7;
                            let a1632 = 60u64;
                            if nat_eq(a1631, a1632) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f225(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1633 = v7;
                                let a1634 = 38u64;
                                if nat_eq(a1633, a1634) {
                                    let m1635 = f397(v0.clone(), v1)?;
                                    match m1635 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1636) = v9;
                                                h1636
                                            }, (v6, {
                                                let a1638 = v3;
                                                let a1639 = {
                                                    let (h1637, _) = v9;
                                                    h1637
                                                };
                                                append_bytes(a1638, a1639)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1640 = v7;
                                        let a1641 = 9u64;
                                        if if nat_eq(a1640, a1641) {
                                            true
                                        } else {
                                            let a1642 = v7;
                                            let a1643 = 10u64;
                                            nat_eq(a1642, a1643)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1644 = v1;
                                                let a1645 = 1u64;
                                                nat_add(a1644, a1645)?
                                            }, (v6, {
                                                let a1646 = v3;
                                                let a1647 = Bytes::lit(&[0x20]);
                                                append_bytes(a1646, a1647)
                                            })))
                                        } else {
                                            let v10: u64 = f398(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1650 = v3;
                                                let a1651 = f32(v0.clone(), v1, {
                                                    let a1648 = v10;
                                                    let a1649 = v1;
                                                    nat_sub(a1648, a1649)
                                                });
                                                append_bytes(a1650, a1651)
                                            })))
                                        }
                                    } else {
                                        let a1652 = v7;
                                        let a1653 = 93u64;
                                        if nat_eq(a1652, a1653) {
                                            if f228(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f225(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1654 = v1;
                                                    let a1655 = 1u64;
                                                    nat_add(a1654, a1655)?
                                                }, (v6, {
                                                    let a1656 = v3;
                                                    let a1657 = Bytes::lit(&[0x5d]);
                                                    append_bytes(a1656, a1657)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f398(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1660 = v3;
                                                let a1661 = f32(v0.clone(), v1, {
                                                    let a1658 = v11;
                                                    let a1659 = v1;
                                                    nat_sub(a1658, a1659)
                                                });
                                                append_bytes(a1660, a1661)
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
                Ok(next) => { m1662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f329(v0: Bytes, v1: u64) -> Bytes {
    let m1669 = {
        let a1666 = v0;
        let a1667 = v1;
        let a1668 = 1u64;
        slice_bytes(a1666, a1667, a1668)
    };
    match m1669 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f330(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f399(v1, v0, v2)
}

pub fn f331() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f332() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f333(v0: Bytes) -> R<Bytes> {
    let m1670 = f320(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1670 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f334(p1675: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1674 = p1675;
    {
        let mut m1674 = m1674;
        loop {
            let step = {
                let v0 = m1674;
                let m1671 = v0;
                match m1671.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h1672, _) = v2;
                            h1672
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1673) = v2;
                                h1673
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Bytes) -> R<Bytes> {
    let m1676 = f320(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1676 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1677 = v1;
                let a1678 = 1u64;
                nat_add(a1677, a1678)?
            }, {
                let a1682 = {
                    let a1679 = v0;
                    length_bytes(a1679)
                };
                let a1683 = {
                    let a1680 = v1;
                    let a1681 = 1u64;
                    nat_add(a1680, a1681)?
                };
                nat_sub(a1682, a1683)
            }))
        }
    }
}

pub fn f336() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f337(v0: Bytes, v1: Adt18, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v4: Bytes = f254(v0.clone())?;
    let m1684 = f400(v1, v2)?;
    match m1684 {
        Ok(v6) => {
            let m1685 = v6;
            match m1685 {
                None => {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = {
                        let (h1686, _) = v7.clone();
                        h1686
                    };
                    let v9: Adt18 = {
                        let (h1687, _) = {
                            let (_, h1688) = v7.clone();
                            h1688
                        };
                        h1687
                    };
                    let v10: Adt18 = {
                        let (h1689, _) = {
                            let (_, h1690) = {
                                let (_, h1691) = v7.clone();
                                h1691
                            };
                            h1690
                        };
                        h1689
                    };
                    let v11: List<(Bytes, Bytes)> = {
                        let (_, h1692) = {
                            let (_, h1693) = {
                                let (_, h1694) = v7;
                                h1694
                            };
                            h1693
                        };
                        h1692
                    };
                    if f36(v8.clone(), f8(Str::lit("union"))) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f36(v8, f8(Str::lit("list"))) {
                            let m1695 = f236(v9, f8(Str::lit("itemType")));
                            match m1695 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1696 = f402(v11, v3, v12)?;
                                    match m1696 {
                                        Ok(v14) => {
                                            if f207({
                                                let (h1697, _) = v14.clone();
                                                h1697
                                            }, f8(Str::lit("(List "))) {
                                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f403(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons({
                                                    let (h1698, _) = v14;
                                                    h1698
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), {
                                                    let (_, h1699) = v14;
                                                    h1699
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1700 = f236(v9, f8(Str::lit("base")));
                            match m1700 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1701 = f402(v11.clone(), v3, v15)?;
                                    match m1701 {
                                        Ok(v17) => {
                                            let m1702 = f404(v10, v11)?;
                                            match m1702 {
                                                Ok(v19) => {
                                                    let v20: List<Adt19> = f405(v19, List::<Bytes>::nil())?;
                                                    let m1703 = v20.clone();
                                                    match m1703.uncons() {
                                                        None => {
                                                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f403(v4, {
                                                                let (h1704, _) = v17;
                                                                h1704
                                                            }), {
                                                                let (_, h1705) = v17;
                                                                h1705
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1706 = f406(f407(f154(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1706 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f408(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(v16, v0)))
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
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f401(v5, v0)))
        }
    }
}

pub fn f338(p1709: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1708 = p1709;
    {
        let mut m1708 = m1708;
        loop {
            let step = {
                let v0 = m1708;
                let m1707 = v0;
                match m1707.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f87(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1710 = v0;
    match m1710.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1713 = f338({
                let (h1711, _) = {
                    let (_, h1712) = v2.clone();
                    h1712
                };
                h1711
            }, v1.clone());
            match m1713 {
                None => {
                    Some((({
                        let (h1714, _) = v2;
                        h1714
                    }, {
                        let (_, h1715) = {
                            let (_, h1716) = v2;
                            h1716
                        };
                        h1715
                    }), v3))
                }
                Some(_) => {
                    let m1717 = f339(v3, v1);
                    match m1717 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1718, _) = v5;
                                h1718
                            }, List::cons(v2, {
                                let (_, h1719) = v5;
                                h1719
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f340(v0: Bytes, v1: u64) -> R<u64> {
    let (h1720, _) = f266(Fn3::F409(v1, v0), v1, 0u64)?;
    Ok(h1720)
}

pub fn f341(v0: Bytes) -> R<Bytes> {
    f410(v0.clone(), 0u64, {
        let a1721 = v0;
        length_bytes(a1721)
    }, false, false, Bytes::lit(&[]))
}

pub fn f342(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1722 = v2;
    if m1722 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1722 - 1;
        let v5: u64 = f325(v0.clone(), v1)?;
        let a1723 = v3;
        let a1724 = 1u64;
        if nat_eq(a1723, a1724) {
            let m1725 = f342(v0.clone(), v1, v4, 0u64)?;
            match m1725 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f325(v0.clone(), {
                        let (_, h1726) = v6.clone();
                        h1726
                    })?;
                    let a1727 = v7;
                    let a1728 = 44u64;
                    if nat_eq(a1727, a1728) {
                        let m1732 = f342(v0, {
                            let a1730 = {
                                let (_, h1729) = v6;
                                h1729
                            };
                            let a1731 = 1u64;
                            nat_add(a1730, a1731)?
                        }, v4, 1u64)?;
                        match m1732 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1733, _) = v6;
                                    h1733
                                }), std::rc::Rc::new({
                                    let (h1734, _) = v8;
                                    h1734
                                })), {
                                    let (_, h1735) = v8;
                                    h1735
                                })))
                            }
                        }
                    } else {
                        let a1736 = v7;
                        let a1737 = 93u64;
                        if nat_eq(a1736, a1737) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1738, _) = v6;
                                h1738
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1740 = {
                                    let (_, h1739) = v6;
                                    h1739
                                };
                                let a1741 = 1u64;
                                nat_add(a1740, a1741)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1742 = v3;
            let a1743 = 2u64;
            if nat_eq(a1742, a1743) {
                let m1746 = f411(v0.clone(), {
                    let a1744 = v1;
                    let a1745 = 1u64;
                    nat_add(a1744, a1745)?
                })?;
                match m1746 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1747 = v5;
                        let a1748 = 34u64;
                        if if nat_eq(a1747, a1748) {
                            let a1750 = f325(v0.clone(), {
                                let (h1749, _) = v9.clone();
                                h1749
                            })?;
                            let a1751 = 58u64;
                            nat_eq(a1750, a1751)
                        } else {
                            false
                        } {
                            let m1755 = f342(v0.clone(), {
                                let a1753 = {
                                    let (h1752, _) = v9;
                                    h1752
                                };
                                let a1754 = 1u64;
                                nat_add(a1753, a1754)?
                            }, v4, 0u64)?;
                            match m1755 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f325(v0.clone(), {
                                        let (_, h1756) = v10.clone();
                                        h1756
                                    })?;
                                    let a1757 = v11;
                                    let a1758 = 44u64;
                                    if nat_eq(a1757, a1758) {
                                        let m1762 = f342(v0, {
                                            let a1760 = {
                                                let (_, h1759) = v10;
                                                h1759
                                            };
                                            let a1761 = 1u64;
                                            nat_add(a1760, a1761)?
                                        }, v4, 2u64)?;
                                        match m1762 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1763) = v9;
                                                    h1763
                                                }, std::rc::Rc::new({
                                                    let (h1764, _) = v10;
                                                    h1764
                                                }), std::rc::Rc::new({
                                                    let (h1765, _) = v12;
                                                    h1765
                                                })), {
                                                    let (_, h1766) = v12;
                                                    h1766
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1767 = v11;
                                        let a1768 = 125u64;
                                        if nat_eq(a1767, a1768) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1769) = v9;
                                                h1769
                                            }, std::rc::Rc::new({
                                                let (h1770, _) = v10;
                                                h1770
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1772 = {
                                                    let (_, h1771) = v10;
                                                    h1771
                                                };
                                                let a1773 = 1u64;
                                                nat_add(a1772, a1773)?
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
                let a1774 = v5;
                let a1775 = 34u64;
                if nat_eq(a1774, a1775) {
                    let m1778 = f411(v0, {
                        let a1776 = v1;
                        let a1777 = 1u64;
                        nat_add(a1776, a1777)?
                    })?;
                    match m1778 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1779) = v13;
                                h1779
                            }), {
                                let (h1780, _) = v13;
                                h1780
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1781 = v1;
                            let a1782 = 4u64;
                            nat_add(a1781, a1782)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1783 = v1;
                                let a1784 = 5u64;
                                nat_add(a1783, a1784)?
                            })))
                        } else {
                            let a1785 = v5;
                            let a1786 = 91u64;
                            if nat_eq(a1785, a1786) {
                                let a1789 = f325(v0.clone(), {
                                    let a1787 = v1;
                                    let a1788 = 1u64;
                                    nat_add(a1787, a1788)?
                                })?;
                                let a1790 = 93u64;
                                if nat_eq(a1789, a1790) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1791 = v1;
                                        let a1792 = 2u64;
                                        nat_add(a1791, a1792)?
                                    })))
                                } else {
                                    let m1795 = f342(v0, {
                                        let a1793 = v1;
                                        let a1794 = 1u64;
                                        nat_add(a1793, a1794)?
                                    }, v4, 1u64)?;
                                    match m1795 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1796, _) = v14;
                                                h1796
                                            })), {
                                                let (_, h1797) = v14;
                                                h1797
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1798 = v5;
                                let a1799 = 123u64;
                                if nat_eq(a1798, a1799) {
                                    let a1802 = f325(v0.clone(), {
                                        let a1800 = v1;
                                        let a1801 = 1u64;
                                        nat_add(a1800, a1801)?
                                    })?;
                                    let a1803 = 125u64;
                                    if nat_eq(a1802, a1803) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1804 = v1;
                                            let a1805 = 2u64;
                                            nat_add(a1804, a1805)?
                                        })))
                                    } else {
                                        let m1808 = f342(v0, {
                                            let a1806 = v1;
                                            let a1807 = 1u64;
                                            nat_add(a1806, a1807)?
                                        }, v4, 2u64)?;
                                        match m1808 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1809, _) = v15;
                                                    h1809
                                                })), {
                                                    let (_, h1810) = v15;
                                                    h1810
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1811 = f412(v0, v1)?;
                                    match m1811 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1812) = v16;
                                                h1812
                                            }, {
                                                let (h1813, _) = v16;
                                                h1813
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

pub fn f343() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f344(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1814 = f162(v0, v1);
    match m1814 {
        None => {
            false
        }
        Some(v3) => {
            f205(v3, v2)
        }
    }
}

pub fn f345() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f346(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt19, u64)>> {
    let m1815 = v2;
    if m1815 == 0 {
        Ok(None::<(Adt19, u64)>)
    } else {
        let v4 = m1815 - 1;
        let v5: u64 = f325(v0.clone(), v1)?;
        let a1816 = v3;
        let a1817 = 1u64;
        if nat_eq(a1816, a1817) {
            let m1818 = f346(v0.clone(), v1, v4, 0u64)?;
            match m1818 {
                None => {
                    Ok(None::<(Adt19, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f325(v0.clone(), {
                        let (_, h1819) = v6.clone();
                        h1819
                    })?;
                    let a1820 = v7;
                    let a1821 = 44u64;
                    if nat_eq(a1820, a1821) {
                        let m1825 = f346(v0, {
                            let a1823 = {
                                let (_, h1822) = v6;
                                h1822
                            };
                            let a1824 = 1u64;
                            nat_add(a1823, a1824)?
                        }, v4, 1u64)?;
                        match m1825 {
                            None => {
                                Ok(None::<(Adt19, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt19::C7(std::rc::Rc::new({
                                    let (h1826, _) = v6;
                                    h1826
                                }), std::rc::Rc::new({
                                    let (h1827, _) = v8;
                                    h1827
                                })), {
                                    let (_, h1828) = v8;
                                    h1828
                                })))
                            }
                        }
                    } else {
                        let a1829 = v7;
                        let a1830 = 93u64;
                        if nat_eq(a1829, a1830) {
                            Ok(Some((Adt19::C7(std::rc::Rc::new({
                                let (h1831, _) = v6;
                                h1831
                            }), std::rc::Rc::new(Adt19::C6)), {
                                let a1833 = {
                                    let (_, h1832) = v6;
                                    h1832
                                };
                                let a1834 = 1u64;
                                nat_add(a1833, a1834)?
                            })))
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1835 = v3;
            let a1836 = 2u64;
            if nat_eq(a1835, a1836) {
                let m1839 = f411(v0.clone(), {
                    let a1837 = v1;
                    let a1838 = 1u64;
                    nat_add(a1837, a1838)?
                })?;
                match m1839 {
                    None => {
                        Ok(None::<(Adt19, u64)>)
                    }
                    Some(v9) => {
                        let a1840 = v5;
                        let a1841 = 34u64;
                        if if nat_eq(a1840, a1841) {
                            let a1843 = f325(v0.clone(), {
                                let (h1842, _) = v9.clone();
                                h1842
                            })?;
                            let a1844 = 58u64;
                            nat_eq(a1843, a1844)
                        } else {
                            false
                        } {
                            let m1848 = f346(v0.clone(), {
                                let a1846 = {
                                    let (h1845, _) = v9;
                                    h1845
                                };
                                let a1847 = 1u64;
                                nat_add(a1846, a1847)?
                            }, v4, 0u64)?;
                            match m1848 {
                                None => {
                                    Ok(None::<(Adt19, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f325(v0.clone(), {
                                        let (_, h1849) = v10.clone();
                                        h1849
                                    })?;
                                    let a1850 = v11;
                                    let a1851 = 44u64;
                                    if nat_eq(a1850, a1851) {
                                        let m1855 = f346(v0, {
                                            let a1853 = {
                                                let (_, h1852) = v10;
                                                h1852
                                            };
                                            let a1854 = 1u64;
                                            nat_add(a1853, a1854)?
                                        }, v4, 2u64)?;
                                        match m1855 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt19::C8({
                                                    let (_, h1856) = v9;
                                                    h1856
                                                }, std::rc::Rc::new({
                                                    let (h1857, _) = v10;
                                                    h1857
                                                }), std::rc::Rc::new({
                                                    let (h1858, _) = v12;
                                                    h1858
                                                })), {
                                                    let (_, h1859) = v12;
                                                    h1859
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1860 = v11;
                                        let a1861 = 125u64;
                                        if nat_eq(a1860, a1861) {
                                            Ok(Some((Adt19::C8({
                                                let (_, h1862) = v9;
                                                h1862
                                            }, std::rc::Rc::new({
                                                let (h1863, _) = v10;
                                                h1863
                                            }), std::rc::Rc::new(Adt19::C6)), {
                                                let a1865 = {
                                                    let (_, h1864) = v10;
                                                    h1864
                                                };
                                                let a1866 = 1u64;
                                                nat_add(a1865, a1866)?
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
                let a1867 = v5;
                let a1868 = 34u64;
                if nat_eq(a1867, a1868) {
                    let m1871 = f411(v0, {
                        let a1869 = v1;
                        let a1870 = 1u64;
                        nat_add(a1869, a1870)?
                    })?;
                    match m1871 {
                        None => {
                            Ok(None::<(Adt19, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt19::C3({
                                let (_, h1872) = v13;
                                h1872
                            }), {
                                let (h1873, _) = v13;
                                h1873
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt19::C0(true), {
                            let a1874 = v1;
                            let a1875 = 4u64;
                            nat_add(a1874, a1875)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt19::C0(false), {
                                let a1876 = v1;
                                let a1877 = 5u64;
                                nat_add(a1876, a1877)?
                            })))
                        } else {
                            let a1878 = v5;
                            let a1879 = 91u64;
                            if nat_eq(a1878, a1879) {
                                let a1882 = f325(v0.clone(), {
                                    let a1880 = v1;
                                    let a1881 = 1u64;
                                    nat_add(a1880, a1881)?
                                })?;
                                let a1883 = 93u64;
                                if nat_eq(a1882, a1883) {
                                    Ok(Some((Adt19::C4(std::rc::Rc::new(Adt19::C6)), {
                                        let a1884 = v1;
                                        let a1885 = 2u64;
                                        nat_add(a1884, a1885)?
                                    })))
                                } else {
                                    let m1888 = f346(v0, {
                                        let a1886 = v1;
                                        let a1887 = 1u64;
                                        nat_add(a1886, a1887)?
                                    }, v4, 1u64)?;
                                    match m1888 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt19::C4(std::rc::Rc::new({
                                                let (h1889, _) = v14;
                                                h1889
                                            })), {
                                                let (_, h1890) = v14;
                                                h1890
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1891 = v5;
                                let a1892 = 123u64;
                                if nat_eq(a1891, a1892) {
                                    let a1895 = f325(v0.clone(), {
                                        let a1893 = v1;
                                        let a1894 = 1u64;
                                        nat_add(a1893, a1894)?
                                    })?;
                                    let a1896 = 125u64;
                                    if nat_eq(a1895, a1896) {
                                        Ok(Some((Adt19::C5(std::rc::Rc::new(Adt19::C6)), {
                                            let a1897 = v1;
                                            let a1898 = 2u64;
                                            nat_add(a1897, a1898)?
                                        })))
                                    } else {
                                        let m1901 = f346(v0, {
                                            let a1899 = v1;
                                            let a1900 = 1u64;
                                            nat_add(a1899, a1900)?
                                        }, v4, 2u64)?;
                                        match m1901 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt19::C5(std::rc::Rc::new({
                                                    let (h1902, _) = v15;
                                                    h1902
                                                })), {
                                                    let (_, h1903) = v15;
                                                    h1903
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1904 = f413(v0, v1)?;
                                    match m1904 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1905) = v16;
                                                h1905
                                            }, {
                                                let (h1906, _) = v16;
                                                h1906
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

pub fn f347(v0: Adt19, v1: Bytes) -> bool {
    let a1908 = {
        let a1907 = f153(v0, v1);
        length_bytes(a1907)
    };
    let a1909 = 0u64;
    !nat_eq(a1908, a1909)
}

pub fn f348(v0: Adt19, v1: Bytes) -> bool {
    let m1910 = f150(v0, v1);
    match m1910 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f349(v0: Bytes, p1977: u64, p1978: u64, p1979: bool, p1980: Bytes) -> R<Bytes> {
    let m1976 = (p1977, (p1978, (p1979, p1980)));
    {
        let mut m1976 = m1976;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1976;
                let m1911 = v2;
                if m1911 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1911 - 1;
                    let m1914 = {
                        let a1912 = v0.clone();
                        let a1913 = v1;
                        index_bytes(a1912, a1913)
                    };
                    match m1914 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1959 = {
                                    let a1935 = {
                                        let a1923 = {
                                            let a1917 = {
                                                let a1915 = v7;
                                                let a1916 = 128u8;
                                                fixed_u8::bit_and(a1915, a1916)
                                            };
                                            let a1918 = 128u8;
                                            if equal(a1917, a1918) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1924 = {
                                            let a1921 = {
                                                let a1919 = v7;
                                                let a1920 = 64u8;
                                                fixed_u8::bit_and(a1919, a1920)
                                            };
                                            let a1922 = 64u8;
                                            if equal(a1921, a1922) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1923, a1924)?
                                    };
                                    let a1936 = {
                                        let a1933 = {
                                            let a1927 = {
                                                let a1925 = v7;
                                                let a1926 = 32u8;
                                                fixed_u8::bit_and(a1925, a1926)
                                            };
                                            let a1928 = 32u8;
                                            if equal(a1927, a1928) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1934 = {
                                            let a1931 = {
                                                let a1929 = v7;
                                                let a1930 = 16u8;
                                                fixed_u8::bit_and(a1929, a1930)
                                            };
                                            let a1932 = 16u8;
                                            if equal(a1931, a1932) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1933, a1934)?
                                    };
                                    nat_add(a1935, a1936)?
                                };
                                let a1960 = {
                                    let a1957 = {
                                        let a1945 = {
                                            let a1939 = {
                                                let a1937 = v7;
                                                let a1938 = 8u8;
                                                fixed_u8::bit_and(a1937, a1938)
                                            };
                                            let a1940 = 8u8;
                                            if equal(a1939, a1940) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1946 = {
                                            let a1943 = {
                                                let a1941 = v7;
                                                let a1942 = 4u8;
                                                fixed_u8::bit_and(a1941, a1942)
                                            };
                                            let a1944 = 4u8;
                                            if equal(a1943, a1944) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1945, a1946)?
                                    };
                                    let a1958 = {
                                        let a1955 = {
                                            let a1949 = {
                                                let a1947 = v7;
                                                let a1948 = 2u8;
                                                fixed_u8::bit_and(a1947, a1948)
                                            };
                                            let a1950 = 2u8;
                                            if equal(a1949, a1950) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1956 = {
                                            let a1953 = {
                                                let a1951 = v7;
                                                let a1952 = 1u8;
                                                fixed_u8::bit_and(a1951, a1952)
                                            };
                                            let a1954 = 1u8;
                                            if equal(a1953, a1954) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1955, a1956)?
                                    };
                                    nat_add(a1957, a1958)?
                                };
                                nat_add(a1959, a1960)?
                            };
                            if f414(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1961 = v1;
                                    let a1962 = 1u64;
                                    nat_add(a1961, a1962)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f415(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1963 = v1;
                                        let a1964 = 1u64;
                                        nat_add(a1963, a1964)?
                                    }, (v5, (false, {
                                        let a1968 = v4;
                                        let a1969 = f32({
                                            let a1965 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1965)
                                        }, {
                                            let a1966 = v8;
                                            let a1967 = 97u64;
                                            nat_sub(a1966, a1967)
                                        }, 1u64);
                                        append_bytes(a1968, a1969)
                                    }))))
                                } else {
                                    if if f415(v8) {
                                        true
                                    } else {
                                        if f416(v8) {
                                            true
                                        } else {
                                            f417(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1970 = v1;
                                            let a1971 = 1u64;
                                            nat_add(a1970, a1971)?
                                        }, (v5, (false, {
                                            let a1972 = v4;
                                            let a1973 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1972, a1973)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1974 = v1;
                                            let a1975 = 1u64;
                                            nat_add(a1974, a1975)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Adt19, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt19)> = f418(f419(f150(v0.clone(), f8(Str::lit("$defs")))), f419(f150(v0.clone(), f8(Str::lit("definitions")))));
    let m1981 = f420(4096u64, v3.clone(), List::<Bytes>::nil(), f421(v3)?)?;
    match m1981 {
        Ok(v5) => {
            let m1982 = f422(4096u64, f423(f424(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1982 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f217(f188(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f351(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1983 = v0;
    match m1983.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f36(f160(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f425(v5, v1, v2))
            } else {
                List::cons(v4, f351(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f352(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f217(f426(f84(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f353(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1984 = 0u64;
        let a1985 = v1;
        nat_lt(a1984, a1985)
    } else {
        false
    } {
        let a1986 = Bytes::lit(&[0x2d]);
        let a1987 = f127(v1)?;
        Ok(append_bytes(a1986, a1987))
    } else {
        f127(v1)
    }
}

pub fn f354(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f127(v1)?;
    let v4: u64 = {
        let a1988 = v3.clone();
        length_bytes(a1988)
    };
    let v5: Bytes = {
        let a1989 = v2;
        let a1990 = v4;
        if nat_lt(a1989, a1990) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1991 = v4;
                let a1992 = v2;
                nat_sub(a1991, a1992)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1993 = v4;
                let a1994 = v2;
                nat_sub(a1993, a1994)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f427({
                let a1995 = v2;
                let a1996 = v4;
                nat_sub(a1995, a1996)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1997 = f8(Str::lit("-"));
        let a1998 = v5;
        Ok(append_bytes(a1997, a1998))
    } else {
        Ok(v5)
    }
}

pub fn f355(v0: Bytes) -> R<Bytes> {
    let a2004 = {
        let a2002 = Bytes::lit(&[0x22]);
        let a2003 = {
            let (_, h1999) = {
                let (h2000, _) = f316(Fn5::F428(v0.clone()), {
                    let a2001 = v0;
                    length_bytes(a2001)
                }, (0u64, Bytes::lit(&[])))?;
                h2000
            };
            h1999
        };
        append_bytes(a2002, a2003)
    };
    let a2005 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2004, a2005))
}

pub fn f356(v0: List<Bytes>) -> Bytes {
    let a2008 = {
        let a2006 = Bytes::lit(&[0x5b]);
        let a2007 = f122(v0, Bytes::lit(&[0x2c]));
        append_bytes(a2006, a2007)
    };
    let a2009 = Bytes::lit(&[0x5d]);
    append_bytes(a2008, a2009)
}

pub fn f357(v0: Adt19) -> R<List<Bytes>> {
    let m2010 = v0;
    match m2010 {
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
        Adt19::C7(r2011, r2012) => {
            let v10 = (*r2011).clone();
            let v11 = (*r2012).clone();
            Ok(List::cons(f257(v10)?, f357(v11)?))
        }
        Adt19::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f358(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f429(Fn10::F430, List::<(Bytes, Bytes)>::nil(), f431(v0)?);
    let v2: List<Bytes> = f432(Fn11::F433, List::<Bytes>::nil(), v1)?;
    let a2015 = {
        let a2013 = Bytes::lit(&[0x7b]);
        let a2014 = f122(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2013, a2014)
    };
    let a2016 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2015, a2016))
}

pub fn f359(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m2017 = v0;
    match m2017 {
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
        Adt19::C8(v12, r2018, r2019) => {
            let v13 = (*r2018).clone();
            let v14 = (*r2019).clone();
            Ok(List::cons((v12, f257(v13)?), f359(v14)?))
        }
    }
}

pub fn f360(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m2020 = v0;
    match m2020 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f160(v2, v1)
        }
    }
}

pub fn f361(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f217(f434(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f362(v0: Adt16, v1: Option<u64>) -> bool {
    let m2021 = v1;
    match m2021 {
        None => {
            true
        }
        Some(v2) => {
            f275(v2, match v0 {
                Adt16::C0(_, h2022, _, _) => {
                    h2022
                }
            })
        }
    }
}

pub fn f363(v0: Adt12) -> Adt21 {
    Adt21::C1({
        let a2024 = match v0 {
            Adt12::C0(_, _, h2023, _, _) => {
                h2023
            }
        };
        length_list(a2024)
    })
}

pub fn f364(v0: Bytes) -> Option<Adt22> {
    if f36(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt22::C0)
    } else {
        if f36(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt22::C1)
        } else {
            if f36(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt22::C2)
            } else {
                None::<Adt22>
            }
        }
    }
}

pub fn f365(v0: Bytes, p2029: List<Bytes>, p2030: u64) -> R<Option<u64>> {
    let m2028 = (p2029, p2030);
    {
        let mut m2028 = m2028;
        loop {
            let step = {
                let (v1, v2) = m2028;
                let m2025 = v1;
                match m2025.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2026 = v2;
                                let a2027 = 1u64;
                                nat_add(a2026, a2027)?
                            }))
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

pub fn f366(v0: Option<Adt14>) -> Option<u64> {
    let m2031 = v0;
    match m2031 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2032 = v1;
            match m2032 {
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

pub fn f367(v0: Fn6, p2036: List<Bytes>, p2037: List<(Bytes, ())>) -> List<Bytes> {
    let m2035 = (p2036, p2037);
    {
        let mut m2035 = m2035;
        loop {
            let step = {
                let (v1, v2) = m2035;
                let m2033 = v2;
                match m2033.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c2034: Fn6 = v0.clone();
                            c2034.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h2038, _) = v1;
        h2038
    }, v0)
}

pub fn f369(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f435(Fn12::F436, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f370(v0: Fn7, p2042: List<List<(Bytes, ())>>, p2043: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m2041 = (p2042, p2043);
    {
        let mut m2041 = m2041;
        loop {
            let step = {
                let (v1, v2) = m2041;
                let m2039 = v2;
                match m2039.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c2040: Fn7 = v0.clone();
                            c2040.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f372(v0: Fn8, p2048: u64, p2049: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m2047 = (p2048, p2049);
    {
        let mut m2047 = m2047;
        loop {
            let step = {
                let (v1, v2) = m2047;
                let m2044 = v1;
                if m2044 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2044 - 1;
                    let m2046 = {
                        let c2045: Fn8 = v0.clone();
                        c2045.apply(v2.clone())?
                    };
                    match m2046 {
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
                Ok(next) => { m2047 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f437(v0)
}

pub fn f374(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f438(Fn13::F439, List::<(Bytes, ())>::nil(), v0)
}

pub fn f375(v0: Fn9, p2053: List<(Bytes, ())>, p2054: List<Bytes>) -> List<(Bytes, ())> {
    let m2052 = (p2053, p2054);
    {
        let mut m2052 = m2052;
        loop {
            let step = {
                let (v1, v2) = m2052;
                let m2050 = v2;
                match m2050.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c2051: Fn9 = v0.clone();
                            c2051.apply(v1, v3)
                        }, v4))
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

pub fn f376(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f377(v0: Bytes) -> Bytes {
    if f137(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a2056 = {
                let a2055 = v0;
                length_bytes(a2055)
            };
            let a2057 = 1u64;
            nat_sub(a2056, a2057)
        })
    } else {
        v0
    }
}

pub fn f378(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f84(f71(v0.clone(), v1.clone()))?;
    let m2058 = f156(v2.clone())?;
    match m2058 {
        None => {
            f441(v0, v1, v2)
        }
        Some(v3) => {
            f440(v0, v1, v3)
        }
    }
}

pub fn f379(v0: List<Bytes>) -> List<Bytes> {
    let m2059 = v0;
    match m2059.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a2061 = {
                    let a2060 = v1.clone();
                    length_bytes(a2060)
                };
                let a2062 = 0u64;
                if nat_eq(a2061, a2062) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f379(v2))
        }
    }
}

pub fn f380(v0: List<Bytes>) -> Bytes {
    let m2063 = v0;
    match m2063.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f381(v0: (Bytes, Bytes), p2066: List<(Bytes, Bytes)>) -> bool {
    let m2065 = p2066;
    {
        let mut m2065 = m2065;
        loop {
            let step = {
                let v1 = m2065;
                let m2064 = v1;
                match m2064.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f442(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m2067 = v1;
    match m2067.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h2068, _) = v2;
                h2068
            }, v0.clone()) {
                List::cons({
                    let (_, h2069) = v2;
                    h2069
                }, f382(v0, v3))
            } else {
                f382(v0, v3)
            }
        }
    }
}

pub fn f383(v0: u32) -> Bytes {
    let a2072 = {
        let a2070 = v0;
        let a2071 = 8u32;
        fixed_u32::bit_and(a2070, a2071)
    };
    let a2073 = 0u32;
    if equal(a2072, a2073) {
        let a2076 = {
            let a2074 = v0;
            let a2075 = 4u32;
            fixed_u32::bit_and(a2074, a2075)
        };
        let a2077 = 0u32;
        if equal(a2076, a2077) {
            let a2080 = {
                let a2078 = v0;
                let a2079 = 2u32;
                fixed_u32::bit_and(a2078, a2079)
            };
            let a2081 = 0u32;
            if equal(a2080, a2081) {
                let a2084 = {
                    let a2082 = v0;
                    let a2083 = 1u32;
                    fixed_u32::bit_and(a2082, a2083)
                };
                let a2085 = 0u32;
                if equal(a2084, a2085) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2088 = {
                    let a2086 = v0;
                    let a2087 = 1u32;
                    fixed_u32::bit_and(a2086, a2087)
                };
                let a2089 = 0u32;
                if equal(a2088, a2089) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2092 = {
                let a2090 = v0;
                let a2091 = 2u32;
                fixed_u32::bit_and(a2090, a2091)
            };
            let a2093 = 0u32;
            if equal(a2092, a2093) {
                let a2096 = {
                    let a2094 = v0;
                    let a2095 = 1u32;
                    fixed_u32::bit_and(a2094, a2095)
                };
                let a2097 = 0u32;
                if equal(a2096, a2097) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2100 = {
                    let a2098 = v0;
                    let a2099 = 1u32;
                    fixed_u32::bit_and(a2098, a2099)
                };
                let a2101 = 0u32;
                if equal(a2100, a2101) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2104 = {
            let a2102 = v0;
            let a2103 = 4u32;
            fixed_u32::bit_and(a2102, a2103)
        };
        let a2105 = 0u32;
        if equal(a2104, a2105) {
            let a2108 = {
                let a2106 = v0;
                let a2107 = 2u32;
                fixed_u32::bit_and(a2106, a2107)
            };
            let a2109 = 0u32;
            if equal(a2108, a2109) {
                let a2112 = {
                    let a2110 = v0;
                    let a2111 = 1u32;
                    fixed_u32::bit_and(a2110, a2111)
                };
                let a2113 = 0u32;
                if equal(a2112, a2113) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2116 = {
                    let a2114 = v0;
                    let a2115 = 1u32;
                    fixed_u32::bit_and(a2114, a2115)
                };
                let a2117 = 0u32;
                if equal(a2116, a2117) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2120 = {
                let a2118 = v0;
                let a2119 = 2u32;
                fixed_u32::bit_and(a2118, a2119)
            };
            let a2121 = 0u32;
            if equal(a2120, a2121) {
                let a2124 = {
                    let a2122 = v0;
                    let a2123 = 1u32;
                    fixed_u32::bit_and(a2122, a2123)
                };
                let a2125 = 0u32;
                if equal(a2124, a2125) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2128 = {
                    let a2126 = v0;
                    let a2127 = 1u32;
                    fixed_u32::bit_and(a2126, a2127)
                };
                let a2129 = 0u32;
                if equal(a2128, a2129) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f384(v0: u32, v1: u32) -> u32 {
    let m2132 = {
        let a2130 = v0;
        let a2131 = v1;
        fixed_u32::shift_right(a2130, a2131)
    };
    match m2132 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f385(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f443(Fn14::F444, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2133, _, _, _, _, _, _, _) => {
            h2133
        }
    }, match v0.clone() {
        Adt24::C0(_, h2134, _, _, _, _, _, _) => {
            h2134
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2135, _, _, _, _, _) => {
            h2135
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2136, _, _, _, _) => {
            h2136
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2137, _, _, _) => {
            h2137
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2138, _, _) => {
            h2138
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2139, _) => {
            h2139
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2140) => {
            h2140
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f445(match v0.clone() {
        Adt24::C0(h2141, _, _, _, _, _, _, _) => {
            h2141
        }
    }, match v2.clone() {
        Adt27::C0(h2142, _, _, _, _, _, _, _, _, _) => {
            h2142
        }
    }), f445(match v0.clone() {
        Adt24::C0(_, h2143, _, _, _, _, _, _) => {
            h2143
        }
    }, match v2.clone() {
        Adt27::C0(_, h2144, _, _, _, _, _, _, _, _) => {
            h2144
        }
    }), f445(match v0.clone() {
        Adt24::C0(_, _, h2145, _, _, _, _, _) => {
            h2145
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2146, _, _, _, _, _, _, _) => {
            h2146
        }
    }), f445(match v0.clone() {
        Adt24::C0(_, _, _, h2147, _, _, _, _) => {
            h2147
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2148, _, _, _, _, _, _) => {
            h2148
        }
    }), f445(match v0.clone() {
        Adt24::C0(_, _, _, _, h2149, _, _, _) => {
            h2149
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2150, _, _, _, _, _) => {
            h2150
        }
    }), f445(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2151, _, _) => {
            h2151
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2152, _, _, _, _) => {
            h2152
        }
    }), f445(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2153, _) => {
            h2153
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2154, _, _, _) => {
            h2154
        }
    }), f445(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2155) => {
            h2155
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2156, _, _) => {
            h2156
        }
    })))
}

pub fn f386(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f446(v0.clone(), v1, v2, v3)?, f446(v0.clone(), v1, v2, {
        let a2157 = v3;
        let a2158 = 4u64;
        nat_add(a2157, a2158)?
    })?, f446(v0.clone(), v1, v2, {
        let a2159 = v3;
        let a2160 = 8u64;
        nat_add(a2159, a2160)?
    })?, f446(v0.clone(), v1, v2, {
        let a2161 = v3;
        let a2162 = 12u64;
        nat_add(a2161, a2162)?
    })?, f446(v0.clone(), v1, v2, {
        let a2163 = v3;
        let a2164 = 16u64;
        nat_add(a2163, a2164)?
    })?, f446(v0.clone(), v1, v2, {
        let a2165 = v3;
        let a2166 = 20u64;
        nat_add(a2165, a2166)?
    })?, f446(v0.clone(), v1, v2, {
        let a2167 = v3;
        let a2168 = 24u64;
        nat_add(a2167, a2168)?
    })?, f446(v0.clone(), v1, v2, {
        let a2169 = v3;
        let a2170 = 28u64;
        nat_add(a2169, a2170)?
    })?, f446(v0.clone(), v1, v2, {
        let a2171 = v3;
        let a2172 = 32u64;
        nat_add(a2171, a2172)?
    })?, f446(v0.clone(), v1, v2, {
        let a2173 = v3;
        let a2174 = 36u64;
        nat_add(a2173, a2174)?
    })?, f446(v0.clone(), v1, v2, {
        let a2175 = v3;
        let a2176 = 40u64;
        nat_add(a2175, a2176)?
    })?, f446(v0.clone(), v1, v2, {
        let a2177 = v3;
        let a2178 = 44u64;
        nat_add(a2177, a2178)?
    })?, f446(v0.clone(), v1, v2, {
        let a2179 = v3;
        let a2180 = 48u64;
        nat_add(a2179, a2180)?
    })?, f446(v0.clone(), v1, v2, {
        let a2181 = v3;
        let a2182 = 52u64;
        nat_add(a2181, a2182)?
    })?, f446(v0.clone(), v1, v2, {
        let a2183 = v3;
        let a2184 = 56u64;
        nat_add(a2183, a2184)?
    })?, f446(v0, v1, v2, {
        let a2185 = v3;
        let a2186 = 60u64;
        nat_add(a2185, a2186)?
    })?))
}

pub fn f387(p2189: List<Bytes>, v1: Bytes) -> bool {
    let m2188 = p2189;
    {
        let mut m2188 = m2188;
        loop {
            let step = {
                let v0 = m2188;
                let m2187 = v0;
                match m2187.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f207(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388(v0: Bytes, v1: u64) -> Option<u64> {
    let a2190 = 0u64;
    let a2191 = v1;
    if nat_lt(a2190, a2191) {
        if f447(v0, {
            let a2192 = v1;
            let a2193 = 1u64;
            nat_sub(a2192, a2193)
        }) {
            Some({
                let a2194 = v1;
                let a2195 = 1u64;
                nat_sub(a2194, a2195)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f389(v0: Adt14) -> R<List<Bytes>> {
    let m2196 = v0;
    match m2196 {
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
        Adt14::C6(r2197, r2198) => {
            let v7 = (*r2197).clone();
            let v8 = (*r2198).clone();
            Ok(List::cons(f315(v7)?, f389(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f390(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m2199 = v0;
    match m2199 {
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
        Adt14::C7(v9, r2200, r2201) => {
            let v10 = (*r2200).clone();
            let v11 = (*r2201).clone();
            Ok(List::cons((v9, f315(v10)?), f390(v11)?))
        }
    }
}

pub fn f391(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2202, _) = v1;
        h2202
    };
    let a2204 = {
        let a2203 = v0.clone();
        length_bytes(a2203)
    };
    let a2205 = v2;
    if nat_le(a2204, a2205) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2206 = f229(v0.clone(), v2)?;
        let a2207 = 13u64;
        if nat_eq(a2206, a2207) {
            Ok(Some(({
                let a2210 = f229(v0, {
                    let a2208 = v2;
                    let a2209 = 1u64;
                    nat_add(a2208, a2209)?
                })?;
                let a2211 = 10u64;
                if nat_eq(a2210, a2211) {
                    let a2212 = v2;
                    let a2213 = 2u64;
                    nat_add(a2212, a2213)?
                } else {
                    let a2214 = v2;
                    let a2215 = 1u64;
                    nat_add(a2214, a2215)?
                }
            }, {
                let a2217 = {
                    let (_, h2216) = v1;
                    h2216
                };
                let a2218 = Bytes::lit(&[0x0a]);
                append_bytes(a2217, a2218)
            })))
        } else {
            let v4: u64 = {
                let m2219 = f320(v0.clone(), Bytes::lit(&[0x0d]), v2)?;
                match m2219 {
                    None => {
                        let a2220 = v0.clone();
                        length_bytes(a2220)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2224 = {
                    let (_, h2221) = v1;
                    h2221
                };
                let a2225 = f32(v0, v2, {
                    let a2222 = v4;
                    let a2223 = v2;
                    nat_sub(a2222, a2223)
                });
                append_bytes(a2224, a2225)
            })))
        }
    }
}

pub fn f392(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2226 = f448(v0, v1)?;
    match m2226 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f449({
                let (h2227, _) = v2;
                h2227
            }) {
                Ok(Some({
                    let a2229 = v1;
                    let a2230 = {
                        let (_, h2228) = v2;
                        h2228
                    };
                    nat_add(a2229, a2230)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f393(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f450(v1, v0, v2)
}

pub fn f394(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f451(v0, v1)
}

pub fn f395(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f452(v0, v1, v2)
}

pub fn f396(v0: Bytes, p2233: List<Bytes>) -> bool {
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
                        if f36(v2, v0.clone()) {
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

pub fn f397(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f228(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2236 = f229(v0.clone(), {
                let a2234 = v1;
                let a2235 = 2u64;
                nat_add(a2234, a2235)?
            })?;
            let a2237 = 120u64;
            nat_eq(a2236, a2237)
        };
        let v3: u64 = if v2 {
            let a2238 = v1;
            let a2239 = 3u64;
            nat_add(a2238, a2239)?
        } else {
            let a2240 = v1;
            let a2241 = 2u64;
            nat_add(a2240, a2241)?
        };
        let v4: (u64, u64) = f453(v0.clone(), v2, v3)?;
        let a2243 = {
            let (h2242, _) = v4;
            h2242
        };
        let a2244 = v3;
        if if nat_eq(a2243, a2244) {
            true
        } else {
            let a2246 = f229(v0, {
                let (h2245, _) = v4;
                h2245
            })?;
            let a2247 = 59u64;
            !nat_eq(a2246, a2247)
        } {
            Ok(Err::<(Bytes, u64), Bytes>(f225(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f449({
                let (_, h2248) = v4;
                h2248
            }) {
                Ok(Ok::<(Bytes, u64), Bytes>((f454({
                    let (_, h2249) = v4;
                    h2249
                })?, {
                    let a2251 = {
                        let (h2250, _) = v4;
                        h2250
                    };
                    let a2252 = 1u64;
                    nat_add(a2251, a2252)?
                })))
            } else {
                Ok(Err::<(Bytes, u64), Bytes>(f225(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f228(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3c]), {
                let a2253 = v1;
                let a2254 = 4u64;
                nat_add(a2253, a2254)?
            })))
        } else {
            if f228(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3e]), {
                    let a2255 = v1;
                    let a2256 = 4u64;
                    nat_add(a2255, a2256)?
                })))
            } else {
                if f228(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x26]), {
                        let a2257 = v1;
                        let a2258 = 5u64;
                        nat_add(a2257, a2258)?
                    })))
                } else {
                    if f228(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x27]), {
                            let a2259 = v1;
                            let a2260 = 6u64;
                            nat_add(a2259, a2260)?
                        })))
                    } else {
                        if f228(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x22]), {
                                let a2261 = v1;
                                let a2262 = 6u64;
                                nat_add(a2261, a2262)?
                            })))
                        } else {
                            Ok(Err::<(Bytes, u64), Bytes>(f225(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f398(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2263, _) = f266(Fn3::F455(v1, v2, v0.clone()), {
        let a2265 = {
            let a2264 = v0;
            length_bytes(a2264)
        };
        let a2266 = 1u64;
        nat_add(a2265, a2266)?
    }, v3)?;
    Ok(h2263)
}

pub fn f399(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2267, _) = v2;
        h2267
    };
    let a2268 = v3;
    let a2269 = 0u64;
    if nat_eq(a2268, a2269) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2270 = v3;
            let a2271 = v1;
            let a2272 = 0u64;
            nat_quot(a2270, a2271, a2272)
        }, {
            let a2277 = f329(v0, {
                let a2273 = v3;
                let a2274 = v1;
                let a2275 = 0u64;
                nat_rem(a2273, a2274, a2275)
            });
            let a2278 = {
                let (_, h2276) = v2;
                h2276
            };
            append_bytes(a2277, a2278)
        }))
    }
}

pub fn f400(p2289: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>> {
    let m2288 = p2289;
    {
        let mut m2288 = m2288;
        loop {
            let step = {
                let v0 = m2288;
                let m2279 = v0;
                match m2279 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C1(_, _, _) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C3(r2286, r2287) => {
                        let v2 = (*r2286).clone();
                        let v3 = (*r2287).clone();
                        let m2280 = v2;
                        match m2280 {
                            Adt18::C0(v4, r2284, r2285) => {
                                let v5 = (*r2284).clone();
                                let v6 = (*r2285).clone();
                                let m2281 = f233(v5.clone(), v1.clone());
                                match m2281 {
                                    Ok(v8) => {
                                        let m2282 = f234(v8.clone(), v4)?;
                                        match m2282 {
                                            Ok(v10) => {
                                                if if f235(v10.clone(), Str::lit("restriction")) {
                                                    true
                                                } else {
                                                    if f235(v10.clone(), Str::lit("list")) {
                                                        true
                                                    } else {
                                                        f235(v10.clone(), Str::lit("union"))
                                                    }
                                                } {
                                                    Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(Some(({
                                                        let (_, h2283) = v10;
                                                        h2283
                                                    }, (v5, (v6, v8)))))))
                                                } else {
                                                    Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Err::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Err::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(v7)))
                                    }
                                }
                            }
                            Adt18::C1(_, _, _) => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt18::C2(_) => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt18::C3(_, _) => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt18::C4 => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt18::C4 => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                }
            };
            match step {
                Ok(next) => { m2288 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f401(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f402(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m2290 = f234(v0, v2.clone())?;
    match m2290 {
        Ok(v4) => {
            if f36({
                let (h2291, _) = v4.clone();
                h2291
            }, f336()) {
                let m2293 = f456({
                    let (_, h2292) = v4;
                    h2292
                }, 16u64);
                match m2293 {
                    Ok(v6) => {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v5))
                    }
                }
            } else {
                if f36({
                    let (h2294, _) = v4.clone();
                    h2294
                }, v1) {
                    let v7: Bytes = f254({
                        let (_, h2295) = v4;
                        h2295
                    })?;
                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h2296, _) = v4;
                        h2296
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(v3))
        }
    }
}

pub fn f403(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f404(v0: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt19>, Bytes>> {
    let m2297 = v0;
    match m2297 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C3(r2304, r2305) => {
            let v2 = (*r2304).clone();
            let v3 = (*r2305).clone();
            let m2298 = f404(v3, v1.clone())?;
            match m2298 {
                Ok(v5) => {
                    let m2299 = v2;
                    match m2299 {
                        Adt18::C0(v6, r2303, _) => {
                            let v7 = (*r2303).clone();
                            let m2300 = f233(v7.clone(), v1);
                            match m2300 {
                                Ok(v10) => {
                                    let m2301 = f234(v10, v6)?;
                                    match m2301 {
                                        Ok(v12) => {
                                            if f235(v12, Str::lit("enumeration")) {
                                                let m2302 = f236(v7, f8(Str::lit("value")));
                                                match m2302 {
                                                    None => {
                                                        Ok(Err::<List<Adt19>, Bytes>(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt19>, Bytes>(List::cons(Adt19::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt19>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt19>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt19>, Bytes>(v9))
                                }
                            }
                        }
                        Adt18::C1(_, _, _) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt18::C2(_) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt18::C3(_, _) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt18::C4 => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt19>, Bytes>(v4))
                }
            }
        }
        Adt18::C4 => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
    }
}

pub fn f405(v0: List<Adt19>, v1: List<Bytes>) -> R<List<Adt19>> {
    let m2306 = v0;
    match m2306.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f457(v2.clone())?;
            if f87(v4.clone(), v1.clone()) {
                f405(v3, v1)
            } else {
                Ok(List::cons(v2, f405(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f406(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2307 = v1;
    match m2307.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2308 = f458(v0, v3.clone())?;
            match m2308 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f457(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f87(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2309 = f406(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2309 {
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

pub fn f407(p2313: List<Bytes>) -> R<bool> {
    let m2312 = p2313;
    {
        let mut m2312 = m2312;
        loop {
            let step = {
                let v0 = m2312;
                let m2310 = v0;
                match m2310.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2311 = f459(v1)?;
                        match m2311 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f87(v3, f460(false, v2.clone())?) {
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
                Ok(next) => { m2312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f217(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f409(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a2314 = v2;
    let a2315 = v0;
    if nat_lt(a2314, a2315) {
        if f447(v1, v2) {
            Ok(Some({
                let a2316 = v2;
                let a2317 = 1u64;
                nat_add(a2316, a2317)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f410(v0: Bytes, p2397: u64, p2398: u64, p2399: bool, p2400: bool, p2401: Bytes) -> R<Bytes> {
    let m2396 = (p2397, (p2398, (p2399, (p2400, p2401))));
    {
        let mut m2396 = m2396;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m2396;
                let m2318 = v2;
                if m2318 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m2318 - 1;
                    let m2321 = {
                        let a2319 = v0.clone();
                        let a2320 = v1;
                        index_bytes(a2319, a2320)
                    };
                    match m2321 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a2366 = {
                                    let a2342 = {
                                        let a2330 = {
                                            let a2324 = {
                                                let a2322 = v8;
                                                let a2323 = 128u8;
                                                fixed_u8::bit_and(a2322, a2323)
                                            };
                                            let a2325 = 128u8;
                                            if equal(a2324, a2325) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2331 = {
                                            let a2328 = {
                                                let a2326 = v8;
                                                let a2327 = 64u8;
                                                fixed_u8::bit_and(a2326, a2327)
                                            };
                                            let a2329 = 64u8;
                                            if equal(a2328, a2329) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2330, a2331)?
                                    };
                                    let a2343 = {
                                        let a2340 = {
                                            let a2334 = {
                                                let a2332 = v8;
                                                let a2333 = 32u8;
                                                fixed_u8::bit_and(a2332, a2333)
                                            };
                                            let a2335 = 32u8;
                                            if equal(a2334, a2335) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2341 = {
                                            let a2338 = {
                                                let a2336 = v8;
                                                let a2337 = 16u8;
                                                fixed_u8::bit_and(a2336, a2337)
                                            };
                                            let a2339 = 16u8;
                                            if equal(a2338, a2339) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2340, a2341)?
                                    };
                                    nat_add(a2342, a2343)?
                                };
                                let a2367 = {
                                    let a2364 = {
                                        let a2352 = {
                                            let a2346 = {
                                                let a2344 = v8;
                                                let a2345 = 8u8;
                                                fixed_u8::bit_and(a2344, a2345)
                                            };
                                            let a2347 = 8u8;
                                            if equal(a2346, a2347) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2353 = {
                                            let a2350 = {
                                                let a2348 = v8;
                                                let a2349 = 4u8;
                                                fixed_u8::bit_and(a2348, a2349)
                                            };
                                            let a2351 = 4u8;
                                            if equal(a2350, a2351) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2352, a2353)?
                                    };
                                    let a2365 = {
                                        let a2362 = {
                                            let a2356 = {
                                                let a2354 = v8;
                                                let a2355 = 2u8;
                                                fixed_u8::bit_and(a2354, a2355)
                                            };
                                            let a2357 = 2u8;
                                            if equal(a2356, a2357) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2363 = {
                                            let a2360 = {
                                                let a2358 = v8;
                                                let a2359 = 1u8;
                                                fixed_u8::bit_and(a2358, a2359)
                                            };
                                            let a2361 = 1u8;
                                            if equal(a2360, a2361) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2362, a2363)?
                                    };
                                    nat_add(a2364, a2365)?
                                };
                                nat_add(a2366, a2367)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a2368 = v1;
                                    let a2369 = 1u64;
                                    nat_add(a2368, a2369)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a2370 = v9;
                                    let a2371 = 34u64;
                                    !nat_eq(a2370, a2371)
                                }, (if v4 {
                                    false
                                } else {
                                    let a2372 = v9;
                                    let a2373 = 92u64;
                                    nat_eq(a2372, a2373)
                                }, {
                                    let a2374 = v5;
                                    let a2375 = v10;
                                    append_bytes(a2374, a2375)
                                })))))
                            } else {
                                let a2376 = v9;
                                let a2377 = 32u64;
                                if if if if nat_eq(a2376, a2377) {
                                    true
                                } else {
                                    let a2378 = v9;
                                    let a2379 = 9u64;
                                    nat_eq(a2378, a2379)
                                } {
                                    true
                                } else {
                                    let a2380 = v9;
                                    let a2381 = 10u64;
                                    nat_eq(a2380, a2381)
                                } {
                                    true
                                } else {
                                    let a2382 = v9;
                                    let a2383 = 13u64;
                                    nat_eq(a2382, a2383)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a2384 = v1;
                                        let a2385 = 1u64;
                                        nat_add(a2384, a2385)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2386 = v1;
                                            let a2387 = 4u64;
                                            nat_add(a2386, a2387)?
                                        }, (v6, (false, (false, {
                                            let a2388 = v5;
                                            let a2389 = f8(Str::lit("[]"));
                                            append_bytes(a2388, a2389)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2390 = v1;
                                            let a2391 = 1u64;
                                            nat_add(a2390, a2391)?
                                        }, (v6, ({
                                            let a2392 = v9;
                                            let a2393 = 34u64;
                                            nat_eq(a2392, a2393)
                                        }, (false, {
                                            let a2394 = v5;
                                            let a2395 = v10;
                                            append_bytes(a2394, a2395)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f461(Fn15::F462(v0.clone()), {
        let a2403 = {
            let a2402 = v0;
            length_bytes(a2402)
        };
        let a2404 = 1u64;
        nat_add(a2403, a2404)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a2408 = {
        let (_, h2405) = {
            let (_, h2406) = {
                let (h2407, _) = v2.clone();
                h2407
            };
            h2406
        };
        h2405
    };
    let a2409 = 1u64;
    if nat_eq(a2408, a2409) {
        Ok(Some(({
            let (h2410, _) = {
                let (h2411, _) = v2.clone();
                h2411
            };
            h2410
        }, {
            let (h2412, _) = {
                let (_, h2413) = {
                    let (h2414, _) = v2;
                    h2414
                };
                h2413
            };
            h2412
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f412(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2415 = f325(v0.clone(), v1)?;
        let a2416 = 45u64;
        nat_eq(a2415, a2416)
    };
    let v3: u64 = if v2 {
        let a2417 = v1;
        let a2418 = 1u64;
        nat_add(a2417, a2418)?
    } else {
        v1
    };
    let v4: u64 = f463(v0.clone(), v3)?;
    let v5: u64 = {
        let a2419 = v4;
        let a2420 = v3;
        nat_sub(a2419, a2420)
    };
    let v6: bool = {
        let a2421 = f325(v0.clone(), v4)?;
        let a2422 = 46u64;
        nat_eq(a2421, a2422)
    };
    let v7: u64 = if v6 {
        let a2423 = v4;
        let a2424 = 1u64;
        nat_add(a2423, a2424)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f463(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f325(v0.clone(), v8)?;
    let v10: bool = {
        let a2425 = v9;
        let a2426 = 101u64;
        if nat_eq(a2425, a2426) {
            true
        } else {
            let a2427 = v9;
            let a2428 = 69u64;
            nat_eq(a2427, a2428)
        }
    };
    let v11: u64 = f325(v0.clone(), {
        let a2429 = v8;
        let a2430 = 1u64;
        nat_add(a2429, a2430)?
    })?;
    let v12: bool = if v10 {
        let a2431 = v11;
        let a2432 = 43u64;
        if nat_eq(a2431, a2432) {
            true
        } else {
            let a2433 = v11;
            let a2434 = 45u64;
            nat_eq(a2433, a2434)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2435 = v8;
            let a2436 = 2u64;
            nat_add(a2435, a2436)?
        } else {
            let a2437 = v8;
            let a2438 = 1u64;
            nat_add(a2437, a2438)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f463(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2439 = v14;
        let a2440 = v13;
        nat_sub(a2439, a2440)
    });
    let v16: u64 = f464(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2442 = {
            let a2441 = v15;
            length_bytes(a2441)
        };
        let a2443 = v16;
        nat_sub(a2442, a2443)
    });
    let a2444 = v5;
    let a2445 = 0u64;
    if if nat_eq(a2444, a2445) {
        true
    } else {
        let a2446 = 1u64;
        let a2447 = v5;
        if if nat_lt(a2446, a2447) {
            let a2448 = f325(v0.clone(), v3)?;
            let a2449 = 48u64;
            nat_eq(a2448, a2449)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2450 = v8;
                let a2451 = v7;
                nat_eq(a2450, a2451)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2452 = v14;
                    let a2453 = v13;
                    nat_eq(a2452, a2453)
                } else {
                    false
                } {
                    true
                } else {
                    let a2455 = 18u64;
                    let a2456 = {
                        let a2454 = v17.clone();
                        length_bytes(a2454)
                    };
                    nat_lt(a2455, a2456)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2460 = {
            let a2458 = {
                let a2457 = v17.clone();
                length_bytes(a2457)
            };
            let a2459 = 0u64;
            if nat_eq(a2458, a2459) {
                Some(0u64)
            } else {
                f465(v17)?
            }
        };
        match m2460 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2465 = f466(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2461 = v8;
                    let a2462 = v7;
                    nat_sub(a2461, a2462)
                }), if v12 {
                    let a2463 = v11;
                    let a2464 = 45u64;
                    nat_eq(a2463, a2464)
                } else {
                    false
                }, v18)?;
                match m2465 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2467 = 19u64;
                        let a2468 = {
                            let a2466 = v19.clone();
                            length_bytes(a2466)
                        };
                        if nat_lt(a2467, a2468) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2469 = f465(v19)?;
                            match m2469 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2470 = 0u64;
                                        let a2471 = v20;
                                        nat_lt(a2470, a2471)
                                    } else {
                                        false
                                    };
                                    if f467(v21, v20) {
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

pub fn f413(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2472 = f325(v0.clone(), v1)?;
        let a2473 = 45u64;
        nat_eq(a2472, a2473)
    };
    let v3: u64 = if v2 {
        let a2474 = v1;
        let a2475 = 1u64;
        nat_add(a2474, a2475)?
    } else {
        v1
    };
    let v4: u64 = f463(v0.clone(), v3)?;
    let v5: u64 = {
        let a2476 = v4;
        let a2477 = v3;
        nat_sub(a2476, a2477)
    };
    let v6: bool = {
        let a2478 = f325(v0.clone(), v4)?;
        let a2479 = 46u64;
        nat_eq(a2478, a2479)
    };
    let v7: u64 = if v6 {
        let a2480 = v4;
        let a2481 = 1u64;
        nat_add(a2480, a2481)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f463(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f325(v0.clone(), v8)?;
    let v10: bool = {
        let a2482 = v9;
        let a2483 = 101u64;
        if nat_eq(a2482, a2483) {
            true
        } else {
            let a2484 = v9;
            let a2485 = 69u64;
            nat_eq(a2484, a2485)
        }
    };
    let v11: u64 = f325(v0.clone(), {
        let a2486 = v8;
        let a2487 = 1u64;
        nat_add(a2486, a2487)?
    })?;
    let v12: bool = if v10 {
        let a2488 = v11;
        let a2489 = 43u64;
        if nat_eq(a2488, a2489) {
            true
        } else {
            let a2490 = v11;
            let a2491 = 45u64;
            nat_eq(a2490, a2491)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2492 = v8;
            let a2493 = 2u64;
            nat_add(a2492, a2493)?
        } else {
            let a2494 = v8;
            let a2495 = 1u64;
            nat_add(a2494, a2495)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f463(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2496 = v14;
        let a2497 = v13;
        nat_sub(a2496, a2497)
    });
    let v16: u64 = f464(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2499 = {
            let a2498 = v15;
            length_bytes(a2498)
        };
        let a2500 = v16;
        nat_sub(a2499, a2500)
    });
    let a2501 = v5;
    let a2502 = 0u64;
    if if nat_eq(a2501, a2502) {
        true
    } else {
        let a2503 = 1u64;
        let a2504 = v5;
        if if nat_lt(a2503, a2504) {
            let a2505 = f325(v0.clone(), v3)?;
            let a2506 = 48u64;
            nat_eq(a2505, a2506)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2507 = v8;
                let a2508 = v7;
                nat_eq(a2507, a2508)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2509 = v14;
                    let a2510 = v13;
                    nat_eq(a2509, a2510)
                } else {
                    false
                } {
                    true
                } else {
                    let a2512 = 18u64;
                    let a2513 = {
                        let a2511 = v17.clone();
                        length_bytes(a2511)
                    };
                    nat_lt(a2512, a2513)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2517 = {
            let a2515 = {
                let a2514 = v17.clone();
                length_bytes(a2514)
            };
            let a2516 = 0u64;
            if nat_eq(a2515, a2516) {
                Some(0u64)
            } else {
                f465(v17)?
            }
        };
        match m2517 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2522 = f468(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2518 = v8;
                    let a2519 = v7;
                    nat_sub(a2518, a2519)
                }), if v12 {
                    let a2520 = v11;
                    let a2521 = 45u64;
                    nat_eq(a2520, a2521)
                } else {
                    false
                }, v18)?;
                match m2522 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2525 = 19u64;
                        let a2526 = {
                            let a2524 = {
                                let (h2523, _) = v19.clone();
                                h2523
                            };
                            length_bytes(a2524)
                        };
                        if nat_lt(a2525, a2526) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2528 = f465({
                                let (h2527, _) = v19.clone();
                                h2527
                            })?;
                            match m2528 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2529 = 0u64;
                                        let a2530 = v20;
                                        nat_lt(a2529, a2530)
                                    } else {
                                        false
                                    };
                                    if f467(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2532 = {
                                                let (_, h2531) = v19.clone();
                                                h2531
                                            };
                                            let a2533 = 0u64;
                                            if nat_eq(a2532, a2533) {
                                                Adt19::C1(v21, v20)
                                            } else {
                                                Adt19::C2(v21, v20, {
                                                    let (_, h2534) = v19;
                                                    h2534
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

pub fn f414(v0: u64) -> bool {
    let a2535 = v0;
    let a2536 = 95u64;
    if nat_eq(a2535, a2536) {
        true
    } else {
        let a2537 = v0;
        let a2538 = 45u64;
        if nat_eq(a2537, a2538) {
            true
        } else {
            let a2539 = v0;
            let a2540 = 32u64;
            if nat_eq(a2539, a2540) {
                true
            } else {
                let a2541 = v0;
                let a2542 = 46u64;
                if nat_eq(a2541, a2542) {
                    true
                } else {
                    let a2543 = v0;
                    let a2544 = 47u64;
                    nat_eq(a2543, a2544)
                }
            }
        }
    }
}

pub fn f415(v0: u64) -> bool {
    let a2545 = 97u64;
    let a2546 = v0;
    if nat_le(a2545, a2546) {
        let a2547 = v0;
        let a2548 = 122u64;
        nat_le(a2547, a2548)
    } else {
        false
    }
}

pub fn f416(v0: u64) -> bool {
    let a2549 = 65u64;
    let a2550 = v0;
    if nat_le(a2549, a2550) {
        let a2551 = v0;
        let a2552 = 90u64;
        nat_le(a2551, a2552)
    } else {
        false
    }
}

pub fn f417(v0: u64) -> bool {
    let a2553 = 48u64;
    let a2554 = v0;
    if nat_le(a2553, a2554) {
        let a2555 = v0;
        let a2556 = 57u64;
        nat_le(a2555, a2556)
    } else {
        false
    }
}

pub fn f418(v0: List<(Bytes, Adt19)>, v1: List<(Bytes, Adt19)>) -> List<(Bytes, Adt19)> {
    let m2557 = v0;
    match m2557.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f418(v3, v1))
        }
    }
}

pub fn f419(v0: Option<Adt19>) -> List<(Bytes, Adt19)> {
    let m2558 = v0;
    match m2558 {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some(v1) => {
            f469(v1)
        }
    }
}

pub fn f420(v0: u64, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt19)>, Bytes>> {
    let m2559 = v0;
    if m2559 == 0 {
        let m2560 = v1;
        match m2560.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2559 - 1;
        let m2561 = v1.clone();
        match m2561.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                let m2562 = f470(v1.clone(), v2.clone(), v3.clone())?;
                match m2562 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f217(f421(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2565 = f420(v6, f471(v1, {
                            let (h2563, _) = v9.clone();
                            h2563
                        }), List::cons(f254({
                            let (h2564, _) = v9.clone();
                            h2564
                        })?, v2), v3)?;
                        match m2565 {
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

pub fn f421(v0: List<(Bytes, Adt19)>) -> R<List<Bytes>> {
    let m2566 = v0;
    match m2566.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f254({
                let (h2567, _) = v1;
                h2567
            })?, f421(v2)?))
        }
    }
}

pub fn f422(p2575: u64, p2576: List<Adt25>, p2577: List<Bytes>, p2578: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2574 = (p2575, (p2576, (p2577, p2578)));
    {
        let mut m2574 = m2574;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2574;
                let m2568 = v0;
                if m2568 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2568 - 1;
                    let m2569 = v1;
                    match m2569.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2570 = v5;
                            match m2570 {
                                Adt25::C0(v8, v9) => {
                                    if f36(v8.clone(), f472()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f87(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2571 = f474(v8.clone(), v9)?;
                                            match m2571 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f423({
                                                        let (_, h2572) = v11;
                                                        h2572
                                                    }, List::cons(Adt25::C1({
                                                        let (h2573, _) = v11;
                                                        h2573
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
                                    if f87(f472(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f472(), v2), List::cons(f473(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2574 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f423(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2579 = v0;
    match m2579.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f423(v3, v1))
        }
    }
}

pub fn f424(v0: List<(Bytes, Adt19)>) -> R<List<Adt25>> {
    let m2580 = v0;
    match m2580.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f254({
                let (h2581, _) = v1;
                h2581
            })?, {
                let (_, h2582) = v1;
                h2582
            }), f424(v2)?))
        }
    }
}

pub fn f425(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2583 = v0;
    match m2583.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f36(f160(v3.clone(), v1.clone()), v2.clone()) {
                f425(v4, v1, v2)
            } else {
                List::cons(v3, f425(v4, v1, v2))
            }
        }
    }
}

pub fn f426(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2584 = v0;
    match m2584.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f207(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f217(f434(f79(List::cons(v1, f475(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f426(v3, v1)?))
            }
        }
    }
}

pub fn f427(v0: u64) -> Bytes {
    let m2585 = v0;
    if m2585 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2585 - 1;
        let a2586 = f8(Str::lit("0"));
        let a2587 = f427(v1);
        append_bytes(a2586, a2587)
    }
}

pub fn f428(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f476(v0, v1)
}

pub fn f429(v0: Fn10, p2591: List<(Bytes, Bytes)>, p2592: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2590 = (p2591, p2592);
    {
        let mut m2590 = m2590;
        loop {
            let step = {
                let (v1, v2) = m2590;
                let m2588 = v2;
                match m2588.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2589: Fn10 = v0.clone();
                            c2589.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2590 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f430(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f477(v0, v1)
}

pub fn f431(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f478(v0)
}

pub fn f432(v0: Fn11, p2596: List<Bytes>, p2597: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2595 = (p2596, p2597);
    {
        let mut m2595 = m2595;
        loop {
            let step = {
                let (v1, v2) = m2595;
                let m2593 = v2;
                match m2593.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2594: Fn11 = v0.clone();
                            c2594.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f479(v1)?, v0))
}

pub fn f434(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2598 = v0;
    match m2598.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f212(v1)?, f434(v2)?))
        }
    }
}

pub fn f435(v0: Fn12, p2602: List<List<(Bytes, ())>>, p2603: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2601 = (p2602, p2603);
    {
        let mut m2601 = m2601;
        loop {
            let step = {
                let (v1, v2) = m2601;
                let m2599 = v2;
                match m2599.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2600: Fn12 = v0.clone();
                            c2600.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f437(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2605 = {
        let a2604 = v0.clone();
        length_list(a2604)
    };
    let a2606 = 1u64;
    if nat_le(a2605, a2606) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f480(v0)?))
    }
}

pub fn f438(v0: Fn13, p2610: List<(Bytes, ())>, p2611: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2609 = (p2610, p2611);
    {
        let mut m2609 = m2609;
        loop {
            let step = {
                let (v1, v2) = m2609;
                let m2607 = v2;
                match m2607.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2608: Fn13 = v0.clone();
                            c2608.apply(v1, v3)
                        }, v4))
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

pub fn f439(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f440(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f73(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f481(v3.clone(), f73(f74(v0, f90())?, f8(Str::lit("at"))));
    let a2613 = {
        let a2612 = v4.clone();
        length_list(a2612)
    };
    let a2614 = 0u64;
    if nat_eq(a2613, a2614) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2617 = {
            let a2615 = v4;
            length_list(a2615)
        };
        let a2618 = {
            let a2616 = v3;
            length_list(a2616)
        };
        if nat_eq(a2617, a2618) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f482(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f441(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f249(f143(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f249(f143(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f65(f73(v3.clone(), f8(Str::lit("id"))), f73(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f73(f74(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f481(v5.clone(), f65(v6.clone(), f73(f74(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2620 = {
        let a2619 = v7.clone();
        length_list(a2619)
    };
    let a2621 = 0u64;
    if nat_eq(a2620, a2621) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f76({
            let a2624 = {
                let a2622 = v7;
                length_list(a2622)
            };
            let a2625 = {
                let a2623 = v5;
                length_list(a2623)
            };
            nat_eq(a2624, a2625)
        }) {
            Ok((false, f482(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f79(f65(f481(f483(v4.clone()), f70(f71(v0, f8(Str::lit("model/authorities.toml"))))?), f481(f484(v4), f65(v6, f73(v3, f8(Str::lit("id")))))))?;
            let a2627 = {
                let a2626 = v8.clone();
                length_list(a2626)
            };
            let a2628 = 0u64;
            if nat_eq(a2627, a2628) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f482(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f122(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f442(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h2629, _) = v0;
        h2629
    }, {
        let (h2630, _) = v1;
        h2630
    }) {
        f36({
            let (_, h2631) = v0;
            h2631
        }, {
            let (_, h2632) = v1;
            h2632
        })
    } else {
        false
    }
}

pub fn f443(v0: Fn14, p2636: u64, p2637: Adt27) -> R<Adt27> {
    let m2635 = (p2636, p2637);
    {
        let mut m2635 = m2635;
        loop {
            let step = {
                let (v1, v2) = m2635;
                let m2633 = v1;
                if m2633 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2633 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2634: Fn14 = v0.clone();
                        c2634.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f444(v0: Adt27) -> R<Adt27> {
    f485(v0)
}

pub fn f445(v0: u32, v1: u32) -> u32 {
    let m2640 = {
        let a2638 = f486(v0);
        let a2639 = f486(v1);
        fixed_u64::checked_add(a2638, a2639)
    };
    match m2640 {
        None => {
            0u32
        }
        Some(v2) => {
            f487(v2)
        }
    }
}

pub fn f446(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2643 = {
        let a2641 = v3;
        let a2642 = 8u64;
        nat_add(a2641, a2642)?
    };
    let a2644 = v2;
    if nat_eq(a2643, a2644) {
        Ok(f488({
            let a2647 = {
                let a2645 = v1;
                let a2646 = 8u64;
                nat_mul(a2645, a2646)?
            };
            let a2648 = 4294967296u64;
            let a2649 = 0u64;
            nat_quot(a2647, a2648, a2649)
        }))
    } else {
        let a2652 = {
            let a2650 = v3;
            let a2651 = 4u64;
            nat_add(a2650, a2651)?
        };
        let a2653 = v2;
        if nat_eq(a2652, a2653) {
            Ok(f488({
                let a2656 = {
                    let a2654 = v1;
                    let a2655 = 8u64;
                    nat_mul(a2654, a2655)?
                };
                let a2657 = 4294967296u64;
                let a2658 = 0u64;
                nat_rem(a2656, a2657, a2658)
            }))
        } else {
            Ok(f489(f490(v0.clone(), v1, v3), f490(v0.clone(), v1, {
                let a2659 = v3;
                let a2660 = 1u64;
                nat_add(a2659, a2660)?
            }), f490(v0.clone(), v1, {
                let a2661 = v3;
                let a2662 = 2u64;
                nat_add(a2661, a2662)?
            }), f490(v0, v1, {
                let a2663 = v3;
                let a2664 = 3u64;
                nat_add(a2663, a2664)?
            })))
        }
    }
}

pub fn f447(v0: Bytes, v1: u64) -> bool {
    let m2665 = f264(v0, v1);
    match m2665 {
        None => {
            false
        }
        Some(v2) => {
            f491(v2)
        }
    }
}

pub fn f448(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2668 = {
        let a2666 = v0.clone();
        let a2667 = v1;
        index_bytes(a2666, a2667)
    };
    match m2668 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2713 = {
                    let a2689 = {
                        let a2677 = {
                            let a2671 = {
                                let a2669 = v3;
                                let a2670 = 128u8;
                                fixed_u8::bit_and(a2669, a2670)
                            };
                            let a2672 = 128u8;
                            if equal(a2671, a2672) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2678 = {
                            let a2675 = {
                                let a2673 = v3;
                                let a2674 = 64u8;
                                fixed_u8::bit_and(a2673, a2674)
                            };
                            let a2676 = 64u8;
                            if equal(a2675, a2676) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2677, a2678)?
                    };
                    let a2690 = {
                        let a2687 = {
                            let a2681 = {
                                let a2679 = v3;
                                let a2680 = 32u8;
                                fixed_u8::bit_and(a2679, a2680)
                            };
                            let a2682 = 32u8;
                            if equal(a2681, a2682) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2688 = {
                            let a2685 = {
                                let a2683 = v3;
                                let a2684 = 16u8;
                                fixed_u8::bit_and(a2683, a2684)
                            };
                            let a2686 = 16u8;
                            if equal(a2685, a2686) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2687, a2688)?
                    };
                    nat_add(a2689, a2690)?
                };
                let a2714 = {
                    let a2711 = {
                        let a2699 = {
                            let a2693 = {
                                let a2691 = v3;
                                let a2692 = 8u8;
                                fixed_u8::bit_and(a2691, a2692)
                            };
                            let a2694 = 8u8;
                            if equal(a2693, a2694) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2700 = {
                            let a2697 = {
                                let a2695 = v3;
                                let a2696 = 4u8;
                                fixed_u8::bit_and(a2695, a2696)
                            };
                            let a2698 = 4u8;
                            if equal(a2697, a2698) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2699, a2700)?
                    };
                    let a2712 = {
                        let a2709 = {
                            let a2703 = {
                                let a2701 = v3;
                                let a2702 = 2u8;
                                fixed_u8::bit_and(a2701, a2702)
                            };
                            let a2704 = 2u8;
                            if equal(a2703, a2704) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2710 = {
                            let a2707 = {
                                let a2705 = v3;
                                let a2706 = 1u8;
                                fixed_u8::bit_and(a2705, a2706)
                            };
                            let a2708 = 1u8;
                            if equal(a2707, a2708) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2709, a2710)?
                    };
                    nat_add(a2711, a2712)?
                };
                nat_add(a2713, a2714)?
            };
            let a2715 = v4;
            let a2716 = 128u64;
            if nat_lt(a2715, a2716) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2717 = 194u64;
                let a2718 = v4;
                if if nat_le(a2717, a2718) {
                    let a2719 = v4;
                    let a2720 = 223u64;
                    nat_le(a2719, a2720)
                } else {
                    false
                } {
                    let m2723 = f492(v0, {
                        let a2721 = v1;
                        let a2722 = 1u64;
                        nat_add(a2721, a2722)?
                    })?;
                    match m2723 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f493({
                                let a2728 = {
                                    let a2726 = {
                                        let a2724 = v4;
                                        let a2725 = 192u64;
                                        nat_sub(a2724, a2725)
                                    };
                                    let a2727 = 64u64;
                                    nat_mul(a2726, a2727)?
                                };
                                let a2729 = v5;
                                nat_add(a2728, a2729)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2730 = 224u64;
                    let a2731 = v4;
                    if if nat_le(a2730, a2731) {
                        let a2732 = v4;
                        let a2733 = 239u64;
                        nat_le(a2732, a2733)
                    } else {
                        false
                    } {
                        let m2736 = f492(v0.clone(), {
                            let a2734 = v1;
                            let a2735 = 1u64;
                            nat_add(a2734, a2735)?
                        })?;
                        match m2736 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2739 = f492(v0, {
                                    let a2737 = v1;
                                    let a2738 = 2u64;
                                    nat_add(a2737, a2738)?
                                })?;
                                match m2739 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f493({
                                            let a2748 = {
                                                let a2742 = {
                                                    let a2740 = v4;
                                                    let a2741 = 224u64;
                                                    nat_sub(a2740, a2741)
                                                };
                                                let a2743 = 4096u64;
                                                nat_mul(a2742, a2743)?
                                            };
                                            let a2749 = {
                                                let a2746 = {
                                                    let a2744 = v6;
                                                    let a2745 = 64u64;
                                                    nat_mul(a2744, a2745)?
                                                };
                                                let a2747 = v7;
                                                nat_add(a2746, a2747)?
                                            };
                                            nat_add(a2748, a2749)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2750 = 240u64;
                        let a2751 = v4;
                        if if nat_le(a2750, a2751) {
                            let a2752 = v4;
                            let a2753 = 244u64;
                            nat_le(a2752, a2753)
                        } else {
                            false
                        } {
                            let m2756 = f492(v0.clone(), {
                                let a2754 = v1;
                                let a2755 = 1u64;
                                nat_add(a2754, a2755)?
                            })?;
                            match m2756 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2759 = f492(v0.clone(), {
                                        let a2757 = v1;
                                        let a2758 = 2u64;
                                        nat_add(a2757, a2758)?
                                    })?;
                                    match m2759 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2762 = f492(v0, {
                                                let a2760 = v1;
                                                let a2761 = 3u64;
                                                nat_add(a2760, a2761)?
                                            })?;
                                            match m2762 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f493({
                                                        let a2775 = {
                                                            let a2765 = {
                                                                let a2763 = v4;
                                                                let a2764 = 240u64;
                                                                nat_sub(a2763, a2764)
                                                            };
                                                            let a2766 = 262144u64;
                                                            nat_mul(a2765, a2766)?
                                                        };
                                                        let a2776 = {
                                                            let a2773 = {
                                                                let a2767 = v8;
                                                                let a2768 = 4096u64;
                                                                nat_mul(a2767, a2768)?
                                                            };
                                                            let a2774 = {
                                                                let a2771 = {
                                                                    let a2769 = v9;
                                                                    let a2770 = 64u64;
                                                                    nat_mul(a2769, a2770)?
                                                                };
                                                                let a2772 = v10;
                                                                nat_add(a2771, a2772)?
                                                            };
                                                            nat_add(a2773, a2774)?
                                                        };
                                                        nat_add(a2775, a2776)?
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

pub fn f449(v0: u64) -> bool {
    let a2777 = v0;
    let a2778 = 9u64;
    if if if nat_eq(a2777, a2778) {
        true
    } else {
        let a2779 = v0;
        let a2780 = 10u64;
        nat_eq(a2779, a2780)
    } {
        true
    } else {
        let a2781 = v0;
        let a2782 = 13u64;
        nat_eq(a2781, a2782)
    } {
        true
    } else {
        let a2783 = 32u64;
        let a2784 = v0;
        if if if nat_le(a2783, a2784) {
            let a2785 = v0;
            let a2786 = 55295u64;
            nat_le(a2785, a2786)
        } else {
            false
        } {
            true
        } else {
            let a2787 = 57344u64;
            let a2788 = v0;
            if nat_le(a2787, a2788) {
                let a2789 = v0;
                let a2790 = 65533u64;
                nat_le(a2789, a2790)
            } else {
                false
            }
        } {
            true
        } else {
            let a2791 = 65536u64;
            let a2792 = v0;
            if nat_le(a2791, a2792) {
                let a2793 = v0;
                let a2794 = 1114111u64;
                nat_le(a2793, a2794)
            } else {
                false
            }
        }
    }
}

pub fn f450(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f222(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2796 = v2;
        let a2797 = {
            let a2795 = v0;
            length_bytes(a2795)
        };
        if nat_lt(a2796, a2797) {
            Ok(Some({
                let a2798 = v2;
                let a2799 = 1u64;
                nat_add(a2798, a2799)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f451(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f319(f229(v0, v1)?) {
        Ok(Some({
            let a2800 = v1;
            let a2801 = 1u64;
            nat_add(a2800, a2801)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f452(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2802 = f448(v0, v2)?;
    match m2802 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2803 = v2;
            let a2804 = v1;
            if if nat_eq(a2803, a2804) {
                f494({
                    let (h2805, _) = v3;
                    h2805
                })
            } else {
                f495({
                    let (h2806, _) = v3;
                    h2806
                })
            } {
                Ok(Some({
                    let a2808 = v2;
                    let a2809 = {
                        let (_, h2807) = v3;
                        h2807
                    };
                    nat_add(a2808, a2809)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f453(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2810, _) = f496(Fn16::F497(v1, v0.clone()), {
        let a2812 = {
            let a2811 = v0;
            length_bytes(a2811)
        };
        let a2813 = 1u64;
        nat_add(a2812, a2813)?
    }, (v2, 0u64))?;
    Ok(h2810)
}

pub fn f454(v0: u64) -> R<Bytes> {
    let a2814 = v0;
    let a2815 = 128u64;
    if nat_lt(a2814, a2815) {
        Ok(f498(v0))
    } else {
        let a2816 = v0;
        let a2817 = 2048u64;
        if nat_lt(a2816, a2817) {
            let a2828 = f498({
                let a2821 = 192u64;
                let a2822 = {
                    let a2818 = v0;
                    let a2819 = 64u64;
                    let a2820 = 0u64;
                    nat_quot(a2818, a2819, a2820)
                };
                nat_add(a2821, a2822)?
            });
            let a2829 = f498({
                let a2826 = 128u64;
                let a2827 = {
                    let a2823 = v0;
                    let a2824 = 64u64;
                    let a2825 = 0u64;
                    nat_rem(a2823, a2824, a2825)
                };
                nat_add(a2826, a2827)?
            });
            Ok(append_bytes(a2828, a2829))
        } else {
            let a2830 = v0;
            let a2831 = 65536u64;
            if nat_lt(a2830, a2831) {
                let a2852 = f498({
                    let a2835 = 224u64;
                    let a2836 = {
                        let a2832 = v0;
                        let a2833 = 4096u64;
                        let a2834 = 0u64;
                        nat_quot(a2832, a2833, a2834)
                    };
                    nat_add(a2835, a2836)?
                });
                let a2853 = {
                    let a2850 = f498({
                        let a2843 = 128u64;
                        let a2844 = {
                            let a2840 = {
                                let a2837 = v0;
                                let a2838 = 64u64;
                                let a2839 = 0u64;
                                nat_quot(a2837, a2838, a2839)
                            };
                            let a2841 = 64u64;
                            let a2842 = 0u64;
                            nat_rem(a2840, a2841, a2842)
                        };
                        nat_add(a2843, a2844)?
                    });
                    let a2851 = f498({
                        let a2848 = 128u64;
                        let a2849 = {
                            let a2845 = v0;
                            let a2846 = 64u64;
                            let a2847 = 0u64;
                            nat_rem(a2845, a2846, a2847)
                        };
                        nat_add(a2848, a2849)?
                    });
                    append_bytes(a2850, a2851)
                };
                Ok(append_bytes(a2852, a2853))
            } else {
                let a2884 = f498({
                    let a2857 = 240u64;
                    let a2858 = {
                        let a2854 = v0;
                        let a2855 = 262144u64;
                        let a2856 = 0u64;
                        nat_quot(a2854, a2855, a2856)
                    };
                    nat_add(a2857, a2858)?
                });
                let a2885 = {
                    let a2882 = f498({
                        let a2865 = 128u64;
                        let a2866 = {
                            let a2862 = {
                                let a2859 = v0;
                                let a2860 = 4096u64;
                                let a2861 = 0u64;
                                nat_quot(a2859, a2860, a2861)
                            };
                            let a2863 = 64u64;
                            let a2864 = 0u64;
                            nat_rem(a2862, a2863, a2864)
                        };
                        nat_add(a2865, a2866)?
                    });
                    let a2883 = {
                        let a2880 = f498({
                            let a2873 = 128u64;
                            let a2874 = {
                                let a2870 = {
                                    let a2867 = v0;
                                    let a2868 = 64u64;
                                    let a2869 = 0u64;
                                    nat_quot(a2867, a2868, a2869)
                                };
                                let a2871 = 64u64;
                                let a2872 = 0u64;
                                nat_rem(a2870, a2871, a2872)
                            };
                            nat_add(a2873, a2874)?
                        });
                        let a2881 = f498({
                            let a2878 = 128u64;
                            let a2879 = {
                                let a2875 = v0;
                                let a2876 = 64u64;
                                let a2877 = 0u64;
                                nat_rem(a2875, a2876, a2877)
                            };
                            nat_add(a2878, a2879)?
                        });
                        append_bytes(a2880, a2881)
                    };
                    append_bytes(a2882, a2883)
                };
                Ok(append_bytes(a2884, a2885))
            }
        }
    }
}

pub fn f455(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f499(v2, v0, v1, v3)
}

pub fn f456(p2889: Bytes, p2890: u64) -> Result<Bytes, Bytes> {
    let m2888 = (p2889, p2890);
    {
        let mut m2888 = m2888;
        loop {
            let step = {
                let (v0, v1) = m2888;
                let m2886 = v1;
                if m2886 == 0 {
                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines"))))
                } else {
                    let v2 = m2886 - 1;
                    if f396(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)"))))
                    } else {
                        if f36(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Nat"))))
                        } else {
                            if f36(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Int"))))
                            } else {
                                if f36(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f500()))
                                } else {
                                    if f36(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Bool"))))
                                    } else {
                                        if if f36(v0.clone(), f8(Str::lit("float"))) {
                                            true
                                        } else {
                                            f36(v0.clone(), f8(Str::lit("double")))
                                        } {
                                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil()))))))
                                        } else {
                                            if if f36(v0.clone(), f8(Str::lit("anySimpleType"))) {
                                                true
                                            } else {
                                                f36(v0.clone(), f8(Str::lit("anyAtomicType")))
                                            } {
                                                Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil()))))))
                                            } else {
                                                let m2887 = f501(v0.clone());
                                                match m2887 {
                                                    None => {
                                                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil()))))))
                                                    }
                                                    Some(v3) => {
                                                        if f36(v3.clone(), f8(Str::lit("anyAtomicType"))) {
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
                Ok(next) => { m2888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: Adt19) -> R<Bytes> {
    f257(v0)
}

pub fn f458(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m2891 = v1;
    match m2891 {
        Adt19::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C1(v7, v8) => {
            Ok(Some({
                let a2902 = if if v7 {
                    let a2900 = 0u64;
                    let a2901 = v8;
                    nat_lt(a2900, a2901)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2903 = f353(false, v8)?;
                append_bytes(a2902, a2903)
            }))
        }
        Adt19::C2(v3, v4, v5) => {
            let m2894 = f503({
                let a2892 = f8(Str::lit("n"));
                let a2893 = f354(false, v4, v5)?;
                append_bytes(a2892, a2893)
            })?;
            match m2894 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2898 = f8(Str::lit("nMinus"));
                        let a2899 = f32(v6.clone(), 1u64, {
                            let a2896 = {
                                let a2895 = v6;
                                length_bytes(a2895)
                            };
                            let a2897 = 1u64;
                            nat_sub(a2896, a2897)
                        });
                        append_bytes(a2898, a2899)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C3(v2) => {
            f502(v0, v2)
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

pub fn f459(v0: Bytes) -> R<Option<Bytes>> {
    if f504({
        let a2904 = v0.clone();
        length_bytes(a2904)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f505(v0)?;
        if f506(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2907 = {
                let a2905 = v1.clone();
                let a2906 = 0u64;
                index_bytes(a2905, a2906)
            };
            match m2907 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f417({
                        let v3: u8 = v2;
                        let a2952 = {
                            let a2928 = {
                                let a2916 = {
                                    let a2910 = {
                                        let a2908 = v3;
                                        let a2909 = 128u8;
                                        fixed_u8::bit_and(a2908, a2909)
                                    };
                                    let a2911 = 128u8;
                                    if equal(a2910, a2911) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2917 = {
                                    let a2914 = {
                                        let a2912 = v3;
                                        let a2913 = 64u8;
                                        fixed_u8::bit_and(a2912, a2913)
                                    };
                                    let a2915 = 64u8;
                                    if equal(a2914, a2915) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2916, a2917)?
                            };
                            let a2929 = {
                                let a2926 = {
                                    let a2920 = {
                                        let a2918 = v3;
                                        let a2919 = 32u8;
                                        fixed_u8::bit_and(a2918, a2919)
                                    };
                                    let a2921 = 32u8;
                                    if equal(a2920, a2921) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2927 = {
                                    let a2924 = {
                                        let a2922 = v3;
                                        let a2923 = 16u8;
                                        fixed_u8::bit_and(a2922, a2923)
                                    };
                                    let a2925 = 16u8;
                                    if equal(a2924, a2925) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2926, a2927)?
                            };
                            nat_add(a2928, a2929)?
                        };
                        let a2953 = {
                            let a2950 = {
                                let a2938 = {
                                    let a2932 = {
                                        let a2930 = v3;
                                        let a2931 = 8u8;
                                        fixed_u8::bit_and(a2930, a2931)
                                    };
                                    let a2933 = 8u8;
                                    if equal(a2932, a2933) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2939 = {
                                    let a2936 = {
                                        let a2934 = v3;
                                        let a2935 = 4u8;
                                        fixed_u8::bit_and(a2934, a2935)
                                    };
                                    let a2937 = 4u8;
                                    if equal(a2936, a2937) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2938, a2939)?
                            };
                            let a2951 = {
                                let a2948 = {
                                    let a2942 = {
                                        let a2940 = v3;
                                        let a2941 = 2u8;
                                        fixed_u8::bit_and(a2940, a2941)
                                    };
                                    let a2943 = 2u8;
                                    if equal(a2942, a2943) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2949 = {
                                    let a2946 = {
                                        let a2944 = v3;
                                        let a2945 = 1u8;
                                        fixed_u8::bit_and(a2944, a2945)
                                    };
                                    let a2947 = 1u8;
                                    if equal(a2946, a2947) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2948, a2949)?
                            };
                            nat_add(a2950, a2951)?
                        };
                        nat_add(a2952, a2953)?
                    }) {
                        Ok(Some({
                            let a2954 = f8(Str::lit("n"));
                            let a2955 = v1;
                            append_bytes(a2954, a2955)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f507({
            let a2956 = v0.clone();
            length_bytes(a2956)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f460(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2957 = v1;
    match m2957.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2958 = f502(v0, v2)?;
            match m2958 {
                None => {
                    f460(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f460(v0, v3)?))
                }
            }
        }
    }
}

pub fn f461(v0: Fn15, p2963: u64, p2964: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2962 = (p2963, p2964);
    {
        let mut m2962 = m2962;
        loop {
            let step = {
                let (v1, v2) = m2962;
                let m2959 = v1;
                if m2959 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2959 - 1;
                    let m2961 = {
                        let c2960: Fn15 = v0.clone();
                        c2960.apply(v2.clone())?
                    };
                    match m2961 {
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
                Ok(next) => { m2962 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f508(v0, v1)
}

pub fn f463(v0: Bytes, v1: u64) -> R<u64> {
    let (h2965, _) = f266(Fn3::F509(v0.clone()), {
        let a2967 = {
            let a2966 = v0;
            length_bytes(a2966)
        };
        let a2968 = 1u64;
        nat_add(a2967, a2968)?
    }, v1)?;
    Ok(h2965)
}

pub fn f464(v0: Bytes) -> R<u64> {
    let (h2969, _) = f266(Fn3::F510(v0.clone()), {
        let a2971 = {
            let a2970 = v0;
            length_bytes(a2970)
        };
        let a2972 = 1u64;
        nat_add(a2971, a2972)?
    }, 0u64)?;
    Ok(h2969)
}

pub fn f465(v0: Bytes) -> R<Option<u64>> {
    if f511(v0.clone())? {
        Ok(Some({
            let (_, h2973) = {
                let (h2974, _) = f496(Fn16::F512(v0.clone()), {
                    let a2975 = v0;
                    length_bytes(a2975)
                }, (0u64, 0u64))?;
                h2974
            };
            h2973
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f466(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2977 = {
            let a2976 = v1.clone();
            length_bytes(a2976)
        };
        let a2978 = f513(v1)?;
        nat_sub(a2977, a2978)
    });
    let v5: Bytes = {
        let a2979 = v0;
        let a2980 = v4.clone();
        append_bytes(a2979, a2980)
    };
    let v6: u64 = f464(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2982 = {
            let a2981 = v5;
            length_bytes(a2981)
        };
        let a2983 = v6;
        nat_sub(a2982, a2983)
    });
    let v8: u64 = {
        let a2984 = v4;
        length_bytes(a2984)
    };
    let v9: u64 = {
        let a2985 = v7.clone();
        length_bytes(a2985)
    };
    let a2986 = v9;
    let a2987 = 0u64;
    if nat_eq(a2986, a2987) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a2988 = v3;
            let a2989 = v8;
            nat_lt(a2988, a2989)
        } {
            let v10: u64 = if v2 {
                let a2990 = v3;
                let a2991 = v8;
                nat_add(a2990, a2991)?
            } else {
                let a2992 = v8;
                let a2993 = v3;
                nat_sub(a2992, a2993)
            };
            let a2994 = v10;
            let a2995 = f513(v7.clone())?;
            if nat_le(a2994, a2995) {
                Ok(Some(f32(v7, 0u64, {
                    let a2996 = v9;
                    let a2997 = v10;
                    nat_sub(a2996, a2997)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a2998 = v3;
                let a2999 = v8;
                nat_sub(a2998, a2999)
            };
            let a3002 = {
                let a3000 = v9;
                let a3001 = v11;
                nat_add(a3000, a3001)?
            };
            let a3003 = 19u64;
            if nat_le(a3002, a3003) {
                Ok(Some({
                    let a3004 = v7;
                    let a3005 = f427(v11);
                    append_bytes(a3004, a3005)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f467(v0: bool, v1: u64) -> bool {
    if v0 {
        let a3006 = 0u64;
        let a3007 = v1;
        if nat_lt(a3006, a3007) {
            let a3008 = v1;
            let a3009 = 9223372036854775808u64;
            nat_le(a3008, a3009)
        } else {
            false
        }
    } else {
        let a3010 = v1;
        let a3011 = 9223372036854775807u64;
        nat_le(a3010, a3011)
    }
}

pub fn f468(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3013 = {
            let a3012 = v1.clone();
            length_bytes(a3012)
        };
        let a3014 = f513(v1)?;
        nat_sub(a3013, a3014)
    });
    let v5: Bytes = {
        let a3015 = v0;
        let a3016 = v4.clone();
        append_bytes(a3015, a3016)
    };
    let v6: u64 = f464(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3018 = {
            let a3017 = v5;
            length_bytes(a3017)
        };
        let a3019 = v6;
        nat_sub(a3018, a3019)
    });
    let v8: u64 = {
        let a3020 = v4;
        length_bytes(a3020)
    };
    let v9: u64 = {
        let a3021 = v7.clone();
        length_bytes(a3021)
    };
    let a3022 = v9;
    let a3023 = 0u64;
    if nat_eq(a3022, a3023) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if if v2 {
            true
        } else {
            let a3024 = v3;
            let a3025 = v8;
            nat_lt(a3024, a3025)
        } {
            let v10: u64 = if v2 {
                let a3026 = v3;
                let a3027 = v8;
                nat_add(a3026, a3027)?
            } else {
                let a3028 = v8;
                let a3029 = v3;
                nat_sub(a3028, a3029)
            };
            let v11: u64 = f513(v7.clone())?;
            let a3030 = v10;
            let a3031 = v11;
            if nat_le(a3030, a3031) {
                Ok(Some((f32(v7, 0u64, {
                    let a3032 = v9;
                    let a3033 = v10;
                    nat_sub(a3032, a3033)
                }), 0u64)))
            } else {
                let a3036 = {
                    let a3034 = v10;
                    let a3035 = v11;
                    nat_sub(a3034, a3035)
                };
                let a3037 = 19u64;
                if nat_le(a3036, a3037) {
                    Ok(Some((f32(v7, 0u64, {
                        let a3038 = v9;
                        let a3039 = v11;
                        nat_sub(a3038, a3039)
                    }), {
                        let a3040 = v10;
                        let a3041 = v11;
                        nat_sub(a3040, a3041)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3042 = v3;
                let a3043 = v8;
                nat_sub(a3042, a3043)
            };
            let a3046 = {
                let a3044 = v9;
                let a3045 = v12;
                nat_add(a3044, a3045)?
            };
            let a3047 = 19u64;
            if nat_le(a3046, a3047) {
                Ok(Some(({
                    let a3048 = v7;
                    let a3049 = f427(v12);
                    append_bytes(a3048, a3049)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f469(v0: Adt19) -> List<(Bytes, Adt19)> {
    let m3050 = v0;
    match m3050 {
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
        Adt19::C5(r3051) => {
            let v4 = (*r3051).clone();
            f469(v4)
        }
        Adt19::C6 => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C7(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C8(v1, r3052, r3053) => {
            let v2 = (*r3052).clone();
            let v3 = (*r3053).clone();
            List::cons((v1, v2), f469(v3))
        }
    }
}

pub fn f470(p3058: List<(Bytes, Adt19)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt19)>> {
    let m3057 = p3058;
    {
        let mut m3057 = m3057;
        loop {
            let step = {
                let v0 = m3057;
                let m3054 = v0;
                match m3054.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(None::<(Bytes, Adt19)>))
                    }
                    Some((v3, v4)) => {
                        if f514(f515({
                            let (_, h3055) = v3.clone();
                            h3055
                        }, List::<Bytes>::nil())?, f254({
                            let (h3056, _) = v3.clone();
                            h3056
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3057 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(v0: List<(Bytes, Adt19)>, v1: Bytes) -> List<(Bytes, Adt19)> {
    let m3059 = v0;
    match m3059.uncons() {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h3060, _) = v2.clone();
                h3060
            }, v1.clone()) {
                f471(v3, v1)
            } else {
                List::cons(v2, f471(v3, v1))
            }
        }
    }
}

pub fn f472() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f473() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f474(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f516(v0, f517(v1))
}

pub fn f475(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m3061 = v0;
    match m3061.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f475(v3, false))
            } else {
                f475(v3, true)
            }
        }
    }
}

pub fn f476(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3065 = {
        let a3063 = v0.clone();
        let a3064 = {
            let (h3062, _) = v1.clone();
            h3062
        };
        index_bytes(a3063, a3064)
    };
    match m3065 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3067 = {
                    let (h3066, _) = v1.clone();
                    h3066
                };
                let a3068 = 1u64;
                nat_add(a3067, a3068)?
            }, {
                let a3071 = {
                    let (_, h3069) = v1;
                    h3069
                };
                let a3072 = f518(v0, {
                    let (h3070, _) = v1;
                    h3070
                }, v2)?;
                append_bytes(a3071, a3072)
            })))
        }
    }
}

pub fn f477(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3073 = v0.clone();
    match m3073.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3076 = {
                let (h3074, _) = v2;
                h3074
            };
            let a3077 = {
                let (h3075, _) = v1.clone();
                h3075
            };
            if equal(a3076, a3077) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f478(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f519(f520(Fn17::F521, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3082 = {
        let (h3078, _) = f522(Fn18::F523, {
            let a3080 = {
                let a3079 = v0;
                length_list(a3079)
            };
            let a3081 = 1u64;
            nat_add(a3080, a3081)?
        }, v1)?;
        h3078
    };
    match m3082.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f479(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3087 = {
        let a3084 = f355({
            let (h3083, _) = v0;
            h3083
        })?;
        let a3085 = Bytes::lit(&[0x3a]);
        append_bytes(a3084, a3085)
    };
    let a3088 = {
        let (_, h3086) = v0;
        h3086
    };
    Ok(append_bytes(a3087, a3088))
}

pub fn f480(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f369({
        let (_, h3089) = {
            let (h3090, _) = f524(Fn19::F525, {
                let a3092 = {
                    let a3091 = v0.clone();
                    length_list(a3091)
                };
                let a3093 = 1u64;
                nat_add(a3092, a3093)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h3090
        };
        h3089
    }))
}

pub fn f481(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3094 = v0;
    match m3094.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f87(v2.clone(), v1.clone()) {
                f481(v3, v1)
            } else {
                List::cons(v2, f481(v3, v1))
            }
        }
    }
}

pub fn f482(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f483(v0: List<Adt14>) -> List<Bytes> {
    let m3095 = v0;
    match m3095.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3096 = f162(v1, f8(Str::lit("std")));
            match m3096 {
                None => {
                    f483(v2)
                }
                Some(v3) => {
                    List::cons(f160(v3, f8(Str::lit("authority"))), f483(v2))
                }
            }
        }
    }
}

pub fn f484(v0: List<Adt14>) -> List<Bytes> {
    let m3097 = v0;
    match m3097.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3098 = f163(f162(v1, f8(Str::lit("req"))));
            match m3098 {
                None => {
                    f484(v2)
                }
                Some(v3) => {
                    List::cons(v3, f484(v2))
                }
            }
        }
    }
}

pub fn f485(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f526(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h3099) => {
            h3099
        }
    });
    let v2: u32 = f445(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h3100, _, _) => {
            h3100
        }
    }, f445(f527(match v0.clone() {
        Adt27::C0(_, _, _, _, h3101, _, _, _, _, _) => {
            h3101
        }
    }), f445(f528(match v0.clone() {
        Adt27::C0(_, _, _, _, h3102, _, _, _, _, _) => {
            h3102
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h3103, _, _, _, _) => {
            h3103
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h3104, _, _, _) => {
            h3104
        }
    }), f445(v1, {
        let m3107 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h3106, _) => {
                h3106
            }
        };
        match m3107 {
            Adt26::C0(h3105, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3105
            }
        }
    }))));
    let v3: u32 = f445(f529(match v0.clone() {
        Adt27::C0(h3108, _, _, _, _, _, _, _, _, _) => {
            h3108
        }
    }), f530(match v0.clone() {
        Adt27::C0(h3109, _, _, _, _, _, _, _, _, _) => {
            h3109
        }
    }, match v0.clone() {
        Adt27::C0(_, h3110, _, _, _, _, _, _, _, _) => {
            h3110
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h3111, _, _, _, _, _, _, _) => {
            h3111
        }
    }));
    Ok(Adt27::C0(f445(v2, v3), match v0.clone() {
        Adt27::C0(h3112, _, _, _, _, _, _, _, _, _) => {
            h3112
        }
    }, match v0.clone() {
        Adt27::C0(_, h3113, _, _, _, _, _, _, _, _) => {
            h3113
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h3114, _, _, _, _, _, _, _) => {
            h3114
        }
    }, f445(match v0.clone() {
        Adt27::C0(_, _, _, h3115, _, _, _, _, _, _) => {
            h3115
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h3116, _, _, _, _, _) => {
            h3116
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h3117, _, _, _, _) => {
            h3117
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h3118, _, _, _) => {
            h3118
        }
    }, f531(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h3119, _) => {
            h3119
        }
    }), {
        let a3121 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h3120) => {
                h3120
            }
        };
        let a3122 = 1u64;
        nat_add(a3121, a3122)?
    }))
}

pub fn f486(v0: u32) -> u64 {
    let m3124 = {
        let a3123 = v0;
        fixed_u64::convert(i128::from(a3123))
    };
    match m3124 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f487(v0: u64) -> u32 {
    let m3128 = {
        let a3127 = {
            let a3125 = v0;
            let a3126 = 4294967295u64;
            fixed_u64::bit_and(a3125, a3126)
        };
        fixed_u32::convert(i128::from(a3127))
    };
    match m3128 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f488(v0: u64) -> u32 {
    let (_, h3129) = {
        let (_, h3130) = f532(Fn20::F533, 32u64, (v0, (1u32, 0u32)));
        h3130
    };
    h3129
}

pub fn f489(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3135 = f534(v0, 24u32);
    let a3136 = {
        let a3133 = f534(v1, 16u32);
        let a3134 = {
            let a3131 = f534(v2, 8u32);
            let a3132 = v3;
            fixed_u32::bit_or(a3131, a3132)
        };
        fixed_u32::bit_or(a3133, a3134)
    };
    fixed_u32::bit_or(a3135, a3136)
}

pub fn f490(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3137 = v2;
    let a3138 = v1;
    if nat_lt(a3137, a3138) {
        let m3141 = {
            let a3139 = v0;
            let a3140 = v2;
            index_bytes(a3139, a3140)
        };
        match m3141 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3143 = {
                    let a3142 = v3;
                    fixed_u32::convert(i128::from(a3142))
                };
                match m3143 {
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
        let a3144 = v2;
        let a3145 = v1;
        if nat_eq(a3144, a3145) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f491(v0: u8) -> bool {
    let a3146 = v0;
    let a3147 = 32u8;
    if if if equal(a3146, a3147) {
        true
    } else {
        let a3148 = v0;
        let a3149 = 9u8;
        equal(a3148, a3149)
    } {
        true
    } else {
        let a3150 = v0;
        let a3151 = 10u8;
        equal(a3150, a3151)
    } {
        true
    } else {
        let a3152 = v0;
        let a3153 = 12u8;
        if equal(a3152, a3153) {
            true
        } else {
            let a3154 = v0;
            let a3155 = 13u8;
            equal(a3154, a3155)
        }
    }
}

pub fn f492(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3158 = {
        let a3156 = v0;
        let a3157 = v1;
        index_bytes(a3156, a3157)
    };
    match m3158 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3203 = {
                    let a3179 = {
                        let a3167 = {
                            let a3161 = {
                                let a3159 = v3;
                                let a3160 = 128u8;
                                fixed_u8::bit_and(a3159, a3160)
                            };
                            let a3162 = 128u8;
                            if equal(a3161, a3162) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3168 = {
                            let a3165 = {
                                let a3163 = v3;
                                let a3164 = 64u8;
                                fixed_u8::bit_and(a3163, a3164)
                            };
                            let a3166 = 64u8;
                            if equal(a3165, a3166) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3167, a3168)?
                    };
                    let a3180 = {
                        let a3177 = {
                            let a3171 = {
                                let a3169 = v3;
                                let a3170 = 32u8;
                                fixed_u8::bit_and(a3169, a3170)
                            };
                            let a3172 = 32u8;
                            if equal(a3171, a3172) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3178 = {
                            let a3175 = {
                                let a3173 = v3;
                                let a3174 = 16u8;
                                fixed_u8::bit_and(a3173, a3174)
                            };
                            let a3176 = 16u8;
                            if equal(a3175, a3176) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3177, a3178)?
                    };
                    nat_add(a3179, a3180)?
                };
                let a3204 = {
                    let a3201 = {
                        let a3189 = {
                            let a3183 = {
                                let a3181 = v3;
                                let a3182 = 8u8;
                                fixed_u8::bit_and(a3181, a3182)
                            };
                            let a3184 = 8u8;
                            if equal(a3183, a3184) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3190 = {
                            let a3187 = {
                                let a3185 = v3;
                                let a3186 = 4u8;
                                fixed_u8::bit_and(a3185, a3186)
                            };
                            let a3188 = 4u8;
                            if equal(a3187, a3188) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3189, a3190)?
                    };
                    let a3202 = {
                        let a3199 = {
                            let a3193 = {
                                let a3191 = v3;
                                let a3192 = 2u8;
                                fixed_u8::bit_and(a3191, a3192)
                            };
                            let a3194 = 2u8;
                            if equal(a3193, a3194) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3200 = {
                            let a3197 = {
                                let a3195 = v3;
                                let a3196 = 1u8;
                                fixed_u8::bit_and(a3195, a3196)
                            };
                            let a3198 = 1u8;
                            if equal(a3197, a3198) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3199, a3200)?
                    };
                    nat_add(a3201, a3202)?
                };
                nat_add(a3203, a3204)?
            };
            let a3205 = 128u64;
            let a3206 = v4;
            if if nat_le(a3205, a3206) {
                let a3207 = v4;
                let a3208 = 191u64;
                nat_le(a3207, a3208)
            } else {
                false
            } {
                Ok(Some({
                    let a3209 = v4;
                    let a3210 = 128u64;
                    nat_sub(a3209, a3210)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f493(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3211 = v1;
    let a3212 = v0;
    if if nat_le(a3211, a3212) {
        let a3213 = v0;
        let a3214 = v2;
        if nat_le(a3213, a3214) {
            let a3215 = v0;
            let a3216 = 55296u64;
            if nat_lt(a3215, a3216) {
                true
            } else {
                let a3217 = 57343u64;
                let a3218 = v0;
                nat_lt(a3217, a3218)
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

pub fn f494(v0: u64) -> bool {
    let a3219 = v0;
    let a3220 = 58u64;
    if if if nat_eq(a3219, a3220) {
        true
    } else {
        let a3221 = v0;
        let a3222 = 95u64;
        nat_eq(a3221, a3222)
    } {
        true
    } else {
        let a3223 = 65u64;
        let a3224 = v0;
        if if nat_le(a3223, a3224) {
            let a3225 = v0;
            let a3226 = 90u64;
            nat_le(a3225, a3226)
        } else {
            false
        } {
            true
        } else {
            let a3227 = 97u64;
            let a3228 = v0;
            if nat_le(a3227, a3228) {
                let a3229 = v0;
                let a3230 = 122u64;
                nat_le(a3229, a3230)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a3231 = 192u64;
        let a3232 = v0;
        if if if if nat_le(a3231, a3232) {
            let a3233 = v0;
            let a3234 = 214u64;
            nat_le(a3233, a3234)
        } else {
            false
        } {
            true
        } else {
            let a3235 = 216u64;
            let a3236 = v0;
            if nat_le(a3235, a3236) {
                let a3237 = v0;
                let a3238 = 246u64;
                nat_le(a3237, a3238)
            } else {
                false
            }
        } {
            true
        } else {
            let a3239 = 248u64;
            let a3240 = v0;
            if if nat_le(a3239, a3240) {
                let a3241 = v0;
                let a3242 = 767u64;
                nat_le(a3241, a3242)
            } else {
                false
            } {
                true
            } else {
                let a3243 = 880u64;
                let a3244 = v0;
                if nat_le(a3243, a3244) {
                    let a3245 = v0;
                    let a3246 = 893u64;
                    nat_le(a3245, a3246)
                } else {
                    false
                }
            }
        } {
            true
        } else {
            let a3247 = 895u64;
            let a3248 = v0;
            if if if if nat_le(a3247, a3248) {
                let a3249 = v0;
                let a3250 = 8191u64;
                nat_le(a3249, a3250)
            } else {
                false
            } {
                true
            } else {
                let a3251 = 8204u64;
                let a3252 = v0;
                if nat_le(a3251, a3252) {
                    let a3253 = v0;
                    let a3254 = 8205u64;
                    nat_le(a3253, a3254)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3255 = 8304u64;
                let a3256 = v0;
                if if nat_le(a3255, a3256) {
                    let a3257 = v0;
                    let a3258 = 8591u64;
                    nat_le(a3257, a3258)
                } else {
                    false
                } {
                    true
                } else {
                    let a3259 = 11264u64;
                    let a3260 = v0;
                    if nat_le(a3259, a3260) {
                        let a3261 = v0;
                        let a3262 = 12271u64;
                        nat_le(a3261, a3262)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a3263 = 12289u64;
                let a3264 = v0;
                if if if nat_le(a3263, a3264) {
                    let a3265 = v0;
                    let a3266 = 55295u64;
                    nat_le(a3265, a3266)
                } else {
                    false
                } {
                    true
                } else {
                    let a3267 = 63744u64;
                    let a3268 = v0;
                    if nat_le(a3267, a3268) {
                        let a3269 = v0;
                        let a3270 = 64975u64;
                        nat_le(a3269, a3270)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a3271 = 65008u64;
                    let a3272 = v0;
                    if if nat_le(a3271, a3272) {
                        let a3273 = v0;
                        let a3274 = 65533u64;
                        nat_le(a3273, a3274)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a3275 = 65536u64;
                        let a3276 = v0;
                        if nat_le(a3275, a3276) {
                            let a3277 = v0;
                            let a3278 = 983039u64;
                            nat_le(a3277, a3278)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f495(v0: u64) -> bool {
    if f494(v0) {
        true
    } else {
        let a3279 = v0;
        let a3280 = 45u64;
        if if if nat_eq(a3279, a3280) {
            true
        } else {
            let a3281 = v0;
            let a3282 = 46u64;
            nat_eq(a3281, a3282)
        } {
            true
        } else {
            let a3283 = 48u64;
            let a3284 = v0;
            if nat_le(a3283, a3284) {
                let a3285 = v0;
                let a3286 = 57u64;
                nat_le(a3285, a3286)
            } else {
                false
            }
        } {
            true
        } else {
            let a3287 = v0;
            let a3288 = 183u64;
            if if nat_eq(a3287, a3288) {
                true
            } else {
                let a3289 = 768u64;
                let a3290 = v0;
                if nat_le(a3289, a3290) {
                    let a3291 = v0;
                    let a3292 = 879u64;
                    nat_le(a3291, a3292)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3293 = 8255u64;
                let a3294 = v0;
                if nat_le(a3293, a3294) {
                    let a3295 = v0;
                    let a3296 = 8256u64;
                    nat_le(a3295, a3296)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f496(v0: Fn16, p3301: u64, p3302: (u64, u64)) -> R<((u64, u64), bool)> {
    let m3300 = (p3301, p3302);
    {
        let mut m3300 = m3300;
        loop {
            let step = {
                let (v1, v2) = m3300;
                let m3297 = v1;
                if m3297 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3297 - 1;
                    let m3299 = {
                        let c3298: Fn16 = v0.clone();
                        c3298.apply(v2)?
                    };
                    match m3299 {
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
                Ok(next) => { m3300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f535(v1, v0, v2)
}

pub fn f498(v0: u64) -> Bytes {
    f536(f537({
        let a3303 = v0;
        let a3304 = 16u64;
        let a3305 = 0u64;
        nat_quot(a3303, a3304, a3305)
    }), {
        let a3306 = v0;
        let a3307 = 16u64;
        let a3308 = 0u64;
        nat_rem(a3306, a3307, a3308)
    })
}

pub fn f499(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f538(f229(v0, v3)?, v1, v2) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3309 = v3;
            let a3310 = 1u64;
            nat_add(a3309, a3310)?
        }))
    }
}

pub fn f500() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f501(v0: Bytes) -> Option<Bytes> {
    if f396(v0.clone(), List::cons(f8(Str::lit("string")), List::cons(f8(Str::lit("boolean")), List::cons(f8(Str::lit("decimal")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::cons(f8(Str::lit("duration")), List::cons(f8(Str::lit("dateTime")), List::cons(f8(Str::lit("time")), List::cons(f8(Str::lit("date")), List::cons(f8(Str::lit("gYearMonth")), List::cons(f8(Str::lit("gYear")), List::cons(f8(Str::lit("gMonthDay")), List::cons(f8(Str::lit("gDay")), List::cons(f8(Str::lit("gMonth")), List::cons(f8(Str::lit("hexBinary")), List::cons(f8(Str::lit("base64Binary")), List::cons(f8(Str::lit("anyURI")), List::cons(f8(Str::lit("QName")), List::cons(f8(Str::lit("NOTATION")), List::<Bytes>::nil())))))))))))))))))))) {
        Some(f8(Str::lit("anyAtomicType")))
    } else {
        if f36(v0.clone(), f8(Str::lit("normalizedString"))) {
            Some(f8(Str::lit("string")))
        } else {
            if f36(v0.clone(), f8(Str::lit("token"))) {
                Some(f8(Str::lit("normalizedString")))
            } else {
                if f396(v0.clone(), List::cons(f8(Str::lit("language")), List::cons(f8(Str::lit("NMTOKEN")), List::cons(f8(Str::lit("Name")), List::<Bytes>::nil())))) {
                    Some(f8(Str::lit("token")))
                } else {
                    if f36(v0.clone(), f8(Str::lit("NCName"))) {
                        Some(f8(Str::lit("Name")))
                    } else {
                        if f396(v0.clone(), List::cons(f8(Str::lit("ID")), List::cons(f8(Str::lit("IDREF")), List::cons(f8(Str::lit("ENTITY")), List::<Bytes>::nil())))) {
                            Some(f8(Str::lit("NCName")))
                        } else {
                            if f36(v0.clone(), f8(Str::lit("integer"))) {
                                Some(f8(Str::lit("decimal")))
                            } else {
                                if f396(v0.clone(), List::cons(f8(Str::lit("nonPositiveInteger")), List::cons(f8(Str::lit("long")), List::cons(f8(Str::lit("nonNegativeInteger")), List::<Bytes>::nil())))) {
                                    Some(f8(Str::lit("integer")))
                                } else {
                                    if f36(v0.clone(), f8(Str::lit("negativeInteger"))) {
                                        Some(f8(Str::lit("nonPositiveInteger")))
                                    } else {
                                        if f36(v0.clone(), f8(Str::lit("int"))) {
                                            Some(f8(Str::lit("long")))
                                        } else {
                                            if f36(v0.clone(), f8(Str::lit("short"))) {
                                                Some(f8(Str::lit("int")))
                                            } else {
                                                if f36(v0.clone(), f8(Str::lit("byte"))) {
                                                    Some(f8(Str::lit("short")))
                                                } else {
                                                    if f396(v0.clone(), List::cons(f8(Str::lit("unsignedLong")), List::cons(f8(Str::lit("positiveInteger")), List::<Bytes>::nil()))) {
                                                        Some(f8(Str::lit("nonNegativeInteger")))
                                                    } else {
                                                        if f36(v0.clone(), f8(Str::lit("unsignedInt"))) {
                                                            Some(f8(Str::lit("unsignedLong")))
                                                        } else {
                                                            if f36(v0.clone(), f8(Str::lit("unsignedShort"))) {
                                                                Some(f8(Str::lit("unsignedInt")))
                                                            } else {
                                                                if f36(v0.clone(), f8(Str::lit("unsignedByte"))) {
                                                                    Some(f8(Str::lit("unsignedShort")))
                                                                } else {
                                                                    if f396(v0.clone(), List::cons(f8(Str::lit("yearMonthDuration")), List::cons(f8(Str::lit("dayTimeDuration")), List::<Bytes>::nil()))) {
                                                                        Some(f8(Str::lit("duration")))
                                                                    } else {
                                                                        if f36(v0.clone(), f8(Str::lit("dateTimeStamp"))) {
                                                                            Some(f8(Str::lit("dateTime")))
                                                                        } else {
                                                                            if f396(v0, List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))))) {
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

pub fn f502(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f503(v1)
    } else {
        f459(v1)
    }
}

pub fn f503(v0: Bytes) -> R<Option<Bytes>> {
    let m3312 = f539({
        let a3311 = v0.clone();
        length_bytes(a3311)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3312 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3314 = {
                let a3313 = v1.clone();
                length_bytes(a3313)
            };
            let a3315 = 0u64;
            if nat_eq(a3314, a3315) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f504(p3371: u64, v1: Bytes, p3372: u64) -> R<bool> {
    let m3370 = (p3371, p3372);
    {
        let mut m3370 = m3370;
        loop {
            let step = {
                let (v0, v2) = m3370;
                let m3316 = v0;
                if m3316 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3316 - 1;
                    let m3319 = {
                        let a3317 = v1.clone();
                        let a3318 = v2;
                        index_bytes(a3317, a3318)
                    };
                    match m3319 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3366 = {
                                let v5: u8 = v4;
                                let a3364 = {
                                    let a3340 = {
                                        let a3328 = {
                                            let a3322 = {
                                                let a3320 = v5;
                                                let a3321 = 128u8;
                                                fixed_u8::bit_and(a3320, a3321)
                                            };
                                            let a3323 = 128u8;
                                            if equal(a3322, a3323) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3329 = {
                                            let a3326 = {
                                                let a3324 = v5;
                                                let a3325 = 64u8;
                                                fixed_u8::bit_and(a3324, a3325)
                                            };
                                            let a3327 = 64u8;
                                            if equal(a3326, a3327) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3328, a3329)?
                                    };
                                    let a3341 = {
                                        let a3338 = {
                                            let a3332 = {
                                                let a3330 = v5;
                                                let a3331 = 32u8;
                                                fixed_u8::bit_and(a3330, a3331)
                                            };
                                            let a3333 = 32u8;
                                            if equal(a3332, a3333) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3339 = {
                                            let a3336 = {
                                                let a3334 = v5;
                                                let a3335 = 16u8;
                                                fixed_u8::bit_and(a3334, a3335)
                                            };
                                            let a3337 = 16u8;
                                            if equal(a3336, a3337) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3338, a3339)?
                                    };
                                    nat_add(a3340, a3341)?
                                };
                                let a3365 = {
                                    let a3362 = {
                                        let a3350 = {
                                            let a3344 = {
                                                let a3342 = v5;
                                                let a3343 = 8u8;
                                                fixed_u8::bit_and(a3342, a3343)
                                            };
                                            let a3345 = 8u8;
                                            if equal(a3344, a3345) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3351 = {
                                            let a3348 = {
                                                let a3346 = v5;
                                                let a3347 = 4u8;
                                                fixed_u8::bit_and(a3346, a3347)
                                            };
                                            let a3349 = 4u8;
                                            if equal(a3348, a3349) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3350, a3351)?
                                    };
                                    let a3363 = {
                                        let a3360 = {
                                            let a3354 = {
                                                let a3352 = v5;
                                                let a3353 = 2u8;
                                                fixed_u8::bit_and(a3352, a3353)
                                            };
                                            let a3355 = 2u8;
                                            if equal(a3354, a3355) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3361 = {
                                            let a3358 = {
                                                let a3356 = v5;
                                                let a3357 = 1u8;
                                                fixed_u8::bit_and(a3356, a3357)
                                            };
                                            let a3359 = 1u8;
                                            if equal(a3358, a3359) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3360, a3361)?
                                    };
                                    nat_add(a3362, a3363)?
                                };
                                nat_add(a3364, a3365)?
                            };
                            let a3367 = 128u64;
                            if nat_lt(a3366, a3367) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3368 = v2;
                                    let a3369 = 1u64;
                                    nat_add(a3368, a3369)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3370 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f505(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f349(v0.clone(), 0u64, {
        let a3373 = v0;
        length_bytes(a3373)
    }, false, Bytes::lit(&[]))?;
    let m3376 = {
        let a3374 = v1.clone();
        let a3375 = 0u64;
        index_bytes(a3374, a3375)
    };
    match m3376 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3421 = {
                    let a3397 = {
                        let a3385 = {
                            let a3379 = {
                                let a3377 = v3;
                                let a3378 = 128u8;
                                fixed_u8::bit_and(a3377, a3378)
                            };
                            let a3380 = 128u8;
                            if equal(a3379, a3380) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3386 = {
                            let a3383 = {
                                let a3381 = v3;
                                let a3382 = 64u8;
                                fixed_u8::bit_and(a3381, a3382)
                            };
                            let a3384 = 64u8;
                            if equal(a3383, a3384) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3385, a3386)?
                    };
                    let a3398 = {
                        let a3395 = {
                            let a3389 = {
                                let a3387 = v3;
                                let a3388 = 32u8;
                                fixed_u8::bit_and(a3387, a3388)
                            };
                            let a3390 = 32u8;
                            if equal(a3389, a3390) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3396 = {
                            let a3393 = {
                                let a3391 = v3;
                                let a3392 = 16u8;
                                fixed_u8::bit_and(a3391, a3392)
                            };
                            let a3394 = 16u8;
                            if equal(a3393, a3394) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3395, a3396)?
                    };
                    nat_add(a3397, a3398)?
                };
                let a3422 = {
                    let a3419 = {
                        let a3407 = {
                            let a3401 = {
                                let a3399 = v3;
                                let a3400 = 8u8;
                                fixed_u8::bit_and(a3399, a3400)
                            };
                            let a3402 = 8u8;
                            if equal(a3401, a3402) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3408 = {
                            let a3405 = {
                                let a3403 = v3;
                                let a3404 = 4u8;
                                fixed_u8::bit_and(a3403, a3404)
                            };
                            let a3406 = 4u8;
                            if equal(a3405, a3406) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3407, a3408)?
                    };
                    let a3420 = {
                        let a3417 = {
                            let a3411 = {
                                let a3409 = v3;
                                let a3410 = 2u8;
                                fixed_u8::bit_and(a3409, a3410)
                            };
                            let a3412 = 2u8;
                            if equal(a3411, a3412) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3418 = {
                            let a3415 = {
                                let a3413 = v3;
                                let a3414 = 1u8;
                                fixed_u8::bit_and(a3413, a3414)
                            };
                            let a3416 = 1u8;
                            if equal(a3415, a3416) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3417, a3418)?
                    };
                    nat_add(a3419, a3420)?
                };
                nat_add(a3421, a3422)?
            };
            if f416(v4) {
                let a3429 = f32({
                    let a3423 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3423)
                }, {
                    let a3424 = v4;
                    let a3425 = 65u64;
                    nat_sub(a3424, a3425)
                }, 1u64);
                let a3430 = f32(v1.clone(), 1u64, {
                    let a3427 = {
                        let a3426 = v1;
                        length_bytes(a3426)
                    };
                    let a3428 = 1u64;
                    nat_sub(a3427, a3428)
                });
                Ok(append_bytes(a3429, a3430))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f506(v0: Bytes) -> R<bool> {
    let m3433 = {
        let a3431 = v0;
        let a3432 = 0u64;
        index_bytes(a3431, a3432)
    };
    match m3433 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f415({
                let v2: u8 = v1;
                let a3478 = {
                    let a3454 = {
                        let a3442 = {
                            let a3436 = {
                                let a3434 = v2;
                                let a3435 = 128u8;
                                fixed_u8::bit_and(a3434, a3435)
                            };
                            let a3437 = 128u8;
                            if equal(a3436, a3437) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3443 = {
                            let a3440 = {
                                let a3438 = v2;
                                let a3439 = 64u8;
                                fixed_u8::bit_and(a3438, a3439)
                            };
                            let a3441 = 64u8;
                            if equal(a3440, a3441) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3442, a3443)?
                    };
                    let a3455 = {
                        let a3452 = {
                            let a3446 = {
                                let a3444 = v2;
                                let a3445 = 32u8;
                                fixed_u8::bit_and(a3444, a3445)
                            };
                            let a3447 = 32u8;
                            if equal(a3446, a3447) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3453 = {
                            let a3450 = {
                                let a3448 = v2;
                                let a3449 = 16u8;
                                fixed_u8::bit_and(a3448, a3449)
                            };
                            let a3451 = 16u8;
                            if equal(a3450, a3451) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3452, a3453)?
                    };
                    nat_add(a3454, a3455)?
                };
                let a3479 = {
                    let a3476 = {
                        let a3464 = {
                            let a3458 = {
                                let a3456 = v2;
                                let a3457 = 8u8;
                                fixed_u8::bit_and(a3456, a3457)
                            };
                            let a3459 = 8u8;
                            if equal(a3458, a3459) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3465 = {
                            let a3462 = {
                                let a3460 = v2;
                                let a3461 = 4u8;
                                fixed_u8::bit_and(a3460, a3461)
                            };
                            let a3463 = 4u8;
                            if equal(a3462, a3463) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3464, a3465)?
                    };
                    let a3477 = {
                        let a3474 = {
                            let a3468 = {
                                let a3466 = v2;
                                let a3467 = 2u8;
                                fixed_u8::bit_and(a3466, a3467)
                            };
                            let a3469 = 2u8;
                            if equal(a3468, a3469) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3475 = {
                            let a3472 = {
                                let a3470 = v2;
                                let a3471 = 1u8;
                                fixed_u8::bit_and(a3470, a3471)
                            };
                            let a3473 = 1u8;
                            if equal(a3472, a3473) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3474, a3475)?
                    };
                    nat_add(a3476, a3477)?
                };
                nat_add(a3478, a3479)?
            }) {
                Ok(true)
            } else {
                Ok(f416({
                    let v3: u8 = v1;
                    let a3524 = {
                        let a3500 = {
                            let a3488 = {
                                let a3482 = {
                                    let a3480 = v3;
                                    let a3481 = 128u8;
                                    fixed_u8::bit_and(a3480, a3481)
                                };
                                let a3483 = 128u8;
                                if equal(a3482, a3483) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a3489 = {
                                let a3486 = {
                                    let a3484 = v3;
                                    let a3485 = 64u8;
                                    fixed_u8::bit_and(a3484, a3485)
                                };
                                let a3487 = 64u8;
                                if equal(a3486, a3487) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3488, a3489)?
                        };
                        let a3501 = {
                            let a3498 = {
                                let a3492 = {
                                    let a3490 = v3;
                                    let a3491 = 32u8;
                                    fixed_u8::bit_and(a3490, a3491)
                                };
                                let a3493 = 32u8;
                                if equal(a3492, a3493) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a3499 = {
                                let a3496 = {
                                    let a3494 = v3;
                                    let a3495 = 16u8;
                                    fixed_u8::bit_and(a3494, a3495)
                                };
                                let a3497 = 16u8;
                                if equal(a3496, a3497) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3498, a3499)?
                        };
                        nat_add(a3500, a3501)?
                    };
                    let a3525 = {
                        let a3522 = {
                            let a3510 = {
                                let a3504 = {
                                    let a3502 = v3;
                                    let a3503 = 8u8;
                                    fixed_u8::bit_and(a3502, a3503)
                                };
                                let a3505 = 8u8;
                                if equal(a3504, a3505) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a3511 = {
                                let a3508 = {
                                    let a3506 = v3;
                                    let a3507 = 4u8;
                                    fixed_u8::bit_and(a3506, a3507)
                                };
                                let a3509 = 4u8;
                                if equal(a3508, a3509) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3510, a3511)?
                        };
                        let a3523 = {
                            let a3520 = {
                                let a3514 = {
                                    let a3512 = v3;
                                    let a3513 = 2u8;
                                    fixed_u8::bit_and(a3512, a3513)
                                };
                                let a3515 = 2u8;
                                if equal(a3514, a3515) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a3521 = {
                                let a3518 = {
                                    let a3516 = v3;
                                    let a3517 = 1u8;
                                    fixed_u8::bit_and(a3516, a3517)
                                };
                                let a3519 = 1u8;
                                if equal(a3518, a3519) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3520, a3521)?
                        };
                        nat_add(a3522, a3523)?
                    };
                    nat_add(a3524, a3525)?
                }))
            }
        }
    }
}

pub fn f507(p3538: u64, v1: Bytes, p3539: u64, p3540: Bytes) -> R<Option<Bytes>> {
    let m3537 = (p3538, (p3539, p3540));
    {
        let mut m3537 = m3537;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3537;
                let m3526 = v0;
                if m3526 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3526 - 1;
                    let a3528 = {
                        let a3527 = v1.clone();
                        length_bytes(a3527)
                    };
                    let a3529 = v2;
                    if nat_le(a3528, a3529) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3530 = f448(v1.clone(), v2)?;
                        match m3530 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3532 = v2;
                                    let a3533 = {
                                        let (_, h3531) = v5;
                                        h3531
                                    };
                                    nat_add(a3532, a3533)?
                                }, {
                                    let a3535 = v3;
                                    let a3536 = f540({
                                        let (h3534, _) = v5;
                                        h3534
                                    });
                                    append_bytes(a3535, a3536)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3537 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h3541, _) = v1.clone();
        h3541
    };
    let v3: Bytes = {
        let (h3542, _) = {
            let (_, h3543) = v1.clone();
            h3543
        };
        h3542
    };
    let a3546 = {
        let (_, h3544) = {
            let (_, h3545) = v1;
            h3545
        };
        h3544
    };
    let a3547 = 0u64;
    if nat_eq(a3546, a3547) {
        let v4: u64 = f325(v0.clone(), v2)?;
        let a3548 = v4;
        let a3549 = 34u64;
        if nat_eq(a3548, a3549) {
            Ok(Some(({
                let a3550 = v2;
                let a3551 = 1u64;
                nat_add(a3550, a3551)?
            }, (v3, 1u64))))
        } else {
            let a3552 = v4;
            let a3553 = 92u64;
            if nat_eq(a3552, a3553) {
                f541(v0, v2, v3)
            } else {
                let a3554 = v4;
                let a3555 = 32u64;
                if if nat_lt(a3554, a3555) {
                    true
                } else {
                    let a3556 = v4;
                    let a3557 = 256u64;
                    nat_eq(a3556, a3557)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a3558 = v2;
                        let a3559 = 1u64;
                        nat_add(a3558, a3559)?
                    }, ({
                        let a3560 = v3;
                        let a3561 = f32(v0, v2, 1u64);
                        append_bytes(a3560, a3561)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f509(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f542(v0, v1)
}

pub fn f510(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f543(v0, v1)
}

pub fn f511(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3562 = v0.clone();
        length_bytes(a3562)
    };
    let a3563 = 0u64;
    let a3564 = v1;
    if nat_lt(a3563, a3564) {
        let a3565 = f544(v0.clone())?;
        let a3566 = v1;
        if nat_eq(a3565, a3566) {
            let a3567 = 1u64;
            let a3568 = v1;
            if nat_lt(a3567, a3568) {
                let m3571 = {
                    let a3569 = v0;
                    let a3570 = 0u64;
                    index_bytes(a3569, a3570)
                };
                match m3571 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3572 = v2;
                        let a3573 = 48u8;
                        Ok(!equal(a3572, a3573))
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

pub fn f512(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f545(v0, v1)
}

pub fn f513(v0: Bytes) -> R<u64> {
    let (h3574, _) = f266(Fn3::F546(v0.clone()), {
        let a3576 = {
            let a3575 = v0;
            length_bytes(a3575)
        };
        let a3577 = 1u64;
        nat_add(a3576, a3577)?
    }, 0u64)?;
    Ok(h3574)
}

pub fn f514(p3580: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m3579 = p3580;
    {
        let mut m3579 = m3579;
        loop {
            let step = {
                let v0 = m3579;
                let m3578 = v0;
                match m3578.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f36(v4.clone(), v1.clone()) {
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
                Ok(next) => { m3579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(v0: Adt19, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3581 = v0;
    match m3581 {
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
        Adt19::C4(r3584) => {
            let v2 = (*r3584).clone();
            f515(v2, v1)
        }
        Adt19::C5(r3585) => {
            let v3 = (*r3585).clone();
            f515(v3, v1)
        }
        Adt19::C6 => {
            Ok(v1)
        }
        Adt19::C7(r3586, r3587) => {
            let v4 = (*r3586).clone();
            let v5 = (*r3587).clone();
            f515(v4, f515(v5, v1)?)
        }
        Adt19::C8(v6, r3588, r3589) => {
            let v7 = (*r3588).clone();
            let v8 = (*r3589).clone();
            let v9: List<Bytes> = f515(v7.clone(), f515(v8, v1)?)?;
            if f36(v6, f8(Str::lit("$ref"))) {
                let m3582 = f258(Some(v7));
                match m3582 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m3583 = f547(v10)?;
                        match m3583 {
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

pub fn f516(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3590 = f150(v1.clone(), f8(Str::lit("enum")));
    match m3590 {
        None => {
            let m3592 = f150(v1.clone(), f8(Str::lit("oneOf")));
            match m3592 {
                None => {
                    if f552(v1.clone(), v0.clone())? {
                        let m3593 = f553(v0.clone(), f419(f150(v1.clone(), f8(Str::lit("properties")))), f154(f155(f150(v1, f8(Str::lit("required"))))), true)?;
                        match m3593 {
                            Ok(v7) => {
                                Ok(f550(f408(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f217({
                                    let (h3594, _) = v7;
                                    h3594
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h3595) = v7;
                                    h3595
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt25>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m3596 = f419(f150(v1.clone(), f8(Str::lit("properties"))));
                        match m3596.uncons() {
                            None => {
                                let m3599 = f554(v1, {
                                    let a3597 = v0.clone();
                                    let a3598 = f8(Str::lit("Value"));
                                    append_bytes(a3597, a3598)
                                })?;
                                match m3599 {
                                    Ok(v9) => {
                                        Ok(f550(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h3600, _) = v9;
                                            h3600
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3601) = v9;
                                            h3601
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m3602 = f553(v0.clone(), f419(f150(v1.clone(), f8(Str::lit("properties")))), f154(f155(f150(v1, f8(Str::lit("required"))))), false)?;
                                match m3602 {
                                    Ok(v13) => {
                                        Ok(f550(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m3604 = {
                                                let (h3603, _) = v13.clone();
                                                h3603
                                            };
                                            match m3604.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f217({
                                                        let (h3605, _) = v13;
                                                        h3605
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3606) = v13;
                                            h3606
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
                    f551(v0, f259(v5))
                }
            }
        }
        Some(v2) => {
            let m3591 = f548(f259(v2), List::<Bytes>::nil())?;
            match m3591 {
                Ok(v4) => {
                    Ok(f550(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f217(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f549(v3, v0))
                }
            }
        }
    }
}

pub fn f517(v0: Adt19) -> Adt19 {
    let v2: Adt19 = {
        let m3607 = f150(v0.clone(), f8(Str::lit("anyOf")));
        match m3607 {
            None => {
                v0
            }
            Some(v1) => {
                if f555(f259(v1.clone())) {
                    f556(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f557(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f558(f259(v1.clone()))
                    } {
                        v0
                    } else {
                        f559(f556(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt19 = {
        let m3608 = f150(v2.clone(), f8(Str::lit("oneOf")));
        match m3608 {
            None => {
                v2
            }
            Some(v3) => {
                if f555(f259(v3)) {
                    f556(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m3609 = f150(v4.clone(), f8(Str::lit("allOf")));
    match m3609 {
        None => {
            v4
        }
        Some(v5) => {
            if f560(f259(v5)) {
                f556(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f518(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3654 = {
            let a3630 = {
                let a3618 = {
                    let a3612 = {
                        let a3610 = v3;
                        let a3611 = 128u8;
                        fixed_u8::bit_and(a3610, a3611)
                    };
                    let a3613 = 128u8;
                    if equal(a3612, a3613) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3619 = {
                    let a3616 = {
                        let a3614 = v3;
                        let a3615 = 64u8;
                        fixed_u8::bit_and(a3614, a3615)
                    };
                    let a3617 = 64u8;
                    if equal(a3616, a3617) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3618, a3619)?
            };
            let a3631 = {
                let a3628 = {
                    let a3622 = {
                        let a3620 = v3;
                        let a3621 = 32u8;
                        fixed_u8::bit_and(a3620, a3621)
                    };
                    let a3623 = 32u8;
                    if equal(a3622, a3623) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3629 = {
                    let a3626 = {
                        let a3624 = v3;
                        let a3625 = 16u8;
                        fixed_u8::bit_and(a3624, a3625)
                    };
                    let a3627 = 16u8;
                    if equal(a3626, a3627) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3628, a3629)?
            };
            nat_add(a3630, a3631)?
        };
        let a3655 = {
            let a3652 = {
                let a3640 = {
                    let a3634 = {
                        let a3632 = v3;
                        let a3633 = 8u8;
                        fixed_u8::bit_and(a3632, a3633)
                    };
                    let a3635 = 8u8;
                    if equal(a3634, a3635) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3641 = {
                    let a3638 = {
                        let a3636 = v3;
                        let a3637 = 4u8;
                        fixed_u8::bit_and(a3636, a3637)
                    };
                    let a3639 = 4u8;
                    if equal(a3638, a3639) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3640, a3641)?
            };
            let a3653 = {
                let a3650 = {
                    let a3644 = {
                        let a3642 = v3;
                        let a3643 = 2u8;
                        fixed_u8::bit_and(a3642, a3643)
                    };
                    let a3645 = 2u8;
                    if equal(a3644, a3645) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3651 = {
                    let a3648 = {
                        let a3646 = v3;
                        let a3647 = 1u8;
                        fixed_u8::bit_and(a3646, a3647)
                    };
                    let a3649 = 1u8;
                    if equal(a3648, a3649) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3650, a3651)?
            };
            nat_add(a3652, a3653)?
        };
        nat_add(a3654, a3655)?
    };
    let a3656 = v4;
    let a3657 = 34u64;
    if nat_eq(a3656, a3657) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3658 = v4;
        let a3659 = 92u64;
        if nat_eq(a3658, a3659) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3660 = v4;
            let a3661 = 8u64;
            if nat_eq(a3660, a3661) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3662 = v4;
                let a3663 = 12u64;
                if nat_eq(a3662, a3663) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3664 = v4;
                    let a3665 = 10u64;
                    if nat_eq(a3664, a3665) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3666 = v4;
                        let a3667 = 13u64;
                        if nat_eq(a3666, a3667) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3668 = v4;
                            let a3669 = 9u64;
                            if nat_eq(a3668, a3669) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3670 = v4;
                                let a3671 = 32u64;
                                if nat_lt(a3670, a3671) {
                                    let a3672 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3673 = f561(v2)?;
                                    Ok(append_bytes(a3672, a3673))
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

pub fn f519(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f562(Fn21::F563, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f520(v0: Fn17, p3677: List<List<(Bytes, Bytes)>>, p3678: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3676 = (p3677, p3678);
    {
        let mut m3676 = m3676;
        loop {
            let step = {
                let (v1, v2) = m3676;
                let m3674 = v2;
                match m3674.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3675: Fn17 = v0.clone();
                            c3675.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3676 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f522(v0: Fn18, p3683: u64, p3684: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3682 = (p3683, p3684);
    {
        let mut m3682 = m3682;
        loop {
            let step = {
                let (v1, v2) = m3682;
                let m3679 = v1;
                if m3679 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3679 - 1;
                    let m3681 = {
                        let c3680: Fn18 = v0.clone();
                        c3680.apply(v2.clone())?
                    };
                    match m3681 {
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
                Ok(next) => { m3682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f564(v0)
}

pub fn f524(v0: Fn19, p3689: u64, p3690: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m3688 = (p3689, p3690);
    {
        let mut m3688 = m3688;
        loop {
            let step = {
                let (v1, v2) = m3688;
                let m3685 = v1;
                if m3685 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3685 - 1;
                    let m3687 = {
                        let c3686: Fn19 = v0.clone();
                        c3686.apply(v2.clone())?
                    };
                    match m3687 {
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
                Ok(next) => { m3688 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f565(v0)
}

pub fn f526(v0: u64) -> u32 {
    let a3691 = v0;
    let a3692 = 32u64;
    if nat_lt(a3691, a3692) {
        let a3693 = v0;
        let a3694 = 16u64;
        if nat_lt(a3693, a3694) {
            let a3695 = v0;
            let a3696 = 8u64;
            if nat_lt(a3695, a3696) {
                let a3697 = v0;
                let a3698 = 4u64;
                if nat_lt(a3697, a3698) {
                    let a3699 = v0;
                    let a3700 = 2u64;
                    if nat_lt(a3699, a3700) {
                        let a3701 = v0;
                        let a3702 = 1u64;
                        if nat_lt(a3701, a3702) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3703 = v0;
                        let a3704 = 3u64;
                        if nat_lt(a3703, a3704) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3705 = v0;
                    let a3706 = 6u64;
                    if nat_lt(a3705, a3706) {
                        let a3707 = v0;
                        let a3708 = 5u64;
                        if nat_lt(a3707, a3708) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3709 = v0;
                        let a3710 = 7u64;
                        if nat_lt(a3709, a3710) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3711 = v0;
                let a3712 = 12u64;
                if nat_lt(a3711, a3712) {
                    let a3713 = v0;
                    let a3714 = 10u64;
                    if nat_lt(a3713, a3714) {
                        let a3715 = v0;
                        let a3716 = 9u64;
                        if nat_lt(a3715, a3716) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3717 = v0;
                        let a3718 = 11u64;
                        if nat_lt(a3717, a3718) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3719 = v0;
                    let a3720 = 14u64;
                    if nat_lt(a3719, a3720) {
                        let a3721 = v0;
                        let a3722 = 13u64;
                        if nat_lt(a3721, a3722) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3723 = v0;
                        let a3724 = 15u64;
                        if nat_lt(a3723, a3724) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3725 = v0;
            let a3726 = 24u64;
            if nat_lt(a3725, a3726) {
                let a3727 = v0;
                let a3728 = 20u64;
                if nat_lt(a3727, a3728) {
                    let a3729 = v0;
                    let a3730 = 18u64;
                    if nat_lt(a3729, a3730) {
                        let a3731 = v0;
                        let a3732 = 17u64;
                        if nat_lt(a3731, a3732) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3733 = v0;
                        let a3734 = 19u64;
                        if nat_lt(a3733, a3734) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3735 = v0;
                    let a3736 = 22u64;
                    if nat_lt(a3735, a3736) {
                        let a3737 = v0;
                        let a3738 = 21u64;
                        if nat_lt(a3737, a3738) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3739 = v0;
                        let a3740 = 23u64;
                        if nat_lt(a3739, a3740) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3741 = v0;
                let a3742 = 28u64;
                if nat_lt(a3741, a3742) {
                    let a3743 = v0;
                    let a3744 = 26u64;
                    if nat_lt(a3743, a3744) {
                        let a3745 = v0;
                        let a3746 = 25u64;
                        if nat_lt(a3745, a3746) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3747 = v0;
                        let a3748 = 27u64;
                        if nat_lt(a3747, a3748) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3749 = v0;
                    let a3750 = 30u64;
                    if nat_lt(a3749, a3750) {
                        let a3751 = v0;
                        let a3752 = 29u64;
                        if nat_lt(a3751, a3752) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3753 = v0;
                        let a3754 = 31u64;
                        if nat_lt(a3753, a3754) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3755 = v0;
        let a3756 = 48u64;
        if nat_lt(a3755, a3756) {
            let a3757 = v0;
            let a3758 = 40u64;
            if nat_lt(a3757, a3758) {
                let a3759 = v0;
                let a3760 = 36u64;
                if nat_lt(a3759, a3760) {
                    let a3761 = v0;
                    let a3762 = 34u64;
                    if nat_lt(a3761, a3762) {
                        let a3763 = v0;
                        let a3764 = 33u64;
                        if nat_lt(a3763, a3764) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3765 = v0;
                        let a3766 = 35u64;
                        if nat_lt(a3765, a3766) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3767 = v0;
                    let a3768 = 38u64;
                    if nat_lt(a3767, a3768) {
                        let a3769 = v0;
                        let a3770 = 37u64;
                        if nat_lt(a3769, a3770) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3771 = v0;
                        let a3772 = 39u64;
                        if nat_lt(a3771, a3772) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3773 = v0;
                let a3774 = 44u64;
                if nat_lt(a3773, a3774) {
                    let a3775 = v0;
                    let a3776 = 42u64;
                    if nat_lt(a3775, a3776) {
                        let a3777 = v0;
                        let a3778 = 41u64;
                        if nat_lt(a3777, a3778) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3779 = v0;
                        let a3780 = 43u64;
                        if nat_lt(a3779, a3780) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3781 = v0;
                    let a3782 = 46u64;
                    if nat_lt(a3781, a3782) {
                        let a3783 = v0;
                        let a3784 = 45u64;
                        if nat_lt(a3783, a3784) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3785 = v0;
                        let a3786 = 47u64;
                        if nat_lt(a3785, a3786) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3787 = v0;
            let a3788 = 56u64;
            if nat_lt(a3787, a3788) {
                let a3789 = v0;
                let a3790 = 52u64;
                if nat_lt(a3789, a3790) {
                    let a3791 = v0;
                    let a3792 = 50u64;
                    if nat_lt(a3791, a3792) {
                        let a3793 = v0;
                        let a3794 = 49u64;
                        if nat_lt(a3793, a3794) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3795 = v0;
                        let a3796 = 51u64;
                        if nat_lt(a3795, a3796) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3797 = v0;
                    let a3798 = 54u64;
                    if nat_lt(a3797, a3798) {
                        let a3799 = v0;
                        let a3800 = 53u64;
                        if nat_lt(a3799, a3800) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3801 = v0;
                        let a3802 = 55u64;
                        if nat_lt(a3801, a3802) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3803 = v0;
                let a3804 = 60u64;
                if nat_lt(a3803, a3804) {
                    let a3805 = v0;
                    let a3806 = 58u64;
                    if nat_lt(a3805, a3806) {
                        let a3807 = v0;
                        let a3808 = 57u64;
                        if nat_lt(a3807, a3808) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3809 = v0;
                        let a3810 = 59u64;
                        if nat_lt(a3809, a3810) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3811 = v0;
                    let a3812 = 62u64;
                    if nat_lt(a3811, a3812) {
                        let a3813 = v0;
                        let a3814 = 61u64;
                        if nat_lt(a3813, a3814) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3815 = v0;
                        let a3816 = 63u64;
                        if nat_lt(a3815, a3816) {
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

pub fn f527(v0: u32) -> u32 {
    let a3819 = {
        let a3817 = f566(v0, 6u32, 26u32);
        let a3818 = f566(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3817, a3818)
    };
    let a3820 = f566(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3819, a3820)
}

pub fn f528(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3826 = {
        let a3821 = v0;
        let a3822 = v1;
        fixed_u32::bit_and(a3821, a3822)
    };
    let a3827 = {
        let a3824 = {
            let a3823 = v0;
            fixed_u32::bit_not(a3823)
        };
        let a3825 = v2;
        fixed_u32::bit_and(a3824, a3825)
    };
    fixed_u32::bit_xor(a3826, a3827)
}

pub fn f529(v0: u32) -> u32 {
    let a3830 = {
        let a3828 = f566(v0, 2u32, 30u32);
        let a3829 = f566(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3828, a3829)
    };
    let a3831 = f566(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3830, a3831)
}

pub fn f530(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3840 = {
        let a3836 = {
            let a3832 = v0;
            let a3833 = v1;
            fixed_u32::bit_and(a3832, a3833)
        };
        let a3837 = {
            let a3834 = v0;
            let a3835 = v2;
            fixed_u32::bit_and(a3834, a3835)
        };
        fixed_u32::bit_xor(a3836, a3837)
    };
    let a3841 = {
        let a3838 = v1;
        let a3839 = v2;
        fixed_u32::bit_and(a3838, a3839)
    };
    fixed_u32::bit_xor(a3840, a3841)
}

pub fn f531(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3842, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3842
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3843, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3843
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3844, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3844
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3845, _, _, _, _, _, _, _, _, _, _, _) => {
            h3845
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3846, _, _, _, _, _, _, _, _, _, _) => {
            h3846
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3847, _, _, _, _, _, _, _, _, _) => {
            h3847
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3848, _, _, _, _, _, _, _, _) => {
            h3848
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3849, _, _, _, _, _, _, _) => {
            h3849
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3850, _, _, _, _, _, _) => {
            h3850
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3851, _, _, _, _, _) => {
            h3851
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3852, _, _, _, _) => {
            h3852
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3853, _, _, _) => {
            h3853
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3854, _, _) => {
            h3854
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3855, _) => {
            h3855
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3856) => {
            h3856
        }
    }, f445(f567(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3857, _) => {
            h3857
        }
    }), f445(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3858, _, _, _, _, _, _) => {
            h3858
        }
    }, f445(f568(match v0.clone() {
        Adt26::C0(_, h3859, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3859
        }
    }), match v0 {
        Adt26::C0(h3860, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3860
        }
    }))))
}

pub fn f532(v0: Fn20, p3864: u64, p3865: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3863 = (p3864, p3865);
    {
        let mut m3863 = m3863;
        loop {
            let step = {
                let (v1, v2) = m3863;
                let m3861 = v1;
                if m3861 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3861 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3862: Fn20 = v0.clone();
                        c3862.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f533(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3866, _) = v0;
        h3866
    };
    let v2: u32 = {
        let (h3867, _) = {
            let (_, h3868) = v0;
            h3868
        };
        h3867
    };
    let v3: u32 = {
        let (_, h3869) = {
            let (_, h3870) = v0;
            h3870
        };
        h3869
    };
    ({
        let a3871 = v1;
        let a3872 = 2u64;
        let a3873 = 0u64;
        nat_quot(a3871, a3872, a3873)
    }, (f534(v2, 1u32), {
        let a3877 = {
            let a3874 = v1;
            let a3875 = 2u64;
            let a3876 = 0u64;
            nat_rem(a3874, a3875, a3876)
        };
        let a3878 = 1u64;
        if nat_eq(a3877, a3878) {
            let a3879 = v3;
            let a3880 = v2;
            fixed_u32::bit_or(a3879, a3880)
        } else {
            v3
        }
    }))
}

pub fn f534(v0: u32, v1: u32) -> u32 {
    let m3883 = {
        let a3881 = v0;
        let a3882 = v1;
        fixed_u32::shift_left(a3881, a3882)
    };
    match m3883 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f535(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f229(v0, {
        let (h3884, _) = v2;
        h3884
    })?;
    let a3886 = 1114111u64;
    let a3887 = {
        let (_, h3885) = v2;
        h3885
    };
    if nat_lt(a3886, a3887) {
        Ok(None::<(u64, u64)>)
    } else {
        let a3888 = 48u64;
        let a3889 = v3;
        if if nat_le(a3888, a3889) {
            let a3890 = v3;
            let a3891 = 57u64;
            nat_le(a3890, a3891)
        } else {
            false
        } {
            Ok(Some(({
                let a3893 = {
                    let (h3892, _) = v2;
                    h3892
                };
                let a3894 = 1u64;
                nat_add(a3893, a3894)?
            }, {
                let a3900 = {
                    let a3896 = {
                        let (_, h3895) = v2;
                        h3895
                    };
                    let a3897 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3896, a3897)?
                };
                let a3901 = {
                    let a3898 = v3;
                    let a3899 = 48u64;
                    nat_sub(a3898, a3899)
                };
                nat_add(a3900, a3901)?
            })))
        } else {
            if if v1 {
                let a3902 = 97u64;
                let a3903 = v3;
                if nat_le(a3902, a3903) {
                    let a3904 = v3;
                    let a3905 = 102u64;
                    nat_le(a3904, a3905)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some(({
                    let a3907 = {
                        let (h3906, _) = v2;
                        h3906
                    };
                    let a3908 = 1u64;
                    nat_add(a3907, a3908)?
                }, {
                    let a3914 = {
                        let a3910 = {
                            let (_, h3909) = v2;
                            h3909
                        };
                        let a3911 = 16u64;
                        nat_mul(a3910, a3911)?
                    };
                    let a3915 = {
                        let a3912 = v3;
                        let a3913 = 87u64;
                        nat_sub(a3912, a3913)
                    };
                    nat_add(a3914, a3915)?
                })))
            } else {
                if if v1 {
                    let a3916 = 65u64;
                    let a3917 = v3;
                    if nat_le(a3916, a3917) {
                        let a3918 = v3;
                        let a3919 = 70u64;
                        nat_le(a3918, a3919)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(({
                        let a3921 = {
                            let (h3920, _) = v2;
                            h3920
                        };
                        let a3922 = 1u64;
                        nat_add(a3921, a3922)?
                    }, {
                        let a3928 = {
                            let a3924 = {
                                let (_, h3923) = v2;
                                h3923
                            };
                            let a3925 = 16u64;
                            nat_mul(a3924, a3925)?
                        };
                        let a3929 = {
                            let a3926 = v3;
                            let a3927 = 55u64;
                            nat_sub(a3926, a3927)
                        };
                        nat_add(a3928, a3929)?
                    })))
                } else {
                    Ok(None::<(u64, u64)>)
                }
            }
        }
    }
}

pub fn f536(p3936: List<Bytes>, p3937: u64) -> Bytes {
    let m3935 = (p3936, p3937);
    {
        let mut m3935 = m3935;
        loop {
            let step = {
                let (v0, v1) = m3935;
                let m3930 = v0;
                match m3930.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a3931 = v1;
                        let a3932 = 0u64;
                        if nat_eq(a3931, a3932) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a3933 = v1;
                                let a3934 = 1u64;
                                nat_sub(a3933, a3934)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3935 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f537(v0: u64) -> List<Bytes> {
    let a3938 = v0;
    let a3939 = 0u64;
    if nat_eq(a3938, a3939) {
        f569()
    } else {
        let a3940 = v0;
        let a3941 = 1u64;
        if nat_eq(a3940, a3941) {
            f570()
        } else {
            let a3942 = v0;
            let a3943 = 2u64;
            if nat_eq(a3942, a3943) {
                f571()
            } else {
                let a3944 = v0;
                let a3945 = 3u64;
                if nat_eq(a3944, a3945) {
                    f572()
                } else {
                    let a3946 = v0;
                    let a3947 = 4u64;
                    if nat_eq(a3946, a3947) {
                        f573()
                    } else {
                        let a3948 = v0;
                        let a3949 = 5u64;
                        if nat_eq(a3948, a3949) {
                            f574()
                        } else {
                            let a3950 = v0;
                            let a3951 = 6u64;
                            if nat_eq(a3950, a3951) {
                                f575()
                            } else {
                                let a3952 = v0;
                                let a3953 = 7u64;
                                if nat_eq(a3952, a3953) {
                                    f576()
                                } else {
                                    let a3954 = v0;
                                    let a3955 = 8u64;
                                    if nat_eq(a3954, a3955) {
                                        f577()
                                    } else {
                                        let a3956 = v0;
                                        let a3957 = 9u64;
                                        if nat_eq(a3956, a3957) {
                                            f578()
                                        } else {
                                            let a3958 = v0;
                                            let a3959 = 10u64;
                                            if nat_eq(a3958, a3959) {
                                                f579()
                                            } else {
                                                let a3960 = v0;
                                                let a3961 = 11u64;
                                                if nat_eq(a3960, a3961) {
                                                    f580()
                                                } else {
                                                    let a3962 = v0;
                                                    let a3963 = 12u64;
                                                    if nat_eq(a3962, a3963) {
                                                        f581()
                                                    } else {
                                                        let a3964 = v0;
                                                        let a3965 = 13u64;
                                                        if nat_eq(a3964, a3965) {
                                                            f582()
                                                        } else {
                                                            let a3966 = v0;
                                                            let a3967 = 14u64;
                                                            if nat_eq(a3966, a3967) {
                                                                f583()
                                                            } else {
                                                                f584()
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

pub fn f538(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        let a3968 = v0;
        let a3969 = v2;
        if if if nat_eq(a3968, a3969) {
            true
        } else {
            let a3970 = v0;
            let a3971 = 60u64;
            nat_eq(a3970, a3971)
        } {
            true
        } else {
            let a3972 = v0;
            let a3973 = 38u64;
            if nat_eq(a3972, a3973) {
                true
            } else {
                let a3974 = v0;
                let a3975 = 256u64;
                nat_eq(a3974, a3975)
            }
        } {
            true
        } else {
            let a3976 = v0;
            let a3977 = 9u64;
            if nat_eq(a3976, a3977) {
                true
            } else {
                let a3978 = v0;
                let a3979 = 10u64;
                nat_eq(a3978, a3979)
            }
        }
    } else {
        let a3980 = v0;
        let a3981 = 60u64;
        if if nat_eq(a3980, a3981) {
            true
        } else {
            let a3982 = v0;
            let a3983 = 38u64;
            nat_eq(a3982, a3983)
        } {
            true
        } else {
            let a3984 = v0;
            let a3985 = 93u64;
            if nat_eq(a3984, a3985) {
                true
            } else {
                let a3986 = v0;
                let a3987 = 256u64;
                nat_eq(a3986, a3987)
            }
        }
    }
}

pub fn f539(p4000: u64, v1: Bytes, p4001: u64, p4002: Bytes) -> R<Option<Bytes>> {
    let m3999 = (p4000, (p4001, p4002));
    {
        let mut m3999 = m3999;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3999;
                let m3988 = v0;
                if m3988 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3988 - 1;
                    let a3990 = {
                        let a3989 = v1.clone();
                        length_bytes(a3989)
                    };
                    let a3991 = v2;
                    if nat_le(a3990, a3991) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3992 = f448(v1.clone(), v2)?;
                        match m3992 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3994 = v2;
                                    let a3995 = {
                                        let (_, h3993) = v5;
                                        h3993
                                    };
                                    nat_add(a3994, a3995)?
                                }, {
                                    let a3997 = v3;
                                    let a3998 = f585(v1.clone(), v2, {
                                        let (h3996, _) = v5;
                                        h3996
                                    });
                                    append_bytes(a3997, a3998)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3999 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540(v0: u64) -> Bytes {
    let a4003 = f8(Str::lit("u"));
    let a4004 = f586(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4003, a4004)
}

pub fn f541(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f325(v0.clone(), {
        let a4005 = v1;
        let a4006 = 1u64;
        nat_add(a4005, a4006)?
    })?;
    let v4: Bytes = {
        let a4007 = v3;
        let a4008 = 117u64;
        if nat_eq(a4007, a4008) {
            f587(v0, v1)?
        } else {
            f588(v3)
        }
    };
    let a4009 = v4.clone();
    let a4010 = Bytes::lit(&[]);
    if equal(a4009, a4010) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a4011 = v3;
            let a4012 = 117u64;
            if nat_eq(a4011, a4012) {
                let a4013 = v1;
                let a4014 = 6u64;
                nat_add(a4013, a4014)?
            } else {
                let a4015 = v1;
                let a4016 = 2u64;
                nat_add(a4015, a4016)?
            }
        }, ({
            let a4017 = v2;
            let a4018 = v4;
            append_bytes(a4017, a4018)
        }, 0u64))))
    }
}

pub fn f542(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f325(v0, v1)?;
    let a4019 = 47u64;
    let a4020 = v2;
    if if nat_lt(a4019, a4020) {
        let a4021 = v2;
        let a4022 = 58u64;
        nat_lt(a4021, a4022)
    } else {
        false
    } {
        Ok(Some({
            let a4023 = v1;
            let a4024 = 1u64;
            nat_add(a4023, a4024)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f543(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a4025 = f325(v0, v1)?;
    let a4026 = 48u64;
    if nat_eq(a4025, a4026) {
        Ok(Some({
            let a4027 = v1;
            let a4028 = 1u64;
            nat_add(a4027, a4028)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f544(v0: Bytes) -> R<u64> {
    let (h4029, _) = f266(Fn3::F589(v0.clone()), {
        let a4030 = v0;
        length_bytes(a4030)
    }, 0u64)?;
    Ok(h4029)
}

pub fn f545(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4034 = {
        let a4032 = v0;
        let a4033 = {
            let (h4031, _) = v1;
            h4031
        };
        index_bytes(a4032, a4033)
    };
    match m4034 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a4036 = {
                    let (h4035, _) = v1;
                    h4035
                };
                let a4037 = 1u64;
                nat_add(a4036, a4037)?
            }, {
                let a4041 = {
                    let a4039 = {
                        let (_, h4038) = v1;
                        h4038
                    };
                    let a4040 = 10u64;
                    nat_mul(a4039, a4040)?
                };
                let a4042 = f590(v2)?;
                nat_add(a4041, a4042)?
            })))
        }
    }
}

pub fn f546(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f591(v0, v1)
}

pub fn f547(v0: Bytes) -> R<Option<Bytes>> {
    if f207(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f254(f32(v0.clone(), 8u64, {
            let a4044 = {
                let a4043 = v0;
                length_bytes(a4043)
            };
            let a4045 = 8u64;
            nat_sub(a4044, a4045)
        }))?))
    } else {
        if f207(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f254(f32(v0.clone(), 14u64, {
                let a4047 = {
                    let a4046 = v0;
                    length_bytes(a4046)
                };
                let a4048 = 14u64;
                nat_sub(a4047, a4048)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f548(v0: List<Adt19>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f406(f407(f154(v0.clone()))?, v0, v1)
}

pub fn f549(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt25>), Bytes> {
    Err::<(Bytes, List<Adt25>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f550(v0: Bytes, v1: List<Adt25>) -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((v0, v1))
}

pub fn f551(v0: Bytes, v1: List<Adt19>) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4049 = v1.clone();
    match m4049.uncons() {
        None => {
            Ok(f549(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m4050 = f592(f419(f150(v2, f8(Str::lit("properties")))), v1.clone());
            match m4050.uncons() {
                None => {
                    let m4051 = f593(f407(f594(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m4051 {
                        Ok(v5) => {
                            Ok(f550(f408(v0, {
                                let (h4052, _) = v5;
                                h4052
                            }), {
                                let (_, h4053) = v5;
                                h4053
                            }))
                        }
                        Err(v4) => {
                            Ok(f549(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m4054 = f595(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m4054 {
                        Ok(v9) => {
                            Ok(f550(f408(v0, {
                                let (h4055, _) = v9;
                                h4055
                            }), {
                                let (_, h4056) = v9;
                                h4056
                            }))
                        }
                        Err(v8) => {
                            Ok(f549(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f552(v0: Adt19, v1: Bytes) -> R<bool> {
    let m4057 = v0;
    match m4057 {
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
        Adt19::C4(r4058) => {
            let v2 = (*r4058).clone();
            f552(v2, v1)
        }
        Adt19::C5(r4059) => {
            let v3 = (*r4059).clone();
            f552(v3, v1)
        }
        Adt19::C6 => {
            Ok(false)
        }
        Adt19::C7(r4060, r4061) => {
            let v4 = (*r4060).clone();
            let v5 = (*r4061).clone();
            if f552(v4, v1.clone())? {
                Ok(true)
            } else {
                f552(v5, v1)
            }
        }
        Adt19::C8(v6, r4062, r4063) => {
            let v7 = (*r4062).clone();
            let v8 = (*r4063).clone();
            if if f36(v6, f8(Str::lit("$ref"))) {
                f596(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f552(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f552(v8, v1)
                }
            }
        }
    }
}

pub fn f553(v0: Bytes, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f597(f407(f598(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f554(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4064 = f599(v0.clone());
    match m4064 {
        None => {
            if f36(f153(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4065 = f150(v0, f8(Str::lit("items")));
                match m4065 {
                    None => {
                        Ok(f600(f8(Str::lit("List")), f602()))
                    }
                    Some(v3) => {
                        Ok(f600(f8(Str::lit("List")), f601(v3, {
                            let a4066 = v1;
                            let a4067 = f8(Str::lit("Item"));
                            append_bytes(a4066, a4067)
                        })?))
                    }
                }
            } else {
                f601(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f600(f8(Str::lit("Option")), f601(v2, v1)?))
        }
    }
}

pub fn f555(p4070: List<Adt19>) -> bool {
    let m4069 = p4070;
    {
        let mut m4069 = m4069;
        loop {
            let step = {
                let v0 = m4069;
                let m4068 = v0;
                match m4068.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f603(f469(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt19>, bool>(v2)
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(v0: Adt19, v1: Bytes) -> Adt19 {
    let m4071 = v0;
    match m4071 {
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
        Adt19::C4(r4072) => {
            let v13 = (*r4072).clone();
            Adt19::C4(std::rc::Rc::new(v13))
        }
        Adt19::C5(r4073) => {
            let v5 = (*r4073).clone();
            Adt19::C5(std::rc::Rc::new(f556(v5, v1)))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r4074, r4075) => {
            let v14 = (*r4074).clone();
            let v15 = (*r4075).clone();
            Adt19::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt19::C8(v2, r4076, r4077) => {
            let v3 = (*r4076).clone();
            let v4 = (*r4077).clone();
            if f36(v2.clone(), v1.clone()) {
                f556(v4, v1)
            } else {
                Adt19::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f556(v4, v1)))
            }
        }
    }
}

pub fn f557(v0: Adt19, v1: Bytes) -> bool {
    let m4078 = f150(v0, v1);
    match m4078 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f558(v0: List<Adt19>) -> bool {
    let m4079 = v0.clone();
    match m4079.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m4080 = f592(f419(f150(v1, f8(Str::lit("properties")))), v0.clone());
            match m4080.uncons() {
                None => {
                    f605(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f604(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f559(v0: Adt19, v1: Bytes, v2: Adt19) -> Adt19 {
    let m4081 = v0;
    match m4081 {
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
        Adt19::C4(r4082) => {
            let v14 = (*r4082).clone();
            Adt19::C4(std::rc::Rc::new(v14))
        }
        Adt19::C5(r4083) => {
            let v3 = (*r4083).clone();
            Adt19::C5(std::rc::Rc::new(Adt19::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r4084, r4085) => {
            let v15 = (*r4084).clone();
            let v16 = (*r4085).clone();
            Adt19::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt19::C8(v4, r4086, r4087) => {
            let v5 = (*r4086).clone();
            let v6 = (*r4087).clone();
            Adt19::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f560(p4091: List<Adt19>) -> bool {
    let m4090 = p4091;
    {
        let mut m4090 = m4090;
        loop {
            let step = {
                let v0 = m4090;
                let m4088 = v0;
                match m4088.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f603(f469(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f555(f259({
                                let m4089 = f150(v1, f8(Str::lit("anyOf")));
                                match m4089 {
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
                Ok(next) => { m4090 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4136 = {
            let a4112 = {
                let a4100 = {
                    let a4094 = {
                        let a4092 = v1;
                        let a4093 = 128u8;
                        fixed_u8::bit_and(a4092, a4093)
                    };
                    let a4095 = 128u8;
                    if equal(a4094, a4095) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4101 = {
                    let a4098 = {
                        let a4096 = v1;
                        let a4097 = 64u8;
                        fixed_u8::bit_and(a4096, a4097)
                    };
                    let a4099 = 64u8;
                    if equal(a4098, a4099) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4100, a4101)?
            };
            let a4113 = {
                let a4110 = {
                    let a4104 = {
                        let a4102 = v1;
                        let a4103 = 32u8;
                        fixed_u8::bit_and(a4102, a4103)
                    };
                    let a4105 = 32u8;
                    if equal(a4104, a4105) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4111 = {
                    let a4108 = {
                        let a4106 = v1;
                        let a4107 = 16u8;
                        fixed_u8::bit_and(a4106, a4107)
                    };
                    let a4109 = 16u8;
                    if equal(a4108, a4109) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4110, a4111)?
            };
            nat_add(a4112, a4113)?
        };
        let a4137 = {
            let a4134 = {
                let a4122 = {
                    let a4116 = {
                        let a4114 = v1;
                        let a4115 = 8u8;
                        fixed_u8::bit_and(a4114, a4115)
                    };
                    let a4117 = 8u8;
                    if equal(a4116, a4117) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4123 = {
                    let a4120 = {
                        let a4118 = v1;
                        let a4119 = 4u8;
                        fixed_u8::bit_and(a4118, a4119)
                    };
                    let a4121 = 4u8;
                    if equal(a4120, a4121) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4122, a4123)?
            };
            let a4135 = {
                let a4132 = {
                    let a4126 = {
                        let a4124 = v1;
                        let a4125 = 2u8;
                        fixed_u8::bit_and(a4124, a4125)
                    };
                    let a4127 = 2u8;
                    if equal(a4126, a4127) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4133 = {
                    let a4130 = {
                        let a4128 = v1;
                        let a4129 = 1u8;
                        fixed_u8::bit_and(a4128, a4129)
                    };
                    let a4131 = 1u8;
                    if equal(a4130, a4131) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4132, a4133)?
            };
            nat_add(a4134, a4135)?
        };
        nat_add(a4136, a4137)?
    };
    let a4144 = f329(f606(), {
        let a4138 = v2;
        let a4139 = 16u64;
        let a4140 = 0u64;
        nat_quot(a4138, a4139, a4140)
    });
    let a4145 = f329(f606(), {
        let a4141 = v2;
        let a4142 = 16u64;
        let a4143 = 0u64;
        nat_rem(a4141, a4142, a4143)
    });
    Ok(append_bytes(a4144, a4145))
}

pub fn f562(v0: Fn21, p4149: List<List<(Bytes, Bytes)>>, p4150: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4148 = (p4149, p4150);
    {
        let mut m4148 = m4148;
        loop {
            let step = {
                let (v1, v2) = m4148;
                let m4146 = v2;
                match m4146.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4147: Fn21 = v0.clone();
                            c4147.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f563(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f564(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4152 = {
        let a4151 = v0.clone();
        length_list(a4151)
    };
    let a4153 = 1u64;
    if nat_le(a4152, a4153) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f607(v0)?))
    }
}

pub fn f565(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h4154, _) = v0;
        h4154
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h4155) = v0;
        h4155
    };
    let m4156 = v1;
    match m4156.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m4157 = v4;
            match m4157.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f608(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f566(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4158 = f384(v0, v1);
    let a4159 = f534(v0, v2);
    fixed_u32::bit_or(a4158, a4159)
}

pub fn f567(v0: u32) -> u32 {
    let a4162 = {
        let a4160 = f566(v0, 17u32, 15u32);
        let a4161 = f566(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4160, a4161)
    };
    let a4163 = f384(v0, 10u32);
    fixed_u32::bit_xor(a4162, a4163)
}

pub fn f568(v0: u32) -> u32 {
    let a4166 = {
        let a4164 = f566(v0, 7u32, 25u32);
        let a4165 = f566(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4164, a4165)
    };
    let a4167 = f384(v0, 3u32);
    fixed_u32::bit_xor(a4166, a4167)
}

pub fn f569() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x00]), List::cons(Bytes::lit(&[0x01]), List::cons(Bytes::lit(&[0x02]), List::cons(Bytes::lit(&[0x03]), List::cons(Bytes::lit(&[0x04]), List::cons(Bytes::lit(&[0x05]), List::cons(Bytes::lit(&[0x06]), List::cons(Bytes::lit(&[0x07]), List::cons(Bytes::lit(&[0x08]), List::cons(Bytes::lit(&[0x09]), List::cons(Bytes::lit(&[0x0a]), List::cons(Bytes::lit(&[0x0b]), List::cons(Bytes::lit(&[0x0c]), List::cons(Bytes::lit(&[0x0d]), List::cons(Bytes::lit(&[0x0e]), List::cons(Bytes::lit(&[0x0f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f570() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x10]), List::cons(Bytes::lit(&[0x11]), List::cons(Bytes::lit(&[0x12]), List::cons(Bytes::lit(&[0x13]), List::cons(Bytes::lit(&[0x14]), List::cons(Bytes::lit(&[0x15]), List::cons(Bytes::lit(&[0x16]), List::cons(Bytes::lit(&[0x17]), List::cons(Bytes::lit(&[0x18]), List::cons(Bytes::lit(&[0x19]), List::cons(Bytes::lit(&[0x1a]), List::cons(Bytes::lit(&[0x1b]), List::cons(Bytes::lit(&[0x1c]), List::cons(Bytes::lit(&[0x1d]), List::cons(Bytes::lit(&[0x1e]), List::cons(Bytes::lit(&[0x1f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f571() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x20]), List::cons(Bytes::lit(&[0x21]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x27]), List::cons(Bytes::lit(&[0x28]), List::cons(Bytes::lit(&[0x29]), List::cons(Bytes::lit(&[0x2a]), List::cons(Bytes::lit(&[0x2b]), List::cons(Bytes::lit(&[0x2c]), List::cons(Bytes::lit(&[0x2d]), List::cons(Bytes::lit(&[0x2e]), List::cons(Bytes::lit(&[0x2f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f572() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x30]), List::cons(Bytes::lit(&[0x31]), List::cons(Bytes::lit(&[0x32]), List::cons(Bytes::lit(&[0x33]), List::cons(Bytes::lit(&[0x34]), List::cons(Bytes::lit(&[0x35]), List::cons(Bytes::lit(&[0x36]), List::cons(Bytes::lit(&[0x37]), List::cons(Bytes::lit(&[0x38]), List::cons(Bytes::lit(&[0x39]), List::cons(Bytes::lit(&[0x3a]), List::cons(Bytes::lit(&[0x3b]), List::cons(Bytes::lit(&[0x3c]), List::cons(Bytes::lit(&[0x3d]), List::cons(Bytes::lit(&[0x3e]), List::cons(Bytes::lit(&[0x3f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f573() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x40]), List::cons(Bytes::lit(&[0x41]), List::cons(Bytes::lit(&[0x42]), List::cons(Bytes::lit(&[0x43]), List::cons(Bytes::lit(&[0x44]), List::cons(Bytes::lit(&[0x45]), List::cons(Bytes::lit(&[0x46]), List::cons(Bytes::lit(&[0x47]), List::cons(Bytes::lit(&[0x48]), List::cons(Bytes::lit(&[0x49]), List::cons(Bytes::lit(&[0x4a]), List::cons(Bytes::lit(&[0x4b]), List::cons(Bytes::lit(&[0x4c]), List::cons(Bytes::lit(&[0x4d]), List::cons(Bytes::lit(&[0x4e]), List::cons(Bytes::lit(&[0x4f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f574() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x50]), List::cons(Bytes::lit(&[0x51]), List::cons(Bytes::lit(&[0x52]), List::cons(Bytes::lit(&[0x53]), List::cons(Bytes::lit(&[0x54]), List::cons(Bytes::lit(&[0x55]), List::cons(Bytes::lit(&[0x56]), List::cons(Bytes::lit(&[0x57]), List::cons(Bytes::lit(&[0x58]), List::cons(Bytes::lit(&[0x59]), List::cons(Bytes::lit(&[0x5a]), List::cons(Bytes::lit(&[0x5b]), List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x5d]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f575() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x60]), List::cons(Bytes::lit(&[0x61]), List::cons(Bytes::lit(&[0x62]), List::cons(Bytes::lit(&[0x63]), List::cons(Bytes::lit(&[0x64]), List::cons(Bytes::lit(&[0x65]), List::cons(Bytes::lit(&[0x66]), List::cons(Bytes::lit(&[0x67]), List::cons(Bytes::lit(&[0x68]), List::cons(Bytes::lit(&[0x69]), List::cons(Bytes::lit(&[0x6a]), List::cons(Bytes::lit(&[0x6b]), List::cons(Bytes::lit(&[0x6c]), List::cons(Bytes::lit(&[0x6d]), List::cons(Bytes::lit(&[0x6e]), List::cons(Bytes::lit(&[0x6f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f576() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x70]), List::cons(Bytes::lit(&[0x71]), List::cons(Bytes::lit(&[0x72]), List::cons(Bytes::lit(&[0x73]), List::cons(Bytes::lit(&[0x74]), List::cons(Bytes::lit(&[0x75]), List::cons(Bytes::lit(&[0x76]), List::cons(Bytes::lit(&[0x77]), List::cons(Bytes::lit(&[0x78]), List::cons(Bytes::lit(&[0x79]), List::cons(Bytes::lit(&[0x7a]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7c]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x7f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f577() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x80]), List::cons(Bytes::lit(&[0x81]), List::cons(Bytes::lit(&[0x82]), List::cons(Bytes::lit(&[0x83]), List::cons(Bytes::lit(&[0x84]), List::cons(Bytes::lit(&[0x85]), List::cons(Bytes::lit(&[0x86]), List::cons(Bytes::lit(&[0x87]), List::cons(Bytes::lit(&[0x88]), List::cons(Bytes::lit(&[0x89]), List::cons(Bytes::lit(&[0x8a]), List::cons(Bytes::lit(&[0x8b]), List::cons(Bytes::lit(&[0x8c]), List::cons(Bytes::lit(&[0x8d]), List::cons(Bytes::lit(&[0x8e]), List::cons(Bytes::lit(&[0x8f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f578() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x90]), List::cons(Bytes::lit(&[0x91]), List::cons(Bytes::lit(&[0x92]), List::cons(Bytes::lit(&[0x93]), List::cons(Bytes::lit(&[0x94]), List::cons(Bytes::lit(&[0x95]), List::cons(Bytes::lit(&[0x96]), List::cons(Bytes::lit(&[0x97]), List::cons(Bytes::lit(&[0x98]), List::cons(Bytes::lit(&[0x99]), List::cons(Bytes::lit(&[0x9a]), List::cons(Bytes::lit(&[0x9b]), List::cons(Bytes::lit(&[0x9c]), List::cons(Bytes::lit(&[0x9d]), List::cons(Bytes::lit(&[0x9e]), List::cons(Bytes::lit(&[0x9f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f579() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xa0]), List::cons(Bytes::lit(&[0xa1]), List::cons(Bytes::lit(&[0xa2]), List::cons(Bytes::lit(&[0xa3]), List::cons(Bytes::lit(&[0xa4]), List::cons(Bytes::lit(&[0xa5]), List::cons(Bytes::lit(&[0xa6]), List::cons(Bytes::lit(&[0xa7]), List::cons(Bytes::lit(&[0xa8]), List::cons(Bytes::lit(&[0xa9]), List::cons(Bytes::lit(&[0xaa]), List::cons(Bytes::lit(&[0xab]), List::cons(Bytes::lit(&[0xac]), List::cons(Bytes::lit(&[0xad]), List::cons(Bytes::lit(&[0xae]), List::cons(Bytes::lit(&[0xaf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f580() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xb0]), List::cons(Bytes::lit(&[0xb1]), List::cons(Bytes::lit(&[0xb2]), List::cons(Bytes::lit(&[0xb3]), List::cons(Bytes::lit(&[0xb4]), List::cons(Bytes::lit(&[0xb5]), List::cons(Bytes::lit(&[0xb6]), List::cons(Bytes::lit(&[0xb7]), List::cons(Bytes::lit(&[0xb8]), List::cons(Bytes::lit(&[0xb9]), List::cons(Bytes::lit(&[0xba]), List::cons(Bytes::lit(&[0xbb]), List::cons(Bytes::lit(&[0xbc]), List::cons(Bytes::lit(&[0xbd]), List::cons(Bytes::lit(&[0xbe]), List::cons(Bytes::lit(&[0xbf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f581() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xc0]), List::cons(Bytes::lit(&[0xc1]), List::cons(Bytes::lit(&[0xc2]), List::cons(Bytes::lit(&[0xc3]), List::cons(Bytes::lit(&[0xc4]), List::cons(Bytes::lit(&[0xc5]), List::cons(Bytes::lit(&[0xc6]), List::cons(Bytes::lit(&[0xc7]), List::cons(Bytes::lit(&[0xc8]), List::cons(Bytes::lit(&[0xc9]), List::cons(Bytes::lit(&[0xca]), List::cons(Bytes::lit(&[0xcb]), List::cons(Bytes::lit(&[0xcc]), List::cons(Bytes::lit(&[0xcd]), List::cons(Bytes::lit(&[0xce]), List::cons(Bytes::lit(&[0xcf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f582() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xd0]), List::cons(Bytes::lit(&[0xd1]), List::cons(Bytes::lit(&[0xd2]), List::cons(Bytes::lit(&[0xd3]), List::cons(Bytes::lit(&[0xd4]), List::cons(Bytes::lit(&[0xd5]), List::cons(Bytes::lit(&[0xd6]), List::cons(Bytes::lit(&[0xd7]), List::cons(Bytes::lit(&[0xd8]), List::cons(Bytes::lit(&[0xd9]), List::cons(Bytes::lit(&[0xda]), List::cons(Bytes::lit(&[0xdb]), List::cons(Bytes::lit(&[0xdc]), List::cons(Bytes::lit(&[0xdd]), List::cons(Bytes::lit(&[0xde]), List::cons(Bytes::lit(&[0xdf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f583() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xe0]), List::cons(Bytes::lit(&[0xe1]), List::cons(Bytes::lit(&[0xe2]), List::cons(Bytes::lit(&[0xe3]), List::cons(Bytes::lit(&[0xe4]), List::cons(Bytes::lit(&[0xe5]), List::cons(Bytes::lit(&[0xe6]), List::cons(Bytes::lit(&[0xe7]), List::cons(Bytes::lit(&[0xe8]), List::cons(Bytes::lit(&[0xe9]), List::cons(Bytes::lit(&[0xea]), List::cons(Bytes::lit(&[0xeb]), List::cons(Bytes::lit(&[0xec]), List::cons(Bytes::lit(&[0xed]), List::cons(Bytes::lit(&[0xee]), List::cons(Bytes::lit(&[0xef]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f584() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xf0]), List::cons(Bytes::lit(&[0xf1]), List::cons(Bytes::lit(&[0xf2]), List::cons(Bytes::lit(&[0xf3]), List::cons(Bytes::lit(&[0xf4]), List::cons(Bytes::lit(&[0xf5]), List::cons(Bytes::lit(&[0xf6]), List::cons(Bytes::lit(&[0xf7]), List::cons(Bytes::lit(&[0xf8]), List::cons(Bytes::lit(&[0xf9]), List::cons(Bytes::lit(&[0xfa]), List::cons(Bytes::lit(&[0xfb]), List::cons(Bytes::lit(&[0xfc]), List::cons(Bytes::lit(&[0xfd]), List::cons(Bytes::lit(&[0xfe]), List::cons(Bytes::lit(&[0xff]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f585(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f415(v2) {
        true
    } else {
        f416(v2)
    } {
        let a4168 = v2;
        let a4169 = 117u64;
        if !nat_eq(a4168, a4169) {
            let a4170 = v2;
            let a4171 = 85u64;
            !nat_eq(a4170, a4171)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f417(v2) {
            let a4172 = 0u64;
            let a4173 = v1;
            nat_lt(a4172, a4173)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a4174 = v2;
        let a4175 = 65536u64;
        if nat_lt(a4174, a4175) {
            let a4176 = f8(Str::lit("u"));
            let a4177 = f586(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4176, a4177)
        } else {
            let a4178 = f8(Str::lit("U"));
            let a4179 = f586(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4178, a4179)
        }
    }
}

pub fn f586(p4196: u64, p4197: u64, p4198: u64, p4199: Bytes) -> Bytes {
    let m4195 = (p4196, (p4197, (p4198, p4199)));
    {
        let mut m4195 = m4195;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m4195;
                let m4180 = v0;
                if m4180 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m4180 - 1;
                    let a4181 = v1;
                    let a4182 = 0u64;
                    if if nat_eq(a4181, a4182) {
                        let a4183 = v2;
                        let a4184 = 0u64;
                        nat_eq(a4183, a4184)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a4185 = v1;
                            let a4186 = 16u64;
                            let a4187 = 0u64;
                            nat_quot(a4185, a4186, a4187)
                        }, ({
                            let a4188 = v2;
                            let a4189 = 1u64;
                            nat_sub(a4188, a4189)
                        }, {
                            let a4193 = f609({
                                let a4190 = v1;
                                let a4191 = 16u64;
                                let a4192 = 0u64;
                                nat_rem(a4190, a4191, a4192)
                            });
                            let a4194 = v3;
                            append_bytes(a4193, a4194)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m4195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(v0: Bytes, v1: u64) -> R<Bytes> {
    let a4202 = f32(v0.clone(), {
        let a4200 = v1;
        let a4201 = 2u64;
        nat_add(a4200, a4201)?
    }, 2u64);
    let a4203 = Bytes::lit(&[0x30, 0x30]);
    if equal(a4202, a4203) {
        let m4206 = f610(f32(v0, {
            let a4204 = v1;
            let a4205 = 4u64;
            nat_add(a4204, a4205)?
        }, 2u64))?;
        match m4206 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f325(v2.clone(), 0u64)?;
                let a4207 = v3;
                let a4208 = 32u64;
                if nat_lt(a4207, a4208) {
                    if f611(v3) {
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

pub fn f588(v0: u64) -> Bytes {
    let a4209 = v0;
    let a4210 = 34u64;
    if nat_eq(a4209, a4210) {
        Bytes::lit(&[0x22])
    } else {
        let a4211 = v0;
        let a4212 = 92u64;
        if nat_eq(a4211, a4212) {
            Bytes::lit(&[0x5c])
        } else {
            let a4213 = v0;
            let a4214 = 98u64;
            if nat_eq(a4213, a4214) {
                Bytes::lit(&[0x08])
            } else {
                let a4215 = v0;
                let a4216 = 102u64;
                if nat_eq(a4215, a4216) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a4217 = v0;
                    let a4218 = 110u64;
                    if nat_eq(a4217, a4218) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a4219 = v0;
                        let a4220 = 114u64;
                        if nat_eq(a4219, a4220) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a4221 = v0;
                            let a4222 = 116u64;
                            if nat_eq(a4221, a4222) {
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

pub fn f589(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m4225 = {
        let a4223 = v0;
        let a4224 = v1;
        index_bytes(a4223, a4224)
    };
    match m4225 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f612(v2)? {
                Ok(Some({
                    let a4226 = v1;
                    let a4227 = 1u64;
                    nat_add(a4226, a4227)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f590(v0: u8) -> R<u64> {
    let a4274 = {
        let v1: u8 = v0;
        let a4272 = {
            let a4248 = {
                let a4236 = {
                    let a4230 = {
                        let a4228 = v1;
                        let a4229 = 128u8;
                        fixed_u8::bit_and(a4228, a4229)
                    };
                    let a4231 = 128u8;
                    if equal(a4230, a4231) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4237 = {
                    let a4234 = {
                        let a4232 = v1;
                        let a4233 = 64u8;
                        fixed_u8::bit_and(a4232, a4233)
                    };
                    let a4235 = 64u8;
                    if equal(a4234, a4235) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4236, a4237)?
            };
            let a4249 = {
                let a4246 = {
                    let a4240 = {
                        let a4238 = v1;
                        let a4239 = 32u8;
                        fixed_u8::bit_and(a4238, a4239)
                    };
                    let a4241 = 32u8;
                    if equal(a4240, a4241) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4247 = {
                    let a4244 = {
                        let a4242 = v1;
                        let a4243 = 16u8;
                        fixed_u8::bit_and(a4242, a4243)
                    };
                    let a4245 = 16u8;
                    if equal(a4244, a4245) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4246, a4247)?
            };
            nat_add(a4248, a4249)?
        };
        let a4273 = {
            let a4270 = {
                let a4258 = {
                    let a4252 = {
                        let a4250 = v1;
                        let a4251 = 8u8;
                        fixed_u8::bit_and(a4250, a4251)
                    };
                    let a4253 = 8u8;
                    if equal(a4252, a4253) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4259 = {
                    let a4256 = {
                        let a4254 = v1;
                        let a4255 = 4u8;
                        fixed_u8::bit_and(a4254, a4255)
                    };
                    let a4257 = 4u8;
                    if equal(a4256, a4257) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4258, a4259)?
            };
            let a4271 = {
                let a4268 = {
                    let a4262 = {
                        let a4260 = v1;
                        let a4261 = 2u8;
                        fixed_u8::bit_and(a4260, a4261)
                    };
                    let a4263 = 2u8;
                    if equal(a4262, a4263) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4269 = {
                    let a4266 = {
                        let a4264 = v1;
                        let a4265 = 1u8;
                        fixed_u8::bit_and(a4264, a4265)
                    };
                    let a4267 = 1u8;
                    if equal(a4266, a4267) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4268, a4269)?
            };
            nat_add(a4270, a4271)?
        };
        nat_add(a4272, a4273)?
    };
    let a4275 = 48u64;
    Ok(nat_sub(a4274, a4275))
}

pub fn f591(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a4276 = v0.clone();
        length_bytes(a4276)
    };
    let a4277 = v1;
    let a4278 = v2;
    if if nat_lt(a4277, a4278) {
        let a4283 = f325(v0, {
            let a4281 = v2;
            let a4282 = {
                let a4279 = v1;
                let a4280 = 1u64;
                nat_add(a4279, a4280)?
            };
            nat_sub(a4281, a4282)
        })?;
        let a4284 = 48u64;
        nat_eq(a4283, a4284)
    } else {
        false
    } {
        Ok(Some({
            let a4285 = v1;
            let a4286 = 1u64;
            nat_add(a4285, a4286)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f592(v0: List<(Bytes, Adt19)>, v1: List<Adt19>) -> List<Bytes> {
    let m4287 = v0;
    match m4287.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f613(v1.clone(), {
                let (h4288, _) = v2.clone();
                h4288
            }) {
                List::cons({
                    let (h4289, _) = v2;
                    h4289
                }, f592(v3, v1))
            } else {
                f592(v3, v1)
            }
        }
    }
}

pub fn f593(v0: bool, v1: Bytes, v2: List<Adt19>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4290 = v2;
    match m4290.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m4291 = f614(v4.clone())?;
            match m4291 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m4292 = f502(v0, v6.clone())?;
                    match m4292 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f87(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4295 = f615(v4, {
                                    let a4293 = v1.clone();
                                    let a4294 = f254(v6)?;
                                    append_bytes(a4293, a4294)
                                })?;
                                match m4295 {
                                    Ok(v9) => {
                                        let m4296 = f593(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m4296 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h4297, _) = v9;
                                                    h4297
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4298, _) = v11;
                                                    h4298
                                                }), f423({
                                                    let (_, h4299) = v9;
                                                    h4299
                                                }, {
                                                    let (_, h4300) = v11;
                                                    h4300
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

pub fn f594(v0: List<Adt19>) -> R<List<Bytes>> {
    let m4301 = v0;
    match m4301.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m4302 = f614(v1)?;
            match m4302 {
                None => {
                    f594(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f594(v2)?))
                }
            }
        }
    }
}

pub fn f595(v0: Bytes, v1: List<Adt19>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f616(f407(f617(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f596(v0: Adt19, v1: Bytes) -> R<bool> {
    let m4303 = f258(Some(v0));
    match m4303 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m4304 = f547(v2)?;
            match m4304 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f36(v3, v1))
                }
            }
        }
    }
}

pub fn f597(v0: bool, v1: Bytes, v2: List<(Bytes, Adt19)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4305 = v2;
    match m4305.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m4307 = f502(v0, {
                let (h4306, _) = v5.clone();
                h4306
            })?;
            match m4307 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h4308, _) = v5;
                        h4308
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f87(v7.clone(), f460(v0, f598(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m4313 = f554({
                            let (_, h4309) = v5.clone();
                            h4309
                        }, {
                            let a4311 = v1.clone();
                            let a4312 = f254({
                                let (h4310, _) = v5.clone();
                                h4310
                            })?;
                            append_bytes(a4311, a4312)
                        })?;
                        match m4313 {
                            Ok(v9) => {
                                let v10: Bytes = if f87({
                                    let (h4314, _) = v5;
                                    h4314
                                }, v3.clone()) {
                                    let (h4315, _) = v9;
                                    h4315
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h4316, _) = v9;
                                        h4316
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m4317 = f597(v0, v1, v6, v3, v4)?;
                                match m4317 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h4318, _) = v12;
                                            h4318
                                        }), f423({
                                            let (_, h4319) = v9;
                                            h4319
                                        }, {
                                            let (_, h4320) = v12;
                                            h4320
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

pub fn f598(v0: List<(Bytes, Adt19)>) -> List<Bytes> {
    let m4321 = v0;
    match m4321.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h4322, _) = v1;
                h4322
            }, f598(v2))
        }
    }
}

pub fn f599(v0: Adt19) -> Option<Adt19> {
    let m4323 = f155(f150(v0, f8(Str::lit("oneOf"))));
    match m4323.uncons() {
        None => {
            None::<Adt19>
        }
        Some((v1, v2)) => {
            let m4324 = v2;
            match m4324.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v3, v4)) => {
                    let m4325 = v4;
                    match m4325.uncons() {
                        None => {
                            if f618(v3.clone()) {
                                Some(v1)
                            } else {
                                if f618(v1) {
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

pub fn f600(v0: Bytes, v1: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4326 = v1;
    match m4326 {
        Ok(v3) => {
            f550(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h4327, _) = v3;
                h4327
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h4328) = v3;
                h4328
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v2)
        }
    }
}

pub fn f601(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4329 = f599(v0.clone());
    match m4329 {
        None => {
            if f36(f153(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4330 = f150(v0, f8(Str::lit("items")));
                match m4330 {
                    None => {
                        Ok(f600(f8(Str::lit("List")), f602()))
                    }
                    Some(v3) => {
                        Ok(f600(f8(Str::lit("List")), f615(v3, {
                            let a4331 = v1;
                            let a4332 = f8(Str::lit("Item"));
                            append_bytes(a4331, a4332)
                        })?))
                    }
                }
            } else {
                f615(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f600(f8(Str::lit("Option")), f615(v2, v1)?))
        }
    }
}

pub fn f602() -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((f472(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f603(p4336: List<(Bytes, Adt19)>, v1: List<Bytes>) -> bool {
    let m4335 = p4336;
    {
        let mut m4335 = m4335;
        loop {
            let step = {
                let v0 = m4335;
                let m4333 = v0;
                match m4333.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f87({
                            let (h4334, _) = v2;
                            h4334
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt19)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt19)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4335 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(p4340: List<Adt19>, v1: Bytes, p4341: List<Bytes>) -> bool {
    let m4339 = (p4340, p4341);
    {
        let mut m4339 = m4339;
        loop {
            let step = {
                let (v0, v2) = m4339;
                let m4337 = v0;
                match m4337.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m4338 = f619(v3, v1.clone());
                        match m4338 {
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
                Ok(next) => { m4339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f605(p4344: List<Adt19>, p4345: List<Bytes>) -> bool {
    let m4343 = (p4344, p4345);
    {
        let mut m4343 = m4343;
        loop {
            let step = {
                let (v0, v1) = m4343;
                let m4342 = v0;
                match m4342.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f153(v2, f8(Str::lit("type")));
                        if !f36(v4.clone(), Bytes::lit(&[])) {
                            if !f87(v4.clone(), v1.clone()) {
                                if !if if f36(v4.clone(), f8(Str::lit("integer"))) {
                                    f87(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f36(v4.clone(), f8(Str::lit("number"))) {
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
                Ok(next) => { m4343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f606() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f607(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f519({
        let (_, h4346) = {
            let (h4347, _) = f620(Fn22::F621, {
                let a4349 = {
                    let a4348 = v0.clone();
                    length_list(a4348)
                };
                let a4350 = 1u64;
                nat_add(a4349, a4350)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4347
        };
        h4346
    }))
}

pub fn f608(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a4355 = {
            let a4353 = {
                let a4351 = v0.clone();
                length_list(a4351)
            };
            let a4354 = {
                let a4352 = v1.clone();
                length_list(a4352)
            };
            nat_add(a4353, a4354)?
        };
        let a4356 = 1u64;
        nat_add(a4355, a4356)?
    };
    Ok(f374({
        let (_, h4357) = {
            let (_, h4358) = {
                let (h4359, _) = f622(Fn23::F623, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h4359
            };
            h4358
        };
        h4357
    }))
}

pub fn f609(v0: u64) -> Bytes {
    f32({
        let a4360 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4360)
    }, v0, 1u64)
}

pub fn f610(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a4361 = v0.clone();
        length_bytes(a4361)
    };
    let a4365 = {
        let a4362 = v1;
        let a4363 = 2u64;
        let a4364 = 0u64;
        nat_rem(a4362, a4363, a4364)
    };
    let a4366 = 0u64;
    if nat_eq(a4365, a4366) {
        let (_, h4367) = {
            let (h4368, _) = f624(Fn24::F625(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h4368
        };
        Ok(h4367)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f611(v0: u64) -> bool {
    let a4369 = v0;
    let a4370 = 8u64;
    if nat_eq(a4369, a4370) {
        true
    } else {
        let a4371 = v0;
        let a4372 = 9u64;
        if nat_eq(a4371, a4372) {
            true
        } else {
            let a4373 = v0;
            let a4374 = 10u64;
            if nat_eq(a4373, a4374) {
                true
            } else {
                let a4375 = v0;
                let a4376 = 12u64;
                if nat_eq(a4375, a4376) {
                    true
                } else {
                    let a4377 = v0;
                    let a4378 = 13u64;
                    nat_eq(a4377, a4378)
                }
            }
        }
    }
}

pub fn f612(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4423 = {
            let a4399 = {
                let a4387 = {
                    let a4381 = {
                        let a4379 = v1;
                        let a4380 = 128u8;
                        fixed_u8::bit_and(a4379, a4380)
                    };
                    let a4382 = 128u8;
                    if equal(a4381, a4382) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4388 = {
                    let a4385 = {
                        let a4383 = v1;
                        let a4384 = 64u8;
                        fixed_u8::bit_and(a4383, a4384)
                    };
                    let a4386 = 64u8;
                    if equal(a4385, a4386) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4387, a4388)?
            };
            let a4400 = {
                let a4397 = {
                    let a4391 = {
                        let a4389 = v1;
                        let a4390 = 32u8;
                        fixed_u8::bit_and(a4389, a4390)
                    };
                    let a4392 = 32u8;
                    if equal(a4391, a4392) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4398 = {
                    let a4395 = {
                        let a4393 = v1;
                        let a4394 = 16u8;
                        fixed_u8::bit_and(a4393, a4394)
                    };
                    let a4396 = 16u8;
                    if equal(a4395, a4396) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4397, a4398)?
            };
            nat_add(a4399, a4400)?
        };
        let a4424 = {
            let a4421 = {
                let a4409 = {
                    let a4403 = {
                        let a4401 = v1;
                        let a4402 = 8u8;
                        fixed_u8::bit_and(a4401, a4402)
                    };
                    let a4404 = 8u8;
                    if equal(a4403, a4404) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4410 = {
                    let a4407 = {
                        let a4405 = v1;
                        let a4406 = 4u8;
                        fixed_u8::bit_and(a4405, a4406)
                    };
                    let a4408 = 4u8;
                    if equal(a4407, a4408) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4409, a4410)?
            };
            let a4422 = {
                let a4419 = {
                    let a4413 = {
                        let a4411 = v1;
                        let a4412 = 2u8;
                        fixed_u8::bit_and(a4411, a4412)
                    };
                    let a4414 = 2u8;
                    if equal(a4413, a4414) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4420 = {
                    let a4417 = {
                        let a4415 = v1;
                        let a4416 = 1u8;
                        fixed_u8::bit_and(a4415, a4416)
                    };
                    let a4418 = 1u8;
                    if equal(a4417, a4418) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4419, a4420)?
            };
            nat_add(a4421, a4422)?
        };
        nat_add(a4423, a4424)?
    };
    let a4425 = 48u64;
    let a4426 = v2;
    if nat_le(a4425, a4426) {
        let a4427 = v2;
        let a4428 = 57u64;
        Ok(nat_le(a4427, a4428))
    } else {
        Ok(false)
    }
}

pub fn f613(p4432: List<Adt19>, v1: Bytes) -> bool {
    let m4431 = p4432;
    {
        let mut m4431 = m4431;
        loop {
            let step = {
                let v0 = m4431;
                let m4429 = v0;
                match m4429.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m4430 = f619(v2, v1.clone());
                        if match m4430 {
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
                Ok(next) => { m4431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: Adt19) -> R<Option<Bytes>> {
    let m4433 = f258(f150(v0.clone(), f8(Str::lit("$ref"))));
    match m4433 {
        None => {
            Ok(f258(f150(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f547(v1)
        }
    }
}

pub fn f615(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f626(f517(v0), v1)
}

pub fn f616(v0: bool, v1: Bytes, v2: List<Adt19>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4434 = v2;
    match m4434.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m4435 = f619(v5.clone(), v3.clone());
            match m4435 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m4436 = f502(v0, v7.clone())?;
                    match m4436 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f87(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4439 = f553({
                                    let a4437 = v1.clone();
                                    let a4438 = f254(v7)?;
                                    append_bytes(a4437, a4438)
                                }, f471(f419(f150(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f154(f155(f150(v5, f8(Str::lit("required"))))), true)?;
                                match m4439 {
                                    Ok(v10) => {
                                        let m4440 = f616(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m4440 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f217({
                                                    let (h4441, _) = v10;
                                                    h4441
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4442, _) = v12;
                                                    h4442
                                                }), f423({
                                                    let (_, h4443) = v10;
                                                    h4443
                                                }, {
                                                    let (_, h4444) = v12;
                                                    h4444
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

pub fn f617(v0: List<Adt19>, v1: Bytes) -> List<Bytes> {
    let m4445 = v0;
    match m4445.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4446 = f619(v2, v1.clone());
            match m4446 {
                None => {
                    f617(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f617(v3, v1))
                }
            }
        }
    }
}

pub fn f618(v0: Adt19) -> bool {
    f36(f153(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f619(v0: Adt19, v1: Bytes) -> Option<Bytes> {
    if f87(v1.clone(), f154(f155(f150(v0.clone(), f8(Str::lit("required")))))) {
        let m4447 = f150(v0, f8(Str::lit("properties")));
        match m4447 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m4448 = f150(v2, v1);
                match m4448 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f258(f150(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f620(v0: Fn22, p4453: u64, p4454: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4452 = (p4453, p4454);
    {
        let mut m4452 = m4452;
        loop {
            let step = {
                let (v1, v2) = m4452;
                let m4449 = v1;
                if m4449 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4449 - 1;
                    let m4451 = {
                        let c4450: Fn22 = v0.clone();
                        c4450.apply(v2.clone())?
                    };
                    match m4451 {
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
                Ok(next) => { m4452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f627(v0)
}

pub fn f622(v0: Fn23, p4459: u64, p4460: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4458 = (p4459, p4460);
    {
        let mut m4458 = m4458;
        loop {
            let step = {
                let (v1, v2) = m4458;
                let m4455 = v1;
                if m4455 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4455 - 1;
                    let m4457 = {
                        let c4456: Fn23 = v0.clone();
                        c4456.apply(v2.clone())
                    };
                    match m4457 {
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
                Ok(next) => { m4458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f628(v0)
}

pub fn f624(v0: Fn24, p4465: u64, p4466: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4464 = (p4465, p4466);
    {
        let mut m4464 = m4464;
        loop {
            let step = {
                let (v1, v2) = m4464;
                let m4461 = v1;
                if m4461 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4461 - 1;
                    let m4463 = {
                        let c4462: Fn24 = v0.clone();
                        c4462.apply(v2.clone())?
                    };
                    match m4463 {
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
                Ok(next) => { m4464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f625(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f629(v0, v1)
}

pub fn f626(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4467 = v0.clone();
    match m4467 {
        Adt19::C0(v2) => {
            if v2 {
                Ok(f602())
            } else {
                Ok(f549(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt19::C1(_, _) => {
            Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C2(_, _, _) => {
            Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C3(_) => {
            Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C4(_) => {
            Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C5(_) => {
            let m4468 = f630(f469(v0.clone()));
            match m4468 {
                None => {
                    let m4469 = f258(f150(v0.clone(), f8(Str::lit("$ref"))));
                    match m4469 {
                        None => {
                            let m4471 = f150(v0.clone(), f8(Str::lit("allOf")));
                            match m4471 {
                                None => {
                                    let m4476 = f150(v0.clone(), f8(Str::lit("enum")));
                                    match m4476 {
                                        None => {
                                            let m4477 = f150(v0.clone(), f8(Str::lit("const")));
                                            match m4477 {
                                                None => {
                                                    let m4479 = f150(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4479 {
                                                        None => {
                                                            let v30: Bytes = f153(v0.clone(), f8(Str::lit("type")));
                                                            if if f36(v30.clone(), Bytes::lit(&[])) {
                                                                !f631(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f602())
                                                            } else {
                                                                if f36(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f550(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f36(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f550(f632(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f36(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f550(f500(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f36(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f550(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f36(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m4480 = f419(f150(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4480.uncons() {
                                                                                        None => {
                                                                                            let m4481 = f633(v0);
                                                                                            match m4481 {
                                                                                                None => {
                                                                                                    Ok(f600(f8(Str::lit("List")), f634(f602())))
                                                                                                }
                                                                                                Some(v33) => {
                                                                                                    Ok(f600(f8(Str::lit("List")), f634(f635(v33, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f550(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f549(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f550(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m4478 = f258(Some(v27));
                                                    match m4478 {
                                                        None => {
                                                            Ok(f549(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f550(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f550(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m4472 = f259(v19);
                                    match m4472.uncons() {
                                        None => {
                                            Ok(f549(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m4473 = v21;
                                            match m4473.uncons() {
                                                None => {
                                                    let m4474 = f258(f150(v20, f8(Str::lit("$ref"))));
                                                    match m4474 {
                                                        None => {
                                                            Ok(f549(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m4475 = f547(v24)?;
                                                            match m4475 {
                                                                None => {
                                                                    Ok(f549(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f550(v25, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f549(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m4470 = f547(v17)?;
                            match m4470 {
                                None => {
                                    Ok(f549(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f550(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f549(if f36(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt19::C6 => {
            Ok(f602())
        }
        Adt19::C7(_, _) => {
            Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C8(_, _, _) => {
            Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f627(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4482, _) = v0;
        h4482
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4483) = v0;
        h4483
    };
    let m4484 = v1;
    match m4484.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4485 = v4;
            match m4485.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f636(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f628(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4486, _) = v0.clone();
        h4486
    };
    let v2: List<(Bytes, ())> = {
        let (h4487, _) = {
            let (_, h4488) = v0.clone();
            h4488
        };
        h4487
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4489) = {
            let (_, h4490) = v0;
            h4490
        };
        h4489
    };
    let m4491 = v1.clone();
    match m4491.uncons() {
        None => {
            let m4492 = v2;
            match m4492.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4493 = v2.clone();
            match m4493.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f637({
                        let (h4494, _) = v8.clone();
                        h4494
                    }, {
                        let (h4495, _) = v6.clone();
                        h4495
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

pub fn f629(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4496, _) = v1;
        h4496
    };
    let m4498 = {
        let (_, h4497) = v1;
        h4497
    };
    match m4498 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4501 = {
                let a4499 = v0.clone();
                let a4500 = v2;
                index_bytes(a4499, a4500)
            };
            match m4501 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4506 = {
                        let a4504 = v0;
                        let a4505 = {
                            let a4502 = v2;
                            let a4503 = 1u64;
                            nat_add(a4502, a4503)?
                        };
                        index_bytes(a4504, a4505)
                    };
                    match m4506 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f638(v4)? {
                                f638(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4507 = v2;
                                    let a4508 = 2u64;
                                    nat_add(a4507, a4508)?
                                }, Some({
                                    let a4513 = v3;
                                    let a4514 = f329(f639(), {
                                        let a4511 = {
                                            let a4509 = f640(v4)?;
                                            let a4510 = 16u64;
                                            nat_mul(a4509, a4510)?
                                        };
                                        let a4512 = f640(v5)?;
                                        nat_add(a4511, a4512)?
                                    });
                                    append_bytes(a4513, a4514)
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

pub fn f630(p4519: List<(Bytes, Adt19)>) -> Option<Bytes> {
    let m4518 = p4519;
    {
        let mut m4518 = m4518;
        loop {
            let step = {
                let v0 = m4518;
                let m4515 = v0;
                match m4515.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f87({
                            let (h4516, _) = v1.clone();
                            h4516
                        }, f641()) {
                            Err::<List<(Bytes, Adt19)>, Option<Bytes>>(Some({
                                let (h4517, _) = v1;
                                h4517
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631(v0: Adt19) -> bool {
    if f557(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f557(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f557(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f557(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f557(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f557(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f632(v0: Adt19) -> Bytes {
    let m4520 = f642(f150(v0, f8(Str::lit("minimum"))));
    match m4520 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f633(v0: Adt19) -> Option<Adt19> {
    let m4521 = f150(v0.clone(), f8(Str::lit("additionalProperties")));
    match m4521 {
        None => {
            let m4522 = f419(f150(v0, f8(Str::lit("patternProperties"))));
            match m4522.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v2, v3)) => {
                    let m4523 = v3;
                    match m4523.uncons() {
                        None => {
                            Some({
                                let (_, h4524) = v2;
                                h4524
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

pub fn f634(v0: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4525 = v0;
    match m4525 {
        Ok(v2) => {
            f550(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h4526, _) = v2;
                h4526
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h4527) = v2;
                h4527
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v1)
        }
    }
}

pub fn f635(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f643(16u64, v0, v1)
}

pub fn f636(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4532 = {
            let a4530 = {
                let a4528 = v0.clone();
                length_list(a4528)
            };
            let a4531 = {
                let a4529 = v1.clone();
                length_list(a4529)
            };
            nat_add(a4530, a4531)?
        };
        let a4533 = 1u64;
        nat_add(a4532, a4533)?
    };
    Ok(f644({
        let (_, h4534) = {
            let (_, h4535) = {
                let (h4536, _) = f645(Fn25::F646, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4536
            };
            h4535
        };
        h4534
    }))
}

pub fn f637(v0: Bytes, v1: Bytes) -> bool {
    let a4541 = {
        let a4537 = v0;
        let a4538 = v1;
        compare_bytes(a4537, a4538)
    };
    let a4542 = {
        let a4539 = Bytes::lit(&[]);
        let a4540 = Bytes::lit(&[0x01]);
        compare_bytes(a4539, a4540)
    };
    equal(a4541, a4542)
}

pub fn f638(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4587 = {
            let a4563 = {
                let a4551 = {
                    let a4545 = {
                        let a4543 = v1;
                        let a4544 = 128u8;
                        fixed_u8::bit_and(a4543, a4544)
                    };
                    let a4546 = 128u8;
                    if equal(a4545, a4546) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4552 = {
                    let a4549 = {
                        let a4547 = v1;
                        let a4548 = 64u8;
                        fixed_u8::bit_and(a4547, a4548)
                    };
                    let a4550 = 64u8;
                    if equal(a4549, a4550) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4551, a4552)?
            };
            let a4564 = {
                let a4561 = {
                    let a4555 = {
                        let a4553 = v1;
                        let a4554 = 32u8;
                        fixed_u8::bit_and(a4553, a4554)
                    };
                    let a4556 = 32u8;
                    if equal(a4555, a4556) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4562 = {
                    let a4559 = {
                        let a4557 = v1;
                        let a4558 = 16u8;
                        fixed_u8::bit_and(a4557, a4558)
                    };
                    let a4560 = 16u8;
                    if equal(a4559, a4560) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4561, a4562)?
            };
            nat_add(a4563, a4564)?
        };
        let a4588 = {
            let a4585 = {
                let a4573 = {
                    let a4567 = {
                        let a4565 = v1;
                        let a4566 = 8u8;
                        fixed_u8::bit_and(a4565, a4566)
                    };
                    let a4568 = 8u8;
                    if equal(a4567, a4568) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4574 = {
                    let a4571 = {
                        let a4569 = v1;
                        let a4570 = 4u8;
                        fixed_u8::bit_and(a4569, a4570)
                    };
                    let a4572 = 4u8;
                    if equal(a4571, a4572) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4573, a4574)?
            };
            let a4586 = {
                let a4583 = {
                    let a4577 = {
                        let a4575 = v1;
                        let a4576 = 2u8;
                        fixed_u8::bit_and(a4575, a4576)
                    };
                    let a4578 = 2u8;
                    if equal(a4577, a4578) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4584 = {
                    let a4581 = {
                        let a4579 = v1;
                        let a4580 = 1u8;
                        fixed_u8::bit_and(a4579, a4580)
                    };
                    let a4582 = 1u8;
                    if equal(a4581, a4582) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4583, a4584)?
            };
            nat_add(a4585, a4586)?
        };
        nat_add(a4587, a4588)?
    };
    let a4589 = 48u64;
    let a4590 = v2;
    if if nat_le(a4589, a4590) {
        let a4591 = v2;
        let a4592 = 57u64;
        nat_le(a4591, a4592)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4593 = 97u64;
        let a4594 = v2;
        if nat_le(a4593, a4594) {
            let a4595 = v2;
            let a4596 = 102u64;
            Ok(nat_le(a4595, a4596))
        } else {
            Ok(false)
        }
    }
}

pub fn f639() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f640(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4641 = {
            let a4617 = {
                let a4605 = {
                    let a4599 = {
                        let a4597 = v1;
                        let a4598 = 128u8;
                        fixed_u8::bit_and(a4597, a4598)
                    };
                    let a4600 = 128u8;
                    if equal(a4599, a4600) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4606 = {
                    let a4603 = {
                        let a4601 = v1;
                        let a4602 = 64u8;
                        fixed_u8::bit_and(a4601, a4602)
                    };
                    let a4604 = 64u8;
                    if equal(a4603, a4604) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4605, a4606)?
            };
            let a4618 = {
                let a4615 = {
                    let a4609 = {
                        let a4607 = v1;
                        let a4608 = 32u8;
                        fixed_u8::bit_and(a4607, a4608)
                    };
                    let a4610 = 32u8;
                    if equal(a4609, a4610) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4616 = {
                    let a4613 = {
                        let a4611 = v1;
                        let a4612 = 16u8;
                        fixed_u8::bit_and(a4611, a4612)
                    };
                    let a4614 = 16u8;
                    if equal(a4613, a4614) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4615, a4616)?
            };
            nat_add(a4617, a4618)?
        };
        let a4642 = {
            let a4639 = {
                let a4627 = {
                    let a4621 = {
                        let a4619 = v1;
                        let a4620 = 8u8;
                        fixed_u8::bit_and(a4619, a4620)
                    };
                    let a4622 = 8u8;
                    if equal(a4621, a4622) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4628 = {
                    let a4625 = {
                        let a4623 = v1;
                        let a4624 = 4u8;
                        fixed_u8::bit_and(a4623, a4624)
                    };
                    let a4626 = 4u8;
                    if equal(a4625, a4626) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4627, a4628)?
            };
            let a4640 = {
                let a4637 = {
                    let a4631 = {
                        let a4629 = v1;
                        let a4630 = 2u8;
                        fixed_u8::bit_and(a4629, a4630)
                    };
                    let a4632 = 2u8;
                    if equal(a4631, a4632) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4638 = {
                    let a4635 = {
                        let a4633 = v1;
                        let a4634 = 1u8;
                        fixed_u8::bit_and(a4633, a4634)
                    };
                    let a4636 = 1u8;
                    if equal(a4635, a4636) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4637, a4638)?
            };
            nat_add(a4639, a4640)?
        };
        nat_add(a4641, a4642)?
    };
    let a4643 = v2;
    let a4644 = 57u64;
    if nat_le(a4643, a4644) {
        let a4645 = v2;
        let a4646 = 48u64;
        Ok(nat_sub(a4645, a4646))
    } else {
        let a4647 = v2;
        let a4648 = 87u64;
        Ok(nat_sub(a4647, a4648))
    }
}

pub fn f641() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f642(v0: Option<Adt19>) -> Option<u64> {
    let m4649 = v0;
    match m4649 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m4650 = v1;
            match m4650 {
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

pub fn f643(v0: u64, v1: Adt19, v2: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4651 = v0;
    if m4651 == 0 {
        Ok(f549(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4651 - 1;
        let m4652 = v1.clone();
        match m4652 {
            Adt19::C0(v4) => {
                if v4 {
                    Ok(f602())
                } else {
                    Ok(f549(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt19::C1(_, _) => {
                Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C2(_, _, _) => {
                Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C3(_) => {
                Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C4(_) => {
                Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C5(_) => {
                let m4653 = f258(f150(v1.clone(), f8(Str::lit("$ref"))));
                match m4653 {
                    None => {
                        if if f557(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f557(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f557(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f550(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f153(v1.clone(), f8(Str::lit("type")));
                            if if f36(v20.clone(), Bytes::lit(&[])) {
                                !f631(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f602())
                            } else {
                                if f36(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f550(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f36(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f550(f632(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f36(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f550(f500(), List::<Adt25>::nil()))
                                        } else {
                                            if f36(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f550(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f36(v20.clone(), f8(Str::lit("array"))) {
                                                    let m4655 = f150(v1, f8(Str::lit("items")));
                                                    match m4655 {
                                                        None => {
                                                            Ok(f600(f8(Str::lit("List")), f602()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f600(f8(Str::lit("List")), f643(v3, v21, {
                                                                let a4656 = v2;
                                                                let a4657 = f8(Str::lit("Item"));
                                                                append_bytes(a4656, a4657)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f36(v20.clone(), f8(Str::lit("object"))) {
                                                        let m4658 = f633(v1);
                                                        match m4658 {
                                                            None => {
                                                                Ok(f600(f8(Str::lit("List")), f634(f602())))
                                                            }
                                                            Some(v22) => {
                                                                Ok(f600(f8(Str::lit("List")), f634(f643(v3, v22, {
                                                                    let a4659 = v2;
                                                                    let a4660 = f8(Str::lit("Value"));
                                                                    append_bytes(a4659, a4660)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f549(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m4654 = f547(v18)?;
                        match m4654 {
                            None => {
                                Ok(f549(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f550(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt19::C6 => {
                Ok(f602())
            }
            Adt19::C7(_, _) => {
                Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C8(_, _, _) => {
                Ok(f549(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f644(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f429(Fn10::F647, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f645(v0: Fn25, p4665: u64, p4666: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4664 = (p4665, p4666);
    {
        let mut m4664 = m4664;
        loop {
            let step = {
                let (v1, v2) = m4664;
                let m4661 = v1;
                if m4661 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4661 - 1;
                    let m4663 = {
                        let c4662: Fn25 = v0.clone();
                        c4662.apply(v2.clone())
                    };
                    match m4663 {
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
                Ok(next) => { m4664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f646(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f648(v0)
}

pub fn f647(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f648(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4667, _) = v0.clone();
        h4667
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4668, _) = {
            let (_, h4669) = v0.clone();
            h4669
        };
        h4668
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4670) = {
            let (_, h4671) = v0;
            h4671
        };
        h4670
    };
    let m4672 = v1.clone();
    match m4672.uncons() {
        None => {
            let m4673 = v2;
            match m4673.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4674 = v2.clone();
            match m4674.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f637({
                        let (h4675, _) = v8.clone();
                        h4675
                    }, {
                        let (h4676, _) = v6.clone();
                        h4676
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
