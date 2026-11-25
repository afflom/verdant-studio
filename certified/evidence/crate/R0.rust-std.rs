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
    F57(u8, Bytes),
    F186(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F57(k0, k1) => f57(*k0, k1.clone(), p0),
            Fn0::F186(k0) => f186(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F164,
    F182,
    F264,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F164 => f164(p0, p1),
            Fn1::F182 => f182(p0, p1),
            Fn1::F264 => f264(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F218(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F218(k0) => f218(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F254(Bytes, u64, Bytes, u64),
    F305(Bytes),
    F366(Bytes),
    F371(Bytes, Bytes),
    F372(Bytes),
    F373(Bytes, u64),
    F378(u64, Bytes),
    F424(bool, u64, Bytes),
    F465(Bytes),
    F466(Bytes),
    F500(Bytes),
    F542(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F254(k0, k1, k2, k3) => f254(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F305(k0) => f305(k0.clone(), p0),
            Fn3::F366(k0) => Ok(f366(k0.clone(), p0)),
            Fn3::F371(k0, k1) => f371(k0.clone(), k1.clone(), p0),
            Fn3::F372(k0) => f372(k0.clone(), p0),
            Fn3::F373(k0, k1) => f373(k0.clone(), *k1, p0),
            Fn3::F378(k0, k1) => f378(*k0, k1.clone(), p0),
            Fn3::F424(k0, k1, k2) => f424(*k0, *k1, k2.clone(), p0),
            Fn3::F465(k0) => f465(k0.clone(), p0),
            Fn3::F466(k0) => f466(k0.clone(), p0),
            Fn3::F500(k0) => f500(k0.clone(), p0),
            Fn3::F542(k0) => f542(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F297(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn4::F297(k0, k1, k2) => f297(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F304(Bytes),
    F317(u64, Bytes),
    F397(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F304(k0) => f304(k0.clone(), p0),
            Fn5::F317(k0, k1) => Ok(f317(*k0, k1.clone(), p0)),
            Fn5::F397(k0) => f397(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F346,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn6::F346 => f346(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F349,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn7::F349 => f349(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F351,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn8::F351 => f351(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F354,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn9::F354 => f354(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F399,
    F617,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F399 => f399(p0, p1),
            Fn10::F617 => f617(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F402,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F402 => f402(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F405,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F405 => f405(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F408,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F408 => f408(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F413,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn14::F413 => f413(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F426(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F426(k0) => f426(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F461(bool, Bytes),
    F468(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F461(k0, k1) => f461(*k0, k1.clone(), p0),
            Fn16::F468(k0) => f468(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F477,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F477 => f477(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F479,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F479 => f479(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F481,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F481 => f481(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F489,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F489 => f489(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F518,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F518 => f518(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F580,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F580 => f580(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F582,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F582 => f582(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F584(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F584(k0) => f584(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F615,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F615 => f615(p0),
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
                                                if f36(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                    f44(v1.clone(), v4.clone(), f45(v1, v4)?)
                                                } else {
                                                    if f36(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                        f44(v1.clone(), v4.clone(), f46(v1, v4, false)?)
                                                    } else {
                                                        if f36(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                            f44(v1.clone(), v4.clone(), f46(v1, v4, true)?)
                                                        } else {
                                                            if f36(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                                f44(v1.clone(), v4.clone(), f47(v1, v4, false)?)
                                                            } else {
                                                                if f36(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                    f44(v1.clone(), v4.clone(), f47(v1, v4, true)?)
                                                                } else {
                                                                    if f36(v2.clone(), f8(Str::lit("admit"))) {
                                                                        f48(v1, v4)
                                                                    } else {
                                                                        if f36(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                            f44(v1.clone(), v4.clone(), f49(v1, v4)?)
                                                                        } else {
                                                                            if f36(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                                f50(v1, v4)
                                                                            } else {
                                                                                if f36(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                    f51(v1, v4)
                                                                                } else {
                                                                                    if f36(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                        f52(v1, v4)
                                                                                    } else {
                                                                                        if f36(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                            f53(v1, v4)
                                                                                        } else {
                                                                                            if f36(v2, f8(Str::lit("propose-encoding"))) {
                                                                                                let m179 = v5;
                                                                                                match m179.uncons() {
                                                                                                    None => {
                                                                                                        Ok(Adt11::C4(f35()))
                                                                                                    }
                                                                                                    Some((v6, v7)) => {
                                                                                                        f54(v1, v4, v6, v7)
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

pub fn f29(v0: u8) -> R<Str> {
    let m228 = {
        let a226 = f55();
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
        let (h230, _) = f56(Fn0::F57(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f58(List::cons(f32(v0, v4, {
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
                        if f59(v1.clone(), 0u64, {
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
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | read-xml FILE | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f36(v0: Bytes, v1: Bytes) -> bool {
    let a251 = v0;
    let a252 = v1;
    equal(a251, a252)
}

pub fn f37(v0: Adt1) -> R<Adt11> {
    let m253 = f60(v0.clone(), f61());
    match m253.uncons() {
        None => {
            let m254 = f62(v0.clone(), f61())?;
            match m254.uncons() {
                None => {
                    let m257 = f63(match v0.clone() {
                        Adt1::C0(_, h255, _) => {
                            h255
                        }
                    }, f64(f65(), f66(match v0.clone() {
                        Adt1::C0(_, h256, _) => {
                            h256
                        }
                    }, false)));
                    match m257.uncons() {
                        None => {
                            let m259 = f60(v0.clone(), f66(match v0.clone() {
                                Adt1::C0(_, h258, _) => {
                                    h258
                                }
                            }, true));
                            match m259.uncons() {
                                None => {
                                    let m261 = f67(v0.clone(), f66(match v0.clone() {
                                        Adt1::C0(_, h260, _) => {
                                            h260
                                        }
                                    }, true))?;
                                    match m261 {
                                        None => {
                                            let v10: (bool, Bytes) = f68(Adt13::C0(Adt12::C0(f69(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f71(v0.clone())?, f72(f73(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f69(f70(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f72(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f73(v0.clone(), f8(Str::lit("model/strata.json")))?, f74(f73(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f66(match v0 {
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
    if f75(f76(match v0.clone() {
        Adt1::C0(_, h266, _) => {
            h266
        }
    }, f77())) {
        Ok(Adt11::C1(List::cons(f77(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f78(f79(f77(), f80(match v0.clone() {
            Adt1::C0(_, h267, _) => {
                h267
            }
        }, f77())))?;
        let m268 = f60(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m268.uncons() {
            None => {
                let m269 = f81(v0.clone(), f82(f83(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f84(v2, v1.clone()))?;
                match m269 {
                    None => {
                        f85(v0, v1)
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
    if f75(f86(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h270) => {
            h270
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f87(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f75(f86(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h271) => {
                h271
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f87(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f40(v0: Adt1) -> R<Adt11> {
    let m272 = f88(v0.clone());
    match m272.uncons() {
        None => {
            let m273 = f64(f60(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f89(), List::<Bytes>::nil())))), f90(v0.clone())?);
            match m273.uncons() {
                None => {
                    Ok(Adt11::C3(f91(f92(v0.clone())?, List::cons((f93(), f94()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f95()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f96()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f97()), List::cons((f8(Str::lit("docs/catalog.md")), f98(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f99(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f100(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f101()), List::<(Bytes, Bytes)>::nil()))))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m274 = f88(v0.clone());
    match m274.uncons() {
        None => {
            let m275 = f64(f60(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f90(v0.clone())?);
            match m275.uncons() {
                None => {
                    let v5: Adt15 = f102(v0.clone())?;
                    let v6: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f64(f103(f104(match v5.clone() {
                        Adt15::C0(h276, _, _) => {
                            h276
                        }
                    }, f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f106(v6.clone())), f107(f104(match v5.clone() {
                        Adt15::C0(h277, _, _) => {
                            h277
                        }
                    }, f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f108(v5.clone(), v6, f109(v5, f110(v0.clone(), f111(v0)?)?))?)))))
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
    let m278 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m278.uncons() {
        None => {
            let v4: Bytes = f70(v0.clone(), v1);
            let v5: List<List<Bytes>> = f112(v4.clone())?;
            if f75(f113(f114(v5.clone()), f115())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f116(Str::lit("id"), v6.clone())?, List::cons(f116(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f116(Str::lit("artifact"), f117())?, List::cons(f116(Str::lit("checksum"), f118(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f117(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f119(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f120(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f121(f122(f123(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m279 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m279.uncons() {
        None => {
            let m280 = f124(f70(v0, v1))?;
            match m280 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f125(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f126(f127(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f44(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m281 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m281.uncons() {
        None => {
            if f128(f83(f70(v0, v1))?)? {
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

pub fn f45(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m282 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m282.uncons() {
        None => {
            let m283 = f129(f83(f70(v0, v1))?)?;
            match m283 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f121(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f46(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m284 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m284.uncons() {
        None => {
            let v5: Bytes = f130(if v2 {
                f131()
            } else {
                f132()
            }, f133(v1.clone())?);
            if f75(f134(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m285 = f135(f83(f70(v0.clone(), v1.clone()))?, v2)?;
                match m285 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a287 = {
                            let a286 = v6;
                            length_list(a286)
                        };
                        let a288 = 0u64;
                        if nat_eq(a287, a288) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f70(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f47(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m289 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m289.uncons() {
        None => {
            let v5: List<Bytes> = f83(f70(v0.clone(), v1.clone()))?;
            if f75(f134(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f136(v5)?
                } else {
                    f137(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f130(f138(), f133(v1.clone())?), f70(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f130(f138(), f133(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f139()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f48(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m290 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m290.uncons() {
        None => {
            let v4: List<Bytes> = f83(f70(v0.clone(), v1))?;
            let m291 = f140(v4.clone(), f8(Str::lit("requirement")))?;
            match m291 {
                None => {
                    Ok(Adt11::C4(f139()))
                }
                Some(v5) => {
                    let m292 = f140(v4, f8(Str::lit("admission")))?;
                    match m292 {
                        None => {
                            Ok(Adt11::C4(f139()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f64(f72(v5.clone(), f8(Str::lit("id"))), f72(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f78(f64(f72(v7.clone(), f8(Str::lit("id"))), f72(v8.clone(), f8(Str::lit("construct")))))?;
                            let m293 = f141(f106(v6.clone()), f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m293 {
                                None => {
                                    let a295 = {
                                        let a294 = v9.clone();
                                        length_list(a294)
                                    };
                                    let a296 = 0u64;
                                    if nat_eq(a295, a296) {
                                        Ok(Adt11::C4(f139()))
                                    } else {
                                        if f75({
                                            let a299 = {
                                                let a297 = f78(f64(v9.clone(), v10.clone()))?;
                                                length_list(a297)
                                            };
                                            let a300 = {
                                                let a298 = f64(v9.clone(), v10);
                                                length_list(a298)
                                            };
                                            nat_eq(a299, a300)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f143(f8(Str::lit("model/requirements.json")), f144(v7, v5))?, List::cons(f143(f8(Str::lit("model/admissions.json")), f144(v8, v6))?, List::cons(f120(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f121(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f142(v11))
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

pub fn f49(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m301 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m301.uncons() {
        None => {
            let m302 = f145(f83(f70(v0, v1))?)?;
            match m302 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f121(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m303 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m303.uncons() {
        None => {
            let m304 = f146(v0.clone(), v1.clone())?;
            match m304 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m305 = f147(v4.clone(), f8(Str::lit("schema")));
                    match m305 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f148(v0, f149(v5.clone())?, v5, f150(v4.clone(), f8(Str::lit("name"))), f151(f152(f147(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f51(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m306 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f89(), List::<Bytes>::nil())));
    match m306.uncons() {
        None => {
            let m307 = f153(f83(f70(v0.clone(), v1))?)?;
            match m307 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f73(v0, f89())?;
                    let v6: List<Bytes> = f72(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f78(f72(v5.clone(), f8(Str::lit("at"))))?;
                    let a309 = {
                        let a308 = v6.clone();
                        length_list(a308)
                    };
                    let a310 = 0u64;
                    if if nat_eq(a309, a310) {
                        true
                    } else {
                        f75({
                            let a313 = {
                                let a311 = f78(f64(v6.clone(), v7.clone()))?;
                                length_list(a311)
                            };
                            let a314 = {
                                let a312 = f64(v6.clone(), v7);
                                length_list(a312)
                            };
                            nat_eq(a313, a314)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f143(f89(), f144(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f121(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m315 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m315.uncons() {
        None => {
            let m316 = f154(v0, v1)?;
            match m316 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f155(f156(Some(v4))) {
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

pub fn f53(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m317 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m317.uncons() {
        None => {
            let m318 = f154(v0.clone(), v1.clone())?;
            match m318 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f75(f36(f157(v4.clone(), f8(Str::lit("schema"))), f158())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f157(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m319 = f159(v4.clone(), f8(Str::lit("source")));
                            match m319 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f160(f159(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f130(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f130(v9.clone(), f8(Str::lit("binding.json"))), f70(v0.clone(), v1));
                        let m320 = v8;
                        match m320 {
                            None => {
                                f161(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f75(f36(f157(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f161(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f130(v9, f133(f157(v7.clone(), f8(Str::lit("url"))))?);
                                    if f75(f86(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h321) => {
                                            h321
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f157(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m322 = f60(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m322.uncons() {
                                            None => {
                                                if f36(f118(f70(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f161(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f54(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m323 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m323.uncons() {
        None => {
            let m324 = f146(v0.clone(), v1.clone())?;
            match m324 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f148(v0.clone(), f70(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f55() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f56(v0: Fn0, p329: u64, p330: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m328 = (p329, p330);
    {
        let mut m328 = m328;
        loop {
            let step = {
                let (v1, v2) = m328;
                let m325 = v1;
                if m325 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m325 - 1;
                    let m327 = {
                        let c326: Fn0 = v0.clone();
                        c326.apply(v2.clone())?
                    };
                    match m327 {
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
                Ok(next) => { m328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f162(v1, v0, v2)
}

pub fn f58(v0: List<Bytes>) -> List<Bytes> {
    f163(Fn1::F164, List::<Bytes>::nil(), v0)
}

pub fn f59(v0: Bytes, p386: u64, p387: u64) -> R<bool> {
    let m385 = (p386, p387);
    {
        let mut m385 = m385;
        loop {
            let step = {
                let (v1, v2) = m385;
                let m331 = v2;
                if m331 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m331 - 1;
                    let m334 = {
                        let a332 = v0.clone();
                        let a333 = v1;
                        index_bytes(a332, a333)
                    };
                    match m334 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a381 = {
                                let v5: u8 = v4;
                                let a379 = {
                                    let a355 = {
                                        let a343 = {
                                            let a337 = {
                                                let a335 = v5;
                                                let a336 = 128u8;
                                                fixed_u8::bit_and(a335, a336)
                                            };
                                            let a338 = 128u8;
                                            if equal(a337, a338) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a344 = {
                                            let a341 = {
                                                let a339 = v5;
                                                let a340 = 64u8;
                                                fixed_u8::bit_and(a339, a340)
                                            };
                                            let a342 = 64u8;
                                            if equal(a341, a342) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a343, a344)?
                                    };
                                    let a356 = {
                                        let a353 = {
                                            let a347 = {
                                                let a345 = v5;
                                                let a346 = 32u8;
                                                fixed_u8::bit_and(a345, a346)
                                            };
                                            let a348 = 32u8;
                                            if equal(a347, a348) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a354 = {
                                            let a351 = {
                                                let a349 = v5;
                                                let a350 = 16u8;
                                                fixed_u8::bit_and(a349, a350)
                                            };
                                            let a352 = 16u8;
                                            if equal(a351, a352) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a353, a354)?
                                    };
                                    nat_add(a355, a356)?
                                };
                                let a380 = {
                                    let a377 = {
                                        let a365 = {
                                            let a359 = {
                                                let a357 = v5;
                                                let a358 = 8u8;
                                                fixed_u8::bit_and(a357, a358)
                                            };
                                            let a360 = 8u8;
                                            if equal(a359, a360) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a366 = {
                                            let a363 = {
                                                let a361 = v5;
                                                let a362 = 4u8;
                                                fixed_u8::bit_and(a361, a362)
                                            };
                                            let a364 = 4u8;
                                            if equal(a363, a364) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a365, a366)?
                                    };
                                    let a378 = {
                                        let a375 = {
                                            let a369 = {
                                                let a367 = v5;
                                                let a368 = 2u8;
                                                fixed_u8::bit_and(a367, a368)
                                            };
                                            let a370 = 2u8;
                                            if equal(a369, a370) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a376 = {
                                            let a373 = {
                                                let a371 = v5;
                                                let a372 = 1u8;
                                                fixed_u8::bit_and(a371, a372)
                                            };
                                            let a374 = 1u8;
                                            if equal(a373, a374) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a375, a376)?
                                    };
                                    nat_add(a377, a378)?
                                };
                                nat_add(a379, a380)?
                            };
                            let a382 = 127u64;
                            if nat_le(a381, a382) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a383 = v1;
                                    let a384 = 1u64;
                                    nat_add(a383, a384)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
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

pub fn f60(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m388 = v1;
    match m388.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m390 = f165(match v0.clone() {
                Adt1::C0(h389, _, _) => {
                    h389
                }
            }, v2.clone());
            match m390 {
                None => {
                    List::cons(v2, f60(v0, v3))
                }
                Some(_) => {
                    f60(v0, v3)
                }
            }
        }
    }
}

pub fn f61() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f117(), List::<Bytes>::nil())))))))
}

pub fn f62(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m391 = v1;
    match m391.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f166(v0.clone(), v2.clone()) {
                f134(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m392 = f154(v0.clone(), v2.clone())?;
                match m392 {
                    None => {
                        Ok(List::cons(v2, f62(v0, v3)?))
                    }
                    Some(_) => {
                        f62(v0, v3)
                    }
                }
            } else {
                f62(v0, v3)
            }
        }
    }
}

pub fn f63(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m393 = v1;
    match m393.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f76(v0.clone(), v2.clone()) {
                f63(v0, v3)
            } else {
                List::cons(v2, f63(v0, v3))
            }
        }
    }
}

pub fn f64(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m394 = v0;
    match m394.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f64(v3, v1))
        }
    }
}

pub fn f65() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f66(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m395 = v0;
    match m395.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f64(f167({
                let (h396, _) = v2;
                h396
            }, {
                let (_, h397) = v2;
                h397
            }, v1), f66(v3, v1))
        }
    }
}

pub fn f67(v0: Adt1, p401: List<Bytes>) -> R<Option<Bytes>> {
    let m400 = p401;
    {
        let mut m400 = m400;
        loop {
            let step = {
                let v1 = m400;
                let m398 = v1;
                match m398.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m399 = f168(f70(v0.clone(), v2.clone()), f169())?;
                        match m399 {
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
                Ok(next) => { m400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h402, _, _, _, _) => {
            h402
        }
    };
    let v2: Adt16 = f170(v1.clone())?;
    let v3: List<Bytes> = f171(v1.clone());
    let v4: List<Bytes> = f72(match v0.clone() {
        Adt13::C0(_, _, h403, _, _) => {
            h403
        }
    }, f8(Str::lit("path")));
    let m405 = f172(v2, List::<u64>::nil(), f173(v1.clone(), match v0.clone() {
        Adt13::C0(_, h404, _, _, _) => {
            h404
        }
    }, 0u64)?);
    match m405 {
        None => {
            if f175(f176(match v0.clone() {
                Adt13::C0(_, _, _, _, h407) => {
                    h407
                }
            }, f64(match v0.clone() {
                Adt13::C0(_, _, _, _, h408) => {
                    h408
                }
            }, v4.clone()))?, f176(v4.clone(), f64(match v0.clone() {
                Adt13::C0(_, _, _, _, h409) => {
                    h409
                }
            }, v4.clone()))?, f177(match v0.clone() {
                Adt13::C0(_, _, h410, _, _) => {
                    h410
                }
            })) {
                let m414 = f178(f176(match v0.clone() {
                    Adt13::C0(_, _, _, h411, _) => {
                        h411
                    }
                }, f64(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h412, _) => {
                        h412
                    }
                }))?, f176(v3.clone(), f64(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h413, _) => {
                        h413
                    }
                }))?);
                match m414 {
                    None => {
                        let m417 = f179(f72(match v0 {
                            Adt13::C0(_, h416, _, _, _) => {
                                h416
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m417 {
                            None => {
                                let m418 = f179(v4, List::<Bytes>::nil());
                                match m418 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f121(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f174(f64(v3, match v0 {
                            Adt13::C0(_, _, _, h415, _) => {
                                h415
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f174(match v1 {
                Adt12::C0(_, _, _, _, h406) => {
                    h406
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f69(v0: Bytes) -> R<List<Bytes>> {
    f180(f83(v0)?, f8(Str::lit("id = \"")))
}

pub fn f70(v0: Adt1, v1: Bytes) -> Bytes {
    let m420 = f165(match v0 {
        Adt1::C0(h419, _, _) => {
            h419
        }
    }, v1);
    match m420 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m421 = v2;
            match m421 {
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

pub fn f71(v0: Adt1) -> R<List<Bytes>> {
    let m422 = f112(f70(v0, f117()))?;
    match m422.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f122(v2))
        }
    }
}

pub fn f72(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m423 = v0;
    match m423.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f157(v2, v1.clone()), f72(v3, v1))
        }
    }
}

pub fn f73(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f156(f154(v0, v1)?))
}

pub fn f74(v0: List<Adt14>) -> List<Bytes> {
    let m424 = v0;
    match m424.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m425 = f160(Some(v1));
            match m425 {
                None => {
                    f74(v2)
                }
                Some(v3) => {
                    List::cons(v3, f74(v2))
                }
            }
        }
    }
}

pub fn f75(v0: bool) -> bool {
    !v0
}

pub fn f76(p429: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m428 = p429;
    {
        let mut m428 = m428;
        loop {
            let step = {
                let v0 = m428;
                let m426 = v0;
                match m426.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h427, _) = v2;
                            h427
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
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

pub fn f77() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f78(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f181(f163(Fn1::F182, List::<Bytes>::nil(), f183(v0)?)))
}

pub fn f79(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m430 = v1;
    match m430.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f134(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f130(v0.clone(), v2), f79(v0, v3))
            } else {
                f79(v0, v3)
            }
        }
    }
}

pub fn f80(p435: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m434 = p435;
    {
        let mut m434 = m434;
        loop {
            let step = {
                let v0 = m434;
                let m431 = v0;
                match m431.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h432, _) = v2;
                            h432
                        }, v1.clone()) {
                            let (_, h433) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h433)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Adt1, v1: List<List<Bytes>>, p438: List<Bytes>) -> R<Option<Bytes>> {
    let m437 = p438;
    {
        let mut m437 = m437;
        loop {
            let step = {
                let v2 = m437;
                let m436 = v2;
                match m436.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f184(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(p441: List<Bytes>, p442: List<Bytes>, p443: List<List<Bytes>>) -> List<List<Bytes>> {
    let m440 = (p441, (p442, p443));
    {
        let mut m440 = m440;
        loop {
            let step = {
                let (v0, (v1, v2)) = m440;
                let m439 = v0;
                match m439.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f185(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f185(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a444 = v0.clone();
        length_bytes(a444)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h445, _) = f56(Fn0::F186(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h445
    };
    let v3: u64 = {
        let (h446, _) = {
            let (_, h447) = v2.clone();
            h447
        };
        h446
    };
    let v4: List<Bytes> = {
        let (_, h448) = {
            let (_, h449) = v2;
            h449
        };
        h448
    };
    let a450 = v3;
    let a451 = v1;
    if nat_lt(a450, a451) {
        Ok(f58(List::cons(f32(v0, v3, {
            let a452 = v1;
            let a453 = v3;
            nat_sub(a452, a453)
        }), v4)))
    } else {
        Ok(f58(v4))
    }
}

pub fn f84(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m454 = v0;
    match m454.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f84(v3, v1)
            } else {
                List::cons(v2, f84(v3, v1))
            }
        }
    }
}

pub fn f85(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f75(f76(match v0.clone() {
        Adt1::C0(_, h455, _) => {
            h455
        }
    }, f187())) {
        Ok(Adt11::C1(List::cons(f187(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f84(f78(f79(f187(), f80(match v0.clone() {
            Adt1::C0(_, h456, _) => {
                h456
            }
        }, f187())))?, v1.clone());
        let m457 = f60(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m457.uncons() {
            None => {
                let v5: List<Bytes> = f72(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m458 = f60(v0.clone(), f188(v0.clone(), v2.clone(), v5.clone())?);
                match m458.uncons() {
                    None => {
                        let m459 = f189(v0.clone(), v2, v5)?;
                        match m459 {
                            None => {
                                f190(v0, v1)
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

pub fn f86(v0: Bytes, p462: List<Bytes>) -> bool {
    let m461 = p462;
    {
        let mut m461 = m461;
        loop {
            let step = {
                let v1 = m461;
                let m460 = v1;
                match m460.uncons() {
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
                Ok(next) => { m461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f88(v0: Adt1) -> List<Bytes> {
    if f76(match v0.clone() {
        Adt1::C0(_, h463, _) => {
            h463
        }
    }, f132()) {
        if f76(match v0 {
            Adt1::C0(_, h464, _) => {
                h464
            }
        }, f131()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f131(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f132(), List::<Bytes>::nil())
    }
}

pub fn f89() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f90(v0: Adt1) -> R<List<Bytes>> {
    Ok(f60(v0.clone(), List::cons(f117(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f111(v0)?)))))
}

pub fn f91(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m465 = v0;
    match m465.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f91(v3, v1))
        }
    }
}

pub fn f92(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m466 = f73(v0.clone(), f89())?;
    match m466.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f191(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f93() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f94() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f95() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f192(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f96() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught when it opens, too.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  pull_request:\u{a}    types: [opened]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}              { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f192(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f97() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f98(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f102(v0.clone())?;
    let v2: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f109(v1.clone(), f110(v0.clone(), f111(v0.clone())?)?);
    let v4: List<List<Bytes>> = f123(f112(f70(v0, f117()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f193(v4.clone(), f122(v4), f194(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f108(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f99(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f195(f102(v0.clone())?, f110(v0.clone(), f111(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f100(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f196(f73(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f197(f73(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f101() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f102(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f71(v0.clone())?, f69(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f72(f73(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f103(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m467 = v0;
    match m467.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f103(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f103(v3, v1))
            }
        }
    }
}

pub fn f104(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m468 = v0;
    match m468.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f104(v3, v1)
            } else {
                List::cons(v2, f104(v3, v1))
            }
        }
    }
}

pub fn f105(v0: List<Adt14>) -> List<Bytes> {
    let m469 = v0;
    match m469.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f64(f74(f156(f159(v1, f8(Str::lit("defers"))))), f105(v2))
        }
    }
}

pub fn f106(v0: List<Adt14>) -> List<Bytes> {
    let m470 = v0;
    match m470.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m471 = f159(v1, f8(Str::lit("std")));
            match m471 {
                None => {
                    f106(v2)
                }
                Some(v3) => {
                    List::cons(f157(v3, f8(Str::lit("row"))), f106(v2))
                }
            }
        }
    }
}

pub fn f107(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m472 = v0;
    match m472.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f107(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f107(v3, v1))
            }
        }
    }
}

pub fn f108(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f91(f198(v2.clone()), f199(f194(v0, v1.clone(), v2)?));
    f200(f78(f201(v1))?, v3.clone(), {
        let a474 = {
            let a473 = v3;
            length_list(a473)
        };
        let a475 = 1u64;
        nat_add(a474, a475)?
    })
}

pub fn f109(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m476 = v1;
    match m476.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f202(v2.clone(), f8(Str::lit("subject"))) {
                f203(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f109(v0, v3))
            } else {
                f109(v0, v3)
            }
        }
    }
}

pub fn f110(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m477 = v1;
    match m477.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m478 = f135(f83(f70(v0.clone(), v2.clone()))?, f204(v2, f131()))?;
            match m478 {
                None => {
                    f110(v0, v3)
                }
                Some(v4) => {
                    Ok(f144(v4, f110(v0, v3)?))
                }
            }
        }
    }
}

pub fn f111(v0: Adt1) -> R<List<Bytes>> {
    Ok(f64(f78(f205(f132(), f80(match v0.clone() {
        Adt1::C0(_, h479, _) => {
            h479
        }
    }, f132())))?, f78(f205(f131(), f80(match v0 {
        Adt1::C0(_, h480, _) => {
            h480
        }
    }, f131())))?))
}

pub fn f112(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f206(v0.clone(), 0u64, {
        let a481 = v0;
        length_bytes(a481)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a484 = {
            let a483 = match v1.clone() {
                Adt17::C0(_, _, h482, _, _) => {
                    h482
                }
            };
            length_bytes(a483)
        };
        let a485 = 0u64;
        if if nat_eq(a484, a485) {
            let m487 = match v1.clone() {
                Adt17::C0(_, _, _, h486, _) => {
                    h486
                }
            };
            match m487.uncons() {
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
            f207(v1)
        }
    };
    Ok(f208(match v4 {
        Adt17::C0(_, _, _, _, h488) => {
            h488
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f113(p493: List<Bytes>, p494: List<Bytes>) -> bool {
    let m492 = (p493, p494);
    {
        let mut m492 = m492;
        loop {
            let step = {
                let (v0, v1) = m492;
                let m489 = v0;
                match m489.uncons() {
                    None => {
                        let m490 = v1;
                        match m490.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m491 = v1;
                        match m491.uncons() {
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
                Ok(next) => { m492 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<List<Bytes>>) -> List<Bytes> {
    let m495 = v0;
    match m495.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f115() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f116(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f209(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f117() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f118(v0: Bytes) -> R<Bytes> {
    Ok(f210(f211(v0)?))
}

pub fn f119(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f212(f82(f83(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f213(f214(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f120(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f215(Adt14::C3(std::rc::Rc::new(f216(f78(f64(v1, f74(f73(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f121(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m496 = f217(Fn2::F218(v1), None::<Bytes>, v0);
    match m496 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f122(v0: List<List<Bytes>>) -> List<Bytes> {
    let m497 = v0;
    match m497.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m498 = v1;
            match m498.uncons() {
                None => {
                    f122(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f122(v2))
                }
            }
        }
    }
}

pub fn f123(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m499 = v0;
    match m499.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f124(v0: Bytes) -> R<Result<Adt18, Bytes>> {
    let v1: Bytes = if f219(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
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
    let v2: Bytes = f220(v1)?;
    let v3: u64 = {
        let a503 = v2.clone();
        length_bytes(a503)
    };
    let v4: u64 = f221(v2.clone())?;
    let a504 = v4;
    let a505 = v3;
    if nat_lt(a504, a505) {
        Ok(Err::<Adt18, Bytes>(f222(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m506 = f223(v2.clone())?;
        match m506 {
            Ok(v6) => {
                let m509 = f224(v2.clone(), v6, {
                    let a507 = v3;
                    let a508 = 2u64;
                    nat_add(a507, a508)?
                })?;
                match m509 {
                    Ok(v8) => {
                        if f225(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(Err::<Adt18, Bytes>(f222(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a510 = f226(v2.clone(), v8)?;
                            let a511 = 60u64;
                            if !nat_eq(a510, a511) {
                                Ok(Err::<Adt18, Bytes>(f222(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            } else {
                                let m514 = f227(v2.clone(), v8, {
                                    let a512 = v3;
                                    let a513 = 2u64;
                                    nat_add(a512, a513)?
                                }, 0u64)?;
                                match m514 {
                                    Ok(v10) => {
                                        let m518 = f224(v2, {
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
                                                    Ok(Ok::<Adt18, Bytes>({
                                                        let (h521, _) = v10;
                                                        h521
                                                    }))
                                                } else {
                                                    Ok(Err::<Adt18, Bytes>(f222(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
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

pub fn f125(v0: Adt18) -> Bytes {
    let m522 = v0;
    match m522 {
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

pub fn f126(v0: u64) -> R<Bytes> {
    f228(f229(), 10u64, v0)
}

pub fn f127(v0: Adt18) -> R<u64> {
    let m523 = v0;
    match m523 {
        Adt18::C0(_, _, r528) => {
            let v3 = (*r528).clone();
            let a524 = 1u64;
            let a525 = f127(v3)?;
            nat_add(a524, a525)
        }
        Adt18::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt18::C2(_) => {
            Ok(0u64)
        }
        Adt18::C3(r529, r530) => {
            let v8 = (*r529).clone();
            let v9 = (*r530).clone();
            let a526 = f127(v8)?;
            let a527 = f127(v9)?;
            nat_add(a526, a527)
        }
        Adt18::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f128(p536: List<Bytes>) -> R<bool> {
    let m535 = p536;
    {
        let mut m535 = m535;
        loop {
            let step = {
                let v0 = m535;
                let m531 = v0;
                match m531.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a533 = {
                            let a532 = f230(v1)?;
                            length_bytes(a532)
                        };
                        let a534 = 0u64;
                        if nat_eq(a533, a534) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m537 = v0;
    match m537.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f230(v1)?;
            let a539 = {
                let a538 = v3.clone();
                length_bytes(a538)
            };
            let a540 = 0u64;
            if nat_eq(a539, a540) {
                f129(v2)
            } else {
                let m541 = f231(v3.clone())?;
                match m541 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f157(v4.clone(), f8(Str::lit("id")));
                        if if f36(f157(v4, f8(Str::lit("schema"))), f158()) {
                            f75({
                                let a543 = {
                                    let a542 = v5.clone();
                                    length_bytes(a542)
                                };
                                let a544 = 0u64;
                                nat_eq(a543, a544)
                            })
                        } else {
                            false
                        } {
                            let m545 = f129(v2)?;
                            match m545 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f232(v5), {
                                        let a546 = v3;
                                        let a547 = Bytes::lit(&[0x0a]);
                                        append_bytes(a546, a547)
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

pub fn f130(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f131() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f132() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f133(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f134(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a548 = v0.clone();
        length_bytes(a548)
    };
    let v3: u64 = {
        let a549 = v1.clone();
        length_bytes(a549)
    };
    let a550 = v3;
    let a551 = v2;
    if nat_le(a550, a551) {
        let m557 = {
            let a554 = v0;
            let a555 = {
                let a552 = v2;
                let a553 = v3;
                nat_sub(a552, a553)
            };
            let a556 = v3;
            slice_bytes(a554, a555, a556)
        };
        match m557 {
            None => {
                false
            }
            Some(v4) => {
                let a558 = v4;
                let a559 = v1;
                equal(a558, a559)
            }
        }
    } else {
        false
    }
}

pub fn f135(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m560 = v0;
    match m560.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f230(v2)?;
            let a562 = {
                let a561 = v4.clone();
                length_bytes(a561)
            };
            let a563 = 0u64;
            if nat_eq(a562, a563) {
                f135(v3, v1)
            } else {
                let m564 = f231(v4)?;
                match m564 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f233(v5.clone())
                        } else {
                            f234(v5.clone())
                        } {
                            let m565 = f135(v3, v1)?;
                            match m565 {
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

pub fn f136(v0: List<Bytes>) -> R<bool> {
    if f235(f153(v0.clone())?) {
        Ok(f75({
            let a567 = {
                let a566 = f236(f153(v0)?);
                length_list(a566)
            };
            let a568 = 0u64;
            nat_eq(a567, a568)
        }))
    } else {
        Ok(false)
    }
}

pub fn f137(v0: List<Bytes>) -> R<bool> {
    if f235(f140(v0.clone(), f8(Str::lit("requirement")))?) {
        if f235(f140(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f75({
                let a570 = {
                    let a569 = f144(f236(f140(v0.clone(), f8(Str::lit("requirement")))?), f236(f140(v0, f8(Str::lit("admission")))?));
                    length_list(a569)
                };
                let a571 = 0u64;
                nat_eq(a570, a571)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f138() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f139() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f140(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m572 = v0;
    match m572.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f230(v2)?;
            let a574 = {
                let a573 = v4.clone();
                length_bytes(a573)
            };
            let a575 = 0u64;
            if nat_eq(a574, a575) {
                f140(v3, v1)
            } else {
                let m576 = f231(v4)?;
                match m576 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f237(v5.clone());
                        let a578 = {
                            let a577 = v6.clone();
                            length_bytes(a577)
                        };
                        let a579 = 0u64;
                        if nat_eq(a578, a579) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m580 = f140(v3, v1.clone())?;
                            match m580 {
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

pub fn f141(p583: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m582 = p583;
    {
        let mut m582 = m582;
        loop {
            let step = {
                let v0 = m582;
                let m581 = v0;
                match m581.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f86(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f143(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f215(Adt14::C3(std::rc::Rc::new(f238(v1))))?))
}

pub fn f144(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m584 = v0;
    match m584.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f144(v3, v1))
        }
    }
}

pub fn f145(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m585 = v0;
    match m585.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f230(v1)?;
            let a587 = {
                let a586 = v3.clone();
                length_bytes(a586)
            };
            let a588 = 0u64;
            if nat_eq(a587, a588) {
                f145(v2)
            } else {
                let m589 = f239(v3.clone())?;
                match m589 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f240(v4.clone()) {
                            let m590 = f145(v2)?;
                            match m590 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f187(), List::cons(f8(Str::lit("/")), List::cons(f241(f150(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a591 = v3;
                                        let a592 = Bytes::lit(&[0x0a]);
                                        append_bytes(a591, a592)
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

pub fn f146(v0: Adt1, v1: Bytes) -> R<Option<Adt19>> {
    f239(f70(v0, v1))
}

pub fn f147(p598: Adt19, v1: Bytes) -> Option<Adt19> {
    let m597 = p598;
    {
        let mut m597 = m597;
        loop {
            let step = {
                let v0 = m597;
                let m593 = v0;
                match m593 {
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
                    Adt19::C5(r594) => {
                        let v5 = (*r594).clone();
                        Ok::<Adt19, Option<Adt19>>(v5)
                    }
                    Adt19::C6 => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C7(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C8(v2, r595, r596) => {
                        let v3 = (*r595).clone();
                        let v4 = (*r596).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt19, Option<Adt19>>(Some(v3))
                        } else {
                            Ok::<Adt19, Option<Adt19>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m599 = f60(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m599.uncons() {
        None => {
            let m601 = f141(f106(List::cons(Adt14::C4(std::rc::Rc::new({
                let m600 = f242(v4.clone());
                match m600 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m601 {
                None => {
                    f243(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f142(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f149(v0: Adt19) -> R<Bytes> {
    let a602 = f244(v0)?;
    let a603 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a602, a603))
}

pub fn f150(v0: Adt19, v1: Bytes) -> Bytes {
    let m604 = f245(f147(v0, v1));
    match m604 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f151(v0: List<Adt19>) -> List<Bytes> {
    let m605 = v0;
    match m605.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m606 = f245(Some(v1));
            match m606 {
                None => {
                    f151(v2)
                }
                Some(v3) => {
                    List::cons(v3, f151(v2))
                }
            }
        }
    }
}

pub fn f152(v0: Option<Adt19>) -> List<Adt19> {
    let m607 = v0;
    match m607 {
        None => {
            List::<Adt19>::nil()
        }
        Some(v1) => {
            f246(v1)
        }
    }
}

pub fn f153(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m608 = v0;
    match m608.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f230(v1)?;
            let a610 = {
                let a609 = v3.clone();
                length_bytes(a609)
            };
            let a611 = 0u64;
            if nat_eq(a610, a611) {
                f153(v2)
            } else {
                let m612 = f231(v3)?;
                match m612 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f202(v4.clone(), f8(Str::lit("agent"))) {
                            if f202(v4.clone(), f8(Str::lit("at"))) {
                                if f202(v4.clone(), f8(Str::lit("quote"))) {
                                    f247(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m613 = f153(v2)?;
                            match m613 {
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

pub fn f154(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f231(f70(v0, v1))
}

pub fn f155(p616: List<Adt14>) -> bool {
    let m615 = p616;
    {
        let mut m615 = m615;
        loop {
            let step = {
                let v0 = m615;
                let m614 = v0;
                match m614.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f157(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f86(f8(Str::lit("studio")), f72(f248(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m615 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Option<Adt14>) -> List<Adt14> {
    let m617 = v0;
    match m617 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f249(v1)
        }
    }
}

pub fn f157(v0: Adt14, v1: Bytes) -> Bytes {
    let m618 = f160(f159(v0, v1));
    match m618 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f158() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f159(p624: Adt14, v1: Bytes) -> Option<Adt14> {
    let m623 = p624;
    {
        let mut m623 = m623;
        loop {
            let step = {
                let v0 = m623;
                let m619 = v0;
                match m619 {
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
                    Adt14::C4(r620) => {
                        let v5 = (*r620).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r621, r622) => {
                        let v3 = (*r621).clone();
                        let v4 = (*r622).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
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

pub fn f160(v0: Option<Adt14>) -> Option<Bytes> {
    let m625 = v0;
    match m625 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m626 = v1;
            match m626 {
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

pub fn f161(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f119(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f250(v1, v4)?)?), List::cons(f120(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f162(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h627, _) = v2.clone();
        h627
    };
    let v4: u64 = {
        let (h628, _) = {
            let (_, h629) = v2.clone();
            h629
        };
        h628
    };
    let v5: List<Bytes> = {
        let (_, h630) = {
            let (_, h631) = v2;
            h631
        };
        h630
    };
    let m632 = f251(v0.clone(), v3);
    match m632 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a633 = v6;
            let a634 = v1;
            if equal(a633, a634) {
                Ok(Some(({
                    let a635 = v3;
                    let a636 = 1u64;
                    nat_add(a635, a636)?
                }, ({
                    let a637 = v3;
                    let a638 = 1u64;
                    nat_add(a637, a638)?
                }, List::cons(f32(v0, v4, {
                    let a639 = v3;
                    let a640 = v4;
                    nat_sub(a639, a640)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a641 = v3;
                    let a642 = 1u64;
                    nat_add(a641, a642)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f163(v0: Fn1, p646: List<Bytes>, p647: List<Bytes>) -> List<Bytes> {
    let m645 = (p646, p647);
    {
        let mut m645 = m645;
        loop {
            let step = {
                let (v1, v2) = m645;
                let m643 = v2;
                match m643.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c644: Fn1 = v0.clone();
                            c644.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m645 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f165(p652: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m651 = p652;
    {
        let mut m651 = m651;
        loop {
            let step = {
                let v0 = m651;
                let m648 = v0;
                match m648.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h649, _) = v2;
                            h649
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h650) = v2;
                                h650
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f166(v0: Adt1, v1: Bytes) -> bool {
    let m654 = f165(match v0 {
        Adt1::C0(h653, _, _) => {
            h653
        }
    }, v1);
    match m654 {
        None => {
            false
        }
        Some(v2) => {
            let m655 = v2;
            match m655 {
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

pub fn f167(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m656 = v1;
    match m656.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f252(v3.clone())
            } else {
                f75(f252(v3.clone()))
            } {
                List::cons(f130(v0.clone(), v3), f167(v0, v4, v2))
            } else {
                f167(v0, v4, v2)
            }
        }
    }
}

pub fn f168(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a657 = v0.clone();
        length_bytes(a657)
    };
    let v3: u64 = {
        let a658 = v1.clone();
        length_bytes(a658)
    };
    let v4: u64 = {
        let (h659, _) = f253(Fn3::F254(v1.clone(), v3, v0.clone(), v2), {
            let a660 = v2;
            let a661 = 1u64;
            nat_add(a660, a661)?
        }, 0u64)?;
        h659
    };
    if f219(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f169() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f170(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f255(match v0.clone() {
        Adt12::C0(h662, _, _, _, _) => {
            h662
        }
    }, 0u64)?, f255(match v0.clone() {
        Adt12::C0(_, h663, _, _, _) => {
            h663
        }
    }, 0u64)?, f255(match v0.clone() {
        Adt12::C0(_, _, h664, _, _) => {
            h664
        }
    }, 0u64)?, f255(match v0 {
        Adt12::C0(_, _, _, h665, _) => {
            h665
        }
    }, 0u64)?))
}

pub fn f171(v0: Adt12) -> List<Bytes> {
    f64(match v0.clone() {
        Adt12::C0(h666, _, _, _, _) => {
            h666
        }
    }, f64(match v0.clone() {
        Adt12::C0(_, _, h667, _, _) => {
            h667
        }
    }, f64(match v0.clone() {
        Adt12::C0(_, _, _, h668, _) => {
            h668
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h669) => {
            h669
        }
    })))
}

pub fn f172(v0: Adt16, p675: List<u64>, p676: List<Adt20>) -> Option<u64> {
    let m674 = (p675, p676);
    {
        let mut m674 = m674;
        loop {
            let step = {
                let (v1, v2) = m674;
                let m670 = v2;
                match m670.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f256(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h671) => {
                                h671
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h672, _) => {
                                    h672
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h673, _) => {
                                    h673
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m677 = v1;
    match m677.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f257(v0.clone(), v3)?), f173(v0, v4, {
                let a678 = v2;
                let a679 = 1u64;
                nat_add(a678, a679)?
            })?))
        }
    }
}

pub fn f174(p683: List<Bytes>, p684: u64) -> Bytes {
    let m682 = (p683, p684);
    {
        let mut m682 = m682;
        loop {
            let step = {
                let (v0, v1) = m682;
                let m680 = v0;
                match m680.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m681 = v1;
                        if m681 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m681 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f258(v0.clone(), v1.clone()) {
        if f258(v1, v0) {
            f259(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f176(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m685 = v0;
    match m685.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f260(v2, v1.clone())?, f176(v3, v1)?))
        }
    }
}

pub fn f177(v0: List<Adt14>) -> List<u64> {
    let m686 = v0;
    match m686.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f261(v1), f177(v2))
        }
    }
}

pub fn f178(p689: List<u64>, v1: List<u64>) -> Option<u64> {
    let m688 = p689;
    {
        let mut m688 = m688;
        loop {
            let step = {
                let v0 = m688;
                let m687 = v0;
                match m687.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f262(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m688 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(p692: List<Bytes>, p693: List<Bytes>) -> Option<Bytes> {
    let m691 = (p692, p693);
    {
        let mut m691 = m691;
        loop {
            let step = {
                let (v0, v1) = m691;
                let m690 = v0;
                match m690.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f86(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m694 = v0;
    match m694.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m695 = f263(v2, v1.clone())?;
            match m695 {
                None => {
                    f180(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f180(v3, v1)?))
                }
            }
        }
    }
}

pub fn f181(v0: List<Bytes>) -> List<Bytes> {
    f163(Fn1::F264, List::<Bytes>::nil(), v0)
}

pub fn f182(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f265(v0, v1)
}

pub fn f183(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f266(f267(f268(v0))?))
}

pub fn f184(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m696 = f154(v0, v2)?;
    match m696 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f269(v1, f157(v3.clone(), f8(Str::lit("id"))))?;
            let m697 = v4.clone();
            match m697.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m699 = f160(f159({
                        let m698 = f159(v3.clone(), f8(Str::lit("source")));
                        match m698 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m699 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f75(f86(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f209(f270(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f185(p702: List<Bytes>, p703: List<Bytes>) -> List<Bytes> {
    let m701 = (p702, p703);
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v0, v1) = m701;
                let m700 = v0;
                match m700.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f271(v0, v1)
}

pub fn f187() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f188(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m704 = v1;
    match m704.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt19 = f272(v0.clone(), v3)?;
            if f86(f273(f150(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f274(v5)?, f188(v0, v4, v2)?))
            } else {
                f188(v0, v4, v2)
            }
        }
    }
}

pub fn f189(v0: Adt1, p708: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m707 = p708;
    {
        let mut m707 = m707;
        loop {
            let step = {
                let v1 = m707;
                let m705 = v1;
                match m705.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt19 = f272(v0.clone(), v3.clone())?;
                        if if f86(f273(f150(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f36(f149({
                                let m706 = f147(v5.clone(), f8(Str::lit("schema")));
                                match m706 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f70(v0.clone(), f274(v5)?))
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
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f75(f76(match v0.clone() {
        Adt1::C0(_, h709, _) => {
            h709
        }
    }, f138())) {
        Ok(Adt11::C1(List::cons(f138(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f84(f78(f275(f80(match v0.clone() {
            Adt1::C0(_, h710, _) => {
                h710
            }
        }, f138())))?, v1);
        let m711 = f60(v0.clone(), List::cons(f89(), List::cons(f117(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m711.uncons() {
            None => {
                let v5: (bool, Bytes) = f276(v0.clone(), v2)?;
                let (h712, _) = v5;
                if h712 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h713) = v5;
                        h713
                    }))
                } else {
                    let m714 = f88(v0.clone());
                    match m714.uncons() {
                        None => {
                            let m715 = f90(v0.clone())?;
                            match m715.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h716) = v5;
                                        h716
                                    }, List::cons(f7(f78(f277(f102(v0.clone())?, f110(v0.clone(), f111(v0.clone())?)?))?), List::cons(f278(v0)?, List::<Bytes>::nil()))))))
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

pub fn f191(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f279(f73(v0, f89())?)?)))
}

pub fn f192() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f193(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt23>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m717 = v1;
    match m717.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f64(f280(v5.clone(), v2.clone()), f281(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f282(v0.clone(), v5.clone()), List::cons(if f86(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a719 = {
                    let a718 = v7.clone();
                    length_list(a718)
                };
                let a720 = 0u64;
                if nat_eq(a719, a720) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f121(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f193(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f194(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt23>> {
    let v3: List<(Bytes, Bytes)> = f283(v1);
    let v4: List<(Bytes, Bytes)> = f284(v0, v2);
    f285(v3.clone(), f91(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f195(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m721 = v1;
    match m721.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f202(v2.clone(), f8(Str::lit("domain"))) {
                f286(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f157(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f157(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m722 = f159(v2.clone(), f8(Str::lit("role")));
                    match m722 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f157(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f157(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f157(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f287(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m723 = f159(v2.clone(), f8(Str::lit("origin")));
                    match m723 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f157(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m724 = f159(v2, f8(Str::lit("evidence")));
                    match m724 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f157(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f195(v0, v3))
            } else {
                f195(v0, v3)
            }
        }
    }
}

pub fn f196(v0: List<Adt14>) -> List<Bytes> {
    let m725 = v0;
    match m725.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f157(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f157(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f157(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f196(v2))
        }
    }
}

pub fn f197(v0: List<Adt14>) -> List<Bytes> {
    let m726 = v0;
    match m726.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f157(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f288(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f197(v2))
        }
    }
}

pub fn f198(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m727 = v0;
    match m727.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f157(v1.clone(), f8(Str::lit("subject"))), f157(v1, f8(Str::lit("object")))), f198(v2))
        }
    }
}

pub fn f199(v0: List<Adt23>) -> List<(Bytes, Bytes)> {
    let m728 = v0;
    match m728.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt23::C0(h729, _, _) => {
                    h729
                }
            }, match v1 {
                Adt23::C0(_, _, h730) => {
                    h730
                }
            }), f199(v2))
        }
    }
}

pub fn f200(p737: List<Bytes>, v1: List<(Bytes, Bytes)>, p738: u64) -> R<List<Bytes>> {
    let m736 = (p737, p738);
    {
        let mut m736 = m736;
        loop {
            let step = {
                let (v0, v2) = m736;
                let m731 = v2;
                if m731 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m731 - 1;
                    let v4: List<Bytes> = f78(f64(v0.clone(), f289(v0.clone(), v1.clone())))?;
                    let a734 = {
                        let a732 = v4.clone();
                        length_list(a732)
                    };
                    let a735 = {
                        let a733 = v0.clone();
                        length_list(a733)
                    };
                    if nat_eq(a734, a735) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m736 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: List<Adt14>) -> List<Bytes> {
    let m739 = v0;
    match m739.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m740 = f159(v1, f8(Str::lit("std")));
            match m740 {
                None => {
                    f201(v2)
                }
                Some(v3) => {
                    List::cons(f157(v3.clone(), f8(Str::lit("authority"))), if f202(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f157(v3, f8(Str::lit("row"))), f201(v2))
                    } else {
                        f201(v2)
                    })
                }
            }
        }
    }
}

pub fn f202(v0: Adt14, v1: Bytes) -> bool {
    f75({
        let a742 = {
            let a741 = f157(v0, v1);
            length_bytes(a741)
        };
        let a743 = 0u64;
        nat_eq(a742, a743)
    })
}

pub fn f203(v0: Adt15, v1: Adt14) -> bool {
    if f290(v0.clone(), v1.clone(), f157(v1.clone(), f8(Str::lit("subject")))) {
        f290(v0, v1.clone(), f157(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f204(v0: Bytes, v1: Bytes) -> bool {
    let m748 = {
        let a745 = v0;
        let a746 = 0u64;
        let a747 = {
            let a744 = v1.clone();
            length_bytes(a744)
        };
        slice_bytes(a745, a746, a747)
    };
    match m748 {
        None => {
            false
        }
        Some(v2) => {
            let a749 = v2;
            let a750 = v1;
            equal(a749, a750)
        }
    }
}

pub fn f205(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m751 = v1;
    match m751.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f134(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f130(v0.clone(), v2), f205(v0, v3))
            } else {
                f205(v0, v3)
            }
        }
    }
}

pub fn f206(v0: Bytes, p809: u64, p810: u64, p811: Adt17) -> R<Adt17> {
    let m808 = (p809, (p810, p811));
    {
        let mut m808 = m808;
        loop {
            let step = {
                let (v1, (v2, v3)) = m808;
                let m752 = v2;
                if m752 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m752 - 1;
                    let v5: u64 = f291(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h753, _, _, _) => {
                            h753
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a754 = v1;
                            let a755 = 1u64;
                            nat_add(a754, a755)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h756, _, _) => {
                                h756
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h757, _) => {
                                h757
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h758) => {
                                h758
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h759, _, _, _, _) => {
                                h759
                            }
                        } {
                            let a760 = v5;
                            let a761 = 34u64;
                            if nat_eq(a760, a761) {
                                let a764 = f291(v0.clone(), {
                                    let a762 = v1;
                                    let a763 = 1u64;
                                    nat_add(a762, a763)?
                                })?;
                                let a765 = 34u64;
                                if nat_eq(a764, a765) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a766 = v1;
                                        let a767 = 1u64;
                                        nat_add(a766, a767)?
                                    }, (v4, Adt17::C0(true, true, f292(match v3.clone() {
                                        Adt17::C0(_, _, h768, _, _) => {
                                            h768
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h769, _) => {
                                            h769
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h770) => {
                                            h770
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a771 = v1;
                                        let a772 = 1u64;
                                        nat_add(a771, a772)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h773, _, _) => {
                                            h773
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h774, _) => {
                                            h774
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h775) => {
                                            h775
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a776 = v1;
                                    let a777 = 1u64;
                                    nat_add(a776, a777)?
                                }, (v4, Adt17::C0(true, false, f292(match v3.clone() {
                                    Adt17::C0(_, _, h778, _, _) => {
                                        h778
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h779, _) => {
                                        h779
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h780) => {
                                        h780
                                    }
                                }))))
                            }
                        } else {
                            let a781 = v5;
                            let a782 = 34u64;
                            if nat_eq(a781, a782) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a783 = v1;
                                    let a784 = 1u64;
                                    nat_add(a783, a784)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h785, _, _) => {
                                        h785
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h786, _) => {
                                        h786
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h787) => {
                                        h787
                                    }
                                }))))
                            } else {
                                let a788 = v5;
                                let a789 = 44u64;
                                if nat_eq(a788, a789) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a790 = v1;
                                        let a791 = 1u64;
                                        nat_add(a790, a791)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h792, _, _) => {
                                            h792
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h793, _) => {
                                            h793
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h794) => {
                                            h794
                                        }
                                    }))))
                                } else {
                                    let a795 = v5;
                                    let a796 = 13u64;
                                    if nat_eq(a795, a796) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a797 = v1;
                                            let a798 = 1u64;
                                            nat_add(a797, a798)?
                                        }, (v4, v3)))
                                    } else {
                                        let a799 = v5;
                                        let a800 = 10u64;
                                        if nat_eq(a799, a800) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a801 = v1;
                                                let a802 = 1u64;
                                                nat_add(a801, a802)?
                                            }, (v4, f207(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a803 = v1;
                                                let a804 = 1u64;
                                                nat_add(a803, a804)?
                                            }, (v4, Adt17::C0(false, false, f292(match v3.clone() {
                                                Adt17::C0(_, _, h805, _, _) => {
                                                    h805
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h806, _) => {
                                                    h806
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h807) => {
                                                    h807
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
                Ok(next) => { m808 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f293(List::cons(match v0.clone() {
        Adt17::C0(_, _, h812, _, _) => {
            h812
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h813, _) => {
            h813
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h814) => {
            h814
        }
    }))
}

pub fn f208(p817: List<List<Bytes>>, p818: List<List<Bytes>>) -> List<List<Bytes>> {
    let m816 = (p817, p818);
    {
        let mut m816 = m816;
        loop {
            let step = {
                let (v0, v1) = m816;
                let m815 = v0;
                match m815.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m816 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f294(v0.clone(), 0u64, {
        let a819 = v0;
        length_bytes(a819)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f210(v0: Adt24) -> Bytes {
    let a840 = f295(match v0.clone() {
        Adt24::C0(h820, _, _, _, _, _, _, _) => {
            h820
        }
    });
    let a841 = {
        let a838 = f295(match v0.clone() {
            Adt24::C0(_, h821, _, _, _, _, _, _) => {
                h821
            }
        });
        let a839 = {
            let a836 = f295(match v0.clone() {
                Adt24::C0(_, _, h822, _, _, _, _, _) => {
                    h822
                }
            });
            let a837 = {
                let a834 = f295(match v0.clone() {
                    Adt24::C0(_, _, _, h823, _, _, _, _) => {
                        h823
                    }
                });
                let a835 = {
                    let a832 = f295(match v0.clone() {
                        Adt24::C0(_, _, _, _, h824, _, _, _) => {
                            h824
                        }
                    });
                    let a833 = {
                        let a830 = f295(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h825, _, _) => {
                                h825
                            }
                        });
                        let a831 = {
                            let a828 = f295(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h826, _) => {
                                    h826
                                }
                            });
                            let a829 = f295(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h827) => {
                                    h827
                                }
                            });
                            append_bytes(a828, a829)
                        };
                        append_bytes(a830, a831)
                    };
                    append_bytes(a832, a833)
                };
                append_bytes(a834, a835)
            };
            append_bytes(a836, a837)
        };
        append_bytes(a838, a839)
    };
    append_bytes(a840, a841)
}

pub fn f211(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a842 = v0.clone();
        length_bytes(a842)
    };
    let v2: u64 = {
        let a848 = {
            let a845 = {
                let a843 = v1;
                let a844 = 72u64;
                nat_add(a843, a844)?
            };
            let a846 = 64u64;
            let a847 = 0u64;
            nat_quot(a845, a846, a847)
        };
        let a849 = 64u64;
        nat_mul(a848, a849)?
    };
    let (_, h850) = f296(Fn4::F297(v0, v1, v2), {
        let a851 = v2;
        let a852 = 64u64;
        let a853 = 0u64;
        nat_quot(a851, a852, a853)
    }, (0u64, f298()))?;
    Ok(h850)
}

pub fn f212(p856: List<List<Bytes>>, v1: Bytes, p857: List<Bytes>) -> R<List<Bytes>> {
    let m855 = (p856, p857);
    {
        let mut m855 = m855;
        loop {
            let step = {
                let (v0, v2) = m855;
                let m854 = v0;
                match m854.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f299(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f214(f300(v3), Bytes::lit(&[0x0a])), v2)))
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

pub fn f213(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f301(v0)?))
}

pub fn f214(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m858 = v0;
    match m858.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m859 = v3.clone();
            match m859.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a862 = v2;
                    let a863 = {
                        let a860 = v1.clone();
                        let a861 = f214(v3, v1);
                        append_bytes(a860, a861)
                    };
                    append_bytes(a862, a863)
                }
            }
        }
    }
}

pub fn f215(v0: Adt14) -> R<Bytes> {
    let a864 = f302(v0)?;
    let a865 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a864, a865))
}

pub fn f216(v0: List<Bytes>) -> Adt14 {
    let m866 = v0;
    match m866.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f216(v2)))
        }
    }
}

pub fn f217(v0: Fn2, p870: Option<Bytes>, p871: List<Bytes>) -> Option<Bytes> {
    let m869 = (p870, p871);
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v1, v2) = m869;
                let m867 = v2;
                match m867.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c868: Fn2 = v0.clone();
                            c868.apply(v1, v3)
                        }, v4))
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

pub fn f218(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m872 = v1;
    match m872 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a875 = {
                    let a873 = v3;
                    let a874 = v0;
                    append_bytes(a873, a874)
                };
                let a876 = v2;
                append_bytes(a875, a876)
            })
        }
    }
}

pub fn f219(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m881 = {
        let a878 = v0;
        let a879 = v2;
        let a880 = {
            let a877 = v1.clone();
            length_bytes(a877)
        };
        slice_bytes(a878, a879, a880)
    };
    match m881 {
        None => {
            false
        }
        Some(v3) => {
            let a882 = v3;
            let a883 = v1;
            equal(a882, a883)
        }
    }
}

pub fn f220(v0: Bytes) -> R<Bytes> {
    let (_, h884) = {
        let (h885, _) = f303(Fn5::F304(v0.clone()), {
            let a887 = {
                let a886 = v0;
                length_bytes(a886)
            };
            let a888 = 1u64;
            nat_add(a887, a888)?
        }, (0u64, Bytes::lit(&[])))?;
        h885
    };
    Ok(h884)
}

pub fn f221(v0: Bytes) -> R<u64> {
    let (h889, _) = f253(Fn3::F305(v0.clone()), {
        let a891 = {
            let a890 = v0;
            length_bytes(a890)
        };
        let a892 = 1u64;
        nat_add(a891, a892)?
    }, 0u64)?;
    Ok(h889)
}

pub fn f222(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a893 = v0;
        utf8_encode(a893)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f126(v1)?, List::<Bytes>::nil())))))
}

pub fn f223(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f225(v0.clone(), 0u64, Str::lit("<?xml")) {
        f306(f226(v0.clone(), 5u64)?)
    } else {
        false
    } {
        let m895 = f307(v0.clone(), {
            let a894 = Str::lit("?>");
            utf8_encode(a894)
        }, 5u64)?;
        match m895 {
            None => {
                Ok(Err::<u64, Bytes>(f222(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m897 = f307(f32(v0.clone(), 0u64, v1), {
                    let a896 = Str::lit("encoding");
                    utf8_encode(a896)
                }, 5u64)?;
                match m897 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a898 = v1;
                            let a899 = 2u64;
                            nat_add(a898, a899)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f308(v0.clone(), {
                            let a900 = v2;
                            let a901 = 8u64;
                            nat_add(a900, a901)?
                        })?;
                        let v4: u64 = f308(v0.clone(), {
                            let a902 = v3;
                            let a903 = 1u64;
                            nat_add(a902, a903)?
                        })?;
                        let _: u64 = f226(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m906 = f307(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a904 = v4;
                                let a905 = 1u64;
                                nat_add(a904, a905)?
                            })?;
                            match m906 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f309(f32(v0, {
                            let a907 = v4;
                            let a908 = 1u64;
                            nat_add(a907, a908)?
                        }, {
                            let a911 = v7;
                            let a912 = {
                                let a909 = v4;
                                let a910 = 1u64;
                                nat_add(a909, a910)?
                            };
                            nat_sub(a911, a912)
                        }))? {
                            Ok(Ok::<u64, Bytes>({
                                let a913 = v1;
                                let a914 = 2u64;
                                nat_add(a913, a914)?
                            }))
                        } else {
                            Ok(Err::<u64, Bytes>(f222(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f224(v0: Bytes, p919: u64, p920: u64) -> R<Result<u64, Bytes>> {
    let m918 = (p919, p920);
    {
        let mut m918 = m918;
        loop {
            let step = {
                let (v1, v2) = m918;
                let m915 = v2;
                if m915 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(f222(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m915 - 1;
                    let v4: u64 = f308(v0.clone(), v1)?;
                    if f225(v0.clone(), v4, Str::lit("<!--")) {
                        let m916 = f310(v0.clone(), v4)?;
                        match m916 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v5)))
                            }
                        }
                    } else {
                        if f225(v0.clone(), v4, Str::lit("<?")) {
                            let m917 = f311(v0.clone(), v4)?;
                            match m917 {
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
                Ok(next) => { m918 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: Bytes, v1: u64, v2: Str) -> bool {
    f219(v0, {
        let a921 = v2;
        utf8_encode(a921)
    }, v1)
}

pub fn f226(v0: Bytes, v1: u64) -> R<u64> {
    f312(v0, v1)
}

pub fn f227(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt18, u64), Bytes>> {
    let m922 = v2;
    if m922 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m922 - 1;
        let a923 = v3;
        let a924 = 0u64;
        if nat_eq(a923, a924) {
            let v5: u64 = f313(v0.clone(), {
                let a925 = v1;
                let a926 = 1u64;
                nat_add(a925, a926)?
            })?;
            let a929 = v5;
            let a930 = {
                let a927 = v1;
                let a928 = 1u64;
                nat_add(a927, a928)?
            };
            if nat_eq(a929, a930) {
                Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a931 = v1;
                    let a932 = 1u64;
                    nat_add(a931, a932)?
                }, {
                    let a935 = v5;
                    let a936 = {
                        let a933 = v1;
                        let a934 = 1u64;
                        nat_add(a933, a934)?
                    };
                    nat_sub(a935, a936)
                });
                let m937 = f314(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m937 {
                    Ok(v8) => {
                        if f225(v0.clone(), {
                            let (_, h938) = v8.clone();
                            h938
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                let (h939, _) = v8;
                                h939
                            }), std::rc::Rc::new(Adt18::C4)), {
                                let a941 = {
                                    let (_, h940) = v8;
                                    h940
                                };
                                let a942 = 2u64;
                                nat_add(a941, a942)?
                            })))
                        } else {
                            let m946 = f227(v0.clone(), {
                                let a944 = {
                                    let (_, h943) = v8;
                                    h943
                                };
                                let a945 = 1u64;
                                nat_add(a944, a945)?
                            }, v4, 1u64)?;
                            match m946 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a948 = {
                                            let (_, h947) = v10.clone();
                                            h947
                                        };
                                        let a949 = 2u64;
                                        nat_add(a948, a949)?
                                    };
                                    let v12: u64 = f313(v0.clone(), v11)?;
                                    if !f36(f32(v0.clone(), v11, {
                                        let a950 = v12;
                                        let a951 = v11;
                                        nat_sub(a950, a951)
                                    }), v6.clone()) {
                                        Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h952) = v10;
                                            h952
                                        })?))
                                    } else {
                                        let v13: u64 = f308(v0.clone(), v12)?;
                                        let a953 = f226(v0, v13)?;
                                        let a954 = 62u64;
                                        if nat_eq(a953, a954) {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                                let (h955, _) = v8;
                                                h955
                                            }), std::rc::Rc::new({
                                                let (h956, _) = v10;
                                                h956
                                            })), {
                                                let a957 = v13;
                                                let a958 = 1u64;
                                                nat_add(a957, a958)?
                                            })))
                                        } else {
                                            Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
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
            let a959 = f226(v0.clone(), v1)?;
            let a960 = 256u64;
            if nat_eq(a959, a960) {
                Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f225(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v1)))
                } else {
                    if f225(v0.clone(), v1, Str::lit("<!--")) {
                        let m961 = f310(v0.clone(), v1)?;
                        match m961 {
                            Ok(v15) => {
                                f227(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(Err::<(Adt18, u64), Bytes>(v14))
                            }
                        }
                    } else {
                        if f225(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m965 = f307(v0.clone(), {
                                let a962 = Str::lit("]]>");
                                utf8_encode(a962)
                            }, {
                                let a963 = v1;
                                let a964 = 9u64;
                                nat_add(a963, a964)?
                            })?;
                            match m965 {
                                None => {
                                    Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m968 = f227(v0.clone(), {
                                        let a966 = v16;
                                        let a967 = 3u64;
                                        nat_add(a966, a967)?
                                    }, v4, 1u64)?;
                                    match m968 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2(f32(v0, {
                                                let a969 = v1;
                                                let a970 = 9u64;
                                                nat_add(a969, a970)?
                                            }, {
                                                let a973 = v16;
                                                let a974 = {
                                                    let a971 = v1;
                                                    let a972 = 9u64;
                                                    nat_add(a971, a972)?
                                                };
                                                nat_sub(a973, a974)
                                            }))), std::rc::Rc::new({
                                                let (h975, _) = v18;
                                                h975
                                            })), {
                                                let (_, h976) = v18;
                                                h976
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(Err::<(Adt18, u64), Bytes>(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f225(v0.clone(), v1, Str::lit("<?")) {
                                let m977 = f311(v0.clone(), v1)?;
                                match m977 {
                                    Ok(v20) => {
                                        f227(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v19))
                                    }
                                }
                            } else {
                                if f225(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a978 = f226(v0.clone(), v1)?;
                                    let a979 = 60u64;
                                    if nat_eq(a978, a979) {
                                        let m980 = f227(v0.clone(), v1, v4, 0u64)?;
                                        match m980 {
                                            Ok(v22) => {
                                                let m982 = f227(v0, {
                                                    let (_, h981) = v22;
                                                    h981
                                                }, v4, 1u64)?;
                                                match m982 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new({
                                                            let (h983, _) = v22;
                                                            h983
                                                        }), std::rc::Rc::new({
                                                            let (h984, _) = v24;
                                                            h984
                                                        })), {
                                                            let (_, h985) = v24;
                                                            h985
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
                                        let m986 = f315(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m986 {
                                            Ok(v26) => {
                                                let m988 = f227(v0, {
                                                    let (_, h987) = v26;
                                                    h987
                                                }, v4, 1u64)?;
                                                match m988 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2({
                                                            let (h989, _) = v26;
                                                            h989
                                                        })), std::rc::Rc::new({
                                                            let (h990, _) = v28;
                                                            h990
                                                        })), {
                                                            let (_, h991) = v28;
                                                            h991
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

pub fn f228(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a992 = v2;
    let a993 = 0u64;
    if nat_eq(a992, a993) {
        Ok(f316(v0, 0u64))
    } else {
        let (_, h994) = {
            let (h995, _) = f303(Fn5::F317(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h995
        };
        Ok(h994)
    }
}

pub fn f229() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f230(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f301(v0.clone())?;
    let v2: u64 = f318(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a996 = v1;
        let a997 = v2;
        nat_sub(a996, a997)
    }))
}

pub fn f231(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f319(v0)?;
    let m1001 = f320(v1.clone(), 0u64, {
        let a999 = {
            let a998 = v1.clone();
            length_bytes(a998)
        };
        let a1000 = 2u64;
        nat_add(a999, a1000)?
    }, 0u64)?;
    match m1001 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a1004 = {
                let (_, h1002) = v2;
                h1002
            };
            let a1005 = {
                let a1003 = v1;
                length_bytes(a1003)
            };
            if nat_eq(a1004, a1005) {
                Ok(Some({
                    let (h1006, _) = v2;
                    h1006
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f232(v0: Bytes) -> Bytes {
    f7(List::cons(f77(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f233(v0: Adt14) -> bool {
    if f202(v0.clone(), f8(Str::lit("domain"))) {
        if f202(v0.clone(), f8(Str::lit("row"))) {
            if f86(f157(v0.clone(), f8(Str::lit("relation"))), f321()) {
                if f322(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f322(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f322(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f322(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f234(v0: Adt14) -> bool {
    if f202(v0.clone(), f8(Str::lit("subject"))) {
        if f202(v0.clone(), f8(Str::lit("object"))) {
            if f202(v0.clone(), f8(Str::lit("evidence"))) {
                f86(f157(v0, f8(Str::lit("relation"))), f323())
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

pub fn f235(v0: Option<List<Adt14>>) -> bool {
    let m1007 = v0;
    match m1007 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f236(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1008 = v0;
    match m1008 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f237(v0: Adt14) -> Bytes {
    if f202(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f202(v0.clone(), f8(Str::lit("id"))) {
            if f202(v0.clone(), f8(Str::lit("statement"))) {
                if f202(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f247(v0, f8(Str::lit("evidence")))
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

pub fn f238(v0: List<Adt14>) -> Adt14 {
    let m1009 = v0;
    match m1009.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f238(v2)))
        }
    }
}

pub fn f239(v0: Bytes) -> R<Option<Adt19>> {
    let v1: Bytes = f319(v0)?;
    let m1013 = f324(v1.clone(), 0u64, {
        let a1011 = {
            let a1010 = v1.clone();
            length_bytes(a1010)
        };
        let a1012 = 2u64;
        nat_add(a1011, a1012)?
    }, 0u64)?;
    match m1013 {
        None => {
            Ok(None::<Adt19>)
        }
        Some(v2) => {
            let a1016 = {
                let (_, h1014) = v2;
                h1014
            };
            let a1017 = {
                let a1015 = v1;
                length_bytes(a1015)
            };
            if nat_eq(a1016, a1017) {
                Ok(Some({
                    let (h1018, _) = v2;
                    h1018
                }))
            } else {
                Ok(None::<Adt19>)
            }
        }
    }
}

pub fn f240(v0: Adt19) -> bool {
    if f325(v0.clone(), f8(Str::lit("name"))) {
        if f326(v0.clone(), f8(Str::lit("schema"))) {
            let m1019 = f242(f151(f152(f147(v0, f8(Str::lit("citation"))))));
            match m1019 {
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

pub fn f241(v0: Bytes) -> R<Bytes> {
    f327(v0.clone(), 0u64, {
        let a1020 = v0;
        length_bytes(a1020)
    }, true, Bytes::lit(&[]))
}

pub fn f242(v0: List<Bytes>) -> Option<Adt14> {
    let m1021 = v0;
    match m1021.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m1022 = v2;
                match m1022.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m1023 = v2;
                    match m1023.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1024 = v6;
                                match m1024.uncons() {
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
                        let m1025 = v2;
                        match m1025.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1026 = v10;
                                match m1026.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f216(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f243(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1027 = f60(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1027.uncons() {
        None => {
            let m1028 = f242(v4);
            match m1028 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1029 = f8(Str::lit("Catalog."));
                        let a1030 = f241(v3.clone())?;
                        append_bytes(a1029, a1030)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f241(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1031 = f328(v2, v9.clone(), f241(v3.clone())?)?;
                    match m1031 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f241(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f215(Adt14::C3(std::rc::Rc::new(f238(f329(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f215(Adt14::C3(std::rc::Rc::new(f238(f329(f73(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1035 = f330(f70(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a1033 = {
                                        let a1032 = v10;
                                        length_bytes(a1032)
                                    };
                                    let a1034 = 4u64;
                                    nat_sub(a1033, a1034)
                                }))?;
                                let a1036 = Bytes::lit(&[0x0a]);
                                append_bytes(a1035, a1036)
                            }), List::cons(f120(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f244(v0: Adt19) -> R<Bytes> {
    let m1037 = v0;
    match m1037 {
        Adt19::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C1(v2, v3) => {
            f331(v2, v3)
        }
        Adt19::C2(v4, v5, v6) => {
            f332(v4, v5, v6)
        }
        Adt19::C3(v7) => {
            f333(v7)
        }
        Adt19::C4(r1038) => {
            let v8 = (*r1038).clone();
            Ok(f334(f335(v8)?))
        }
        Adt19::C5(r1039) => {
            let v9 = (*r1039).clone();
            f336(f337(v9)?)
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

pub fn f245(v0: Option<Adt19>) -> Option<Bytes> {
    let m1040 = v0;
    match m1040 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1041 = v1;
            match m1041 {
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

pub fn f246(v0: Adt19) -> List<Adt19> {
    let m1042 = v0;
    match m1042 {
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
        Adt19::C4(r1043) => {
            let v3 = (*r1043).clone();
            f246(v3)
        }
        Adt19::C5(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C6 => {
            List::<Adt19>::nil()
        }
        Adt19::C7(r1044, r1045) => {
            let v1 = (*r1044).clone();
            let v2 = (*r1045).clone();
            List::cons(v1, f246(v2))
        }
        Adt19::C8(_, _, _) => {
            List::<Adt19>::nil()
        }
    }
}

pub fn f247(v0: Adt14, v1: Bytes) -> bool {
    let m1046 = f159(v0, v1);
    match m1046 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f248(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1047 = f159(v0, v1);
    match m1047 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f156(f159(v3, v2))
        }
    }
}

pub fn f249(v0: Adt14) -> List<Adt14> {
    let m1048 = v0;
    match m1048 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1049) => {
            let v3 = (*r1049).clone();
            f249(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1050, r1051) => {
            let v1 = (*r1050).clone();
            let v2 = (*r1051).clone();
            List::cons(v1, f249(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f250(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f159(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f116(Str::lit("id"), f157(v0.clone(), f8(Str::lit("id"))))?, List::cons(f116(Str::lit("name"), f157(v0.clone(), f8(Str::lit("name"))))?, List::cons(f116(Str::lit("issuer"), f157(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f116(Str::lit("edition"), f157(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f116(Str::lit("canonical_id"), f157(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f116(Str::lit("source_role"), f157(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f116(Str::lit("retrieval_date"), f157(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f116(Str::lit("url"), f338(v2.clone(), f8(Str::lit("url"))))?, List::cons(f116(Str::lit("media_type"), f338(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f116(Str::lit("license"), f338(v2.clone(), f8(Str::lit("license"))))?, List::cons(f116(Str::lit("redistribution"), f338(v2, f8(Str::lit("redistribution"))))?, List::cons(f116(Str::lit("statement"), f157(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1052 = v1;
        match m1052 {
            None => {
                f7(List::cons(f116(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f116(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f116(Str::lit("artifact"), {
                    let (h1053, _) = v3;
                    h1053
                })?, List::cons(f116(Str::lit("checksum"), {
                    let (_, h1054) = v3;
                    h1054
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f339(f74(f156(f159(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f251(v0: Bytes, v1: u64) -> Option<u8> {
    let a1055 = v0;
    let a1056 = v1;
    index_bytes(a1055, a1056)
}

pub fn f252(v0: Bytes) -> bool {
    f134(v0, f8(Str::lit(".lex.tex")))
}

pub fn f253(v0: Fn3, p1061: u64, p1062: u64) -> R<(u64, bool)> {
    let m1060 = (p1061, p1062);
    {
        let mut m1060 = m1060;
        loop {
            let step = {
                let (v1, v2) = m1060;
                let m1057 = v1;
                if m1057 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1057 - 1;
                    let m1059 = {
                        let c1058: Fn3 = v0.clone();
                        c1058.apply(v2)?
                    };
                    match m1059 {
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
                Ok(next) => { m1060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f254(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f219(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1065 = {
            let a1063 = v4;
            let a1064 = v1;
            nat_add(a1063, a1064)?
        };
        let a1066 = v3;
        if nat_lt(a1065, a1066) {
            Ok(Some({
                let a1067 = v4;
                let a1068 = 1u64;
                nat_add(a1067, a1068)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f255(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1069 = v0;
    match m1069.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f255(v3, {
                let a1070 = v1;
                let a1071 = 1u64;
                nat_add(a1070, a1071)?
            })?))
        }
    }
}

pub fn f256(v0: Adt16, v1: List<u64>, v2: Adt21) -> bool {
    let m1072 = v2;
    match m1072 {
        Adt21::C0(v3, v4) => {
            if f262(v3, match v0.clone() {
                Adt16::C0(h1073, _, _, _) => {
                    h1073
                }
            }) {
                f340(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f262(v5, match v0 {
                Adt16::C0(_, _, h1074, _) => {
                    h1074
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f258(v7, v1) {
                f262(v8, match v0 {
                    Adt16::C0(_, _, _, h1075) => {
                        h1075
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f257(v0: Adt12, v1: Adt14) -> R<Adt21> {
    let m1076 = f159(v1.clone(), f8(Str::lit("std")));
    match m1076 {
        None => {
            let m1081 = f160(f159(v1.clone(), f8(Str::lit("req"))));
            match m1081 {
                None => {
                    let m1083 = f159(v1, f8(Str::lit("der")));
                    match m1083 {
                        None => {
                            Ok(f341(v0))
                        }
                        Some(v5) => {
                            let m1084 = f342(f157(v5.clone(), f8(Str::lit("rule"))));
                            match m1084 {
                                None => {
                                    Ok(f341(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f176(f74(f156(f159(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h1085) => {
                                            h1085
                                        }
                                    })?, f260(f157(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h1086, _) => {
                                            h1086
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f260(v4, match v0 {
                        Adt12::C0(_, _, h1082, _, _) => {
                            h1082
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1077 = f160(f159(v2.clone(), f8(Str::lit("row"))));
            match m1077 {
                None => {
                    Ok(Adt21::C0(f260(f157(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h1078, _, _, _, _) => {
                            h1078
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f260(f157(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h1079, _, _, _, _) => {
                            h1079
                        }
                    })?, Some(f260(v3, match v0 {
                        Adt12::C0(_, h1080, _, _, _) => {
                            h1080
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f258(p1089: List<u64>, v1: List<u64>) -> bool {
    let m1088 = p1089;
    {
        let mut m1088 = m1088;
        loop {
            let step = {
                let v0 = m1088;
                let m1087 = v0;
                match m1087.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f262(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f259(p1094: List<u64>) -> bool {
    let m1093 = p1094;
    {
        let mut m1093 = m1093;
        loop {
            let step = {
                let v0 = m1093;
                let m1090 = v0;
                match m1090.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1091 = v1;
                        let a1092 = 7u64;
                        if nat_le(a1091, a1092) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1093 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1095 = f343(v0, v1.clone(), 0u64)?;
    match m1095 {
        None => {
            let a1096 = v1;
            Ok(length_list(a1096))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f261(v0: Adt14) -> u64 {
    let m1097 = f344(f159(v0, f8(Str::lit("stratum"))));
    match m1097 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f262(v0: u64, p1102: List<u64>) -> bool {
    let m1101 = p1102;
    {
        let mut m1101 = m1101;
        loop {
            let step = {
                let v1 = m1101;
                let m1098 = v1;
                match m1098.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1099 = v2;
                        let a1100 = v0;
                        if nat_eq(a1099, a1100) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1101 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f204(v0.clone(), v1) {
        let m1103 = f30(v0, 34u8)?;
        match m1103.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1104 = v3;
                match m1104.uncons() {
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

pub fn f264(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f265(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1105 = v0.clone();
    match m1105.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1106 = v2;
            let a1107 = v1.clone();
            if equal(a1106, a1107) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f266(v0: List<(Bytes, ())>) -> List<Bytes> {
    f181(f345(Fn6::F346, List::<Bytes>::nil(), v0))
}

pub fn f267(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f347(f348(Fn7::F349, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1112 = {
        let (h1108, _) = f350(Fn8::F351, {
            let a1110 = {
                let a1109 = v0;
                length_list(a1109)
            };
            let a1111 = 1u64;
            nat_add(a1110, a1111)?
        }, v1)?;
        h1108
    };
    match m1112.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f268(v0: List<Bytes>) -> List<(Bytes, ())> {
    f352(f353(Fn9::F354, List::<(Bytes, ())>::nil(), v0))
}

pub fn f269(p1115: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1114 = p1115;
    {
        let mut m1114 = m1114;
        loop {
            let step = {
                let v0 = m1114;
                let m1113 = v0;
                match m1113.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f299(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f338(f159(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f271(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1116, _) = v1.clone();
        h1116
    };
    let v3: u64 = {
        let (h1117, _) = {
            let (_, h1118) = v1.clone();
            h1118
        };
        h1117
    };
    let v4: List<Bytes> = {
        let (_, h1119) = {
            let (_, h1120) = v1;
            h1120
        };
        h1119
    };
    let m1121 = f251(v0.clone(), v2);
    match m1121 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1122 = v5;
            let a1123 = 10u8;
            if equal(a1122, a1123) {
                Ok(Some(({
                    let a1124 = v2;
                    let a1125 = 1u64;
                    nat_add(a1124, a1125)?
                }, ({
                    let a1126 = v2;
                    let a1127 = 1u64;
                    nat_add(a1126, a1127)?
                }, List::cons(f355(f32(v0, v3, {
                    let a1128 = v2;
                    let a1129 = v3;
                    nat_sub(a1128, a1129)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1130 = v2;
                    let a1131 = 1u64;
                    nat_add(a1130, a1131)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f272(v0: Adt1, v1: Bytes) -> R<Adt19> {
    let m1132 = f146(v0, v1)?;
    match m1132 {
        None => {
            Ok(Adt19::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f273(v0: Bytes) -> R<Bytes> {
    let a1133 = f8(Str::lit("Catalog."));
    let a1134 = f241(v0)?;
    Ok(append_bytes(a1133, a1134))
}

pub fn f274(v0: Adt19) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f241(f150(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f275(v0: List<Bytes>) -> List<Bytes> {
    let m1135 = v0;
    match m1135.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f134(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f130(f138(), v1), f275(v2))
            } else {
                f275(v2)
            }
        }
    }
}

pub fn f276(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1136 = v1;
    match m1136.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f356(v0.clone(), v2)?;
            let (h1137, _) = v4.clone();
            if h1137 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f276(v0, v3)?;
                let (h1138, _) = v5.clone();
                if h1138 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1141 = {
                            let (_, h1139) = v4;
                            h1139
                        };
                        let a1142 = {
                            let (_, h1140) = v5;
                            h1140
                        };
                        append_bytes(a1141, a1142)
                    }))
                }
            }
        }
    }
}

pub fn f277(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1143 = v1;
    match m1143.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f202(v2.clone(), f8(Str::lit("subject"))) {
                if f203(v0.clone(), v2.clone()) {
                    f277(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f157(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f157(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f157(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f277(v0, v3))
                }
            } else {
                if f286(v0.clone(), v2.clone()) {
                    f277(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f157(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f287(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f277(v0, v3))
                }
            }
        }
    }
}

pub fn f278(v0: Adt1) -> R<Bytes> {
    let a1145 = {
        let a1144 = f103(f104(f122(f123(f112(f70(v0.clone(), f117()))?)), f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f106(f73(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1144)
    };
    let a1146 = 0u64;
    if nat_eq(a1145, a1146) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f279(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1147 = v0;
    match m1147.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f121(f74(f156(f159(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f157(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f157(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f357(f83(f157(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f279(v2)?))
        }
    }
}

pub fn f280(v0: Bytes, v1: List<Adt23>) -> List<Bytes> {
    let m1148 = v1;
    match m1148.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt23::C0(h1149, _, _) => {
                    h1149
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt23::C0(_, h1150, _) => {
                        h1150
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt23::C0(_, _, h1151) => {
                        h1151
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f280(v0, v3))
            } else {
                f280(v0, v3)
            }
        }
    }
}

pub fn f281(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1152 = v1;
    match m1152.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f157(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f157(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f157(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f281(v0, v3))
            } else {
                if f36(f157(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f157(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f157(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f281(v0, v3))
                } else {
                    f281(v0, v3)
                }
            }
        }
    }
}

pub fn f282(p1157: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1156 = p1157;
    {
        let mut m1156 = m1156;
        loop {
            let step = {
                let v0 = m1156;
                let m1153 = v0;
                match m1153.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1154 = v2;
                        match m1154.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m1155 = v5;
                                    match m1155.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f358(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1158 = v0;
    match m1158.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1159 = f159(v1, f8(Str::lit("std")));
            match m1159 {
                None => {
                    f283(v2)
                }
                Some(v3) => {
                    if f202(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f157(v3.clone(), f8(Str::lit("row"))), f157(v3, f8(Str::lit("authority")))), f283(v2))
                    } else {
                        f283(v2)
                    }
                }
            }
        }
    }
}

pub fn f284(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1160 = v1;
    match m1160.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f157(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f157(v2, f8(Str::lit("object")));
            if if f86(v4.clone(), match v0.clone() {
                Adt15::C0(h1161, _, _) => {
                    h1161
                }
            }) {
                f86(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1162, _) => {
                        h1162
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f284(v0, v3))
            } else {
                if if f86(v5.clone(), match v0.clone() {
                    Adt15::C0(h1163, _, _) => {
                        h1163
                    }
                }) {
                    f86(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1164, _) => {
                            h1164
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f284(v0, v3))
                } else {
                    f284(v0, v3)
                }
            }
        }
    }
}

pub fn f285(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt23>> {
    let m1165 = v0;
    match m1165.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v4, v5)) => {
            if if f359(v4.clone(), v2.clone()) {
                true
            } else {
                f359(v4.clone(), v3.clone())
            } {
                f285(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt23::C0({
                    let (h1166, _) = v4.clone();
                    h1166
                }, {
                    let a1169 = {
                        let a1168 = f78(f360({
                            let (h1167, _) = v4.clone();
                            h1167
                        }, v1.clone()))?;
                        length_list(a1168)
                    };
                    let a1170 = 1u64;
                    if nat_eq(a1169, a1170) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1171) = v4.clone();
                    h1171
                }), f285(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f286(v0: Adt15, v1: Adt14) -> bool {
    if f86(f157(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1172, _, _) => {
            h1172
        }
    }) {
        f86(f287(v1), match v0 {
            Adt15::C0(_, h1173, _) => {
                h1173
            }
        })
    } else {
        false
    }
}

pub fn f287(v0: Adt14) -> Bytes {
    let m1174 = f159(v0, f8(Str::lit("evidence")));
    match m1174 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f157(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f288(v0: Adt14) -> Bytes {
    if f202(v0.clone(), f8(Str::lit("req"))) {
        let a1175 = f8(Str::lit("req "));
        let a1176 = f157(v0, f8(Str::lit("req")));
        append_bytes(a1175, a1176)
    } else {
        let m1177 = f159(v0.clone(), f8(Str::lit("std")));
        match m1177 {
            None => {
                let m1180 = f159(v0, f8(Str::lit("der")));
                match m1180 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f157(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f121(f74(f156(f159(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f157(v1.clone(), f8(Str::lit("authority"))), List::cons(if f202(v1.clone(), f8(Str::lit("row"))) {
                    let a1178 = f8(Str::lit(" row "));
                    let a1179 = f157(v1, f8(Str::lit("row")));
                    append_bytes(a1178, a1179)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f289(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1181 = v1;
    match m1181.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86({
                let (h1182, _) = v2.clone();
                h1182
            }, v0.clone()) {
                List::cons({
                    let (_, h1183) = v2;
                    h1183
                }, f289(v0, v3))
            } else {
                if f86({
                    let (_, h1184) = v2;
                    h1184
                }, v0.clone()) {
                    List::cons({
                        let (h1185, _) = v2;
                        h1185
                    }, f289(v0, v3))
                } else {
                    f289(v0, v3)
                }
            }
        }
    }
}

pub fn f290(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f86(v2.clone(), match v0.clone() {
        Adt15::C0(h1186, _, _) => {
            h1186
        }
    }) {
        true
    } else {
        if f86(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1187, _) => {
                h1187
            }
        }) {
            true
        } else {
            if f86(v2, match v0.clone() {
                Adt15::C0(_, _, h1188) => {
                    h1188
                }
            }) {
                true
            } else {
                if f202(v1.clone(), f8(Str::lit("scheme"))) {
                    f86(f157(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1189, _) => {
                            h1189
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f291(v0: Bytes, v1: u64) -> R<u64> {
    let m1192 = {
        let a1190 = v0;
        let a1191 = v1;
        index_bytes(a1190, a1191)
    };
    match m1192 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1237 = {
                let a1213 = {
                    let a1201 = {
                        let a1195 = {
                            let a1193 = v3;
                            let a1194 = 128u8;
                            fixed_u8::bit_and(a1193, a1194)
                        };
                        let a1196 = 128u8;
                        if equal(a1195, a1196) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1202 = {
                        let a1199 = {
                            let a1197 = v3;
                            let a1198 = 64u8;
                            fixed_u8::bit_and(a1197, a1198)
                        };
                        let a1200 = 64u8;
                        if equal(a1199, a1200) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1201, a1202)?
                };
                let a1214 = {
                    let a1211 = {
                        let a1205 = {
                            let a1203 = v3;
                            let a1204 = 32u8;
                            fixed_u8::bit_and(a1203, a1204)
                        };
                        let a1206 = 32u8;
                        if equal(a1205, a1206) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1212 = {
                        let a1209 = {
                            let a1207 = v3;
                            let a1208 = 16u8;
                            fixed_u8::bit_and(a1207, a1208)
                        };
                        let a1210 = 16u8;
                        if equal(a1209, a1210) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1211, a1212)?
                };
                nat_add(a1213, a1214)?
            };
            let a1238 = {
                let a1235 = {
                    let a1223 = {
                        let a1217 = {
                            let a1215 = v3;
                            let a1216 = 8u8;
                            fixed_u8::bit_and(a1215, a1216)
                        };
                        let a1218 = 8u8;
                        if equal(a1217, a1218) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1224 = {
                        let a1221 = {
                            let a1219 = v3;
                            let a1220 = 4u8;
                            fixed_u8::bit_and(a1219, a1220)
                        };
                        let a1222 = 4u8;
                        if equal(a1221, a1222) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1223, a1224)?
                };
                let a1236 = {
                    let a1233 = {
                        let a1227 = {
                            let a1225 = v3;
                            let a1226 = 2u8;
                            fixed_u8::bit_and(a1225, a1226)
                        };
                        let a1228 = 2u8;
                        if equal(a1227, a1228) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1234 = {
                        let a1231 = {
                            let a1229 = v3;
                            let a1230 = 1u8;
                            fixed_u8::bit_and(a1229, a1230)
                        };
                        let a1232 = 1u8;
                        if equal(a1231, a1232) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1233, a1234)?
                };
                nat_add(a1235, a1236)?
            };
            nat_add(a1237, a1238)
        }
    }
}

pub fn f292(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1242 = {
        let a1239 = v1;
        let a1240 = v2;
        let a1241 = 1u64;
        slice_bytes(a1239, a1240, a1241)
    };
    match m1242 {
        None => {
            v0
        }
        Some(v3) => {
            let a1243 = v0;
            let a1244 = v3;
            append_bytes(a1243, a1244)
        }
    }
}

pub fn f293(p1247: List<Bytes>, p1248: List<Bytes>) -> List<Bytes> {
    let m1246 = (p1247, p1248);
    {
        let mut m1246 = m1246;
        loop {
            let step = {
                let (v0, v1) = m1246;
                let m1245 = v0;
                match m1245.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: Bytes, p1320: u64, p1321: u64, p1322: Bytes) -> R<Bytes> {
    let m1319 = (p1320, (p1321, p1322));
    {
        let mut m1319 = m1319;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1319;
                let m1249 = v2;
                if m1249 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1249 - 1;
                    let m1252 = {
                        let a1250 = v0.clone();
                        let a1251 = v1;
                        index_bytes(a1250, a1251)
                    };
                    match m1252 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1297 = {
                                    let a1273 = {
                                        let a1261 = {
                                            let a1255 = {
                                                let a1253 = v6;
                                                let a1254 = 128u8;
                                                fixed_u8::bit_and(a1253, a1254)
                                            };
                                            let a1256 = 128u8;
                                            if equal(a1255, a1256) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1262 = {
                                            let a1259 = {
                                                let a1257 = v6;
                                                let a1258 = 64u8;
                                                fixed_u8::bit_and(a1257, a1258)
                                            };
                                            let a1260 = 64u8;
                                            if equal(a1259, a1260) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1261, a1262)?
                                    };
                                    let a1274 = {
                                        let a1271 = {
                                            let a1265 = {
                                                let a1263 = v6;
                                                let a1264 = 32u8;
                                                fixed_u8::bit_and(a1263, a1264)
                                            };
                                            let a1266 = 32u8;
                                            if equal(a1265, a1266) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1272 = {
                                            let a1269 = {
                                                let a1267 = v6;
                                                let a1268 = 16u8;
                                                fixed_u8::bit_and(a1267, a1268)
                                            };
                                            let a1270 = 16u8;
                                            if equal(a1269, a1270) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1271, a1272)?
                                    };
                                    nat_add(a1273, a1274)?
                                };
                                let a1298 = {
                                    let a1295 = {
                                        let a1283 = {
                                            let a1277 = {
                                                let a1275 = v6;
                                                let a1276 = 8u8;
                                                fixed_u8::bit_and(a1275, a1276)
                                            };
                                            let a1278 = 8u8;
                                            if equal(a1277, a1278) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1284 = {
                                            let a1281 = {
                                                let a1279 = v6;
                                                let a1280 = 4u8;
                                                fixed_u8::bit_and(a1279, a1280)
                                            };
                                            let a1282 = 4u8;
                                            if equal(a1281, a1282) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1283, a1284)?
                                    };
                                    let a1296 = {
                                        let a1293 = {
                                            let a1287 = {
                                                let a1285 = v6;
                                                let a1286 = 2u8;
                                                fixed_u8::bit_and(a1285, a1286)
                                            };
                                            let a1288 = 2u8;
                                            if equal(a1287, a1288) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1294 = {
                                            let a1291 = {
                                                let a1289 = v6;
                                                let a1290 = 1u8;
                                                fixed_u8::bit_and(a1289, a1290)
                                            };
                                            let a1292 = 1u8;
                                            if equal(a1291, a1292) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1293, a1294)?
                                    };
                                    nat_add(a1295, a1296)?
                                };
                                nat_add(a1297, a1298)?
                            };
                            let a1299 = v7;
                            let a1300 = 34u64;
                            if if nat_eq(a1299, a1300) {
                                true
                            } else {
                                let a1301 = v7;
                                let a1302 = 92u64;
                                nat_eq(a1301, a1302)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1303 = v1;
                                    let a1304 = 1u64;
                                    nat_add(a1303, a1304)?
                                }, (v4, {
                                    let a1307 = v3;
                                    let a1308 = {
                                        let a1305 = Bytes::lit(&[0x5c]);
                                        let a1306 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1305, a1306)
                                    };
                                    append_bytes(a1307, a1308)
                                })))
                            } else {
                                let a1309 = v7;
                                let a1310 = 10u64;
                                if nat_eq(a1309, a1310) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1311 = v1;
                                        let a1312 = 1u64;
                                        nat_add(a1311, a1312)?
                                    }, (v4, {
                                        let a1313 = v3;
                                        let a1314 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1313, a1314)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1315 = v1;
                                        let a1316 = 1u64;
                                        nat_add(a1315, a1316)?
                                    }, (v4, {
                                        let a1317 = v3;
                                        let a1318 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1317, a1318)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: u32) -> Bytes {
    let a1351 = f361({
        let a1323 = f362(v0, 28u32);
        let a1324 = 15u32;
        fixed_u32::bit_and(a1323, a1324)
    });
    let a1352 = {
        let a1349 = f361({
            let a1325 = f362(v0, 24u32);
            let a1326 = 15u32;
            fixed_u32::bit_and(a1325, a1326)
        });
        let a1350 = {
            let a1347 = f361({
                let a1327 = f362(v0, 20u32);
                let a1328 = 15u32;
                fixed_u32::bit_and(a1327, a1328)
            });
            let a1348 = {
                let a1345 = f361({
                    let a1329 = f362(v0, 16u32);
                    let a1330 = 15u32;
                    fixed_u32::bit_and(a1329, a1330)
                });
                let a1346 = {
                    let a1343 = f361({
                        let a1331 = f362(v0, 12u32);
                        let a1332 = 15u32;
                        fixed_u32::bit_and(a1331, a1332)
                    });
                    let a1344 = {
                        let a1341 = f361({
                            let a1333 = f362(v0, 8u32);
                            let a1334 = 15u32;
                            fixed_u32::bit_and(a1333, a1334)
                        });
                        let a1342 = {
                            let a1339 = f361({
                                let a1335 = f362(v0, 4u32);
                                let a1336 = 15u32;
                                fixed_u32::bit_and(a1335, a1336)
                            });
                            let a1340 = f361({
                                let a1337 = v0;
                                let a1338 = 15u32;
                                fixed_u32::bit_and(a1337, a1338)
                            });
                            append_bytes(a1339, a1340)
                        };
                        append_bytes(a1341, a1342)
                    };
                    append_bytes(a1343, a1344)
                };
                append_bytes(a1345, a1346)
            };
            append_bytes(a1347, a1348)
        };
        append_bytes(a1349, a1350)
    };
    append_bytes(a1351, a1352)
}

pub fn f296(v0: Fn4, p1356: u64, p1357: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1355 = (p1356, p1357);
    {
        let mut m1355 = m1355;
        loop {
            let step = {
                let (v1, v2) = m1355;
                let m1353 = v1;
                if m1353 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1353 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1354: Fn4 = v0.clone();
                        c1354.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1359 = {
            let (h1358, _) = v3.clone();
            h1358
        };
        let a1360 = 64u64;
        nat_add(a1359, a1360)?
    }, f363({
        let (_, h1361) = v3;
        h1361
    }, f364(v0, v1, v2, {
        let (h1362, _) = v3;
        h1362
    })?)?))
}

pub fn f298() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f299(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1363 = f30(v1, 34u8)?;
    match m1363.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1364 = v3;
            match m1364.uncons() {
                None => {
                    Ok(f365(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f300(v0: List<Bytes>) -> List<Bytes> {
    let m1365 = v0;
    match m1365.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f300(v2)
            } else {
                List::cons(v1, f300(v2))
            }
        }
    }
}

pub fn f301(v0: Bytes) -> R<u64> {
    let (h1366, _) = f253(Fn3::F366(v0.clone()), {
        let a1367 = v0.clone();
        length_bytes(a1367)
    }, {
        let a1368 = v0;
        length_bytes(a1368)
    })?;
    Ok(h1366)
}

pub fn f302(v0: Adt14) -> R<Bytes> {
    let m1369 = v0;
    match m1369 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f331(v2, v3)
        }
        Adt14::C2(v4) => {
            f333(v4)
        }
        Adt14::C3(r1370) => {
            let v5 = (*r1370).clone();
            Ok(f334(f367(v5)?))
        }
        Adt14::C4(r1371) => {
            let v6 = (*r1371).clone();
            f336(f368(v6)?)
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

pub fn f303(v0: Fn5, p1376: u64, p1377: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1375 = (p1376, p1377);
    {
        let mut m1375 = m1375;
        loop {
            let step = {
                let (v1, v2) = m1375;
                let m1372 = v1;
                if m1372 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1372 - 1;
                    let m1374 = {
                        let c1373: Fn5 = v0.clone();
                        c1373.apply(v2.clone())?
                    };
                    match m1374 {
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
                Ok(next) => { m1375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f369(v0, v1)
}

pub fn f305(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f370(v0, v1)
}

pub fn f306(v0: u64) -> bool {
    let a1378 = v0;
    let a1379 = 32u64;
    if if nat_eq(a1378, a1379) {
        true
    } else {
        let a1380 = v0;
        let a1381 = 9u64;
        nat_eq(a1380, a1381)
    } {
        true
    } else {
        let a1382 = v0;
        let a1383 = 10u64;
        if nat_eq(a1382, a1383) {
            true
        } else {
            let a1384 = v0;
            let a1385 = 13u64;
            nat_eq(a1384, a1385)
        }
    }
}

pub fn f307(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1386, _) = f253(Fn3::F371(v1.clone(), v0.clone()), {
            let a1388 = {
                let a1387 = v0.clone();
                length_bytes(a1387)
            };
            let a1389 = 1u64;
            nat_add(a1388, a1389)?
        }, v2)?;
        h1386
    };
    if f219(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f308(v0: Bytes, v1: u64) -> R<u64> {
    let (h1390, _) = f253(Fn3::F372(v0.clone()), {
        let a1392 = {
            let a1391 = v0;
            length_bytes(a1391)
        };
        let a1393 = 1u64;
        nat_add(a1392, a1393)?
    }, v1)?;
    Ok(h1390)
}

pub fn f309(v0: Bytes) -> R<bool> {
    let a1395 = {
        let a1394 = v0.clone();
        length_bytes(a1394)
    };
    let a1396 = 5u64;
    if nat_eq(a1395, a1396) {
        let a1397 = f312(v0.clone(), 0u64)?;
        let a1398 = 85u64;
        if if nat_eq(a1397, a1398) {
            true
        } else {
            let a1399 = f312(v0.clone(), 0u64)?;
            let a1400 = 117u64;
            nat_eq(a1399, a1400)
        } {
            let a1401 = f312(v0.clone(), 1u64)?;
            let a1402 = 84u64;
            if if nat_eq(a1401, a1402) {
                true
            } else {
                let a1403 = f312(v0.clone(), 1u64)?;
                let a1404 = 116u64;
                nat_eq(a1403, a1404)
            } {
                let a1405 = f312(v0.clone(), 2u64)?;
                let a1406 = 70u64;
                if if nat_eq(a1405, a1406) {
                    true
                } else {
                    let a1407 = f312(v0.clone(), 2u64)?;
                    let a1408 = 102u64;
                    nat_eq(a1407, a1408)
                } {
                    let a1409 = f312(v0.clone(), 3u64)?;
                    let a1410 = 45u64;
                    if nat_eq(a1409, a1410) {
                        let a1411 = f312(v0, 4u64)?;
                        let a1412 = 56u64;
                        Ok(nat_eq(a1411, a1412))
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

pub fn f310(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1416 = f307(v0.clone(), {
        let a1413 = Str::lit("--");
        utf8_encode(a1413)
    }, {
        let a1414 = v1;
        let a1415 = 4u64;
        nat_add(a1414, a1415)?
    })?;
    match m1416 {
        None => {
            Ok(Err::<u64, Bytes>(f222(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1419 = f226(v0, {
                let a1417 = v2;
                let a1418 = 2u64;
                nat_add(a1417, a1418)?
            })?;
            let a1420 = 62u64;
            if nat_eq(a1419, a1420) {
                Ok(Ok::<u64, Bytes>({
                    let a1421 = v2;
                    let a1422 = 3u64;
                    nat_add(a1421, a1422)?
                }))
            } else {
                Ok(Err::<u64, Bytes>(f222(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f311(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f313(v0.clone(), {
        let a1423 = v1;
        let a1424 = 2u64;
        nat_add(a1423, a1424)?
    })?;
    let a1427 = v2;
    let a1428 = {
        let a1425 = v1;
        let a1426 = 2u64;
        nat_add(a1425, a1426)?
    };
    if nat_eq(a1427, a1428) {
        Ok(Err::<u64, Bytes>(f222(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1431 = v2;
        let a1432 = {
            let a1429 = v1;
            let a1430 = 5u64;
            nat_add(a1429, a1430)?
        };
        if if nat_eq(a1431, a1432) {
            let a1435 = f226(v0.clone(), {
                let a1433 = v1;
                let a1434 = 2u64;
                nat_add(a1433, a1434)?
            })?;
            let a1436 = 120u64;
            if if nat_eq(a1435, a1436) {
                true
            } else {
                let a1439 = f226(v0.clone(), {
                    let a1437 = v1;
                    let a1438 = 2u64;
                    nat_add(a1437, a1438)?
                })?;
                let a1440 = 88u64;
                nat_eq(a1439, a1440)
            } {
                let a1443 = f226(v0.clone(), {
                    let a1441 = v1;
                    let a1442 = 3u64;
                    nat_add(a1441, a1442)?
                })?;
                let a1444 = 109u64;
                if if nat_eq(a1443, a1444) {
                    true
                } else {
                    let a1447 = f226(v0.clone(), {
                        let a1445 = v1;
                        let a1446 = 3u64;
                        nat_add(a1445, a1446)?
                    })?;
                    let a1448 = 77u64;
                    nat_eq(a1447, a1448)
                } {
                    let a1451 = f226(v0.clone(), {
                        let a1449 = v1;
                        let a1450 = 4u64;
                        nat_add(a1449, a1450)?
                    })?;
                    let a1452 = 108u64;
                    if nat_eq(a1451, a1452) {
                        true
                    } else {
                        let a1455 = f226(v0.clone(), {
                            let a1453 = v1;
                            let a1454 = 4u64;
                            nat_add(a1453, a1454)?
                        })?;
                        let a1456 = 76u64;
                        nat_eq(a1455, a1456)
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
            Ok(Err::<u64, Bytes>(f222(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1458 = f307(v0, {
                let a1457 = Str::lit("?>");
                utf8_encode(a1457)
            }, v2)?;
            match m1458 {
                None => {
                    Ok(Err::<u64, Bytes>(f222(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1459 = v3;
                        let a1460 = 2u64;
                        nat_add(a1459, a1460)?
                    }))
                }
            }
        }
    }
}

pub fn f312(v0: Bytes, v1: u64) -> R<u64> {
    let m1463 = {
        let a1461 = v0;
        let a1462 = v1;
        index_bytes(a1461, a1462)
    };
    match m1463 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1508 = {
                let a1484 = {
                    let a1472 = {
                        let a1466 = {
                            let a1464 = v3;
                            let a1465 = 128u8;
                            fixed_u8::bit_and(a1464, a1465)
                        };
                        let a1467 = 128u8;
                        if equal(a1466, a1467) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1473 = {
                        let a1470 = {
                            let a1468 = v3;
                            let a1469 = 64u8;
                            fixed_u8::bit_and(a1468, a1469)
                        };
                        let a1471 = 64u8;
                        if equal(a1470, a1471) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1472, a1473)?
                };
                let a1485 = {
                    let a1482 = {
                        let a1476 = {
                            let a1474 = v3;
                            let a1475 = 32u8;
                            fixed_u8::bit_and(a1474, a1475)
                        };
                        let a1477 = 32u8;
                        if equal(a1476, a1477) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1483 = {
                        let a1480 = {
                            let a1478 = v3;
                            let a1479 = 16u8;
                            fixed_u8::bit_and(a1478, a1479)
                        };
                        let a1481 = 16u8;
                        if equal(a1480, a1481) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1482, a1483)?
                };
                nat_add(a1484, a1485)?
            };
            let a1509 = {
                let a1506 = {
                    let a1494 = {
                        let a1488 = {
                            let a1486 = v3;
                            let a1487 = 8u8;
                            fixed_u8::bit_and(a1486, a1487)
                        };
                        let a1489 = 8u8;
                        if equal(a1488, a1489) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1495 = {
                        let a1492 = {
                            let a1490 = v3;
                            let a1491 = 4u8;
                            fixed_u8::bit_and(a1490, a1491)
                        };
                        let a1493 = 4u8;
                        if equal(a1492, a1493) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1494, a1495)?
                };
                let a1507 = {
                    let a1504 = {
                        let a1498 = {
                            let a1496 = v3;
                            let a1497 = 2u8;
                            fixed_u8::bit_and(a1496, a1497)
                        };
                        let a1499 = 2u8;
                        if equal(a1498, a1499) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1505 = {
                        let a1502 = {
                            let a1500 = v3;
                            let a1501 = 1u8;
                            fixed_u8::bit_and(a1500, a1501)
                        };
                        let a1503 = 1u8;
                        if equal(a1502, a1503) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1504, a1505)?
                };
                nat_add(a1506, a1507)?
            };
            nat_add(a1508, a1509)
        }
    }
}

pub fn f313(v0: Bytes, v1: u64) -> R<u64> {
    let (h1510, _) = f253(Fn3::F373(v0.clone(), v1), {
        let a1512 = {
            let a1511 = v0;
            length_bytes(a1511)
        };
        let a1513 = 1u64;
        nat_add(a1512, a1513)?
    }, v1)?;
    Ok(h1510)
}

pub fn f314(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt18, u64), Bytes>> {
    let m1514 = v2;
    if m1514 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1514 - 1;
        let v5: u64 = f308(v0.clone(), v1)?;
        let a1515 = f226(v0.clone(), v5)?;
        let a1516 = 62u64;
        if if nat_eq(a1515, a1516) {
            true
        } else {
            f225(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v5)))
        } else {
            let a1517 = v5;
            let a1518 = v1;
            if nat_eq(a1517, a1518) {
                Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f313(v0.clone(), v5)?;
                let a1519 = v6;
                let a1520 = v5;
                if nat_eq(a1519, a1520) {
                    Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a1521 = v6;
                        let a1522 = v5;
                        nat_sub(a1521, a1522)
                    });
                    if f374(v7.clone(), v3.clone()) {
                        Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f308(v0.clone(), v6)?;
                        let a1523 = f226(v0.clone(), v8)?;
                        let a1524 = 61u64;
                        if !nat_eq(a1523, a1524) {
                            Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        } else {
                            let v9: u64 = f308(v0.clone(), {
                                let a1525 = v8;
                                let a1526 = 1u64;
                                nat_add(a1525, a1526)?
                            })?;
                            let v10: u64 = f226(v0.clone(), v9)?;
                            let a1527 = v10;
                            let a1528 = 34u64;
                            if !if nat_eq(a1527, a1528) {
                                true
                            } else {
                                let a1529 = v10;
                                let a1530 = 39u64;
                                nat_eq(a1529, a1530)
                            } {
                                Ok(Err::<(Adt18, u64), Bytes>(f222(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            } else {
                                let m1533 = f315(v0.clone(), {
                                    let a1531 = v9;
                                    let a1532 = 1u64;
                                    nat_add(a1531, a1532)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1533 {
                                    Ok(v12) => {
                                        let m1535 = f314(v0, {
                                            let (_, h1534) = v12;
                                            h1534
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1535 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C1(v7, {
                                                    let (h1536, _) = v12;
                                                    h1536
                                                }, std::rc::Rc::new({
                                                    let (h1537, _) = v14;
                                                    h1537
                                                })), {
                                                    let (_, h1538) = v14;
                                                    h1538
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

pub fn f315(v0: Bytes, p1578: u64, p1579: u64, p1580: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1577 = (p1578, (p1579, p1580));
    {
        let mut m1577 = m1577;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1577;
                let m1539 = v2;
                if m1539 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f222(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1539 - 1;
                    let v7: u64 = f226(v0.clone(), v1)?;
                    if if v4 {
                        let a1540 = v7;
                        let a1541 = v5;
                        nat_eq(a1540, a1541)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1542 = v1;
                            let a1543 = 1u64;
                            nat_add(a1542, a1543)?
                        }))))
                    } else {
                        let a1544 = v7;
                        let a1545 = 256u64;
                        if nat_eq(a1544, a1545) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f222(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1546 = v7;
                            let a1547 = 60u64;
                            if nat_eq(a1546, a1547) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f222(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1548 = v7;
                                let a1549 = 38u64;
                                if nat_eq(a1548, a1549) {
                                    let m1550 = f375(v0.clone(), v1)?;
                                    match m1550 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1551) = v9;
                                                h1551
                                            }, (v6, {
                                                let a1553 = v3;
                                                let a1554 = {
                                                    let (h1552, _) = v9;
                                                    h1552
                                                };
                                                append_bytes(a1553, a1554)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1555 = v7;
                                        let a1556 = 9u64;
                                        if if nat_eq(a1555, a1556) {
                                            true
                                        } else {
                                            let a1557 = v7;
                                            let a1558 = 10u64;
                                            nat_eq(a1557, a1558)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1559 = v1;
                                                let a1560 = 1u64;
                                                nat_add(a1559, a1560)?
                                            }, (v6, {
                                                let a1561 = v3;
                                                let a1562 = Bytes::lit(&[0x20]);
                                                append_bytes(a1561, a1562)
                                            })))
                                        } else {
                                            let v10: u64 = f376(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1565 = v3;
                                                let a1566 = f32(v0.clone(), v1, {
                                                    let a1563 = v10;
                                                    let a1564 = v1;
                                                    nat_sub(a1563, a1564)
                                                });
                                                append_bytes(a1565, a1566)
                                            })))
                                        }
                                    } else {
                                        let a1567 = v7;
                                        let a1568 = 93u64;
                                        if nat_eq(a1567, a1568) {
                                            if f225(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f222(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1569 = v1;
                                                    let a1570 = 1u64;
                                                    nat_add(a1569, a1570)?
                                                }, (v6, {
                                                    let a1571 = v3;
                                                    let a1572 = Bytes::lit(&[0x5d]);
                                                    append_bytes(a1571, a1572)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f376(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1575 = v3;
                                                let a1576 = f32(v0.clone(), v1, {
                                                    let a1573 = v11;
                                                    let a1574 = v1;
                                                    nat_sub(a1573, a1574)
                                                });
                                                append_bytes(a1575, a1576)
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
                Ok(next) => { m1577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: Bytes, v1: u64) -> Bytes {
    let m1584 = {
        let a1581 = v0;
        let a1582 = v1;
        let a1583 = 1u64;
        slice_bytes(a1581, a1582, a1583)
    };
    match m1584 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f317(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f377(v1, v0, v2)
}

pub fn f318(v0: Bytes, v1: u64) -> R<u64> {
    let (h1585, _) = f253(Fn3::F378(v1, v0), v1, 0u64)?;
    Ok(h1585)
}

pub fn f319(v0: Bytes) -> R<Bytes> {
    f379(v0.clone(), 0u64, {
        let a1586 = v0;
        length_bytes(a1586)
    }, false, false, Bytes::lit(&[]))
}

pub fn f320(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1587 = v2;
    if m1587 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1587 - 1;
        let v5: u64 = f312(v0.clone(), v1)?;
        let a1588 = v3;
        let a1589 = 1u64;
        if nat_eq(a1588, a1589) {
            let m1590 = f320(v0.clone(), v1, v4, 0u64)?;
            match m1590 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f312(v0.clone(), {
                        let (_, h1591) = v6.clone();
                        h1591
                    })?;
                    let a1592 = v7;
                    let a1593 = 44u64;
                    if nat_eq(a1592, a1593) {
                        let m1597 = f320(v0, {
                            let a1595 = {
                                let (_, h1594) = v6;
                                h1594
                            };
                            let a1596 = 1u64;
                            nat_add(a1595, a1596)?
                        }, v4, 1u64)?;
                        match m1597 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1598, _) = v6;
                                    h1598
                                }), std::rc::Rc::new({
                                    let (h1599, _) = v8;
                                    h1599
                                })), {
                                    let (_, h1600) = v8;
                                    h1600
                                })))
                            }
                        }
                    } else {
                        let a1601 = v7;
                        let a1602 = 93u64;
                        if nat_eq(a1601, a1602) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1603, _) = v6;
                                h1603
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1605 = {
                                    let (_, h1604) = v6;
                                    h1604
                                };
                                let a1606 = 1u64;
                                nat_add(a1605, a1606)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1607 = v3;
            let a1608 = 2u64;
            if nat_eq(a1607, a1608) {
                let m1611 = f380(v0.clone(), {
                    let a1609 = v1;
                    let a1610 = 1u64;
                    nat_add(a1609, a1610)?
                })?;
                match m1611 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1612 = v5;
                        let a1613 = 34u64;
                        if if nat_eq(a1612, a1613) {
                            let a1615 = f312(v0.clone(), {
                                let (h1614, _) = v9.clone();
                                h1614
                            })?;
                            let a1616 = 58u64;
                            nat_eq(a1615, a1616)
                        } else {
                            false
                        } {
                            let m1620 = f320(v0.clone(), {
                                let a1618 = {
                                    let (h1617, _) = v9;
                                    h1617
                                };
                                let a1619 = 1u64;
                                nat_add(a1618, a1619)?
                            }, v4, 0u64)?;
                            match m1620 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f312(v0.clone(), {
                                        let (_, h1621) = v10.clone();
                                        h1621
                                    })?;
                                    let a1622 = v11;
                                    let a1623 = 44u64;
                                    if nat_eq(a1622, a1623) {
                                        let m1627 = f320(v0, {
                                            let a1625 = {
                                                let (_, h1624) = v10;
                                                h1624
                                            };
                                            let a1626 = 1u64;
                                            nat_add(a1625, a1626)?
                                        }, v4, 2u64)?;
                                        match m1627 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1628) = v9;
                                                    h1628
                                                }, std::rc::Rc::new({
                                                    let (h1629, _) = v10;
                                                    h1629
                                                }), std::rc::Rc::new({
                                                    let (h1630, _) = v12;
                                                    h1630
                                                })), {
                                                    let (_, h1631) = v12;
                                                    h1631
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1632 = v11;
                                        let a1633 = 125u64;
                                        if nat_eq(a1632, a1633) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1634) = v9;
                                                h1634
                                            }, std::rc::Rc::new({
                                                let (h1635, _) = v10;
                                                h1635
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1637 = {
                                                    let (_, h1636) = v10;
                                                    h1636
                                                };
                                                let a1638 = 1u64;
                                                nat_add(a1637, a1638)?
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
                let a1639 = v5;
                let a1640 = 34u64;
                if nat_eq(a1639, a1640) {
                    let m1643 = f380(v0, {
                        let a1641 = v1;
                        let a1642 = 1u64;
                        nat_add(a1641, a1642)?
                    })?;
                    match m1643 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1644) = v13;
                                h1644
                            }), {
                                let (h1645, _) = v13;
                                h1645
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1646 = v1;
                            let a1647 = 4u64;
                            nat_add(a1646, a1647)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1648 = v1;
                                let a1649 = 5u64;
                                nat_add(a1648, a1649)?
                            })))
                        } else {
                            let a1650 = v5;
                            let a1651 = 91u64;
                            if nat_eq(a1650, a1651) {
                                let a1654 = f312(v0.clone(), {
                                    let a1652 = v1;
                                    let a1653 = 1u64;
                                    nat_add(a1652, a1653)?
                                })?;
                                let a1655 = 93u64;
                                if nat_eq(a1654, a1655) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1656 = v1;
                                        let a1657 = 2u64;
                                        nat_add(a1656, a1657)?
                                    })))
                                } else {
                                    let m1660 = f320(v0, {
                                        let a1658 = v1;
                                        let a1659 = 1u64;
                                        nat_add(a1658, a1659)?
                                    }, v4, 1u64)?;
                                    match m1660 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1661, _) = v14;
                                                h1661
                                            })), {
                                                let (_, h1662) = v14;
                                                h1662
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1663 = v5;
                                let a1664 = 123u64;
                                if nat_eq(a1663, a1664) {
                                    let a1667 = f312(v0.clone(), {
                                        let a1665 = v1;
                                        let a1666 = 1u64;
                                        nat_add(a1665, a1666)?
                                    })?;
                                    let a1668 = 125u64;
                                    if nat_eq(a1667, a1668) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1669 = v1;
                                            let a1670 = 2u64;
                                            nat_add(a1669, a1670)?
                                        })))
                                    } else {
                                        let m1673 = f320(v0, {
                                            let a1671 = v1;
                                            let a1672 = 1u64;
                                            nat_add(a1671, a1672)?
                                        }, v4, 2u64)?;
                                        match m1673 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1674, _) = v15;
                                                    h1674
                                                })), {
                                                    let (_, h1675) = v15;
                                                    h1675
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1676 = f381(v0, v1)?;
                                    match m1676 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1677) = v16;
                                                h1677
                                            }, {
                                                let (h1678, _) = v16;
                                                h1678
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

pub fn f321() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f322(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1679 = f159(v0, v1);
    match m1679 {
        None => {
            false
        }
        Some(v3) => {
            f202(v3, v2)
        }
    }
}

pub fn f323() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f324(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt19, u64)>> {
    let m1680 = v2;
    if m1680 == 0 {
        Ok(None::<(Adt19, u64)>)
    } else {
        let v4 = m1680 - 1;
        let v5: u64 = f312(v0.clone(), v1)?;
        let a1681 = v3;
        let a1682 = 1u64;
        if nat_eq(a1681, a1682) {
            let m1683 = f324(v0.clone(), v1, v4, 0u64)?;
            match m1683 {
                None => {
                    Ok(None::<(Adt19, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f312(v0.clone(), {
                        let (_, h1684) = v6.clone();
                        h1684
                    })?;
                    let a1685 = v7;
                    let a1686 = 44u64;
                    if nat_eq(a1685, a1686) {
                        let m1690 = f324(v0, {
                            let a1688 = {
                                let (_, h1687) = v6;
                                h1687
                            };
                            let a1689 = 1u64;
                            nat_add(a1688, a1689)?
                        }, v4, 1u64)?;
                        match m1690 {
                            None => {
                                Ok(None::<(Adt19, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt19::C7(std::rc::Rc::new({
                                    let (h1691, _) = v6;
                                    h1691
                                }), std::rc::Rc::new({
                                    let (h1692, _) = v8;
                                    h1692
                                })), {
                                    let (_, h1693) = v8;
                                    h1693
                                })))
                            }
                        }
                    } else {
                        let a1694 = v7;
                        let a1695 = 93u64;
                        if nat_eq(a1694, a1695) {
                            Ok(Some((Adt19::C7(std::rc::Rc::new({
                                let (h1696, _) = v6;
                                h1696
                            }), std::rc::Rc::new(Adt19::C6)), {
                                let a1698 = {
                                    let (_, h1697) = v6;
                                    h1697
                                };
                                let a1699 = 1u64;
                                nat_add(a1698, a1699)?
                            })))
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1700 = v3;
            let a1701 = 2u64;
            if nat_eq(a1700, a1701) {
                let m1704 = f380(v0.clone(), {
                    let a1702 = v1;
                    let a1703 = 1u64;
                    nat_add(a1702, a1703)?
                })?;
                match m1704 {
                    None => {
                        Ok(None::<(Adt19, u64)>)
                    }
                    Some(v9) => {
                        let a1705 = v5;
                        let a1706 = 34u64;
                        if if nat_eq(a1705, a1706) {
                            let a1708 = f312(v0.clone(), {
                                let (h1707, _) = v9.clone();
                                h1707
                            })?;
                            let a1709 = 58u64;
                            nat_eq(a1708, a1709)
                        } else {
                            false
                        } {
                            let m1713 = f324(v0.clone(), {
                                let a1711 = {
                                    let (h1710, _) = v9;
                                    h1710
                                };
                                let a1712 = 1u64;
                                nat_add(a1711, a1712)?
                            }, v4, 0u64)?;
                            match m1713 {
                                None => {
                                    Ok(None::<(Adt19, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f312(v0.clone(), {
                                        let (_, h1714) = v10.clone();
                                        h1714
                                    })?;
                                    let a1715 = v11;
                                    let a1716 = 44u64;
                                    if nat_eq(a1715, a1716) {
                                        let m1720 = f324(v0, {
                                            let a1718 = {
                                                let (_, h1717) = v10;
                                                h1717
                                            };
                                            let a1719 = 1u64;
                                            nat_add(a1718, a1719)?
                                        }, v4, 2u64)?;
                                        match m1720 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt19::C8({
                                                    let (_, h1721) = v9;
                                                    h1721
                                                }, std::rc::Rc::new({
                                                    let (h1722, _) = v10;
                                                    h1722
                                                }), std::rc::Rc::new({
                                                    let (h1723, _) = v12;
                                                    h1723
                                                })), {
                                                    let (_, h1724) = v12;
                                                    h1724
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1725 = v11;
                                        let a1726 = 125u64;
                                        if nat_eq(a1725, a1726) {
                                            Ok(Some((Adt19::C8({
                                                let (_, h1727) = v9;
                                                h1727
                                            }, std::rc::Rc::new({
                                                let (h1728, _) = v10;
                                                h1728
                                            }), std::rc::Rc::new(Adt19::C6)), {
                                                let a1730 = {
                                                    let (_, h1729) = v10;
                                                    h1729
                                                };
                                                let a1731 = 1u64;
                                                nat_add(a1730, a1731)?
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
                let a1732 = v5;
                let a1733 = 34u64;
                if nat_eq(a1732, a1733) {
                    let m1736 = f380(v0, {
                        let a1734 = v1;
                        let a1735 = 1u64;
                        nat_add(a1734, a1735)?
                    })?;
                    match m1736 {
                        None => {
                            Ok(None::<(Adt19, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt19::C3({
                                let (_, h1737) = v13;
                                h1737
                            }), {
                                let (h1738, _) = v13;
                                h1738
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt19::C0(true), {
                            let a1739 = v1;
                            let a1740 = 4u64;
                            nat_add(a1739, a1740)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt19::C0(false), {
                                let a1741 = v1;
                                let a1742 = 5u64;
                                nat_add(a1741, a1742)?
                            })))
                        } else {
                            let a1743 = v5;
                            let a1744 = 91u64;
                            if nat_eq(a1743, a1744) {
                                let a1747 = f312(v0.clone(), {
                                    let a1745 = v1;
                                    let a1746 = 1u64;
                                    nat_add(a1745, a1746)?
                                })?;
                                let a1748 = 93u64;
                                if nat_eq(a1747, a1748) {
                                    Ok(Some((Adt19::C4(std::rc::Rc::new(Adt19::C6)), {
                                        let a1749 = v1;
                                        let a1750 = 2u64;
                                        nat_add(a1749, a1750)?
                                    })))
                                } else {
                                    let m1753 = f324(v0, {
                                        let a1751 = v1;
                                        let a1752 = 1u64;
                                        nat_add(a1751, a1752)?
                                    }, v4, 1u64)?;
                                    match m1753 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt19::C4(std::rc::Rc::new({
                                                let (h1754, _) = v14;
                                                h1754
                                            })), {
                                                let (_, h1755) = v14;
                                                h1755
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1756 = v5;
                                let a1757 = 123u64;
                                if nat_eq(a1756, a1757) {
                                    let a1760 = f312(v0.clone(), {
                                        let a1758 = v1;
                                        let a1759 = 1u64;
                                        nat_add(a1758, a1759)?
                                    })?;
                                    let a1761 = 125u64;
                                    if nat_eq(a1760, a1761) {
                                        Ok(Some((Adt19::C5(std::rc::Rc::new(Adt19::C6)), {
                                            let a1762 = v1;
                                            let a1763 = 2u64;
                                            nat_add(a1762, a1763)?
                                        })))
                                    } else {
                                        let m1766 = f324(v0, {
                                            let a1764 = v1;
                                            let a1765 = 1u64;
                                            nat_add(a1764, a1765)?
                                        }, v4, 2u64)?;
                                        match m1766 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt19::C5(std::rc::Rc::new({
                                                    let (h1767, _) = v15;
                                                    h1767
                                                })), {
                                                    let (_, h1768) = v15;
                                                    h1768
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1769 = f382(v0, v1)?;
                                    match m1769 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1770) = v16;
                                                h1770
                                            }, {
                                                let (h1771, _) = v16;
                                                h1771
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

pub fn f325(v0: Adt19, v1: Bytes) -> bool {
    let a1773 = {
        let a1772 = f150(v0, v1);
        length_bytes(a1772)
    };
    let a1774 = 0u64;
    !nat_eq(a1773, a1774)
}

pub fn f326(v0: Adt19, v1: Bytes) -> bool {
    let m1775 = f147(v0, v1);
    match m1775 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f327(v0: Bytes, p1842: u64, p1843: u64, p1844: bool, p1845: Bytes) -> R<Bytes> {
    let m1841 = (p1842, (p1843, (p1844, p1845)));
    {
        let mut m1841 = m1841;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1841;
                let m1776 = v2;
                if m1776 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1776 - 1;
                    let m1779 = {
                        let a1777 = v0.clone();
                        let a1778 = v1;
                        index_bytes(a1777, a1778)
                    };
                    match m1779 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1824 = {
                                    let a1800 = {
                                        let a1788 = {
                                            let a1782 = {
                                                let a1780 = v7;
                                                let a1781 = 128u8;
                                                fixed_u8::bit_and(a1780, a1781)
                                            };
                                            let a1783 = 128u8;
                                            if equal(a1782, a1783) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1789 = {
                                            let a1786 = {
                                                let a1784 = v7;
                                                let a1785 = 64u8;
                                                fixed_u8::bit_and(a1784, a1785)
                                            };
                                            let a1787 = 64u8;
                                            if equal(a1786, a1787) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1788, a1789)?
                                    };
                                    let a1801 = {
                                        let a1798 = {
                                            let a1792 = {
                                                let a1790 = v7;
                                                let a1791 = 32u8;
                                                fixed_u8::bit_and(a1790, a1791)
                                            };
                                            let a1793 = 32u8;
                                            if equal(a1792, a1793) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1799 = {
                                            let a1796 = {
                                                let a1794 = v7;
                                                let a1795 = 16u8;
                                                fixed_u8::bit_and(a1794, a1795)
                                            };
                                            let a1797 = 16u8;
                                            if equal(a1796, a1797) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1798, a1799)?
                                    };
                                    nat_add(a1800, a1801)?
                                };
                                let a1825 = {
                                    let a1822 = {
                                        let a1810 = {
                                            let a1804 = {
                                                let a1802 = v7;
                                                let a1803 = 8u8;
                                                fixed_u8::bit_and(a1802, a1803)
                                            };
                                            let a1805 = 8u8;
                                            if equal(a1804, a1805) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1811 = {
                                            let a1808 = {
                                                let a1806 = v7;
                                                let a1807 = 4u8;
                                                fixed_u8::bit_and(a1806, a1807)
                                            };
                                            let a1809 = 4u8;
                                            if equal(a1808, a1809) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1810, a1811)?
                                    };
                                    let a1823 = {
                                        let a1820 = {
                                            let a1814 = {
                                                let a1812 = v7;
                                                let a1813 = 2u8;
                                                fixed_u8::bit_and(a1812, a1813)
                                            };
                                            let a1815 = 2u8;
                                            if equal(a1814, a1815) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1821 = {
                                            let a1818 = {
                                                let a1816 = v7;
                                                let a1817 = 1u8;
                                                fixed_u8::bit_and(a1816, a1817)
                                            };
                                            let a1819 = 1u8;
                                            if equal(a1818, a1819) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1820, a1821)?
                                    };
                                    nat_add(a1822, a1823)?
                                };
                                nat_add(a1824, a1825)?
                            };
                            if f383(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1826 = v1;
                                    let a1827 = 1u64;
                                    nat_add(a1826, a1827)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f384(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1828 = v1;
                                        let a1829 = 1u64;
                                        nat_add(a1828, a1829)?
                                    }, (v5, (false, {
                                        let a1833 = v4;
                                        let a1834 = f32({
                                            let a1830 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1830)
                                        }, {
                                            let a1831 = v8;
                                            let a1832 = 97u64;
                                            nat_sub(a1831, a1832)
                                        }, 1u64);
                                        append_bytes(a1833, a1834)
                                    }))))
                                } else {
                                    if if f384(v8) {
                                        true
                                    } else {
                                        if f385(v8) {
                                            true
                                        } else {
                                            f386(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1835 = v1;
                                            let a1836 = 1u64;
                                            nat_add(a1835, a1836)?
                                        }, (v5, (false, {
                                            let a1837 = v4;
                                            let a1838 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1837, a1838)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1839 = v1;
                                            let a1840 = 1u64;
                                            nat_add(a1839, a1840)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328(v0: Adt19, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt19)> = f387(f388(f147(v0.clone(), f8(Str::lit("$defs")))), f388(f147(v0.clone(), f8(Str::lit("definitions")))));
    let m1846 = f389(4096u64, v3.clone(), List::<Bytes>::nil(), f390(v3)?)?;
    match m1846 {
        Ok(v5) => {
            let m1847 = f391(4096u64, f392(f393(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1847 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f214(f185(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f329(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1848 = v0;
    match m1848.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f36(f157(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f394(v5, v1, v2))
            } else {
                List::cons(v4, f329(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f330(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f214(f395(f83(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f331(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1849 = 0u64;
        let a1850 = v1;
        nat_lt(a1849, a1850)
    } else {
        false
    } {
        let a1851 = Bytes::lit(&[0x2d]);
        let a1852 = f126(v1)?;
        Ok(append_bytes(a1851, a1852))
    } else {
        f126(v1)
    }
}

pub fn f332(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f126(v1)?;
    let v4: u64 = {
        let a1853 = v3.clone();
        length_bytes(a1853)
    };
    let v5: Bytes = {
        let a1854 = v2;
        let a1855 = v4;
        if nat_lt(a1854, a1855) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1856 = v4;
                let a1857 = v2;
                nat_sub(a1856, a1857)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1858 = v4;
                let a1859 = v2;
                nat_sub(a1858, a1859)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f396({
                let a1860 = v2;
                let a1861 = v4;
                nat_sub(a1860, a1861)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1862 = f8(Str::lit("-"));
        let a1863 = v5;
        Ok(append_bytes(a1862, a1863))
    } else {
        Ok(v5)
    }
}

pub fn f333(v0: Bytes) -> R<Bytes> {
    let a1869 = {
        let a1867 = Bytes::lit(&[0x22]);
        let a1868 = {
            let (_, h1864) = {
                let (h1865, _) = f303(Fn5::F397(v0.clone()), {
                    let a1866 = v0;
                    length_bytes(a1866)
                }, (0u64, Bytes::lit(&[])))?;
                h1865
            };
            h1864
        };
        append_bytes(a1867, a1868)
    };
    let a1870 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1869, a1870))
}

pub fn f334(v0: List<Bytes>) -> Bytes {
    let a1873 = {
        let a1871 = Bytes::lit(&[0x5b]);
        let a1872 = f121(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1871, a1872)
    };
    let a1874 = Bytes::lit(&[0x5d]);
    append_bytes(a1873, a1874)
}

pub fn f335(v0: Adt19) -> R<List<Bytes>> {
    let m1875 = v0;
    match m1875 {
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
        Adt19::C7(r1876, r1877) => {
            let v10 = (*r1876).clone();
            let v11 = (*r1877).clone();
            Ok(List::cons(f244(v10)?, f335(v11)?))
        }
        Adt19::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f336(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f398(Fn10::F399, List::<(Bytes, Bytes)>::nil(), f400(v0)?);
    let v2: List<Bytes> = f401(Fn11::F402, List::<Bytes>::nil(), v1)?;
    let a1880 = {
        let a1878 = Bytes::lit(&[0x7b]);
        let a1879 = f121(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1878, a1879)
    };
    let a1881 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1880, a1881))
}

pub fn f337(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m1882 = v0;
    match m1882 {
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
        Adt19::C8(v12, r1883, r1884) => {
            let v13 = (*r1883).clone();
            let v14 = (*r1884).clone();
            Ok(List::cons((v12, f244(v13)?), f337(v14)?))
        }
    }
}

pub fn f338(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1885 = v0;
    match m1885 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f157(v2, v1)
        }
    }
}

pub fn f339(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f214(f403(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f340(v0: Adt16, v1: Option<u64>) -> bool {
    let m1886 = v1;
    match m1886 {
        None => {
            true
        }
        Some(v2) => {
            f262(v2, match v0 {
                Adt16::C0(_, h1887, _, _) => {
                    h1887
                }
            })
        }
    }
}

pub fn f341(v0: Adt12) -> Adt21 {
    Adt21::C1({
        let a1889 = match v0 {
            Adt12::C0(_, _, h1888, _, _) => {
                h1888
            }
        };
        length_list(a1889)
    })
}

pub fn f342(v0: Bytes) -> Option<Adt22> {
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

pub fn f343(v0: Bytes, p1894: List<Bytes>, p1895: u64) -> R<Option<u64>> {
    let m1893 = (p1894, p1895);
    {
        let mut m1893 = m1893;
        loop {
            let step = {
                let (v1, v2) = m1893;
                let m1890 = v1;
                match m1890.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1891 = v2;
                                let a1892 = 1u64;
                                nat_add(a1891, a1892)?
                            }))
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

pub fn f344(v0: Option<Adt14>) -> Option<u64> {
    let m1896 = v0;
    match m1896 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1897 = v1;
            match m1897 {
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

pub fn f345(v0: Fn6, p1901: List<Bytes>, p1902: List<(Bytes, ())>) -> List<Bytes> {
    let m1900 = (p1901, p1902);
    {
        let mut m1900 = m1900;
        loop {
            let step = {
                let (v1, v2) = m1900;
                let m1898 = v2;
                match m1898.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1899: Fn6 = v0.clone();
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

pub fn f346(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1903, _) = v1;
        h1903
    }, v0)
}

pub fn f347(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f404(Fn12::F405, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f348(v0: Fn7, p1907: List<List<(Bytes, ())>>, p1908: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1906 = (p1907, p1908);
    {
        let mut m1906 = m1906;
        loop {
            let step = {
                let (v1, v2) = m1906;
                let m1904 = v2;
                match m1904.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1905: Fn7 = v0.clone();
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

pub fn f349(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f350(v0: Fn8, p1913: u64, p1914: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1912 = (p1913, p1914);
    {
        let mut m1912 = m1912;
        loop {
            let step = {
                let (v1, v2) = m1912;
                let m1909 = v1;
                if m1909 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1909 - 1;
                    let m1911 = {
                        let c1910: Fn8 = v0.clone();
                        c1910.apply(v2.clone())?
                    };
                    match m1911 {
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
                Ok(next) => { m1912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f406(v0)
}

pub fn f352(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f407(Fn13::F408, List::<(Bytes, ())>::nil(), v0)
}

pub fn f353(v0: Fn9, p1918: List<(Bytes, ())>, p1919: List<Bytes>) -> List<(Bytes, ())> {
    let m1917 = (p1918, p1919);
    {
        let mut m1917 = m1917;
        loop {
            let step = {
                let (v1, v2) = m1917;
                let m1915 = v2;
                match m1915.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1916: Fn9 = v0.clone();
                            c1916.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f355(v0: Bytes) -> Bytes {
    if f134(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1921 = {
                let a1920 = v0;
                length_bytes(a1920)
            };
            let a1922 = 1u64;
            nat_sub(a1921, a1922)
        })
    } else {
        v0
    }
}

pub fn f356(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f83(f70(v0.clone(), v1.clone()))?;
    let m1923 = f153(v2.clone())?;
    match m1923 {
        None => {
            f410(v0, v1, v2)
        }
        Some(v3) => {
            f409(v0, v1, v3)
        }
    }
}

pub fn f357(v0: List<Bytes>) -> List<Bytes> {
    let m1924 = v0;
    match m1924.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1926 = {
                    let a1925 = v1.clone();
                    length_bytes(a1925)
                };
                let a1927 = 0u64;
                if nat_eq(a1926, a1927) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f357(v2))
        }
    }
}

pub fn f358(v0: List<Bytes>) -> Bytes {
    let m1928 = v0;
    match m1928.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f359(v0: (Bytes, Bytes), p1931: List<(Bytes, Bytes)>) -> bool {
    let m1930 = p1931;
    {
        let mut m1930 = m1930;
        loop {
            let step = {
                let v1 = m1930;
                let m1929 = v1;
                match m1929.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f411(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
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

pub fn f360(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1932 = v1;
    match m1932.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1933, _) = v2;
                h1933
            }, v0.clone()) {
                List::cons({
                    let (_, h1934) = v2;
                    h1934
                }, f360(v0, v3))
            } else {
                f360(v0, v3)
            }
        }
    }
}

pub fn f361(v0: u32) -> Bytes {
    let a1937 = {
        let a1935 = v0;
        let a1936 = 8u32;
        fixed_u32::bit_and(a1935, a1936)
    };
    let a1938 = 0u32;
    if equal(a1937, a1938) {
        let a1941 = {
            let a1939 = v0;
            let a1940 = 4u32;
            fixed_u32::bit_and(a1939, a1940)
        };
        let a1942 = 0u32;
        if equal(a1941, a1942) {
            let a1945 = {
                let a1943 = v0;
                let a1944 = 2u32;
                fixed_u32::bit_and(a1943, a1944)
            };
            let a1946 = 0u32;
            if equal(a1945, a1946) {
                let a1949 = {
                    let a1947 = v0;
                    let a1948 = 1u32;
                    fixed_u32::bit_and(a1947, a1948)
                };
                let a1950 = 0u32;
                if equal(a1949, a1950) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1953 = {
                    let a1951 = v0;
                    let a1952 = 1u32;
                    fixed_u32::bit_and(a1951, a1952)
                };
                let a1954 = 0u32;
                if equal(a1953, a1954) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1957 = {
                let a1955 = v0;
                let a1956 = 2u32;
                fixed_u32::bit_and(a1955, a1956)
            };
            let a1958 = 0u32;
            if equal(a1957, a1958) {
                let a1961 = {
                    let a1959 = v0;
                    let a1960 = 1u32;
                    fixed_u32::bit_and(a1959, a1960)
                };
                let a1962 = 0u32;
                if equal(a1961, a1962) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1965 = {
                    let a1963 = v0;
                    let a1964 = 1u32;
                    fixed_u32::bit_and(a1963, a1964)
                };
                let a1966 = 0u32;
                if equal(a1965, a1966) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1969 = {
            let a1967 = v0;
            let a1968 = 4u32;
            fixed_u32::bit_and(a1967, a1968)
        };
        let a1970 = 0u32;
        if equal(a1969, a1970) {
            let a1973 = {
                let a1971 = v0;
                let a1972 = 2u32;
                fixed_u32::bit_and(a1971, a1972)
            };
            let a1974 = 0u32;
            if equal(a1973, a1974) {
                let a1977 = {
                    let a1975 = v0;
                    let a1976 = 1u32;
                    fixed_u32::bit_and(a1975, a1976)
                };
                let a1978 = 0u32;
                if equal(a1977, a1978) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1981 = {
                    let a1979 = v0;
                    let a1980 = 1u32;
                    fixed_u32::bit_and(a1979, a1980)
                };
                let a1982 = 0u32;
                if equal(a1981, a1982) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1985 = {
                let a1983 = v0;
                let a1984 = 2u32;
                fixed_u32::bit_and(a1983, a1984)
            };
            let a1986 = 0u32;
            if equal(a1985, a1986) {
                let a1989 = {
                    let a1987 = v0;
                    let a1988 = 1u32;
                    fixed_u32::bit_and(a1987, a1988)
                };
                let a1990 = 0u32;
                if equal(a1989, a1990) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1993 = {
                    let a1991 = v0;
                    let a1992 = 1u32;
                    fixed_u32::bit_and(a1991, a1992)
                };
                let a1994 = 0u32;
                if equal(a1993, a1994) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f362(v0: u32, v1: u32) -> u32 {
    let m1997 = {
        let a1995 = v0;
        let a1996 = v1;
        fixed_u32::shift_right(a1995, a1996)
    };
    match m1997 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f363(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f412(Fn14::F413, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h1998, _, _, _, _, _, _, _) => {
            h1998
        }
    }, match v0.clone() {
        Adt24::C0(_, h1999, _, _, _, _, _, _) => {
            h1999
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2000, _, _, _, _, _) => {
            h2000
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2001, _, _, _, _) => {
            h2001
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2002, _, _, _) => {
            h2002
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2003, _, _) => {
            h2003
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2004, _) => {
            h2004
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2005) => {
            h2005
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f414(match v0.clone() {
        Adt24::C0(h2006, _, _, _, _, _, _, _) => {
            h2006
        }
    }, match v2.clone() {
        Adt27::C0(h2007, _, _, _, _, _, _, _, _, _) => {
            h2007
        }
    }), f414(match v0.clone() {
        Adt24::C0(_, h2008, _, _, _, _, _, _) => {
            h2008
        }
    }, match v2.clone() {
        Adt27::C0(_, h2009, _, _, _, _, _, _, _, _) => {
            h2009
        }
    }), f414(match v0.clone() {
        Adt24::C0(_, _, h2010, _, _, _, _, _) => {
            h2010
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2011, _, _, _, _, _, _, _) => {
            h2011
        }
    }), f414(match v0.clone() {
        Adt24::C0(_, _, _, h2012, _, _, _, _) => {
            h2012
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2013, _, _, _, _, _, _) => {
            h2013
        }
    }), f414(match v0.clone() {
        Adt24::C0(_, _, _, _, h2014, _, _, _) => {
            h2014
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2015, _, _, _, _, _) => {
            h2015
        }
    }), f414(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2016, _, _) => {
            h2016
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2017, _, _, _, _) => {
            h2017
        }
    }), f414(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2018, _) => {
            h2018
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2019, _, _, _) => {
            h2019
        }
    }), f414(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2020) => {
            h2020
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2021, _, _) => {
            h2021
        }
    })))
}

pub fn f364(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f415(v0.clone(), v1, v2, v3)?, f415(v0.clone(), v1, v2, {
        let a2022 = v3;
        let a2023 = 4u64;
        nat_add(a2022, a2023)?
    })?, f415(v0.clone(), v1, v2, {
        let a2024 = v3;
        let a2025 = 8u64;
        nat_add(a2024, a2025)?
    })?, f415(v0.clone(), v1, v2, {
        let a2026 = v3;
        let a2027 = 12u64;
        nat_add(a2026, a2027)?
    })?, f415(v0.clone(), v1, v2, {
        let a2028 = v3;
        let a2029 = 16u64;
        nat_add(a2028, a2029)?
    })?, f415(v0.clone(), v1, v2, {
        let a2030 = v3;
        let a2031 = 20u64;
        nat_add(a2030, a2031)?
    })?, f415(v0.clone(), v1, v2, {
        let a2032 = v3;
        let a2033 = 24u64;
        nat_add(a2032, a2033)?
    })?, f415(v0.clone(), v1, v2, {
        let a2034 = v3;
        let a2035 = 28u64;
        nat_add(a2034, a2035)?
    })?, f415(v0.clone(), v1, v2, {
        let a2036 = v3;
        let a2037 = 32u64;
        nat_add(a2036, a2037)?
    })?, f415(v0.clone(), v1, v2, {
        let a2038 = v3;
        let a2039 = 36u64;
        nat_add(a2038, a2039)?
    })?, f415(v0.clone(), v1, v2, {
        let a2040 = v3;
        let a2041 = 40u64;
        nat_add(a2040, a2041)?
    })?, f415(v0.clone(), v1, v2, {
        let a2042 = v3;
        let a2043 = 44u64;
        nat_add(a2042, a2043)?
    })?, f415(v0.clone(), v1, v2, {
        let a2044 = v3;
        let a2045 = 48u64;
        nat_add(a2044, a2045)?
    })?, f415(v0.clone(), v1, v2, {
        let a2046 = v3;
        let a2047 = 52u64;
        nat_add(a2046, a2047)?
    })?, f415(v0.clone(), v1, v2, {
        let a2048 = v3;
        let a2049 = 56u64;
        nat_add(a2048, a2049)?
    })?, f415(v0, v1, v2, {
        let a2050 = v3;
        let a2051 = 60u64;
        nat_add(a2050, a2051)?
    })?))
}

pub fn f365(p2054: List<Bytes>, v1: Bytes) -> bool {
    let m2053 = p2054;
    {
        let mut m2053 = m2053;
        loop {
            let step = {
                let v0 = m2053;
                let m2052 = v0;
                match m2052.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f204(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2053 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f366(v0: Bytes, v1: u64) -> Option<u64> {
    let a2055 = 0u64;
    let a2056 = v1;
    if nat_lt(a2055, a2056) {
        if f416(v0, {
            let a2057 = v1;
            let a2058 = 1u64;
            nat_sub(a2057, a2058)
        }) {
            Some({
                let a2059 = v1;
                let a2060 = 1u64;
                nat_sub(a2059, a2060)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f367(v0: Adt14) -> R<List<Bytes>> {
    let m2061 = v0;
    match m2061 {
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
        Adt14::C6(r2062, r2063) => {
            let v7 = (*r2062).clone();
            let v8 = (*r2063).clone();
            Ok(List::cons(f302(v7)?, f367(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f368(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m2064 = v0;
    match m2064 {
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
        Adt14::C7(v9, r2065, r2066) => {
            let v10 = (*r2065).clone();
            let v11 = (*r2066).clone();
            Ok(List::cons((v9, f302(v10)?), f368(v11)?))
        }
    }
}

pub fn f369(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2067, _) = v1;
        h2067
    };
    let a2069 = {
        let a2068 = v0.clone();
        length_bytes(a2068)
    };
    let a2070 = v2;
    if nat_le(a2069, a2070) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2071 = f226(v0.clone(), v2)?;
        let a2072 = 13u64;
        if nat_eq(a2071, a2072) {
            Ok(Some(({
                let a2075 = f226(v0, {
                    let a2073 = v2;
                    let a2074 = 1u64;
                    nat_add(a2073, a2074)?
                })?;
                let a2076 = 10u64;
                if nat_eq(a2075, a2076) {
                    let a2077 = v2;
                    let a2078 = 2u64;
                    nat_add(a2077, a2078)?
                } else {
                    let a2079 = v2;
                    let a2080 = 1u64;
                    nat_add(a2079, a2080)?
                }
            }, {
                let a2082 = {
                    let (_, h2081) = v1;
                    h2081
                };
                let a2083 = Bytes::lit(&[0x0a]);
                append_bytes(a2082, a2083)
            })))
        } else {
            let v4: u64 = {
                let m2084 = f307(v0.clone(), Bytes::lit(&[0x0d]), v2)?;
                match m2084 {
                    None => {
                        let a2085 = v0.clone();
                        length_bytes(a2085)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2089 = {
                    let (_, h2086) = v1;
                    h2086
                };
                let a2090 = f32(v0, v2, {
                    let a2087 = v4;
                    let a2088 = v2;
                    nat_sub(a2087, a2088)
                });
                append_bytes(a2089, a2090)
            })))
        }
    }
}

pub fn f370(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2091 = f417(v0, v1)?;
    match m2091 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f418({
                let (h2092, _) = v2;
                h2092
            }) {
                Ok(Some({
                    let a2094 = v1;
                    let a2095 = {
                        let (_, h2093) = v2;
                        h2093
                    };
                    nat_add(a2094, a2095)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f371(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f419(v1, v0, v2)
}

pub fn f372(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f420(v0, v1)
}

pub fn f373(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f421(v0, v1, v2)
}

pub fn f374(v0: Bytes, p2098: List<Bytes>) -> bool {
    let m2097 = p2098;
    {
        let mut m2097 = m2097;
        loop {
            let step = {
                let v1 = m2097;
                let m2096 = v1;
                match m2096.uncons() {
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
                Ok(next) => { m2097 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f225(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2101 = f226(v0.clone(), {
                let a2099 = v1;
                let a2100 = 2u64;
                nat_add(a2099, a2100)?
            })?;
            let a2102 = 120u64;
            nat_eq(a2101, a2102)
        };
        let v3: u64 = if v2 {
            let a2103 = v1;
            let a2104 = 3u64;
            nat_add(a2103, a2104)?
        } else {
            let a2105 = v1;
            let a2106 = 2u64;
            nat_add(a2105, a2106)?
        };
        let v4: (u64, u64) = f422(v0.clone(), v2, v3)?;
        let a2108 = {
            let (h2107, _) = v4;
            h2107
        };
        let a2109 = v3;
        if if nat_eq(a2108, a2109) {
            true
        } else {
            let a2111 = f226(v0, {
                let (h2110, _) = v4;
                h2110
            })?;
            let a2112 = 59u64;
            !nat_eq(a2111, a2112)
        } {
            Ok(Err::<(Bytes, u64), Bytes>(f222(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f418({
                let (_, h2113) = v4;
                h2113
            }) {
                Ok(Ok::<(Bytes, u64), Bytes>((f423({
                    let (_, h2114) = v4;
                    h2114
                })?, {
                    let a2116 = {
                        let (h2115, _) = v4;
                        h2115
                    };
                    let a2117 = 1u64;
                    nat_add(a2116, a2117)?
                })))
            } else {
                Ok(Err::<(Bytes, u64), Bytes>(f222(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f225(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3c]), {
                let a2118 = v1;
                let a2119 = 4u64;
                nat_add(a2118, a2119)?
            })))
        } else {
            if f225(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3e]), {
                    let a2120 = v1;
                    let a2121 = 4u64;
                    nat_add(a2120, a2121)?
                })))
            } else {
                if f225(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x26]), {
                        let a2122 = v1;
                        let a2123 = 5u64;
                        nat_add(a2122, a2123)?
                    })))
                } else {
                    if f225(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x27]), {
                            let a2124 = v1;
                            let a2125 = 6u64;
                            nat_add(a2124, a2125)?
                        })))
                    } else {
                        if f225(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x22]), {
                                let a2126 = v1;
                                let a2127 = 6u64;
                                nat_add(a2126, a2127)?
                            })))
                        } else {
                            Ok(Err::<(Bytes, u64), Bytes>(f222(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f376(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2128, _) = f253(Fn3::F424(v1, v2, v0.clone()), {
        let a2130 = {
            let a2129 = v0;
            length_bytes(a2129)
        };
        let a2131 = 1u64;
        nat_add(a2130, a2131)?
    }, v3)?;
    Ok(h2128)
}

pub fn f377(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2132, _) = v2;
        h2132
    };
    let a2133 = v3;
    let a2134 = 0u64;
    if nat_eq(a2133, a2134) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2135 = v3;
            let a2136 = v1;
            let a2137 = 0u64;
            nat_quot(a2135, a2136, a2137)
        }, {
            let a2142 = f316(v0, {
                let a2138 = v3;
                let a2139 = v1;
                let a2140 = 0u64;
                nat_rem(a2138, a2139, a2140)
            });
            let a2143 = {
                let (_, h2141) = v2;
                h2141
            };
            append_bytes(a2142, a2143)
        }))
    }
}

pub fn f378(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a2144 = v2;
    let a2145 = v0;
    if nat_lt(a2144, a2145) {
        if f416(v1, v2) {
            Ok(Some({
                let a2146 = v2;
                let a2147 = 1u64;
                nat_add(a2146, a2147)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f379(v0: Bytes, p2227: u64, p2228: u64, p2229: bool, p2230: bool, p2231: Bytes) -> R<Bytes> {
    let m2226 = (p2227, (p2228, (p2229, (p2230, p2231))));
    {
        let mut m2226 = m2226;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m2226;
                let m2148 = v2;
                if m2148 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m2148 - 1;
                    let m2151 = {
                        let a2149 = v0.clone();
                        let a2150 = v1;
                        index_bytes(a2149, a2150)
                    };
                    match m2151 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a2196 = {
                                    let a2172 = {
                                        let a2160 = {
                                            let a2154 = {
                                                let a2152 = v8;
                                                let a2153 = 128u8;
                                                fixed_u8::bit_and(a2152, a2153)
                                            };
                                            let a2155 = 128u8;
                                            if equal(a2154, a2155) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2161 = {
                                            let a2158 = {
                                                let a2156 = v8;
                                                let a2157 = 64u8;
                                                fixed_u8::bit_and(a2156, a2157)
                                            };
                                            let a2159 = 64u8;
                                            if equal(a2158, a2159) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2160, a2161)?
                                    };
                                    let a2173 = {
                                        let a2170 = {
                                            let a2164 = {
                                                let a2162 = v8;
                                                let a2163 = 32u8;
                                                fixed_u8::bit_and(a2162, a2163)
                                            };
                                            let a2165 = 32u8;
                                            if equal(a2164, a2165) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2171 = {
                                            let a2168 = {
                                                let a2166 = v8;
                                                let a2167 = 16u8;
                                                fixed_u8::bit_and(a2166, a2167)
                                            };
                                            let a2169 = 16u8;
                                            if equal(a2168, a2169) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2170, a2171)?
                                    };
                                    nat_add(a2172, a2173)?
                                };
                                let a2197 = {
                                    let a2194 = {
                                        let a2182 = {
                                            let a2176 = {
                                                let a2174 = v8;
                                                let a2175 = 8u8;
                                                fixed_u8::bit_and(a2174, a2175)
                                            };
                                            let a2177 = 8u8;
                                            if equal(a2176, a2177) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2183 = {
                                            let a2180 = {
                                                let a2178 = v8;
                                                let a2179 = 4u8;
                                                fixed_u8::bit_and(a2178, a2179)
                                            };
                                            let a2181 = 4u8;
                                            if equal(a2180, a2181) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2182, a2183)?
                                    };
                                    let a2195 = {
                                        let a2192 = {
                                            let a2186 = {
                                                let a2184 = v8;
                                                let a2185 = 2u8;
                                                fixed_u8::bit_and(a2184, a2185)
                                            };
                                            let a2187 = 2u8;
                                            if equal(a2186, a2187) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2193 = {
                                            let a2190 = {
                                                let a2188 = v8;
                                                let a2189 = 1u8;
                                                fixed_u8::bit_and(a2188, a2189)
                                            };
                                            let a2191 = 1u8;
                                            if equal(a2190, a2191) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2192, a2193)?
                                    };
                                    nat_add(a2194, a2195)?
                                };
                                nat_add(a2196, a2197)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a2198 = v1;
                                    let a2199 = 1u64;
                                    nat_add(a2198, a2199)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a2200 = v9;
                                    let a2201 = 34u64;
                                    !nat_eq(a2200, a2201)
                                }, (if v4 {
                                    false
                                } else {
                                    let a2202 = v9;
                                    let a2203 = 92u64;
                                    nat_eq(a2202, a2203)
                                }, {
                                    let a2204 = v5;
                                    let a2205 = v10;
                                    append_bytes(a2204, a2205)
                                })))))
                            } else {
                                let a2206 = v9;
                                let a2207 = 32u64;
                                if if if if nat_eq(a2206, a2207) {
                                    true
                                } else {
                                    let a2208 = v9;
                                    let a2209 = 9u64;
                                    nat_eq(a2208, a2209)
                                } {
                                    true
                                } else {
                                    let a2210 = v9;
                                    let a2211 = 10u64;
                                    nat_eq(a2210, a2211)
                                } {
                                    true
                                } else {
                                    let a2212 = v9;
                                    let a2213 = 13u64;
                                    nat_eq(a2212, a2213)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a2214 = v1;
                                        let a2215 = 1u64;
                                        nat_add(a2214, a2215)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2216 = v1;
                                            let a2217 = 4u64;
                                            nat_add(a2216, a2217)?
                                        }, (v6, (false, (false, {
                                            let a2218 = v5;
                                            let a2219 = f8(Str::lit("[]"));
                                            append_bytes(a2218, a2219)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2220 = v1;
                                            let a2221 = 1u64;
                                            nat_add(a2220, a2221)?
                                        }, (v6, ({
                                            let a2222 = v9;
                                            let a2223 = 34u64;
                                            nat_eq(a2222, a2223)
                                        }, (false, {
                                            let a2224 = v5;
                                            let a2225 = v10;
                                            append_bytes(a2224, a2225)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2226 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f425(Fn15::F426(v0.clone()), {
        let a2233 = {
            let a2232 = v0;
            length_bytes(a2232)
        };
        let a2234 = 1u64;
        nat_add(a2233, a2234)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a2238 = {
        let (_, h2235) = {
            let (_, h2236) = {
                let (h2237, _) = v2.clone();
                h2237
            };
            h2236
        };
        h2235
    };
    let a2239 = 1u64;
    if nat_eq(a2238, a2239) {
        Ok(Some(({
            let (h2240, _) = {
                let (h2241, _) = v2.clone();
                h2241
            };
            h2240
        }, {
            let (h2242, _) = {
                let (_, h2243) = {
                    let (h2244, _) = v2;
                    h2244
                };
                h2243
            };
            h2242
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f381(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2245 = f312(v0.clone(), v1)?;
        let a2246 = 45u64;
        nat_eq(a2245, a2246)
    };
    let v3: u64 = if v2 {
        let a2247 = v1;
        let a2248 = 1u64;
        nat_add(a2247, a2248)?
    } else {
        v1
    };
    let v4: u64 = f427(v0.clone(), v3)?;
    let v5: u64 = {
        let a2249 = v4;
        let a2250 = v3;
        nat_sub(a2249, a2250)
    };
    let v6: bool = {
        let a2251 = f312(v0.clone(), v4)?;
        let a2252 = 46u64;
        nat_eq(a2251, a2252)
    };
    let v7: u64 = if v6 {
        let a2253 = v4;
        let a2254 = 1u64;
        nat_add(a2253, a2254)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f427(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f312(v0.clone(), v8)?;
    let v10: bool = {
        let a2255 = v9;
        let a2256 = 101u64;
        if nat_eq(a2255, a2256) {
            true
        } else {
            let a2257 = v9;
            let a2258 = 69u64;
            nat_eq(a2257, a2258)
        }
    };
    let v11: u64 = f312(v0.clone(), {
        let a2259 = v8;
        let a2260 = 1u64;
        nat_add(a2259, a2260)?
    })?;
    let v12: bool = if v10 {
        let a2261 = v11;
        let a2262 = 43u64;
        if nat_eq(a2261, a2262) {
            true
        } else {
            let a2263 = v11;
            let a2264 = 45u64;
            nat_eq(a2263, a2264)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2265 = v8;
            let a2266 = 2u64;
            nat_add(a2265, a2266)?
        } else {
            let a2267 = v8;
            let a2268 = 1u64;
            nat_add(a2267, a2268)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f427(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2269 = v14;
        let a2270 = v13;
        nat_sub(a2269, a2270)
    });
    let v16: u64 = f428(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2272 = {
            let a2271 = v15;
            length_bytes(a2271)
        };
        let a2273 = v16;
        nat_sub(a2272, a2273)
    });
    let a2274 = v5;
    let a2275 = 0u64;
    if if nat_eq(a2274, a2275) {
        true
    } else {
        let a2276 = 1u64;
        let a2277 = v5;
        if if nat_lt(a2276, a2277) {
            let a2278 = f312(v0.clone(), v3)?;
            let a2279 = 48u64;
            nat_eq(a2278, a2279)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2280 = v8;
                let a2281 = v7;
                nat_eq(a2280, a2281)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2282 = v14;
                    let a2283 = v13;
                    nat_eq(a2282, a2283)
                } else {
                    false
                } {
                    true
                } else {
                    let a2285 = 18u64;
                    let a2286 = {
                        let a2284 = v17.clone();
                        length_bytes(a2284)
                    };
                    nat_lt(a2285, a2286)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2290 = {
            let a2288 = {
                let a2287 = v17.clone();
                length_bytes(a2287)
            };
            let a2289 = 0u64;
            if nat_eq(a2288, a2289) {
                Some(0u64)
            } else {
                f429(v17)?
            }
        };
        match m2290 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2295 = f430(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2291 = v8;
                    let a2292 = v7;
                    nat_sub(a2291, a2292)
                }), if v12 {
                    let a2293 = v11;
                    let a2294 = 45u64;
                    nat_eq(a2293, a2294)
                } else {
                    false
                }, v18)?;
                match m2295 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2297 = 19u64;
                        let a2298 = {
                            let a2296 = v19.clone();
                            length_bytes(a2296)
                        };
                        if nat_lt(a2297, a2298) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2299 = f429(v19)?;
                            match m2299 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2300 = 0u64;
                                        let a2301 = v20;
                                        nat_lt(a2300, a2301)
                                    } else {
                                        false
                                    };
                                    if f431(v21, v20) {
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

pub fn f382(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2302 = f312(v0.clone(), v1)?;
        let a2303 = 45u64;
        nat_eq(a2302, a2303)
    };
    let v3: u64 = if v2 {
        let a2304 = v1;
        let a2305 = 1u64;
        nat_add(a2304, a2305)?
    } else {
        v1
    };
    let v4: u64 = f427(v0.clone(), v3)?;
    let v5: u64 = {
        let a2306 = v4;
        let a2307 = v3;
        nat_sub(a2306, a2307)
    };
    let v6: bool = {
        let a2308 = f312(v0.clone(), v4)?;
        let a2309 = 46u64;
        nat_eq(a2308, a2309)
    };
    let v7: u64 = if v6 {
        let a2310 = v4;
        let a2311 = 1u64;
        nat_add(a2310, a2311)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f427(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f312(v0.clone(), v8)?;
    let v10: bool = {
        let a2312 = v9;
        let a2313 = 101u64;
        if nat_eq(a2312, a2313) {
            true
        } else {
            let a2314 = v9;
            let a2315 = 69u64;
            nat_eq(a2314, a2315)
        }
    };
    let v11: u64 = f312(v0.clone(), {
        let a2316 = v8;
        let a2317 = 1u64;
        nat_add(a2316, a2317)?
    })?;
    let v12: bool = if v10 {
        let a2318 = v11;
        let a2319 = 43u64;
        if nat_eq(a2318, a2319) {
            true
        } else {
            let a2320 = v11;
            let a2321 = 45u64;
            nat_eq(a2320, a2321)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2322 = v8;
            let a2323 = 2u64;
            nat_add(a2322, a2323)?
        } else {
            let a2324 = v8;
            let a2325 = 1u64;
            nat_add(a2324, a2325)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f427(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2326 = v14;
        let a2327 = v13;
        nat_sub(a2326, a2327)
    });
    let v16: u64 = f428(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2329 = {
            let a2328 = v15;
            length_bytes(a2328)
        };
        let a2330 = v16;
        nat_sub(a2329, a2330)
    });
    let a2331 = v5;
    let a2332 = 0u64;
    if if nat_eq(a2331, a2332) {
        true
    } else {
        let a2333 = 1u64;
        let a2334 = v5;
        if if nat_lt(a2333, a2334) {
            let a2335 = f312(v0.clone(), v3)?;
            let a2336 = 48u64;
            nat_eq(a2335, a2336)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2337 = v8;
                let a2338 = v7;
                nat_eq(a2337, a2338)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2339 = v14;
                    let a2340 = v13;
                    nat_eq(a2339, a2340)
                } else {
                    false
                } {
                    true
                } else {
                    let a2342 = 18u64;
                    let a2343 = {
                        let a2341 = v17.clone();
                        length_bytes(a2341)
                    };
                    nat_lt(a2342, a2343)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2347 = {
            let a2345 = {
                let a2344 = v17.clone();
                length_bytes(a2344)
            };
            let a2346 = 0u64;
            if nat_eq(a2345, a2346) {
                Some(0u64)
            } else {
                f429(v17)?
            }
        };
        match m2347 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2352 = f432(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2348 = v8;
                    let a2349 = v7;
                    nat_sub(a2348, a2349)
                }), if v12 {
                    let a2350 = v11;
                    let a2351 = 45u64;
                    nat_eq(a2350, a2351)
                } else {
                    false
                }, v18)?;
                match m2352 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2355 = 19u64;
                        let a2356 = {
                            let a2354 = {
                                let (h2353, _) = v19.clone();
                                h2353
                            };
                            length_bytes(a2354)
                        };
                        if nat_lt(a2355, a2356) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2358 = f429({
                                let (h2357, _) = v19.clone();
                                h2357
                            })?;
                            match m2358 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2359 = 0u64;
                                        let a2360 = v20;
                                        nat_lt(a2359, a2360)
                                    } else {
                                        false
                                    };
                                    if f431(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2362 = {
                                                let (_, h2361) = v19.clone();
                                                h2361
                                            };
                                            let a2363 = 0u64;
                                            if nat_eq(a2362, a2363) {
                                                Adt19::C1(v21, v20)
                                            } else {
                                                Adt19::C2(v21, v20, {
                                                    let (_, h2364) = v19;
                                                    h2364
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

pub fn f383(v0: u64) -> bool {
    let a2365 = v0;
    let a2366 = 95u64;
    if nat_eq(a2365, a2366) {
        true
    } else {
        let a2367 = v0;
        let a2368 = 45u64;
        if nat_eq(a2367, a2368) {
            true
        } else {
            let a2369 = v0;
            let a2370 = 32u64;
            if nat_eq(a2369, a2370) {
                true
            } else {
                let a2371 = v0;
                let a2372 = 46u64;
                if nat_eq(a2371, a2372) {
                    true
                } else {
                    let a2373 = v0;
                    let a2374 = 47u64;
                    nat_eq(a2373, a2374)
                }
            }
        }
    }
}

pub fn f384(v0: u64) -> bool {
    let a2375 = 97u64;
    let a2376 = v0;
    if nat_le(a2375, a2376) {
        let a2377 = v0;
        let a2378 = 122u64;
        nat_le(a2377, a2378)
    } else {
        false
    }
}

pub fn f385(v0: u64) -> bool {
    let a2379 = 65u64;
    let a2380 = v0;
    if nat_le(a2379, a2380) {
        let a2381 = v0;
        let a2382 = 90u64;
        nat_le(a2381, a2382)
    } else {
        false
    }
}

pub fn f386(v0: u64) -> bool {
    let a2383 = 48u64;
    let a2384 = v0;
    if nat_le(a2383, a2384) {
        let a2385 = v0;
        let a2386 = 57u64;
        nat_le(a2385, a2386)
    } else {
        false
    }
}

pub fn f387(v0: List<(Bytes, Adt19)>, v1: List<(Bytes, Adt19)>) -> List<(Bytes, Adt19)> {
    let m2387 = v0;
    match m2387.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f387(v3, v1))
        }
    }
}

pub fn f388(v0: Option<Adt19>) -> List<(Bytes, Adt19)> {
    let m2388 = v0;
    match m2388 {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some(v1) => {
            f433(v1)
        }
    }
}

pub fn f389(v0: u64, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt19)>, Bytes>> {
    let m2389 = v0;
    if m2389 == 0 {
        let m2390 = v1;
        match m2390.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2389 - 1;
        let m2391 = v1.clone();
        match m2391.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                let m2392 = f434(v1.clone(), v2.clone(), v3.clone())?;
                match m2392 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f214(f390(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2395 = f389(v6, f435(v1, {
                            let (h2393, _) = v9.clone();
                            h2393
                        }), List::cons(f241({
                            let (h2394, _) = v9.clone();
                            h2394
                        })?, v2), v3)?;
                        match m2395 {
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

pub fn f390(v0: List<(Bytes, Adt19)>) -> R<List<Bytes>> {
    let m2396 = v0;
    match m2396.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f241({
                let (h2397, _) = v1;
                h2397
            })?, f390(v2)?))
        }
    }
}

pub fn f391(p2405: u64, p2406: List<Adt25>, p2407: List<Bytes>, p2408: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2404 = (p2405, (p2406, (p2407, p2408)));
    {
        let mut m2404 = m2404;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2404;
                let m2398 = v0;
                if m2398 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2398 - 1;
                    let m2399 = v1;
                    match m2399.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2400 = v5;
                            match m2400 {
                                Adt25::C0(v8, v9) => {
                                    if f36(v8.clone(), f436()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f86(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2401 = f438(v8.clone(), v9)?;
                                            match m2401 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f392({
                                                        let (_, h2402) = v11;
                                                        h2402
                                                    }, List::cons(Adt25::C1({
                                                        let (h2403, _) = v11;
                                                        h2403
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
                                    if f86(f436(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f436(), v2), List::cons(f437(), v3)))))
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

pub fn f392(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2409 = v0;
    match m2409.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f392(v3, v1))
        }
    }
}

pub fn f393(v0: List<(Bytes, Adt19)>) -> R<List<Adt25>> {
    let m2410 = v0;
    match m2410.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f241({
                let (h2411, _) = v1;
                h2411
            })?, {
                let (_, h2412) = v1;
                h2412
            }), f393(v2)?))
        }
    }
}

pub fn f394(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2413 = v0;
    match m2413.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f36(f157(v3.clone(), v1.clone()), v2.clone()) {
                f394(v4, v1, v2)
            } else {
                List::cons(v3, f394(v4, v1, v2))
            }
        }
    }
}

pub fn f395(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2414 = v0;
    match m2414.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f204(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f214(f403(f78(List::cons(v1, f439(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f395(v3, v1)?))
            }
        }
    }
}

pub fn f396(v0: u64) -> Bytes {
    let m2415 = v0;
    if m2415 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2415 - 1;
        let a2416 = f8(Str::lit("0"));
        let a2417 = f396(v1);
        append_bytes(a2416, a2417)
    }
}

pub fn f397(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f440(v0, v1)
}

pub fn f398(v0: Fn10, p2421: List<(Bytes, Bytes)>, p2422: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2420 = (p2421, p2422);
    {
        let mut m2420 = m2420;
        loop {
            let step = {
                let (v1, v2) = m2420;
                let m2418 = v2;
                match m2418.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2419: Fn10 = v0.clone();
                            c2419.apply(v1, v3)
                        }, v4))
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

pub fn f399(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f441(v0, v1)
}

pub fn f400(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f442(v0)
}

pub fn f401(v0: Fn11, p2426: List<Bytes>, p2427: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2425 = (p2426, p2427);
    {
        let mut m2425 = m2425;
        loop {
            let step = {
                let (v1, v2) = m2425;
                let m2423 = v2;
                match m2423.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2424: Fn11 = v0.clone();
                            c2424.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f443(v1)?, v0))
}

pub fn f403(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2428 = v0;
    match m2428.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f209(v1)?, f403(v2)?))
        }
    }
}

pub fn f404(v0: Fn12, p2432: List<List<(Bytes, ())>>, p2433: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2431 = (p2432, p2433);
    {
        let mut m2431 = m2431;
        loop {
            let step = {
                let (v1, v2) = m2431;
                let m2429 = v2;
                match m2429.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2430: Fn12 = v0.clone();
                            c2430.apply(v1, v3)
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

pub fn f405(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f406(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2435 = {
        let a2434 = v0.clone();
        length_list(a2434)
    };
    let a2436 = 1u64;
    if nat_le(a2435, a2436) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f444(v0)?))
    }
}

pub fn f407(v0: Fn13, p2440: List<(Bytes, ())>, p2441: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2439 = (p2440, p2441);
    {
        let mut m2439 = m2439;
        loop {
            let step = {
                let (v1, v2) = m2439;
                let m2437 = v2;
                match m2437.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2438: Fn13 = v0.clone();
                            c2438.apply(v1, v3)
                        }, v4))
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

pub fn f408(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f409(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f72(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f445(v3.clone(), f72(f73(v0, f89())?, f8(Str::lit("at"))));
    let a2443 = {
        let a2442 = v4.clone();
        length_list(a2442)
    };
    let a2444 = 0u64;
    if nat_eq(a2443, a2444) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2447 = {
            let a2445 = v4;
            length_list(a2445)
        };
        let a2448 = {
            let a2446 = v3;
            length_list(a2446)
        };
        if nat_eq(a2447, a2448) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f446(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f410(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f236(f140(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f236(f140(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f64(f72(v3.clone(), f8(Str::lit("id"))), f72(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f72(f73(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f445(v5.clone(), f64(v6.clone(), f72(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2450 = {
        let a2449 = v7.clone();
        length_list(a2449)
    };
    let a2451 = 0u64;
    if nat_eq(a2450, a2451) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f75({
            let a2454 = {
                let a2452 = v7;
                length_list(a2452)
            };
            let a2455 = {
                let a2453 = v5;
                length_list(a2453)
            };
            nat_eq(a2454, a2455)
        }) {
            Ok((false, f446(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f78(f64(f445(f447(v4.clone()), f69(f70(v0, f8(Str::lit("model/authorities.toml"))))?), f445(f448(v4), f64(v6, f72(v3, f8(Str::lit("id")))))))?;
            let a2457 = {
                let a2456 = v8.clone();
                length_list(a2456)
            };
            let a2458 = 0u64;
            if nat_eq(a2457, a2458) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f446(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f121(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f411(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h2459, _) = v0;
        h2459
    }, {
        let (h2460, _) = v1;
        h2460
    }) {
        f36({
            let (_, h2461) = v0;
            h2461
        }, {
            let (_, h2462) = v1;
            h2462
        })
    } else {
        false
    }
}

pub fn f412(v0: Fn14, p2466: u64, p2467: Adt27) -> R<Adt27> {
    let m2465 = (p2466, p2467);
    {
        let mut m2465 = m2465;
        loop {
            let step = {
                let (v1, v2) = m2465;
                let m2463 = v1;
                if m2463 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2463 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2464: Fn14 = v0.clone();
                        c2464.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(v0: Adt27) -> R<Adt27> {
    f449(v0)
}

pub fn f414(v0: u32, v1: u32) -> u32 {
    let m2470 = {
        let a2468 = f450(v0);
        let a2469 = f450(v1);
        fixed_u64::checked_add(a2468, a2469)
    };
    match m2470 {
        None => {
            0u32
        }
        Some(v2) => {
            f451(v2)
        }
    }
}

pub fn f415(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2473 = {
        let a2471 = v3;
        let a2472 = 8u64;
        nat_add(a2471, a2472)?
    };
    let a2474 = v2;
    if nat_eq(a2473, a2474) {
        Ok(f452({
            let a2477 = {
                let a2475 = v1;
                let a2476 = 8u64;
                nat_mul(a2475, a2476)?
            };
            let a2478 = 4294967296u64;
            let a2479 = 0u64;
            nat_quot(a2477, a2478, a2479)
        }))
    } else {
        let a2482 = {
            let a2480 = v3;
            let a2481 = 4u64;
            nat_add(a2480, a2481)?
        };
        let a2483 = v2;
        if nat_eq(a2482, a2483) {
            Ok(f452({
                let a2486 = {
                    let a2484 = v1;
                    let a2485 = 8u64;
                    nat_mul(a2484, a2485)?
                };
                let a2487 = 4294967296u64;
                let a2488 = 0u64;
                nat_rem(a2486, a2487, a2488)
            }))
        } else {
            Ok(f453(f454(v0.clone(), v1, v3), f454(v0.clone(), v1, {
                let a2489 = v3;
                let a2490 = 1u64;
                nat_add(a2489, a2490)?
            }), f454(v0.clone(), v1, {
                let a2491 = v3;
                let a2492 = 2u64;
                nat_add(a2491, a2492)?
            }), f454(v0, v1, {
                let a2493 = v3;
                let a2494 = 3u64;
                nat_add(a2493, a2494)?
            })))
        }
    }
}

pub fn f416(v0: Bytes, v1: u64) -> bool {
    let m2495 = f251(v0, v1);
    match m2495 {
        None => {
            false
        }
        Some(v2) => {
            f455(v2)
        }
    }
}

pub fn f417(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2498 = {
        let a2496 = v0.clone();
        let a2497 = v1;
        index_bytes(a2496, a2497)
    };
    match m2498 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2543 = {
                    let a2519 = {
                        let a2507 = {
                            let a2501 = {
                                let a2499 = v3;
                                let a2500 = 128u8;
                                fixed_u8::bit_and(a2499, a2500)
                            };
                            let a2502 = 128u8;
                            if equal(a2501, a2502) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2508 = {
                            let a2505 = {
                                let a2503 = v3;
                                let a2504 = 64u8;
                                fixed_u8::bit_and(a2503, a2504)
                            };
                            let a2506 = 64u8;
                            if equal(a2505, a2506) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2507, a2508)?
                    };
                    let a2520 = {
                        let a2517 = {
                            let a2511 = {
                                let a2509 = v3;
                                let a2510 = 32u8;
                                fixed_u8::bit_and(a2509, a2510)
                            };
                            let a2512 = 32u8;
                            if equal(a2511, a2512) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2518 = {
                            let a2515 = {
                                let a2513 = v3;
                                let a2514 = 16u8;
                                fixed_u8::bit_and(a2513, a2514)
                            };
                            let a2516 = 16u8;
                            if equal(a2515, a2516) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2517, a2518)?
                    };
                    nat_add(a2519, a2520)?
                };
                let a2544 = {
                    let a2541 = {
                        let a2529 = {
                            let a2523 = {
                                let a2521 = v3;
                                let a2522 = 8u8;
                                fixed_u8::bit_and(a2521, a2522)
                            };
                            let a2524 = 8u8;
                            if equal(a2523, a2524) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2530 = {
                            let a2527 = {
                                let a2525 = v3;
                                let a2526 = 4u8;
                                fixed_u8::bit_and(a2525, a2526)
                            };
                            let a2528 = 4u8;
                            if equal(a2527, a2528) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2529, a2530)?
                    };
                    let a2542 = {
                        let a2539 = {
                            let a2533 = {
                                let a2531 = v3;
                                let a2532 = 2u8;
                                fixed_u8::bit_and(a2531, a2532)
                            };
                            let a2534 = 2u8;
                            if equal(a2533, a2534) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2540 = {
                            let a2537 = {
                                let a2535 = v3;
                                let a2536 = 1u8;
                                fixed_u8::bit_and(a2535, a2536)
                            };
                            let a2538 = 1u8;
                            if equal(a2537, a2538) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2539, a2540)?
                    };
                    nat_add(a2541, a2542)?
                };
                nat_add(a2543, a2544)?
            };
            let a2545 = v4;
            let a2546 = 128u64;
            if nat_lt(a2545, a2546) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2547 = 194u64;
                let a2548 = v4;
                if if nat_le(a2547, a2548) {
                    let a2549 = v4;
                    let a2550 = 223u64;
                    nat_le(a2549, a2550)
                } else {
                    false
                } {
                    let m2553 = f456(v0, {
                        let a2551 = v1;
                        let a2552 = 1u64;
                        nat_add(a2551, a2552)?
                    })?;
                    match m2553 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f457({
                                let a2558 = {
                                    let a2556 = {
                                        let a2554 = v4;
                                        let a2555 = 192u64;
                                        nat_sub(a2554, a2555)
                                    };
                                    let a2557 = 64u64;
                                    nat_mul(a2556, a2557)?
                                };
                                let a2559 = v5;
                                nat_add(a2558, a2559)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2560 = 224u64;
                    let a2561 = v4;
                    if if nat_le(a2560, a2561) {
                        let a2562 = v4;
                        let a2563 = 239u64;
                        nat_le(a2562, a2563)
                    } else {
                        false
                    } {
                        let m2566 = f456(v0.clone(), {
                            let a2564 = v1;
                            let a2565 = 1u64;
                            nat_add(a2564, a2565)?
                        })?;
                        match m2566 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2569 = f456(v0, {
                                    let a2567 = v1;
                                    let a2568 = 2u64;
                                    nat_add(a2567, a2568)?
                                })?;
                                match m2569 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f457({
                                            let a2578 = {
                                                let a2572 = {
                                                    let a2570 = v4;
                                                    let a2571 = 224u64;
                                                    nat_sub(a2570, a2571)
                                                };
                                                let a2573 = 4096u64;
                                                nat_mul(a2572, a2573)?
                                            };
                                            let a2579 = {
                                                let a2576 = {
                                                    let a2574 = v6;
                                                    let a2575 = 64u64;
                                                    nat_mul(a2574, a2575)?
                                                };
                                                let a2577 = v7;
                                                nat_add(a2576, a2577)?
                                            };
                                            nat_add(a2578, a2579)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2580 = 240u64;
                        let a2581 = v4;
                        if if nat_le(a2580, a2581) {
                            let a2582 = v4;
                            let a2583 = 244u64;
                            nat_le(a2582, a2583)
                        } else {
                            false
                        } {
                            let m2586 = f456(v0.clone(), {
                                let a2584 = v1;
                                let a2585 = 1u64;
                                nat_add(a2584, a2585)?
                            })?;
                            match m2586 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2589 = f456(v0.clone(), {
                                        let a2587 = v1;
                                        let a2588 = 2u64;
                                        nat_add(a2587, a2588)?
                                    })?;
                                    match m2589 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2592 = f456(v0, {
                                                let a2590 = v1;
                                                let a2591 = 3u64;
                                                nat_add(a2590, a2591)?
                                            })?;
                                            match m2592 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f457({
                                                        let a2605 = {
                                                            let a2595 = {
                                                                let a2593 = v4;
                                                                let a2594 = 240u64;
                                                                nat_sub(a2593, a2594)
                                                            };
                                                            let a2596 = 262144u64;
                                                            nat_mul(a2595, a2596)?
                                                        };
                                                        let a2606 = {
                                                            let a2603 = {
                                                                let a2597 = v8;
                                                                let a2598 = 4096u64;
                                                                nat_mul(a2597, a2598)?
                                                            };
                                                            let a2604 = {
                                                                let a2601 = {
                                                                    let a2599 = v9;
                                                                    let a2600 = 64u64;
                                                                    nat_mul(a2599, a2600)?
                                                                };
                                                                let a2602 = v10;
                                                                nat_add(a2601, a2602)?
                                                            };
                                                            nat_add(a2603, a2604)?
                                                        };
                                                        nat_add(a2605, a2606)?
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

pub fn f418(v0: u64) -> bool {
    let a2607 = v0;
    let a2608 = 9u64;
    if if if nat_eq(a2607, a2608) {
        true
    } else {
        let a2609 = v0;
        let a2610 = 10u64;
        nat_eq(a2609, a2610)
    } {
        true
    } else {
        let a2611 = v0;
        let a2612 = 13u64;
        nat_eq(a2611, a2612)
    } {
        true
    } else {
        let a2613 = 32u64;
        let a2614 = v0;
        if if if nat_le(a2613, a2614) {
            let a2615 = v0;
            let a2616 = 55295u64;
            nat_le(a2615, a2616)
        } else {
            false
        } {
            true
        } else {
            let a2617 = 57344u64;
            let a2618 = v0;
            if nat_le(a2617, a2618) {
                let a2619 = v0;
                let a2620 = 65533u64;
                nat_le(a2619, a2620)
            } else {
                false
            }
        } {
            true
        } else {
            let a2621 = 65536u64;
            let a2622 = v0;
            if nat_le(a2621, a2622) {
                let a2623 = v0;
                let a2624 = 1114111u64;
                nat_le(a2623, a2624)
            } else {
                false
            }
        }
    }
}

pub fn f419(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f219(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2626 = v2;
        let a2627 = {
            let a2625 = v0;
            length_bytes(a2625)
        };
        if nat_lt(a2626, a2627) {
            Ok(Some({
                let a2628 = v2;
                let a2629 = 1u64;
                nat_add(a2628, a2629)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f420(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f306(f226(v0, v1)?) {
        Ok(Some({
            let a2630 = v1;
            let a2631 = 1u64;
            nat_add(a2630, a2631)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f421(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2632 = f417(v0, v2)?;
    match m2632 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2633 = v2;
            let a2634 = v1;
            if if nat_eq(a2633, a2634) {
                f458({
                    let (h2635, _) = v3;
                    h2635
                })
            } else {
                f459({
                    let (h2636, _) = v3;
                    h2636
                })
            } {
                Ok(Some({
                    let a2638 = v2;
                    let a2639 = {
                        let (_, h2637) = v3;
                        h2637
                    };
                    nat_add(a2638, a2639)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f422(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2640, _) = f460(Fn16::F461(v1, v0.clone()), {
        let a2642 = {
            let a2641 = v0;
            length_bytes(a2641)
        };
        let a2643 = 1u64;
        nat_add(a2642, a2643)?
    }, (v2, 0u64))?;
    Ok(h2640)
}

pub fn f423(v0: u64) -> R<Bytes> {
    let a2644 = v0;
    let a2645 = 128u64;
    if nat_lt(a2644, a2645) {
        Ok(f462(v0))
    } else {
        let a2646 = v0;
        let a2647 = 2048u64;
        if nat_lt(a2646, a2647) {
            let a2658 = f462({
                let a2651 = 192u64;
                let a2652 = {
                    let a2648 = v0;
                    let a2649 = 64u64;
                    let a2650 = 0u64;
                    nat_quot(a2648, a2649, a2650)
                };
                nat_add(a2651, a2652)?
            });
            let a2659 = f462({
                let a2656 = 128u64;
                let a2657 = {
                    let a2653 = v0;
                    let a2654 = 64u64;
                    let a2655 = 0u64;
                    nat_rem(a2653, a2654, a2655)
                };
                nat_add(a2656, a2657)?
            });
            Ok(append_bytes(a2658, a2659))
        } else {
            let a2660 = v0;
            let a2661 = 65536u64;
            if nat_lt(a2660, a2661) {
                let a2682 = f462({
                    let a2665 = 224u64;
                    let a2666 = {
                        let a2662 = v0;
                        let a2663 = 4096u64;
                        let a2664 = 0u64;
                        nat_quot(a2662, a2663, a2664)
                    };
                    nat_add(a2665, a2666)?
                });
                let a2683 = {
                    let a2680 = f462({
                        let a2673 = 128u64;
                        let a2674 = {
                            let a2670 = {
                                let a2667 = v0;
                                let a2668 = 64u64;
                                let a2669 = 0u64;
                                nat_quot(a2667, a2668, a2669)
                            };
                            let a2671 = 64u64;
                            let a2672 = 0u64;
                            nat_rem(a2670, a2671, a2672)
                        };
                        nat_add(a2673, a2674)?
                    });
                    let a2681 = f462({
                        let a2678 = 128u64;
                        let a2679 = {
                            let a2675 = v0;
                            let a2676 = 64u64;
                            let a2677 = 0u64;
                            nat_rem(a2675, a2676, a2677)
                        };
                        nat_add(a2678, a2679)?
                    });
                    append_bytes(a2680, a2681)
                };
                Ok(append_bytes(a2682, a2683))
            } else {
                let a2714 = f462({
                    let a2687 = 240u64;
                    let a2688 = {
                        let a2684 = v0;
                        let a2685 = 262144u64;
                        let a2686 = 0u64;
                        nat_quot(a2684, a2685, a2686)
                    };
                    nat_add(a2687, a2688)?
                });
                let a2715 = {
                    let a2712 = f462({
                        let a2695 = 128u64;
                        let a2696 = {
                            let a2692 = {
                                let a2689 = v0;
                                let a2690 = 4096u64;
                                let a2691 = 0u64;
                                nat_quot(a2689, a2690, a2691)
                            };
                            let a2693 = 64u64;
                            let a2694 = 0u64;
                            nat_rem(a2692, a2693, a2694)
                        };
                        nat_add(a2695, a2696)?
                    });
                    let a2713 = {
                        let a2710 = f462({
                            let a2703 = 128u64;
                            let a2704 = {
                                let a2700 = {
                                    let a2697 = v0;
                                    let a2698 = 64u64;
                                    let a2699 = 0u64;
                                    nat_quot(a2697, a2698, a2699)
                                };
                                let a2701 = 64u64;
                                let a2702 = 0u64;
                                nat_rem(a2700, a2701, a2702)
                            };
                            nat_add(a2703, a2704)?
                        });
                        let a2711 = f462({
                            let a2708 = 128u64;
                            let a2709 = {
                                let a2705 = v0;
                                let a2706 = 64u64;
                                let a2707 = 0u64;
                                nat_rem(a2705, a2706, a2707)
                            };
                            nat_add(a2708, a2709)?
                        });
                        append_bytes(a2710, a2711)
                    };
                    append_bytes(a2712, a2713)
                };
                Ok(append_bytes(a2714, a2715))
            }
        }
    }
}

pub fn f424(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f463(v2, v0, v1, v3)
}

pub fn f425(v0: Fn15, p2720: u64, p2721: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2719 = (p2720, p2721);
    {
        let mut m2719 = m2719;
        loop {
            let step = {
                let (v1, v2) = m2719;
                let m2716 = v1;
                if m2716 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2716 - 1;
                    let m2718 = {
                        let c2717: Fn15 = v0.clone();
                        c2717.apply(v2.clone())?
                    };
                    match m2718 {
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
                Ok(next) => { m2719 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f464(v0, v1)
}

pub fn f427(v0: Bytes, v1: u64) -> R<u64> {
    let (h2722, _) = f253(Fn3::F465(v0.clone()), {
        let a2724 = {
            let a2723 = v0;
            length_bytes(a2723)
        };
        let a2725 = 1u64;
        nat_add(a2724, a2725)?
    }, v1)?;
    Ok(h2722)
}

pub fn f428(v0: Bytes) -> R<u64> {
    let (h2726, _) = f253(Fn3::F466(v0.clone()), {
        let a2728 = {
            let a2727 = v0;
            length_bytes(a2727)
        };
        let a2729 = 1u64;
        nat_add(a2728, a2729)?
    }, 0u64)?;
    Ok(h2726)
}

pub fn f429(v0: Bytes) -> R<Option<u64>> {
    if f467(v0.clone())? {
        Ok(Some({
            let (_, h2730) = {
                let (h2731, _) = f460(Fn16::F468(v0.clone()), {
                    let a2732 = v0;
                    length_bytes(a2732)
                }, (0u64, 0u64))?;
                h2731
            };
            h2730
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f430(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2734 = {
            let a2733 = v1.clone();
            length_bytes(a2733)
        };
        let a2735 = f469(v1)?;
        nat_sub(a2734, a2735)
    });
    let v5: Bytes = {
        let a2736 = v0;
        let a2737 = v4.clone();
        append_bytes(a2736, a2737)
    };
    let v6: u64 = f428(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2739 = {
            let a2738 = v5;
            length_bytes(a2738)
        };
        let a2740 = v6;
        nat_sub(a2739, a2740)
    });
    let v8: u64 = {
        let a2741 = v4;
        length_bytes(a2741)
    };
    let v9: u64 = {
        let a2742 = v7.clone();
        length_bytes(a2742)
    };
    let a2743 = v9;
    let a2744 = 0u64;
    if nat_eq(a2743, a2744) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a2745 = v3;
            let a2746 = v8;
            nat_lt(a2745, a2746)
        } {
            let v10: u64 = if v2 {
                let a2747 = v3;
                let a2748 = v8;
                nat_add(a2747, a2748)?
            } else {
                let a2749 = v8;
                let a2750 = v3;
                nat_sub(a2749, a2750)
            };
            let a2751 = v10;
            let a2752 = f469(v7.clone())?;
            if nat_le(a2751, a2752) {
                Ok(Some(f32(v7, 0u64, {
                    let a2753 = v9;
                    let a2754 = v10;
                    nat_sub(a2753, a2754)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a2755 = v3;
                let a2756 = v8;
                nat_sub(a2755, a2756)
            };
            let a2759 = {
                let a2757 = v9;
                let a2758 = v11;
                nat_add(a2757, a2758)?
            };
            let a2760 = 19u64;
            if nat_le(a2759, a2760) {
                Ok(Some({
                    let a2761 = v7;
                    let a2762 = f396(v11);
                    append_bytes(a2761, a2762)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f431(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2763 = 0u64;
        let a2764 = v1;
        if nat_lt(a2763, a2764) {
            let a2765 = v1;
            let a2766 = 9223372036854775808u64;
            nat_le(a2765, a2766)
        } else {
            false
        }
    } else {
        let a2767 = v1;
        let a2768 = 9223372036854775807u64;
        nat_le(a2767, a2768)
    }
}

pub fn f432(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2770 = {
            let a2769 = v1.clone();
            length_bytes(a2769)
        };
        let a2771 = f469(v1)?;
        nat_sub(a2770, a2771)
    });
    let v5: Bytes = {
        let a2772 = v0;
        let a2773 = v4.clone();
        append_bytes(a2772, a2773)
    };
    let v6: u64 = f428(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2775 = {
            let a2774 = v5;
            length_bytes(a2774)
        };
        let a2776 = v6;
        nat_sub(a2775, a2776)
    });
    let v8: u64 = {
        let a2777 = v4;
        length_bytes(a2777)
    };
    let v9: u64 = {
        let a2778 = v7.clone();
        length_bytes(a2778)
    };
    let a2779 = v9;
    let a2780 = 0u64;
    if nat_eq(a2779, a2780) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if if v2 {
            true
        } else {
            let a2781 = v3;
            let a2782 = v8;
            nat_lt(a2781, a2782)
        } {
            let v10: u64 = if v2 {
                let a2783 = v3;
                let a2784 = v8;
                nat_add(a2783, a2784)?
            } else {
                let a2785 = v8;
                let a2786 = v3;
                nat_sub(a2785, a2786)
            };
            let v11: u64 = f469(v7.clone())?;
            let a2787 = v10;
            let a2788 = v11;
            if nat_le(a2787, a2788) {
                Ok(Some((f32(v7, 0u64, {
                    let a2789 = v9;
                    let a2790 = v10;
                    nat_sub(a2789, a2790)
                }), 0u64)))
            } else {
                let a2793 = {
                    let a2791 = v10;
                    let a2792 = v11;
                    nat_sub(a2791, a2792)
                };
                let a2794 = 19u64;
                if nat_le(a2793, a2794) {
                    Ok(Some((f32(v7, 0u64, {
                        let a2795 = v9;
                        let a2796 = v11;
                        nat_sub(a2795, a2796)
                    }), {
                        let a2797 = v10;
                        let a2798 = v11;
                        nat_sub(a2797, a2798)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a2799 = v3;
                let a2800 = v8;
                nat_sub(a2799, a2800)
            };
            let a2803 = {
                let a2801 = v9;
                let a2802 = v12;
                nat_add(a2801, a2802)?
            };
            let a2804 = 19u64;
            if nat_le(a2803, a2804) {
                Ok(Some(({
                    let a2805 = v7;
                    let a2806 = f396(v12);
                    append_bytes(a2805, a2806)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f433(v0: Adt19) -> List<(Bytes, Adt19)> {
    let m2807 = v0;
    match m2807 {
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
        Adt19::C5(r2808) => {
            let v4 = (*r2808).clone();
            f433(v4)
        }
        Adt19::C6 => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C7(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C8(v1, r2809, r2810) => {
            let v2 = (*r2809).clone();
            let v3 = (*r2810).clone();
            List::cons((v1, v2), f433(v3))
        }
    }
}

pub fn f434(p2815: List<(Bytes, Adt19)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt19)>> {
    let m2814 = p2815;
    {
        let mut m2814 = m2814;
        loop {
            let step = {
                let v0 = m2814;
                let m2811 = v0;
                match m2811.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(None::<(Bytes, Adt19)>))
                    }
                    Some((v3, v4)) => {
                        if f470(f471({
                            let (_, h2812) = v3.clone();
                            h2812
                        }, List::<Bytes>::nil())?, f241({
                            let (h2813, _) = v3.clone();
                            h2813
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f435(v0: List<(Bytes, Adt19)>, v1: Bytes) -> List<(Bytes, Adt19)> {
    let m2816 = v0;
    match m2816.uncons() {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h2817, _) = v2.clone();
                h2817
            }, v1.clone()) {
                f435(v3, v1)
            } else {
                List::cons(v2, f435(v3, v1))
            }
        }
    }
}

pub fn f436() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f437() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f438(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f472(v0, f473(v1))
}

pub fn f439(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2818 = v0;
    match m2818.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f439(v3, false))
            } else {
                f439(v3, true)
            }
        }
    }
}

pub fn f440(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2822 = {
        let a2820 = v0.clone();
        let a2821 = {
            let (h2819, _) = v1.clone();
            h2819
        };
        index_bytes(a2820, a2821)
    };
    match m2822 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2824 = {
                    let (h2823, _) = v1.clone();
                    h2823
                };
                let a2825 = 1u64;
                nat_add(a2824, a2825)?
            }, {
                let a2828 = {
                    let (_, h2826) = v1;
                    h2826
                };
                let a2829 = f474(v0, {
                    let (h2827, _) = v1;
                    h2827
                }, v2)?;
                append_bytes(a2828, a2829)
            })))
        }
    }
}

pub fn f441(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2830 = v0.clone();
    match m2830.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2833 = {
                let (h2831, _) = v2;
                h2831
            };
            let a2834 = {
                let (h2832, _) = v1.clone();
                h2832
            };
            if equal(a2833, a2834) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f442(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f475(f476(Fn17::F477, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2839 = {
        let (h2835, _) = f478(Fn18::F479, {
            let a2837 = {
                let a2836 = v0;
                length_list(a2836)
            };
            let a2838 = 1u64;
            nat_add(a2837, a2838)?
        }, v1)?;
        h2835
    };
    match m2839.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f443(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2844 = {
        let a2841 = f333({
            let (h2840, _) = v0;
            h2840
        })?;
        let a2842 = Bytes::lit(&[0x3a]);
        append_bytes(a2841, a2842)
    };
    let a2845 = {
        let (_, h2843) = v0;
        h2843
    };
    Ok(append_bytes(a2844, a2845))
}

pub fn f444(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f347({
        let (_, h2846) = {
            let (h2847, _) = f480(Fn19::F481, {
                let a2849 = {
                    let a2848 = v0.clone();
                    length_list(a2848)
                };
                let a2850 = 1u64;
                nat_add(a2849, a2850)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2847
        };
        h2846
    }))
}

pub fn f445(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2851 = v0;
    match m2851.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f445(v3, v1)
            } else {
                List::cons(v2, f445(v3, v1))
            }
        }
    }
}

pub fn f446(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f447(v0: List<Adt14>) -> List<Bytes> {
    let m2852 = v0;
    match m2852.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2853 = f159(v1, f8(Str::lit("std")));
            match m2853 {
                None => {
                    f447(v2)
                }
                Some(v3) => {
                    List::cons(f157(v3, f8(Str::lit("authority"))), f447(v2))
                }
            }
        }
    }
}

pub fn f448(v0: List<Adt14>) -> List<Bytes> {
    let m2854 = v0;
    match m2854.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2855 = f160(f159(v1, f8(Str::lit("req"))));
            match m2855 {
                None => {
                    f448(v2)
                }
                Some(v3) => {
                    List::cons(v3, f448(v2))
                }
            }
        }
    }
}

pub fn f449(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f482(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2856) => {
            h2856
        }
    });
    let v2: u32 = f414(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2857, _, _) => {
            h2857
        }
    }, f414(f483(match v0.clone() {
        Adt27::C0(_, _, _, _, h2858, _, _, _, _, _) => {
            h2858
        }
    }), f414(f484(match v0.clone() {
        Adt27::C0(_, _, _, _, h2859, _, _, _, _, _) => {
            h2859
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2860, _, _, _, _) => {
            h2860
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2861, _, _, _) => {
            h2861
        }
    }), f414(v1, {
        let m2864 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2863, _) => {
                h2863
            }
        };
        match m2864 {
            Adt26::C0(h2862, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2862
            }
        }
    }))));
    let v3: u32 = f414(f485(match v0.clone() {
        Adt27::C0(h2865, _, _, _, _, _, _, _, _, _) => {
            h2865
        }
    }), f486(match v0.clone() {
        Adt27::C0(h2866, _, _, _, _, _, _, _, _, _) => {
            h2866
        }
    }, match v0.clone() {
        Adt27::C0(_, h2867, _, _, _, _, _, _, _, _) => {
            h2867
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2868, _, _, _, _, _, _, _) => {
            h2868
        }
    }));
    Ok(Adt27::C0(f414(v2, v3), match v0.clone() {
        Adt27::C0(h2869, _, _, _, _, _, _, _, _, _) => {
            h2869
        }
    }, match v0.clone() {
        Adt27::C0(_, h2870, _, _, _, _, _, _, _, _) => {
            h2870
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2871, _, _, _, _, _, _, _) => {
            h2871
        }
    }, f414(match v0.clone() {
        Adt27::C0(_, _, _, h2872, _, _, _, _, _, _) => {
            h2872
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2873, _, _, _, _, _) => {
            h2873
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2874, _, _, _, _) => {
            h2874
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2875, _, _, _) => {
            h2875
        }
    }, f487(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2876, _) => {
            h2876
        }
    }), {
        let a2878 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2877) => {
                h2877
            }
        };
        let a2879 = 1u64;
        nat_add(a2878, a2879)?
    }))
}

pub fn f450(v0: u32) -> u64 {
    let m2881 = {
        let a2880 = v0;
        fixed_u64::convert(i128::from(a2880))
    };
    match m2881 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f451(v0: u64) -> u32 {
    let m2885 = {
        let a2884 = {
            let a2882 = v0;
            let a2883 = 4294967295u64;
            fixed_u64::bit_and(a2882, a2883)
        };
        fixed_u32::convert(i128::from(a2884))
    };
    match m2885 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f452(v0: u64) -> u32 {
    let (_, h2886) = {
        let (_, h2887) = f488(Fn20::F489, 32u64, (v0, (1u32, 0u32)));
        h2887
    };
    h2886
}

pub fn f453(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2892 = f490(v0, 24u32);
    let a2893 = {
        let a2890 = f490(v1, 16u32);
        let a2891 = {
            let a2888 = f490(v2, 8u32);
            let a2889 = v3;
            fixed_u32::bit_or(a2888, a2889)
        };
        fixed_u32::bit_or(a2890, a2891)
    };
    fixed_u32::bit_or(a2892, a2893)
}

pub fn f454(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2894 = v2;
    let a2895 = v1;
    if nat_lt(a2894, a2895) {
        let m2898 = {
            let a2896 = v0;
            let a2897 = v2;
            index_bytes(a2896, a2897)
        };
        match m2898 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2900 = {
                    let a2899 = v3;
                    fixed_u32::convert(i128::from(a2899))
                };
                match m2900 {
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
        let a2901 = v2;
        let a2902 = v1;
        if nat_eq(a2901, a2902) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f455(v0: u8) -> bool {
    let a2903 = v0;
    let a2904 = 32u8;
    if if if equal(a2903, a2904) {
        true
    } else {
        let a2905 = v0;
        let a2906 = 9u8;
        equal(a2905, a2906)
    } {
        true
    } else {
        let a2907 = v0;
        let a2908 = 10u8;
        equal(a2907, a2908)
    } {
        true
    } else {
        let a2909 = v0;
        let a2910 = 12u8;
        if equal(a2909, a2910) {
            true
        } else {
            let a2911 = v0;
            let a2912 = 13u8;
            equal(a2911, a2912)
        }
    }
}

pub fn f456(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2915 = {
        let a2913 = v0;
        let a2914 = v1;
        index_bytes(a2913, a2914)
    };
    match m2915 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2960 = {
                    let a2936 = {
                        let a2924 = {
                            let a2918 = {
                                let a2916 = v3;
                                let a2917 = 128u8;
                                fixed_u8::bit_and(a2916, a2917)
                            };
                            let a2919 = 128u8;
                            if equal(a2918, a2919) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2925 = {
                            let a2922 = {
                                let a2920 = v3;
                                let a2921 = 64u8;
                                fixed_u8::bit_and(a2920, a2921)
                            };
                            let a2923 = 64u8;
                            if equal(a2922, a2923) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2924, a2925)?
                    };
                    let a2937 = {
                        let a2934 = {
                            let a2928 = {
                                let a2926 = v3;
                                let a2927 = 32u8;
                                fixed_u8::bit_and(a2926, a2927)
                            };
                            let a2929 = 32u8;
                            if equal(a2928, a2929) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2935 = {
                            let a2932 = {
                                let a2930 = v3;
                                let a2931 = 16u8;
                                fixed_u8::bit_and(a2930, a2931)
                            };
                            let a2933 = 16u8;
                            if equal(a2932, a2933) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2934, a2935)?
                    };
                    nat_add(a2936, a2937)?
                };
                let a2961 = {
                    let a2958 = {
                        let a2946 = {
                            let a2940 = {
                                let a2938 = v3;
                                let a2939 = 8u8;
                                fixed_u8::bit_and(a2938, a2939)
                            };
                            let a2941 = 8u8;
                            if equal(a2940, a2941) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2947 = {
                            let a2944 = {
                                let a2942 = v3;
                                let a2943 = 4u8;
                                fixed_u8::bit_and(a2942, a2943)
                            };
                            let a2945 = 4u8;
                            if equal(a2944, a2945) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2946, a2947)?
                    };
                    let a2959 = {
                        let a2956 = {
                            let a2950 = {
                                let a2948 = v3;
                                let a2949 = 2u8;
                                fixed_u8::bit_and(a2948, a2949)
                            };
                            let a2951 = 2u8;
                            if equal(a2950, a2951) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2957 = {
                            let a2954 = {
                                let a2952 = v3;
                                let a2953 = 1u8;
                                fixed_u8::bit_and(a2952, a2953)
                            };
                            let a2955 = 1u8;
                            if equal(a2954, a2955) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2956, a2957)?
                    };
                    nat_add(a2958, a2959)?
                };
                nat_add(a2960, a2961)?
            };
            let a2962 = 128u64;
            let a2963 = v4;
            if if nat_le(a2962, a2963) {
                let a2964 = v4;
                let a2965 = 191u64;
                nat_le(a2964, a2965)
            } else {
                false
            } {
                Ok(Some({
                    let a2966 = v4;
                    let a2967 = 128u64;
                    nat_sub(a2966, a2967)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f457(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a2968 = v1;
    let a2969 = v0;
    if if nat_le(a2968, a2969) {
        let a2970 = v0;
        let a2971 = v2;
        if nat_le(a2970, a2971) {
            let a2972 = v0;
            let a2973 = 55296u64;
            if nat_lt(a2972, a2973) {
                true
            } else {
                let a2974 = 57343u64;
                let a2975 = v0;
                nat_lt(a2974, a2975)
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

pub fn f458(v0: u64) -> bool {
    let a2976 = v0;
    let a2977 = 58u64;
    if if if nat_eq(a2976, a2977) {
        true
    } else {
        let a2978 = v0;
        let a2979 = 95u64;
        nat_eq(a2978, a2979)
    } {
        true
    } else {
        let a2980 = 65u64;
        let a2981 = v0;
        if if nat_le(a2980, a2981) {
            let a2982 = v0;
            let a2983 = 90u64;
            nat_le(a2982, a2983)
        } else {
            false
        } {
            true
        } else {
            let a2984 = 97u64;
            let a2985 = v0;
            if nat_le(a2984, a2985) {
                let a2986 = v0;
                let a2987 = 122u64;
                nat_le(a2986, a2987)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2988 = 192u64;
        let a2989 = v0;
        if if if if nat_le(a2988, a2989) {
            let a2990 = v0;
            let a2991 = 214u64;
            nat_le(a2990, a2991)
        } else {
            false
        } {
            true
        } else {
            let a2992 = 216u64;
            let a2993 = v0;
            if nat_le(a2992, a2993) {
                let a2994 = v0;
                let a2995 = 246u64;
                nat_le(a2994, a2995)
            } else {
                false
            }
        } {
            true
        } else {
            let a2996 = 248u64;
            let a2997 = v0;
            if if nat_le(a2996, a2997) {
                let a2998 = v0;
                let a2999 = 767u64;
                nat_le(a2998, a2999)
            } else {
                false
            } {
                true
            } else {
                let a3000 = 880u64;
                let a3001 = v0;
                if nat_le(a3000, a3001) {
                    let a3002 = v0;
                    let a3003 = 893u64;
                    nat_le(a3002, a3003)
                } else {
                    false
                }
            }
        } {
            true
        } else {
            let a3004 = 895u64;
            let a3005 = v0;
            if if if if nat_le(a3004, a3005) {
                let a3006 = v0;
                let a3007 = 8191u64;
                nat_le(a3006, a3007)
            } else {
                false
            } {
                true
            } else {
                let a3008 = 8204u64;
                let a3009 = v0;
                if nat_le(a3008, a3009) {
                    let a3010 = v0;
                    let a3011 = 8205u64;
                    nat_le(a3010, a3011)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3012 = 8304u64;
                let a3013 = v0;
                if if nat_le(a3012, a3013) {
                    let a3014 = v0;
                    let a3015 = 8591u64;
                    nat_le(a3014, a3015)
                } else {
                    false
                } {
                    true
                } else {
                    let a3016 = 11264u64;
                    let a3017 = v0;
                    if nat_le(a3016, a3017) {
                        let a3018 = v0;
                        let a3019 = 12271u64;
                        nat_le(a3018, a3019)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a3020 = 12289u64;
                let a3021 = v0;
                if if if nat_le(a3020, a3021) {
                    let a3022 = v0;
                    let a3023 = 55295u64;
                    nat_le(a3022, a3023)
                } else {
                    false
                } {
                    true
                } else {
                    let a3024 = 63744u64;
                    let a3025 = v0;
                    if nat_le(a3024, a3025) {
                        let a3026 = v0;
                        let a3027 = 64975u64;
                        nat_le(a3026, a3027)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a3028 = 65008u64;
                    let a3029 = v0;
                    if if nat_le(a3028, a3029) {
                        let a3030 = v0;
                        let a3031 = 65533u64;
                        nat_le(a3030, a3031)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a3032 = 65536u64;
                        let a3033 = v0;
                        if nat_le(a3032, a3033) {
                            let a3034 = v0;
                            let a3035 = 983039u64;
                            nat_le(a3034, a3035)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f459(v0: u64) -> bool {
    if f458(v0) {
        true
    } else {
        let a3036 = v0;
        let a3037 = 45u64;
        if if if nat_eq(a3036, a3037) {
            true
        } else {
            let a3038 = v0;
            let a3039 = 46u64;
            nat_eq(a3038, a3039)
        } {
            true
        } else {
            let a3040 = 48u64;
            let a3041 = v0;
            if nat_le(a3040, a3041) {
                let a3042 = v0;
                let a3043 = 57u64;
                nat_le(a3042, a3043)
            } else {
                false
            }
        } {
            true
        } else {
            let a3044 = v0;
            let a3045 = 183u64;
            if if nat_eq(a3044, a3045) {
                true
            } else {
                let a3046 = 768u64;
                let a3047 = v0;
                if nat_le(a3046, a3047) {
                    let a3048 = v0;
                    let a3049 = 879u64;
                    nat_le(a3048, a3049)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3050 = 8255u64;
                let a3051 = v0;
                if nat_le(a3050, a3051) {
                    let a3052 = v0;
                    let a3053 = 8256u64;
                    nat_le(a3052, a3053)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f460(v0: Fn16, p3058: u64, p3059: (u64, u64)) -> R<((u64, u64), bool)> {
    let m3057 = (p3058, p3059);
    {
        let mut m3057 = m3057;
        loop {
            let step = {
                let (v1, v2) = m3057;
                let m3054 = v1;
                if m3054 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3054 - 1;
                    let m3056 = {
                        let c3055: Fn16 = v0.clone();
                        c3055.apply(v2)?
                    };
                    match m3056 {
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
                Ok(next) => { m3057 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f491(v1, v0, v2)
}

pub fn f462(v0: u64) -> Bytes {
    f492(f493({
        let a3060 = v0;
        let a3061 = 16u64;
        let a3062 = 0u64;
        nat_quot(a3060, a3061, a3062)
    }), {
        let a3063 = v0;
        let a3064 = 16u64;
        let a3065 = 0u64;
        nat_rem(a3063, a3064, a3065)
    })
}

pub fn f463(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f494(f226(v0, v3)?, v1, v2) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3066 = v3;
            let a3067 = 1u64;
            nat_add(a3066, a3067)?
        }))
    }
}

pub fn f464(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h3068, _) = v1.clone();
        h3068
    };
    let v3: Bytes = {
        let (h3069, _) = {
            let (_, h3070) = v1.clone();
            h3070
        };
        h3069
    };
    let a3073 = {
        let (_, h3071) = {
            let (_, h3072) = v1;
            h3072
        };
        h3071
    };
    let a3074 = 0u64;
    if nat_eq(a3073, a3074) {
        let v4: u64 = f312(v0.clone(), v2)?;
        let a3075 = v4;
        let a3076 = 34u64;
        if nat_eq(a3075, a3076) {
            Ok(Some(({
                let a3077 = v2;
                let a3078 = 1u64;
                nat_add(a3077, a3078)?
            }, (v3, 1u64))))
        } else {
            let a3079 = v4;
            let a3080 = 92u64;
            if nat_eq(a3079, a3080) {
                f495(v0, v2, v3)
            } else {
                let a3081 = v4;
                let a3082 = 32u64;
                if if nat_lt(a3081, a3082) {
                    true
                } else {
                    let a3083 = v4;
                    let a3084 = 256u64;
                    nat_eq(a3083, a3084)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a3085 = v2;
                        let a3086 = 1u64;
                        nat_add(a3085, a3086)?
                    }, ({
                        let a3087 = v3;
                        let a3088 = f32(v0, v2, 1u64);
                        append_bytes(a3087, a3088)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f465(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f496(v0, v1)
}

pub fn f466(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f497(v0, v1)
}

pub fn f467(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3089 = v0.clone();
        length_bytes(a3089)
    };
    let a3090 = 0u64;
    let a3091 = v1;
    if nat_lt(a3090, a3091) {
        let a3092 = f498(v0.clone())?;
        let a3093 = v1;
        if nat_eq(a3092, a3093) {
            let a3094 = 1u64;
            let a3095 = v1;
            if nat_lt(a3094, a3095) {
                let m3098 = {
                    let a3096 = v0;
                    let a3097 = 0u64;
                    index_bytes(a3096, a3097)
                };
                match m3098 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3099 = v2;
                        let a3100 = 48u8;
                        Ok(!equal(a3099, a3100))
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

pub fn f468(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f499(v0, v1)
}

pub fn f469(v0: Bytes) -> R<u64> {
    let (h3101, _) = f253(Fn3::F500(v0.clone()), {
        let a3103 = {
            let a3102 = v0;
            length_bytes(a3102)
        };
        let a3104 = 1u64;
        nat_add(a3103, a3104)?
    }, 0u64)?;
    Ok(h3101)
}

pub fn f470(p3107: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m3106 = p3107;
    {
        let mut m3106 = m3106;
        loop {
            let step = {
                let v0 = m3106;
                let m3105 = v0;
                match m3105.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f36(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f86(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f86(v4, v3.clone())
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
                Ok(next) => { m3106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(v0: Adt19, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3108 = v0;
    match m3108 {
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
        Adt19::C4(r3111) => {
            let v2 = (*r3111).clone();
            f471(v2, v1)
        }
        Adt19::C5(r3112) => {
            let v3 = (*r3112).clone();
            f471(v3, v1)
        }
        Adt19::C6 => {
            Ok(v1)
        }
        Adt19::C7(r3113, r3114) => {
            let v4 = (*r3113).clone();
            let v5 = (*r3114).clone();
            f471(v4, f471(v5, v1)?)
        }
        Adt19::C8(v6, r3115, r3116) => {
            let v7 = (*r3115).clone();
            let v8 = (*r3116).clone();
            let v9: List<Bytes> = f471(v7.clone(), f471(v8, v1)?)?;
            if f36(v6, f8(Str::lit("$ref"))) {
                let m3109 = f245(Some(v7));
                match m3109 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m3110 = f501(v10)?;
                        match m3110 {
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

pub fn f472(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3117 = f147(v1.clone(), f8(Str::lit("enum")));
    match m3117 {
        None => {
            let m3119 = f147(v1.clone(), f8(Str::lit("oneOf")));
            match m3119 {
                None => {
                    if f506(v1.clone(), v0.clone())? {
                        let m3120 = f507(v0.clone(), f388(f147(v1.clone(), f8(Str::lit("properties")))), f151(f152(f147(v1, f8(Str::lit("required"))))), true)?;
                        match m3120 {
                            Ok(v7) => {
                                Ok(f504(f508(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f214({
                                    let (h3121, _) = v7;
                                    h3121
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h3122) = v7;
                                    h3122
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt25>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m3123 = f388(f147(v1.clone(), f8(Str::lit("properties"))));
                        match m3123.uncons() {
                            None => {
                                let m3126 = f509(v1, {
                                    let a3124 = v0.clone();
                                    let a3125 = f8(Str::lit("Value"));
                                    append_bytes(a3124, a3125)
                                })?;
                                match m3126 {
                                    Ok(v9) => {
                                        Ok(f504(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h3127, _) = v9;
                                            h3127
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3128) = v9;
                                            h3128
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m3129 = f507(v0.clone(), f388(f147(v1.clone(), f8(Str::lit("properties")))), f151(f152(f147(v1, f8(Str::lit("required"))))), false)?;
                                match m3129 {
                                    Ok(v13) => {
                                        Ok(f504(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m3131 = {
                                                let (h3130, _) = v13.clone();
                                                h3130
                                            };
                                            match m3131.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f214({
                                                        let (h3132, _) = v13;
                                                        h3132
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3133) = v13;
                                            h3133
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
                    f505(v0, f246(v5))
                }
            }
        }
        Some(v2) => {
            let m3118 = f502(f246(v2), List::<Bytes>::nil())?;
            match m3118 {
                Ok(v4) => {
                    Ok(f504(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f214(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f503(v3, v0))
                }
            }
        }
    }
}

pub fn f473(v0: Adt19) -> Adt19 {
    let v2: Adt19 = {
        let m3134 = f147(v0.clone(), f8(Str::lit("anyOf")));
        match m3134 {
            None => {
                v0
            }
            Some(v1) => {
                if f510(f246(v1.clone())) {
                    f511(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f512(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f513(f246(v1.clone()))
                    } {
                        v0
                    } else {
                        f514(f511(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt19 = {
        let m3135 = f147(v2.clone(), f8(Str::lit("oneOf")));
        match m3135 {
            None => {
                v2
            }
            Some(v3) => {
                if f510(f246(v3)) {
                    f511(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m3136 = f147(v4.clone(), f8(Str::lit("allOf")));
    match m3136 {
        None => {
            v4
        }
        Some(v5) => {
            if f515(f246(v5)) {
                f511(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f474(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3181 = {
            let a3157 = {
                let a3145 = {
                    let a3139 = {
                        let a3137 = v3;
                        let a3138 = 128u8;
                        fixed_u8::bit_and(a3137, a3138)
                    };
                    let a3140 = 128u8;
                    if equal(a3139, a3140) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3146 = {
                    let a3143 = {
                        let a3141 = v3;
                        let a3142 = 64u8;
                        fixed_u8::bit_and(a3141, a3142)
                    };
                    let a3144 = 64u8;
                    if equal(a3143, a3144) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3145, a3146)?
            };
            let a3158 = {
                let a3155 = {
                    let a3149 = {
                        let a3147 = v3;
                        let a3148 = 32u8;
                        fixed_u8::bit_and(a3147, a3148)
                    };
                    let a3150 = 32u8;
                    if equal(a3149, a3150) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3156 = {
                    let a3153 = {
                        let a3151 = v3;
                        let a3152 = 16u8;
                        fixed_u8::bit_and(a3151, a3152)
                    };
                    let a3154 = 16u8;
                    if equal(a3153, a3154) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3155, a3156)?
            };
            nat_add(a3157, a3158)?
        };
        let a3182 = {
            let a3179 = {
                let a3167 = {
                    let a3161 = {
                        let a3159 = v3;
                        let a3160 = 8u8;
                        fixed_u8::bit_and(a3159, a3160)
                    };
                    let a3162 = 8u8;
                    if equal(a3161, a3162) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3168 = {
                    let a3165 = {
                        let a3163 = v3;
                        let a3164 = 4u8;
                        fixed_u8::bit_and(a3163, a3164)
                    };
                    let a3166 = 4u8;
                    if equal(a3165, a3166) {
                        4u64
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
                        let a3170 = 2u8;
                        fixed_u8::bit_and(a3169, a3170)
                    };
                    let a3172 = 2u8;
                    if equal(a3171, a3172) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3178 = {
                    let a3175 = {
                        let a3173 = v3;
                        let a3174 = 1u8;
                        fixed_u8::bit_and(a3173, a3174)
                    };
                    let a3176 = 1u8;
                    if equal(a3175, a3176) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3177, a3178)?
            };
            nat_add(a3179, a3180)?
        };
        nat_add(a3181, a3182)?
    };
    let a3183 = v4;
    let a3184 = 34u64;
    if nat_eq(a3183, a3184) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3185 = v4;
        let a3186 = 92u64;
        if nat_eq(a3185, a3186) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3187 = v4;
            let a3188 = 8u64;
            if nat_eq(a3187, a3188) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3189 = v4;
                let a3190 = 12u64;
                if nat_eq(a3189, a3190) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3191 = v4;
                    let a3192 = 10u64;
                    if nat_eq(a3191, a3192) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3193 = v4;
                        let a3194 = 13u64;
                        if nat_eq(a3193, a3194) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3195 = v4;
                            let a3196 = 9u64;
                            if nat_eq(a3195, a3196) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3197 = v4;
                                let a3198 = 32u64;
                                if nat_lt(a3197, a3198) {
                                    let a3199 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3200 = f516(v2)?;
                                    Ok(append_bytes(a3199, a3200))
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

pub fn f475(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f517(Fn21::F518, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f476(v0: Fn17, p3204: List<List<(Bytes, Bytes)>>, p3205: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3203 = (p3204, p3205);
    {
        let mut m3203 = m3203;
        loop {
            let step = {
                let (v1, v2) = m3203;
                let m3201 = v2;
                match m3201.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3202: Fn17 = v0.clone();
                            c3202.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3203 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f478(v0: Fn18, p3210: u64, p3211: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3209 = (p3210, p3211);
    {
        let mut m3209 = m3209;
        loop {
            let step = {
                let (v1, v2) = m3209;
                let m3206 = v1;
                if m3206 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3206 - 1;
                    let m3208 = {
                        let c3207: Fn18 = v0.clone();
                        c3207.apply(v2.clone())?
                    };
                    match m3208 {
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
                Ok(next) => { m3209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f519(v0)
}

pub fn f480(v0: Fn19, p3216: u64, p3217: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m3215 = (p3216, p3217);
    {
        let mut m3215 = m3215;
        loop {
            let step = {
                let (v1, v2) = m3215;
                let m3212 = v1;
                if m3212 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3212 - 1;
                    let m3214 = {
                        let c3213: Fn19 = v0.clone();
                        c3213.apply(v2.clone())?
                    };
                    match m3214 {
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
                Ok(next) => { m3215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f520(v0)
}

pub fn f482(v0: u64) -> u32 {
    let a3218 = v0;
    let a3219 = 32u64;
    if nat_lt(a3218, a3219) {
        let a3220 = v0;
        let a3221 = 16u64;
        if nat_lt(a3220, a3221) {
            let a3222 = v0;
            let a3223 = 8u64;
            if nat_lt(a3222, a3223) {
                let a3224 = v0;
                let a3225 = 4u64;
                if nat_lt(a3224, a3225) {
                    let a3226 = v0;
                    let a3227 = 2u64;
                    if nat_lt(a3226, a3227) {
                        let a3228 = v0;
                        let a3229 = 1u64;
                        if nat_lt(a3228, a3229) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3230 = v0;
                        let a3231 = 3u64;
                        if nat_lt(a3230, a3231) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3232 = v0;
                    let a3233 = 6u64;
                    if nat_lt(a3232, a3233) {
                        let a3234 = v0;
                        let a3235 = 5u64;
                        if nat_lt(a3234, a3235) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3236 = v0;
                        let a3237 = 7u64;
                        if nat_lt(a3236, a3237) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3238 = v0;
                let a3239 = 12u64;
                if nat_lt(a3238, a3239) {
                    let a3240 = v0;
                    let a3241 = 10u64;
                    if nat_lt(a3240, a3241) {
                        let a3242 = v0;
                        let a3243 = 9u64;
                        if nat_lt(a3242, a3243) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3244 = v0;
                        let a3245 = 11u64;
                        if nat_lt(a3244, a3245) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3246 = v0;
                    let a3247 = 14u64;
                    if nat_lt(a3246, a3247) {
                        let a3248 = v0;
                        let a3249 = 13u64;
                        if nat_lt(a3248, a3249) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3250 = v0;
                        let a3251 = 15u64;
                        if nat_lt(a3250, a3251) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3252 = v0;
            let a3253 = 24u64;
            if nat_lt(a3252, a3253) {
                let a3254 = v0;
                let a3255 = 20u64;
                if nat_lt(a3254, a3255) {
                    let a3256 = v0;
                    let a3257 = 18u64;
                    if nat_lt(a3256, a3257) {
                        let a3258 = v0;
                        let a3259 = 17u64;
                        if nat_lt(a3258, a3259) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3260 = v0;
                        let a3261 = 19u64;
                        if nat_lt(a3260, a3261) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3262 = v0;
                    let a3263 = 22u64;
                    if nat_lt(a3262, a3263) {
                        let a3264 = v0;
                        let a3265 = 21u64;
                        if nat_lt(a3264, a3265) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3266 = v0;
                        let a3267 = 23u64;
                        if nat_lt(a3266, a3267) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3268 = v0;
                let a3269 = 28u64;
                if nat_lt(a3268, a3269) {
                    let a3270 = v0;
                    let a3271 = 26u64;
                    if nat_lt(a3270, a3271) {
                        let a3272 = v0;
                        let a3273 = 25u64;
                        if nat_lt(a3272, a3273) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3274 = v0;
                        let a3275 = 27u64;
                        if nat_lt(a3274, a3275) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3276 = v0;
                    let a3277 = 30u64;
                    if nat_lt(a3276, a3277) {
                        let a3278 = v0;
                        let a3279 = 29u64;
                        if nat_lt(a3278, a3279) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3280 = v0;
                        let a3281 = 31u64;
                        if nat_lt(a3280, a3281) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3282 = v0;
        let a3283 = 48u64;
        if nat_lt(a3282, a3283) {
            let a3284 = v0;
            let a3285 = 40u64;
            if nat_lt(a3284, a3285) {
                let a3286 = v0;
                let a3287 = 36u64;
                if nat_lt(a3286, a3287) {
                    let a3288 = v0;
                    let a3289 = 34u64;
                    if nat_lt(a3288, a3289) {
                        let a3290 = v0;
                        let a3291 = 33u64;
                        if nat_lt(a3290, a3291) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3292 = v0;
                        let a3293 = 35u64;
                        if nat_lt(a3292, a3293) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3294 = v0;
                    let a3295 = 38u64;
                    if nat_lt(a3294, a3295) {
                        let a3296 = v0;
                        let a3297 = 37u64;
                        if nat_lt(a3296, a3297) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3298 = v0;
                        let a3299 = 39u64;
                        if nat_lt(a3298, a3299) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3300 = v0;
                let a3301 = 44u64;
                if nat_lt(a3300, a3301) {
                    let a3302 = v0;
                    let a3303 = 42u64;
                    if nat_lt(a3302, a3303) {
                        let a3304 = v0;
                        let a3305 = 41u64;
                        if nat_lt(a3304, a3305) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3306 = v0;
                        let a3307 = 43u64;
                        if nat_lt(a3306, a3307) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3308 = v0;
                    let a3309 = 46u64;
                    if nat_lt(a3308, a3309) {
                        let a3310 = v0;
                        let a3311 = 45u64;
                        if nat_lt(a3310, a3311) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3312 = v0;
                        let a3313 = 47u64;
                        if nat_lt(a3312, a3313) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3314 = v0;
            let a3315 = 56u64;
            if nat_lt(a3314, a3315) {
                let a3316 = v0;
                let a3317 = 52u64;
                if nat_lt(a3316, a3317) {
                    let a3318 = v0;
                    let a3319 = 50u64;
                    if nat_lt(a3318, a3319) {
                        let a3320 = v0;
                        let a3321 = 49u64;
                        if nat_lt(a3320, a3321) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3322 = v0;
                        let a3323 = 51u64;
                        if nat_lt(a3322, a3323) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3324 = v0;
                    let a3325 = 54u64;
                    if nat_lt(a3324, a3325) {
                        let a3326 = v0;
                        let a3327 = 53u64;
                        if nat_lt(a3326, a3327) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3328 = v0;
                        let a3329 = 55u64;
                        if nat_lt(a3328, a3329) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3330 = v0;
                let a3331 = 60u64;
                if nat_lt(a3330, a3331) {
                    let a3332 = v0;
                    let a3333 = 58u64;
                    if nat_lt(a3332, a3333) {
                        let a3334 = v0;
                        let a3335 = 57u64;
                        if nat_lt(a3334, a3335) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3336 = v0;
                        let a3337 = 59u64;
                        if nat_lt(a3336, a3337) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3338 = v0;
                    let a3339 = 62u64;
                    if nat_lt(a3338, a3339) {
                        let a3340 = v0;
                        let a3341 = 61u64;
                        if nat_lt(a3340, a3341) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3342 = v0;
                        let a3343 = 63u64;
                        if nat_lt(a3342, a3343) {
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

pub fn f483(v0: u32) -> u32 {
    let a3346 = {
        let a3344 = f521(v0, 6u32, 26u32);
        let a3345 = f521(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3344, a3345)
    };
    let a3347 = f521(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3346, a3347)
}

pub fn f484(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3353 = {
        let a3348 = v0;
        let a3349 = v1;
        fixed_u32::bit_and(a3348, a3349)
    };
    let a3354 = {
        let a3351 = {
            let a3350 = v0;
            fixed_u32::bit_not(a3350)
        };
        let a3352 = v2;
        fixed_u32::bit_and(a3351, a3352)
    };
    fixed_u32::bit_xor(a3353, a3354)
}

pub fn f485(v0: u32) -> u32 {
    let a3357 = {
        let a3355 = f521(v0, 2u32, 30u32);
        let a3356 = f521(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3355, a3356)
    };
    let a3358 = f521(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3357, a3358)
}

pub fn f486(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3367 = {
        let a3363 = {
            let a3359 = v0;
            let a3360 = v1;
            fixed_u32::bit_and(a3359, a3360)
        };
        let a3364 = {
            let a3361 = v0;
            let a3362 = v2;
            fixed_u32::bit_and(a3361, a3362)
        };
        fixed_u32::bit_xor(a3363, a3364)
    };
    let a3368 = {
        let a3365 = v1;
        let a3366 = v2;
        fixed_u32::bit_and(a3365, a3366)
    };
    fixed_u32::bit_xor(a3367, a3368)
}

pub fn f487(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3369, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3369
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3370, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3370
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3371, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3371
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3372, _, _, _, _, _, _, _, _, _, _, _) => {
            h3372
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3373, _, _, _, _, _, _, _, _, _, _) => {
            h3373
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3374, _, _, _, _, _, _, _, _, _) => {
            h3374
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3375, _, _, _, _, _, _, _, _) => {
            h3375
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3376, _, _, _, _, _, _, _) => {
            h3376
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3377, _, _, _, _, _, _) => {
            h3377
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3378, _, _, _, _, _) => {
            h3378
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3379, _, _, _, _) => {
            h3379
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3380, _, _, _) => {
            h3380
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3381, _, _) => {
            h3381
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3382, _) => {
            h3382
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3383) => {
            h3383
        }
    }, f414(f522(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3384, _) => {
            h3384
        }
    }), f414(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3385, _, _, _, _, _, _) => {
            h3385
        }
    }, f414(f523(match v0.clone() {
        Adt26::C0(_, h3386, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3386
        }
    }), match v0 {
        Adt26::C0(h3387, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3387
        }
    }))))
}

pub fn f488(v0: Fn20, p3391: u64, p3392: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3390 = (p3391, p3392);
    {
        let mut m3390 = m3390;
        loop {
            let step = {
                let (v1, v2) = m3390;
                let m3388 = v1;
                if m3388 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3388 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3389: Fn20 = v0.clone();
                        c3389.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3393, _) = v0;
        h3393
    };
    let v2: u32 = {
        let (h3394, _) = {
            let (_, h3395) = v0;
            h3395
        };
        h3394
    };
    let v3: u32 = {
        let (_, h3396) = {
            let (_, h3397) = v0;
            h3397
        };
        h3396
    };
    ({
        let a3398 = v1;
        let a3399 = 2u64;
        let a3400 = 0u64;
        nat_quot(a3398, a3399, a3400)
    }, (f490(v2, 1u32), {
        let a3404 = {
            let a3401 = v1;
            let a3402 = 2u64;
            let a3403 = 0u64;
            nat_rem(a3401, a3402, a3403)
        };
        let a3405 = 1u64;
        if nat_eq(a3404, a3405) {
            let a3406 = v3;
            let a3407 = v2;
            fixed_u32::bit_or(a3406, a3407)
        } else {
            v3
        }
    }))
}

pub fn f490(v0: u32, v1: u32) -> u32 {
    let m3410 = {
        let a3408 = v0;
        let a3409 = v1;
        fixed_u32::shift_left(a3408, a3409)
    };
    match m3410 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f491(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f226(v0, {
        let (h3411, _) = v2;
        h3411
    })?;
    let a3413 = 1114111u64;
    let a3414 = {
        let (_, h3412) = v2;
        h3412
    };
    if nat_lt(a3413, a3414) {
        Ok(None::<(u64, u64)>)
    } else {
        let a3415 = 48u64;
        let a3416 = v3;
        if if nat_le(a3415, a3416) {
            let a3417 = v3;
            let a3418 = 57u64;
            nat_le(a3417, a3418)
        } else {
            false
        } {
            Ok(Some(({
                let a3420 = {
                    let (h3419, _) = v2;
                    h3419
                };
                let a3421 = 1u64;
                nat_add(a3420, a3421)?
            }, {
                let a3427 = {
                    let a3423 = {
                        let (_, h3422) = v2;
                        h3422
                    };
                    let a3424 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3423, a3424)?
                };
                let a3428 = {
                    let a3425 = v3;
                    let a3426 = 48u64;
                    nat_sub(a3425, a3426)
                };
                nat_add(a3427, a3428)?
            })))
        } else {
            if if v1 {
                let a3429 = 97u64;
                let a3430 = v3;
                if nat_le(a3429, a3430) {
                    let a3431 = v3;
                    let a3432 = 102u64;
                    nat_le(a3431, a3432)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some(({
                    let a3434 = {
                        let (h3433, _) = v2;
                        h3433
                    };
                    let a3435 = 1u64;
                    nat_add(a3434, a3435)?
                }, {
                    let a3441 = {
                        let a3437 = {
                            let (_, h3436) = v2;
                            h3436
                        };
                        let a3438 = 16u64;
                        nat_mul(a3437, a3438)?
                    };
                    let a3442 = {
                        let a3439 = v3;
                        let a3440 = 87u64;
                        nat_sub(a3439, a3440)
                    };
                    nat_add(a3441, a3442)?
                })))
            } else {
                if if v1 {
                    let a3443 = 65u64;
                    let a3444 = v3;
                    if nat_le(a3443, a3444) {
                        let a3445 = v3;
                        let a3446 = 70u64;
                        nat_le(a3445, a3446)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(({
                        let a3448 = {
                            let (h3447, _) = v2;
                            h3447
                        };
                        let a3449 = 1u64;
                        nat_add(a3448, a3449)?
                    }, {
                        let a3455 = {
                            let a3451 = {
                                let (_, h3450) = v2;
                                h3450
                            };
                            let a3452 = 16u64;
                            nat_mul(a3451, a3452)?
                        };
                        let a3456 = {
                            let a3453 = v3;
                            let a3454 = 55u64;
                            nat_sub(a3453, a3454)
                        };
                        nat_add(a3455, a3456)?
                    })))
                } else {
                    Ok(None::<(u64, u64)>)
                }
            }
        }
    }
}

pub fn f492(p3463: List<Bytes>, p3464: u64) -> Bytes {
    let m3462 = (p3463, p3464);
    {
        let mut m3462 = m3462;
        loop {
            let step = {
                let (v0, v1) = m3462;
                let m3457 = v0;
                match m3457.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a3458 = v1;
                        let a3459 = 0u64;
                        if nat_eq(a3458, a3459) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a3460 = v1;
                                let a3461 = 1u64;
                                nat_sub(a3460, a3461)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: u64) -> List<Bytes> {
    let a3465 = v0;
    let a3466 = 0u64;
    if nat_eq(a3465, a3466) {
        f524()
    } else {
        let a3467 = v0;
        let a3468 = 1u64;
        if nat_eq(a3467, a3468) {
            f525()
        } else {
            let a3469 = v0;
            let a3470 = 2u64;
            if nat_eq(a3469, a3470) {
                f526()
            } else {
                let a3471 = v0;
                let a3472 = 3u64;
                if nat_eq(a3471, a3472) {
                    f527()
                } else {
                    let a3473 = v0;
                    let a3474 = 4u64;
                    if nat_eq(a3473, a3474) {
                        f528()
                    } else {
                        let a3475 = v0;
                        let a3476 = 5u64;
                        if nat_eq(a3475, a3476) {
                            f529()
                        } else {
                            let a3477 = v0;
                            let a3478 = 6u64;
                            if nat_eq(a3477, a3478) {
                                f530()
                            } else {
                                let a3479 = v0;
                                let a3480 = 7u64;
                                if nat_eq(a3479, a3480) {
                                    f531()
                                } else {
                                    let a3481 = v0;
                                    let a3482 = 8u64;
                                    if nat_eq(a3481, a3482) {
                                        f532()
                                    } else {
                                        let a3483 = v0;
                                        let a3484 = 9u64;
                                        if nat_eq(a3483, a3484) {
                                            f533()
                                        } else {
                                            let a3485 = v0;
                                            let a3486 = 10u64;
                                            if nat_eq(a3485, a3486) {
                                                f534()
                                            } else {
                                                let a3487 = v0;
                                                let a3488 = 11u64;
                                                if nat_eq(a3487, a3488) {
                                                    f535()
                                                } else {
                                                    let a3489 = v0;
                                                    let a3490 = 12u64;
                                                    if nat_eq(a3489, a3490) {
                                                        f536()
                                                    } else {
                                                        let a3491 = v0;
                                                        let a3492 = 13u64;
                                                        if nat_eq(a3491, a3492) {
                                                            f537()
                                                        } else {
                                                            let a3493 = v0;
                                                            let a3494 = 14u64;
                                                            if nat_eq(a3493, a3494) {
                                                                f538()
                                                            } else {
                                                                f539()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f494(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        let a3495 = v0;
        let a3496 = v2;
        if if if nat_eq(a3495, a3496) {
            true
        } else {
            let a3497 = v0;
            let a3498 = 60u64;
            nat_eq(a3497, a3498)
        } {
            true
        } else {
            let a3499 = v0;
            let a3500 = 38u64;
            if nat_eq(a3499, a3500) {
                true
            } else {
                let a3501 = v0;
                let a3502 = 256u64;
                nat_eq(a3501, a3502)
            }
        } {
            true
        } else {
            let a3503 = v0;
            let a3504 = 9u64;
            if nat_eq(a3503, a3504) {
                true
            } else {
                let a3505 = v0;
                let a3506 = 10u64;
                nat_eq(a3505, a3506)
            }
        }
    } else {
        let a3507 = v0;
        let a3508 = 60u64;
        if if nat_eq(a3507, a3508) {
            true
        } else {
            let a3509 = v0;
            let a3510 = 38u64;
            nat_eq(a3509, a3510)
        } {
            true
        } else {
            let a3511 = v0;
            let a3512 = 93u64;
            if nat_eq(a3511, a3512) {
                true
            } else {
                let a3513 = v0;
                let a3514 = 256u64;
                nat_eq(a3513, a3514)
            }
        }
    }
}

pub fn f495(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f312(v0.clone(), {
        let a3515 = v1;
        let a3516 = 1u64;
        nat_add(a3515, a3516)?
    })?;
    let v4: Bytes = {
        let a3517 = v3;
        let a3518 = 117u64;
        if nat_eq(a3517, a3518) {
            f540(v0, v1)?
        } else {
            f541(v3)
        }
    };
    let a3519 = v4.clone();
    let a3520 = Bytes::lit(&[]);
    if equal(a3519, a3520) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a3521 = v3;
            let a3522 = 117u64;
            if nat_eq(a3521, a3522) {
                let a3523 = v1;
                let a3524 = 6u64;
                nat_add(a3523, a3524)?
            } else {
                let a3525 = v1;
                let a3526 = 2u64;
                nat_add(a3525, a3526)?
            }
        }, ({
            let a3527 = v2;
            let a3528 = v4;
            append_bytes(a3527, a3528)
        }, 0u64))))
    }
}

pub fn f496(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f312(v0, v1)?;
    let a3529 = 47u64;
    let a3530 = v2;
    if if nat_lt(a3529, a3530) {
        let a3531 = v2;
        let a3532 = 58u64;
        nat_lt(a3531, a3532)
    } else {
        false
    } {
        Ok(Some({
            let a3533 = v1;
            let a3534 = 1u64;
            nat_add(a3533, a3534)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f497(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3535 = f312(v0, v1)?;
    let a3536 = 48u64;
    if nat_eq(a3535, a3536) {
        Ok(Some({
            let a3537 = v1;
            let a3538 = 1u64;
            nat_add(a3537, a3538)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f498(v0: Bytes) -> R<u64> {
    let (h3539, _) = f253(Fn3::F542(v0.clone()), {
        let a3540 = v0;
        length_bytes(a3540)
    }, 0u64)?;
    Ok(h3539)
}

pub fn f499(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3544 = {
        let a3542 = v0;
        let a3543 = {
            let (h3541, _) = v1;
            h3541
        };
        index_bytes(a3542, a3543)
    };
    match m3544 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3546 = {
                    let (h3545, _) = v1;
                    h3545
                };
                let a3547 = 1u64;
                nat_add(a3546, a3547)?
            }, {
                let a3551 = {
                    let a3549 = {
                        let (_, h3548) = v1;
                        h3548
                    };
                    let a3550 = 10u64;
                    nat_mul(a3549, a3550)?
                };
                let a3552 = f543(v2)?;
                nat_add(a3551, a3552)?
            })))
        }
    }
}

pub fn f500(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f544(v0, v1)
}

pub fn f501(v0: Bytes) -> R<Option<Bytes>> {
    if f204(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f241(f32(v0.clone(), 8u64, {
            let a3554 = {
                let a3553 = v0;
                length_bytes(a3553)
            };
            let a3555 = 8u64;
            nat_sub(a3554, a3555)
        }))?))
    } else {
        if f204(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f241(f32(v0.clone(), 14u64, {
                let a3557 = {
                    let a3556 = v0;
                    length_bytes(a3556)
                };
                let a3558 = 14u64;
                nat_sub(a3557, a3558)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f502(v0: List<Adt19>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f545(f546(f151(v0.clone()))?, v0, v1)
}

pub fn f503(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt25>), Bytes> {
    Err::<(Bytes, List<Adt25>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f504(v0: Bytes, v1: List<Adt25>) -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((v0, v1))
}

pub fn f505(v0: Bytes, v1: List<Adt19>) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3559 = v1.clone();
    match m3559.uncons() {
        None => {
            Ok(f503(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3560 = f547(f388(f147(v2, f8(Str::lit("properties")))), v1.clone());
            match m3560.uncons() {
                None => {
                    let m3561 = f548(f546(f549(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3561 {
                        Ok(v5) => {
                            Ok(f504(f508(v0, {
                                let (h3562, _) = v5;
                                h3562
                            }), {
                                let (_, h3563) = v5;
                                h3563
                            }))
                        }
                        Err(v4) => {
                            Ok(f503(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3564 = f550(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3564 {
                        Ok(v9) => {
                            Ok(f504(f508(v0, {
                                let (h3565, _) = v9;
                                h3565
                            }), {
                                let (_, h3566) = v9;
                                h3566
                            }))
                        }
                        Err(v8) => {
                            Ok(f503(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f506(v0: Adt19, v1: Bytes) -> R<bool> {
    let m3567 = v0;
    match m3567 {
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
        Adt19::C4(r3568) => {
            let v2 = (*r3568).clone();
            f506(v2, v1)
        }
        Adt19::C5(r3569) => {
            let v3 = (*r3569).clone();
            f506(v3, v1)
        }
        Adt19::C6 => {
            Ok(false)
        }
        Adt19::C7(r3570, r3571) => {
            let v4 = (*r3570).clone();
            let v5 = (*r3571).clone();
            if f506(v4, v1.clone())? {
                Ok(true)
            } else {
                f506(v5, v1)
            }
        }
        Adt19::C8(v6, r3572, r3573) => {
            let v7 = (*r3572).clone();
            let v8 = (*r3573).clone();
            if if f36(v6, f8(Str::lit("$ref"))) {
                f551(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f506(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f506(v8, v1)
                }
            }
        }
    }
}

pub fn f507(v0: Bytes, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f552(f546(f553(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f508(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f214(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f509(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3574 = f554(v0.clone());
    match m3574 {
        None => {
            if f36(f150(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3575 = f147(v0, f8(Str::lit("items")));
                match m3575 {
                    None => {
                        Ok(f555(f8(Str::lit("List")), f557()))
                    }
                    Some(v3) => {
                        Ok(f555(f8(Str::lit("List")), f556(v3, {
                            let a3576 = v1;
                            let a3577 = f8(Str::lit("Item"));
                            append_bytes(a3576, a3577)
                        })?))
                    }
                }
            } else {
                f556(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f555(f8(Str::lit("Option")), f556(v2, v1)?))
        }
    }
}

pub fn f510(p3580: List<Adt19>) -> bool {
    let m3579 = p3580;
    {
        let mut m3579 = m3579;
        loop {
            let step = {
                let v0 = m3579;
                let m3578 = v0;
                match m3578.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f558(f433(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt19>, bool>(v2)
                        } else {
                            Err::<List<Adt19>, bool>(false)
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

pub fn f511(v0: Adt19, v1: Bytes) -> Adt19 {
    let m3581 = v0;
    match m3581 {
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
        Adt19::C4(r3582) => {
            let v13 = (*r3582).clone();
            Adt19::C4(std::rc::Rc::new(v13))
        }
        Adt19::C5(r3583) => {
            let v5 = (*r3583).clone();
            Adt19::C5(std::rc::Rc::new(f511(v5, v1)))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r3584, r3585) => {
            let v14 = (*r3584).clone();
            let v15 = (*r3585).clone();
            Adt19::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt19::C8(v2, r3586, r3587) => {
            let v3 = (*r3586).clone();
            let v4 = (*r3587).clone();
            if f36(v2.clone(), v1.clone()) {
                f511(v4, v1)
            } else {
                Adt19::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f511(v4, v1)))
            }
        }
    }
}

pub fn f512(v0: Adt19, v1: Bytes) -> bool {
    let m3588 = f147(v0, v1);
    match m3588 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f513(v0: List<Adt19>) -> bool {
    let m3589 = v0.clone();
    match m3589.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3590 = f547(f388(f147(v1, f8(Str::lit("properties")))), v0.clone());
            match m3590.uncons() {
                None => {
                    f560(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f559(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f514(v0: Adt19, v1: Bytes, v2: Adt19) -> Adt19 {
    let m3591 = v0;
    match m3591 {
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
        Adt19::C4(r3592) => {
            let v14 = (*r3592).clone();
            Adt19::C4(std::rc::Rc::new(v14))
        }
        Adt19::C5(r3593) => {
            let v3 = (*r3593).clone();
            Adt19::C5(std::rc::Rc::new(Adt19::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r3594, r3595) => {
            let v15 = (*r3594).clone();
            let v16 = (*r3595).clone();
            Adt19::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt19::C8(v4, r3596, r3597) => {
            let v5 = (*r3596).clone();
            let v6 = (*r3597).clone();
            Adt19::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f515(p3601: List<Adt19>) -> bool {
    let m3600 = p3601;
    {
        let mut m3600 = m3600;
        loop {
            let step = {
                let v0 = m3600;
                let m3598 = v0;
                match m3598.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f558(f433(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f510(f246({
                                let m3599 = f147(v1, f8(Str::lit("anyOf")));
                                match m3599 {
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
                Ok(next) => { m3600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3646 = {
            let a3622 = {
                let a3610 = {
                    let a3604 = {
                        let a3602 = v1;
                        let a3603 = 128u8;
                        fixed_u8::bit_and(a3602, a3603)
                    };
                    let a3605 = 128u8;
                    if equal(a3604, a3605) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3611 = {
                    let a3608 = {
                        let a3606 = v1;
                        let a3607 = 64u8;
                        fixed_u8::bit_and(a3606, a3607)
                    };
                    let a3609 = 64u8;
                    if equal(a3608, a3609) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3610, a3611)?
            };
            let a3623 = {
                let a3620 = {
                    let a3614 = {
                        let a3612 = v1;
                        let a3613 = 32u8;
                        fixed_u8::bit_and(a3612, a3613)
                    };
                    let a3615 = 32u8;
                    if equal(a3614, a3615) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3621 = {
                    let a3618 = {
                        let a3616 = v1;
                        let a3617 = 16u8;
                        fixed_u8::bit_and(a3616, a3617)
                    };
                    let a3619 = 16u8;
                    if equal(a3618, a3619) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3620, a3621)?
            };
            nat_add(a3622, a3623)?
        };
        let a3647 = {
            let a3644 = {
                let a3632 = {
                    let a3626 = {
                        let a3624 = v1;
                        let a3625 = 8u8;
                        fixed_u8::bit_and(a3624, a3625)
                    };
                    let a3627 = 8u8;
                    if equal(a3626, a3627) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3633 = {
                    let a3630 = {
                        let a3628 = v1;
                        let a3629 = 4u8;
                        fixed_u8::bit_and(a3628, a3629)
                    };
                    let a3631 = 4u8;
                    if equal(a3630, a3631) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3632, a3633)?
            };
            let a3645 = {
                let a3642 = {
                    let a3636 = {
                        let a3634 = v1;
                        let a3635 = 2u8;
                        fixed_u8::bit_and(a3634, a3635)
                    };
                    let a3637 = 2u8;
                    if equal(a3636, a3637) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3643 = {
                    let a3640 = {
                        let a3638 = v1;
                        let a3639 = 1u8;
                        fixed_u8::bit_and(a3638, a3639)
                    };
                    let a3641 = 1u8;
                    if equal(a3640, a3641) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3642, a3643)?
            };
            nat_add(a3644, a3645)?
        };
        nat_add(a3646, a3647)?
    };
    let a3654 = f316(f561(), {
        let a3648 = v2;
        let a3649 = 16u64;
        let a3650 = 0u64;
        nat_quot(a3648, a3649, a3650)
    });
    let a3655 = f316(f561(), {
        let a3651 = v2;
        let a3652 = 16u64;
        let a3653 = 0u64;
        nat_rem(a3651, a3652, a3653)
    });
    Ok(append_bytes(a3654, a3655))
}

pub fn f517(v0: Fn21, p3659: List<List<(Bytes, Bytes)>>, p3660: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3658 = (p3659, p3660);
    {
        let mut m3658 = m3658;
        loop {
            let step = {
                let (v1, v2) = m3658;
                let m3656 = v2;
                match m3656.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3657: Fn21 = v0.clone();
                            c3657.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f518(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f519(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3662 = {
        let a3661 = v0.clone();
        length_list(a3661)
    };
    let a3663 = 1u64;
    if nat_le(a3662, a3663) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f562(v0)?))
    }
}

pub fn f520(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3664, _) = v0;
        h3664
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3665) = v0;
        h3665
    };
    let m3666 = v1;
    match m3666.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3667 = v4;
            match m3667.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f563(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f521(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3668 = f362(v0, v1);
    let a3669 = f490(v0, v2);
    fixed_u32::bit_or(a3668, a3669)
}

pub fn f522(v0: u32) -> u32 {
    let a3672 = {
        let a3670 = f521(v0, 17u32, 15u32);
        let a3671 = f521(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3670, a3671)
    };
    let a3673 = f362(v0, 10u32);
    fixed_u32::bit_xor(a3672, a3673)
}

pub fn f523(v0: u32) -> u32 {
    let a3676 = {
        let a3674 = f521(v0, 7u32, 25u32);
        let a3675 = f521(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3674, a3675)
    };
    let a3677 = f362(v0, 3u32);
    fixed_u32::bit_xor(a3676, a3677)
}

pub fn f524() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x00]), List::cons(Bytes::lit(&[0x01]), List::cons(Bytes::lit(&[0x02]), List::cons(Bytes::lit(&[0x03]), List::cons(Bytes::lit(&[0x04]), List::cons(Bytes::lit(&[0x05]), List::cons(Bytes::lit(&[0x06]), List::cons(Bytes::lit(&[0x07]), List::cons(Bytes::lit(&[0x08]), List::cons(Bytes::lit(&[0x09]), List::cons(Bytes::lit(&[0x0a]), List::cons(Bytes::lit(&[0x0b]), List::cons(Bytes::lit(&[0x0c]), List::cons(Bytes::lit(&[0x0d]), List::cons(Bytes::lit(&[0x0e]), List::cons(Bytes::lit(&[0x0f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f525() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x10]), List::cons(Bytes::lit(&[0x11]), List::cons(Bytes::lit(&[0x12]), List::cons(Bytes::lit(&[0x13]), List::cons(Bytes::lit(&[0x14]), List::cons(Bytes::lit(&[0x15]), List::cons(Bytes::lit(&[0x16]), List::cons(Bytes::lit(&[0x17]), List::cons(Bytes::lit(&[0x18]), List::cons(Bytes::lit(&[0x19]), List::cons(Bytes::lit(&[0x1a]), List::cons(Bytes::lit(&[0x1b]), List::cons(Bytes::lit(&[0x1c]), List::cons(Bytes::lit(&[0x1d]), List::cons(Bytes::lit(&[0x1e]), List::cons(Bytes::lit(&[0x1f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f526() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x20]), List::cons(Bytes::lit(&[0x21]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x27]), List::cons(Bytes::lit(&[0x28]), List::cons(Bytes::lit(&[0x29]), List::cons(Bytes::lit(&[0x2a]), List::cons(Bytes::lit(&[0x2b]), List::cons(Bytes::lit(&[0x2c]), List::cons(Bytes::lit(&[0x2d]), List::cons(Bytes::lit(&[0x2e]), List::cons(Bytes::lit(&[0x2f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f527() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x30]), List::cons(Bytes::lit(&[0x31]), List::cons(Bytes::lit(&[0x32]), List::cons(Bytes::lit(&[0x33]), List::cons(Bytes::lit(&[0x34]), List::cons(Bytes::lit(&[0x35]), List::cons(Bytes::lit(&[0x36]), List::cons(Bytes::lit(&[0x37]), List::cons(Bytes::lit(&[0x38]), List::cons(Bytes::lit(&[0x39]), List::cons(Bytes::lit(&[0x3a]), List::cons(Bytes::lit(&[0x3b]), List::cons(Bytes::lit(&[0x3c]), List::cons(Bytes::lit(&[0x3d]), List::cons(Bytes::lit(&[0x3e]), List::cons(Bytes::lit(&[0x3f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f528() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x40]), List::cons(Bytes::lit(&[0x41]), List::cons(Bytes::lit(&[0x42]), List::cons(Bytes::lit(&[0x43]), List::cons(Bytes::lit(&[0x44]), List::cons(Bytes::lit(&[0x45]), List::cons(Bytes::lit(&[0x46]), List::cons(Bytes::lit(&[0x47]), List::cons(Bytes::lit(&[0x48]), List::cons(Bytes::lit(&[0x49]), List::cons(Bytes::lit(&[0x4a]), List::cons(Bytes::lit(&[0x4b]), List::cons(Bytes::lit(&[0x4c]), List::cons(Bytes::lit(&[0x4d]), List::cons(Bytes::lit(&[0x4e]), List::cons(Bytes::lit(&[0x4f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f529() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x50]), List::cons(Bytes::lit(&[0x51]), List::cons(Bytes::lit(&[0x52]), List::cons(Bytes::lit(&[0x53]), List::cons(Bytes::lit(&[0x54]), List::cons(Bytes::lit(&[0x55]), List::cons(Bytes::lit(&[0x56]), List::cons(Bytes::lit(&[0x57]), List::cons(Bytes::lit(&[0x58]), List::cons(Bytes::lit(&[0x59]), List::cons(Bytes::lit(&[0x5a]), List::cons(Bytes::lit(&[0x5b]), List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x5d]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f530() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x60]), List::cons(Bytes::lit(&[0x61]), List::cons(Bytes::lit(&[0x62]), List::cons(Bytes::lit(&[0x63]), List::cons(Bytes::lit(&[0x64]), List::cons(Bytes::lit(&[0x65]), List::cons(Bytes::lit(&[0x66]), List::cons(Bytes::lit(&[0x67]), List::cons(Bytes::lit(&[0x68]), List::cons(Bytes::lit(&[0x69]), List::cons(Bytes::lit(&[0x6a]), List::cons(Bytes::lit(&[0x6b]), List::cons(Bytes::lit(&[0x6c]), List::cons(Bytes::lit(&[0x6d]), List::cons(Bytes::lit(&[0x6e]), List::cons(Bytes::lit(&[0x6f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f531() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x70]), List::cons(Bytes::lit(&[0x71]), List::cons(Bytes::lit(&[0x72]), List::cons(Bytes::lit(&[0x73]), List::cons(Bytes::lit(&[0x74]), List::cons(Bytes::lit(&[0x75]), List::cons(Bytes::lit(&[0x76]), List::cons(Bytes::lit(&[0x77]), List::cons(Bytes::lit(&[0x78]), List::cons(Bytes::lit(&[0x79]), List::cons(Bytes::lit(&[0x7a]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7c]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x7f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f532() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x80]), List::cons(Bytes::lit(&[0x81]), List::cons(Bytes::lit(&[0x82]), List::cons(Bytes::lit(&[0x83]), List::cons(Bytes::lit(&[0x84]), List::cons(Bytes::lit(&[0x85]), List::cons(Bytes::lit(&[0x86]), List::cons(Bytes::lit(&[0x87]), List::cons(Bytes::lit(&[0x88]), List::cons(Bytes::lit(&[0x89]), List::cons(Bytes::lit(&[0x8a]), List::cons(Bytes::lit(&[0x8b]), List::cons(Bytes::lit(&[0x8c]), List::cons(Bytes::lit(&[0x8d]), List::cons(Bytes::lit(&[0x8e]), List::cons(Bytes::lit(&[0x8f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f533() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x90]), List::cons(Bytes::lit(&[0x91]), List::cons(Bytes::lit(&[0x92]), List::cons(Bytes::lit(&[0x93]), List::cons(Bytes::lit(&[0x94]), List::cons(Bytes::lit(&[0x95]), List::cons(Bytes::lit(&[0x96]), List::cons(Bytes::lit(&[0x97]), List::cons(Bytes::lit(&[0x98]), List::cons(Bytes::lit(&[0x99]), List::cons(Bytes::lit(&[0x9a]), List::cons(Bytes::lit(&[0x9b]), List::cons(Bytes::lit(&[0x9c]), List::cons(Bytes::lit(&[0x9d]), List::cons(Bytes::lit(&[0x9e]), List::cons(Bytes::lit(&[0x9f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f534() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xa0]), List::cons(Bytes::lit(&[0xa1]), List::cons(Bytes::lit(&[0xa2]), List::cons(Bytes::lit(&[0xa3]), List::cons(Bytes::lit(&[0xa4]), List::cons(Bytes::lit(&[0xa5]), List::cons(Bytes::lit(&[0xa6]), List::cons(Bytes::lit(&[0xa7]), List::cons(Bytes::lit(&[0xa8]), List::cons(Bytes::lit(&[0xa9]), List::cons(Bytes::lit(&[0xaa]), List::cons(Bytes::lit(&[0xab]), List::cons(Bytes::lit(&[0xac]), List::cons(Bytes::lit(&[0xad]), List::cons(Bytes::lit(&[0xae]), List::cons(Bytes::lit(&[0xaf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f535() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xb0]), List::cons(Bytes::lit(&[0xb1]), List::cons(Bytes::lit(&[0xb2]), List::cons(Bytes::lit(&[0xb3]), List::cons(Bytes::lit(&[0xb4]), List::cons(Bytes::lit(&[0xb5]), List::cons(Bytes::lit(&[0xb6]), List::cons(Bytes::lit(&[0xb7]), List::cons(Bytes::lit(&[0xb8]), List::cons(Bytes::lit(&[0xb9]), List::cons(Bytes::lit(&[0xba]), List::cons(Bytes::lit(&[0xbb]), List::cons(Bytes::lit(&[0xbc]), List::cons(Bytes::lit(&[0xbd]), List::cons(Bytes::lit(&[0xbe]), List::cons(Bytes::lit(&[0xbf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f536() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xc0]), List::cons(Bytes::lit(&[0xc1]), List::cons(Bytes::lit(&[0xc2]), List::cons(Bytes::lit(&[0xc3]), List::cons(Bytes::lit(&[0xc4]), List::cons(Bytes::lit(&[0xc5]), List::cons(Bytes::lit(&[0xc6]), List::cons(Bytes::lit(&[0xc7]), List::cons(Bytes::lit(&[0xc8]), List::cons(Bytes::lit(&[0xc9]), List::cons(Bytes::lit(&[0xca]), List::cons(Bytes::lit(&[0xcb]), List::cons(Bytes::lit(&[0xcc]), List::cons(Bytes::lit(&[0xcd]), List::cons(Bytes::lit(&[0xce]), List::cons(Bytes::lit(&[0xcf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f537() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xd0]), List::cons(Bytes::lit(&[0xd1]), List::cons(Bytes::lit(&[0xd2]), List::cons(Bytes::lit(&[0xd3]), List::cons(Bytes::lit(&[0xd4]), List::cons(Bytes::lit(&[0xd5]), List::cons(Bytes::lit(&[0xd6]), List::cons(Bytes::lit(&[0xd7]), List::cons(Bytes::lit(&[0xd8]), List::cons(Bytes::lit(&[0xd9]), List::cons(Bytes::lit(&[0xda]), List::cons(Bytes::lit(&[0xdb]), List::cons(Bytes::lit(&[0xdc]), List::cons(Bytes::lit(&[0xdd]), List::cons(Bytes::lit(&[0xde]), List::cons(Bytes::lit(&[0xdf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f538() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xe0]), List::cons(Bytes::lit(&[0xe1]), List::cons(Bytes::lit(&[0xe2]), List::cons(Bytes::lit(&[0xe3]), List::cons(Bytes::lit(&[0xe4]), List::cons(Bytes::lit(&[0xe5]), List::cons(Bytes::lit(&[0xe6]), List::cons(Bytes::lit(&[0xe7]), List::cons(Bytes::lit(&[0xe8]), List::cons(Bytes::lit(&[0xe9]), List::cons(Bytes::lit(&[0xea]), List::cons(Bytes::lit(&[0xeb]), List::cons(Bytes::lit(&[0xec]), List::cons(Bytes::lit(&[0xed]), List::cons(Bytes::lit(&[0xee]), List::cons(Bytes::lit(&[0xef]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f539() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xf0]), List::cons(Bytes::lit(&[0xf1]), List::cons(Bytes::lit(&[0xf2]), List::cons(Bytes::lit(&[0xf3]), List::cons(Bytes::lit(&[0xf4]), List::cons(Bytes::lit(&[0xf5]), List::cons(Bytes::lit(&[0xf6]), List::cons(Bytes::lit(&[0xf7]), List::cons(Bytes::lit(&[0xf8]), List::cons(Bytes::lit(&[0xf9]), List::cons(Bytes::lit(&[0xfa]), List::cons(Bytes::lit(&[0xfb]), List::cons(Bytes::lit(&[0xfc]), List::cons(Bytes::lit(&[0xfd]), List::cons(Bytes::lit(&[0xfe]), List::cons(Bytes::lit(&[0xff]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f540(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3680 = f32(v0.clone(), {
        let a3678 = v1;
        let a3679 = 2u64;
        nat_add(a3678, a3679)?
    }, 2u64);
    let a3681 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3680, a3681) {
        let m3684 = f564(f32(v0, {
            let a3682 = v1;
            let a3683 = 4u64;
            nat_add(a3682, a3683)?
        }, 2u64))?;
        match m3684 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f312(v2.clone(), 0u64)?;
                let a3685 = v3;
                let a3686 = 32u64;
                if nat_lt(a3685, a3686) {
                    if f565(v3) {
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

pub fn f541(v0: u64) -> Bytes {
    let a3687 = v0;
    let a3688 = 34u64;
    if nat_eq(a3687, a3688) {
        Bytes::lit(&[0x22])
    } else {
        let a3689 = v0;
        let a3690 = 92u64;
        if nat_eq(a3689, a3690) {
            Bytes::lit(&[0x5c])
        } else {
            let a3691 = v0;
            let a3692 = 98u64;
            if nat_eq(a3691, a3692) {
                Bytes::lit(&[0x08])
            } else {
                let a3693 = v0;
                let a3694 = 102u64;
                if nat_eq(a3693, a3694) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3695 = v0;
                    let a3696 = 110u64;
                    if nat_eq(a3695, a3696) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3697 = v0;
                        let a3698 = 114u64;
                        if nat_eq(a3697, a3698) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3699 = v0;
                            let a3700 = 116u64;
                            if nat_eq(a3699, a3700) {
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

pub fn f542(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3703 = {
        let a3701 = v0;
        let a3702 = v1;
        index_bytes(a3701, a3702)
    };
    match m3703 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f566(v2)? {
                Ok(Some({
                    let a3704 = v1;
                    let a3705 = 1u64;
                    nat_add(a3704, a3705)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f543(v0: u8) -> R<u64> {
    let a3752 = {
        let v1: u8 = v0;
        let a3750 = {
            let a3726 = {
                let a3714 = {
                    let a3708 = {
                        let a3706 = v1;
                        let a3707 = 128u8;
                        fixed_u8::bit_and(a3706, a3707)
                    };
                    let a3709 = 128u8;
                    if equal(a3708, a3709) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3715 = {
                    let a3712 = {
                        let a3710 = v1;
                        let a3711 = 64u8;
                        fixed_u8::bit_and(a3710, a3711)
                    };
                    let a3713 = 64u8;
                    if equal(a3712, a3713) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3714, a3715)?
            };
            let a3727 = {
                let a3724 = {
                    let a3718 = {
                        let a3716 = v1;
                        let a3717 = 32u8;
                        fixed_u8::bit_and(a3716, a3717)
                    };
                    let a3719 = 32u8;
                    if equal(a3718, a3719) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3725 = {
                    let a3722 = {
                        let a3720 = v1;
                        let a3721 = 16u8;
                        fixed_u8::bit_and(a3720, a3721)
                    };
                    let a3723 = 16u8;
                    if equal(a3722, a3723) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3724, a3725)?
            };
            nat_add(a3726, a3727)?
        };
        let a3751 = {
            let a3748 = {
                let a3736 = {
                    let a3730 = {
                        let a3728 = v1;
                        let a3729 = 8u8;
                        fixed_u8::bit_and(a3728, a3729)
                    };
                    let a3731 = 8u8;
                    if equal(a3730, a3731) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3737 = {
                    let a3734 = {
                        let a3732 = v1;
                        let a3733 = 4u8;
                        fixed_u8::bit_and(a3732, a3733)
                    };
                    let a3735 = 4u8;
                    if equal(a3734, a3735) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3736, a3737)?
            };
            let a3749 = {
                let a3746 = {
                    let a3740 = {
                        let a3738 = v1;
                        let a3739 = 2u8;
                        fixed_u8::bit_and(a3738, a3739)
                    };
                    let a3741 = 2u8;
                    if equal(a3740, a3741) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3747 = {
                    let a3744 = {
                        let a3742 = v1;
                        let a3743 = 1u8;
                        fixed_u8::bit_and(a3742, a3743)
                    };
                    let a3745 = 1u8;
                    if equal(a3744, a3745) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3746, a3747)?
            };
            nat_add(a3748, a3749)?
        };
        nat_add(a3750, a3751)?
    };
    let a3753 = 48u64;
    Ok(nat_sub(a3752, a3753))
}

pub fn f544(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3754 = v0.clone();
        length_bytes(a3754)
    };
    let a3755 = v1;
    let a3756 = v2;
    if if nat_lt(a3755, a3756) {
        let a3761 = f312(v0, {
            let a3759 = v2;
            let a3760 = {
                let a3757 = v1;
                let a3758 = 1u64;
                nat_add(a3757, a3758)?
            };
            nat_sub(a3759, a3760)
        })?;
        let a3762 = 48u64;
        nat_eq(a3761, a3762)
    } else {
        false
    } {
        Ok(Some({
            let a3763 = v1;
            let a3764 = 1u64;
            nat_add(a3763, a3764)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f545(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3765 = v1;
    match m3765.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3766 = f567(v0, v3.clone())?;
            match m3766 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f568(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f86(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3767 = f545(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3767 {
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

pub fn f546(p3771: List<Bytes>) -> R<bool> {
    let m3770 = p3771;
    {
        let mut m3770 = m3770;
        loop {
            let step = {
                let v0 = m3770;
                let m3768 = v0;
                match m3768.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3769 = f569(v1)?;
                        match m3769 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f86(v3, f570(false, v2.clone())?) {
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
                Ok(next) => { m3770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547(v0: List<(Bytes, Adt19)>, v1: List<Adt19>) -> List<Bytes> {
    let m3772 = v0;
    match m3772.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f571(v1.clone(), {
                let (h3773, _) = v2.clone();
                h3773
            }) {
                List::cons({
                    let (h3774, _) = v2;
                    h3774
                }, f547(v3, v1))
            } else {
                f547(v3, v1)
            }
        }
    }
}

pub fn f548(v0: bool, v1: Bytes, v2: List<Adt19>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m3775 = v2;
    match m3775.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3776 = f572(v4.clone())?;
            match m3776 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3777 = f573(v0, v6.clone())?;
                    match m3777 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f86(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3780 = f574(v4, {
                                    let a3778 = v1.clone();
                                    let a3779 = f241(v6)?;
                                    append_bytes(a3778, a3779)
                                })?;
                                match m3780 {
                                    Ok(v9) => {
                                        let m3781 = f548(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3781 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h3782, _) = v9;
                                                    h3782
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h3783, _) = v11;
                                                    h3783
                                                }), f392({
                                                    let (_, h3784) = v9;
                                                    h3784
                                                }, {
                                                    let (_, h3785) = v11;
                                                    h3785
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

pub fn f549(v0: List<Adt19>) -> R<List<Bytes>> {
    let m3786 = v0;
    match m3786.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3787 = f572(v1)?;
            match m3787 {
                None => {
                    f549(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f549(v2)?))
                }
            }
        }
    }
}

pub fn f550(v0: Bytes, v1: List<Adt19>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f575(f546(f576(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f551(v0: Adt19, v1: Bytes) -> R<bool> {
    let m3788 = f245(Some(v0));
    match m3788 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3789 = f501(v2)?;
            match m3789 {
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

pub fn f552(v0: bool, v1: Bytes, v2: List<(Bytes, Adt19)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m3790 = v2;
    match m3790.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3792 = f573(v0, {
                let (h3791, _) = v5.clone();
                h3791
            })?;
            match m3792 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3793, _) = v5;
                        h3793
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f86(v7.clone(), f570(v0, f553(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3798 = f509({
                            let (_, h3794) = v5.clone();
                            h3794
                        }, {
                            let a3796 = v1.clone();
                            let a3797 = f241({
                                let (h3795, _) = v5.clone();
                                h3795
                            })?;
                            append_bytes(a3796, a3797)
                        })?;
                        match m3798 {
                            Ok(v9) => {
                                let v10: Bytes = if f86({
                                    let (h3799, _) = v5;
                                    h3799
                                }, v3.clone()) {
                                    let (h3800, _) = v9;
                                    h3800
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h3801, _) = v9;
                                        h3801
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3802 = f552(v0, v1, v6, v3, v4)?;
                                match m3802 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h3803, _) = v12;
                                            h3803
                                        }), f392({
                                            let (_, h3804) = v9;
                                            h3804
                                        }, {
                                            let (_, h3805) = v12;
                                            h3805
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

pub fn f553(v0: List<(Bytes, Adt19)>) -> List<Bytes> {
    let m3806 = v0;
    match m3806.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3807, _) = v1;
                h3807
            }, f553(v2))
        }
    }
}

pub fn f554(v0: Adt19) -> Option<Adt19> {
    let m3808 = f152(f147(v0, f8(Str::lit("oneOf"))));
    match m3808.uncons() {
        None => {
            None::<Adt19>
        }
        Some((v1, v2)) => {
            let m3809 = v2;
            match m3809.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v3, v4)) => {
                    let m3810 = v4;
                    match m3810.uncons() {
                        None => {
                            if f577(v3.clone()) {
                                Some(v1)
                            } else {
                                if f577(v1) {
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

pub fn f555(v0: Bytes, v1: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m3811 = v1;
    match m3811 {
        Ok(v3) => {
            f504(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h3812, _) = v3;
                h3812
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h3813) = v3;
                h3813
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v2)
        }
    }
}

pub fn f556(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3814 = f554(v0.clone());
    match m3814 {
        None => {
            if f36(f150(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3815 = f147(v0, f8(Str::lit("items")));
                match m3815 {
                    None => {
                        Ok(f555(f8(Str::lit("List")), f557()))
                    }
                    Some(v3) => {
                        Ok(f555(f8(Str::lit("List")), f574(v3, {
                            let a3816 = v1;
                            let a3817 = f8(Str::lit("Item"));
                            append_bytes(a3816, a3817)
                        })?))
                    }
                }
            } else {
                f574(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f555(f8(Str::lit("Option")), f574(v2, v1)?))
        }
    }
}

pub fn f557() -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((f436(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f558(p3821: List<(Bytes, Adt19)>, v1: List<Bytes>) -> bool {
    let m3820 = p3821;
    {
        let mut m3820 = m3820;
        loop {
            let step = {
                let v0 = m3820;
                let m3818 = v0;
                match m3818.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f86({
                            let (h3819, _) = v2;
                            h3819
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt19)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt19)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f559(p3825: List<Adt19>, v1: Bytes, p3826: List<Bytes>) -> bool {
    let m3824 = (p3825, p3826);
    {
        let mut m3824 = m3824;
        loop {
            let step = {
                let (v0, v2) = m3824;
                let m3822 = v0;
                match m3822.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3823 = f578(v3, v1.clone());
                        match m3823 {
                            None => {
                                Err::<(List<Adt19>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f86(v5.clone(), v2.clone()) {
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
                Ok(next) => { m3824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f560(p3829: List<Adt19>, p3830: List<Bytes>) -> bool {
    let m3828 = (p3829, p3830);
    {
        let mut m3828 = m3828;
        loop {
            let step = {
                let (v0, v1) = m3828;
                let m3827 = v0;
                match m3827.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f150(v2, f8(Str::lit("type")));
                        if !f36(v4.clone(), Bytes::lit(&[])) {
                            if !f86(v4.clone(), v1.clone()) {
                                if !if if f36(v4.clone(), f8(Str::lit("integer"))) {
                                    f86(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f36(v4.clone(), f8(Str::lit("number"))) {
                                        f86(f8(Str::lit("integer")), v1.clone())
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
                Ok(next) => { m3828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f562(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f475({
        let (_, h3831) = {
            let (h3832, _) = f579(Fn22::F580, {
                let a3834 = {
                    let a3833 = v0.clone();
                    length_list(a3833)
                };
                let a3835 = 1u64;
                nat_add(a3834, a3835)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3832
        };
        h3831
    }))
}

pub fn f563(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3840 = {
            let a3838 = {
                let a3836 = v0.clone();
                length_list(a3836)
            };
            let a3839 = {
                let a3837 = v1.clone();
                length_list(a3837)
            };
            nat_add(a3838, a3839)?
        };
        let a3841 = 1u64;
        nat_add(a3840, a3841)?
    };
    Ok(f352({
        let (_, h3842) = {
            let (_, h3843) = {
                let (h3844, _) = f581(Fn23::F582, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3844
            };
            h3843
        };
        h3842
    }))
}

pub fn f564(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3845 = v0.clone();
        length_bytes(a3845)
    };
    let a3849 = {
        let a3846 = v1;
        let a3847 = 2u64;
        let a3848 = 0u64;
        nat_rem(a3846, a3847, a3848)
    };
    let a3850 = 0u64;
    if nat_eq(a3849, a3850) {
        let (_, h3851) = {
            let (h3852, _) = f583(Fn24::F584(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3852
        };
        Ok(h3851)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f565(v0: u64) -> bool {
    let a3853 = v0;
    let a3854 = 8u64;
    if nat_eq(a3853, a3854) {
        true
    } else {
        let a3855 = v0;
        let a3856 = 9u64;
        if nat_eq(a3855, a3856) {
            true
        } else {
            let a3857 = v0;
            let a3858 = 10u64;
            if nat_eq(a3857, a3858) {
                true
            } else {
                let a3859 = v0;
                let a3860 = 12u64;
                if nat_eq(a3859, a3860) {
                    true
                } else {
                    let a3861 = v0;
                    let a3862 = 13u64;
                    nat_eq(a3861, a3862)
                }
            }
        }
    }
}

pub fn f566(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3907 = {
            let a3883 = {
                let a3871 = {
                    let a3865 = {
                        let a3863 = v1;
                        let a3864 = 128u8;
                        fixed_u8::bit_and(a3863, a3864)
                    };
                    let a3866 = 128u8;
                    if equal(a3865, a3866) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3872 = {
                    let a3869 = {
                        let a3867 = v1;
                        let a3868 = 64u8;
                        fixed_u8::bit_and(a3867, a3868)
                    };
                    let a3870 = 64u8;
                    if equal(a3869, a3870) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3871, a3872)?
            };
            let a3884 = {
                let a3881 = {
                    let a3875 = {
                        let a3873 = v1;
                        let a3874 = 32u8;
                        fixed_u8::bit_and(a3873, a3874)
                    };
                    let a3876 = 32u8;
                    if equal(a3875, a3876) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3882 = {
                    let a3879 = {
                        let a3877 = v1;
                        let a3878 = 16u8;
                        fixed_u8::bit_and(a3877, a3878)
                    };
                    let a3880 = 16u8;
                    if equal(a3879, a3880) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3881, a3882)?
            };
            nat_add(a3883, a3884)?
        };
        let a3908 = {
            let a3905 = {
                let a3893 = {
                    let a3887 = {
                        let a3885 = v1;
                        let a3886 = 8u8;
                        fixed_u8::bit_and(a3885, a3886)
                    };
                    let a3888 = 8u8;
                    if equal(a3887, a3888) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3894 = {
                    let a3891 = {
                        let a3889 = v1;
                        let a3890 = 4u8;
                        fixed_u8::bit_and(a3889, a3890)
                    };
                    let a3892 = 4u8;
                    if equal(a3891, a3892) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3893, a3894)?
            };
            let a3906 = {
                let a3903 = {
                    let a3897 = {
                        let a3895 = v1;
                        let a3896 = 2u8;
                        fixed_u8::bit_and(a3895, a3896)
                    };
                    let a3898 = 2u8;
                    if equal(a3897, a3898) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3904 = {
                    let a3901 = {
                        let a3899 = v1;
                        let a3900 = 1u8;
                        fixed_u8::bit_and(a3899, a3900)
                    };
                    let a3902 = 1u8;
                    if equal(a3901, a3902) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3903, a3904)?
            };
            nat_add(a3905, a3906)?
        };
        nat_add(a3907, a3908)?
    };
    let a3909 = 48u64;
    let a3910 = v2;
    if nat_le(a3909, a3910) {
        let a3911 = v2;
        let a3912 = 57u64;
        Ok(nat_le(a3911, a3912))
    } else {
        Ok(false)
    }
}

pub fn f567(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m3913 = v1;
    match m3913 {
        Adt19::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C1(v7, v8) => {
            Ok(Some({
                let a3924 = if if v7 {
                    let a3922 = 0u64;
                    let a3923 = v8;
                    nat_lt(a3922, a3923)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3925 = f331(false, v8)?;
                append_bytes(a3924, a3925)
            }))
        }
        Adt19::C2(v3, v4, v5) => {
            let m3916 = f585({
                let a3914 = f8(Str::lit("n"));
                let a3915 = f332(false, v4, v5)?;
                append_bytes(a3914, a3915)
            })?;
            match m3916 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3920 = f8(Str::lit("nMinus"));
                        let a3921 = f32(v6.clone(), 1u64, {
                            let a3918 = {
                                let a3917 = v6;
                                length_bytes(a3917)
                            };
                            let a3919 = 1u64;
                            nat_sub(a3918, a3919)
                        });
                        append_bytes(a3920, a3921)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C3(v2) => {
            f573(v0, v2)
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

pub fn f568(v0: Adt19) -> R<Bytes> {
    f244(v0)
}

pub fn f569(v0: Bytes) -> R<Option<Bytes>> {
    if f586({
        let a3926 = v0.clone();
        length_bytes(a3926)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f587(v0)?;
        if f588(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3929 = {
                let a3927 = v1.clone();
                let a3928 = 0u64;
                index_bytes(a3927, a3928)
            };
            match m3929 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f386({
                        let v3: u8 = v2;
                        let a3974 = {
                            let a3950 = {
                                let a3938 = {
                                    let a3932 = {
                                        let a3930 = v3;
                                        let a3931 = 128u8;
                                        fixed_u8::bit_and(a3930, a3931)
                                    };
                                    let a3933 = 128u8;
                                    if equal(a3932, a3933) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3939 = {
                                    let a3936 = {
                                        let a3934 = v3;
                                        let a3935 = 64u8;
                                        fixed_u8::bit_and(a3934, a3935)
                                    };
                                    let a3937 = 64u8;
                                    if equal(a3936, a3937) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3938, a3939)?
                            };
                            let a3951 = {
                                let a3948 = {
                                    let a3942 = {
                                        let a3940 = v3;
                                        let a3941 = 32u8;
                                        fixed_u8::bit_and(a3940, a3941)
                                    };
                                    let a3943 = 32u8;
                                    if equal(a3942, a3943) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3949 = {
                                    let a3946 = {
                                        let a3944 = v3;
                                        let a3945 = 16u8;
                                        fixed_u8::bit_and(a3944, a3945)
                                    };
                                    let a3947 = 16u8;
                                    if equal(a3946, a3947) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3948, a3949)?
                            };
                            nat_add(a3950, a3951)?
                        };
                        let a3975 = {
                            let a3972 = {
                                let a3960 = {
                                    let a3954 = {
                                        let a3952 = v3;
                                        let a3953 = 8u8;
                                        fixed_u8::bit_and(a3952, a3953)
                                    };
                                    let a3955 = 8u8;
                                    if equal(a3954, a3955) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3961 = {
                                    let a3958 = {
                                        let a3956 = v3;
                                        let a3957 = 4u8;
                                        fixed_u8::bit_and(a3956, a3957)
                                    };
                                    let a3959 = 4u8;
                                    if equal(a3958, a3959) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3960, a3961)?
                            };
                            let a3973 = {
                                let a3970 = {
                                    let a3964 = {
                                        let a3962 = v3;
                                        let a3963 = 2u8;
                                        fixed_u8::bit_and(a3962, a3963)
                                    };
                                    let a3965 = 2u8;
                                    if equal(a3964, a3965) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3971 = {
                                    let a3968 = {
                                        let a3966 = v3;
                                        let a3967 = 1u8;
                                        fixed_u8::bit_and(a3966, a3967)
                                    };
                                    let a3969 = 1u8;
                                    if equal(a3968, a3969) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3970, a3971)?
                            };
                            nat_add(a3972, a3973)?
                        };
                        nat_add(a3974, a3975)?
                    }) {
                        Ok(Some({
                            let a3976 = f8(Str::lit("n"));
                            let a3977 = v1;
                            append_bytes(a3976, a3977)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f589({
            let a3978 = v0.clone();
            length_bytes(a3978)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f570(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3979 = v1;
    match m3979.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3980 = f573(v0, v2)?;
            match m3980 {
                None => {
                    f570(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f570(v0, v3)?))
                }
            }
        }
    }
}

pub fn f571(p3984: List<Adt19>, v1: Bytes) -> bool {
    let m3983 = p3984;
    {
        let mut m3983 = m3983;
        loop {
            let step = {
                let v0 = m3983;
                let m3981 = v0;
                match m3981.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3982 = f578(v2, v1.clone());
                        if match m3982 {
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
                Ok(next) => { m3983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: Adt19) -> R<Option<Bytes>> {
    let m3985 = f245(f147(v0.clone(), f8(Str::lit("$ref"))));
    match m3985 {
        None => {
            Ok(f245(f147(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f501(v1)
        }
    }
}

pub fn f573(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f585(v1)
    } else {
        f569(v1)
    }
}

pub fn f574(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f590(f473(v0), v1)
}

pub fn f575(v0: bool, v1: Bytes, v2: List<Adt19>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m3986 = v2;
    match m3986.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3987 = f578(v5.clone(), v3.clone());
            match m3987 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3988 = f573(v0, v7.clone())?;
                    match m3988 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f86(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3991 = f507({
                                    let a3989 = v1.clone();
                                    let a3990 = f241(v7)?;
                                    append_bytes(a3989, a3990)
                                }, f435(f388(f147(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f151(f152(f147(v5, f8(Str::lit("required"))))), true)?;
                                match m3991 {
                                    Ok(v10) => {
                                        let m3992 = f575(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3992 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f214({
                                                    let (h3993, _) = v10;
                                                    h3993
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h3994, _) = v12;
                                                    h3994
                                                }), f392({
                                                    let (_, h3995) = v10;
                                                    h3995
                                                }, {
                                                    let (_, h3996) = v12;
                                                    h3996
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

pub fn f576(v0: List<Adt19>, v1: Bytes) -> List<Bytes> {
    let m3997 = v0;
    match m3997.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3998 = f578(v2, v1.clone());
            match m3998 {
                None => {
                    f576(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f576(v3, v1))
                }
            }
        }
    }
}

pub fn f577(v0: Adt19) -> bool {
    f36(f150(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f578(v0: Adt19, v1: Bytes) -> Option<Bytes> {
    if f86(v1.clone(), f151(f152(f147(v0.clone(), f8(Str::lit("required")))))) {
        let m3999 = f147(v0, f8(Str::lit("properties")));
        match m3999 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m4000 = f147(v2, v1);
                match m4000 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f245(f147(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f579(v0: Fn22, p4005: u64, p4006: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4004 = (p4005, p4006);
    {
        let mut m4004 = m4004;
        loop {
            let step = {
                let (v1, v2) = m4004;
                let m4001 = v1;
                if m4001 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4001 - 1;
                    let m4003 = {
                        let c4002: Fn22 = v0.clone();
                        c4002.apply(v2.clone())?
                    };
                    match m4003 {
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
                Ok(next) => { m4004 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f591(v0)
}

pub fn f581(v0: Fn23, p4011: u64, p4012: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4010 = (p4011, p4012);
    {
        let mut m4010 = m4010;
        loop {
            let step = {
                let (v1, v2) = m4010;
                let m4007 = v1;
                if m4007 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4007 - 1;
                    let m4009 = {
                        let c4008: Fn23 = v0.clone();
                        c4008.apply(v2.clone())
                    };
                    match m4009 {
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
                Ok(next) => { m4010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f592(v0)
}

pub fn f583(v0: Fn24, p4017: u64, p4018: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4016 = (p4017, p4018);
    {
        let mut m4016 = m4016;
        loop {
            let step = {
                let (v1, v2) = m4016;
                let m4013 = v1;
                if m4013 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4013 - 1;
                    let m4015 = {
                        let c4014: Fn24 = v0.clone();
                        c4014.apply(v2.clone())?
                    };
                    match m4015 {
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
                Ok(next) => { m4016 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f593(v0, v1)
}

pub fn f585(v0: Bytes) -> R<Option<Bytes>> {
    let m4020 = f594({
        let a4019 = v0.clone();
        length_bytes(a4019)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m4020 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a4022 = {
                let a4021 = v1.clone();
                length_bytes(a4021)
            };
            let a4023 = 0u64;
            if nat_eq(a4022, a4023) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f586(p4079: u64, v1: Bytes, p4080: u64) -> R<bool> {
    let m4078 = (p4079, p4080);
    {
        let mut m4078 = m4078;
        loop {
            let step = {
                let (v0, v2) = m4078;
                let m4024 = v0;
                if m4024 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4024 - 1;
                    let m4027 = {
                        let a4025 = v1.clone();
                        let a4026 = v2;
                        index_bytes(a4025, a4026)
                    };
                    match m4027 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4074 = {
                                let v5: u8 = v4;
                                let a4072 = {
                                    let a4048 = {
                                        let a4036 = {
                                            let a4030 = {
                                                let a4028 = v5;
                                                let a4029 = 128u8;
                                                fixed_u8::bit_and(a4028, a4029)
                                            };
                                            let a4031 = 128u8;
                                            if equal(a4030, a4031) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4037 = {
                                            let a4034 = {
                                                let a4032 = v5;
                                                let a4033 = 64u8;
                                                fixed_u8::bit_and(a4032, a4033)
                                            };
                                            let a4035 = 64u8;
                                            if equal(a4034, a4035) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4036, a4037)?
                                    };
                                    let a4049 = {
                                        let a4046 = {
                                            let a4040 = {
                                                let a4038 = v5;
                                                let a4039 = 32u8;
                                                fixed_u8::bit_and(a4038, a4039)
                                            };
                                            let a4041 = 32u8;
                                            if equal(a4040, a4041) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4047 = {
                                            let a4044 = {
                                                let a4042 = v5;
                                                let a4043 = 16u8;
                                                fixed_u8::bit_and(a4042, a4043)
                                            };
                                            let a4045 = 16u8;
                                            if equal(a4044, a4045) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4046, a4047)?
                                    };
                                    nat_add(a4048, a4049)?
                                };
                                let a4073 = {
                                    let a4070 = {
                                        let a4058 = {
                                            let a4052 = {
                                                let a4050 = v5;
                                                let a4051 = 8u8;
                                                fixed_u8::bit_and(a4050, a4051)
                                            };
                                            let a4053 = 8u8;
                                            if equal(a4052, a4053) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4059 = {
                                            let a4056 = {
                                                let a4054 = v5;
                                                let a4055 = 4u8;
                                                fixed_u8::bit_and(a4054, a4055)
                                            };
                                            let a4057 = 4u8;
                                            if equal(a4056, a4057) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4058, a4059)?
                                    };
                                    let a4071 = {
                                        let a4068 = {
                                            let a4062 = {
                                                let a4060 = v5;
                                                let a4061 = 2u8;
                                                fixed_u8::bit_and(a4060, a4061)
                                            };
                                            let a4063 = 2u8;
                                            if equal(a4062, a4063) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4069 = {
                                            let a4066 = {
                                                let a4064 = v5;
                                                let a4065 = 1u8;
                                                fixed_u8::bit_and(a4064, a4065)
                                            };
                                            let a4067 = 1u8;
                                            if equal(a4066, a4067) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4068, a4069)?
                                    };
                                    nat_add(a4070, a4071)?
                                };
                                nat_add(a4072, a4073)?
                            };
                            let a4075 = 128u64;
                            if nat_lt(a4074, a4075) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4076 = v2;
                                    let a4077 = 1u64;
                                    nat_add(a4076, a4077)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f327(v0.clone(), 0u64, {
        let a4081 = v0;
        length_bytes(a4081)
    }, false, Bytes::lit(&[]))?;
    let m4084 = {
        let a4082 = v1.clone();
        let a4083 = 0u64;
        index_bytes(a4082, a4083)
    };
    match m4084 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a4129 = {
                    let a4105 = {
                        let a4093 = {
                            let a4087 = {
                                let a4085 = v3;
                                let a4086 = 128u8;
                                fixed_u8::bit_and(a4085, a4086)
                            };
                            let a4088 = 128u8;
                            if equal(a4087, a4088) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4094 = {
                            let a4091 = {
                                let a4089 = v3;
                                let a4090 = 64u8;
                                fixed_u8::bit_and(a4089, a4090)
                            };
                            let a4092 = 64u8;
                            if equal(a4091, a4092) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4093, a4094)?
                    };
                    let a4106 = {
                        let a4103 = {
                            let a4097 = {
                                let a4095 = v3;
                                let a4096 = 32u8;
                                fixed_u8::bit_and(a4095, a4096)
                            };
                            let a4098 = 32u8;
                            if equal(a4097, a4098) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4104 = {
                            let a4101 = {
                                let a4099 = v3;
                                let a4100 = 16u8;
                                fixed_u8::bit_and(a4099, a4100)
                            };
                            let a4102 = 16u8;
                            if equal(a4101, a4102) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4103, a4104)?
                    };
                    nat_add(a4105, a4106)?
                };
                let a4130 = {
                    let a4127 = {
                        let a4115 = {
                            let a4109 = {
                                let a4107 = v3;
                                let a4108 = 8u8;
                                fixed_u8::bit_and(a4107, a4108)
                            };
                            let a4110 = 8u8;
                            if equal(a4109, a4110) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4116 = {
                            let a4113 = {
                                let a4111 = v3;
                                let a4112 = 4u8;
                                fixed_u8::bit_and(a4111, a4112)
                            };
                            let a4114 = 4u8;
                            if equal(a4113, a4114) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4115, a4116)?
                    };
                    let a4128 = {
                        let a4125 = {
                            let a4119 = {
                                let a4117 = v3;
                                let a4118 = 2u8;
                                fixed_u8::bit_and(a4117, a4118)
                            };
                            let a4120 = 2u8;
                            if equal(a4119, a4120) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4126 = {
                            let a4123 = {
                                let a4121 = v3;
                                let a4122 = 1u8;
                                fixed_u8::bit_and(a4121, a4122)
                            };
                            let a4124 = 1u8;
                            if equal(a4123, a4124) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4125, a4126)?
                    };
                    nat_add(a4127, a4128)?
                };
                nat_add(a4129, a4130)?
            };
            if f385(v4) {
                let a4137 = f32({
                    let a4131 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4131)
                }, {
                    let a4132 = v4;
                    let a4133 = 65u64;
                    nat_sub(a4132, a4133)
                }, 1u64);
                let a4138 = f32(v1.clone(), 1u64, {
                    let a4135 = {
                        let a4134 = v1;
                        length_bytes(a4134)
                    };
                    let a4136 = 1u64;
                    nat_sub(a4135, a4136)
                });
                Ok(append_bytes(a4137, a4138))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f588(v0: Bytes) -> R<bool> {
    let m4141 = {
        let a4139 = v0;
        let a4140 = 0u64;
        index_bytes(a4139, a4140)
    };
    match m4141 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f384({
                let v2: u8 = v1;
                let a4186 = {
                    let a4162 = {
                        let a4150 = {
                            let a4144 = {
                                let a4142 = v2;
                                let a4143 = 128u8;
                                fixed_u8::bit_and(a4142, a4143)
                            };
                            let a4145 = 128u8;
                            if equal(a4144, a4145) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4151 = {
                            let a4148 = {
                                let a4146 = v2;
                                let a4147 = 64u8;
                                fixed_u8::bit_and(a4146, a4147)
                            };
                            let a4149 = 64u8;
                            if equal(a4148, a4149) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4150, a4151)?
                    };
                    let a4163 = {
                        let a4160 = {
                            let a4154 = {
                                let a4152 = v2;
                                let a4153 = 32u8;
                                fixed_u8::bit_and(a4152, a4153)
                            };
                            let a4155 = 32u8;
                            if equal(a4154, a4155) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4161 = {
                            let a4158 = {
                                let a4156 = v2;
                                let a4157 = 16u8;
                                fixed_u8::bit_and(a4156, a4157)
                            };
                            let a4159 = 16u8;
                            if equal(a4158, a4159) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4160, a4161)?
                    };
                    nat_add(a4162, a4163)?
                };
                let a4187 = {
                    let a4184 = {
                        let a4172 = {
                            let a4166 = {
                                let a4164 = v2;
                                let a4165 = 8u8;
                                fixed_u8::bit_and(a4164, a4165)
                            };
                            let a4167 = 8u8;
                            if equal(a4166, a4167) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4173 = {
                            let a4170 = {
                                let a4168 = v2;
                                let a4169 = 4u8;
                                fixed_u8::bit_and(a4168, a4169)
                            };
                            let a4171 = 4u8;
                            if equal(a4170, a4171) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4172, a4173)?
                    };
                    let a4185 = {
                        let a4182 = {
                            let a4176 = {
                                let a4174 = v2;
                                let a4175 = 2u8;
                                fixed_u8::bit_and(a4174, a4175)
                            };
                            let a4177 = 2u8;
                            if equal(a4176, a4177) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4183 = {
                            let a4180 = {
                                let a4178 = v2;
                                let a4179 = 1u8;
                                fixed_u8::bit_and(a4178, a4179)
                            };
                            let a4181 = 1u8;
                            if equal(a4180, a4181) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4182, a4183)?
                    };
                    nat_add(a4184, a4185)?
                };
                nat_add(a4186, a4187)?
            }) {
                Ok(true)
            } else {
                Ok(f385({
                    let v3: u8 = v1;
                    let a4232 = {
                        let a4208 = {
                            let a4196 = {
                                let a4190 = {
                                    let a4188 = v3;
                                    let a4189 = 128u8;
                                    fixed_u8::bit_and(a4188, a4189)
                                };
                                let a4191 = 128u8;
                                if equal(a4190, a4191) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a4197 = {
                                let a4194 = {
                                    let a4192 = v3;
                                    let a4193 = 64u8;
                                    fixed_u8::bit_and(a4192, a4193)
                                };
                                let a4195 = 64u8;
                                if equal(a4194, a4195) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4196, a4197)?
                        };
                        let a4209 = {
                            let a4206 = {
                                let a4200 = {
                                    let a4198 = v3;
                                    let a4199 = 32u8;
                                    fixed_u8::bit_and(a4198, a4199)
                                };
                                let a4201 = 32u8;
                                if equal(a4200, a4201) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a4207 = {
                                let a4204 = {
                                    let a4202 = v3;
                                    let a4203 = 16u8;
                                    fixed_u8::bit_and(a4202, a4203)
                                };
                                let a4205 = 16u8;
                                if equal(a4204, a4205) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4206, a4207)?
                        };
                        nat_add(a4208, a4209)?
                    };
                    let a4233 = {
                        let a4230 = {
                            let a4218 = {
                                let a4212 = {
                                    let a4210 = v3;
                                    let a4211 = 8u8;
                                    fixed_u8::bit_and(a4210, a4211)
                                };
                                let a4213 = 8u8;
                                if equal(a4212, a4213) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a4219 = {
                                let a4216 = {
                                    let a4214 = v3;
                                    let a4215 = 4u8;
                                    fixed_u8::bit_and(a4214, a4215)
                                };
                                let a4217 = 4u8;
                                if equal(a4216, a4217) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4218, a4219)?
                        };
                        let a4231 = {
                            let a4228 = {
                                let a4222 = {
                                    let a4220 = v3;
                                    let a4221 = 2u8;
                                    fixed_u8::bit_and(a4220, a4221)
                                };
                                let a4223 = 2u8;
                                if equal(a4222, a4223) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a4229 = {
                                let a4226 = {
                                    let a4224 = v3;
                                    let a4225 = 1u8;
                                    fixed_u8::bit_and(a4224, a4225)
                                };
                                let a4227 = 1u8;
                                if equal(a4226, a4227) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4228, a4229)?
                        };
                        nat_add(a4230, a4231)?
                    };
                    nat_add(a4232, a4233)?
                }))
            }
        }
    }
}

pub fn f589(p4246: u64, v1: Bytes, p4247: u64, p4248: Bytes) -> R<Option<Bytes>> {
    let m4245 = (p4246, (p4247, p4248));
    {
        let mut m4245 = m4245;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4245;
                let m4234 = v0;
                if m4234 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4234 - 1;
                    let a4236 = {
                        let a4235 = v1.clone();
                        length_bytes(a4235)
                    };
                    let a4237 = v2;
                    if nat_le(a4236, a4237) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4238 = f417(v1.clone(), v2)?;
                        match m4238 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4240 = v2;
                                    let a4241 = {
                                        let (_, h4239) = v5;
                                        h4239
                                    };
                                    nat_add(a4240, a4241)?
                                }, {
                                    let a4243 = v3;
                                    let a4244 = f595({
                                        let (h4242, _) = v5;
                                        h4242
                                    });
                                    append_bytes(a4243, a4244)
                                })))
                            }
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

pub fn f590(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4249 = v0.clone();
    match m4249 {
        Adt19::C0(v2) => {
            if v2 {
                Ok(f557())
            } else {
                Ok(f503(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt19::C1(_, _) => {
            Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C2(_, _, _) => {
            Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C3(_) => {
            Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C4(_) => {
            Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C5(_) => {
            let m4250 = f596(f433(v0.clone()));
            match m4250 {
                None => {
                    let m4251 = f245(f147(v0.clone(), f8(Str::lit("$ref"))));
                    match m4251 {
                        None => {
                            let m4253 = f147(v0.clone(), f8(Str::lit("allOf")));
                            match m4253 {
                                None => {
                                    let m4258 = f147(v0.clone(), f8(Str::lit("enum")));
                                    match m4258 {
                                        None => {
                                            let m4259 = f147(v0.clone(), f8(Str::lit("const")));
                                            match m4259 {
                                                None => {
                                                    let m4261 = f147(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4261 {
                                                        None => {
                                                            let v30: Bytes = f150(v0.clone(), f8(Str::lit("type")));
                                                            if if f36(v30.clone(), Bytes::lit(&[])) {
                                                                !f597(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f557())
                                                            } else {
                                                                if f36(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f504(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f36(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f504(f598(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f36(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f504(f599(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f36(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f504(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f36(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m4262 = f388(f147(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4262.uncons() {
                                                                                        None => {
                                                                                            let m4263 = f600(v0);
                                                                                            match m4263 {
                                                                                                None => {
                                                                                                    Ok(f555(f8(Str::lit("List")), f601(f557())))
                                                                                                }
                                                                                                Some(v33) => {
                                                                                                    Ok(f555(f8(Str::lit("List")), f601(f602(v33, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f504(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f503(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f504(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m4260 = f245(Some(v27));
                                                    match m4260 {
                                                        None => {
                                                            Ok(f503(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f504(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f504(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m4254 = f246(v19);
                                    match m4254.uncons() {
                                        None => {
                                            Ok(f503(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m4255 = v21;
                                            match m4255.uncons() {
                                                None => {
                                                    let m4256 = f245(f147(v20, f8(Str::lit("$ref"))));
                                                    match m4256 {
                                                        None => {
                                                            Ok(f503(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m4257 = f501(v24)?;
                                                            match m4257 {
                                                                None => {
                                                                    Ok(f503(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f504(v25, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f503(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m4252 = f501(v17)?;
                            match m4252 {
                                None => {
                                    Ok(f503(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f504(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f503(if f36(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt19::C6 => {
            Ok(f557())
        }
        Adt19::C7(_, _) => {
            Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C8(_, _, _) => {
            Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f591(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4264, _) = v0;
        h4264
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4265) = v0;
        h4265
    };
    let m4266 = v1;
    match m4266.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4267 = v4;
            match m4267.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f603(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f592(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4268, _) = v0.clone();
        h4268
    };
    let v2: List<(Bytes, ())> = {
        let (h4269, _) = {
            let (_, h4270) = v0.clone();
            h4270
        };
        h4269
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4271) = {
            let (_, h4272) = v0;
            h4272
        };
        h4271
    };
    let m4273 = v1.clone();
    match m4273.uncons() {
        None => {
            let m4274 = v2;
            match m4274.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4275 = v2.clone();
            match m4275.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f604({
                        let (h4276, _) = v8.clone();
                        h4276
                    }, {
                        let (h4277, _) = v6.clone();
                        h4277
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

pub fn f593(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4278, _) = v1;
        h4278
    };
    let m4280 = {
        let (_, h4279) = v1;
        h4279
    };
    match m4280 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4283 = {
                let a4281 = v0.clone();
                let a4282 = v2;
                index_bytes(a4281, a4282)
            };
            match m4283 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4288 = {
                        let a4286 = v0;
                        let a4287 = {
                            let a4284 = v2;
                            let a4285 = 1u64;
                            nat_add(a4284, a4285)?
                        };
                        index_bytes(a4286, a4287)
                    };
                    match m4288 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f605(v4)? {
                                f605(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4289 = v2;
                                    let a4290 = 2u64;
                                    nat_add(a4289, a4290)?
                                }, Some({
                                    let a4295 = v3;
                                    let a4296 = f316(f606(), {
                                        let a4293 = {
                                            let a4291 = f607(v4)?;
                                            let a4292 = 16u64;
                                            nat_mul(a4291, a4292)?
                                        };
                                        let a4294 = f607(v5)?;
                                        nat_add(a4293, a4294)?
                                    });
                                    append_bytes(a4295, a4296)
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

pub fn f594(p4309: u64, v1: Bytes, p4310: u64, p4311: Bytes) -> R<Option<Bytes>> {
    let m4308 = (p4309, (p4310, p4311));
    {
        let mut m4308 = m4308;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4308;
                let m4297 = v0;
                if m4297 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4297 - 1;
                    let a4299 = {
                        let a4298 = v1.clone();
                        length_bytes(a4298)
                    };
                    let a4300 = v2;
                    if nat_le(a4299, a4300) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4301 = f417(v1.clone(), v2)?;
                        match m4301 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4303 = v2;
                                    let a4304 = {
                                        let (_, h4302) = v5;
                                        h4302
                                    };
                                    nat_add(a4303, a4304)?
                                }, {
                                    let a4306 = v3;
                                    let a4307 = f608(v1.clone(), v2, {
                                        let (h4305, _) = v5;
                                        h4305
                                    });
                                    append_bytes(a4306, a4307)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: u64) -> Bytes {
    let a4312 = f8(Str::lit("u"));
    let a4313 = f609(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4312, a4313)
}

pub fn f596(p4318: List<(Bytes, Adt19)>) -> Option<Bytes> {
    let m4317 = p4318;
    {
        let mut m4317 = m4317;
        loop {
            let step = {
                let v0 = m4317;
                let m4314 = v0;
                match m4314.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f86({
                            let (h4315, _) = v1.clone();
                            h4315
                        }, f610()) {
                            Err::<List<(Bytes, Adt19)>, Option<Bytes>>(Some({
                                let (h4316, _) = v1;
                                h4316
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(v0: Adt19) -> bool {
    if f512(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f512(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f512(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f512(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f512(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f512(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f598(v0: Adt19) -> Bytes {
    let m4319 = f611(f147(v0, f8(Str::lit("minimum"))));
    match m4319 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f599() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f600(v0: Adt19) -> Option<Adt19> {
    let m4320 = f147(v0.clone(), f8(Str::lit("additionalProperties")));
    match m4320 {
        None => {
            let m4321 = f388(f147(v0, f8(Str::lit("patternProperties"))));
            match m4321.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v2, v3)) => {
                    let m4322 = v3;
                    match m4322.uncons() {
                        None => {
                            Some({
                                let (_, h4323) = v2;
                                h4323
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

pub fn f601(v0: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4324 = v0;
    match m4324 {
        Ok(v2) => {
            f504(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h4325, _) = v2;
                h4325
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h4326) = v2;
                h4326
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v1)
        }
    }
}

pub fn f602(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f612(16u64, v0, v1)
}

pub fn f603(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4331 = {
            let a4329 = {
                let a4327 = v0.clone();
                length_list(a4327)
            };
            let a4330 = {
                let a4328 = v1.clone();
                length_list(a4328)
            };
            nat_add(a4329, a4330)?
        };
        let a4332 = 1u64;
        nat_add(a4331, a4332)?
    };
    Ok(f613({
        let (_, h4333) = {
            let (_, h4334) = {
                let (h4335, _) = f614(Fn25::F615, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4335
            };
            h4334
        };
        h4333
    }))
}

pub fn f604(v0: Bytes, v1: Bytes) -> bool {
    let a4340 = {
        let a4336 = v0;
        let a4337 = v1;
        compare_bytes(a4336, a4337)
    };
    let a4341 = {
        let a4338 = Bytes::lit(&[]);
        let a4339 = Bytes::lit(&[0x01]);
        compare_bytes(a4338, a4339)
    };
    equal(a4340, a4341)
}

pub fn f605(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4386 = {
            let a4362 = {
                let a4350 = {
                    let a4344 = {
                        let a4342 = v1;
                        let a4343 = 128u8;
                        fixed_u8::bit_and(a4342, a4343)
                    };
                    let a4345 = 128u8;
                    if equal(a4344, a4345) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4351 = {
                    let a4348 = {
                        let a4346 = v1;
                        let a4347 = 64u8;
                        fixed_u8::bit_and(a4346, a4347)
                    };
                    let a4349 = 64u8;
                    if equal(a4348, a4349) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4350, a4351)?
            };
            let a4363 = {
                let a4360 = {
                    let a4354 = {
                        let a4352 = v1;
                        let a4353 = 32u8;
                        fixed_u8::bit_and(a4352, a4353)
                    };
                    let a4355 = 32u8;
                    if equal(a4354, a4355) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4361 = {
                    let a4358 = {
                        let a4356 = v1;
                        let a4357 = 16u8;
                        fixed_u8::bit_and(a4356, a4357)
                    };
                    let a4359 = 16u8;
                    if equal(a4358, a4359) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4360, a4361)?
            };
            nat_add(a4362, a4363)?
        };
        let a4387 = {
            let a4384 = {
                let a4372 = {
                    let a4366 = {
                        let a4364 = v1;
                        let a4365 = 8u8;
                        fixed_u8::bit_and(a4364, a4365)
                    };
                    let a4367 = 8u8;
                    if equal(a4366, a4367) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4373 = {
                    let a4370 = {
                        let a4368 = v1;
                        let a4369 = 4u8;
                        fixed_u8::bit_and(a4368, a4369)
                    };
                    let a4371 = 4u8;
                    if equal(a4370, a4371) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4372, a4373)?
            };
            let a4385 = {
                let a4382 = {
                    let a4376 = {
                        let a4374 = v1;
                        let a4375 = 2u8;
                        fixed_u8::bit_and(a4374, a4375)
                    };
                    let a4377 = 2u8;
                    if equal(a4376, a4377) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4383 = {
                    let a4380 = {
                        let a4378 = v1;
                        let a4379 = 1u8;
                        fixed_u8::bit_and(a4378, a4379)
                    };
                    let a4381 = 1u8;
                    if equal(a4380, a4381) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4382, a4383)?
            };
            nat_add(a4384, a4385)?
        };
        nat_add(a4386, a4387)?
    };
    let a4388 = 48u64;
    let a4389 = v2;
    if if nat_le(a4388, a4389) {
        let a4390 = v2;
        let a4391 = 57u64;
        nat_le(a4390, a4391)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4392 = 97u64;
        let a4393 = v2;
        if nat_le(a4392, a4393) {
            let a4394 = v2;
            let a4395 = 102u64;
            Ok(nat_le(a4394, a4395))
        } else {
            Ok(false)
        }
    }
}

pub fn f606() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f607(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4440 = {
            let a4416 = {
                let a4404 = {
                    let a4398 = {
                        let a4396 = v1;
                        let a4397 = 128u8;
                        fixed_u8::bit_and(a4396, a4397)
                    };
                    let a4399 = 128u8;
                    if equal(a4398, a4399) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4405 = {
                    let a4402 = {
                        let a4400 = v1;
                        let a4401 = 64u8;
                        fixed_u8::bit_and(a4400, a4401)
                    };
                    let a4403 = 64u8;
                    if equal(a4402, a4403) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4404, a4405)?
            };
            let a4417 = {
                let a4414 = {
                    let a4408 = {
                        let a4406 = v1;
                        let a4407 = 32u8;
                        fixed_u8::bit_and(a4406, a4407)
                    };
                    let a4409 = 32u8;
                    if equal(a4408, a4409) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4415 = {
                    let a4412 = {
                        let a4410 = v1;
                        let a4411 = 16u8;
                        fixed_u8::bit_and(a4410, a4411)
                    };
                    let a4413 = 16u8;
                    if equal(a4412, a4413) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4414, a4415)?
            };
            nat_add(a4416, a4417)?
        };
        let a4441 = {
            let a4438 = {
                let a4426 = {
                    let a4420 = {
                        let a4418 = v1;
                        let a4419 = 8u8;
                        fixed_u8::bit_and(a4418, a4419)
                    };
                    let a4421 = 8u8;
                    if equal(a4420, a4421) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4427 = {
                    let a4424 = {
                        let a4422 = v1;
                        let a4423 = 4u8;
                        fixed_u8::bit_and(a4422, a4423)
                    };
                    let a4425 = 4u8;
                    if equal(a4424, a4425) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4426, a4427)?
            };
            let a4439 = {
                let a4436 = {
                    let a4430 = {
                        let a4428 = v1;
                        let a4429 = 2u8;
                        fixed_u8::bit_and(a4428, a4429)
                    };
                    let a4431 = 2u8;
                    if equal(a4430, a4431) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4437 = {
                    let a4434 = {
                        let a4432 = v1;
                        let a4433 = 1u8;
                        fixed_u8::bit_and(a4432, a4433)
                    };
                    let a4435 = 1u8;
                    if equal(a4434, a4435) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4436, a4437)?
            };
            nat_add(a4438, a4439)?
        };
        nat_add(a4440, a4441)?
    };
    let a4442 = v2;
    let a4443 = 57u64;
    if nat_le(a4442, a4443) {
        let a4444 = v2;
        let a4445 = 48u64;
        Ok(nat_sub(a4444, a4445))
    } else {
        let a4446 = v2;
        let a4447 = 87u64;
        Ok(nat_sub(a4446, a4447))
    }
}

pub fn f608(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f384(v2) {
        true
    } else {
        f385(v2)
    } {
        let a4448 = v2;
        let a4449 = 117u64;
        if !nat_eq(a4448, a4449) {
            let a4450 = v2;
            let a4451 = 85u64;
            !nat_eq(a4450, a4451)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f386(v2) {
            let a4452 = 0u64;
            let a4453 = v1;
            nat_lt(a4452, a4453)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a4454 = v2;
        let a4455 = 65536u64;
        if nat_lt(a4454, a4455) {
            let a4456 = f8(Str::lit("u"));
            let a4457 = f609(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4456, a4457)
        } else {
            let a4458 = f8(Str::lit("U"));
            let a4459 = f609(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4458, a4459)
        }
    }
}

pub fn f609(p4476: u64, p4477: u64, p4478: u64, p4479: Bytes) -> Bytes {
    let m4475 = (p4476, (p4477, (p4478, p4479)));
    {
        let mut m4475 = m4475;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m4475;
                let m4460 = v0;
                if m4460 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m4460 - 1;
                    let a4461 = v1;
                    let a4462 = 0u64;
                    if if nat_eq(a4461, a4462) {
                        let a4463 = v2;
                        let a4464 = 0u64;
                        nat_eq(a4463, a4464)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a4465 = v1;
                            let a4466 = 16u64;
                            let a4467 = 0u64;
                            nat_quot(a4465, a4466, a4467)
                        }, ({
                            let a4468 = v2;
                            let a4469 = 1u64;
                            nat_sub(a4468, a4469)
                        }, {
                            let a4473 = f616({
                                let a4470 = v1;
                                let a4471 = 16u64;
                                let a4472 = 0u64;
                                nat_rem(a4470, a4471, a4472)
                            });
                            let a4474 = v3;
                            append_bytes(a4473, a4474)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m4475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f610() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f611(v0: Option<Adt19>) -> Option<u64> {
    let m4480 = v0;
    match m4480 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m4481 = v1;
            match m4481 {
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

pub fn f612(v0: u64, v1: Adt19, v2: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4482 = v0;
    if m4482 == 0 {
        Ok(f503(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4482 - 1;
        let m4483 = v1.clone();
        match m4483 {
            Adt19::C0(v4) => {
                if v4 {
                    Ok(f557())
                } else {
                    Ok(f503(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt19::C1(_, _) => {
                Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C2(_, _, _) => {
                Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C3(_) => {
                Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C4(_) => {
                Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C5(_) => {
                let m4484 = f245(f147(v1.clone(), f8(Str::lit("$ref"))));
                match m4484 {
                    None => {
                        if if f512(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f512(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f512(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f504(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f150(v1.clone(), f8(Str::lit("type")));
                            if if f36(v20.clone(), Bytes::lit(&[])) {
                                !f597(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f557())
                            } else {
                                if f36(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f504(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f36(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f504(f598(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f36(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f504(f599(), List::<Adt25>::nil()))
                                        } else {
                                            if f36(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f504(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f36(v20.clone(), f8(Str::lit("array"))) {
                                                    let m4486 = f147(v1, f8(Str::lit("items")));
                                                    match m4486 {
                                                        None => {
                                                            Ok(f555(f8(Str::lit("List")), f557()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f555(f8(Str::lit("List")), f612(v3, v21, {
                                                                let a4487 = v2;
                                                                let a4488 = f8(Str::lit("Item"));
                                                                append_bytes(a4487, a4488)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f36(v20.clone(), f8(Str::lit("object"))) {
                                                        let m4489 = f600(v1);
                                                        match m4489 {
                                                            None => {
                                                                Ok(f555(f8(Str::lit("List")), f601(f557())))
                                                            }
                                                            Some(v22) => {
                                                                Ok(f555(f8(Str::lit("List")), f601(f612(v3, v22, {
                                                                    let a4490 = v2;
                                                                    let a4491 = f8(Str::lit("Value"));
                                                                    append_bytes(a4490, a4491)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f503(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m4485 = f501(v18)?;
                        match m4485 {
                            None => {
                                Ok(f503(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f504(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt19::C6 => {
                Ok(f557())
            }
            Adt19::C7(_, _) => {
                Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C8(_, _, _) => {
                Ok(f503(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f613(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f398(Fn10::F617, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f614(v0: Fn25, p4496: u64, p4497: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4495 = (p4496, p4497);
    {
        let mut m4495 = m4495;
        loop {
            let step = {
                let (v1, v2) = m4495;
                let m4492 = v1;
                if m4492 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4492 - 1;
                    let m4494 = {
                        let c4493: Fn25 = v0.clone();
                        c4493.apply(v2.clone())
                    };
                    match m4494 {
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
                Ok(next) => { m4495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f615(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f618(v0)
}

pub fn f616(v0: u64) -> Bytes {
    f32({
        let a4498 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4498)
    }, v0, 1u64)
}

pub fn f617(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f618(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4499, _) = v0.clone();
        h4499
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4500, _) = {
            let (_, h4501) = v0.clone();
            h4501
        };
        h4500
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4502) = {
            let (_, h4503) = v0;
            h4503
        };
        h4502
    };
    let m4504 = v1.clone();
    match m4504.uncons() {
        None => {
            let m4505 = v2;
            match m4505.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4506 = v2.clone();
            match m4506.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f604({
                        let (h4507, _) = v8.clone();
                        h4507
                    }, {
                        let (h4508, _) = v6.clone();
                        h4508
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
