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
    C0,
    C1(Bytes, std::rc::Rc<Adt17>),
    C2(bool),
    C3(bool, u64),
    C4(bool, u64, u64),
    C5(Bytes),
    C6(std::rc::Rc<Adt17>),
    C7(std::rc::Rc<Adt17>),
    C8,
    C9(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C10(Bytes, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(bool, bool, Bytes, List<Bytes>, List<List<Bytes>>),
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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Adt17),
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
pub enum Adt28 {
    C0(Bytes, List<Adt25>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(List<Bytes>, List<Adt25>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(bool, List<(u64, u64)>, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F64(u8, Bytes),
    F157(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn0::F157(k0) => f157(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F94,
    F199,
    F300,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F94 => f94(p0, p1),
            Fn1::F199 => f199(p0, p1),
            Fn1::F300 => f300(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F171(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F171(k0) => f171(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F357(Bytes),
    F497(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F357(k0) => f357(k0.clone(), p0),
            Fn3::F497(k0, k1) => Ok(f497(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F359,
    F707,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn4::F359 => f359(p0, p1),
            Fn4::F707 => f707(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F362,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn5::F362 => f362(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F368(Bytes),
    F369(u64, Bytes),
    F377(Bytes, u64, Bytes, u64),
    F490(Bytes),
    F491(Bytes),
    F541(Bytes),
    F579(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn6::F368(k0) => Ok(f368(k0.clone(), p0)),
            Fn6::F369(k0, k1) => f369(*k0, k1.clone(), p0),
            Fn6::F377(k0, k1, k2, k3) => f377(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn6::F490(k0) => f490(k0.clone(), p0),
            Fn6::F491(k0) => f491(k0.clone(), p0),
            Fn6::F541(k0) => f541(k0.clone(), p0),
            Fn6::F579(k0) => f579(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F388,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F388 => f388(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F391,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F391 => f391(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F393,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F393 => f393(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F396,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F396 => f396(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F421(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn11::F421(k0, k1, k2) => f421(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F454,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F454 => f454(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F457,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F457 => f457(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F489(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn14::F489(k0) => f489(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F494(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn15::F494(k0) => f494(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F501,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F501 => f501(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F503,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F503 => f503(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F521,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F521 => f521(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F545,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F545 => f545(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F548,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F548 => f548(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F612,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F612 => f612(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F626,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F626 => f626(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F659,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F659 => f659(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F680(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F680(k0) => f680(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F690,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F690 => f690(p0),
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
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h308, _) => {
            h308
        }
    }, f112())) {
        Ok(Adt11::C1(List::cons(f112(), List::<Bytes>::nil())))
    } else {
        let m309 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f113(v0.clone())?)));
        match m309.uncons() {
            None => {
                let m310 = f114(v0.clone(), f113(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m310.uncons() {
                    None => {
                        f116(v0)
                    }
                    Some((v3, v4)) => {
                        Ok(Adt11::C4(f7(List::cons(v3, v4))))
                    }
                }
            }
            Some((v1, v2)) => {
                Ok(Adt11::C0(List::cons(v1, v2)))
            }
        }
    }
}

pub fn f70(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h311, _) => {
            h311
        }
    }, f117())) {
        Ok(Adt11::C1(List::cons(f117(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f118(f119(f117(), f120(match v0.clone() {
            Adt1::C0(_, h312, _) => {
                h312
            }
        }, f117())))?;
        let m313 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m313.uncons() {
            None => {
                let m314 = f121(v0.clone(), f122(f81(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f123(v2, v1.clone()))?;
                match m314 {
                    None => {
                        f124(v0, v1)
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
    let m315 = f97(v0.clone());
    match m315.uncons() {
        None => {
            let m316 = f98(f45(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f125(), List::<Bytes>::nil())))), f99(v0.clone())?);
            match m316.uncons() {
                None => {
                    Ok(Adt11::C3(f126(f127(v0.clone())?, List::cons((f128(), f129()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f130()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f131()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f132()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f133()), List::cons((f8(Str::lit("docs/catalog.md")), f134(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f135(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f136(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f137()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m321 = {
        let a318 = v0;
        let a319 = 0u64;
        let a320 = {
            let a317 = v1.clone();
            length_bytes(a317)
        };
        slice_bytes(a318, a319, a320)
    };
    match m321 {
        None => {
            false
        }
        Some(v2) => {
            let a322 = v2;
            let a323 = v1;
            equal(a322, a323)
        }
    }
}

pub fn f73(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f138(v3.clone(), v1.clone(), f139(v3, v1, false)?)
    } else {
        if f28(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f138(v3.clone(), v1.clone(), f140(v3, v1)?)
        } else {
            if f28(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f138(v3.clone(), v1.clone(), f139(v3, v1, true)?)
            } else {
                if f28(v0.clone(), f8(Str::lit("stage-registries"))) {
                    f138(v3.clone(), v1.clone(), f141(v3, v1)?)
                } else {
                    if f28(v0.clone(), f8(Str::lit("stage-encodings"))) {
                        f138(v3.clone(), v1.clone(), f142(v3, v1)?)
                    } else {
                        if f28(v0.clone(), f8(Str::lit("stage-mappings"))) {
                            f138(v3.clone(), v1.clone(), f143(v3, v1, false)?)
                        } else {
                            if f28(v0, f8(Str::lit("stage-matrix"))) {
                                f138(v3.clone(), v1.clone(), f143(v3, v1, true)?)
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

pub fn f74(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("admit"))) {
        f144(v3, v1)
    } else {
        if f28(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f145(v3, v1)
        } else {
            if f28(v0.clone(), f8(Str::lit("admit-registry"))) {
                f146(v3, v1, v2)
            } else {
                if f28(v0.clone(), f8(Str::lit("ingest-authority"))) {
                    f147(v3, v1)
                } else {
                    if f28(v0.clone(), f8(Str::lit("ingest-index"))) {
                        f148(v3, v1)
                    } else {
                        if f28(v0.clone(), f8(Str::lit("merge-gate"))) {
                            f149(v3, v1)
                        } else {
                            if f28(v0.clone(), f8(Str::lit("propose-encoding"))) {
                                let m324 = v2;
                                match m324.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f66()))
                                    }
                                    Some((v4, v5)) => {
                                        f150(v3, v1, v4, v5)
                                    }
                                }
                            } else {
                                if f28(v0.clone(), f8(Str::lit("record-decision"))) {
                                    f151(v3, v1)
                                } else {
                                    if f28(v0, f8(Str::lit("record-error"))) {
                                        f152(v3, v1, v2)
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
}

pub fn f75(v0: Bytes) -> R<bool> {
    f153(v0.clone(), 0u64, {
        let a325 = v0;
        length_bytes(a325)
    })
}

pub fn f76(p330: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m329 = p330;
    {
        let mut m329 = m329;
        loop {
            let step = {
                let v0 = m329;
                let m326 = v0;
                match m326.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h327, _) = v2;
                            h327
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h328) = v2;
                                h328
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Bytes) -> R<Option<Adt13>> {
    let m331 = f154(v0)?;
    match m331 {
        None => {
            Ok(None::<Adt13>)
        }
        Some(v1) => {
            Ok(f155(v1))
        }
    }
}

pub fn f78(v0: Option<Adt13>) -> Option<Bytes> {
    let m332 = v0;
    match m332 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m333 = v1;
            match m333 {
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

pub fn f79(p339: Adt13, v1: Bytes) -> Option<Adt13> {
    let m338 = p339;
    {
        let mut m338 = m338;
        loop {
            let step = {
                let v0 = m338;
                let m334 = v0;
                match m334 {
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
                    Adt13::C4(r335) => {
                        let v5 = (*r335).clone();
                        Ok::<Adt13, Option<Adt13>>(v5)
                    }
                    Adt13::C5 => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C6(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C7(v2, r336, r337) => {
                        let v3 = (*r336).clone();
                        let v4 = (*r337).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt13, Option<Adt13>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Adt13>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m340 = v0;
    match m340.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m341 = f156(v2, v1.clone())?;
            match m341 {
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
        let a342 = v0.clone();
        length_bytes(a342)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h343, _) = f63(Fn0::F157(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h343
    };
    let v3: u64 = {
        let (h344, _) = {
            let (_, h345) = v2.clone();
            h345
        };
        h344
    };
    let v4: List<Bytes> = {
        let (_, h346) = {
            let (_, h347) = v2;
            h347
        };
        h346
    };
    let a348 = v3;
    let a349 = v1;
    if nat_lt(a348, a349) {
        Ok(f65(List::cons(f35(v0, v3, {
            let a350 = v1;
            let a351 = v3;
            nat_sub(a350, a351)
        }), v4)))
    } else {
        Ok(f65(v4))
    }
}

pub fn f82(v0: Adt1, v1: Bytes) -> R<List<Adt13>> {
    Ok(f158(f46(v0, v1)?))
}

pub fn f83(v0: List<Adt13>, v1: Bytes) -> List<Adt13> {
    let m352 = v0;
    match m352.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f159(v2, v1.clone()), f83(v3, v1))
        }
    }
}

pub fn f84(p357: List<Bytes>, p358: List<Bytes>) -> bool {
    let m356 = (p357, p358);
    {
        let mut m356 = m356;
        loop {
            let step = {
                let (v0, v1) = m356;
                let m353 = v0;
                match m353.uncons() {
                    None => {
                        let m354 = v1;
                        match m354.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m355 = v1;
                        match m355.uncons() {
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
                Ok(next) => { m356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<Adt13>) -> List<Bytes> {
    let m359 = v0;
    match m359.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m360 = f79(v1, f8(Str::lit("std")));
            match m360 {
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
    let a361 = f160(v0)?;
    let a362 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a361, a362))
}

pub fn f87(v0: List<Adt13>) -> Adt13 {
    let m363 = v0;
    match m363.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f87(v2)))
        }
    }
}

pub fn f88(v0: Adt12) -> Bytes {
    let m364 = v0;
    match m364 {
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
    let a366 = {
        let a365 = v2.clone();
        length_bytes(a365)
    };
    let a367 = 0u64;
    if nat_eq(a366, a367) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m368 = f161(v0.clone(), f49())?;
        match m368 {
            None => {
                if f40(f41(f164(), match v0.clone() {
                    Adt1::C0(_, _, h369) => {
                        h369
                    }
                })) {
                    Ok(Adt11::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f164()))
                } else {
                    let m370 = f45(v0.clone(), List::cons(f165(), f166(f49())?));
                    match m370.uncons() {
                        None => {
                            Ok(Adt11::C3(f167(v0.clone(), f49(), v1.clone(), v2.clone(), f168(f53(v0, f165()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt11::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f43(f162(v3.clone())?, f163(v3, v2)))
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h371, _) = v2.clone();
        h371
    };
    let v4: u64 = {
        let (h372, _) = {
            let (_, h373) = v2.clone();
            h373
        };
        h372
    };
    let v5: List<Bytes> = {
        let (_, h374) = {
            let (_, h375) = v2;
            h375
        };
        h374
    };
    let m376 = f169(v0.clone(), v3);
    match m376 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a377 = v6;
            let a378 = v1;
            if equal(a377, a378) {
                Ok(Some(({
                    let a379 = v3;
                    let a380 = 1u64;
                    nat_add(a379, a380)?
                }, ({
                    let a381 = v3;
                    let a382 = 1u64;
                    nat_add(a381, a382)?
                }, List::cons(f35(v0, v4, {
                    let a383 = v3;
                    let a384 = v4;
                    nat_sub(a383, a384)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a385 = v3;
                    let a386 = 1u64;
                    nat_add(a385, a386)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f93(v0: Fn1, p390: List<Bytes>, p391: List<Bytes>) -> List<Bytes> {
    let m389 = (p390, p391);
    {
        let mut m389 = m389;
        loop {
            let step = {
                let (v1, v2) = m389;
                let m387 = v2;
                match m387.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c388: Fn1 = v0.clone();
                            c388.apply(v1, v3)
                        }, v4))
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

pub fn f94(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m392 = f170(Fn2::F171(v1), None::<Bytes>, v0);
    match m392 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f96() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))))
}

pub fn f97(v0: Adt1) -> List<Bytes> {
    if f111(match v0.clone() {
        Adt1::C0(_, h393, _) => {
            h393
        }
    }, f172()) {
        if f111(match v0 {
            Adt1::C0(_, h394, _) => {
                h394
            }
        }, f173()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f173(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f172(), List::<Bytes>::nil())
    }
}

pub fn f98(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m395 = v0;
    match m395.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f98(v3, v1))
        }
    }
}

pub fn f99(v0: Adt1) -> R<List<Bytes>> {
    Ok(f45(v0.clone(), List::cons(f174(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f109(v0)?)))))
}

pub fn f100(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f175(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f115(f82(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f101(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m396 = v0;
    match m396.uncons() {
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
    let m397 = v0;
    match m397.uncons() {
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
    let m398 = v0;
    match m398.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f98(f176(f158(f79(v1, f8(Str::lit("defers"))))), f103(v2))
        }
    }
}

pub fn f104(v0: List<Adt13>) -> List<Bytes> {
    let m399 = v0;
    match m399.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m400 = f79(v1, f8(Str::lit("std")));
            match m400 {
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
    let m401 = v0;
    match m401.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f126(f177(v2.clone()), f178(f179(v0, v1.clone(), v2)?));
    f180(f118(f181(v1))?, v3.clone(), {
        let a403 = {
            let a402 = v3;
            length_list(a402)
        };
        let a404 = 1u64;
        nat_add(a403, a404)?
    })
}

pub fn f107(v0: Adt14, v1: List<Adt13>) -> List<Adt13> {
    let m405 = v1;
    match m405.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            if if f182(v2.clone(), f8(Str::lit("subject"))) {
                f183(v0.clone(), v2.clone())
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
    let m406 = v1;
    match m406.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v2, v3)) => {
            let m407 = f184(f81(f53(v0.clone(), v2.clone()))?, f72(v2, f173()))?;
            match m407 {
                None => {
                    f108(v0, v3)
                }
                Some(v4) => {
                    Ok(f185(v4, f108(v0, v3)?))
                }
            }
        }
    }
}

pub fn f109(v0: Adt1) -> R<List<Bytes>> {
    Ok(f98(f118(f186(f172(), f120(match v0.clone() {
        Adt1::C0(_, h408, _) => {
            h408
        }
    }, f172())))?, f118(f186(f173(), f120(match v0 {
        Adt1::C0(_, h409, _) => {
            h409
        }
    }, f173())))?))
}

pub fn f110(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f111(p413: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m412 = p413;
    {
        let mut m412 = m412;
        loop {
            let step = {
                let v0 = m412;
                let m410 = v0;
                match m410.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h411, _) = v2;
                            h411
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m412 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f113(v0: Adt1) -> R<List<Bytes>> {
    f118(f186(f112(), f120(match v0 {
        Adt1::C0(_, h414, _) => {
            h414
        }
    }, f112())))
}

pub fn f114(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m415 = v1;
    match m415.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f98(if f41(v4.clone(), v3.clone()) {
                f187(v4.clone(), f81(f53(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f188(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
            }, f114(v0, v5, v2, v3)?))
        }
    }
}

pub fn f115(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m416 = v0;
    match m416.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f47(v2, v1.clone()), f115(v3, v1))
        }
    }
}

pub fn f116(v0: Adt1) -> R<Adt11> {
    let m417 = f45(v0.clone(), f189());
    match m417.uncons() {
        None => {
            let m418 = f190(v0.clone(), f189())?;
            match m418.uncons() {
                None => {
                    let m421 = f191(match v0.clone() {
                        Adt1::C0(_, h419, _) => {
                            h419
                        }
                    }, f98(f192(), f193(match v0.clone() {
                        Adt1::C0(_, h420, _) => {
                            h420
                        }
                    }, false)));
                    match m421.uncons() {
                        None => {
                            let m423 = f45(v0.clone(), f193(match v0.clone() {
                                Adt1::C0(_, h422, _) => {
                                    h422
                                }
                            }, true));
                            match m423.uncons() {
                                None => {
                                    let m424 = f45(v0.clone(), f194());
                                    match m424.uncons() {
                                        None => {
                                            let m425 = f195(v0.clone())?;
                                            match m425 {
                                                None => {
                                                    let m427 = f196(v0.clone(), f193(match v0.clone() {
                                                        Adt1::C0(_, h426, _) => {
                                                            h426
                                                        }
                                                    }, true))?;
                                                    match m427 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f197(Adt16::C0(Adt15::C0(f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f175(v0.clone())?, f115(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f52(f53(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f176(f82(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f193(match v0 {
                                                                Adt1::C0(_, h428, _) => {
                                                                    h428
                                                                }
                                                            }, true)))?;
                                                            let (h429, _) = v13;
                                                            if h429 {
                                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h430) = v13;
                                                                    h430
                                                                }))
                                                            } else {
                                                                Ok(Adt11::C4({
                                                                    let (_, h431) = v13;
                                                                    h431
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

pub fn f117() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f118(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f198(f93(Fn1::F199, List::<Bytes>::nil(), f200(v0)?)))
}

pub fn f119(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m432 = v1;
    match m432.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f201(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f202(v0.clone(), v2), f119(v0, v3))
            } else {
                f119(v0, v3)
            }
        }
    }
}

pub fn f120(p437: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m436 = p437;
    {
        let mut m436 = m436;
        loop {
            let step = {
                let v0 = m436;
                let m433 = v0;
                match m433.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h434, _) = v2;
                            h434
                        }, v1.clone()) {
                            let (_, h435) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h435)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt1, v1: List<List<Bytes>>, p440: List<Bytes>) -> R<Option<Bytes>> {
    let m439 = p440;
    {
        let mut m439 = m439;
        loop {
            let step = {
                let v2 = m439;
                let m438 = v2;
                match m438.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f203(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
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

pub fn f122(p443: List<Bytes>, p444: List<Bytes>, p445: List<List<Bytes>>) -> List<List<Bytes>> {
    let m442 = (p443, (p444, p445));
    {
        let mut m442 = m442;
        loop {
            let step = {
                let (v0, (v1, v2)) = m442;
                let m441 = v0;
                match m441.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f204(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f28(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f204(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f123(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m446 = v0;
    match m446.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f123(v3, v1)
            } else {
                List::cons(v2, f123(v3, v1))
            }
        }
    }
}

pub fn f124(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h447, _) => {
            h447
        }
    }, f205())) {
        Ok(Adt11::C1(List::cons(f205(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f123(f118(f119(f205(), f120(match v0.clone() {
            Adt1::C0(_, h448, _) => {
                h448
            }
        }, f205())))?, v1.clone());
        let m449 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m449.uncons() {
            None => {
                let v5: List<Bytes> = f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m450 = f45(v0.clone(), f206(v0.clone(), v2.clone(), v5.clone())?);
                match m450.uncons() {
                    None => {
                        let m451 = f207(v0.clone(), v2, v5)?;
                        match m451 {
                            None => {
                                f208(v0, v1)
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

pub fn f125() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f126(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m452 = v0;
    match m452.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m453 = f82(v0.clone(), f125())?;
    match m453.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f209(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f128() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f129() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f130() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f210(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op > target/attempt.txt 2>&1; then\u{a}                tee -a target/operation.txt < target/attempt.txt\u{a}                last=$op\u{a}              else\u{a}                cat target/attempt.txt\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f131() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      merges: ${{ steps.stale.outputs.merges }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          merges=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # Drafting drops a pull request's auto-merge, so the owner's merge method is kept\u{a}              # here and restored once the regenerated pull request is ready again.\u{a}              method=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge.merge_method // \"\"')\u{a}              if [ -n \"$method\" ]; then merges=\"$merges $pr:$method\"; fi\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"merges=$merges\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          MERGES: ${{ needs.find.outputs.merges }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f210(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            # The owner's auto-merge comes back with the same method, and only where it was set.\u{a}            for entry in $MERGES; do\u{a}              if [ \"${entry/:*/}\" = \"$pr\" ] && ! gh pr merge \"$pr\" --auto \"--${entry#*:}\"; then\u{a}                echo \"::error::#${pr}: its auto-merge (${entry#*:}) could not be restored\"\u{a}                failed=\"$failed $pr\"\u{a}              fi\u{a}            done\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f132() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f133() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f134(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f100(v0.clone())?;
    let v2: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f107(v1.clone(), f108(v0.clone(), f109(v0.clone())?)?);
    let v4: List<List<Bytes>> = f211(f212(f53(v0, f174()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f213(v4.clone(), f214(v4), f179(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f106(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f135(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f215(f100(v0.clone())?, f108(v0.clone(), f109(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f136(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f216(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f217(f82(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f137() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f138(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m454 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m454.uncons() {
        None => {
            if f218(f81(f53(v0, v1))?)? {
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

pub fn f139(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m455 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m455.uncons() {
        None => {
            let v5: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
            if f40(f201(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f219(v5)?
                } else {
                    f220(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f202(f221(), f222(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f202(f221(), f222(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f223()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f140(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m456 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m456.uncons() {
        None => {
            let m457 = f224(f81(f53(v0.clone(), v1.clone()))?)?;
            match m457 {
                None => {
                    Ok(Adt11::C4(f225()))
                }
                Some(v4) => {
                    let m458 = f226(f81(f53(v0, v1))?)?;
                    match m458 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f95(f38(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f227(v5)))
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

pub fn f141(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let v2: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
    if f40(f201(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f228(f229(f230(v2.clone())?), f40({
            let a460 = {
                let a459 = f231(f230(v2)?);
                length_list(a459)
            };
            let a461 = 0u64;
            nat_eq(a460, a461)
        })) {
            Ok(Adt11::C3(List::cons((f202(f221(), f222(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f202(f221(), f222(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt11::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f142(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m462 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m462.uncons() {
        None => {
            let m463 = f232(f81(f53(v0, v1))?)?;
            match m463 {
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

pub fn f143(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m464 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m464.uncons() {
        None => {
            let v5: Bytes = f202(if v2 {
                f173()
            } else {
                f172()
            }, f222(v1.clone())?);
            if f40(f201(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m465 = f184(f81(f53(v0.clone(), v1.clone()))?, v2)?;
                match m465 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a467 = {
                            let a466 = v6;
                            length_list(a466)
                        };
                        let a468 = 0u64;
                        if nat_eq(a467, a468) {
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

pub fn f144(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m469 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m469.uncons() {
        None => {
            let v4: List<Bytes> = f81(f53(v0.clone(), v1))?;
            let m470 = f233(v4.clone(), f8(Str::lit("requirement")))?;
            match m470 {
                None => {
                    Ok(Adt11::C4(f223()))
                }
                Some(v5) => {
                    let m471 = f233(v4, f8(Str::lit("admission")))?;
                    match m471 {
                        None => {
                            Ok(Adt11::C4(f223()))
                        }
                        Some(v6) => {
                            let v7: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f98(f115(v5.clone(), f8(Str::lit("id"))), f115(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f118(f98(f115(v7.clone(), f8(Str::lit("id"))), f115(v8.clone(), f8(Str::lit("construct")))))?;
                            let m472 = f234(f104(v6.clone()), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m472 {
                                None => {
                                    let a474 = {
                                        let a473 = v9.clone();
                                        length_list(a473)
                                    };
                                    let a475 = 0u64;
                                    if nat_eq(a474, a475) {
                                        Ok(Adt11::C4(f223()))
                                    } else {
                                        if f40({
                                            let a478 = {
                                                let a476 = f118(f98(v9.clone(), v10.clone()))?;
                                                length_list(a476)
                                            };
                                            let a479 = {
                                                let a477 = f98(v9.clone(), v10);
                                                length_list(a477)
                                            };
                                            nat_eq(a478, a479)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f236(f8(Str::lit("model/requirements.json")), f185(v7, v5))?, List::cons(f236(f8(Str::lit("model/admissions.json")), f185(v8, v6))?, List::cons(f237(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f95(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f235(v11))
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

pub fn f145(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m480 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m480.uncons() {
        None => {
            let m481 = f238(v0.clone(), v1.clone())?;
            match m481 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m482 = f239(v4.clone(), f8(Str::lit("schema")));
                    match m482 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f240(v0, f241(v5.clone())?, v5, f242(v4.clone(), f8(Str::lit("name"))), f243(f244(f239(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f146(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt11> {
    let m483 = v2;
    match m483.uncons() {
        None => {
            Ok(Adt11::C4(f66()))
        }
        Some((v3, v4)) => {
            let m484 = v4;
            match m484.uncons() {
                None => {
                    Ok(Adt11::C4(f66()))
                }
                Some((v5, v6)) => {
                    let m485 = v6.clone();
                    match m485.uncons() {
                        None => {
                            Ok(Adt11::C4(f66()))
                        }
                        Some((v7, v8)) => {
                            let m486 = v8;
                            match m486.uncons() {
                                None => {
                                    Ok(Adt11::C4(f66()))
                                }
                                Some((v9, _)) => {
                                    if f40(f72(v7, f8(Str::lit("read-")))) {
                                        Ok(Adt11::C4(f8(Str::lit("name the verdant_readers command that reads the artifact: admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT\u{a}"))))
                                    } else {
                                        let m487 = f45(v0.clone(), List::cons(f245(v1.clone()), List::cons(v9.clone(), List::cons(f8(Str::lit("model/admissions.json")), if f28(v5.clone(), f8(Str::lit("none"))) {
                                            List::<Bytes>::nil()
                                        } else {
                                            List::cons(f7(List::cons(f8(Str::lit("encodings/")), List::cons(v5.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), List::<Bytes>::nil())
                                        }))));
                                        match m487.uncons() {
                                            None => {
                                                if f40(f56(v0.clone(), f245(v1.clone()))) {
                                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v1.clone(), List::cons(f8(Str::lit(" is not a bound authority, because ")), List::cons(f245(v1), List::cons(f8(Str::lit(" is absent, and every admitted value cites a bound standard (REQ-ADMISSION)\u{a}")), List::<Bytes>::nil()))))))))
                                                } else {
                                                    let m488 = f46(v0.clone(), f245(v1.clone()))?;
                                                    match m488 {
                                                        None => {
                                                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(f245(v1), List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                        Some(v13) => {
                                                            let v15: Bytes = {
                                                                let m489 = f79(v13, f8(Str::lit("source")));
                                                                match m489 {
                                                                    None => {
                                                                        Bytes::lit(&[])
                                                                    }
                                                                    Some(v14) => {
                                                                        f47(v14, f8(Str::lit("sha256")))
                                                                    }
                                                                }
                                                            };
                                                            let v16: Bytes = f246(f53(v0.clone(), v9.clone()))?;
                                                            if f40(f28(v16.clone(), v15.clone())) {
                                                                Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v9, List::cons(f8(Str::lit(" is not the artifact ")), List::cons(v1, List::cons(f8(Str::lit(" binds: its SHA-256 is ")), List::cons(v16, List::cons(f8(Str::lit(" and the binding's source.sha256 is ")), List::cons(v15, List::cons(f8(Str::lit(" (FIPS 180-4 section 6.2)\u{a}")), List::<Bytes>::nil()))))))))))))
                                                            } else {
                                                                f247(v0, v1, v3, v5, v6, v9)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Some((v11, v12)) => {
                                                Ok(Adt11::C0(List::cons(v11, v12)))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f147(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m490 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m490.uncons() {
        None => {
            let m491 = f46(v0.clone(), v1.clone())?;
            match m491 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f40(f28(f47(v4.clone(), f8(Str::lit("schema"))), f248())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt13 = {
                            let m492 = f79(v4.clone(), f8(Str::lit("source")));
                            match m492 {
                                None => {
                                    Adt13::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f78(f79(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f202(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f202(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m493 = v8;
                        match m493 {
                            None => {
                                f249(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f40(f28(f47(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f249(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f202(v9, f222(f47(v7.clone(), f8(Str::lit("url"))))?);
                                    if f40(f41(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h494) => {
                                            h494
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f47(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m495 = f45(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m495.uncons() {
                                            None => {
                                                if f28(f246(f53(v0.clone(), v12.clone()))?, v11.clone()) {
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

pub fn f148(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m496 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m496.uncons() {
        None => {
            let v4: Bytes = f53(v0.clone(), v1);
            let v5: List<List<Bytes>> = f212(v4.clone())?;
            if f40(f84(f250(v5.clone()), f251())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f252(Str::lit("id"), v6.clone())?, List::cons(f252(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f252(Str::lit("artifact"), f174())?, List::cons(f252(Str::lit("checksum"), f246(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f174(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f253(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f237(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f95(f214(f211(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f149(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m497 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m497.uncons() {
        None => {
            let m498 = f46(v0, v1)?;
            match m498 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f254(f158(Some(v4))) {
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

pub fn f150(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m499 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m499.uncons() {
        None => {
            let m500 = f238(v0.clone(), v1.clone())?;
            match m500 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f240(v0.clone(), f53(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f151(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m501 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f125(), List::<Bytes>::nil())));
    match m501.uncons() {
        None => {
            let m502 = f255(f81(f53(v0.clone(), v1))?)?;
            match m502 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt13> = f82(v0, f125())?;
                    let v6: List<Bytes> = f115(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f118(f115(v5.clone(), f8(Str::lit("at"))))?;
                    let a504 = {
                        let a503 = v6.clone();
                        length_list(a503)
                    };
                    let a505 = 0u64;
                    if if nat_eq(a504, a505) {
                        true
                    } else {
                        f40({
                            let a508 = {
                                let a506 = f118(f98(v6.clone(), v7.clone()))?;
                                length_list(a506)
                            };
                            let a509 = {
                                let a507 = f98(v6.clone(), v7);
                                length_list(a507)
                            };
                            nat_eq(a508, a509)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f236(f125(), f185(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f95(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f152(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt11> {
    let m510 = v2;
    match m510.uncons() {
        None => {
            Ok(Adt11::C4(f66()))
        }
        Some((v3, v4)) => {
            let m511 = v4;
            match m511.uncons() {
                None => {
                    Ok(Adt11::C4(f66()))
                }
                Some((v5, v6)) => {
                    let m512 = v6.clone();
                    match m512.uncons() {
                        None => {
                            Ok(Adt11::C4(f66()))
                        }
                        Some((_, _)) => {
                            let m513 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f256(), List::<Bytes>::nil())));
                            match m513.uncons() {
                                None => {
                                    let v11: Bytes = f95(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f53(v0.clone(), v1);
                                    let v13: List<Adt13> = f82(v0, f256())?;
                                    if f257(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt11::C3(List::cons(f236(f256(), f185(v13, List::cons(f258(v11.clone(), v12, v5, v3), List::<Adt13>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
                                    }
                                }
                                Some((v9, v10)) => {
                                    Ok(Adt11::C0(List::cons(v9, v10)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f153(v0: Bytes, p567: u64, p568: u64) -> R<bool> {
    let m566 = (p567, p568);
    {
        let mut m566 = m566;
        loop {
            let step = {
                let (v1, v2) = m566;
                let m514 = v2;
                if m514 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m514 - 1;
                    let m517 = {
                        let a515 = v0.clone();
                        let a516 = v1;
                        index_bytes(a515, a516)
                    };
                    match m517 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f259({
                                let v5: u8 = v4;
                                let a562 = {
                                    let a538 = {
                                        let a526 = {
                                            let a520 = {
                                                let a518 = v5;
                                                let a519 = 128u8;
                                                fixed_u8::bit_and(a518, a519)
                                            };
                                            let a521 = 128u8;
                                            if equal(a520, a521) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a527 = {
                                            let a524 = {
                                                let a522 = v5;
                                                let a523 = 64u8;
                                                fixed_u8::bit_and(a522, a523)
                                            };
                                            let a525 = 64u8;
                                            if equal(a524, a525) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a526, a527)?
                                    };
                                    let a539 = {
                                        let a536 = {
                                            let a530 = {
                                                let a528 = v5;
                                                let a529 = 32u8;
                                                fixed_u8::bit_and(a528, a529)
                                            };
                                            let a531 = 32u8;
                                            if equal(a530, a531) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a537 = {
                                            let a534 = {
                                                let a532 = v5;
                                                let a533 = 16u8;
                                                fixed_u8::bit_and(a532, a533)
                                            };
                                            let a535 = 16u8;
                                            if equal(a534, a535) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a536, a537)?
                                    };
                                    nat_add(a538, a539)?
                                };
                                let a563 = {
                                    let a560 = {
                                        let a548 = {
                                            let a542 = {
                                                let a540 = v5;
                                                let a541 = 8u8;
                                                fixed_u8::bit_and(a540, a541)
                                            };
                                            let a543 = 8u8;
                                            if equal(a542, a543) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a549 = {
                                            let a546 = {
                                                let a544 = v5;
                                                let a545 = 4u8;
                                                fixed_u8::bit_and(a544, a545)
                                            };
                                            let a547 = 4u8;
                                            if equal(a546, a547) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a548, a549)?
                                    };
                                    let a561 = {
                                        let a558 = {
                                            let a552 = {
                                                let a550 = v5;
                                                let a551 = 2u8;
                                                fixed_u8::bit_and(a550, a551)
                                            };
                                            let a553 = 2u8;
                                            if equal(a552, a553) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a559 = {
                                            let a556 = {
                                                let a554 = v5;
                                                let a555 = 1u8;
                                                fixed_u8::bit_and(a554, a555)
                                            };
                                            let a557 = 1u8;
                                            if equal(a556, a557) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a558, a559)?
                                    };
                                    nat_add(a560, a561)?
                                };
                                nat_add(a562, a563)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a564 = v1;
                                    let a565 = 1u64;
                                    nat_add(a564, a565)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m566 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: Bytes) -> R<Option<Adt17>> {
    let v1: Bytes = f260(v0)?;
    let m572 = f261(v1.clone(), 0u64, {
        let a570 = {
            let a569 = v1.clone();
            length_bytes(a569)
        };
        let a571 = 2u64;
        nat_add(a570, a571)?
    }, 0u64)?;
    match m572 {
        None => {
            Ok(None::<Adt17>)
        }
        Some(v2) => {
            let a575 = {
                let (_, h573) = v2;
                h573
            };
            let a576 = {
                let a574 = v1;
                length_bytes(a574)
            };
            if nat_eq(a575, a576) {
                Ok(Some({
                    let (h577, _) = v2;
                    h577
                }))
            } else {
                Ok(None::<Adt17>)
            }
        }
    }
}

pub fn f155(v0: Adt17) -> Option<Adt13> {
    let m578 = v0;
    match m578 {
        Adt17::C0 => {
            Some(Adt13::C3(std::rc::Rc::new(Adt13::C5)))
        }
        Adt17::C1(_, _) => {
            None::<Adt13>
        }
        Adt17::C2(v3) => {
            Some(Adt13::C0(v3))
        }
        Adt17::C3(v4, v5) => {
            Some(Adt13::C1(v4, v5))
        }
        Adt17::C4(_, _, _) => {
            None::<Adt13>
        }
        Adt17::C5(v9) => {
            Some(Adt13::C2(v9))
        }
        Adt17::C6(r585) => {
            let v10 = (*r585).clone();
            let m579 = f155(v10);
            match m579 {
                None => {
                    None::<Adt13>
                }
                Some(v11) => {
                    Some(Adt13::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt17::C7(r586) => {
            let v12 = (*r586).clone();
            let m580 = f155(v12);
            match m580 {
                None => {
                    None::<Adt13>
                }
                Some(v13) => {
                    Some(Adt13::C4(std::rc::Rc::new(v13)))
                }
            }
        }
        Adt17::C8 => {
            Some(Adt13::C5)
        }
        Adt17::C9(r587, r588) => {
            let v14 = (*r587).clone();
            let v15 = (*r588).clone();
            let m581 = f155(v14);
            match m581 {
                None => {
                    None::<Adt13>
                }
                Some(v16) => {
                    let m582 = f155(v15);
                    match m582 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v17) => {
                            Some(Adt13::C6(std::rc::Rc::new(v16), std::rc::Rc::new(v17)))
                        }
                    }
                }
            }
        }
        Adt17::C10(v18, r589, r590) => {
            let v19 = (*r589).clone();
            let v20 = (*r590).clone();
            let m583 = f155(v19);
            match m583 {
                None => {
                    None::<Adt13>
                }
                Some(v21) => {
                    let m584 = f155(v20);
                    match m584 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v22) => {
                            Some(Adt13::C7(v18, std::rc::Rc::new(v21), std::rc::Rc::new(v22)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f156(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), v1) {
        let m591 = f33(v0, 34u8)?;
        match m591.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m592 = v3;
                match m592.uncons() {
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

pub fn f157(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f262(v0, v1)
}

pub fn f158(v0: Option<Adt13>) -> List<Adt13> {
    let m593 = v0;
    match m593 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            f263(v1)
        }
    }
}

pub fn f159(v0: Adt13, v1: Bytes) -> Adt13 {
    let m594 = v0;
    match m594 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r595) => {
            let v10 = (*r595).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r596) => {
            let v5 = (*r596).clone();
            Adt13::C4(std::rc::Rc::new(f159(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r597, r598) => {
            let v11 = (*r597).clone();
            let v12 = (*r598).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r599, r600) => {
            let v3 = (*r599).clone();
            let v4 = (*r600).clone();
            if f28(v2.clone(), f8(Str::lit("std"))) {
                Adt13::C7(v2, std::rc::Rc::new(f264(v3, v1.clone())), std::rc::Rc::new(f159(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f159(v4, v1)))
            }
        }
    }
}

pub fn f160(v0: Adt13) -> R<Bytes> {
    let m601 = v0;
    match m601 {
        Adt13::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt13::C1(v2, v3) => {
            f265(v2, v3)
        }
        Adt13::C2(v4) => {
            f266(v4)
        }
        Adt13::C3(r602) => {
            let v5 = (*r602).clone();
            Ok(f267(f268(v5)?))
        }
        Adt13::C4(r603) => {
            let v6 = (*r603).clone();
            f269(f270(v6)?)
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

pub fn f161(v0: Adt1, p607: List<Adt12>) -> R<Option<Adt12>> {
    let m606 = p607;
    {
        let mut m606 = m606;
        loop {
            let step = {
                let v1 = m606;
                let m604 = v1;
                match m604.uncons() {
                    None => {
                        Err::<List<Adt12>, R<Option<Adt12>>>(Ok(None::<Adt12>))
                    }
                    Some((v2, v3)) => {
                        if f41(f162(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h605) => {
                                h605
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
                Ok(next) => { m606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Adt12) -> R<Bytes> {
    Ok(f202(f271(), f222(f272(v0))?))
}

pub fn f163(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f272(v0), List::<Bytes>::nil())))))
}

pub fn f164() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f165() -> Bytes {
    f273()
}

pub fn f166(v0: List<Adt12>) -> R<List<Bytes>> {
    let m608 = v0;
    match m608.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f162(v1)?, f166(v2)?))
        }
    }
}

pub fn f167(v0: Adt1, v1: List<Adt12>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m609 = v1;
    match m609.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f57(f58(v5.clone(), f50(v2.clone()))), f274(v5.clone(), v2.clone(), v3.clone(), f246(f53(v0.clone(), f162(v5)?))?, v4.clone())?), f167(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f168(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f275(v0.clone())?;
    let v2: u64 = f276(v0.clone(), v1)?;
    Ok(f35(v0, v2, {
        let a610 = v1;
        let a611 = v2;
        nat_sub(a610, a611)
    }))
}

pub fn f169(v0: Bytes, v1: u64) -> Option<u8> {
    let a612 = v0;
    let a613 = v1;
    index_bytes(a612, a613)
}

pub fn f170(v0: Fn2, p617: Option<Bytes>, p618: List<Bytes>) -> Option<Bytes> {
    let m616 = (p617, p618);
    {
        let mut m616 = m616;
        loop {
            let step = {
                let (v1, v2) = m616;
                let m614 = v2;
                match m614.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c615: Fn2 = v0.clone();
                            c615.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m619 = v1;
    match m619 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a622 = {
                    let a620 = v3;
                    let a621 = v0;
                    append_bytes(a620, a621)
                };
                let a623 = v2;
                append_bytes(a622, a623)
            })
        }
    }
}

pub fn f172() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f173() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f174() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f175(v0: Adt1) -> R<List<Bytes>> {
    let m624 = f212(f53(v0, f174()))?;
    match m624.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f214(v2))
        }
    }
}

pub fn f176(v0: List<Adt13>) -> List<Bytes> {
    let m625 = v0;
    match m625.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m626 = f78(Some(v1));
            match m626 {
                None => {
                    f176(v2)
                }
                Some(v3) => {
                    List::cons(v3, f176(v2))
                }
            }
        }
    }
}

pub fn f177(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m627 = v0;
    match m627.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f47(v1.clone(), f8(Str::lit("subject"))), f47(v1, f8(Str::lit("object")))), f177(v2))
        }
    }
}

pub fn f178(v0: List<Adt18>) -> List<(Bytes, Bytes)> {
    let m628 = v0;
    match m628.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt18::C0(h629, _, _) => {
                    h629
                }
            }, match v1 {
                Adt18::C0(_, _, h630) => {
                    h630
                }
            }), f178(v2))
        }
    }
}

pub fn f179(v0: Adt14, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Adt18>> {
    let v3: List<(Bytes, Bytes)> = f277(v1);
    let v4: List<(Bytes, Bytes)> = f278(v0, v2);
    f279(v3.clone(), f126(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f180(p637: List<Bytes>, v1: List<(Bytes, Bytes)>, p638: u64) -> R<List<Bytes>> {
    let m636 = (p637, p638);
    {
        let mut m636 = m636;
        loop {
            let step = {
                let (v0, v2) = m636;
                let m631 = v2;
                if m631 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m631 - 1;
                    let v4: List<Bytes> = f118(f98(v0.clone(), f280(v0.clone(), v1.clone())))?;
                    let a634 = {
                        let a632 = v4.clone();
                        length_list(a632)
                    };
                    let a635 = {
                        let a633 = v0.clone();
                        length_list(a633)
                    };
                    if nat_eq(a634, a635) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m636 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: List<Adt13>) -> List<Bytes> {
    let m639 = v0;
    match m639.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m640 = f79(v1, f8(Str::lit("std")));
            match m640 {
                None => {
                    f181(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3.clone(), f8(Str::lit("authority"))), if f182(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f47(v3, f8(Str::lit("row"))), f181(v2))
                    } else {
                        f181(v2)
                    })
                }
            }
        }
    }
}

pub fn f182(v0: Adt13, v1: Bytes) -> bool {
    f40({
        let a642 = {
            let a641 = f47(v0, v1);
            length_bytes(a641)
        };
        let a643 = 0u64;
        nat_eq(a642, a643)
    })
}

pub fn f183(v0: Adt14, v1: Adt13) -> bool {
    if f281(v0.clone(), v1.clone(), f47(v1.clone(), f8(Str::lit("subject")))) {
        f281(v0, v1.clone(), f47(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f184(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt13>>> {
    let m644 = v0;
    match m644.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f168(v2)?;
            let a646 = {
                let a645 = v4.clone();
                length_bytes(a645)
            };
            let a647 = 0u64;
            if nat_eq(a646, a647) {
                f184(v3, v1)
            } else {
                let m648 = f77(v4)?;
                match m648 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f282(v5.clone())
                        } else {
                            f283(v5.clone())
                        } {
                            let m649 = f184(v3, v1)?;
                            match m649 {
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

pub fn f185(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m650 = v0;
    match m650.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f185(v3, v1))
        }
    }
}

pub fn f186(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m651 = v1;
    match m651.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f201(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f202(v0.clone(), v2), f186(v0, v3))
            } else {
                f186(v0, v3)
            }
        }
    }
}

pub fn f187(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m652 = v1;
    match m652.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f98({
                let a654 = {
                    let a653 = f168(v4.clone())?;
                    length_bytes(a653)
                };
                let a655 = 0u64;
                if nat_eq(a654, a655) {
                    List::<Bytes>::nil()
                } else {
                    f284(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f187(v0, v5, v2, v3)?))
        }
    }
}

pub fn f188(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f189() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f174(), List::<Bytes>::nil())))))))
}

pub fn f190(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m656 = v1;
    match m656.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f56(v0.clone(), v2.clone()) {
                f201(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m657 = f46(v0.clone(), v2.clone())?;
                match m657 {
                    None => {
                        Ok(List::cons(v2, f190(v0, v3)?))
                    }
                    Some(_) => {
                        f190(v0, v3)
                    }
                }
            } else {
                f190(v0, v3)
            }
        }
    }
}

pub fn f191(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m658 = v1;
    match m658.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f111(v0.clone(), v2.clone()) {
                f191(v0, v3)
            } else {
                List::cons(v2, f191(v0, v3))
            }
        }
    }
}

pub fn f192() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f193(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m659 = v0;
    match m659.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f98(f285({
                let (h660, _) = v2;
                h660
            }, {
                let (_, h661) = v2;
                h661
            }, v1), f193(v3, v1))
        }
    }
}

pub fn f194() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f195(v0: Adt1) -> R<Option<Bytes>> {
    f286(f287(v0, f194())?)
}

pub fn f196(v0: Adt1, p665: List<Bytes>) -> R<Option<Bytes>> {
    let m664 = p665;
    {
        let mut m664 = m664;
        loop {
            let step = {
                let v1 = m664;
                let m662 = v1;
                match m662.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m663 = f288(f53(v0.clone(), v2.clone()), f289())?;
                        match m663 {
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
                Ok(next) => { m664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h666, _, _, _, _) => {
            h666
        }
    };
    let v2: Adt19 = f290(v1.clone())?;
    let v3: List<Bytes> = f291(v1.clone());
    let v4: List<Bytes> = f115(match v0.clone() {
        Adt16::C0(_, _, h667, _, _) => {
            h667
        }
    }, f8(Str::lit("path")));
    let m669 = f292(v2, List::<u64>::nil(), f293(v1.clone(), match v0.clone() {
        Adt16::C0(_, h668, _, _, _) => {
            h668
        }
    }, 0u64)?);
    match m669 {
        None => {
            if f295(f296(match v0.clone() {
                Adt16::C0(_, _, _, _, h671) => {
                    h671
                }
            }, f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h672) => {
                    h672
                }
            }, v4.clone()))?, f296(v4.clone(), f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h673) => {
                    h673
                }
            }, v4.clone()))?, f297(match v0.clone() {
                Adt16::C0(_, _, h674, _, _) => {
                    h674
                }
            })) {
                let m678 = f298(f296(match v0.clone() {
                    Adt16::C0(_, _, _, h675, _) => {
                        h675
                    }
                }, f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h676, _) => {
                        h676
                    }
                }))?, f296(v3.clone(), f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h677, _) => {
                        h677
                    }
                }))?);
                match m678 {
                    None => {
                        let m681 = f299(f115(match v0 {
                            Adt16::C0(_, h680, _, _, _) => {
                                h680
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m681 {
                            None => {
                                let m682 = f299(v4, List::<Bytes>::nil());
                                match m682 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f294(f98(v3, match v0 {
                            Adt16::C0(_, _, _, h679, _) => {
                                h679
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f294(match v1 {
                Adt15::C0(_, _, _, _, h670) => {
                    h670
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f198(v0: List<Bytes>) -> List<Bytes> {
    f93(Fn1::F300, List::<Bytes>::nil(), v0)
}

pub fn f199(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f301(v0, v1)
}

pub fn f200(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f302(f303(f304(v0))?))
}

pub fn f201(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a683 = v0.clone();
        length_bytes(a683)
    };
    let v3: u64 = {
        let a684 = v1.clone();
        length_bytes(a684)
    };
    let a685 = v3;
    let a686 = v2;
    if nat_le(a685, a686) {
        let m692 = {
            let a689 = v0;
            let a690 = {
                let a687 = v2;
                let a688 = v3;
                nat_sub(a687, a688)
            };
            let a691 = v3;
            slice_bytes(a689, a690, a691)
        };
        match m692 {
            None => {
                false
            }
            Some(v4) => {
                let a693 = v4;
                let a694 = v1;
                equal(a693, a694)
            }
        }
    } else {
        false
    }
}

pub fn f202(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f203(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m695 = f46(v0, v2)?;
    match m695 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f305(v1, f47(v3.clone(), f8(Str::lit("id"))))?;
            let m696 = v4.clone();
            match m696.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m698 = f78(f79({
                        let m697 = f79(v3.clone(), f8(Str::lit("source")));
                        match m697 {
                            None => {
                                Adt13::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m698 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f40(f41(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f306(f307(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f204(p701: List<Bytes>, p702: List<Bytes>) -> List<Bytes> {
    let m700 = (p701, p702);
    {
        let mut m700 = m700;
        loop {
            let step = {
                let (v0, v1) = m700;
                let m699 = v0;
                match m699.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f205() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f206(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m703 = v1;
    match m703.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt17 = f308(v0.clone(), v3)?;
            if f41(f309(f242(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f310(v5)?, f206(v0, v4, v2)?))
            } else {
                f206(v0, v4, v2)
            }
        }
    }
}

pub fn f207(v0: Adt1, p707: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m706 = p707;
    {
        let mut m706 = m706;
        loop {
            let step = {
                let v1 = m706;
                let m704 = v1;
                match m704.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt17 = f308(v0.clone(), v3.clone())?;
                        if if f41(f309(f242(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f28(f241({
                                let m705 = f239(v5.clone(), f8(Str::lit("schema")));
                                match m705 {
                                    None => {
                                        Adt17::C8
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f53(v0.clone(), f310(v5)?))
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
                Ok(next) => { m706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h708, _) => {
            h708
        }
    }, f221())) {
        Ok(Adt11::C1(List::cons(f221(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f123(f118(f311(f120(match v0.clone() {
            Adt1::C0(_, h709, _) => {
                h709
            }
        }, f221())))?, v1.clone());
        let m710 = f45(v0.clone(), List::cons(f125(), List::cons(f174(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m710.uncons() {
            None => {
                let m711 = f45(v0.clone(), f312(v0.clone(), v2.clone())?);
                match m711.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f313(v0.clone(), v2, v1)?;
                        let (h712, _) = v7;
                        if h712 {
                            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h713) = v7;
                                h713
                            }))
                        } else {
                            let m714 = f97(v0.clone());
                            match m714.uncons() {
                                None => {
                                    let m715 = f99(v0.clone())?;
                                    match m715.uncons() {
                                        None => {
                                            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h716) = v7;
                                                h716
                                            }, List::cons(f7(f118(f314(f100(v0.clone())?, f108(v0.clone(), f109(v0.clone())?)?))?), List::cons(f315(v0)?, List::<Bytes>::nil()))))))
                                        }
                                        Some((v10, v11)) => {
                                            Ok(Adt11::C0(List::cons(v10, v11)))
                                        }
                                    }
                                }
                                Some((v8, v9)) => {
                                    Ok(Adt11::C1(List::cons(v8, v9)))
                                }
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        Ok(Adt11::C0(List::cons(v5, v6)))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f209(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f316(f82(v0, f125())?)?)))
}

pub fn f210() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f211(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m717 = v0;
    match m717.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f212(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt20 = f317(v0.clone(), 0u64, {
        let a718 = v0;
        length_bytes(a718)
    }, Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt20 = {
        let a721 = {
            let a720 = match v1.clone() {
                Adt20::C0(_, _, h719, _, _) => {
                    h719
                }
            };
            length_bytes(a720)
        };
        let a722 = 0u64;
        if if nat_eq(a721, a722) {
            let m724 = match v1.clone() {
                Adt20::C0(_, _, _, h723, _) => {
                    h723
                }
            };
            match m724.uncons() {
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
            f318(v1)
        }
    };
    Ok(f319(match v4 {
        Adt20::C0(_, _, _, _, h725) => {
            h725
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f213(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt18>, v3: List<Adt13>, v4: List<Bytes>) -> List<Bytes> {
    let m726 = v1;
    match m726.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f98(f320(v5.clone(), v2.clone()), f321(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f322(v0.clone(), v5.clone()), List::cons(if f41(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a728 = {
                    let a727 = v7.clone();
                    length_list(a727)
                };
                let a729 = 0u64;
                if nat_eq(a728, a729) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f95(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f213(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f214(v0: List<List<Bytes>>) -> List<Bytes> {
    let m730 = v0;
    match m730.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m731 = v1;
            match m731.uncons() {
                None => {
                    f214(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f214(v2))
                }
            }
        }
    }
}

pub fn f215(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m732 = v1;
    match m732.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f182(v2.clone(), f8(Str::lit("domain"))) {
                f323(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f47(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m733 = f79(v2.clone(), f8(Str::lit("role")));
                    match m733 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f47(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f47(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f324(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m734 = f79(v2.clone(), f8(Str::lit("origin")));
                    match m734 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f47(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m735 = f79(v2, f8(Str::lit("evidence")));
                    match m735 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f47(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f215(v0, v3))
            } else {
                f215(v0, v3)
            }
        }
    }
}

pub fn f216(v0: List<Adt13>) -> List<Bytes> {
    let m736 = v0;
    match m736.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f47(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f47(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f47(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f216(v2))
        }
    }
}

pub fn f217(v0: List<Adt13>) -> List<Bytes> {
    let m737 = v0;
    match m737.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f47(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f325(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f217(v2))
        }
    }
}

pub fn f218(p743: List<Bytes>) -> R<bool> {
    let m742 = p743;
    {
        let mut m742 = m742;
        loop {
            let step = {
                let v0 = m742;
                let m738 = v0;
                match m738.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a740 = {
                            let a739 = f168(v1)?;
                            length_bytes(a739)
                        };
                        let a741 = 0u64;
                        if nat_eq(a740, a741) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m742 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: List<Bytes>) -> R<bool> {
    if f229(f255(v0.clone())?) {
        Ok(f40({
            let a745 = {
                let a744 = f231(f255(v0)?);
                length_list(a744)
            };
            let a746 = 0u64;
            nat_eq(a745, a746)
        }))
    } else {
        Ok(false)
    }
}

pub fn f220(v0: List<Bytes>) -> R<bool> {
    if f229(f233(v0.clone(), f8(Str::lit("requirement")))?) {
        if f229(f233(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f40({
                let a748 = {
                    let a747 = f185(f231(f233(v0.clone(), f8(Str::lit("requirement")))?), f231(f233(v0, f8(Str::lit("admission")))?));
                    length_list(a747)
                };
                let a749 = 0u64;
                nat_eq(a748, a749)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f221() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f222(v0: Bytes) -> R<Bytes> {
    Ok(f34(f33(v0, 47u8)?))
}

pub fn f223() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f224(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m750 = v0;
    match m750.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f168(v1)?;
            let a752 = {
                let a751 = v3.clone();
                length_bytes(a751)
            };
            let a753 = 0u64;
            if nat_eq(a752, a753) {
                f224(v2)
            } else {
                let m754 = f77(v3.clone())?;
                match m754 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        if f326(f47(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m755 = f224(v2)?;
                            match m755 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f327(v5), {
                                        let a756 = v3;
                                        let a757 = f8(Str::lit("\u{a}"));
                                        append_bytes(a756, a757)
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

pub fn f225() -> Bytes {
    let a758 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a758)
}

pub fn f226(p763: List<Bytes>) -> R<Option<Bytes>> {
    let m762 = p763;
    {
        let mut m762 = m762;
        loop {
            let step = {
                let v0 = m762;
                let m759 = v0;
                match m759.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m760 = f77(f168(v1)?)?;
                        match m760 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m761 = f79(v3.clone(), f8(Str::lit("source")));
                                match m761 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f328(f47(v4.clone(), f8(Str::lit("redistribution"))), f47(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m762 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: Bytes) -> Bytes {
    let a768 = {
        let a764 = Str::lit("binding ");
        utf8_encode(a764)
    };
    let a769 = {
        let a766 = v0;
        let a767 = {
            let a765 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a765)
        };
        append_bytes(a766, a767)
    };
    append_bytes(a768, a769)
}

pub fn f228(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f229(v0: Option<List<Adt13>>) -> bool {
    let m770 = v0;
    match m770 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f230(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m771 = v0;
    match m771.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f168(v1)?;
            let a773 = {
                let a772 = v3.clone();
                length_bytes(a772)
            };
            let a774 = 0u64;
            if nat_eq(a773, a774) {
                f230(v2)
            } else {
                let m775 = f77(v3)?;
                match m775 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if f228(f228(f182(v4.clone(), f8(Str::lit("authority"))), f182(v4.clone(), f8(Str::lit("row")))), f228(f228(f182(v4.clone(), f8(Str::lit("construct"))), f182(v4.clone(), f8(Str::lit("artifact")))), f72(f47(v4.clone(), f8(Str::lit("reader"))), f8(Str::lit("read-"))))) {
                            let m776 = f230(v2)?;
                            match m776 {
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

pub fn f231(v0: Option<List<Adt13>>) -> List<Adt13> {
    let m777 = v0;
    match m777 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f232(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m778 = v0;
    match m778.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f168(v1)?;
            let a780 = {
                let a779 = v3.clone();
                length_bytes(a779)
            };
            let a781 = 0u64;
            if nat_eq(a780, a781) {
                f232(v2)
            } else {
                let m782 = f154(v3.clone())?;
                match m782 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f329(v4.clone()) {
                            let m783 = f232(v2)?;
                            match m783 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f205(), List::cons(f8(Str::lit("/")), List::cons(f330(f242(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a784 = v3;
                                        let a785 = f8(Str::lit("\u{a}"));
                                        append_bytes(a784, a785)
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

pub fn f233(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt13>>> {
    let m786 = v0;
    match m786.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f168(v2)?;
            let a788 = {
                let a787 = v4.clone();
                length_bytes(a787)
            };
            let a789 = 0u64;
            if nat_eq(a788, a789) {
                f233(v3, v1)
            } else {
                let m790 = f77(v4)?;
                match m790 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f331(v5.clone());
                        let a792 = {
                            let a791 = v6.clone();
                            length_bytes(a791)
                        };
                        let a793 = 0u64;
                        if nat_eq(a792, a793) {
                            Ok(None::<List<Adt13>>)
                        } else {
                            let m794 = f233(v3, v1.clone())?;
                            match m794 {
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

pub fn f234(p797: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m796 = p797;
    {
        let mut m796 = m796;
        loop {
            let step = {
                let v0 = m796;
                let m795 = v0;
                match m795.uncons() {
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
                Ok(next) => { m796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f236(v0: Bytes, v1: List<Adt13>) -> R<(Bytes, Bytes)> {
    Ok((v0, f86(Adt13::C3(std::rc::Rc::new(f87(v1))))?))
}

pub fn f237(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f86(Adt13::C3(std::rc::Rc::new(f332(f118(f98(v1, f176(f82(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f238(v0: Adt1, v1: Bytes) -> R<Option<Adt17>> {
    f154(f53(v0, v1))
}

pub fn f239(p803: Adt17, v1: Bytes) -> Option<Adt17> {
    let m802 = p803;
    {
        let mut m802 = m802;
        loop {
            let step = {
                let v0 = m802;
                let m798 = v0;
                match m798 {
                    Adt17::C0 => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C1(_, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C3(_, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C4(_, _, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C5(_) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C6(_) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C7(r799) => {
                        let v5 = (*r799).clone();
                        Ok::<Adt17, Option<Adt17>>(v5)
                    }
                    Adt17::C8 => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C9(_, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C10(v2, r800, r801) => {
                        let v3 = (*r800).clone();
                        let v4 = (*r801).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt17, Option<Adt17>>(Some(v3))
                        } else {
                            Ok::<Adt17, Option<Adt17>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: Adt1, v1: Bytes, v2: Adt17, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m804 = f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m804.uncons() {
        None => {
            let m806 = f234(f104(List::cons(Adt13::C4(std::rc::Rc::new({
                let m805 = f333(v4.clone());
                match m805 {
                    None => {
                        Adt13::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt13>::nil())), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m806 {
                None => {
                    f334(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f235(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f241(v0: Adt17) -> R<Bytes> {
    let a807 = f335(v0)?;
    let a808 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a807, a808))
}

pub fn f242(v0: Adt17, v1: Bytes) -> Bytes {
    let m809 = f336(f239(v0, v1));
    match m809 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f243(v0: List<Adt17>) -> List<Bytes> {
    let m810 = v0;
    match m810.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m811 = f336(Some(v1));
            match m811 {
                None => {
                    f243(v2)
                }
                Some(v3) => {
                    List::cons(v3, f243(v2))
                }
            }
        }
    }
}

pub fn f244(v0: Option<Adt17>) -> List<Adt17> {
    let m812 = v0;
    match m812 {
        None => {
            List::<Adt17>::nil()
        }
        Some(v1) => {
            f337(v1)
        }
    }
}

pub fn f245(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f246(v0: Bytes) -> R<Bytes> {
    Ok(f338(f339(v0)?))
}

pub fn f247(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: List<Bytes>, v5: Bytes) -> R<Adt11> {
    let v6: Bytes = {
        let a813 = f8(Str::lit("registry-read "));
        let a814 = v5;
        append_bytes(a813, a814)
    };
    if f40(f41(v6.clone(), match v0.clone() {
        Adt1::C0(_, _, h815) => {
            h815
        }
    })) {
        Ok(Adt11::C2(f340(), v4, v6))
    } else {
        let m816 = f45(v0.clone(), List::cons(f273(), List::<Bytes>::nil()));
        match m816.uncons() {
            None => {
                f341(v0, v1, f273(), v2, v3, v4)
            }
            Some((v7, v8)) => {
                Ok(Adt11::C0(List::cons(v7, v8)))
            }
        }
    }
}

pub fn f248() -> Bytes {
    f342()
}

pub fn f249(v0: Adt1, v1: Adt13, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f253(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f343(v1, v4)?)?), List::cons(f237(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f250(v0: List<List<Bytes>>) -> List<Bytes> {
    let m817 = v0;
    match m817.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f251() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f252(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f306(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f253(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f344(f122(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f345(f346(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f254(p820: List<Adt13>) -> bool {
    let m819 = p820;
    {
        let mut m819 = m819;
        loop {
            let step = {
                let v0 = m819;
                let m818 = v0;
                match m818.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f28(f47(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f41(f8(Str::lit("studio")), f115(f347(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m819 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m821 = v0;
    match m821.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f168(v1)?;
            let a823 = {
                let a822 = v3.clone();
                length_bytes(a822)
            };
            let a824 = 0u64;
            if nat_eq(a823, a824) {
                f255(v2)
            } else {
                let m825 = f77(v3)?;
                match m825 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if if f182(v4.clone(), f8(Str::lit("agent"))) {
                            if f182(v4.clone(), f8(Str::lit("at"))) {
                                if f182(v4.clone(), f8(Str::lit("quote"))) {
                                    f348(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m826 = f255(v2)?;
                            match m826 {
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

pub fn f256() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f257(p829: List<Adt13>, v1: Bytes, v2: Bytes) -> bool {
    let m828 = p829;
    {
        let mut m828 = m828;
        loop {
            let step = {
                let v0 = m828;
                let m827 = v0;
                match m827.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f228(f28(f47(v3.clone(), f8(Str::lit("observedProperty"))), v1.clone()), f28(f47(v3, f8(Str::lit("hasSimpleResult"))), v2.clone())) {
                            Err::<List<Adt13>, bool>(true)
                        } else {
                            Ok::<List<Adt13>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f258(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt13 {
    Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt13::C2(v2)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt13::C2(v0)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5))))))))))))
}

pub fn f259(v0: u64) -> bool {
    let a830 = v0;
    let a831 = 127u64;
    nat_le(a830, a831)
}

pub fn f260(v0: Bytes) -> R<Bytes> {
    f349(v0.clone(), 0u64, {
        let a832 = v0;
        length_bytes(a832)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f261(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt17, u64)>> {
    let m833 = v2;
    if m833 == 0 {
        Ok(None::<(Adt17, u64)>)
    } else {
        let v4 = m833 - 1;
        let v5: u64 = f350(v0.clone(), v1)?;
        let a834 = v3;
        let a835 = 1u64;
        if nat_eq(a834, a835) {
            let m836 = f261(v0.clone(), v1, v4, 0u64)?;
            match m836 {
                None => {
                    Ok(None::<(Adt17, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f350(v0.clone(), {
                        let (_, h837) = v6.clone();
                        h837
                    })?;
                    let a838 = v7;
                    let a839 = 44u64;
                    if nat_eq(a838, a839) {
                        let m843 = f261(v0, {
                            let a841 = {
                                let (_, h840) = v6;
                                h840
                            };
                            let a842 = 1u64;
                            nat_add(a841, a842)?
                        }, v4, 1u64)?;
                        match m843 {
                            None => {
                                Ok(None::<(Adt17, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt17::C9(std::rc::Rc::new({
                                    let (h844, _) = v6;
                                    h844
                                }), std::rc::Rc::new({
                                    let (h845, _) = v8;
                                    h845
                                })), {
                                    let (_, h846) = v8;
                                    h846
                                })))
                            }
                        }
                    } else {
                        let a847 = v7;
                        let a848 = 93u64;
                        if nat_eq(a847, a848) {
                            Ok(Some((Adt17::C9(std::rc::Rc::new({
                                let (h849, _) = v6;
                                h849
                            }), std::rc::Rc::new(Adt17::C8)), {
                                let a851 = {
                                    let (_, h850) = v6;
                                    h850
                                };
                                let a852 = 1u64;
                                nat_add(a851, a852)?
                            })))
                        } else {
                            Ok(None::<(Adt17, u64)>)
                        }
                    }
                }
            }
        } else {
            let a853 = v3;
            let a854 = 2u64;
            if nat_eq(a853, a854) {
                let m857 = f351(v0.clone(), {
                    let a855 = v1;
                    let a856 = 1u64;
                    nat_add(a855, a856)?
                })?;
                match m857 {
                    None => {
                        Ok(None::<(Adt17, u64)>)
                    }
                    Some(v9) => {
                        if f228({
                            let a858 = v5;
                            let a859 = 34u64;
                            nat_eq(a858, a859)
                        }, {
                            let a861 = f350(v0.clone(), {
                                let (h860, _) = v9.clone();
                                h860
                            })?;
                            let a862 = 58u64;
                            nat_eq(a861, a862)
                        }) {
                            let m866 = f261(v0.clone(), {
                                let a864 = {
                                    let (h863, _) = v9;
                                    h863
                                };
                                let a865 = 1u64;
                                nat_add(a864, a865)?
                            }, v4, 0u64)?;
                            match m866 {
                                None => {
                                    Ok(None::<(Adt17, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f350(v0.clone(), {
                                        let (_, h867) = v10.clone();
                                        h867
                                    })?;
                                    let a868 = v11;
                                    let a869 = 44u64;
                                    if nat_eq(a868, a869) {
                                        let m873 = f261(v0, {
                                            let a871 = {
                                                let (_, h870) = v10;
                                                h870
                                            };
                                            let a872 = 1u64;
                                            nat_add(a871, a872)?
                                        }, v4, 2u64)?;
                                        match m873 {
                                            None => {
                                                Ok(None::<(Adt17, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt17::C10({
                                                    let (_, h874) = v9;
                                                    h874
                                                }, std::rc::Rc::new({
                                                    let (h875, _) = v10;
                                                    h875
                                                }), std::rc::Rc::new({
                                                    let (h876, _) = v12;
                                                    h876
                                                })), {
                                                    let (_, h877) = v12;
                                                    h877
                                                })))
                                            }
                                        }
                                    } else {
                                        let a878 = v11;
                                        let a879 = 125u64;
                                        if nat_eq(a878, a879) {
                                            Ok(Some((Adt17::C10({
                                                let (_, h880) = v9;
                                                h880
                                            }, std::rc::Rc::new({
                                                let (h881, _) = v10;
                                                h881
                                            }), std::rc::Rc::new(Adt17::C8)), {
                                                let a883 = {
                                                    let (_, h882) = v10;
                                                    h882
                                                };
                                                let a884 = 1u64;
                                                nat_add(a883, a884)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt17, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt17, u64)>)
                        }
                    }
                }
            } else {
                let a885 = v5;
                let a886 = 34u64;
                if nat_eq(a885, a886) {
                    let m889 = f351(v0, {
                        let a887 = v1;
                        let a888 = 1u64;
                        nat_add(a887, a888)?
                    })?;
                    match m889 {
                        None => {
                            Ok(None::<(Adt17, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt17::C5({
                                let (_, h890) = v13;
                                h890
                            }), {
                                let (h891, _) = v13;
                                h891
                            })))
                        }
                    }
                } else {
                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt17::C0, {
                            let a892 = v1;
                            let a893 = 4u64;
                            nat_add(a892, a893)?
                        })))
                    } else {
                        if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt17::C2(true), {
                                let a894 = v1;
                                let a895 = 4u64;
                                nat_add(a894, a895)?
                            })))
                        } else {
                            if f28(f35(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt17::C2(false), {
                                    let a896 = v1;
                                    let a897 = 5u64;
                                    nat_add(a896, a897)?
                                })))
                            } else {
                                let a898 = v5;
                                let a899 = 91u64;
                                if nat_eq(a898, a899) {
                                    let a902 = f350(v0.clone(), {
                                        let a900 = v1;
                                        let a901 = 1u64;
                                        nat_add(a900, a901)?
                                    })?;
                                    let a903 = 93u64;
                                    if nat_eq(a902, a903) {
                                        Ok(Some((Adt17::C6(std::rc::Rc::new(Adt17::C8)), {
                                            let a904 = v1;
                                            let a905 = 2u64;
                                            nat_add(a904, a905)?
                                        })))
                                    } else {
                                        let m908 = f261(v0, {
                                            let a906 = v1;
                                            let a907 = 1u64;
                                            nat_add(a906, a907)?
                                        }, v4, 1u64)?;
                                        match m908 {
                                            None => {
                                                Ok(None::<(Adt17, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt17::C6(std::rc::Rc::new({
                                                    let (h909, _) = v14;
                                                    h909
                                                })), {
                                                    let (_, h910) = v14;
                                                    h910
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a911 = v5;
                                    let a912 = 123u64;
                                    if nat_eq(a911, a912) {
                                        let a915 = f350(v0.clone(), {
                                            let a913 = v1;
                                            let a914 = 1u64;
                                            nat_add(a913, a914)?
                                        })?;
                                        let a916 = 125u64;
                                        if nat_eq(a915, a916) {
                                            Ok(Some((Adt17::C7(std::rc::Rc::new(Adt17::C8)), {
                                                let a917 = v1;
                                                let a918 = 2u64;
                                                nat_add(a917, a918)?
                                            })))
                                        } else {
                                            let m921 = f261(v0, {
                                                let a919 = v1;
                                                let a920 = 1u64;
                                                nat_add(a919, a920)?
                                            }, v4, 2u64)?;
                                            match m921 {
                                                None => {
                                                    Ok(None::<(Adt17, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt17::C7(std::rc::Rc::new({
                                                        let (h922, _) = v15;
                                                        h922
                                                    })), {
                                                        let (_, h923) = v15;
                                                        h923
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m924 = f352(v0, v1)?;
                                        match m924 {
                                            None => {
                                                Ok(None::<(Adt17, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h925) = v16;
                                                    h925
                                                }, {
                                                    let (h926, _) = v16;
                                                    h926
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

pub fn f262(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h927, _) = v1.clone();
        h927
    };
    let v3: u64 = {
        let (h928, _) = {
            let (_, h929) = v1.clone();
            h929
        };
        h928
    };
    let v4: List<Bytes> = {
        let (_, h930) = {
            let (_, h931) = v1;
            h931
        };
        h930
    };
    let m932 = f169(v0.clone(), v2);
    match m932 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a933 = v5;
            let a934 = 10u8;
            if equal(a933, a934) {
                Ok(Some(({
                    let a935 = v2;
                    let a936 = 1u64;
                    nat_add(a935, a936)?
                }, ({
                    let a937 = v2;
                    let a938 = 1u64;
                    nat_add(a937, a938)?
                }, List::cons(f353(f35(v0, v3, {
                    let a939 = v2;
                    let a940 = v3;
                    nat_sub(a939, a940)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a941 = v2;
                    let a942 = 1u64;
                    nat_add(a941, a942)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f263(v0: Adt13) -> List<Adt13> {
    let m943 = v0;
    match m943 {
        Adt13::C0(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C1(_, _) => {
            List::<Adt13>::nil()
        }
        Adt13::C2(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C3(r944) => {
            let v3 = (*r944).clone();
            f263(v3)
        }
        Adt13::C4(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C5 => {
            List::<Adt13>::nil()
        }
        Adt13::C6(r945, r946) => {
            let v1 = (*r945).clone();
            let v2 = (*r946).clone();
            List::cons(v1, f263(v2))
        }
        Adt13::C7(_, _, _) => {
            List::<Adt13>::nil()
        }
    }
}

pub fn f264(v0: Adt13, v1: Bytes) -> Adt13 {
    let m947 = v0;
    match m947 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r948) => {
            let v10 = (*r948).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r949) => {
            let v5 = (*r949).clone();
            Adt13::C4(std::rc::Rc::new(f264(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r950, r951) => {
            let v11 = (*r950).clone();
            let v12 = (*r951).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r952, r953) => {
            let v3 = (*r952).clone();
            let v4 = (*r953).clone();
            if f28(v2.clone(), f8(Str::lit("authority"))) {
                Adt13::C7(v2, std::rc::Rc::new(f354(v3, v1.clone())), std::rc::Rc::new(f264(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f264(v4, v1)))
            }
        }
    }
}

pub fn f265(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a954 = 0u64;
        let a955 = v1;
        nat_lt(a954, a955)
    } else {
        false
    } {
        let a956 = Bytes::lit(&[0x2d]);
        let a957 = f355(v1)?;
        Ok(append_bytes(a956, a957))
    } else {
        f355(v1)
    }
}

pub fn f266(v0: Bytes) -> R<Bytes> {
    let a963 = {
        let a961 = Bytes::lit(&[0x22]);
        let a962 = {
            let (_, h958) = {
                let (h959, _) = f356(Fn3::F357(v0.clone()), {
                    let a960 = v0;
                    length_bytes(a960)
                }, (0u64, Bytes::lit(&[])))?;
                h959
            };
            h958
        };
        append_bytes(a961, a962)
    };
    let a964 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a963, a964))
}

pub fn f267(v0: List<Bytes>) -> Bytes {
    let a967 = {
        let a965 = Bytes::lit(&[0x5b]);
        let a966 = f95(v0, Bytes::lit(&[0x2c]));
        append_bytes(a965, a966)
    };
    let a968 = Bytes::lit(&[0x5d]);
    append_bytes(a967, a968)
}

pub fn f268(v0: Adt13) -> R<List<Bytes>> {
    let m969 = v0;
    match m969 {
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
        Adt13::C6(r970, r971) => {
            let v7 = (*r970).clone();
            let v8 = (*r971).clone();
            Ok(List::cons(f160(v7)?, f268(v8)?))
        }
        Adt13::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f269(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f358(Fn4::F359, List::<(Bytes, Bytes)>::nil(), f360(v0)?);
    let v2: List<Bytes> = f361(Fn5::F362, List::<Bytes>::nil(), v1)?;
    let a974 = {
        let a972 = Bytes::lit(&[0x7b]);
        let a973 = f95(v2, Bytes::lit(&[0x2c]));
        append_bytes(a972, a973)
    };
    let a975 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a974, a975))
}

pub fn f270(v0: Adt13) -> R<List<(Bytes, Bytes)>> {
    let m976 = v0;
    match m976 {
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
        Adt13::C7(v9, r977, r978) => {
            let v10 = (*r977).clone();
            let v11 = (*r978).clone();
            Ok(List::cons((v9, f160(v10)?), f270(v11)?))
        }
    }
}

pub fn f271() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f272(v0: Adt12) -> Bytes {
    let m979 = v0;
    match m979 {
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

pub fn f273() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f274(v0: Adt12, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f363(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f363(f8(Str::lit("id")), f58(v0.clone(), f50(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f363(f8(Str::lit("name")), f364(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f363(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f363(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f363(f8(Str::lit("statement")), f365(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f363(f8(Str::lit("media_type")), f366(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f363(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f363(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f363(f8(Str::lit("url")), f163(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f275(v0: Bytes) -> R<u64> {
    let (h980, _) = f367(Fn6::F368(v0.clone()), {
        let a981 = v0.clone();
        length_bytes(a981)
    }, {
        let a982 = v0;
        length_bytes(a982)
    })?;
    Ok(h980)
}

pub fn f276(v0: Bytes, v1: u64) -> R<u64> {
    let (h983, _) = f367(Fn6::F369(v1, v0), v1, 0u64)?;
    Ok(h983)
}

pub fn f277(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m984 = v0;
    match m984.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m985 = f79(v1, f8(Str::lit("std")));
            match m985 {
                None => {
                    f277(v2)
                }
                Some(v3) => {
                    if f182(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f47(v3.clone(), f8(Str::lit("row"))), f47(v3, f8(Str::lit("authority")))), f277(v2))
                    } else {
                        f277(v2)
                    }
                }
            }
        }
    }
}

pub fn f278(v0: Adt14, v1: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m986 = v1;
    match m986.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f47(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f47(v2, f8(Str::lit("object")));
            if if f41(v4.clone(), match v0.clone() {
                Adt14::C0(h987, _, _) => {
                    h987
                }
            }) {
                f41(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h988, _) => {
                        h988
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f278(v0, v3))
            } else {
                if if f41(v5.clone(), match v0.clone() {
                    Adt14::C0(h989, _, _) => {
                        h989
                    }
                }) {
                    f41(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h990, _) => {
                            h990
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f278(v0, v3))
                } else {
                    f278(v0, v3)
                }
            }
        }
    }
}

pub fn f279(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt18>> {
    let m991 = v0;
    match m991.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v4, v5)) => {
            if if f370(v4.clone(), v2.clone()) {
                true
            } else {
                f370(v4.clone(), v3.clone())
            } {
                f279(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt18::C0({
                    let (h992, _) = v4.clone();
                    h992
                }, {
                    let a995 = {
                        let a994 = f118(f371({
                            let (h993, _) = v4.clone();
                            h993
                        }, v1.clone()))?;
                        length_list(a994)
                    };
                    let a996 = 1u64;
                    if nat_eq(a995, a996) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h997) = v4.clone();
                    h997
                }), f279(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f280(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m998 = v1;
    match m998.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41({
                let (h999, _) = v2.clone();
                h999
            }, v0.clone()) {
                List::cons({
                    let (_, h1000) = v2;
                    h1000
                }, f280(v0, v3))
            } else {
                if f41({
                    let (_, h1001) = v2;
                    h1001
                }, v0.clone()) {
                    List::cons({
                        let (h1002, _) = v2;
                        h1002
                    }, f280(v0, v3))
                } else {
                    f280(v0, v3)
                }
            }
        }
    }
}

pub fn f281(v0: Adt14, v1: Adt13, v2: Bytes) -> bool {
    if f41(v2.clone(), match v0.clone() {
        Adt14::C0(h1003, _, _) => {
            h1003
        }
    }) {
        true
    } else {
        if f41(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1004, _) => {
                h1004
            }
        }) {
            true
        } else {
            if f41(v2, match v0.clone() {
                Adt14::C0(_, _, h1005) => {
                    h1005
                }
            }) {
                true
            } else {
                if f182(v1.clone(), f8(Str::lit("scheme"))) {
                    f41(f47(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1006, _) => {
                            h1006
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f282(v0: Adt13) -> bool {
    if f182(v0.clone(), f8(Str::lit("domain"))) {
        if f182(v0.clone(), f8(Str::lit("row"))) {
            if f41(f47(v0.clone(), f8(Str::lit("relation"))), f372()) {
                if f373(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f373(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f373(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f373(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f283(v0: Adt13) -> bool {
    if f182(v0.clone(), f8(Str::lit("subject"))) {
        if f182(v0.clone(), f8(Str::lit("object"))) {
            if f182(v0.clone(), f8(Str::lit("evidence"))) {
                f41(f47(v0, f8(Str::lit("relation"))), f374())
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

pub fn f284(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1007 = f154(v1)?;
    match m1007 {
        None => {
            Ok(f188(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f243(f244(f239(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1008 = f336(f239(v4.clone(), f8(Str::lit("construct"))));
                match m1008 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1009 = v5;
            match m1009.uncons() {
                None => {
                    Ok(f188(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1010 = v9;
                    match m1010.uncons() {
                        None => {
                            Ok(f188(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f40(f28(v8, f8(Str::lit("std")))) {
                                Ok(f188(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f40(f41(v10.clone(), v2)) {
                                    Ok(f188(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f228(f40(f28(v7.clone(), f8(Str::lit("none")))), f40(f41(v7.clone(), v3))) {
                                        Ok(f188(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1011 = f336(f239(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1011 {
                                            None => {
                                                Ok(f188(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1012 = f239(v4, f8(Str::lit("value")));
                                                match m1012 {
                                                    None => {
                                                        Ok(f188(v0, f8(Str::lit("a row has no value"))))
                                                    }
                                                    Some(_) => {
                                                        Ok(List::<Bytes>::nil())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f285(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1013 = v1;
    match m1013.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f375(v3.clone())
            } else {
                f40(f375(v3.clone()))
            } {
                List::cons(f202(v0.clone(), v3), f285(v0, v4, v2))
            } else {
                f285(v0, v4, v2)
            }
        }
    }
}

pub fn f286(p1016: List<Bytes>) -> R<Option<Bytes>> {
    let m1015 = p1016;
    {
        let mut m1015 = m1015;
        loop {
            let step = {
                let v0 = m1015;
                let m1014 = v0;
                match m1014.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f376(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f168(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1015 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1017 = v1;
    match m1017.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f98(f81(f53(v0.clone(), v2))?, f287(v0, v3)?))
        }
    }
}

pub fn f288(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1018 = v0.clone();
        length_bytes(a1018)
    };
    let v3: u64 = {
        let a1019 = v1.clone();
        length_bytes(a1019)
    };
    let v4: u64 = {
        let (h1020, _) = f367(Fn6::F377(v1.clone(), v3, v0.clone(), v2), {
            let a1021 = v2;
            let a1022 = 1u64;
            nat_add(a1021, a1022)?
        }, 0u64)?;
        h1020
    };
    if f378(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f289() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f290(v0: Adt15) -> R<Adt19> {
    Ok(Adt19::C0(f379(match v0.clone() {
        Adt15::C0(h1023, _, _, _, _) => {
            h1023
        }
    }, 0u64)?, f379(match v0.clone() {
        Adt15::C0(_, h1024, _, _, _) => {
            h1024
        }
    }, 0u64)?, f379(match v0.clone() {
        Adt15::C0(_, _, h1025, _, _) => {
            h1025
        }
    }, 0u64)?, f379(match v0 {
        Adt15::C0(_, _, _, h1026, _) => {
            h1026
        }
    }, 0u64)?))
}

pub fn f291(v0: Adt15) -> List<Bytes> {
    f98(match v0.clone() {
        Adt15::C0(h1027, _, _, _, _) => {
            h1027
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, h1028, _, _) => {
            h1028
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, _, h1029, _) => {
            h1029
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1030) => {
            h1030
        }
    })))
}

pub fn f292(v0: Adt19, p1036: List<u64>, p1037: List<Adt21>) -> Option<u64> {
    let m1035 = (p1036, p1037);
    {
        let mut m1035 = m1035;
        loop {
            let step = {
                let (v1, v2) = m1035;
                let m1031 = v2;
                match m1031.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt21>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f380(v0.clone(), v1.clone(), match v3.clone() {
                            Adt21::C0(_, h1032) => {
                                h1032
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt21>), Option<u64>>((List::cons(match v3 {
                                Adt21::C0(h1033, _) => {
                                    h1033
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt21>), Option<u64>>(Some(match v3 {
                                Adt21::C0(h1034, _) => {
                                    h1034
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f293(v0: Adt15, v1: List<Adt13>, v2: u64) -> R<List<Adt21>> {
    let m1038 = v1;
    match m1038.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt21::C0(v2, f381(v0.clone(), v3)?), f293(v0, v4, {
                let a1039 = v2;
                let a1040 = 1u64;
                nat_add(a1039, a1040)?
            })?))
        }
    }
}

pub fn f294(p1044: List<Bytes>, p1045: u64) -> Bytes {
    let m1043 = (p1044, p1045);
    {
        let mut m1043 = m1043;
        loop {
            let step = {
                let (v0, v1) = m1043;
                let m1041 = v0;
                match m1041.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1042 = v1;
                        if m1042 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1042 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1043 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f382(v0.clone(), v1.clone()) {
        if f382(v1, v0) {
            f383(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f296(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1046 = v0;
    match m1046.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f384(v2, v1.clone())?, f296(v3, v1)?))
        }
    }
}

pub fn f297(v0: List<Adt13>) -> List<u64> {
    let m1047 = v0;
    match m1047.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f385(v1), f297(v2))
        }
    }
}

pub fn f298(p1050: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1049 = p1050;
    {
        let mut m1049 = m1049;
        loop {
            let step = {
                let v0 = m1049;
                let m1048 = v0;
                match m1048.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f386(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f299(p1053: List<Bytes>, p1054: List<Bytes>) -> Option<Bytes> {
    let m1052 = (p1053, p1054);
    {
        let mut m1052 = m1052;
        loop {
            let step = {
                let (v0, v1) = m1052;
                let m1051 = v0;
                match m1051.uncons() {
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
                Ok(next) => { m1052 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f301(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1055 = v0.clone();
    match m1055.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1056 = v2;
            let a1057 = v1.clone();
            if equal(a1056, a1057) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f302(v0: List<(Bytes, ())>) -> List<Bytes> {
    f198(f387(Fn7::F388, List::<Bytes>::nil(), v0))
}

pub fn f303(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f389(f390(Fn8::F391, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1062 = {
        let (h1058, _) = f392(Fn9::F393, {
            let a1060 = {
                let a1059 = v0;
                length_list(a1059)
            };
            let a1061 = 1u64;
            nat_add(a1060, a1061)?
        }, v1)?;
        h1058
    };
    match m1062.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f304(v0: List<Bytes>) -> List<(Bytes, ())> {
    f394(f395(Fn10::F396, List::<(Bytes, ())>::nil(), v0))
}

pub fn f305(v0: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1063 = f33(v1, 34u8)?;
    match m1063.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1064 = v3;
            match m1064.uncons() {
                None => {
                    f397(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f306(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f398(v0.clone(), 0u64, {
        let a1065 = v0;
        length_bytes(a1065)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f307(v0: Adt13, v1: Bytes) -> Bytes {
    if f28(f399(f79(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f308(v0: Adt1, v1: Bytes) -> R<Adt17> {
    let m1066 = f238(v0, v1)?;
    match m1066 {
        None => {
            Ok(Adt17::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f309(v0: Bytes) -> R<Bytes> {
    let a1067 = f8(Str::lit("Catalog."));
    let a1068 = f330(v0)?;
    Ok(append_bytes(a1067, a1068))
}

pub fn f310(v0: Adt17) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f330(f242(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f311(v0: List<Bytes>) -> List<Bytes> {
    let m1069 = v0;
    match m1069.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f201(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f202(f221(), v1), f311(v2))
            } else {
                f311(v2)
            }
        }
    }
}

pub fn f312(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1070 = v1;
    match m1070.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f98(f400(f401(f81(f53(v0.clone(), v2))?)?), f312(v0, v3)?))
        }
    }
}

pub fn f313(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1071 = v1;
    match m1071.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f402(v0.clone(), v3, v2.clone())?;
            let (h1072, _) = v5.clone();
            if h1072 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f313(v0, v4, v2)?;
                let (h1073, _) = v6.clone();
                if h1073 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1076 = {
                            let (_, h1074) = v5;
                            h1074
                        };
                        let a1077 = {
                            let (_, h1075) = v6;
                            h1075
                        };
                        append_bytes(a1076, a1077)
                    }))
                }
            }
        }
    }
}

pub fn f314(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m1078 = v1;
    match m1078.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f182(v2.clone(), f8(Str::lit("subject"))) {
                if f183(v0.clone(), v2.clone()) {
                    f314(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f47(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f314(v0, v3))
                }
            } else {
                if f323(v0.clone(), v2.clone()) {
                    f314(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f324(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f314(v0, v3))
                }
            }
        }
    }
}

pub fn f315(v0: Adt1) -> R<Bytes> {
    let a1080 = {
        let a1079 = f101(f102(f214(f211(f212(f53(v0.clone(), f174()))?)), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(f82(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1079)
    };
    let a1081 = 0u64;
    if nat_eq(a1080, a1081) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f316(v0: List<Adt13>) -> R<List<Bytes>> {
    let m1082 = v0;
    match m1082.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f95(f176(f158(f79(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f47(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f47(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f403(f81(f47(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f316(v2)?))
        }
    }
}

pub fn f317(v0: Bytes, p1140: u64, p1141: u64, p1142: Adt20) -> R<Adt20> {
    let m1139 = (p1140, (p1141, p1142));
    {
        let mut m1139 = m1139;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1139;
                let m1083 = v2;
                if m1083 == 0 {
                    Err::<(u64, (u64, Adt20)), R<Adt20>>(Ok(v3))
                } else {
                    let v4 = m1083 - 1;
                    let v5: u64 = f404(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt20::C0(_, h1084, _, _, _) => {
                            h1084
                        }
                    } {
                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                            let a1085 = v1;
                            let a1086 = 1u64;
                            nat_add(a1085, a1086)?
                        }, (v4, Adt20::C0(true, false, match v3.clone() {
                            Adt20::C0(_, _, h1087, _, _) => {
                                h1087
                            }
                        }, match v3.clone() {
                            Adt20::C0(_, _, _, h1088, _) => {
                                h1088
                            }
                        }, match v3 {
                            Adt20::C0(_, _, _, _, h1089) => {
                                h1089
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt20::C0(h1090, _, _, _, _) => {
                                h1090
                            }
                        } {
                            let a1091 = v5;
                            let a1092 = 34u64;
                            if nat_eq(a1091, a1092) {
                                let a1095 = f404(v0.clone(), {
                                    let a1093 = v1;
                                    let a1094 = 1u64;
                                    nat_add(a1093, a1094)?
                                })?;
                                let a1096 = 34u64;
                                if nat_eq(a1095, a1096) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1097 = v1;
                                        let a1098 = 1u64;
                                        nat_add(a1097, a1098)?
                                    }, (v4, Adt20::C0(true, true, f405(match v3.clone() {
                                        Adt20::C0(_, _, h1099, _, _) => {
                                            h1099
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt20::C0(_, _, _, h1100, _) => {
                                            h1100
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1101) => {
                                            h1101
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1102 = v1;
                                        let a1103 = 1u64;
                                        nat_add(a1102, a1103)?
                                    }, (v4, Adt20::C0(false, false, match v3.clone() {
                                        Adt20::C0(_, _, h1104, _, _) => {
                                            h1104
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1105, _) => {
                                            h1105
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1106) => {
                                            h1106
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1107 = v1;
                                    let a1108 = 1u64;
                                    nat_add(a1107, a1108)?
                                }, (v4, Adt20::C0(true, false, f405(match v3.clone() {
                                    Adt20::C0(_, _, h1109, _, _) => {
                                        h1109
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt20::C0(_, _, _, h1110, _) => {
                                        h1110
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1111) => {
                                        h1111
                                    }
                                }))))
                            }
                        } else {
                            let a1112 = v5;
                            let a1113 = 34u64;
                            if nat_eq(a1112, a1113) {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1114 = v1;
                                    let a1115 = 1u64;
                                    nat_add(a1114, a1115)?
                                }, (v4, Adt20::C0(true, false, match v3.clone() {
                                    Adt20::C0(_, _, h1116, _, _) => {
                                        h1116
                                    }
                                }, match v3.clone() {
                                    Adt20::C0(_, _, _, h1117, _) => {
                                        h1117
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1118) => {
                                        h1118
                                    }
                                }))))
                            } else {
                                let a1119 = v5;
                                let a1120 = 44u64;
                                if nat_eq(a1119, a1120) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1121 = v1;
                                        let a1122 = 1u64;
                                        nat_add(a1121, a1122)?
                                    }, (v4, Adt20::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt20::C0(_, _, h1123, _, _) => {
                                            h1123
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1124, _) => {
                                            h1124
                                        }
                                    }), match v3 {
                                        Adt20::C0(_, _, _, _, h1125) => {
                                            h1125
                                        }
                                    }))))
                                } else {
                                    let a1126 = v5;
                                    let a1127 = 13u64;
                                    if nat_eq(a1126, a1127) {
                                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                            let a1128 = v1;
                                            let a1129 = 1u64;
                                            nat_add(a1128, a1129)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1130 = v5;
                                        let a1131 = 10u64;
                                        if nat_eq(a1130, a1131) {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1132 = v1;
                                                let a1133 = 1u64;
                                                nat_add(a1132, a1133)?
                                            }, (v4, f318(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1134 = v1;
                                                let a1135 = 1u64;
                                                nat_add(a1134, a1135)?
                                            }, (v4, Adt20::C0(false, false, f405(match v3.clone() {
                                                Adt20::C0(_, _, h1136, _, _) => {
                                                    h1136
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt20::C0(_, _, _, h1137, _) => {
                                                    h1137
                                                }
                                            }, match v3 {
                                                Adt20::C0(_, _, _, _, h1138) => {
                                                    h1138
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
                Ok(next) => { m1139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(v0: Adt20) -> Adt20 {
    Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f406(List::cons(match v0.clone() {
        Adt20::C0(_, _, h1143, _, _) => {
            h1143
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1144, _) => {
            h1144
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt20::C0(_, _, _, _, h1145) => {
            h1145
        }
    }))
}

pub fn f319(p1148: List<List<Bytes>>, p1149: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1147 = (p1148, p1149);
    {
        let mut m1147 = m1147;
        loop {
            let step = {
                let (v0, v1) = m1147;
                let m1146 = v0;
                match m1146.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f320(v0: Bytes, v1: List<Adt18>) -> List<Bytes> {
    let m1150 = v1;
    match m1150.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(match v2.clone() {
                Adt18::C0(h1151, _, _) => {
                    h1151
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt18::C0(_, h1152, _) => {
                        h1152
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt18::C0(_, _, h1153) => {
                        h1153
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f320(v0, v3))
            } else {
                f320(v0, v3)
            }
        }
    }
}

pub fn f321(v0: Bytes, v1: List<Adt13>) -> List<Bytes> {
    let m1154 = v1;
    match m1154.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(f47(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f321(v0, v3))
            } else {
                if f28(f47(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f47(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f321(v0, v3))
                } else {
                    f321(v0, v3)
                }
            }
        }
    }
}

pub fn f322(p1159: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1158 = p1159;
    {
        let mut m1158 = m1158;
        loop {
            let step = {
                let v0 = m1158;
                let m1155 = v0;
                match m1155.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1156 = v2;
                        match m1156.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f28(v4, v1.clone()) {
                                    let m1157 = v5;
                                    match m1157.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f407(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f323(v0: Adt14, v1: Adt13) -> bool {
    if f41(f47(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1160, _, _) => {
            h1160
        }
    }) {
        f41(f324(v1), match v0 {
            Adt14::C0(_, h1161, _) => {
                h1161
            }
        })
    } else {
        false
    }
}

pub fn f324(v0: Adt13) -> Bytes {
    let m1162 = f79(v0, f8(Str::lit("evidence")));
    match m1162 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f47(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f325(v0: Adt13) -> Bytes {
    if f182(v0.clone(), f8(Str::lit("req"))) {
        let a1163 = f8(Str::lit("req "));
        let a1164 = f47(v0, f8(Str::lit("req")));
        append_bytes(a1163, a1164)
    } else {
        let m1165 = f79(v0.clone(), f8(Str::lit("std")));
        match m1165 {
            None => {
                let m1168 = f79(v0, f8(Str::lit("der")));
                match m1168 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f47(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f95(f176(f158(f79(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f47(v1.clone(), f8(Str::lit("authority"))), List::cons(if f182(v1.clone(), f8(Str::lit("row"))) {
                    let a1166 = f8(Str::lit(" row "));
                    let a1167 = f47(v1, f8(Str::lit("row")));
                    append_bytes(a1166, a1167)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f326(v0: Bytes, v1: Bytes) -> bool {
    let a1169 = v0;
    let a1170 = f342();
    if equal(a1169, a1170) {
        let a1174 = {
            let a1172 = {
                let a1171 = v1;
                length_bytes(a1171)
            };
            let a1173 = 0u64;
            nat_eq(a1172, a1173)
        };
        bool_not(a1174)
    } else {
        false
    }
}

pub fn f327(v0: Bytes) -> Bytes {
    f7(List::cons(f117(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f328(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1176 = v0;
    let a1177 = {
        let a1175 = Str::lit("redistributable");
        utf8_encode(a1175)
    };
    if equal(a1176, a1177) {
        f408(v1)
    } else {
        Ok(false)
    }
}

pub fn f329(v0: Adt17) -> bool {
    if f409(v0.clone(), f8(Str::lit("name"))) {
        if f410(v0.clone(), f8(Str::lit("schema"))) {
            let m1178 = f333(f243(f244(f239(v0, f8(Str::lit("citation"))))));
            match m1178 {
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

pub fn f330(v0: Bytes) -> R<Bytes> {
    f411(v0.clone(), 0u64, {
        let a1179 = v0;
        length_bytes(a1179)
    }, true, Bytes::lit(&[]))
}

pub fn f331(v0: Adt13) -> Bytes {
    if f182(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f182(v0.clone(), f8(Str::lit("id"))) {
            if f182(v0.clone(), f8(Str::lit("statement"))) {
                if f182(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f348(v0, f8(Str::lit("evidence")))
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

pub fn f332(v0: List<Bytes>) -> Adt13 {
    let m1180 = v0;
    match m1180.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(f332(v2)))
        }
    }
}

pub fn f333(v0: List<Bytes>) -> Option<Adt13> {
    let m1181 = v0;
    match m1181.uncons() {
        None => {
            None::<Adt13>
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("req"))) {
                let m1182 = v2;
                match m1182.uncons() {
                    None => {
                        None::<Adt13>
                    }
                    Some((v3, _)) => {
                        Some(Adt13::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5)))
                    }
                }
            } else {
                if f28(v1.clone(), f8(Str::lit("std"))) {
                    let m1183 = v2;
                    match m1183.uncons() {
                        None => {
                            None::<Adt13>
                        }
                        Some((v5, v6)) => {
                            Some(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v5)), std::rc::Rc::new({
                                let m1184 = v6;
                                match m1184.uncons() {
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
                        let m1185 = v2;
                        match m1185.uncons() {
                            None => {
                                None::<Adt13>
                            }
                            Some((v9, v10)) => {
                                let m1186 = v10;
                                match m1186.uncons() {
                                    None => {
                                        None::<Adt13>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt13::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt13::C2(v11)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f332(v12)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt13::C2(v9)), std::rc::Rc::new(Adt13::C5))))))))), std::rc::Rc::new(Adt13::C5)))
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

pub fn f334(v0: Adt1, v1: Bytes, v2: Adt17, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1187 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1187.uncons() {
        None => {
            let m1188 = f333(v4);
            match m1188 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1189 = f8(Str::lit("Catalog."));
                        let a1190 = f330(v3.clone())?;
                        append_bytes(a1189, a1190)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f330(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1191 = f412(v2, v9.clone(), f330(v3.clone())?)?;
                    match m1191 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f330(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f413(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f413(f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt13::C2(v10.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt13::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt13::C1(false, 0u64)), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1195 = f414(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f35(v10.clone(), 4u64, {
                                    let a1193 = {
                                        let a1192 = v10;
                                        length_bytes(a1192)
                                    };
                                    let a1194 = 4u64;
                                    nat_sub(a1193, a1194)
                                }))?;
                                let a1196 = f8(Str::lit("\u{a}"));
                                append_bytes(a1195, a1196)
                            }), List::cons(f237(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Err(v11) => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v11, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f335(v0: Adt17) -> R<Bytes> {
    let m1197 = v0;
    match m1197 {
        Adt17::C0 => {
            Ok(Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
        }
        Adt17::C1(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C2(v3) => {
            if v3 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt17::C3(v4, v5) => {
            f265(v4, v5)
        }
        Adt17::C4(v6, v7, v8) => {
            f415(v6, v7, v8)
        }
        Adt17::C5(v9) => {
            f416(v9)
        }
        Adt17::C6(r1198) => {
            let v10 = (*r1198).clone();
            Ok(f267(f417(v10)?))
        }
        Adt17::C7(r1199) => {
            let v11 = (*r1199).clone();
            f269(f418(v11)?)
        }
        Adt17::C8 => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C9(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C10(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f336(v0: Option<Adt17>) -> Option<Bytes> {
    let m1200 = v0;
    match m1200 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1201 = v1;
            match m1201 {
                Adt17::C0 => {
                    None::<Bytes>
                }
                Adt17::C1(_, _) => {
                    None::<Bytes>
                }
                Adt17::C2(_) => {
                    None::<Bytes>
                }
                Adt17::C3(_, _) => {
                    None::<Bytes>
                }
                Adt17::C4(_, _, _) => {
                    None::<Bytes>
                }
                Adt17::C5(v2) => {
                    Some(v2)
                }
                Adt17::C6(_) => {
                    None::<Bytes>
                }
                Adt17::C7(_) => {
                    None::<Bytes>
                }
                Adt17::C8 => {
                    None::<Bytes>
                }
                Adt17::C9(_, _) => {
                    None::<Bytes>
                }
                Adt17::C10(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f337(v0: Adt17) -> List<Adt17> {
    let m1202 = v0;
    match m1202 {
        Adt17::C0 => {
            List::<Adt17>::nil()
        }
        Adt17::C1(_, _) => {
            List::<Adt17>::nil()
        }
        Adt17::C2(_) => {
            List::<Adt17>::nil()
        }
        Adt17::C3(_, _) => {
            List::<Adt17>::nil()
        }
        Adt17::C4(_, _, _) => {
            List::<Adt17>::nil()
        }
        Adt17::C5(_) => {
            List::<Adt17>::nil()
        }
        Adt17::C6(r1203) => {
            let v3 = (*r1203).clone();
            f337(v3)
        }
        Adt17::C7(_) => {
            List::<Adt17>::nil()
        }
        Adt17::C8 => {
            List::<Adt17>::nil()
        }
        Adt17::C9(r1204, r1205) => {
            let v1 = (*r1204).clone();
            let v2 = (*r1205).clone();
            List::cons(v1, f337(v2))
        }
        Adt17::C10(_, _, _) => {
            List::<Adt17>::nil()
        }
    }
}

pub fn f338(v0: Adt24) -> Bytes {
    let a1226 = f419(match v0.clone() {
        Adt24::C0(h1206, _, _, _, _, _, _, _) => {
            h1206
        }
    });
    let a1227 = {
        let a1224 = f419(match v0.clone() {
            Adt24::C0(_, h1207, _, _, _, _, _, _) => {
                h1207
            }
        });
        let a1225 = {
            let a1222 = f419(match v0.clone() {
                Adt24::C0(_, _, h1208, _, _, _, _, _) => {
                    h1208
                }
            });
            let a1223 = {
                let a1220 = f419(match v0.clone() {
                    Adt24::C0(_, _, _, h1209, _, _, _, _) => {
                        h1209
                    }
                });
                let a1221 = {
                    let a1218 = f419(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1210, _, _, _) => {
                            h1210
                        }
                    });
                    let a1219 = {
                        let a1216 = f419(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1211, _, _) => {
                                h1211
                            }
                        });
                        let a1217 = {
                            let a1214 = f419(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1212, _) => {
                                    h1212
                                }
                            });
                            let a1215 = f419(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1213) => {
                                    h1213
                                }
                            });
                            append_bytes(a1214, a1215)
                        };
                        append_bytes(a1216, a1217)
                    };
                    append_bytes(a1218, a1219)
                };
                append_bytes(a1220, a1221)
            };
            append_bytes(a1222, a1223)
        };
        append_bytes(a1224, a1225)
    };
    append_bytes(a1226, a1227)
}

pub fn f339(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1228 = v0.clone();
        length_bytes(a1228)
    };
    let v2: u64 = {
        let a1234 = {
            let a1231 = {
                let a1229 = v1;
                let a1230 = 72u64;
                nat_add(a1229, a1230)?
            };
            let a1232 = 64u64;
            let a1233 = 0u64;
            nat_quot(a1231, a1232, a1233)
        };
        let a1235 = 64u64;
        nat_mul(a1234, a1235)?
    };
    let (_, h1236) = f420(Fn11::F421(v0, v1, v2), {
        let a1237 = v2;
        let a1238 = 64u64;
        let a1239 = 0u64;
        nat_quot(a1237, a1238, a1239)
    }, (0u64, f422()))?;
    Ok(h1236)
}

pub fn f340() -> Bytes {
    f8(Str::lit("target/release/verdant_readers"))
}

pub fn f341(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>) -> R<Adt11> {
    let m1240 = f238(v0.clone(), v2.clone())?;
    match m1240 {
        None => {
            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
        }
        Some(v6) => {
            if f28(v4.clone(), f8(Str::lit("none"))) {
                f423(v0, v1, v2, v3, v4, v5, v6)
            } else {
                let v7: Bytes = {
                    let a1241 = f8(Str::lit("Catalog."));
                    let a1242 = v4.clone();
                    append_bytes(a1241, a1242)
                };
                if f40(f424(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, v7.clone(), v3.clone())) {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v3, List::cons(f8(Str::lit(" in model/admissions.json, so it cannot check the entries\u{a}")), List::<Bytes>::nil()))))))))
                } else {
                    let m1243 = f238(v0.clone(), f7(List::cons(f8(Str::lit("encodings/")), List::cons(v4.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))?;
                    match m1243 {
                        None => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: encodings/")), List::cons(v4, List::cons(f8(Str::lit(".schema.json is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v8) => {
                            let m1244 = f425(v8, v6.clone())?;
                            match m1244 {
                                None => {
                                    f423(v0, v1, v2, v3, {
                                        let a1245 = f8(Str::lit("Catalog."));
                                        let a1246 = v4;
                                        append_bytes(a1245, a1246)
                                    }, v5, v6)
                                }
                                Some(v9) => {
                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" does not conform to ")), List::cons(v7, List::cons(f8(Str::lit(": ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f342() -> Bytes {
    let a1247 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1247)
}

pub fn f343(v0: Adt13, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt13> = f79(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f252(Str::lit("id"), f47(v0.clone(), f8(Str::lit("id"))))?, List::cons(f252(Str::lit("name"), f47(v0.clone(), f8(Str::lit("name"))))?, List::cons(f252(Str::lit("issuer"), f47(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f252(Str::lit("edition"), f47(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f252(Str::lit("canonical_id"), f47(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f252(Str::lit("source_role"), f47(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f252(Str::lit("retrieval_date"), f47(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f252(Str::lit("url"), f399(v2.clone(), f8(Str::lit("url"))))?, List::cons(f252(Str::lit("media_type"), f399(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f252(Str::lit("license"), f399(v2.clone(), f8(Str::lit("license"))))?, List::cons(f252(Str::lit("redistribution"), f399(v2, f8(Str::lit("redistribution"))))?, List::cons(f252(Str::lit("statement"), f47(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1248 = v1;
        match m1248 {
            None => {
                f7(List::cons(f252(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f252(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f252(Str::lit("artifact"), {
                    let (h1249, _) = v3;
                    h1249
                })?, List::cons(f252(Str::lit("checksum"), {
                    let (_, h1250) = v3;
                    h1250
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f426(f176(f158(f79(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f344(p1253: List<List<Bytes>>, v1: Bytes, p1254: List<Bytes>) -> R<List<Bytes>> {
    let m1252 = (p1253, p1254);
    {
        let mut m1252 = m1252;
        loop {
            let step = {
                let (v0, v2) = m1252;
                let m1251 = v0;
                match m1251.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f427(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f346(f428(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
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

pub fn f345(v0: Bytes) -> R<Bytes> {
    Ok(f35(v0.clone(), 0u64, f275(v0)?))
}

pub fn f346(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1255 = v0;
    match m1255.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1256 = v3.clone();
            match m1256.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1259 = v2;
                    let a1260 = {
                        let a1257 = v1.clone();
                        let a1258 = f346(v3, v1);
                        append_bytes(a1257, a1258)
                    };
                    append_bytes(a1259, a1260)
                }
            }
        }
    }
}

pub fn f347(v0: Adt13, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1261 = f79(v0, v1);
    match m1261 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v3) => {
            f158(f79(v3, v2))
        }
    }
}

pub fn f348(v0: Adt13, v1: Bytes) -> bool {
    let m1262 = f79(v0, v1);
    match m1262 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f349(v0: Bytes, p1290: u64, p1291: u64, p1292: bool, p1293: bool, p1294: u64, p1295: Bytes) -> R<Bytes> {
    let m1289 = (p1290, (p1291, (p1292, (p1293, (p1294, p1295)))));
    {
        let mut m1289 = m1289;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m1289;
                let m1263 = v2;
                if m1263 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f429(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m1263 - 1;
                    let m1266 = {
                        let a1264 = v0.clone();
                        let a1265 = v1;
                        index_bytes(a1264, a1265)
                    };
                    match m1266 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f429(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f62(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a1267 = v1;
                                    let a1268 = 1u64;
                                    nat_add(a1267, a1268)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a1269 = v9;
                                    let a1270 = 34u64;
                                    !nat_eq(a1269, a1270)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1271 = v9;
                                    let a1272 = 92u64;
                                    nat_eq(a1271, a1272)
                                }, (v5, v6))))))
                            } else {
                                let a1273 = v9;
                                let a1274 = 32u64;
                                if if if if nat_eq(a1273, a1274) {
                                    true
                                } else {
                                    let a1275 = v9;
                                    let a1276 = 9u64;
                                    nat_eq(a1275, a1276)
                                } {
                                    true
                                } else {
                                    let a1277 = v9;
                                    let a1278 = 10u64;
                                    nat_eq(a1277, a1278)
                                } {
                                    true
                                } else {
                                    let a1279 = v9;
                                    let a1280 = 13u64;
                                    nat_eq(a1279, a1280)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1281 = v1;
                                        let a1282 = 1u64;
                                        nat_add(a1281, a1282)?
                                    }, (v7, (false, (false, ({
                                        let a1283 = v1;
                                        let a1284 = 1u64;
                                        nat_add(a1283, a1284)?
                                    }, f429(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1285 = v1;
                                        let a1286 = 1u64;
                                        nat_add(a1285, a1286)?
                                    }, (v7, ({
                                        let a1287 = v9;
                                        let a1288 = 34u64;
                                        nat_eq(a1287, a1288)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Bytes, v1: u64) -> R<u64> {
    let m1298 = {
        let a1296 = v0;
        let a1297 = v1;
        index_bytes(a1296, a1297)
    };
    match m1298 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1343 = {
                let a1319 = {
                    let a1307 = {
                        let a1301 = {
                            let a1299 = v3;
                            let a1300 = 128u8;
                            fixed_u8::bit_and(a1299, a1300)
                        };
                        let a1302 = 128u8;
                        if equal(a1301, a1302) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1308 = {
                        let a1305 = {
                            let a1303 = v3;
                            let a1304 = 64u8;
                            fixed_u8::bit_and(a1303, a1304)
                        };
                        let a1306 = 64u8;
                        if equal(a1305, a1306) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1307, a1308)?
                };
                let a1320 = {
                    let a1317 = {
                        let a1311 = {
                            let a1309 = v3;
                            let a1310 = 32u8;
                            fixed_u8::bit_and(a1309, a1310)
                        };
                        let a1312 = 32u8;
                        if equal(a1311, a1312) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1318 = {
                        let a1315 = {
                            let a1313 = v3;
                            let a1314 = 16u8;
                            fixed_u8::bit_and(a1313, a1314)
                        };
                        let a1316 = 16u8;
                        if equal(a1315, a1316) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1317, a1318)?
                };
                nat_add(a1319, a1320)?
            };
            let a1344 = {
                let a1341 = {
                    let a1329 = {
                        let a1323 = {
                            let a1321 = v3;
                            let a1322 = 8u8;
                            fixed_u8::bit_and(a1321, a1322)
                        };
                        let a1324 = 8u8;
                        if equal(a1323, a1324) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1330 = {
                        let a1327 = {
                            let a1325 = v3;
                            let a1326 = 4u8;
                            fixed_u8::bit_and(a1325, a1326)
                        };
                        let a1328 = 4u8;
                        if equal(a1327, a1328) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1329, a1330)?
                };
                let a1342 = {
                    let a1339 = {
                        let a1333 = {
                            let a1331 = v3;
                            let a1332 = 2u8;
                            fixed_u8::bit_and(a1331, a1332)
                        };
                        let a1334 = 2u8;
                        if equal(a1333, a1334) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1340 = {
                        let a1337 = {
                            let a1335 = v3;
                            let a1336 = 1u8;
                            fixed_u8::bit_and(a1335, a1336)
                        };
                        let a1338 = 1u8;
                        if equal(a1337, a1338) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1339, a1340)?
                };
                nat_add(a1341, a1342)?
            };
            nat_add(a1343, a1344)
        }
    }
}

pub fn f351(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f430(v0.clone(), v1, {
        let a1346 = {
            let a1345 = v0.clone();
            length_bytes(a1345)
        };
        let a1347 = 1u64;
        nat_add(a1346, a1347)?
    })?;
    let a1348 = f350(v0.clone(), v2)?;
    let a1349 = 34u64;
    if nat_eq(a1348, a1349) {
        Ok(Some(({
            let a1350 = v2;
            let a1351 = 1u64;
            nat_add(a1350, a1351)?
        }, f35(v0, v1, {
            let a1352 = v2;
            let a1353 = v1;
            nat_sub(a1352, a1353)
        }))))
    } else {
        f431(v0, v1)
    }
}

pub fn f352(v0: Bytes, v1: u64) -> R<Option<(u64, Adt17)>> {
    let v2: bool = {
        let a1354 = f350(v0.clone(), v1)?;
        let a1355 = 45u64;
        nat_eq(a1354, a1355)
    };
    let v3: u64 = if v2 {
        let a1356 = v1;
        let a1357 = 1u64;
        nat_add(a1356, a1357)?
    } else {
        v1
    };
    let v4: u64 = f432(v0.clone(), v3)?;
    let v5: u64 = {
        let a1358 = v4;
        let a1359 = v3;
        nat_sub(a1358, a1359)
    };
    let v6: bool = {
        let a1360 = f350(v0.clone(), v4)?;
        let a1361 = 46u64;
        nat_eq(a1360, a1361)
    };
    let v7: u64 = if v6 {
        let a1362 = v4;
        let a1363 = 1u64;
        nat_add(a1362, a1363)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f432(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f350(v0.clone(), v8)?;
    let v10: bool = {
        let a1364 = v9;
        let a1365 = 101u64;
        if nat_eq(a1364, a1365) {
            true
        } else {
            let a1366 = v9;
            let a1367 = 69u64;
            nat_eq(a1366, a1367)
        }
    };
    let v11: u64 = f350(v0.clone(), {
        let a1368 = v8;
        let a1369 = 1u64;
        nat_add(a1368, a1369)?
    })?;
    let v12: bool = f228(v10, f433({
        let a1370 = v11;
        let a1371 = 43u64;
        nat_eq(a1370, a1371)
    }, {
        let a1372 = v11;
        let a1373 = 45u64;
        nat_eq(a1372, a1373)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a1374 = v8;
            let a1375 = 2u64;
            nat_add(a1374, a1375)?
        } else {
            let a1376 = v8;
            let a1377 = 1u64;
            nat_add(a1376, a1377)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f432(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f35(v0.clone(), v13, {
        let a1378 = v14;
        let a1379 = v13;
        nat_sub(a1378, a1379)
    });
    let v16: u64 = f434(v15.clone())?;
    let v17: Bytes = f35(v15.clone(), v16, {
        let a1381 = {
            let a1380 = v15;
            length_bytes(a1380)
        };
        let a1382 = v16;
        nat_sub(a1381, a1382)
    });
    let a1383 = v5;
    let a1384 = 0u64;
    if if nat_eq(a1383, a1384) {
        true
    } else {
        if f228({
            let a1385 = 1u64;
            let a1386 = v5;
            nat_lt(a1385, a1386)
        }, {
            let a1387 = f350(v0.clone(), v3)?;
            let a1388 = 48u64;
            nat_eq(a1387, a1388)
        }) {
            true
        } else {
            if f228(v6, {
                let a1389 = v8;
                let a1390 = v7;
                nat_eq(a1389, a1390)
            }) {
                true
            } else {
                f433(if v10 {
                    let a1391 = v14;
                    let a1392 = v13;
                    nat_eq(a1391, a1392)
                } else {
                    false
                }, {
                    let a1394 = 18u64;
                    let a1395 = {
                        let a1393 = v17.clone();
                        length_bytes(a1393)
                    };
                    nat_lt(a1394, a1395)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt17)>)
    } else {
        let m1399 = {
            let a1397 = {
                let a1396 = v17.clone();
                length_bytes(a1396)
            };
            let a1398 = 0u64;
            if nat_eq(a1397, a1398) {
                Some(0u64)
            } else {
                f435(v17)?
            }
        };
        match m1399 {
            None => {
                Ok(None::<(u64, Adt17)>)
            }
            Some(v18) => {
                let m1404 = f436(f35(v0.clone(), v3, v5), f35(v0, v7, {
                    let a1400 = v8;
                    let a1401 = v7;
                    nat_sub(a1400, a1401)
                }), f228(v12, {
                    let a1402 = v11;
                    let a1403 = 45u64;
                    nat_eq(a1402, a1403)
                }), v18)?;
                match m1404 {
                    None => {
                        Ok(None::<(u64, Adt17)>)
                    }
                    Some(v19) => {
                        let a1407 = 19u64;
                        let a1408 = {
                            let a1406 = {
                                let (h1405, _) = v19.clone();
                                h1405
                            };
                            length_bytes(a1406)
                        };
                        if nat_lt(a1407, a1408) {
                            Ok(None::<(u64, Adt17)>)
                        } else {
                            let m1410 = f435({
                                let (h1409, _) = v19.clone();
                                h1409
                            })?;
                            match m1410 {
                                None => {
                                    Ok(None::<(u64, Adt17)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f228(v2, {
                                        let a1411 = 0u64;
                                        let a1412 = v20;
                                        nat_lt(a1411, a1412)
                                    });
                                    if f437(v21, v20) {
                                        Ok(Some((v14, {
                                            let a1414 = {
                                                let (_, h1413) = v19.clone();
                                                h1413
                                            };
                                            let a1415 = 0u64;
                                            if nat_eq(a1414, a1415) {
                                                Adt17::C3(v21, v20)
                                            } else {
                                                Adt17::C4(v21, v20, {
                                                    let (_, h1416) = v19;
                                                    h1416
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt17)>)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f353(v0: Bytes) -> Bytes {
    if f201(v0.clone(), Bytes::lit(&[0x0d])) {
        f35(v0.clone(), 0u64, {
            let a1418 = {
                let a1417 = v0;
                length_bytes(a1417)
            };
            let a1419 = 1u64;
            nat_sub(a1418, a1419)
        })
    } else {
        v0
    }
}

pub fn f354(v0: Adt13, v1: Bytes) -> Adt13 {
    let m1420 = v0;
    match m1420 {
        Adt13::C0(v3) => {
            Adt13::C0(v3)
        }
        Adt13::C1(v4, v5) => {
            Adt13::C1(v4, v5)
        }
        Adt13::C2(v2) => {
            Adt13::C2(f438(f49(), v2, v1))
        }
        Adt13::C3(r1421) => {
            let v6 = (*r1421).clone();
            Adt13::C3(std::rc::Rc::new(v6))
        }
        Adt13::C4(r1422) => {
            let v7 = (*r1422).clone();
            Adt13::C4(std::rc::Rc::new(v7))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r1423, r1424) => {
            let v8 = (*r1423).clone();
            let v9 = (*r1424).clone();
            Adt13::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt13::C7(v10, r1425, r1426) => {
            let v11 = (*r1425).clone();
            let v12 = (*r1426).clone();
            Adt13::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f355(v0: u64) -> R<Bytes> {
    f439(f440(), 10u64, v0)
}

pub fn f356(v0: Fn3, p1431: u64, p1432: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1430 = (p1431, p1432);
    {
        let mut m1430 = m1430;
        loop {
            let step = {
                let (v1, v2) = m1430;
                let m1427 = v1;
                if m1427 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1427 - 1;
                    let m1429 = {
                        let c1428: Fn3 = v0.clone();
                        c1428.apply(v2.clone())?
                    };
                    match m1429 {
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
                Ok(next) => { m1430 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f441(v0, v1)
}

pub fn f358(v0: Fn4, p1436: List<(Bytes, Bytes)>, p1437: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1435 = (p1436, p1437);
    {
        let mut m1435 = m1435;
        loop {
            let step = {
                let (v1, v2) = m1435;
                let m1433 = v2;
                match m1433.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1434: Fn4 = v0.clone();
                            c1434.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f442(v0, v1)
}

pub fn f360(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f443(v0)
}

pub fn f361(v0: Fn5, p1441: List<Bytes>, p1442: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1440 = (p1441, p1442);
    {
        let mut m1440 = m1440;
        loop {
            let step = {
                let (v1, v2) = m1440;
                let m1438 = v2;
                match m1438.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1439: Fn5 = v0.clone();
                            c1439.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f444(v1)?, v0))
}

pub fn f363(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f266(v0)?, List::cons(f8(Str::lit(":")), List::cons(f266(v1)?, List::<Bytes>::nil())))))
}

pub fn f364(v0: Adt12) -> Bytes {
    let m1443 = v0;
    match m1443 {
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

pub fn f365(v0: Adt12) -> Bytes {
    let m1444 = v0;
    match m1444 {
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

pub fn f366(v0: Adt12) -> Bytes {
    let m1445 = v0;
    match m1445 {
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

pub fn f367(v0: Fn6, p1450: u64, p1451: u64) -> R<(u64, bool)> {
    let m1449 = (p1450, p1451);
    {
        let mut m1449 = m1449;
        loop {
            let step = {
                let (v1, v2) = m1449;
                let m1446 = v1;
                if m1446 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1446 - 1;
                    let m1448 = {
                        let c1447: Fn6 = v0.clone();
                        c1447.apply(v2)?
                    };
                    match m1448 {
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
                Ok(next) => { m1449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(v0: Bytes, v1: u64) -> Option<u64> {
    let a1452 = 0u64;
    let a1453 = v1;
    if nat_lt(a1452, a1453) {
        if f445(v0, {
            let a1454 = v1;
            let a1455 = 1u64;
            nat_sub(a1454, a1455)
        }) {
            Some({
                let a1456 = v1;
                let a1457 = 1u64;
                nat_sub(a1456, a1457)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f369(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1458 = v2;
    let a1459 = v0;
    if nat_lt(a1458, a1459) {
        if f445(v1, v2) {
            Ok(Some({
                let a1460 = v2;
                let a1461 = 1u64;
                nat_add(a1460, a1461)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f370(v0: (Bytes, Bytes), p1464: List<(Bytes, Bytes)>) -> bool {
    let m1463 = p1464;
    {
        let mut m1463 = m1463;
        loop {
            let step = {
                let v1 = m1463;
                let m1462 = v1;
                match m1462.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f446(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1463 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1465 = v1;
    match m1465.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h1466, _) = v2;
                h1466
            }, v0.clone()) {
                List::cons({
                    let (_, h1467) = v2;
                    h1467
                }, f371(v0, v3))
            } else {
                f371(v0, v3)
            }
        }
    }
}

pub fn f372() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f373(v0: Adt13, v1: Bytes, v2: Bytes) -> bool {
    let m1468 = f79(v0, v1);
    match m1468 {
        None => {
            false
        }
        Some(v3) => {
            f182(v3, v2)
        }
    }
}

pub fn f374() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f375(v0: Bytes) -> bool {
    f201(v0, f8(Str::lit(".lex.tex")))
}

pub fn f376(v0: Bytes) -> R<bool> {
    let v1: Bytes = f168(v0)?;
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
        let a1470 = {
            let a1469 = v1;
            length_bytes(a1469)
        };
        let a1471 = v2;
        nat_sub(a1470, a1471)
    });
    let v5: Bytes = if f72(v3.clone(), f8(Str::lit("("))) {
        let m1472 = f288(v3.clone(), f8(Str::lit(")")))?;
        match m1472 {
            None => {
                v3
            }
            Some(v4) => {
                f35(v3.clone(), {
                    let a1473 = v4;
                    let a1474 = 1u64;
                    nat_add(a1473, a1474)?
                }, {
                    let a1478 = {
                        let a1475 = v3;
                        length_bytes(a1475)
                    };
                    let a1479 = {
                        let a1476 = v4;
                        let a1477 = 1u64;
                        nat_add(a1476, a1477)?
                    };
                    nat_sub(a1478, a1479)
                })
            }
        }
    } else {
        v3
    };
    let a1480 = 0u64;
    let a1481 = v2;
    if nat_lt(a1480, a1481) {
        f447(v5)
    } else {
        Ok(false)
    }
}

pub fn f377(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f378(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1484 = {
            let a1482 = v4;
            let a1483 = v1;
            nat_add(a1482, a1483)?
        };
        let a1485 = v3;
        if nat_lt(a1484, a1485) {
            Ok(Some({
                let a1486 = v4;
                let a1487 = 1u64;
                nat_add(a1486, a1487)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f378(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1492 = {
        let a1489 = v0;
        let a1490 = v2;
        let a1491 = {
            let a1488 = v1.clone();
            length_bytes(a1488)
        };
        slice_bytes(a1489, a1490, a1491)
    };
    match m1492 {
        None => {
            false
        }
        Some(v3) => {
            let a1493 = v3;
            let a1494 = v1;
            equal(a1493, a1494)
        }
    }
}

pub fn f379(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1495 = v0;
    match m1495.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f379(v3, {
                let a1496 = v1;
                let a1497 = 1u64;
                nat_add(a1496, a1497)?
            })?))
        }
    }
}

pub fn f380(v0: Adt19, v1: List<u64>, v2: Adt22) -> bool {
    let m1498 = v2;
    match m1498 {
        Adt22::C0(v3, v4) => {
            if f386(v3, match v0.clone() {
                Adt19::C0(h1499, _, _, _) => {
                    h1499
                }
            }) {
                f448(v0, v4)
            } else {
                false
            }
        }
        Adt22::C1(v5) => {
            f386(v5, match v0 {
                Adt19::C0(_, _, h1500, _) => {
                    h1500
                }
            })
        }
        Adt22::C2(_, v7, v8) => {
            if f382(v7, v1) {
                f386(v8, match v0 {
                    Adt19::C0(_, _, _, h1501) => {
                        h1501
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f381(v0: Adt15, v1: Adt13) -> R<Adt22> {
    let m1502 = f79(v1.clone(), f8(Str::lit("std")));
    match m1502 {
        None => {
            let m1507 = f78(f79(v1.clone(), f8(Str::lit("req"))));
            match m1507 {
                None => {
                    let m1509 = f79(v1, f8(Str::lit("der")));
                    match m1509 {
                        None => {
                            Ok(f449(v0))
                        }
                        Some(v5) => {
                            let m1510 = f450(f47(v5.clone(), f8(Str::lit("rule"))));
                            match m1510 {
                                None => {
                                    Ok(f449(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt22::C2(v6, f296(f176(f158(f79(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1511) => {
                                            h1511
                                        }
                                    })?, f384(f47(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1512, _) => {
                                            h1512
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt22::C1(f384(v4, match v0 {
                        Adt15::C0(_, _, h1508, _, _) => {
                            h1508
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1503 = f78(f79(v2.clone(), f8(Str::lit("row"))));
            match m1503 {
                None => {
                    Ok(Adt22::C0(f384(f47(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1504, _, _, _, _) => {
                            h1504
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt22::C0(f384(f47(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1505, _, _, _, _) => {
                            h1505
                        }
                    })?, Some(f384(v3, match v0 {
                        Adt15::C0(_, h1506, _, _, _) => {
                            h1506
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f382(p1515: List<u64>, v1: List<u64>) -> bool {
    let m1514 = p1515;
    {
        let mut m1514 = m1514;
        loop {
            let step = {
                let v0 = m1514;
                let m1513 = v0;
                match m1513.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f386(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1514 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(p1520: List<u64>) -> bool {
    let m1519 = p1520;
    {
        let mut m1519 = m1519;
        loop {
            let step = {
                let v0 = m1519;
                let m1516 = v0;
                match m1516.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1517 = v1;
                        let a1518 = 7u64;
                        if nat_le(a1517, a1518) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1521 = f451(v0, v1.clone(), 0u64)?;
    match m1521 {
        None => {
            let a1522 = v1;
            Ok(length_list(a1522))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f385(v0: Adt13) -> u64 {
    let m1523 = f452(f79(v0, f8(Str::lit("stratum"))));
    match m1523 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f386(v0: u64, p1528: List<u64>) -> bool {
    let m1527 = p1528;
    {
        let mut m1527 = m1527;
        loop {
            let step = {
                let v1 = m1527;
                let m1524 = v1;
                match m1524.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1525 = v2;
                        let a1526 = v0;
                        if nat_eq(a1525, a1526) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(v0: Fn7, p1532: List<Bytes>, p1533: List<(Bytes, ())>) -> List<Bytes> {
    let m1531 = (p1532, p1533);
    {
        let mut m1531 = m1531;
        loop {
            let step = {
                let (v1, v2) = m1531;
                let m1529 = v2;
                match m1529.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1530: Fn7 = v0.clone();
                            c1530.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1534, _) = v1;
        h1534
    }, v0)
}

pub fn f389(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f453(Fn12::F454, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f390(v0: Fn8, p1538: List<List<(Bytes, ())>>, p1539: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1537 = (p1538, p1539);
    {
        let mut m1537 = m1537;
        loop {
            let step = {
                let (v1, v2) = m1537;
                let m1535 = v2;
                match m1535.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1536: Fn8 = v0.clone();
                            c1536.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1537 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f392(v0: Fn9, p1544: u64, p1545: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1543 = (p1544, p1545);
    {
        let mut m1543 = m1543;
        loop {
            let step = {
                let (v1, v2) = m1543;
                let m1540 = v1;
                if m1540 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1540 - 1;
                    let m1542 = {
                        let c1541: Fn9 = v0.clone();
                        c1541.apply(v2.clone())?
                    };
                    match m1542 {
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
                Ok(next) => { m1543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f455(v0)
}

pub fn f394(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f456(Fn13::F457, List::<(Bytes, ())>::nil(), v0)
}

pub fn f395(v0: Fn10, p1549: List<(Bytes, ())>, p1550: List<Bytes>) -> List<(Bytes, ())> {
    let m1548 = (p1549, p1550);
    {
        let mut m1548 = m1548;
        loop {
            let step = {
                let (v1, v2) = m1548;
                let m1546 = v2;
                match m1546.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1547: Fn10 = v0.clone();
                            c1547.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1548 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f397(p1553: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1552 = p1553;
    {
        let mut m1552 = m1552;
        loop {
            let step = {
                let v0 = m1552;
                let m1551 = v0;
                match m1551.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f458(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: Bytes, p1579: u64, p1580: u64, p1581: Bytes) -> R<Bytes> {
    let m1578 = (p1579, (p1580, p1581));
    {
        let mut m1578 = m1578;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1578;
                let m1554 = v2;
                if m1554 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1554 - 1;
                    let m1557 = {
                        let a1555 = v0.clone();
                        let a1556 = v1;
                        index_bytes(a1555, a1556)
                    };
                    match m1557 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            if f433({
                                let a1558 = v6;
                                let a1559 = 34u64;
                                nat_eq(a1558, a1559)
                            }, {
                                let a1560 = v6;
                                let a1561 = 92u64;
                                nat_eq(a1560, a1561)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1562 = v1;
                                    let a1563 = 1u64;
                                    nat_add(a1562, a1563)?
                                }, (v4, {
                                    let a1566 = v3;
                                    let a1567 = {
                                        let a1564 = f8(Str::lit("\\"));
                                        let a1565 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1564, a1565)
                                    };
                                    append_bytes(a1566, a1567)
                                })))
                            } else {
                                let a1568 = v6;
                                let a1569 = 10u64;
                                if nat_eq(a1568, a1569) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1570 = v1;
                                        let a1571 = 1u64;
                                        nat_add(a1570, a1571)?
                                    }, (v4, {
                                        let a1572 = v3;
                                        let a1573 = f8(Str::lit("\\n"));
                                        append_bytes(a1572, a1573)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1574 = v1;
                                        let a1575 = 1u64;
                                        nat_add(a1574, a1575)?
                                    }, (v4, {
                                        let a1576 = v3;
                                        let a1577 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1576, a1577)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1578 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(v0: Option<Adt13>, v1: Bytes) -> Bytes {
    let m1582 = v0;
    match m1582 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f47(v2, v1)
        }
    }
}

pub fn f400(v0: List<Adt13>) -> List<Bytes> {
    let m1583 = v0;
    match m1583.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f459(f47(v1.clone(), f8(Str::lit("authority")))), List::cons(f245(f47(v1, f8(Str::lit("authority")))), f400(v2)))
        }
    }
}

pub fn f401(v0: List<Bytes>) -> R<List<Adt13>> {
    let m1584 = f255(v0.clone())?;
    match m1584 {
        None => {
            Ok(f231(f230(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt13>::nil())
        }
    }
}

pub fn f402(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
    let m1585 = f255(v3.clone())?;
    match m1585 {
        None => {
            let m1586 = f230(v3.clone())?;
            match m1586 {
                None => {
                    f462(v0, v1, v3)
                }
                Some(v5) => {
                    f461(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f460(v0, v1, v4)
        }
    }
}

pub fn f403(v0: List<Bytes>) -> List<Bytes> {
    let m1587 = v0;
    match m1587.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1589 = {
                    let a1588 = v1.clone();
                    length_bytes(a1588)
                };
                let a1590 = 0u64;
                if nat_eq(a1589, a1590) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f403(v2))
        }
    }
}

pub fn f404(v0: Bytes, v1: u64) -> R<u64> {
    let m1593 = {
        let a1591 = v0;
        let a1592 = v1;
        index_bytes(a1591, a1592)
    };
    match m1593 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f62(v2)
        }
    }
}

pub fn f405(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1597 = {
        let a1594 = v1;
        let a1595 = v2;
        let a1596 = 1u64;
        slice_bytes(a1594, a1595, a1596)
    };
    match m1597 {
        None => {
            v0
        }
        Some(v3) => {
            let a1598 = v0;
            let a1599 = v3;
            append_bytes(a1598, a1599)
        }
    }
}

pub fn f406(p1602: List<Bytes>, p1603: List<Bytes>) -> List<Bytes> {
    let m1601 = (p1602, p1603);
    {
        let mut m1601 = m1601;
        loop {
            let step = {
                let (v0, v1) = m1601;
                let m1600 = v0;
                match m1600.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: List<Bytes>) -> Bytes {
    let m1604 = v0;
    match m1604.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f408(v0: Bytes) -> R<bool> {
    let m1606 = {
        let a1605 = v0.clone();
        length_bytes(a1605)
    };
    if m1606 == 0 {
        Ok(true)
    } else {
        let v1 = m1606 - 1;
        let m1609 = {
            let a1607 = v0;
            let a1608 = v1;
            index_bytes(a1607, a1608)
        };
        match m1609 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f463({
                    let v3: u8 = v2;
                    let a1654 = {
                        let a1630 = {
                            let a1618 = {
                                let a1612 = {
                                    let a1610 = v3;
                                    let a1611 = 128u8;
                                    fixed_u8::bit_and(a1610, a1611)
                                };
                                let a1613 = 128u8;
                                if equal(a1612, a1613) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1619 = {
                                let a1616 = {
                                    let a1614 = v3;
                                    let a1615 = 64u8;
                                    fixed_u8::bit_and(a1614, a1615)
                                };
                                let a1617 = 64u8;
                                if equal(a1616, a1617) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1618, a1619)?
                        };
                        let a1631 = {
                            let a1628 = {
                                let a1622 = {
                                    let a1620 = v3;
                                    let a1621 = 32u8;
                                    fixed_u8::bit_and(a1620, a1621)
                                };
                                let a1623 = 32u8;
                                if equal(a1622, a1623) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1629 = {
                                let a1626 = {
                                    let a1624 = v3;
                                    let a1625 = 16u8;
                                    fixed_u8::bit_and(a1624, a1625)
                                };
                                let a1627 = 16u8;
                                if equal(a1626, a1627) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1628, a1629)?
                        };
                        nat_add(a1630, a1631)?
                    };
                    let a1655 = {
                        let a1652 = {
                            let a1640 = {
                                let a1634 = {
                                    let a1632 = v3;
                                    let a1633 = 8u8;
                                    fixed_u8::bit_and(a1632, a1633)
                                };
                                let a1635 = 8u8;
                                if equal(a1634, a1635) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1641 = {
                                let a1638 = {
                                    let a1636 = v3;
                                    let a1637 = 4u8;
                                    fixed_u8::bit_and(a1636, a1637)
                                };
                                let a1639 = 4u8;
                                if equal(a1638, a1639) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1640, a1641)?
                        };
                        let a1653 = {
                            let a1650 = {
                                let a1644 = {
                                    let a1642 = v3;
                                    let a1643 = 2u8;
                                    fixed_u8::bit_and(a1642, a1643)
                                };
                                let a1645 = 2u8;
                                if equal(a1644, a1645) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1651 = {
                                let a1648 = {
                                    let a1646 = v3;
                                    let a1647 = 1u8;
                                    fixed_u8::bit_and(a1646, a1647)
                                };
                                let a1649 = 1u8;
                                if equal(a1648, a1649) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1650, a1651)?
                        };
                        nat_add(a1652, a1653)?
                    };
                    nat_add(a1654, a1655)?
                }))
            }
        }
    }
}

pub fn f409(v0: Adt17, v1: Bytes) -> bool {
    let a1657 = {
        let a1656 = f242(v0, v1);
        length_bytes(a1656)
    };
    let a1658 = 0u64;
    !nat_eq(a1657, a1658)
}

pub fn f410(v0: Adt17, v1: Bytes) -> bool {
    let m1659 = f239(v0, v1);
    match m1659 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f411(v0: Bytes, p1680: u64, p1681: u64, p1682: bool, p1683: Bytes) -> R<Bytes> {
    let m1679 = (p1680, (p1681, (p1682, p1683)));
    {
        let mut m1679 = m1679;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1679;
                let m1660 = v2;
                if m1660 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1660 - 1;
                    let m1663 = {
                        let a1661 = v0.clone();
                        let a1662 = v1;
                        index_bytes(a1661, a1662)
                    };
                    match m1663 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f62(v6)?;
                            if f464(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1664 = v1;
                                    let a1665 = 1u64;
                                    nat_add(a1664, a1665)?
                                }, (v5, (true, v4))))
                            } else {
                                if f228(f465(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1666 = v1;
                                        let a1667 = 1u64;
                                        nat_add(a1666, a1667)?
                                    }, (v5, (false, {
                                        let a1671 = v4;
                                        let a1672 = f35({
                                            let a1668 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1668)
                                        }, {
                                            let a1669 = v7;
                                            let a1670 = 97u64;
                                            nat_sub(a1669, a1670)
                                        }, 1u64);
                                        append_bytes(a1671, a1672)
                                    }))))
                                } else {
                                    if f433(f465(v7), f433(f466(v7), f467(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1673 = v1;
                                            let a1674 = 1u64;
                                            nat_add(a1673, a1674)?
                                        }, (v5, (false, {
                                            let a1675 = v4;
                                            let a1676 = f35(v0.clone(), v1, 1u64);
                                            append_bytes(a1675, a1676)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1677 = v1;
                                            let a1678 = 1u64;
                                            nat_add(a1677, a1678)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: Adt17, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt17)> = f468(f469(f239(v0.clone(), f8(Str::lit("$defs")))), f469(f239(v0.clone(), f8(Str::lit("definitions")))));
    let m1684 = f470(4096u64, v3.clone(), List::<Bytes>::nil(), f471(v3)?)?;
    match m1684 {
        Ok(v5) => {
            let m1685 = f472(4096u64, f473(f474(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1685 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f346(f204(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f413(v0: List<Adt13>, v1: Bytes, v2: Bytes, v3: Adt13) -> List<Adt13> {
    let m1686 = v0;
    match m1686.uncons() {
        None => {
            List::cons(v3, List::<Adt13>::nil())
        }
        Some((v4, v5)) => {
            if f28(f47(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f475(v5, v1, v2))
            } else {
                List::cons(v4, f413(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f414(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f346(f476(f81(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f415(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f355(v1)?;
    let v4: u64 = {
        let a1687 = v3.clone();
        length_bytes(a1687)
    };
    let v5: Bytes = {
        let a1688 = v2;
        let a1689 = v4;
        if nat_lt(a1688, a1689) {
            f7(List::cons(f35(v3.clone(), 0u64, {
                let a1690 = v4;
                let a1691 = v2;
                nat_sub(a1690, a1691)
            }), List::cons(f8(Str::lit(".")), List::cons(f35(v3, {
                let a1692 = v4;
                let a1693 = v2;
                nat_sub(a1692, a1693)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f477({
                let a1694 = v2;
                let a1695 = v4;
                nat_sub(a1694, a1695)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1696 = f8(Str::lit("-"));
        let a1697 = v5;
        Ok(append_bytes(a1696, a1697))
    } else {
        Ok(v5)
    }
}

pub fn f416(v0: Bytes) -> R<Bytes> {
    let a1702 = f430(v0.clone(), 0u64, {
        let a1699 = {
            let a1698 = v0.clone();
            length_bytes(a1698)
        };
        let a1700 = 1u64;
        nat_add(a1699, a1700)?
    })?;
    let a1703 = {
        let a1701 = v0.clone();
        length_bytes(a1701)
    };
    if nat_eq(a1702, a1703) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f266(v0)
    }
}

pub fn f417(v0: Adt17) -> R<List<Bytes>> {
    let m1704 = v0;
    match m1704 {
        Adt17::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C3(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C4(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C8 => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C9(r1705, r1706) => {
            let v12 = (*r1705).clone();
            let v13 = (*r1706).clone();
            Ok(List::cons(f335(v12)?, f417(v13)?))
        }
        Adt17::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f418(v0: Adt17) -> R<List<(Bytes, Bytes)>> {
    let m1707 = v0;
    match m1707 {
        Adt17::C0 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C3(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C4(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C8 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C9(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C10(v14, r1708, r1709) => {
            let v15 = (*r1708).clone();
            let v16 = (*r1709).clone();
            Ok(List::cons((v14, f335(v15)?), f418(v16)?))
        }
    }
}

pub fn f419(v0: u32) -> Bytes {
    let a1738 = f478({
        let a1710 = f479(v0, 28u32);
        let a1711 = 15u32;
        fixed_u32::bit_and(a1710, a1711)
    });
    let a1739 = {
        let a1736 = f478({
            let a1712 = f479(v0, 24u32);
            let a1713 = 15u32;
            fixed_u32::bit_and(a1712, a1713)
        });
        let a1737 = {
            let a1734 = f478({
                let a1714 = f479(v0, 20u32);
                let a1715 = 15u32;
                fixed_u32::bit_and(a1714, a1715)
            });
            let a1735 = {
                let a1732 = f478({
                    let a1716 = f479(v0, 16u32);
                    let a1717 = 15u32;
                    fixed_u32::bit_and(a1716, a1717)
                });
                let a1733 = {
                    let a1730 = f478({
                        let a1718 = f479(v0, 12u32);
                        let a1719 = 15u32;
                        fixed_u32::bit_and(a1718, a1719)
                    });
                    let a1731 = {
                        let a1728 = f478({
                            let a1720 = f479(v0, 8u32);
                            let a1721 = 15u32;
                            fixed_u32::bit_and(a1720, a1721)
                        });
                        let a1729 = {
                            let a1726 = f478({
                                let a1722 = f479(v0, 4u32);
                                let a1723 = 15u32;
                                fixed_u32::bit_and(a1722, a1723)
                            });
                            let a1727 = f478({
                                let a1724 = v0;
                                let a1725 = 15u32;
                                fixed_u32::bit_and(a1724, a1725)
                            });
                            append_bytes(a1726, a1727)
                        };
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
}

pub fn f420(v0: Fn11, p1743: u64, p1744: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1742 = (p1743, p1744);
    {
        let mut m1742 = m1742;
        loop {
            let step = {
                let (v1, v2) = m1742;
                let m1740 = v1;
                if m1740 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1740 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1741: Fn11 = v0.clone();
                        c1741.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1742 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1746 = {
            let (h1745, _) = v3.clone();
            h1745
        };
        let a1747 = 64u64;
        nat_add(a1746, a1747)?
    }, f480({
        let (_, h1748) = v3;
        h1748
    }, f481(v0, v1, v2, {
        let (h1749, _) = v3;
        h1749
    })?)?))
}

pub fn f422() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f423(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Adt17) -> R<Adt11> {
    let m1750 = f482(f483(v1.clone(), v3.clone(), v4, v6))?;
    match m1750 {
        Ok(v8) => {
            let v9: Bytes = f459(v1.clone());
            Ok(Adt11::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f413(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v9.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("row")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5))))))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("translation")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("command")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f332(v5)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt13::C2(f246(f53(v0, v2))?)), std::rc::Rc::new(Adt13::C5))))))), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f424(p1754: List<Adt13>, v1: Bytes, v2: Bytes) -> bool {
    let m1753 = p1754;
    {
        let mut m1753 = m1753;
        loop {
            let step = {
                let v0 = m1753;
                let m1751 = v0;
                match m1751.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f228(f28(f47(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m1752 = f79(v3, f8(Str::lit("std")));
                            match m1752 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    f28(f47(v5, f8(Str::lit("row"))), v2.clone())
                                }
                            }
                        }) {
                            Err::<List<Adt13>, bool>(true)
                        } else {
                            Ok::<List<Adt13>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1753 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(v0: Adt17, v1: Adt17) -> R<Option<Bytes>> {
    f484({
        let a1761 = {
            let a1756 = {
                let a1755 = f485(v0.clone())?;
                length_bytes(a1755)
            };
            let a1757 = 2u64;
            nat_add(a1756, a1757)?
        };
        let a1762 = {
            let a1759 = {
                let a1758 = f485(v1.clone())?;
                length_bytes(a1758)
            };
            let a1760 = 2u64;
            nat_add(a1759, a1760)?
        };
        nat_mul(a1761, a1762)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f426(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f346(f486(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f427(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1763 = f33(v1, 34u8)?;
    match m1763.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1764 = v3;
            match m1764.uncons() {
                None => {
                    Ok(f487(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f428(v0: List<Bytes>) -> List<Bytes> {
    let m1765 = v0;
    match m1765.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("authority = []"))) {
                f428(v2)
            } else {
                List::cons(v1, f428(v2))
            }
        }
    }
}

pub fn f429(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1768 = v3;
    let a1769 = f35(v0, v1, {
        let a1766 = v2;
        let a1767 = v1;
        nat_sub(a1766, a1767)
    });
    append_bytes(a1768, a1769)
}

pub fn f430(v0: Bytes, p1782: u64, p1783: u64) -> R<u64> {
    let m1781 = (p1782, p1783);
    {
        let mut m1781 = m1781;
        loop {
            let step = {
                let (v1, v2) = m1781;
                let m1770 = v2;
                if m1770 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1770 - 1;
                    let v4: u64 = f350(v0.clone(), v1)?;
                    let a1771 = v4;
                    let a1772 = 34u64;
                    if nat_eq(a1771, a1772) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1773 = v4;
                        let a1774 = 92u64;
                        if nat_eq(a1773, a1774) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1775 = v4;
                            let a1776 = 32u64;
                            if nat_lt(a1775, a1776) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1777 = v4;
                                let a1778 = 256u64;
                                if nat_eq(a1777, a1778) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1779 = v1;
                                        let a1780 = 1u64;
                                        nat_add(a1779, a1780)?
                                    }, v3))
                                }
                            }
                        }
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

pub fn f431(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f488(Fn14::F489(v0.clone()), {
        let a1785 = {
            let a1784 = v0;
            length_bytes(a1784)
        };
        let a1786 = 1u64;
        nat_add(a1785, a1786)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1790 = {
        let (_, h1787) = {
            let (_, h1788) = {
                let (h1789, _) = v2.clone();
                h1789
            };
            h1788
        };
        h1787
    };
    let a1791 = 1u64;
    if nat_eq(a1790, a1791) {
        Ok(Some(({
            let (h1792, _) = {
                let (h1793, _) = v2.clone();
                h1793
            };
            h1792
        }, {
            let (h1794, _) = {
                let (_, h1795) = {
                    let (h1796, _) = v2;
                    h1796
                };
                h1795
            };
            h1794
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f432(v0: Bytes, v1: u64) -> R<u64> {
    let (h1797, _) = f367(Fn6::F490(v0.clone()), {
        let a1799 = {
            let a1798 = v0;
            length_bytes(a1798)
        };
        let a1800 = 1u64;
        nat_add(a1799, a1800)?
    }, v1)?;
    Ok(h1797)
}

pub fn f433(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f434(v0: Bytes) -> R<u64> {
    let (h1801, _) = f367(Fn6::F491(v0.clone()), {
        let a1803 = {
            let a1802 = v0;
            length_bytes(a1802)
        };
        let a1804 = 1u64;
        nat_add(a1803, a1804)?
    }, 0u64)?;
    Ok(h1801)
}

pub fn f435(v0: Bytes) -> R<Option<u64>> {
    if f492(v0.clone())? {
        Ok(Some({
            let (_, h1805) = {
                let (h1806, _) = f493(Fn15::F494(v0.clone()), {
                    let a1807 = v0;
                    length_bytes(a1807)
                }, (0u64, 0u64))?;
                h1806
            };
            h1805
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f436(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f35(v1.clone(), 0u64, {
        let a1809 = {
            let a1808 = v1.clone();
            length_bytes(a1808)
        };
        let a1810 = f495(v1)?;
        nat_sub(a1809, a1810)
    });
    let v5: Bytes = {
        let a1811 = v0;
        let a1812 = v4.clone();
        append_bytes(a1811, a1812)
    };
    let v6: u64 = f434(v5.clone())?;
    let v7: Bytes = f35(v5.clone(), v6, {
        let a1814 = {
            let a1813 = v5;
            length_bytes(a1813)
        };
        let a1815 = v6;
        nat_sub(a1814, a1815)
    });
    let v8: u64 = {
        let a1816 = v4;
        length_bytes(a1816)
    };
    let v9: u64 = {
        let a1817 = v7.clone();
        length_bytes(a1817)
    };
    let a1818 = v9;
    let a1819 = 0u64;
    if nat_eq(a1818, a1819) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f433(v2, {
            let a1820 = v3;
            let a1821 = v8;
            nat_lt(a1820, a1821)
        }) {
            let v10: u64 = if v2 {
                let a1822 = v3;
                let a1823 = v8;
                nat_add(a1822, a1823)?
            } else {
                let a1824 = v8;
                let a1825 = v3;
                nat_sub(a1824, a1825)
            };
            let v11: u64 = f495(v7.clone())?;
            let a1826 = v10;
            let a1827 = v11;
            if nat_le(a1826, a1827) {
                Ok(Some((f35(v7, 0u64, {
                    let a1828 = v9;
                    let a1829 = v10;
                    nat_sub(a1828, a1829)
                }), 0u64)))
            } else {
                let a1832 = {
                    let a1830 = v10;
                    let a1831 = v11;
                    nat_sub(a1830, a1831)
                };
                let a1833 = 19u64;
                if nat_le(a1832, a1833) {
                    Ok(Some((f35(v7, 0u64, {
                        let a1834 = v9;
                        let a1835 = v11;
                        nat_sub(a1834, a1835)
                    }), {
                        let a1836 = v10;
                        let a1837 = v11;
                        nat_sub(a1836, a1837)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1838 = v3;
                let a1839 = v8;
                nat_sub(a1838, a1839)
            };
            let a1842 = {
                let a1840 = v9;
                let a1841 = v12;
                nat_add(a1840, a1841)?
            };
            let a1843 = 19u64;
            if nat_le(a1842, a1843) {
                Ok(Some(({
                    let a1844 = v7;
                    let a1845 = f477(v12);
                    append_bytes(a1844, a1845)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f437(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1846 = 0u64;
        let a1847 = v1;
        if nat_lt(a1846, a1847) {
            let a1848 = v1;
            let a1849 = 9223372036854775808u64;
            nat_le(a1848, a1849)
        } else {
            false
        }
    } else {
        let a1850 = v1;
        let a1851 = 9223372036854775807u64;
        nat_le(a1850, a1851)
    }
}

pub fn f438(p1854: List<Adt12>, v1: Bytes, v2: Bytes) -> Bytes {
    let m1853 = p1854;
    {
        let mut m1853 = m1853;
        loop {
            let step = {
                let v0 = m1853;
                let m1852 = v0;
                match m1852.uncons() {
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
                Ok(next) => { m1853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1855 = v2;
    let a1856 = 0u64;
    if nat_eq(a1855, a1856) {
        Ok(f496(v0, 0u64))
    } else {
        let (_, h1857) = {
            let (h1858, _) = f356(Fn3::F497(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1858
        };
        Ok(h1857)
    }
}

pub fn f440() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f441(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1862 = {
        let a1860 = v0.clone();
        let a1861 = {
            let (h1859, _) = v1.clone();
            h1859
        };
        index_bytes(a1860, a1861)
    };
    match m1862 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1864 = {
                    let (h1863, _) = v1.clone();
                    h1863
                };
                let a1865 = 1u64;
                nat_add(a1864, a1865)?
            }, {
                let a1868 = {
                    let (_, h1866) = v1;
                    h1866
                };
                let a1869 = f498(v0, {
                    let (h1867, _) = v1;
                    h1867
                }, v2)?;
                append_bytes(a1868, a1869)
            })))
        }
    }
}

pub fn f442(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1870 = v0.clone();
    match m1870.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1873 = {
                let (h1871, _) = v2;
                h1871
            };
            let a1874 = {
                let (h1872, _) = v1.clone();
                h1872
            };
            if equal(a1873, a1874) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f443(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f499(f500(Fn16::F501, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1879 = {
        let (h1875, _) = f502(Fn17::F503, {
            let a1877 = {
                let a1876 = v0;
                length_list(a1876)
            };
            let a1878 = 1u64;
            nat_add(a1877, a1878)?
        }, v1)?;
        h1875
    };
    match m1879.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f444(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1884 = {
        let a1881 = f266({
            let (h1880, _) = v0;
            h1880
        })?;
        let a1882 = Bytes::lit(&[0x3a]);
        append_bytes(a1881, a1882)
    };
    let a1885 = {
        let (_, h1883) = v0;
        h1883
    };
    Ok(append_bytes(a1884, a1885))
}

pub fn f445(v0: Bytes, v1: u64) -> bool {
    let m1886 = f169(v0, v1);
    match m1886 {
        None => {
            false
        }
        Some(v2) => {
            f504(v2)
        }
    }
}

pub fn f446(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f228(f28({
        let (h1887, _) = v0;
        h1887
    }, {
        let (h1888, _) = v1;
        h1888
    }), f28({
        let (_, h1889) = v0;
        h1889
    }, {
        let (_, h1890) = v1;
        h1890
    }))
}

pub fn f447(v0: Bytes) -> R<bool> {
    let m1891 = f288(v0.clone(), f8(Str::lit("&&")))?;
    if match m1891 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1892 = f288(v0, f8(Str::lit("||")))?;
        match m1892 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f448(v0: Adt19, v1: Option<u64>) -> bool {
    let m1893 = v1;
    match m1893 {
        None => {
            true
        }
        Some(v2) => {
            f386(v2, match v0 {
                Adt19::C0(_, h1894, _, _) => {
                    h1894
                }
            })
        }
    }
}

pub fn f449(v0: Adt15) -> Adt22 {
    Adt22::C1({
        let a1896 = match v0 {
            Adt15::C0(_, _, h1895, _, _) => {
                h1895
            }
        };
        length_list(a1896)
    })
}

pub fn f450(v0: Bytes) -> Option<Adt23> {
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

pub fn f451(v0: Bytes, p1901: List<Bytes>, p1902: u64) -> R<Option<u64>> {
    let m1900 = (p1901, p1902);
    {
        let mut m1900 = m1900;
        loop {
            let step = {
                let (v1, v2) = m1900;
                let m1897 = v1;
                match m1897.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f28(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1898 = v2;
                                let a1899 = 1u64;
                                nat_add(a1898, a1899)?
                            }))
                        }
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

pub fn f452(v0: Option<Adt13>) -> Option<u64> {
    let m1903 = v0;
    match m1903 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1904 = v1;
            match m1904 {
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

pub fn f453(v0: Fn12, p1908: List<List<(Bytes, ())>>, p1909: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1907 = (p1908, p1909);
    {
        let mut m1907 = m1907;
        loop {
            let step = {
                let (v1, v2) = m1907;
                let m1905 = v2;
                match m1905.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1906: Fn12 = v0.clone();
                            c1906.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1907 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f455(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1911 = {
        let a1910 = v0.clone();
        length_list(a1910)
    };
    let a1912 = 1u64;
    if nat_le(a1911, a1912) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f505(v0)?))
    }
}

pub fn f456(v0: Fn13, p1916: List<(Bytes, ())>, p1917: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1915 = (p1916, p1917);
    {
        let mut m1915 = m1915;
        loop {
            let step = {
                let (v1, v2) = m1915;
                let m1913 = v2;
                match m1913.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1914: Fn13 = v0.clone();
                            c1914.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f458(p1921: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1920 = p1921;
    {
        let mut m1920 = m1920;
        loop {
            let step = {
                let v0 = m1920;
                let m1918 = v0;
                match m1918.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f506(v2, v1.clone(), 0u64, {
                            let a1919 = v1.clone();
                            length_bytes(a1919)
                        })? {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: Bytes) -> Bytes {
    f7(List::cons(f112(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f460(v0: Adt1, v1: Bytes, v2: List<Adt13>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f115(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f51(v3.clone(), f115(f82(v0, f125())?, f8(Str::lit("at"))));
    let a1923 = {
        let a1922 = v4.clone();
        length_list(a1922)
    };
    let a1924 = 0u64;
    if nat_eq(a1923, a1924) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1927 = {
            let a1925 = v4;
            length_list(a1925)
        };
        let a1928 = {
            let a1926 = v3;
            length_list(a1926)
        };
        if nat_eq(a1927, a1928) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f507(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f461(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt13>) -> R<(bool, Bytes)> {
    let m1929 = v3;
    match m1929.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f47(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f47(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f47(v4.clone(), f8(Str::lit("construct")));
            if f433(f56(v0.clone(), f459(v6.clone())), f41(v6.clone(), v2.clone())) {
                f461(v0, v1, v2, v5)
            } else {
                if f40(f56(v0.clone(), f245(v6.clone()))) {
                    let v9: (bool, Bytes) = f461(v0, v1.clone(), v2, v5)?;
                    let (h1930, _) = v9.clone();
                    if h1930 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a1932 = f507(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a1933 = {
                                let (_, h1931) = v9;
                                h1931
                            };
                            append_bytes(a1932, a1933)
                        }))
                    }
                } else {
                    if f228(f40(f28(v8.clone(), f8(Str::lit("none")))), f40(f424(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a1934 = f8(Str::lit("Catalog."));
                        let a1935 = v8.clone();
                        append_bytes(a1934, a1935)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f461(v0, v1.clone(), v2, v5)?;
                        let (h1936, _) = v10.clone();
                        if h1936 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a1938 = f507(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a1939 = {
                                    let (_, h1937) = v10;
                                    h1937
                                };
                                append_bytes(a1938, a1939)
                            }))
                        }
                    } else {
                        Ok((true, f7(List::cons(f8(Str::lit("admit-registry ")), List::cons(v6, List::cons(f8(Str::lit(" ")), List::cons(v7, List::cons(f8(Str::lit(" ")), List::cons(v8, List::cons(f8(Str::lit(" ")), List::cons(f47(v4.clone(), f8(Str::lit("reader"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v4, f8(Str::lit("artifact"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))))))
                    }
                }
            }
        }
    }
}

pub fn f462(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt13> = f231(f233(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt13> = f231(f233(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f98(f115(v3.clone(), f8(Str::lit("id"))), f115(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f115(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f51(v5.clone(), f98(v6.clone(), f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1941 = {
        let a1940 = v7.clone();
        length_list(a1940)
    };
    let a1942 = 0u64;
    if nat_eq(a1941, a1942) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f40({
            let a1945 = {
                let a1943 = v7;
                length_list(a1943)
            };
            let a1946 = {
                let a1944 = v5;
                length_list(a1944)
            };
            nat_eq(a1945, a1946)
        }) {
            Ok((false, f507(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f118(f98(f51(f508(v4.clone()), f52(f53(v0, f8(Str::lit("model/authorities.toml"))))?), f51(f509(v4), f98(v6, f115(v3, f8(Str::lit("id")))))))?;
            let a1948 = {
                let a1947 = v8.clone();
                length_list(a1947)
            };
            let a1949 = 0u64;
            if nat_eq(a1948, a1949) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f507(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f95(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f463(v0: u64) -> bool {
    let a1950 = v0;
    let a1951 = 47u64;
    nat_eq(a1950, a1951)
}

pub fn f464(v0: u64) -> bool {
    f433({
        let a1952 = v0;
        let a1953 = 95u64;
        nat_eq(a1952, a1953)
    }, f433({
        let a1954 = v0;
        let a1955 = 45u64;
        nat_eq(a1954, a1955)
    }, f433({
        let a1956 = v0;
        let a1957 = 32u64;
        nat_eq(a1956, a1957)
    }, f433({
        let a1958 = v0;
        let a1959 = 46u64;
        nat_eq(a1958, a1959)
    }, {
        let a1960 = v0;
        let a1961 = 47u64;
        nat_eq(a1960, a1961)
    }))))
}

pub fn f465(v0: u64) -> bool {
    f228({
        let a1962 = 97u64;
        let a1963 = v0;
        nat_le(a1962, a1963)
    }, {
        let a1964 = v0;
        let a1965 = 122u64;
        nat_le(a1964, a1965)
    })
}

pub fn f466(v0: u64) -> bool {
    f228({
        let a1966 = 65u64;
        let a1967 = v0;
        nat_le(a1966, a1967)
    }, {
        let a1968 = v0;
        let a1969 = 90u64;
        nat_le(a1968, a1969)
    })
}

pub fn f467(v0: u64) -> bool {
    f228({
        let a1970 = 48u64;
        let a1971 = v0;
        nat_le(a1970, a1971)
    }, {
        let a1972 = v0;
        let a1973 = 57u64;
        nat_le(a1972, a1973)
    })
}

pub fn f468(v0: List<(Bytes, Adt17)>, v1: List<(Bytes, Adt17)>) -> List<(Bytes, Adt17)> {
    let m1974 = v0;
    match m1974.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f468(v3, v1))
        }
    }
}

pub fn f469(v0: Option<Adt17>) -> List<(Bytes, Adt17)> {
    let m1975 = v0;
    match m1975 {
        None => {
            List::<(Bytes, Adt17)>::nil()
        }
        Some(v1) => {
            f510(v1)
        }
    }
}

pub fn f470(v0: u64, v1: List<(Bytes, Adt17)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt17)>, Bytes>> {
    let m1976 = v0;
    if m1976 == 0 {
        let m1977 = v1;
        match m1977.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt17)>, Bytes>(List::<(Bytes, Adt17)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt17)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m1976 - 1;
        let m1978 = v1.clone();
        match m1978.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt17)>, Bytes>(List::<(Bytes, Adt17)>::nil()))
            }
            Some((_, _)) => {
                let m1979 = f511(v1.clone(), v2.clone(), v3.clone())?;
                match m1979 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt17)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f346(f471(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m1982 = f470(v6, f512(v1, {
                            let (h1980, _) = v9.clone();
                            h1980
                        }), List::cons(f513({
                            let (h1981, _) = v9.clone();
                            h1981
                        })?, v2), v3)?;
                        match m1982 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt17)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt17)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f471(v0: List<(Bytes, Adt17)>) -> R<List<Bytes>> {
    let m1983 = v0;
    match m1983.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f513({
                let (h1984, _) = v1;
                h1984
            })?, f471(v2)?))
        }
    }
}

pub fn f472(p1990: u64, p1991: List<Adt25>, p1992: List<Bytes>, p1993: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1989 = (p1990, (p1991, (p1992, p1993)));
    {
        let mut m1989 = m1989;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1989;
                let m1985 = v0;
                if m1985 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1985 - 1;
                    let m1986 = v1;
                    match m1986.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1987 = v5;
                            match m1987 {
                                Adt25::C0(v8, v9) => {
                                    if f28(v8.clone(), f514()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f41(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1988 = f516(v8.clone(), v9)?;
                                            match m1988 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f473(f517(v11.clone()), List::cons(Adt25::C1(f518(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f41(f514(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f514(), v2), List::cons(f515(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1989 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m1994 = v0;
    match m1994.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f473(v3, v1))
        }
    }
}

pub fn f474(v0: List<(Bytes, Adt17)>) -> R<List<Adt25>> {
    let m1995 = v0;
    match m1995.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f513({
                let (h1996, _) = v1;
                h1996
            })?, {
                let (_, h1997) = v1;
                h1997
            }), f474(v2)?))
        }
    }
}

pub fn f475(v0: List<Adt13>, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1998 = v0;
    match m1998.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            if f28(f47(v3.clone(), v1.clone()), v2.clone()) {
                f475(v4, v1, v2)
            } else {
                List::cons(v3, f475(v4, v1, v2))
            }
        }
    }
}

pub fn f476(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1999 = v0;
    match m1999.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f72(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f346(f486(f118(List::cons(v1, f519(f33(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f476(v3, v1)?))
            }
        }
    }
}

pub fn f477(v0: u64) -> Bytes {
    let m2000 = v0;
    if m2000 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2000 - 1;
        let a2001 = f8(Str::lit("0"));
        let a2002 = f477(v1);
        append_bytes(a2001, a2002)
    }
}

pub fn f478(v0: u32) -> Bytes {
    let a2005 = {
        let a2003 = v0;
        let a2004 = 8u32;
        fixed_u32::bit_and(a2003, a2004)
    };
    let a2006 = 0u32;
    if equal(a2005, a2006) {
        let a2009 = {
            let a2007 = v0;
            let a2008 = 4u32;
            fixed_u32::bit_and(a2007, a2008)
        };
        let a2010 = 0u32;
        if equal(a2009, a2010) {
            let a2013 = {
                let a2011 = v0;
                let a2012 = 2u32;
                fixed_u32::bit_and(a2011, a2012)
            };
            let a2014 = 0u32;
            if equal(a2013, a2014) {
                let a2017 = {
                    let a2015 = v0;
                    let a2016 = 1u32;
                    fixed_u32::bit_and(a2015, a2016)
                };
                let a2018 = 0u32;
                if equal(a2017, a2018) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2021 = {
                    let a2019 = v0;
                    let a2020 = 1u32;
                    fixed_u32::bit_and(a2019, a2020)
                };
                let a2022 = 0u32;
                if equal(a2021, a2022) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2025 = {
                let a2023 = v0;
                let a2024 = 2u32;
                fixed_u32::bit_and(a2023, a2024)
            };
            let a2026 = 0u32;
            if equal(a2025, a2026) {
                let a2029 = {
                    let a2027 = v0;
                    let a2028 = 1u32;
                    fixed_u32::bit_and(a2027, a2028)
                };
                let a2030 = 0u32;
                if equal(a2029, a2030) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2033 = {
                    let a2031 = v0;
                    let a2032 = 1u32;
                    fixed_u32::bit_and(a2031, a2032)
                };
                let a2034 = 0u32;
                if equal(a2033, a2034) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2037 = {
            let a2035 = v0;
            let a2036 = 4u32;
            fixed_u32::bit_and(a2035, a2036)
        };
        let a2038 = 0u32;
        if equal(a2037, a2038) {
            let a2041 = {
                let a2039 = v0;
                let a2040 = 2u32;
                fixed_u32::bit_and(a2039, a2040)
            };
            let a2042 = 0u32;
            if equal(a2041, a2042) {
                let a2045 = {
                    let a2043 = v0;
                    let a2044 = 1u32;
                    fixed_u32::bit_and(a2043, a2044)
                };
                let a2046 = 0u32;
                if equal(a2045, a2046) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2049 = {
                    let a2047 = v0;
                    let a2048 = 1u32;
                    fixed_u32::bit_and(a2047, a2048)
                };
                let a2050 = 0u32;
                if equal(a2049, a2050) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2053 = {
                let a2051 = v0;
                let a2052 = 2u32;
                fixed_u32::bit_and(a2051, a2052)
            };
            let a2054 = 0u32;
            if equal(a2053, a2054) {
                let a2057 = {
                    let a2055 = v0;
                    let a2056 = 1u32;
                    fixed_u32::bit_and(a2055, a2056)
                };
                let a2058 = 0u32;
                if equal(a2057, a2058) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2061 = {
                    let a2059 = v0;
                    let a2060 = 1u32;
                    fixed_u32::bit_and(a2059, a2060)
                };
                let a2062 = 0u32;
                if equal(a2061, a2062) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f479(v0: u32, v1: u32) -> u32 {
    let m2065 = {
        let a2063 = v0;
        let a2064 = v1;
        fixed_u32::shift_right(a2063, a2064)
    };
    match m2065 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f480(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f520(Fn18::F521, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2066, _, _, _, _, _, _, _) => {
            h2066
        }
    }, match v0.clone() {
        Adt24::C0(_, h2067, _, _, _, _, _, _) => {
            h2067
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2068, _, _, _, _, _) => {
            h2068
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2069, _, _, _, _) => {
            h2069
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2070, _, _, _) => {
            h2070
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2071, _, _) => {
            h2071
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2072, _) => {
            h2072
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2073) => {
            h2073
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f522(match v0.clone() {
        Adt24::C0(h2074, _, _, _, _, _, _, _) => {
            h2074
        }
    }, match v2.clone() {
        Adt27::C0(h2075, _, _, _, _, _, _, _, _, _) => {
            h2075
        }
    }), f522(match v0.clone() {
        Adt24::C0(_, h2076, _, _, _, _, _, _) => {
            h2076
        }
    }, match v2.clone() {
        Adt27::C0(_, h2077, _, _, _, _, _, _, _, _) => {
            h2077
        }
    }), f522(match v0.clone() {
        Adt24::C0(_, _, h2078, _, _, _, _, _) => {
            h2078
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2079, _, _, _, _, _, _, _) => {
            h2079
        }
    }), f522(match v0.clone() {
        Adt24::C0(_, _, _, h2080, _, _, _, _) => {
            h2080
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2081, _, _, _, _, _, _) => {
            h2081
        }
    }), f522(match v0.clone() {
        Adt24::C0(_, _, _, _, h2082, _, _, _) => {
            h2082
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2083, _, _, _, _, _) => {
            h2083
        }
    }), f522(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2084, _, _) => {
            h2084
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2085, _, _, _, _) => {
            h2085
        }
    }), f522(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2086, _) => {
            h2086
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2087, _, _, _) => {
            h2087
        }
    }), f522(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2088) => {
            h2088
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2089, _, _) => {
            h2089
        }
    })))
}

pub fn f481(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f523(v0.clone(), v1, v2, v3)?, f523(v0.clone(), v1, v2, {
        let a2090 = v3;
        let a2091 = 4u64;
        nat_add(a2090, a2091)?
    })?, f523(v0.clone(), v1, v2, {
        let a2092 = v3;
        let a2093 = 8u64;
        nat_add(a2092, a2093)?
    })?, f523(v0.clone(), v1, v2, {
        let a2094 = v3;
        let a2095 = 12u64;
        nat_add(a2094, a2095)?
    })?, f523(v0.clone(), v1, v2, {
        let a2096 = v3;
        let a2097 = 16u64;
        nat_add(a2096, a2097)?
    })?, f523(v0.clone(), v1, v2, {
        let a2098 = v3;
        let a2099 = 20u64;
        nat_add(a2098, a2099)?
    })?, f523(v0.clone(), v1, v2, {
        let a2100 = v3;
        let a2101 = 24u64;
        nat_add(a2100, a2101)?
    })?, f523(v0.clone(), v1, v2, {
        let a2102 = v3;
        let a2103 = 28u64;
        nat_add(a2102, a2103)?
    })?, f523(v0.clone(), v1, v2, {
        let a2104 = v3;
        let a2105 = 32u64;
        nat_add(a2104, a2105)?
    })?, f523(v0.clone(), v1, v2, {
        let a2106 = v3;
        let a2107 = 36u64;
        nat_add(a2106, a2107)?
    })?, f523(v0.clone(), v1, v2, {
        let a2108 = v3;
        let a2109 = 40u64;
        nat_add(a2108, a2109)?
    })?, f523(v0.clone(), v1, v2, {
        let a2110 = v3;
        let a2111 = 44u64;
        nat_add(a2110, a2111)?
    })?, f523(v0.clone(), v1, v2, {
        let a2112 = v3;
        let a2113 = 48u64;
        nat_add(a2112, a2113)?
    })?, f523(v0.clone(), v1, v2, {
        let a2114 = v3;
        let a2115 = 52u64;
        nat_add(a2114, a2115)?
    })?, f523(v0.clone(), v1, v2, {
        let a2116 = v3;
        let a2117 = 56u64;
        nat_add(a2116, a2117)?
    })?, f523(v0, v1, v2, {
        let a2118 = v3;
        let a2119 = 60u64;
        nat_add(a2118, a2119)?
    })?))
}

pub fn f482(v0: Adt17) -> R<Result<Bytes, Bytes>> {
    let m2120 = f524(v0.clone());
    match m2120 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f335(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f483(v0: Bytes, v1: Bytes, v2: Bytes, v3: Adt17) -> Adt17 {
    Adt17::C7(std::rc::Rc::new(Adt17::C10(f8(Str::lit("citation")), std::rc::Rc::new(Adt17::C6(std::rc::Rc::new(f525(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt17::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt17::C5(v2)), std::rc::Rc::new(Adt17::C10(f8(Str::lit("pointer")), std::rc::Rc::new(Adt17::C5(Bytes::lit(&[]))), std::rc::Rc::new(Adt17::C10(f8(Str::lit("value")), std::rc::Rc::new(v3), std::rc::Rc::new(Adt17::C8))))))))))
}

pub fn f484(v0: u64, v1: Adt17, v2: Adt17, v3: Adt17, v4: Bytes) -> R<Option<Bytes>> {
    let m2121 = v0;
    if m2121 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m2121 - 1;
        let m2122 = v2.clone();
        match m2122 {
            Adt17::C0 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C1(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C2(v6) => {
                if v6 {
                    Ok(None::<Bytes>)
                } else {
                    Ok(f526(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
                }
            }
            Adt17::C3(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C4(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C5(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C6(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C7(r2136) => {
                let v7 = (*r2136).clone();
                let m2123 = f527(f510(v7));
                match m2123 {
                    None => {
                        Ok(f528(f529(v2.clone(), v3.clone(), v4.clone())?, f528({
                            let m2124 = f336(f239(v2.clone(), f8(Str::lit("$ref"))));
                            match m2124 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m2125 = f530(v1.clone(), v9)?;
                                    match m2125 {
                                        Ok(v11) => {
                                            f484(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f528(f531(v5, v1.clone(), f244(f239(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f528({
                            let m2126 = f239(v2.clone(), f8(Str::lit("anyOf")));
                            match m2126 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a2127 = 0u64;
                                    let a2128 = f532(v5, v1.clone(), f244(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a2127, a2128) {
                                        None::<Bytes>
                                    } else {
                                        f526(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f528({
                            let m2129 = f239(v2.clone(), f8(Str::lit("oneOf")));
                            match m2129 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a2130 = f532(v5, v1.clone(), f244(Some(v13)), v3.clone(), v4.clone())?;
                                    let a2131 = 1u64;
                                    if nat_eq(a2130, a2131) {
                                        None::<Bytes>
                                    } else {
                                        f526(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m2132 = v3.clone();
                            match m2132 {
                                Adt17::C0 => {
                                    None::<Bytes>
                                }
                                Adt17::C1(_, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C2(_) => {
                                    None::<Bytes>
                                }
                                Adt17::C3(_, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C4(_, _, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C5(_) => {
                                    None::<Bytes>
                                }
                                Adt17::C6(r2134) => {
                                    let v15 = (*r2134).clone();
                                    let m2133 = f239(v2, f8(Str::lit("items")));
                                    match m2133 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f535(v5, v1, v16, f337(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt17::C7(r2135) => {
                                    let v14 = (*r2135).clone();
                                    f528(f533(v5, v1.clone(), f469(f239(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f534(v5, v1, v2, f510(v14), v4)?)
                                }
                                Adt17::C8 => {
                                    None::<Bytes>
                                }
                                Adt17::C9(_, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C10(_, _, _) => {
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
            Adt17::C8 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C9(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C10(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
        }
    }
}

pub fn f485(v0: Adt17) -> R<Bytes> {
    f335(v0)
}

pub fn f486(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2137 = v0;
    match m2137.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f306(v1)?, f486(v2)?))
        }
    }
}

pub fn f487(p2140: List<Bytes>, v1: Bytes) -> bool {
    let m2139 = p2140;
    {
        let mut m2139 = m2139;
        loop {
            let step = {
                let v0 = m2139;
                let m2138 = v0;
                match m2138.uncons() {
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
                Ok(next) => { m2139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: Fn14, p2145: u64, p2146: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2144 = (p2145, p2146);
    {
        let mut m2144 = m2144;
        loop {
            let step = {
                let (v1, v2) = m2144;
                let m2141 = v1;
                if m2141 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2141 - 1;
                    let m2143 = {
                        let c2142: Fn14 = v0.clone();
                        c2142.apply(v2.clone())?
                    };
                    match m2143 {
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
                Ok(next) => { m2144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f536(v0, v1)
}

pub fn f490(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f537(v0, v1)
}

pub fn f491(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f538(v0, v1)
}

pub fn f492(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2147 = v0.clone();
        length_bytes(a2147)
    };
    let a2148 = 0u64;
    let a2149 = v1;
    if nat_lt(a2148, a2149) {
        let a2150 = f539(v0.clone())?;
        let a2151 = v1;
        if nat_eq(a2150, a2151) {
            let a2152 = 1u64;
            let a2153 = v1;
            if nat_lt(a2152, a2153) {
                let m2156 = {
                    let a2154 = v0;
                    let a2155 = 0u64;
                    index_bytes(a2154, a2155)
                };
                match m2156 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2157 = v2;
                        let a2158 = 48u8;
                        Ok(!equal(a2157, a2158))
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

pub fn f493(v0: Fn15, p2163: u64, p2164: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2162 = (p2163, p2164);
    {
        let mut m2162 = m2162;
        loop {
            let step = {
                let (v1, v2) = m2162;
                let m2159 = v1;
                if m2159 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2159 - 1;
                    let m2161 = {
                        let c2160: Fn15 = v0.clone();
                        c2160.apply(v2)?
                    };
                    match m2161 {
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
                Ok(next) => { m2162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f540(v0, v1)
}

pub fn f495(v0: Bytes) -> R<u64> {
    let (h2165, _) = f367(Fn6::F541(v0.clone()), {
        let a2167 = {
            let a2166 = v0;
            length_bytes(a2166)
        };
        let a2168 = 1u64;
        nat_add(a2167, a2168)?
    }, 0u64)?;
    Ok(h2165)
}

pub fn f496(v0: Bytes, v1: u64) -> Bytes {
    let m2172 = {
        let a2169 = v0;
        let a2170 = v1;
        let a2171 = 1u64;
        slice_bytes(a2169, a2170, a2171)
    };
    match m2172 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f497(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f542(v1, v0, v2)
}

pub fn f498(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2217 = {
            let a2193 = {
                let a2181 = {
                    let a2175 = {
                        let a2173 = v3;
                        let a2174 = 128u8;
                        fixed_u8::bit_and(a2173, a2174)
                    };
                    let a2176 = 128u8;
                    if equal(a2175, a2176) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2182 = {
                    let a2179 = {
                        let a2177 = v3;
                        let a2178 = 64u8;
                        fixed_u8::bit_and(a2177, a2178)
                    };
                    let a2180 = 64u8;
                    if equal(a2179, a2180) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2181, a2182)?
            };
            let a2194 = {
                let a2191 = {
                    let a2185 = {
                        let a2183 = v3;
                        let a2184 = 32u8;
                        fixed_u8::bit_and(a2183, a2184)
                    };
                    let a2186 = 32u8;
                    if equal(a2185, a2186) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2192 = {
                    let a2189 = {
                        let a2187 = v3;
                        let a2188 = 16u8;
                        fixed_u8::bit_and(a2187, a2188)
                    };
                    let a2190 = 16u8;
                    if equal(a2189, a2190) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2191, a2192)?
            };
            nat_add(a2193, a2194)?
        };
        let a2218 = {
            let a2215 = {
                let a2203 = {
                    let a2197 = {
                        let a2195 = v3;
                        let a2196 = 8u8;
                        fixed_u8::bit_and(a2195, a2196)
                    };
                    let a2198 = 8u8;
                    if equal(a2197, a2198) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2204 = {
                    let a2201 = {
                        let a2199 = v3;
                        let a2200 = 4u8;
                        fixed_u8::bit_and(a2199, a2200)
                    };
                    let a2202 = 4u8;
                    if equal(a2201, a2202) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2203, a2204)?
            };
            let a2216 = {
                let a2213 = {
                    let a2207 = {
                        let a2205 = v3;
                        let a2206 = 2u8;
                        fixed_u8::bit_and(a2205, a2206)
                    };
                    let a2208 = 2u8;
                    if equal(a2207, a2208) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2214 = {
                    let a2211 = {
                        let a2209 = v3;
                        let a2210 = 1u8;
                        fixed_u8::bit_and(a2209, a2210)
                    };
                    let a2212 = 1u8;
                    if equal(a2211, a2212) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2213, a2214)?
            };
            nat_add(a2215, a2216)?
        };
        nat_add(a2217, a2218)?
    };
    let a2219 = v4;
    let a2220 = 34u64;
    if nat_eq(a2219, a2220) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2221 = v4;
        let a2222 = 92u64;
        if nat_eq(a2221, a2222) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2223 = v4;
            let a2224 = 8u64;
            if nat_eq(a2223, a2224) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2225 = v4;
                let a2226 = 12u64;
                if nat_eq(a2225, a2226) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2227 = v4;
                    let a2228 = 10u64;
                    if nat_eq(a2227, a2228) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2229 = v4;
                        let a2230 = 13u64;
                        if nat_eq(a2229, a2230) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2231 = v4;
                            let a2232 = 9u64;
                            if nat_eq(a2231, a2232) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2233 = v4;
                                let a2234 = 32u64;
                                if nat_lt(a2233, a2234) {
                                    let a2235 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2236 = f543(v2)?;
                                    Ok(append_bytes(a2235, a2236))
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

pub fn f499(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f544(Fn19::F545, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f500(v0: Fn16, p2240: List<List<(Bytes, Bytes)>>, p2241: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2239 = (p2240, p2241);
    {
        let mut m2239 = m2239;
        loop {
            let step = {
                let (v1, v2) = m2239;
                let m2237 = v2;
                match m2237.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2238: Fn16 = v0.clone();
                            c2238.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f502(v0: Fn17, p2246: u64, p2247: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2245 = (p2246, p2247);
    {
        let mut m2245 = m2245;
        loop {
            let step = {
                let (v1, v2) = m2245;
                let m2242 = v1;
                if m2242 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2242 - 1;
                    let m2244 = {
                        let c2243: Fn17 = v0.clone();
                        c2243.apply(v2.clone())?
                    };
                    match m2244 {
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
                Ok(next) => { m2245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f546(v0)
}

pub fn f504(v0: u8) -> bool {
    let a2248 = v0;
    let a2249 = 32u8;
    if if if equal(a2248, a2249) {
        true
    } else {
        let a2250 = v0;
        let a2251 = 9u8;
        equal(a2250, a2251)
    } {
        true
    } else {
        let a2252 = v0;
        let a2253 = 10u8;
        equal(a2252, a2253)
    } {
        true
    } else {
        let a2254 = v0;
        let a2255 = 12u8;
        if equal(a2254, a2255) {
            true
        } else {
            let a2256 = v0;
            let a2257 = 13u8;
            equal(a2256, a2257)
        }
    }
}

pub fn f505(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f389({
        let (_, h2258) = {
            let (h2259, _) = f547(Fn20::F548, {
                let a2261 = {
                    let a2260 = v0.clone();
                    length_list(a2260)
                };
                let a2262 = 1u64;
                nat_add(a2261, a2262)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2259
        };
        h2258
    }))
}

pub fn f506(v0: Bytes, v1: Bytes, p2275: u64, p2276: u64) -> R<bool> {
    let m2274 = (p2275, p2276);
    {
        let mut m2274 = m2274;
        loop {
            let step = {
                let (v2, v3) = m2274;
                let m2263 = v3;
                if m2263 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m2263 - 1;
                    let m2266 = {
                        let a2264 = v1.clone();
                        let a2265 = v2;
                        index_bytes(a2264, a2265)
                    };
                    match m2266 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m2269 = {
                                let a2267 = v0.clone();
                                let a2268 = v2;
                                index_bytes(a2267, a2268)
                            };
                            match m2269 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a2270 = v5;
                                    let a2271 = v6;
                                    if equal(a2270, a2271) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a2272 = v2;
                                            let a2273 = 1u64;
                                            nat_add(a2272, a2273)?
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
                Ok(next) => { m2274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f508(v0: List<Adt13>) -> List<Bytes> {
    let m2277 = v0;
    match m2277.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2278 = f79(v1, f8(Str::lit("std")));
            match m2278 {
                None => {
                    f508(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f508(v2))
                }
            }
        }
    }
}

pub fn f509(v0: List<Adt13>) -> List<Bytes> {
    let m2279 = v0;
    match m2279.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2280 = f78(f79(v1, f8(Str::lit("req"))));
            match m2280 {
                None => {
                    f509(v2)
                }
                Some(v3) => {
                    List::cons(v3, f509(v2))
                }
            }
        }
    }
}

pub fn f510(v0: Adt17) -> List<(Bytes, Adt17)> {
    let m2281 = v0;
    match m2281 {
        Adt17::C0 => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C1(_, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C2(_) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C3(_, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C4(_, _, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C5(_) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C6(_) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C7(r2282) => {
            let v4 = (*r2282).clone();
            f510(v4)
        }
        Adt17::C8 => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C9(_, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C10(v1, r2283, r2284) => {
            let v2 = (*r2283).clone();
            let v3 = (*r2284).clone();
            List::cons((v1, v2), f510(v3))
        }
    }
}

pub fn f511(p2289: List<(Bytes, Adt17)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt17)>> {
    let m2288 = p2289;
    {
        let mut m2288 = m2288;
        loop {
            let step = {
                let v0 = m2288;
                let m2285 = v0;
                match m2285.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, R<Option<(Bytes, Adt17)>>>(Ok(None::<(Bytes, Adt17)>))
                    }
                    Some((v3, v4)) => {
                        if f549(f550({
                            let (_, h2286) = v3.clone();
                            h2286
                        }, List::<Bytes>::nil())?, f513({
                            let (h2287, _) = v3.clone();
                            h2287
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt17)>, R<Option<(Bytes, Adt17)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt17)>, R<Option<(Bytes, Adt17)>>>(v4)
                        }
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

pub fn f512(v0: List<(Bytes, Adt17)>, v1: Bytes) -> List<(Bytes, Adt17)> {
    let m2290 = v0;
    match m2290.uncons() {
        None => {
            List::<(Bytes, Adt17)>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h2291, _) = v2.clone();
                h2291
            }, v1.clone()) {
                f512(v3, v1)
            } else {
                List::cons(v2, f512(v3, v1))
            }
        }
    }
}

pub fn f513(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f330(v0)?;
    if f41(v1.clone(), f551()) {
        f552(v1)
    } else {
        Ok(v1)
    }
}

pub fn f514() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f515() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f516(v0: Bytes, v1: Adt17) -> R<Result<Adt28, Bytes>> {
    f553(v0, f554(v1))
}

pub fn f517(v0: Adt28) -> List<Adt25> {
    let m2292 = v0;
    match m2292 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f518(v0: Adt28) -> Bytes {
    let m2293 = v0;
    match m2293 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f519(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2294 = v0;
    match m2294.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f519(v3, false))
            } else {
                f519(v3, true)
            }
        }
    }
}

pub fn f520(v0: Fn18, p2298: u64, p2299: Adt27) -> R<Adt27> {
    let m2297 = (p2298, p2299);
    {
        let mut m2297 = m2297;
        loop {
            let step = {
                let (v1, v2) = m2297;
                let m2295 = v1;
                if m2295 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2295 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2296: Fn18 = v0.clone();
                        c2296.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: Adt27) -> R<Adt27> {
    f555(v0)
}

pub fn f522(v0: u32, v1: u32) -> u32 {
    let m2302 = {
        let a2300 = f556(v0);
        let a2301 = f556(v1);
        fixed_u64::checked_add(a2300, a2301)
    };
    match m2302 {
        None => {
            0u32
        }
        Some(v2) => {
            f557(v2)
        }
    }
}

pub fn f523(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2305 = {
        let a2303 = v3;
        let a2304 = 8u64;
        nat_add(a2303, a2304)?
    };
    let a2306 = v2;
    if nat_eq(a2305, a2306) {
        Ok(f558({
            let a2309 = {
                let a2307 = v1;
                let a2308 = 8u64;
                nat_mul(a2307, a2308)?
            };
            let a2310 = 4294967296u64;
            let a2311 = 0u64;
            nat_quot(a2309, a2310, a2311)
        }))
    } else {
        let a2314 = {
            let a2312 = v3;
            let a2313 = 4u64;
            nat_add(a2312, a2313)?
        };
        let a2315 = v2;
        if nat_eq(a2314, a2315) {
            Ok(f558({
                let a2318 = {
                    let a2316 = v1;
                    let a2317 = 8u64;
                    nat_mul(a2316, a2317)?
                };
                let a2319 = 4294967296u64;
                let a2320 = 0u64;
                nat_rem(a2318, a2319, a2320)
            }))
        } else {
            Ok(f559(f560(v0.clone(), v1, v3), f560(v0.clone(), v1, {
                let a2321 = v3;
                let a2322 = 1u64;
                nat_add(a2321, a2322)?
            }), f560(v0.clone(), v1, {
                let a2323 = v3;
                let a2324 = 2u64;
                nat_add(a2323, a2324)?
            }), f560(v0, v1, {
                let a2325 = v3;
                let a2326 = 3u64;
                nat_add(a2325, a2326)?
            })))
        }
    }
}

pub fn f524(v0: Adt17) -> Option<Bytes> {
    let m2327 = v0;
    match m2327 {
        Adt17::C0 => {
            None::<Bytes>
        }
        Adt17::C1(v1, _) => {
            Some(v1)
        }
        Adt17::C2(_) => {
            None::<Bytes>
        }
        Adt17::C3(_, _) => {
            None::<Bytes>
        }
        Adt17::C4(_, _, _) => {
            None::<Bytes>
        }
        Adt17::C5(_) => {
            None::<Bytes>
        }
        Adt17::C6(r2330) => {
            let v3 = (*r2330).clone();
            f524(v3)
        }
        Adt17::C7(r2331) => {
            let v4 = (*r2331).clone();
            f524(v4)
        }
        Adt17::C8 => {
            None::<Bytes>
        }
        Adt17::C9(r2332, r2333) => {
            let v5 = (*r2332).clone();
            let v6 = (*r2333).clone();
            let m2328 = f524(v5);
            match m2328 {
                None => {
                    f524(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt17::C10(_, r2334, r2335) => {
            let v9 = (*r2334).clone();
            let v10 = (*r2335).clone();
            let m2329 = f524(v9);
            match m2329 {
                None => {
                    f524(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f525(v0: List<Bytes>) -> Adt17 {
    let m2336 = v0;
    match m2336.uncons() {
        None => {
            Adt17::C8
        }
        Some((v1, v2)) => {
            Adt17::C9(std::rc::Rc::new(Adt17::C5(v1)), std::rc::Rc::new(f525(v2)))
        }
    }
}

pub fn f526(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f527(p2341: List<(Bytes, Adt17)>) -> Option<Bytes> {
    let m2340 = p2341;
    {
        let mut m2340 = m2340;
        loop {
            let step = {
                let v0 = m2340;
                let m2337 = v0;
                match m2337.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h2338, _) = v1.clone();
                            h2338
                        }, f561()) {
                            Ok::<List<(Bytes, Adt17)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt17)>, Option<Bytes>>(Some({
                                let (h2339, _) = v1;
                                h2339
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2342 = v0;
    match m2342 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f529(v0: Adt17, v1: Adt17, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f562(v1.clone());
    let v7: Option<Bytes> = {
        let m2343 = f239(v0.clone(), f8(Str::lit("type")));
        match m2343 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m2344 = f336(Some(v4.clone()));
                    match m2344 {
                        None => {
                            f243(f244(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f563(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f526(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f346(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m2345 = f239(v0.clone(), f8(Str::lit("enum")));
        match m2345 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f564(f244(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f526(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m2346 = f239(v0.clone(), f8(Str::lit("const")));
        match m2346 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f565(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f526(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v41: Option<Bytes> = {
        let m2347 = v1.clone();
        match m2347 {
            Adt17::C0 => {
                None::<Bytes>
            }
            Adt17::C1(v28, _) => {
                f526(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
            }
            Adt17::C2(_) => {
                None::<Bytes>
            }
            Adt17::C3(_, _) => {
                None::<Bytes>
            }
            Adt17::C4(_, _, _) => {
                None::<Bytes>
            }
            Adt17::C5(v12) => {
                let m2348 = f566(v12.clone())?;
                match m2348 {
                    None => {
                        f526(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f567(v13)?;
                        f528({
                            let m2349 = f568(f239(v0.clone(), f8(Str::lit("minLength"))));
                            match m2349 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a2350 = v14;
                                    let a2351 = v15;
                                    if nat_lt(a2350, a2351) {
                                        f526(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f528({
                            let m2352 = f568(f239(v0.clone(), f8(Str::lit("maxLength"))));
                            match m2352 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a2353 = v16;
                                    let a2354 = v14;
                                    if nat_lt(a2353, a2354) {
                                        f526(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m2355 = f336(f239(v0.clone(), f8(Str::lit("pattern"))));
                            match m2355 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m2356 = f569(v17.clone(), v12)?;
                                    match m2356 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f526(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
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
            Adt17::C6(r2364) => {
                let v20 = (*r2364).clone();
                let v21: u64 = f570(v20)?;
                f528({
                    let m2357 = f568(f239(v0.clone(), f8(Str::lit("minItems"))));
                    match m2357 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a2358 = v21;
                            let a2359 = v22;
                            if nat_lt(a2358, a2359) {
                                f526(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2360 = f568(f239(v0.clone(), f8(Str::lit("maxItems"))));
                    match m2360 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a2361 = v23;
                            let a2362 = v21;
                            if nat_lt(a2361, a2362) {
                                f526(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt17::C7(r2365) => {
                let v24 = (*r2365).clone();
                let v25: List<Bytes> = f571(f243(f244(f239(v0.clone(), f8(Str::lit("required"))))), f572(f510(v24)));
                let m2363 = v25;
                match m2363.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f526(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
                    }
                }
            }
            Adt17::C8 => {
                None::<Bytes>
            }
            Adt17::C9(_, _) => {
                None::<Bytes>
            }
            Adt17::C10(_, _, _) => {
                None::<Bytes>
            }
        }
    };
    let v47: Option<Bytes> = {
        let m2366 = f573(v1);
        match m2366 {
            None => {
                None::<Bytes>
            }
            Some(v42) => {
                f528({
                    let m2368 = {
                        let m2367 = f239(v0.clone(), f8(Str::lit("minimum")));
                        match m2367 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v43) => {
                                f573(v43)
                            }
                        }
                    };
                    match m2368 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v44) => {
                            if f574(v42, v44)? {
                                f526(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2370 = {
                        let m2369 = f239(v0, f8(Str::lit("maximum")));
                        match m2369 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v45) => {
                                f573(v45)
                            }
                        }
                    };
                    match m2370 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v46) => {
                            if f574(v46, v42)? {
                                f526(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f528(v7, f528(v9, f528(v11, f528(v41, v47)))))
}

pub fn f530(v0: Adt17, v1: Bytes) -> R<Result<Adt17, Bytes>> {
    let v2: u64 = {
        let a2371 = v1.clone();
        length_bytes(a2371)
    };
    if f28(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt17, Bytes>(v0))
    } else {
        let m2376 = if f72(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f35(v1.clone(), 8u64, {
                let a2372 = v2;
                let a2373 = 8u64;
                nat_sub(a2372, a2373)
            })))
        } else {
            if f72(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f35(v1.clone(), 14u64, {
                    let a2374 = v2;
                    let a2375 = 14u64;
                    nat_sub(a2374, a2375)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m2376 {
            None => {
                Ok(Err::<Adt17, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f575({
                    let a2379 = {
                        let a2378 = {
                            let (_, h2377) = v3.clone();
                            h2377
                        };
                        length_bytes(a2378)
                    };
                    let a2380 = 1u64;
                    nat_add(a2379, a2380)?
                }, {
                    let (_, h2381) = v3;
                    h2381
                }, 0u64, Bytes::lit(&[]))?;
                let m2384 = {
                    let m2383 = f239(v0, {
                        let (h2382, _) = v3;
                        h2382
                    });
                    match m2383 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v5) => {
                            f239(v5, v4)
                        }
                    }
                };
                match m2384 {
                    None => {
                        Ok(Err::<Adt17, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" names no definition of this schema (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok(Ok::<Adt17, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f531(v0: u64, v1: Adt17, v2: List<Adt17>, v3: Adt17, v4: Bytes) -> R<Option<Bytes>> {
    let m2385 = v0;
    if m2385 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m2385 - 1;
        let m2386 = v2;
        match m2386.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f528(f484(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f531(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f532(v0: u64, v1: Adt17, v2: List<Adt17>, v3: Adt17, v4: Bytes) -> R<u64> {
    let m2387 = v0;
    if m2387 == 0 {
        Ok(0u64)
    } else {
        let v5 = m2387 - 1;
        let m2388 = v2;
        match m2388.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a2390 = {
                    let m2389 = f484(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m2389 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a2391 = f532(v5, v1, v7, v3, v4)?;
                nat_add(a2390, a2391)
            }
        }
    }
}

pub fn f533(v0: u64, v1: Adt17, v2: List<(Bytes, Adt17)>, v3: Adt17, v4: Bytes) -> R<Option<Bytes>> {
    let m2392 = v0;
    if m2392 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m2392 - 1;
        let m2393 = v2;
        match m2393.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f528({
                    let m2395 = f239(v3.clone(), {
                        let (h2394, _) = v6.clone();
                        h2394
                    });
                    match m2395 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f484(v5, v1.clone(), {
                                let (_, h2396) = v6;
                                h2396
                            }, v8, f576(v4.clone(), {
                                let (h2397, _) = v6;
                                h2397
                            })?)?
                        }
                    }
                }, f533(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f534(v0: u64, v1: Adt17, v2: Adt17, v3: List<(Bytes, Adt17)>, v4: Bytes) -> R<Option<Bytes>> {
    let m2398 = v0;
    if m2398 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m2398 - 1;
        let m2399 = v3;
        match m2399.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f576(v4.clone(), {
                    let (h2400, _) = v6.clone();
                    h2400
                })?;
                let m2402 = f577(f469(f239(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h2401, _) = v6.clone();
                    h2401
                })?;
                match m2402 {
                    Ok(v10) => {
                        Ok(f528(f531(v5, v1.clone(), v10.clone(), {
                            let (_, h2403) = v6.clone();
                            h2403
                        }, v8.clone())?, f528({
                            let m2404 = v10;
                            match m2404.uncons() {
                                None => {
                                    if f41({
                                        let (h2405, _) = v6;
                                        h2405
                                    }, f572(f510({
                                        let m2406 = f239(v2.clone(), f8(Str::lit("properties")));
                                        match m2406 {
                                            None => {
                                                Adt17::C7(std::rc::Rc::new(Adt17::C8))
                                            }
                                            Some(v13) => {
                                                v13
                                            }
                                        }
                                    }))) {
                                        None::<Bytes>
                                    } else {
                                        let m2407 = f239(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m2407 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f484(v5, v1.clone(), v14, {
                                                    let (_, h2408) = v6;
                                                    h2408
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f534(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f535(v0: u64, v1: Adt17, v2: Adt17, v3: List<Adt17>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m2409 = v0;
    if m2409 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m2409 - 1;
        let m2410 = v3;
        match m2410.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f528(f484(v6, v1.clone(), v2.clone(), v7, f576(v4.clone(), f355(v5)?)?)?, f535(v6, v1, v2, v8, v4, {
                    let a2411 = v5;
                    let a2412 = 1u64;
                    nat_add(a2411, a2412)?
                })?))
            }
        }
    }
}

pub fn f536(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2413, _) = v1.clone();
        h2413
    };
    let v3: Bytes = {
        let (h2414, _) = {
            let (_, h2415) = v1.clone();
            h2415
        };
        h2414
    };
    let a2418 = {
        let (_, h2416) = {
            let (_, h2417) = v1;
            h2417
        };
        h2416
    };
    let a2419 = 0u64;
    if nat_eq(a2418, a2419) {
        let v4: u64 = f350(v0.clone(), v2)?;
        let a2420 = v4;
        let a2421 = 34u64;
        if nat_eq(a2420, a2421) {
            Ok(Some(({
                let a2422 = v2;
                let a2423 = 1u64;
                nat_add(a2422, a2423)?
            }, (v3, 1u64))))
        } else {
            let a2424 = v4;
            let a2425 = 92u64;
            if nat_eq(a2424, a2425) {
                f578(v0, v2, v3)
            } else {
                let a2426 = v4;
                let a2427 = 32u64;
                if if nat_lt(a2426, a2427) {
                    true
                } else {
                    let a2428 = v4;
                    let a2429 = 256u64;
                    nat_eq(a2428, a2429)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2430 = v2;
                        let a2431 = 1u64;
                        nat_add(a2430, a2431)?
                    }, ({
                        let a2432 = v3;
                        let a2433 = f35(v0, v2, 1u64);
                        append_bytes(a2432, a2433)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f537(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f350(v0, v1)?;
    let a2434 = 47u64;
    let a2435 = v2;
    if if nat_lt(a2434, a2435) {
        let a2436 = v2;
        let a2437 = 58u64;
        nat_lt(a2436, a2437)
    } else {
        false
    } {
        Ok(Some({
            let a2438 = v1;
            let a2439 = 1u64;
            nat_add(a2438, a2439)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f538(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2440 = f350(v0, v1)?;
    let a2441 = 48u64;
    if nat_eq(a2440, a2441) {
        Ok(Some({
            let a2442 = v1;
            let a2443 = 1u64;
            nat_add(a2442, a2443)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f539(v0: Bytes) -> R<u64> {
    let (h2444, _) = f367(Fn6::F579(v0.clone()), {
        let a2445 = v0;
        length_bytes(a2445)
    }, 0u64)?;
    Ok(h2444)
}

pub fn f540(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2449 = {
        let a2447 = v0;
        let a2448 = {
            let (h2446, _) = v1;
            h2446
        };
        index_bytes(a2447, a2448)
    };
    match m2449 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2451 = {
                    let (h2450, _) = v1;
                    h2450
                };
                let a2452 = 1u64;
                nat_add(a2451, a2452)?
            }, {
                let a2456 = {
                    let a2454 = {
                        let (_, h2453) = v1;
                        h2453
                    };
                    let a2455 = 10u64;
                    nat_mul(a2454, a2455)?
                };
                let a2457 = f580(v2)?;
                nat_add(a2456, a2457)?
            })))
        }
    }
}

pub fn f541(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f581(v0, v1)
}

pub fn f542(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2458, _) = v2;
        h2458
    };
    let a2459 = v3;
    let a2460 = 0u64;
    if nat_eq(a2459, a2460) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2461 = v3;
            let a2462 = v1;
            let a2463 = 0u64;
            nat_quot(a2461, a2462, a2463)
        }, {
            let a2468 = f496(v0, {
                let a2464 = v3;
                let a2465 = v1;
                let a2466 = 0u64;
                nat_rem(a2464, a2465, a2466)
            });
            let a2469 = {
                let (_, h2467) = v2;
                h2467
            };
            append_bytes(a2468, a2469)
        }))
    }
}

pub fn f543(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2514 = {
            let a2490 = {
                let a2478 = {
                    let a2472 = {
                        let a2470 = v1;
                        let a2471 = 128u8;
                        fixed_u8::bit_and(a2470, a2471)
                    };
                    let a2473 = 128u8;
                    if equal(a2472, a2473) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2479 = {
                    let a2476 = {
                        let a2474 = v1;
                        let a2475 = 64u8;
                        fixed_u8::bit_and(a2474, a2475)
                    };
                    let a2477 = 64u8;
                    if equal(a2476, a2477) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2478, a2479)?
            };
            let a2491 = {
                let a2488 = {
                    let a2482 = {
                        let a2480 = v1;
                        let a2481 = 32u8;
                        fixed_u8::bit_and(a2480, a2481)
                    };
                    let a2483 = 32u8;
                    if equal(a2482, a2483) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2489 = {
                    let a2486 = {
                        let a2484 = v1;
                        let a2485 = 16u8;
                        fixed_u8::bit_and(a2484, a2485)
                    };
                    let a2487 = 16u8;
                    if equal(a2486, a2487) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2488, a2489)?
            };
            nat_add(a2490, a2491)?
        };
        let a2515 = {
            let a2512 = {
                let a2500 = {
                    let a2494 = {
                        let a2492 = v1;
                        let a2493 = 8u8;
                        fixed_u8::bit_and(a2492, a2493)
                    };
                    let a2495 = 8u8;
                    if equal(a2494, a2495) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2501 = {
                    let a2498 = {
                        let a2496 = v1;
                        let a2497 = 4u8;
                        fixed_u8::bit_and(a2496, a2497)
                    };
                    let a2499 = 4u8;
                    if equal(a2498, a2499) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2500, a2501)?
            };
            let a2513 = {
                let a2510 = {
                    let a2504 = {
                        let a2502 = v1;
                        let a2503 = 2u8;
                        fixed_u8::bit_and(a2502, a2503)
                    };
                    let a2505 = 2u8;
                    if equal(a2504, a2505) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2511 = {
                    let a2508 = {
                        let a2506 = v1;
                        let a2507 = 1u8;
                        fixed_u8::bit_and(a2506, a2507)
                    };
                    let a2509 = 1u8;
                    if equal(a2508, a2509) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2510, a2511)?
            };
            nat_add(a2512, a2513)?
        };
        nat_add(a2514, a2515)?
    };
    let a2522 = f496(f582(), {
        let a2516 = v2;
        let a2517 = 16u64;
        let a2518 = 0u64;
        nat_quot(a2516, a2517, a2518)
    });
    let a2523 = f496(f582(), {
        let a2519 = v2;
        let a2520 = 16u64;
        let a2521 = 0u64;
        nat_rem(a2519, a2520, a2521)
    });
    Ok(append_bytes(a2522, a2523))
}

pub fn f544(v0: Fn19, p2527: List<List<(Bytes, Bytes)>>, p2528: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2526 = (p2527, p2528);
    {
        let mut m2526 = m2526;
        loop {
            let step = {
                let (v1, v2) = m2526;
                let m2524 = v2;
                match m2524.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2525: Fn19 = v0.clone();
                            c2525.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f545(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f546(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2530 = {
        let a2529 = v0.clone();
        length_list(a2529)
    };
    let a2531 = 1u64;
    if nat_le(a2530, a2531) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f583(v0)?))
    }
}

pub fn f547(v0: Fn20, p2536: u64, p2537: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2535 = (p2536, p2537);
    {
        let mut m2535 = m2535;
        loop {
            let step = {
                let (v1, v2) = m2535;
                let m2532 = v1;
                if m2532 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2532 - 1;
                    let m2534 = {
                        let c2533: Fn20 = v0.clone();
                        c2533.apply(v2.clone())?
                    };
                    match m2534 {
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
                Ok(next) => { m2535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f584(v0)
}

pub fn f549(p2540: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2539 = p2540;
    {
        let mut m2539 = m2539;
        loop {
            let step = {
                let v0 = m2539;
                let m2538 = v0;
                match m2538.uncons() {
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
                Ok(next) => { m2539 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: Adt17, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2541 = v0;
    match m2541 {
        Adt17::C0 => {
            Ok(v1)
        }
        Adt17::C1(_, _) => {
            Ok(v1)
        }
        Adt17::C2(_) => {
            Ok(v1)
        }
        Adt17::C3(_, _) => {
            Ok(v1)
        }
        Adt17::C4(_, _, _) => {
            Ok(v1)
        }
        Adt17::C5(_) => {
            Ok(v1)
        }
        Adt17::C6(r2544) => {
            let v2 = (*r2544).clone();
            f550(v2, v1)
        }
        Adt17::C7(r2545) => {
            let v3 = (*r2545).clone();
            f550(v3, v1)
        }
        Adt17::C8 => {
            Ok(v1)
        }
        Adt17::C9(r2546, r2547) => {
            let v4 = (*r2546).clone();
            let v5 = (*r2547).clone();
            f550(v4, f550(v5, v1)?)
        }
        Adt17::C10(v6, r2548, r2549) => {
            let v7 = (*r2548).clone();
            let v8 = (*r2549).clone();
            let v9: List<Bytes> = f550(v7.clone(), f550(v8, v1)?)?;
            if f28(v6, f8(Str::lit("$ref"))) {
                let m2542 = f336(Some(v7));
                match m2542 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2543 = f585(v10)?;
                        match m2543 {
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

pub fn f551() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f552(v0: Bytes) -> R<Bytes> {
    let m2552 = {
        let a2550 = v0.clone();
        let a2551 = 0u64;
        index_bytes(a2550, a2551)
    };
    match m2552 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2558 = {
                let a2553 = f8(Str::lit("u"));
                let a2554 = f586(8u64, f62(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2553, a2554)
            };
            let a2559 = f35(v0.clone(), 1u64, {
                let a2556 = {
                    let a2555 = v0;
                    length_bytes(a2555)
                };
                let a2557 = 1u64;
                nat_sub(a2556, a2557)
            });
            Ok(append_bytes(a2558, a2559))
        }
    }
}

pub fn f553(v0: Bytes, v1: Adt17) -> R<Result<Adt28, Bytes>> {
    let m2560 = f239(v1.clone(), f8(Str::lit("enum")));
    match m2560 {
        None => {
            let m2562 = f239(v1.clone(), f8(Str::lit("oneOf")));
            match m2562 {
                None => {
                    if f591(v1.clone(), v0.clone())? {
                        let m2563 = f592(v0.clone(), f469(f239(v1.clone(), f8(Str::lit("properties")))), f243(f244(f239(v1, f8(Str::lit("required"))))), true)?;
                        match m2563 {
                            Ok(v7) => {
                                Ok(f589(f593(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f346(f594(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f595(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2564 = f469(f239(v1.clone(), f8(Str::lit("properties"))));
                        match m2564.uncons() {
                            None => {
                                if f596(v1.clone()) {
                                    Ok(f589(f597(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2567 = f598(v1, {
                                        let a2565 = v0.clone();
                                        let a2566 = f8(Str::lit("Value"));
                                        append_bytes(a2565, a2566)
                                    })?;
                                    match m2567 {
                                        Ok(v9) => {
                                            Ok(f589(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f518(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f517(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2568 = f592(v0.clone(), f469(f239(v1.clone(), f8(Str::lit("properties")))), f243(f244(f239(v1, f8(Str::lit("required"))))), false)?;
                                match m2568 {
                                    Ok(v13) => {
                                        Ok(f589(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2569 = f594(v13.clone());
                                            match m2569.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f346(f594(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f595(v13)))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<Adt28, Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f590(v0, f337(v5))
                }
            }
        }
        Some(v2) => {
            let m2561 = f587(f337(v2), List::<Bytes>::nil())?;
            match m2561 {
                Ok(v4) => {
                    Ok(f589(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f346(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f588(v3, v0))
                }
            }
        }
    }
}

pub fn f554(v0: Adt17) -> Adt17 {
    let v2: Adt17 = {
        let m2570 = f239(v0.clone(), f8(Str::lit("anyOf")));
        match m2570 {
            None => {
                v0
            }
            Some(v1) => {
                if f599(f337(v1.clone())) {
                    f600(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f601(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f602(f337(v1.clone()))
                    } {
                        v0
                    } else {
                        f603(f600(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt17 = {
        let m2571 = f239(v2.clone(), f8(Str::lit("oneOf")));
        match m2571 {
            None => {
                v2
            }
            Some(v3) => {
                if f599(f337(v3)) {
                    f600(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2572 = f239(v4.clone(), f8(Str::lit("allOf")));
    match m2572 {
        None => {
            v4
        }
        Some(v5) => {
            if f604(f337(v5)) {
                f600(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f555(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f605(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2573) => {
            h2573
        }
    });
    let v2: u32 = f522(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2574, _, _) => {
            h2574
        }
    }, f522(f606(match v0.clone() {
        Adt27::C0(_, _, _, _, h2575, _, _, _, _, _) => {
            h2575
        }
    }), f522(f607(match v0.clone() {
        Adt27::C0(_, _, _, _, h2576, _, _, _, _, _) => {
            h2576
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2577, _, _, _, _) => {
            h2577
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2578, _, _, _) => {
            h2578
        }
    }), f522(v1, {
        let m2581 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2580, _) => {
                h2580
            }
        };
        match m2581 {
            Adt26::C0(h2579, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2579
            }
        }
    }))));
    let v3: u32 = f522(f608(match v0.clone() {
        Adt27::C0(h2582, _, _, _, _, _, _, _, _, _) => {
            h2582
        }
    }), f609(match v0.clone() {
        Adt27::C0(h2583, _, _, _, _, _, _, _, _, _) => {
            h2583
        }
    }, match v0.clone() {
        Adt27::C0(_, h2584, _, _, _, _, _, _, _, _) => {
            h2584
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2585, _, _, _, _, _, _, _) => {
            h2585
        }
    }));
    Ok(Adt27::C0(f522(v2, v3), match v0.clone() {
        Adt27::C0(h2586, _, _, _, _, _, _, _, _, _) => {
            h2586
        }
    }, match v0.clone() {
        Adt27::C0(_, h2587, _, _, _, _, _, _, _, _) => {
            h2587
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2588, _, _, _, _, _, _, _) => {
            h2588
        }
    }, f522(match v0.clone() {
        Adt27::C0(_, _, _, h2589, _, _, _, _, _, _) => {
            h2589
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2590, _, _, _, _, _) => {
            h2590
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2591, _, _, _, _) => {
            h2591
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2592, _, _, _) => {
            h2592
        }
    }, f610(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2593, _) => {
            h2593
        }
    }), {
        let a2595 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2594) => {
                h2594
            }
        };
        let a2596 = 1u64;
        nat_add(a2595, a2596)?
    }))
}

pub fn f556(v0: u32) -> u64 {
    let m2598 = {
        let a2597 = v0;
        fixed_u64::convert(i128::from(a2597))
    };
    match m2598 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f557(v0: u64) -> u32 {
    let m2602 = {
        let a2601 = {
            let a2599 = v0;
            let a2600 = 4294967295u64;
            fixed_u64::bit_and(a2599, a2600)
        };
        fixed_u32::convert(i128::from(a2601))
    };
    match m2602 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f558(v0: u64) -> u32 {
    let (_, h2603) = {
        let (_, h2604) = f611(Fn21::F612, 32u64, (v0, (1u32, 0u32)));
        h2604
    };
    h2603
}

pub fn f559(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2609 = f613(v0, 24u32);
    let a2610 = {
        let a2607 = f613(v1, 16u32);
        let a2608 = {
            let a2605 = f613(v2, 8u32);
            let a2606 = v3;
            fixed_u32::bit_or(a2605, a2606)
        };
        fixed_u32::bit_or(a2607, a2608)
    };
    fixed_u32::bit_or(a2609, a2610)
}

pub fn f560(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2611 = v2;
    let a2612 = v1;
    if nat_lt(a2611, a2612) {
        let m2615 = {
            let a2613 = v0;
            let a2614 = v2;
            index_bytes(a2613, a2614)
        };
        match m2615 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2617 = {
                    let a2616 = v3;
                    fixed_u32::convert(i128::from(a2616))
                };
                match m2617 {
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
        let a2618 = v2;
        let a2619 = v1;
        if nat_eq(a2618, a2619) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f561() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f562(v0: Adt17) -> Bytes {
    let m2620 = v0;
    match m2620 {
        Adt17::C0 => {
            f8(Str::lit("null"))
        }
        Adt17::C1(_, _) => {
            f8(Str::lit("tagged"))
        }
        Adt17::C2(_) => {
            f8(Str::lit("boolean"))
        }
        Adt17::C3(_, _) => {
            f8(Str::lit("integer"))
        }
        Adt17::C4(_, _, _) => {
            f8(Str::lit("number"))
        }
        Adt17::C5(_) => {
            f8(Str::lit("string"))
        }
        Adt17::C6(_) => {
            f8(Str::lit("array"))
        }
        Adt17::C7(_) => {
            f8(Str::lit("object"))
        }
        Adt17::C8 => {
            f8(Str::lit("spine"))
        }
        Adt17::C9(_, _) => {
            f8(Str::lit("spine"))
        }
        Adt17::C10(_, _, _) => {
            f8(Str::lit("spine"))
        }
    }
}

pub fn f563(p2623: List<Bytes>, v1: Adt17) -> bool {
    let m2622 = p2623;
    {
        let mut m2622 = m2622;
        loop {
            let step = {
                let v0 = m2622;
                let m2621 = v0;
                match m2621.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f614(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2622 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f564(p2626: List<Adt17>, v1: Adt17) -> R<bool> {
    let m2625 = p2626;
    {
        let mut m2625 = m2625;
        loop {
            let step = {
                let v0 = m2625;
                let m2624 = v0;
                match m2624.uncons() {
                    None => {
                        Err::<List<Adt17>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f565(false, v2, v1.clone())? {
                            Err::<List<Adt17>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt17>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f565(v0: bool, v1: Adt17, v2: Adt17) -> R<bool> {
    let m2627 = v1;
    match m2627 {
        Adt17::C0 => {
            Ok(f28(f485(v2)?, f8(Str::lit("null"))))
        }
        Adt17::C1(_, _) => {
            Ok(false)
        }
        Adt17::C2(v77) => {
            Ok(f28(f485(v2)?, f485(Adt17::C2(v77))?))
        }
        Adt17::C3(v78, v79) => {
            Ok(f28(f485(v2)?, f485(Adt17::C3(v78, v79))?))
        }
        Adt17::C4(v80, v81, v82) => {
            Ok(f28(f485(v2)?, f485(Adt17::C4(v80, v81, v82))?))
        }
        Adt17::C5(v83) => {
            let m2639 = v2;
            match m2639 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(v84) => {
                    Ok(f28(v83, v84))
                }
                Adt17::C6(_) => {
                    Ok(false)
                }
                Adt17::C7(_) => {
                    Ok(false)
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(_, _) => {
                    Ok(false)
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C6(r2640) => {
            let v58 = (*r2640).clone();
            let m2637 = v2;
            match m2637 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(_) => {
                    Ok(false)
                }
                Adt17::C6(r2638) => {
                    let v59 = (*r2638).clone();
                    f565(false, v58, v59)
                }
                Adt17::C7(_) => {
                    Ok(false)
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(_, _) => {
                    Ok(false)
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C7(r2641) => {
            let v41 = (*r2641).clone();
            let m2633 = v2;
            match m2633 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(_) => {
                    Ok(false)
                }
                Adt17::C6(_) => {
                    Ok(false)
                }
                Adt17::C7(r2636) => {
                    let v42 = (*r2636).clone();
                    if f228({
                        let a2634 = f570(v41.clone())?;
                        let a2635 = f570(v42.clone())?;
                        nat_eq(a2634, a2635)
                    }, true) {
                        f565(true, v41, v42)
                    } else {
                        Ok(false)
                    }
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(_, _) => {
                    Ok(false)
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C8 => {
            if v0 {
                Ok(true)
            } else {
                let m2629 = v2;
                match m2629 {
                    Adt17::C0 => {
                        Ok(false)
                    }
                    Adt17::C1(_, _) => {
                        Ok(false)
                    }
                    Adt17::C2(_) => {
                        Ok(false)
                    }
                    Adt17::C3(_, _) => {
                        Ok(false)
                    }
                    Adt17::C4(_, _, _) => {
                        Ok(false)
                    }
                    Adt17::C5(_) => {
                        Ok(false)
                    }
                    Adt17::C6(_) => {
                        Ok(false)
                    }
                    Adt17::C7(_) => {
                        Ok(false)
                    }
                    Adt17::C8 => {
                        Ok(true)
                    }
                    Adt17::C9(_, _) => {
                        Ok(false)
                    }
                    Adt17::C10(_, _, _) => {
                        Ok(false)
                    }
                }
            }
        }
        Adt17::C9(r2642, r2643) => {
            let v23 = (*r2642).clone();
            let v24 = (*r2643).clone();
            let m2630 = v2;
            match m2630 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(_) => {
                    Ok(false)
                }
                Adt17::C6(_) => {
                    Ok(false)
                }
                Adt17::C7(_) => {
                    Ok(false)
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(r2631, r2632) => {
                    let v25 = (*r2631).clone();
                    let v26 = (*r2632).clone();
                    if f565(false, v23, v25)? {
                        f565(false, v24, v26)
                    } else {
                        Ok(false)
                    }
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C10(v3, r2644, r2645) => {
            let v4 = (*r2644).clone();
            let v5 = (*r2645).clone();
            let m2628 = f239(v2.clone(), v3);
            if match m2628 {
                None => {
                    false
                }
                Some(v6) => {
                    f565(false, v4, v6)?
                }
            } {
                f565(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f566(v0: Bytes) -> R<Option<List<u64>>> {
    f615({
        let a2647 = {
            let a2646 = v0.clone();
            length_bytes(a2646)
        };
        let a2648 = 1u64;
        nat_add(a2647, a2648)?
    }, v0, 0u64)
}

pub fn f567(v0: List<u64>) -> R<u64> {
    let m2649 = v0;
    match m2649.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2650 = f567(v2)?;
            let a2651 = 1u64;
            nat_add(a2650, a2651)
        }
    }
}

pub fn f568(v0: Option<Adt17>) -> Option<u64> {
    let m2652 = v0;
    match m2652 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2653 = v1;
            match m2653 {
                Adt17::C0 => {
                    None::<u64>
                }
                Adt17::C1(_, _) => {
                    None::<u64>
                }
                Adt17::C2(_) => {
                    None::<u64>
                }
                Adt17::C3(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt17::C4(_, _, _) => {
                    None::<u64>
                }
                Adt17::C5(_) => {
                    None::<u64>
                }
                Adt17::C6(_) => {
                    None::<u64>
                }
                Adt17::C7(_) => {
                    None::<u64>
                }
                Adt17::C8 => {
                    None::<u64>
                }
                Adt17::C9(_, _) => {
                    None::<u64>
                }
                Adt17::C10(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f569(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m2654 = f616(v0)?;
    match m2654 {
        Ok(v3) => {
            let m2655 = f566(v1)?;
            match m2655 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h2656, _) = {
                            let (h2657, _) = v3.clone();
                            h2657
                        };
                        if h2656 {
                            f617({
                                let a2661 = {
                                    let a2659 = f618({
                                        let (_, h2658) = v3.clone();
                                        h2658
                                    })?;
                                    let a2660 = f567(v4.clone())?;
                                    nat_add(a2659, a2660)?
                                };
                                let a2662 = 2u64;
                                nat_add(a2661, a2662)?
                            }, {
                                let (_, h2663) = v3;
                                h2663
                            }, v4, {
                                let (_, h2664) = {
                                    let (h2665, _) = v3;
                                    h2665
                                };
                                h2664
                            })?
                        } else {
                            f619({
                                let (_, h2666) = v3;
                                h2666
                            }, v4, {
                                let (_, h2667) = {
                                    let (h2668, _) = v3;
                                    h2668
                                };
                                h2667
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

pub fn f570(v0: Adt17) -> R<u64> {
    let m2669 = v0;
    match m2669 {
        Adt17::C0 => {
            Ok(0u64)
        }
        Adt17::C1(_, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(_, _) => {
            Ok(0u64)
        }
        Adt17::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C5(_) => {
            Ok(0u64)
        }
        Adt17::C6(_) => {
            Ok(0u64)
        }
        Adt17::C7(_) => {
            Ok(0u64)
        }
        Adt17::C8 => {
            Ok(0u64)
        }
        Adt17::C9(_, r2674) => {
            let v5 = (*r2674).clone();
            let a2672 = f570(v5)?;
            let a2673 = 1u64;
            nat_add(a2672, a2673)
        }
        Adt17::C10(_, _, r2675) => {
            let v3 = (*r2675).clone();
            let a2670 = f570(v3)?;
            let a2671 = 1u64;
            nat_add(a2670, a2671)
        }
    }
}

pub fn f571(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2676 = v0;
    match m2676.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f571(v3, v1)
            } else {
                List::cons(v2, f571(v3, v1))
            }
        }
    }
}

pub fn f572(v0: List<(Bytes, Adt17)>) -> List<Bytes> {
    let m2677 = v0;
    match m2677.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2678, _) = v1;
                h2678
            }, f572(v2))
        }
    }
}

pub fn f573(v0: Adt17) -> Option<(bool, (u64, u64))> {
    let m2679 = v0;
    match m2679 {
        Adt17::C0 => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C1(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C2(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C3(v1, v2) => {
            Some((v1, (v2, 0u64)))
        }
        Adt17::C4(v3, v4, v5) => {
            Some((v3, (v4, v5)))
        }
        Adt17::C5(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C6(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C7(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C8 => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C9(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C10(_, _, _) => {
            None::<(bool, (u64, u64))>
        }
    }
}

pub fn f574(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a2684 = {
            let (h2680, _) = {
                let (_, h2681) = v0;
                h2681
            };
            h2680
        };
        let a2685 = f620({
            let (_, h2682) = {
                let (_, h2683) = v1;
                h2683
            };
            h2682
        })?;
        nat_mul(a2684, a2685)?
    };
    let v3: u64 = {
        let a2690 = {
            let (h2686, _) = {
                let (_, h2687) = v1;
                h2687
            };
            h2686
        };
        let a2691 = f620({
            let (_, h2688) = {
                let (_, h2689) = v0;
                h2689
            };
            h2688
        })?;
        nat_mul(a2690, a2691)?
    };
    let (h2692, _) = v0;
    if h2692 {
        let (h2693, _) = v1;
        if h2693 {
            let a2694 = v3;
            let a2695 = v2;
            Ok(nat_lt(a2694, a2695))
        } else {
            Ok(true)
        }
    } else {
        let (h2696, _) = v1;
        if h2696 {
            Ok(false)
        } else {
            let a2697 = v2;
            let a2698 = v3;
            Ok(nat_lt(a2697, a2698))
        }
    }
}

pub fn f575(p2728: u64, v1: Bytes, p2729: u64, p2730: Bytes) -> R<Bytes> {
    let m2727 = (p2728, (p2729, p2730));
    {
        let mut m2727 = m2727;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2727;
                let m2699 = v0;
                if m2699 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2699 - 1;
                    let a2701 = {
                        let a2700 = v1.clone();
                        length_bytes(a2700)
                    };
                    let a2702 = v2;
                    if nat_le(a2701, a2702) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f228({
                            let a2703 = f350(v1.clone(), v2)?;
                            let a2704 = 126u64;
                            nat_eq(a2703, a2704)
                        }, {
                            let a2707 = f350(v1.clone(), {
                                let a2705 = v2;
                                let a2706 = 1u64;
                                nat_add(a2705, a2706)?
                            })?;
                            let a2708 = 48u64;
                            nat_eq(a2707, a2708)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a2709 = v2;
                                let a2710 = 2u64;
                                nat_add(a2709, a2710)?
                            }, {
                                let a2711 = v3;
                                let a2712 = f8(Str::lit("~"));
                                append_bytes(a2711, a2712)
                            })))
                        } else {
                            if f228({
                                let a2713 = f350(v1.clone(), v2)?;
                                let a2714 = 126u64;
                                nat_eq(a2713, a2714)
                            }, {
                                let a2717 = f350(v1.clone(), {
                                    let a2715 = v2;
                                    let a2716 = 1u64;
                                    nat_add(a2715, a2716)?
                                })?;
                                let a2718 = 49u64;
                                nat_eq(a2717, a2718)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2719 = v2;
                                    let a2720 = 2u64;
                                    nat_add(a2719, a2720)?
                                }, {
                                    let a2721 = v3;
                                    let a2722 = f8(Str::lit("/"));
                                    append_bytes(a2721, a2722)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2723 = v2;
                                    let a2724 = 1u64;
                                    nat_add(a2723, a2724)?
                                }, {
                                    let a2725 = v3;
                                    let a2726 = f35(v1.clone(), v2, 1u64);
                                    append_bytes(a2725, a2726)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2727 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f576(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f621({
        let a2732 = {
            let a2731 = v1.clone();
            length_bytes(a2731)
        };
        let a2733 = 1u64;
        nat_add(a2732, a2733)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f577(v0: List<(Bytes, Adt17)>, v1: Bytes) -> R<Result<List<Adt17>, Bytes>> {
    let m2734 = v0;
    match m2734.uncons() {
        None => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
        Some((v2, v3)) => {
            let m2736 = f569({
                let (h2735, _) = v2;
                h2735
            }, v1.clone())?;
            match m2736 {
                Ok(v5) => {
                    let m2737 = f577(v3, v1)?;
                    match m2737 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt17>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h2738) = v2;
                                    h2738
                                }, v7)
                            } else {
                                v7
                            }))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt17>, Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt17>, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f578(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f350(v0.clone(), {
        let a2739 = v1;
        let a2740 = 1u64;
        nat_add(a2739, a2740)?
    })?;
    let v4: Bytes = {
        let a2741 = v3;
        let a2742 = 117u64;
        if nat_eq(a2741, a2742) {
            f622(v0, v1)?
        } else {
            f623(v3)
        }
    };
    let a2743 = v4.clone();
    let a2744 = Bytes::lit(&[]);
    if equal(a2743, a2744) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2745 = v3;
            let a2746 = 117u64;
            if nat_eq(a2745, a2746) {
                let a2747 = v1;
                let a2748 = 6u64;
                nat_add(a2747, a2748)?
            } else {
                let a2749 = v1;
                let a2750 = 2u64;
                nat_add(a2749, a2750)?
            }
        }, ({
            let a2751 = v2;
            let a2752 = v4;
            append_bytes(a2751, a2752)
        }, 0u64))))
    }
}

pub fn f579(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2755 = {
        let a2753 = v0;
        let a2754 = v1;
        index_bytes(a2753, a2754)
    };
    match m2755 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f624(v2)? {
                Ok(Some({
                    let a2756 = v1;
                    let a2757 = 1u64;
                    nat_add(a2756, a2757)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f580(v0: u8) -> R<u64> {
    let a2804 = {
        let v1: u8 = v0;
        let a2802 = {
            let a2778 = {
                let a2766 = {
                    let a2760 = {
                        let a2758 = v1;
                        let a2759 = 128u8;
                        fixed_u8::bit_and(a2758, a2759)
                    };
                    let a2761 = 128u8;
                    if equal(a2760, a2761) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2767 = {
                    let a2764 = {
                        let a2762 = v1;
                        let a2763 = 64u8;
                        fixed_u8::bit_and(a2762, a2763)
                    };
                    let a2765 = 64u8;
                    if equal(a2764, a2765) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2766, a2767)?
            };
            let a2779 = {
                let a2776 = {
                    let a2770 = {
                        let a2768 = v1;
                        let a2769 = 32u8;
                        fixed_u8::bit_and(a2768, a2769)
                    };
                    let a2771 = 32u8;
                    if equal(a2770, a2771) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2777 = {
                    let a2774 = {
                        let a2772 = v1;
                        let a2773 = 16u8;
                        fixed_u8::bit_and(a2772, a2773)
                    };
                    let a2775 = 16u8;
                    if equal(a2774, a2775) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2776, a2777)?
            };
            nat_add(a2778, a2779)?
        };
        let a2803 = {
            let a2800 = {
                let a2788 = {
                    let a2782 = {
                        let a2780 = v1;
                        let a2781 = 8u8;
                        fixed_u8::bit_and(a2780, a2781)
                    };
                    let a2783 = 8u8;
                    if equal(a2782, a2783) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2789 = {
                    let a2786 = {
                        let a2784 = v1;
                        let a2785 = 4u8;
                        fixed_u8::bit_and(a2784, a2785)
                    };
                    let a2787 = 4u8;
                    if equal(a2786, a2787) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2788, a2789)?
            };
            let a2801 = {
                let a2798 = {
                    let a2792 = {
                        let a2790 = v1;
                        let a2791 = 2u8;
                        fixed_u8::bit_and(a2790, a2791)
                    };
                    let a2793 = 2u8;
                    if equal(a2792, a2793) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2799 = {
                    let a2796 = {
                        let a2794 = v1;
                        let a2795 = 1u8;
                        fixed_u8::bit_and(a2794, a2795)
                    };
                    let a2797 = 1u8;
                    if equal(a2796, a2797) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2798, a2799)?
            };
            nat_add(a2800, a2801)?
        };
        nat_add(a2802, a2803)?
    };
    let a2805 = 48u64;
    Ok(nat_sub(a2804, a2805))
}

pub fn f581(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2806 = v0.clone();
        length_bytes(a2806)
    };
    let a2807 = v1;
    let a2808 = v2;
    if if nat_lt(a2807, a2808) {
        let a2813 = f350(v0, {
            let a2811 = v2;
            let a2812 = {
                let a2809 = v1;
                let a2810 = 1u64;
                nat_add(a2809, a2810)?
            };
            nat_sub(a2811, a2812)
        })?;
        let a2814 = 48u64;
        nat_eq(a2813, a2814)
    } else {
        false
    } {
        Ok(Some({
            let a2815 = v1;
            let a2816 = 1u64;
            nat_add(a2815, a2816)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f582() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f583(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f499({
        let (_, h2817) = {
            let (h2818, _) = f625(Fn22::F626, {
                let a2820 = {
                    let a2819 = v0.clone();
                    length_list(a2819)
                };
                let a2821 = 1u64;
                nat_add(a2820, a2821)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2818
        };
        h2817
    }))
}

pub fn f584(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2822, _) = v0;
        h2822
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2823) = v0;
        h2823
    };
    let m2824 = v1;
    match m2824.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2825 = v4;
            match m2825.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f627(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f585(v0: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f513(f35(v0.clone(), 8u64, {
            let a2827 = {
                let a2826 = v0;
                length_bytes(a2826)
            };
            let a2828 = 8u64;
            nat_sub(a2827, a2828)
        }))?))
    } else {
        if f72(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f513(f35(v0.clone(), 14u64, {
                let a2830 = {
                    let a2829 = v0;
                    length_bytes(a2829)
                };
                let a2831 = 14u64;
                nat_sub(a2830, a2831)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f586(p2848: u64, p2849: u64, p2850: u64, p2851: Bytes) -> Bytes {
    let m2847 = (p2848, (p2849, (p2850, p2851)));
    {
        let mut m2847 = m2847;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2847;
                let m2832 = v0;
                if m2832 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2832 - 1;
                    if f228({
                        let a2833 = v1;
                        let a2834 = 0u64;
                        nat_eq(a2833, a2834)
                    }, {
                        let a2835 = v2;
                        let a2836 = 0u64;
                        nat_eq(a2835, a2836)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2837 = v1;
                            let a2838 = 16u64;
                            let a2839 = 0u64;
                            nat_quot(a2837, a2838, a2839)
                        }, ({
                            let a2840 = v2;
                            let a2841 = 1u64;
                            nat_sub(a2840, a2841)
                        }, {
                            let a2845 = f628({
                                let a2842 = v1;
                                let a2843 = 16u64;
                                let a2844 = 0u64;
                                nat_rem(a2842, a2843, a2844)
                            });
                            let a2846 = v3;
                            append_bytes(a2845, a2846)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(v0: List<Adt17>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f629(f630(f243(v0.clone()))?, v0, v1)
}

pub fn f588(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f589(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f590(v0: Bytes, v1: List<Adt17>) -> R<Result<Adt28, Bytes>> {
    let m2852 = v1.clone();
    match m2852.uncons() {
        None => {
            Ok(f588(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2853 = f631(f469(f239(v2, f8(Str::lit("properties")))), v1.clone());
            match m2853.uncons() {
                None => {
                    let m2854 = f632(f630(f633(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2854 {
                        Ok(v5) => {
                            Ok(f589(f593(v0, f594(v5.clone())), f595(v5)))
                        }
                        Err(v4) => {
                            Ok(f588(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2855 = f634(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2855 {
                        Ok(v9) => {
                            Ok(f589(f593(v0, f594(v9.clone())), f595(v9)))
                        }
                        Err(v8) => {
                            Ok(f588(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f591(v0: Adt17, v1: Bytes) -> R<bool> {
    let m2856 = v0;
    match m2856 {
        Adt17::C0 => {
            Ok(false)
        }
        Adt17::C1(_, _) => {
            Ok(false)
        }
        Adt17::C2(_) => {
            Ok(false)
        }
        Adt17::C3(_, _) => {
            Ok(false)
        }
        Adt17::C4(_, _, _) => {
            Ok(false)
        }
        Adt17::C5(_) => {
            Ok(false)
        }
        Adt17::C6(r2857) => {
            let v2 = (*r2857).clone();
            f591(v2, v1)
        }
        Adt17::C7(r2858) => {
            let v3 = (*r2858).clone();
            f591(v3, v1)
        }
        Adt17::C8 => {
            Ok(false)
        }
        Adt17::C9(r2859, r2860) => {
            let v4 = (*r2859).clone();
            let v5 = (*r2860).clone();
            if f591(v4, v1.clone())? {
                Ok(true)
            } else {
                f591(v5, v1)
            }
        }
        Adt17::C10(v6, r2861, r2862) => {
            let v7 = (*r2861).clone();
            let v8 = (*r2862).clone();
            if if f28(v6, f8(Str::lit("$ref"))) {
                f635(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f591(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f591(v8, v1)
                }
            }
        }
    }
}

pub fn f592(v0: Bytes, v1: List<(Bytes, Adt17)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f636(f630(f572(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f593(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f346(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f594(v0: Adt29) -> List<Bytes> {
    let m2863 = v0;
    match m2863 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f595(v0: Adt29) -> List<Adt25> {
    let m2864 = v0;
    match m2864 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f596(v0: Adt17) -> bool {
    let m2865 = f469(f239(v0.clone(), f8(Str::lit("properties"))));
    match m2865.uncons() {
        None => {
            let m2866 = f637(v0);
            match m2866 {
                None => {
                    false
                }
                Some(v3) => {
                    f638(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f597(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f598(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2867 = f639(v0.clone());
    match m2867 {
        None => {
            if f28(f242(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2868 = f239(v0, f8(Str::lit("items")));
                match m2868 {
                    None => {
                        Ok(f640(f8(Str::lit("List")), f642()))
                    }
                    Some(v3) => {
                        Ok(f640(f8(Str::lit("List")), f641(v3, {
                            let a2869 = v1;
                            let a2870 = f8(Str::lit("Item"));
                            append_bytes(a2869, a2870)
                        })?))
                    }
                }
            } else {
                f641(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f640(f8(Str::lit("Option")), f641(v2, v1)?))
        }
    }
}

pub fn f599(p2873: List<Adt17>) -> bool {
    let m2872 = p2873;
    {
        let mut m2872 = m2872;
        loop {
            let step = {
                let v0 = m2872;
                let m2871 = v0;
                match m2871.uncons() {
                    None => {
                        Err::<List<Adt17>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f643(f510(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt17>, bool>(v2)
                        } else {
                            Err::<List<Adt17>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2872 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(v0: Adt17, v1: Bytes) -> Adt17 {
    let m2874 = v0;
    match m2874 {
        Adt17::C0 => {
            Adt17::C0
        }
        Adt17::C1(v6, r2875) => {
            let v7 = (*r2875).clone();
            Adt17::C1(v6, std::rc::Rc::new(v7))
        }
        Adt17::C2(v8) => {
            Adt17::C2(v8)
        }
        Adt17::C3(v12, v13) => {
            Adt17::C3(v12, v13)
        }
        Adt17::C4(v9, v10, v11) => {
            Adt17::C4(v9, v10, v11)
        }
        Adt17::C5(v14) => {
            Adt17::C5(v14)
        }
        Adt17::C6(r2876) => {
            let v15 = (*r2876).clone();
            Adt17::C6(std::rc::Rc::new(v15))
        }
        Adt17::C7(r2877) => {
            let v5 = (*r2877).clone();
            Adt17::C7(std::rc::Rc::new(f600(v5, v1)))
        }
        Adt17::C8 => {
            Adt17::C8
        }
        Adt17::C9(r2878, r2879) => {
            let v16 = (*r2878).clone();
            let v17 = (*r2879).clone();
            Adt17::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt17::C10(v2, r2880, r2881) => {
            let v3 = (*r2880).clone();
            let v4 = (*r2881).clone();
            if f28(v2.clone(), v1.clone()) {
                f600(v4, v1)
            } else {
                Adt17::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f600(v4, v1)))
            }
        }
    }
}

pub fn f601(v0: Adt17, v1: Bytes) -> bool {
    let m2882 = f239(v0, v1);
    match m2882 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f602(v0: List<Adt17>) -> bool {
    let m2883 = v0.clone();
    match m2883.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2884 = f631(f469(f239(v1, f8(Str::lit("properties")))), v0.clone());
            match m2884.uncons() {
                None => {
                    f645(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f644(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f603(v0: Adt17, v1: Bytes, v2: Adt17) -> Adt17 {
    let m2885 = v0;
    match m2885 {
        Adt17::C0 => {
            Adt17::C0
        }
        Adt17::C1(v7, r2886) => {
            let v8 = (*r2886).clone();
            Adt17::C1(v7, std::rc::Rc::new(v8))
        }
        Adt17::C2(v9) => {
            Adt17::C2(v9)
        }
        Adt17::C3(v13, v14) => {
            Adt17::C3(v13, v14)
        }
        Adt17::C4(v10, v11, v12) => {
            Adt17::C4(v10, v11, v12)
        }
        Adt17::C5(v15) => {
            Adt17::C5(v15)
        }
        Adt17::C6(r2887) => {
            let v16 = (*r2887).clone();
            Adt17::C6(std::rc::Rc::new(v16))
        }
        Adt17::C7(r2888) => {
            let v3 = (*r2888).clone();
            Adt17::C7(std::rc::Rc::new(Adt17::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt17::C8 => {
            Adt17::C8
        }
        Adt17::C9(r2889, r2890) => {
            let v17 = (*r2889).clone();
            let v18 = (*r2890).clone();
            Adt17::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt17::C10(v4, r2891, r2892) => {
            let v5 = (*r2891).clone();
            let v6 = (*r2892).clone();
            Adt17::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f604(p2895: List<Adt17>) -> bool {
    let m2894 = p2895;
    {
        let mut m2894 = m2894;
        loop {
            let step = {
                let v0 = m2894;
                let m2893 = v0;
                match m2893.uncons() {
                    None => {
                        Err::<List<Adt17>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f646(v1) {
                            Ok::<List<Adt17>, bool>(v2)
                        } else {
                            Err::<List<Adt17>, bool>(false)
                        }
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

pub fn f605(v0: u64) -> u32 {
    let a2896 = v0;
    let a2897 = 32u64;
    if nat_lt(a2896, a2897) {
        let a2898 = v0;
        let a2899 = 16u64;
        if nat_lt(a2898, a2899) {
            let a2900 = v0;
            let a2901 = 8u64;
            if nat_lt(a2900, a2901) {
                let a2902 = v0;
                let a2903 = 4u64;
                if nat_lt(a2902, a2903) {
                    let a2904 = v0;
                    let a2905 = 2u64;
                    if nat_lt(a2904, a2905) {
                        let a2906 = v0;
                        let a2907 = 1u64;
                        if nat_lt(a2906, a2907) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2908 = v0;
                        let a2909 = 3u64;
                        if nat_lt(a2908, a2909) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2910 = v0;
                    let a2911 = 6u64;
                    if nat_lt(a2910, a2911) {
                        let a2912 = v0;
                        let a2913 = 5u64;
                        if nat_lt(a2912, a2913) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2914 = v0;
                        let a2915 = 7u64;
                        if nat_lt(a2914, a2915) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2916 = v0;
                let a2917 = 12u64;
                if nat_lt(a2916, a2917) {
                    let a2918 = v0;
                    let a2919 = 10u64;
                    if nat_lt(a2918, a2919) {
                        let a2920 = v0;
                        let a2921 = 9u64;
                        if nat_lt(a2920, a2921) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2922 = v0;
                        let a2923 = 11u64;
                        if nat_lt(a2922, a2923) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2924 = v0;
                    let a2925 = 14u64;
                    if nat_lt(a2924, a2925) {
                        let a2926 = v0;
                        let a2927 = 13u64;
                        if nat_lt(a2926, a2927) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2928 = v0;
                        let a2929 = 15u64;
                        if nat_lt(a2928, a2929) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2930 = v0;
            let a2931 = 24u64;
            if nat_lt(a2930, a2931) {
                let a2932 = v0;
                let a2933 = 20u64;
                if nat_lt(a2932, a2933) {
                    let a2934 = v0;
                    let a2935 = 18u64;
                    if nat_lt(a2934, a2935) {
                        let a2936 = v0;
                        let a2937 = 17u64;
                        if nat_lt(a2936, a2937) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2938 = v0;
                        let a2939 = 19u64;
                        if nat_lt(a2938, a2939) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2940 = v0;
                    let a2941 = 22u64;
                    if nat_lt(a2940, a2941) {
                        let a2942 = v0;
                        let a2943 = 21u64;
                        if nat_lt(a2942, a2943) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2944 = v0;
                        let a2945 = 23u64;
                        if nat_lt(a2944, a2945) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2946 = v0;
                let a2947 = 28u64;
                if nat_lt(a2946, a2947) {
                    let a2948 = v0;
                    let a2949 = 26u64;
                    if nat_lt(a2948, a2949) {
                        let a2950 = v0;
                        let a2951 = 25u64;
                        if nat_lt(a2950, a2951) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2952 = v0;
                        let a2953 = 27u64;
                        if nat_lt(a2952, a2953) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2954 = v0;
                    let a2955 = 30u64;
                    if nat_lt(a2954, a2955) {
                        let a2956 = v0;
                        let a2957 = 29u64;
                        if nat_lt(a2956, a2957) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2958 = v0;
                        let a2959 = 31u64;
                        if nat_lt(a2958, a2959) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2960 = v0;
        let a2961 = 48u64;
        if nat_lt(a2960, a2961) {
            let a2962 = v0;
            let a2963 = 40u64;
            if nat_lt(a2962, a2963) {
                let a2964 = v0;
                let a2965 = 36u64;
                if nat_lt(a2964, a2965) {
                    let a2966 = v0;
                    let a2967 = 34u64;
                    if nat_lt(a2966, a2967) {
                        let a2968 = v0;
                        let a2969 = 33u64;
                        if nat_lt(a2968, a2969) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2970 = v0;
                        let a2971 = 35u64;
                        if nat_lt(a2970, a2971) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2972 = v0;
                    let a2973 = 38u64;
                    if nat_lt(a2972, a2973) {
                        let a2974 = v0;
                        let a2975 = 37u64;
                        if nat_lt(a2974, a2975) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2976 = v0;
                        let a2977 = 39u64;
                        if nat_lt(a2976, a2977) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2978 = v0;
                let a2979 = 44u64;
                if nat_lt(a2978, a2979) {
                    let a2980 = v0;
                    let a2981 = 42u64;
                    if nat_lt(a2980, a2981) {
                        let a2982 = v0;
                        let a2983 = 41u64;
                        if nat_lt(a2982, a2983) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2984 = v0;
                        let a2985 = 43u64;
                        if nat_lt(a2984, a2985) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2986 = v0;
                    let a2987 = 46u64;
                    if nat_lt(a2986, a2987) {
                        let a2988 = v0;
                        let a2989 = 45u64;
                        if nat_lt(a2988, a2989) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2990 = v0;
                        let a2991 = 47u64;
                        if nat_lt(a2990, a2991) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2992 = v0;
            let a2993 = 56u64;
            if nat_lt(a2992, a2993) {
                let a2994 = v0;
                let a2995 = 52u64;
                if nat_lt(a2994, a2995) {
                    let a2996 = v0;
                    let a2997 = 50u64;
                    if nat_lt(a2996, a2997) {
                        let a2998 = v0;
                        let a2999 = 49u64;
                        if nat_lt(a2998, a2999) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3000 = v0;
                        let a3001 = 51u64;
                        if nat_lt(a3000, a3001) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3002 = v0;
                    let a3003 = 54u64;
                    if nat_lt(a3002, a3003) {
                        let a3004 = v0;
                        let a3005 = 53u64;
                        if nat_lt(a3004, a3005) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3006 = v0;
                        let a3007 = 55u64;
                        if nat_lt(a3006, a3007) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3008 = v0;
                let a3009 = 60u64;
                if nat_lt(a3008, a3009) {
                    let a3010 = v0;
                    let a3011 = 58u64;
                    if nat_lt(a3010, a3011) {
                        let a3012 = v0;
                        let a3013 = 57u64;
                        if nat_lt(a3012, a3013) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3014 = v0;
                        let a3015 = 59u64;
                        if nat_lt(a3014, a3015) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3016 = v0;
                    let a3017 = 62u64;
                    if nat_lt(a3016, a3017) {
                        let a3018 = v0;
                        let a3019 = 61u64;
                        if nat_lt(a3018, a3019) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3020 = v0;
                        let a3021 = 63u64;
                        if nat_lt(a3020, a3021) {
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

pub fn f606(v0: u32) -> u32 {
    let a3024 = {
        let a3022 = f647(v0, 6u32, 26u32);
        let a3023 = f647(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3022, a3023)
    };
    let a3025 = f647(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3024, a3025)
}

pub fn f607(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3031 = {
        let a3026 = v0;
        let a3027 = v1;
        fixed_u32::bit_and(a3026, a3027)
    };
    let a3032 = {
        let a3029 = {
            let a3028 = v0;
            fixed_u32::bit_not(a3028)
        };
        let a3030 = v2;
        fixed_u32::bit_and(a3029, a3030)
    };
    fixed_u32::bit_xor(a3031, a3032)
}

pub fn f608(v0: u32) -> u32 {
    let a3035 = {
        let a3033 = f647(v0, 2u32, 30u32);
        let a3034 = f647(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3033, a3034)
    };
    let a3036 = f647(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3035, a3036)
}

pub fn f609(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3045 = {
        let a3041 = {
            let a3037 = v0;
            let a3038 = v1;
            fixed_u32::bit_and(a3037, a3038)
        };
        let a3042 = {
            let a3039 = v0;
            let a3040 = v2;
            fixed_u32::bit_and(a3039, a3040)
        };
        fixed_u32::bit_xor(a3041, a3042)
    };
    let a3046 = {
        let a3043 = v1;
        let a3044 = v2;
        fixed_u32::bit_and(a3043, a3044)
    };
    fixed_u32::bit_xor(a3045, a3046)
}

pub fn f610(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3047, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3047
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3048, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3048
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3049, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3049
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3050, _, _, _, _, _, _, _, _, _, _, _) => {
            h3050
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3051, _, _, _, _, _, _, _, _, _, _) => {
            h3051
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3052, _, _, _, _, _, _, _, _, _) => {
            h3052
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3053, _, _, _, _, _, _, _, _) => {
            h3053
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3054, _, _, _, _, _, _, _) => {
            h3054
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3055, _, _, _, _, _, _) => {
            h3055
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3056, _, _, _, _, _) => {
            h3056
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3057, _, _, _, _) => {
            h3057
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3058, _, _, _) => {
            h3058
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3059, _, _) => {
            h3059
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3060, _) => {
            h3060
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3061) => {
            h3061
        }
    }, f522(f648(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3062, _) => {
            h3062
        }
    }), f522(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3063, _, _, _, _, _, _) => {
            h3063
        }
    }, f522(f649(match v0.clone() {
        Adt26::C0(_, h3064, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3064
        }
    }), match v0 {
        Adt26::C0(h3065, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3065
        }
    }))))
}

pub fn f611(v0: Fn21, p3069: u64, p3070: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3068 = (p3069, p3070);
    {
        let mut m3068 = m3068;
        loop {
            let step = {
                let (v1, v2) = m3068;
                let m3066 = v1;
                if m3066 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3066 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3067: Fn21 = v0.clone();
                        c3067.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f612(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3071, _) = v0;
        h3071
    };
    let v2: u32 = {
        let (h3072, _) = {
            let (_, h3073) = v0;
            h3073
        };
        h3072
    };
    let v3: u32 = {
        let (_, h3074) = {
            let (_, h3075) = v0;
            h3075
        };
        h3074
    };
    ({
        let a3076 = v1;
        let a3077 = 2u64;
        let a3078 = 0u64;
        nat_quot(a3076, a3077, a3078)
    }, (f613(v2, 1u32), {
        let a3082 = {
            let a3079 = v1;
            let a3080 = 2u64;
            let a3081 = 0u64;
            nat_rem(a3079, a3080, a3081)
        };
        let a3083 = 1u64;
        if nat_eq(a3082, a3083) {
            let a3084 = v3;
            let a3085 = v2;
            fixed_u32::bit_or(a3084, a3085)
        } else {
            v3
        }
    }))
}

pub fn f613(v0: u32, v1: u32) -> u32 {
    let m3088 = {
        let a3086 = v0;
        let a3087 = v1;
        fixed_u32::shift_left(a3086, a3087)
    };
    match m3088 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f614(v0: Bytes, v1: Adt17) -> bool {
    let v2: Bytes = f562(v1);
    if f28(v0.clone(), f8(Str::lit("number"))) {
        if f28(v2.clone(), f8(Str::lit("integer"))) {
            true
        } else {
            f28(v2, f8(Str::lit("number")))
        }
    } else {
        f28(v0, v2)
    }
}

pub fn f615(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m3089 = v0;
    if m3089 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m3089 - 1;
        let a3091 = {
            let a3090 = v1.clone();
            length_bytes(a3090)
        };
        let a3092 = v2;
        if nat_le(a3091, a3092) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m3093 = f650(v1.clone(), v2)?;
            match m3093 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m3097 = f615(v3, v1, {
                        let a3095 = v2;
                        let a3096 = {
                            let (_, h3094) = v4;
                            h3094
                        };
                        nat_add(a3095, a3096)?
                    })?;
                    match m3097 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h3098, _) = v4;
                                h3098
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f616(v0: Bytes) -> R<Result<((bool, bool), List<Adt30>), Bytes>> {
    let v1: u64 = {
        let a3099 = v0.clone();
        length_bytes(a3099)
    };
    let v2: bool = {
        let a3100 = 0u64;
        let a3101 = v1;
        if nat_lt(a3100, a3101) {
            let a3102 = f350(v0.clone(), 0u64)?;
            let a3103 = 94u64;
            nat_eq(a3102, a3103)
        } else {
            false
        }
    };
    let v3: bool = {
        let a3104 = 0u64;
        let a3105 = v1;
        if nat_lt(a3104, a3105) {
            f228({
                let a3108 = f350(v0.clone(), {
                    let a3106 = v1;
                    let a3107 = 1u64;
                    nat_sub(a3106, a3107)
                })?;
                let a3109 = 36u64;
                nat_eq(a3108, a3109)
            }, f651({
                let a3112 = f350(v0.clone(), {
                    let a3110 = v1;
                    let a3111 = 2u64;
                    nat_sub(a3110, a3111)
                })?;
                let a3113 = 92u64;
                nat_eq(a3112, a3113)
            }))
        } else {
            false
        }
    };
    let m3118 = f652({
        let a3114 = v1;
        let a3115 = 1u64;
        nat_add(a3114, a3115)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a3116 = v1;
        let a3117 = 1u64;
        nat_sub(a3116, a3117)
    } else {
        v1
    })?;
    match m3118 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt30>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt30>), Bytes>(v4))
        }
    }
}

pub fn f617(v0: u64, v1: List<Adt30>, v2: List<u64>, v3: bool) -> R<bool> {
    let m3119 = v0;
    if m3119 == 0 {
        Ok(false)
    } else {
        let v4 = m3119 - 1;
        let m3120 = v1.clone();
        match m3120.uncons() {
            None => {
                if v3 {
                    let a3121 = f567(v2)?;
                    let a3122 = 0u64;
                    Ok(nat_eq(a3121, a3122))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m3123 = v5.clone();
                match m3123 {
                    Adt30::C0(v7, v8, v9) => {
                        let a3124 = v9;
                        let a3125 = 1u64;
                        if nat_eq(a3124, a3125) {
                            if f617(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m3126 = v2;
                                match m3126.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f653(v5, v10) {
                                            f617(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a3127 = v9;
                            let a3128 = 2u64;
                            if nat_eq(a3127, a3128) {
                                let m3129 = v2;
                                match m3129.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f653(v5, v12) {
                                            f617(v4, List::cons(Adt30::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a3130 = v9;
                                let a3131 = 3u64;
                                if nat_eq(a3130, a3131) {
                                    if f617(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m3132 = v2;
                                        match m3132.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f653(v5, v14) {
                                                    f617(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m3133 = v2;
                                    match m3133.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f653(v5, v16) {
                                                f617(v4, v6, v17, v3)
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

pub fn f618(v0: List<Adt30>) -> R<u64> {
    let m3134 = v0;
    match m3134.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3135 = f618(v2)?;
            let a3136 = 1u64;
            nat_add(a3135, a3136)
        }
    }
}

pub fn f619(v0: List<Adt30>, v1: List<u64>, v2: bool) -> R<bool> {
    if f617({
        let a3139 = {
            let a3137 = f618(v0.clone())?;
            let a3138 = f567(v1.clone())?;
            nat_add(a3137, a3138)?
        };
        let a3140 = 2u64;
        nat_add(a3139, a3140)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f654(v0, v1, v2)
    }
}

pub fn f620(v0: u64) -> R<u64> {
    let m3141 = v0;
    if m3141 == 0 {
        Ok(1u64)
    } else {
        let v1 = m3141 - 1;
        let a3142 = f620(v1)?;
        let a3143 = 10u64;
        nat_mul(a3142, a3143)
    }
}

pub fn f621(p3157: u64, v1: Bytes, p3158: u64, p3159: Bytes) -> R<Bytes> {
    let m3156 = (p3157, (p3158, p3159));
    {
        let mut m3156 = m3156;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3156;
                let m3144 = v0;
                if m3144 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3144 - 1;
                    let m3147 = {
                        let a3145 = v1.clone();
                        let a3146 = v2;
                        index_bytes(a3145, a3146)
                    };
                    match m3147 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3148 = v2;
                                let a3149 = 1u64;
                                nat_add(a3148, a3149)?
                            }, {
                                let a3154 = v3;
                                let a3155 = {
                                    let a3150 = v6;
                                    let a3151 = 126u64;
                                    if nat_eq(a3150, a3151) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a3152 = v6;
                                        let a3153 = 47u64;
                                        if nat_eq(a3152, a3153) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f35(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a3154, a3155)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3162 = f35(v0.clone(), {
        let a3160 = v1;
        let a3161 = 2u64;
        nat_add(a3160, a3161)?
    }, 2u64);
    let a3163 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3162, a3163) {
        let m3166 = f655(f35(v0, {
            let a3164 = v1;
            let a3165 = 4u64;
            nat_add(a3164, a3165)?
        }, 2u64))?;
        match m3166 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f350(v2.clone(), 0u64)?;
                let a3167 = v3;
                let a3168 = 32u64;
                if nat_lt(a3167, a3168) {
                    if f656(v3) {
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

pub fn f623(v0: u64) -> Bytes {
    let a3169 = v0;
    let a3170 = 34u64;
    if nat_eq(a3169, a3170) {
        Bytes::lit(&[0x22])
    } else {
        let a3171 = v0;
        let a3172 = 92u64;
        if nat_eq(a3171, a3172) {
            Bytes::lit(&[0x5c])
        } else {
            let a3173 = v0;
            let a3174 = 98u64;
            if nat_eq(a3173, a3174) {
                Bytes::lit(&[0x08])
            } else {
                let a3175 = v0;
                let a3176 = 102u64;
                if nat_eq(a3175, a3176) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3177 = v0;
                    let a3178 = 110u64;
                    if nat_eq(a3177, a3178) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3179 = v0;
                        let a3180 = 114u64;
                        if nat_eq(a3179, a3180) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3181 = v0;
                            let a3182 = 116u64;
                            if nat_eq(a3181, a3182) {
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

pub fn f624(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3227 = {
            let a3203 = {
                let a3191 = {
                    let a3185 = {
                        let a3183 = v1;
                        let a3184 = 128u8;
                        fixed_u8::bit_and(a3183, a3184)
                    };
                    let a3186 = 128u8;
                    if equal(a3185, a3186) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3192 = {
                    let a3189 = {
                        let a3187 = v1;
                        let a3188 = 64u8;
                        fixed_u8::bit_and(a3187, a3188)
                    };
                    let a3190 = 64u8;
                    if equal(a3189, a3190) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3191, a3192)?
            };
            let a3204 = {
                let a3201 = {
                    let a3195 = {
                        let a3193 = v1;
                        let a3194 = 32u8;
                        fixed_u8::bit_and(a3193, a3194)
                    };
                    let a3196 = 32u8;
                    if equal(a3195, a3196) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3202 = {
                    let a3199 = {
                        let a3197 = v1;
                        let a3198 = 16u8;
                        fixed_u8::bit_and(a3197, a3198)
                    };
                    let a3200 = 16u8;
                    if equal(a3199, a3200) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3201, a3202)?
            };
            nat_add(a3203, a3204)?
        };
        let a3228 = {
            let a3225 = {
                let a3213 = {
                    let a3207 = {
                        let a3205 = v1;
                        let a3206 = 8u8;
                        fixed_u8::bit_and(a3205, a3206)
                    };
                    let a3208 = 8u8;
                    if equal(a3207, a3208) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3214 = {
                    let a3211 = {
                        let a3209 = v1;
                        let a3210 = 4u8;
                        fixed_u8::bit_and(a3209, a3210)
                    };
                    let a3212 = 4u8;
                    if equal(a3211, a3212) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3213, a3214)?
            };
            let a3226 = {
                let a3223 = {
                    let a3217 = {
                        let a3215 = v1;
                        let a3216 = 2u8;
                        fixed_u8::bit_and(a3215, a3216)
                    };
                    let a3218 = 2u8;
                    if equal(a3217, a3218) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3224 = {
                    let a3221 = {
                        let a3219 = v1;
                        let a3220 = 1u8;
                        fixed_u8::bit_and(a3219, a3220)
                    };
                    let a3222 = 1u8;
                    if equal(a3221, a3222) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3223, a3224)?
            };
            nat_add(a3225, a3226)?
        };
        nat_add(a3227, a3228)?
    };
    let a3229 = 48u64;
    let a3230 = v2;
    if nat_le(a3229, a3230) {
        let a3231 = v2;
        let a3232 = 57u64;
        Ok(nat_le(a3231, a3232))
    } else {
        Ok(false)
    }
}

pub fn f625(v0: Fn22, p3237: u64, p3238: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3236 = (p3237, p3238);
    {
        let mut m3236 = m3236;
        loop {
            let step = {
                let (v1, v2) = m3236;
                let m3233 = v1;
                if m3233 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3233 - 1;
                    let m3235 = {
                        let c3234: Fn22 = v0.clone();
                        c3234.apply(v2.clone())?
                    };
                    match m3235 {
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
                Ok(next) => { m3236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f657(v0)
}

pub fn f627(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3243 = {
            let a3241 = {
                let a3239 = v0.clone();
                length_list(a3239)
            };
            let a3242 = {
                let a3240 = v1.clone();
                length_list(a3240)
            };
            nat_add(a3241, a3242)?
        };
        let a3244 = 1u64;
        nat_add(a3243, a3244)?
    };
    Ok(f394({
        let (_, h3245) = {
            let (_, h3246) = {
                let (h3247, _) = f658(Fn23::F659, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3247
            };
            h3246
        };
        h3245
    }))
}

pub fn f628(v0: u64) -> Bytes {
    f35({
        let a3248 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3248)
    }, v0, 1u64)
}

pub fn f629(v0: bool, v1: List<Adt17>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3249 = v1;
    match m3249.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3250 = f660(v0, v3.clone())?;
            match m3250 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f485(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f41(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3251 = f629(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3251 {
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

pub fn f630(p3255: List<Bytes>) -> R<bool> {
    let m3254 = p3255;
    {
        let mut m3254 = m3254;
        loop {
            let step = {
                let v0 = m3254;
                let m3252 = v0;
                match m3252.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3253 = f661(v1)?;
                        match m3253 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f41(v3, f662(false, v2.clone())?) {
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
                Ok(next) => { m3254 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631(v0: List<(Bytes, Adt17)>, v1: List<Adt17>) -> List<Bytes> {
    let m3256 = v0;
    match m3256.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f663(v1.clone(), {
                let (h3257, _) = v2.clone();
                h3257
            }) {
                List::cons({
                    let (h3258, _) = v2;
                    h3258
                }, f631(v3, v1))
            } else {
                f631(v3, v1)
            }
        }
    }
}

pub fn f632(v0: bool, v1: Bytes, v2: List<Adt17>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3259 = v2;
    match m3259.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3260 = f664(v4.clone())?;
            match m3260 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3261 = f665(v0, v6.clone())?;
                    match m3261 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f41(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3264 = f666(v4, {
                                    let a3262 = v1.clone();
                                    let a3263 = f330(v6)?;
                                    append_bytes(a3262, a3263)
                                })?;
                                match m3264 {
                                    Ok(v9) => {
                                        let m3265 = f632(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3265 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f518(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f594(v11.clone())), f473(f517(v9), f595(v11)))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<Adt29, Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<Adt29, Bytes>(v8))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f633(v0: List<Adt17>) -> R<List<Bytes>> {
    let m3266 = v0;
    match m3266.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3267 = f664(v1)?;
            match m3267 {
                None => {
                    f633(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f633(v2)?))
                }
            }
        }
    }
}

pub fn f634(v0: Bytes, v1: List<Adt17>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f667(f630(f668(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f635(v0: Adt17, v1: Bytes) -> R<bool> {
    let m3268 = f336(Some(v0));
    match m3268 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3269 = f585(v2)?;
            match m3269 {
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

pub fn f636(v0: bool, v1: Bytes, v2: List<(Bytes, Adt17)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m3270 = v2;
    match m3270.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3272 = f669(v0, {
                let (h3271, _) = v5.clone();
                h3271
            })?;
            match m3272 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3273, _) = v5;
                        h3273
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f41(v7.clone(), f670(v0, f572(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3278 = f598({
                            let (_, h3274) = v5.clone();
                            h3274
                        }, {
                            let a3276 = v1.clone();
                            let a3277 = f330({
                                let (h3275, _) = v5.clone();
                                h3275
                            })?;
                            append_bytes(a3276, a3277)
                        })?;
                        match m3278 {
                            Ok(v9) => {
                                let v10: Bytes = if f41({
                                    let (h3279, _) = v5;
                                    h3279
                                }, v3.clone()) {
                                    f518(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f518(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3280 = f636(v0, v1, v6, v3, v4)?;
                                match m3280 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f594(v12.clone())), f473(f517(v9), f595(v12)))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt29, Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<Adt29, Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f637(v0: Adt17) -> Option<Adt17> {
    let m3281 = f239(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3281 {
        None => {
            f671(v0)
        }
        Some(v1) => {
            if f638(v1.clone()) {
                let m3282 = f671(v0);
                match m3282 {
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

pub fn f638(v0: Adt17) -> bool {
    let m3283 = v0;
    match m3283 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_, _) => {
            false
        }
        Adt17::C2(v3) => {
            !v3
        }
        Adt17::C3(_, _) => {
            false
        }
        Adt17::C4(_, _, _) => {
            false
        }
        Adt17::C5(_) => {
            false
        }
        Adt17::C6(_) => {
            false
        }
        Adt17::C7(_) => {
            false
        }
        Adt17::C8 => {
            false
        }
        Adt17::C9(_, _) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
    }
}

pub fn f639(v0: Adt17) -> Option<Adt17> {
    let m3284 = f244(f239(v0, f8(Str::lit("oneOf"))));
    match m3284.uncons() {
        None => {
            None::<Adt17>
        }
        Some((v1, v2)) => {
            let m3285 = v2;
            match m3285.uncons() {
                None => {
                    None::<Adt17>
                }
                Some((v3, v4)) => {
                    let m3286 = v4;
                    match m3286.uncons() {
                        None => {
                            if f672(v3.clone()) {
                                Some(v1)
                            } else {
                                if f672(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt17>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt17>
                        }
                    }
                }
            }
        }
    }
}

pub fn f640(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3287 = v1;
    match m3287 {
        Ok(v3) => {
            f589(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f518(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f517(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f641(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3288 = f639(v0.clone());
    match m3288 {
        None => {
            if f28(f242(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3289 = f239(v0, f8(Str::lit("items")));
                match m3289 {
                    None => {
                        Ok(f640(f8(Str::lit("List")), f642()))
                    }
                    Some(v3) => {
                        Ok(f640(f8(Str::lit("List")), f666(v3, {
                            let a3290 = v1;
                            let a3291 = f8(Str::lit("Item"));
                            append_bytes(a3290, a3291)
                        })?))
                    }
                }
            } else {
                f666(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f640(f8(Str::lit("Option")), f666(v2, v1)?))
        }
    }
}

pub fn f642() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f514(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f643(p3295: List<(Bytes, Adt17)>, v1: List<Bytes>) -> bool {
    let m3294 = p3295;
    {
        let mut m3294 = m3294;
        loop {
            let step = {
                let v0 = m3294;
                let m3292 = v0;
                match m3292.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h3293, _) = v2;
                            h3293
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt17)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt17)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(p3299: List<Adt17>, v1: Bytes, p3300: List<Bytes>) -> bool {
    let m3298 = (p3299, p3300);
    {
        let mut m3298 = m3298;
        loop {
            let step = {
                let (v0, v2) = m3298;
                let m3296 = v0;
                match m3296.uncons() {
                    None => {
                        Err::<(List<Adt17>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3297 = f673(v3, v1.clone());
                        match m3297 {
                            None => {
                                Err::<(List<Adt17>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f41(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt17>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt17>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645(p3303: List<Adt17>, p3304: List<Bytes>) -> bool {
    let m3302 = (p3303, p3304);
    {
        let mut m3302 = m3302;
        loop {
            let step = {
                let (v0, v1) = m3302;
                let m3301 = v0;
                match m3301.uncons() {
                    None => {
                        Err::<(List<Adt17>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f242(v2, f8(Str::lit("type")));
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
                                    Ok::<(List<Adt17>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt17>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt17>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt17>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f646(v0: Adt17) -> bool {
    let v2: Adt17 = {
        let m3305 = f239(v0.clone(), f8(Str::lit("anyOf")));
        match m3305 {
            None => {
                Adt17::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f643(f510(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f599(f337(v2))
    } else {
        false
    }
}

pub fn f647(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3306 = f479(v0, v1);
    let a3307 = f613(v0, v2);
    fixed_u32::bit_or(a3306, a3307)
}

pub fn f648(v0: u32) -> u32 {
    let a3310 = {
        let a3308 = f647(v0, 17u32, 15u32);
        let a3309 = f647(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3308, a3309)
    };
    let a3311 = f479(v0, 10u32);
    fixed_u32::bit_xor(a3310, a3311)
}

pub fn f649(v0: u32) -> u32 {
    let a3314 = {
        let a3312 = f647(v0, 7u32, 25u32);
        let a3313 = f647(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3312, a3313)
    };
    let a3315 = f479(v0, 3u32);
    fixed_u32::bit_xor(a3314, a3315)
}

pub fn f650(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3318 = {
        let a3316 = v0.clone();
        let a3317 = v1;
        index_bytes(a3316, a3317)
    };
    match m3318 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            let a3319 = v3;
            let a3320 = 128u64;
            if nat_lt(a3319, a3320) {
                Ok(Some((v3, 1u64)))
            } else {
                if f228({
                    let a3321 = 194u64;
                    let a3322 = v3;
                    nat_le(a3321, a3322)
                }, {
                    let a3323 = v3;
                    let a3324 = 223u64;
                    nat_le(a3323, a3324)
                }) {
                    let m3327 = f674(v0, {
                        let a3325 = v1;
                        let a3326 = 1u64;
                        nat_add(a3325, a3326)?
                    })?;
                    match m3327 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f675({
                                let a3332 = {
                                    let a3330 = {
                                        let a3328 = v3;
                                        let a3329 = 192u64;
                                        nat_sub(a3328, a3329)
                                    };
                                    let a3331 = 64u64;
                                    nat_mul(a3330, a3331)?
                                };
                                let a3333 = v4;
                                nat_add(a3332, a3333)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f228({
                        let a3334 = 224u64;
                        let a3335 = v3;
                        nat_le(a3334, a3335)
                    }, {
                        let a3336 = v3;
                        let a3337 = 239u64;
                        nat_le(a3336, a3337)
                    }) {
                        let m3340 = f674(v0.clone(), {
                            let a3338 = v1;
                            let a3339 = 1u64;
                            nat_add(a3338, a3339)?
                        })?;
                        match m3340 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3343 = f674(v0, {
                                    let a3341 = v1;
                                    let a3342 = 2u64;
                                    nat_add(a3341, a3342)?
                                })?;
                                match m3343 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f675({
                                            let a3352 = {
                                                let a3346 = {
                                                    let a3344 = v3;
                                                    let a3345 = 224u64;
                                                    nat_sub(a3344, a3345)
                                                };
                                                let a3347 = 4096u64;
                                                nat_mul(a3346, a3347)?
                                            };
                                            let a3353 = {
                                                let a3350 = {
                                                    let a3348 = v5;
                                                    let a3349 = 64u64;
                                                    nat_mul(a3348, a3349)?
                                                };
                                                let a3351 = v6;
                                                nat_add(a3350, a3351)?
                                            };
                                            nat_add(a3352, a3353)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f228({
                            let a3354 = 240u64;
                            let a3355 = v3;
                            nat_le(a3354, a3355)
                        }, {
                            let a3356 = v3;
                            let a3357 = 244u64;
                            nat_le(a3356, a3357)
                        }) {
                            let m3360 = f674(v0.clone(), {
                                let a3358 = v1;
                                let a3359 = 1u64;
                                nat_add(a3358, a3359)?
                            })?;
                            match m3360 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3363 = f674(v0.clone(), {
                                        let a3361 = v1;
                                        let a3362 = 2u64;
                                        nat_add(a3361, a3362)?
                                    })?;
                                    match m3363 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3366 = f674(v0, {
                                                let a3364 = v1;
                                                let a3365 = 3u64;
                                                nat_add(a3364, a3365)?
                                            })?;
                                            match m3366 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f675({
                                                        let a3379 = {
                                                            let a3369 = {
                                                                let a3367 = v3;
                                                                let a3368 = 240u64;
                                                                nat_sub(a3367, a3368)
                                                            };
                                                            let a3370 = 262144u64;
                                                            nat_mul(a3369, a3370)?
                                                        };
                                                        let a3380 = {
                                                            let a3377 = {
                                                                let a3371 = v7;
                                                                let a3372 = 4096u64;
                                                                nat_mul(a3371, a3372)?
                                                            };
                                                            let a3378 = {
                                                                let a3375 = {
                                                                    let a3373 = v8;
                                                                    let a3374 = 64u64;
                                                                    nat_mul(a3373, a3374)?
                                                                };
                                                                let a3376 = v9;
                                                                nat_add(a3375, a3376)?
                                                            };
                                                            nat_add(a3377, a3378)?
                                                        };
                                                        nat_add(a3379, a3380)?
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

pub fn f651(v0: bool) -> bool {
    !v0
}

pub fn f652(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt30>, Bytes>> {
    let m3381 = v0;
    if m3381 == 0 {
        Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
    } else {
        let v4 = m3381 - 1;
        let a3382 = v3;
        let a3383 = v2;
        if nat_le(a3382, a3383) {
            Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
        } else {
            let v5: u64 = f350(v1.clone(), v2)?;
            let m3432 = {
                let a3384 = v5;
                let a3385 = 91u64;
                if nat_eq(a3384, a3385) {
                    let a3388 = f350(v1.clone(), {
                        let a3386 = v2;
                        let a3387 = 1u64;
                        nat_add(a3386, a3387)?
                    })?;
                    let a3389 = 94u64;
                    if nat_eq(a3388, a3389) {
                        let m3393 = f676({
                            let a3390 = v1.clone();
                            length_bytes(a3390)
                        }, v1.clone(), {
                            let a3391 = v2;
                            let a3392 = 2u64;
                            nat_add(a3391, a3392)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3393 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h3394, _) = v7;
                                    h3394
                                }), {
                                    let (_, h3395) = v7;
                                    h3395
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m3399 = f676({
                            let a3396 = v1.clone();
                            length_bytes(a3396)
                        }, v1.clone(), {
                            let a3397 = v2;
                            let a3398 = 1u64;
                            nat_add(a3397, a3398)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3399 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h3400, _) = v9;
                                    h3400
                                }), {
                                    let (_, h3401) = v9;
                                    h3401
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a3402 = v5;
                    let a3403 = 46u64;
                    if nat_eq(a3402, a3403) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a3404 = v2;
                            let a3405 = 1u64;
                            nat_add(a3404, a3405)?
                        }))
                    } else {
                        let a3406 = v5;
                        let a3407 = 40u64;
                        if if if if if if nat_eq(a3406, a3407) {
                            true
                        } else {
                            let a3408 = v5;
                            let a3409 = 41u64;
                            nat_eq(a3408, a3409)
                        } {
                            true
                        } else {
                            let a3410 = v5;
                            let a3411 = 124u64;
                            if nat_eq(a3410, a3411) {
                                true
                            } else {
                                let a3412 = v5;
                                let a3413 = 123u64;
                                nat_eq(a3412, a3413)
                            }
                        } {
                            true
                        } else {
                            let a3414 = v5;
                            let a3415 = 125u64;
                            if if nat_eq(a3414, a3415) {
                                true
                            } else {
                                let a3416 = v5;
                                let a3417 = 92u64;
                                nat_eq(a3416, a3417)
                            } {
                                true
                            } else {
                                let a3418 = v5;
                                let a3419 = 94u64;
                                if nat_eq(a3418, a3419) {
                                    true
                                } else {
                                    let a3420 = v5;
                                    let a3421 = 36u64;
                                    nat_eq(a3420, a3421)
                                }
                            }
                        } {
                            true
                        } else {
                            let a3422 = v5;
                            let a3423 = 42u64;
                            if if nat_eq(a3422, a3423) {
                                true
                            } else {
                                let a3424 = v5;
                                let a3425 = 43u64;
                                nat_eq(a3424, a3425)
                            } {
                                true
                            } else {
                                let a3426 = v5;
                                let a3427 = 63u64;
                                nat_eq(a3426, a3427)
                            }
                        } {
                            true
                        } else {
                            let a3428 = 127u64;
                            let a3429 = v5;
                            nat_lt(a3428, a3429)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f35(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a3430 = v2;
                                let a3431 = 1u64;
                                nat_add(a3430, a3431)?
                            }))
                        }
                    }
                }
            };
            match m3432 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h3433) = v11.clone();
                        h3433
                    };
                    let v13: u64 = {
                        let a3434 = v12;
                        let a3435 = v3;
                        if nat_lt(a3434, a3435) {
                            f677(f350(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a3436 = v12;
                    let a3437 = v3;
                    if if nat_lt(a3436, a3437) {
                        let a3438 = f350(v1.clone(), v12)?;
                        let a3439 = 123u64;
                        nat_eq(a3438, a3439)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt30>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m3444 = f652(v4, v1, {
                            let a3440 = 0u64;
                            let a3441 = v13;
                            if nat_lt(a3440, a3441) {
                                let a3442 = v12;
                                let a3443 = 1u64;
                                nat_add(a3442, a3443)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m3444 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt30>, Bytes>(List::cons(Adt30::C0({
                                    let (h3445, _) = {
                                        let (h3446, _) = v11.clone();
                                        h3446
                                    };
                                    h3445
                                }, {
                                    let (_, h3447) = {
                                        let (h3448, _) = v11;
                                        h3448
                                    };
                                    h3447
                                }, v13), v15)))
                            }
                            Err(v14) => {
                                Ok(Err::<List<Adt30>, Bytes>(v14))
                            }
                        }
                    }
                }
                Err(v10) => {
                    Ok(Err::<List<Adt30>, Bytes>(v10))
                }
            }
        }
    }
}

pub fn f653(v0: Adt30, v1: u64) -> bool {
    let m3449 = v0;
    match m3449 {
        Adt30::C0(v2, v3, _) => {
            if v2 {
                f651(f678(v3, v1))
            } else {
                f678(v3, v1)
            }
        }
    }
}

pub fn f654(v0: List<Adt30>, p3456: List<u64>, v2: bool) -> R<bool> {
    let m3455 = p3456;
    {
        let mut m3455 = m3455;
        loop {
            let step = {
                let v1 = m3455;
                let m3450 = v1;
                match m3450.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f617({
                            let a3453 = {
                                let a3451 = f618(v0.clone())?;
                                let a3452 = f567(v4.clone())?;
                                nat_add(a3451, a3452)?
                            };
                            let a3454 = 2u64;
                            nat_add(a3453, a3454)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3455 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f655(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3457 = v0.clone();
        length_bytes(a3457)
    };
    let a3461 = {
        let a3458 = v1;
        let a3459 = 2u64;
        let a3460 = 0u64;
        nat_rem(a3458, a3459, a3460)
    };
    let a3462 = 0u64;
    if nat_eq(a3461, a3462) {
        let (_, h3463) = {
            let (h3464, _) = f679(Fn24::F680(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3464
        };
        Ok(h3463)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f656(v0: u64) -> bool {
    let a3465 = v0;
    let a3466 = 8u64;
    if nat_eq(a3465, a3466) {
        true
    } else {
        let a3467 = v0;
        let a3468 = 9u64;
        if nat_eq(a3467, a3468) {
            true
        } else {
            let a3469 = v0;
            let a3470 = 10u64;
            if nat_eq(a3469, a3470) {
                true
            } else {
                let a3471 = v0;
                let a3472 = 12u64;
                if nat_eq(a3471, a3472) {
                    true
                } else {
                    let a3473 = v0;
                    let a3474 = 13u64;
                    nat_eq(a3473, a3474)
                }
            }
        }
    }
}

pub fn f657(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3475, _) = v0;
        h3475
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3476) = v0;
        h3476
    };
    let m3477 = v1;
    match m3477.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3478 = v4;
            match m3478.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f681(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f658(v0: Fn23, p3483: u64, p3484: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3482 = (p3483, p3484);
    {
        let mut m3482 = m3482;
        loop {
            let step = {
                let (v1, v2) = m3482;
                let m3479 = v1;
                if m3479 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3479 - 1;
                    let m3481 = {
                        let c3480: Fn23 = v0.clone();
                        c3480.apply(v2.clone())
                    };
                    match m3481 {
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
                Ok(next) => { m3482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f659(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f682(v0)
}

pub fn f660(v0: bool, v1: Adt17) -> R<Option<Bytes>> {
    let m3485 = v1;
    match m3485 {
        Adt17::C0 => {
            Ok(None::<Bytes>)
        }
        Adt17::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt17::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C3(v7, v8) => {
            Ok(Some({
                let a3496 = if f228(v7, {
                    let a3494 = 0u64;
                    let a3495 = v8;
                    nat_lt(a3494, a3495)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3497 = f265(false, v8)?;
                append_bytes(a3496, a3497)
            }))
        }
        Adt17::C4(v3, v4, v5) => {
            let m3488 = f683({
                let a3486 = f8(Str::lit("n"));
                let a3487 = f415(false, v4, v5)?;
                append_bytes(a3486, a3487)
            })?;
            match m3488 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3492 = f8(Str::lit("nMinus"));
                        let a3493 = f35(v6.clone(), 1u64, {
                            let a3490 = {
                                let a3489 = v6;
                                length_bytes(a3489)
                            };
                            let a3491 = 1u64;
                            nat_sub(a3490, a3491)
                        });
                        append_bytes(a3492, a3493)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt17::C5(v2) => {
            f665(v0, v2)
        }
        Adt17::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C8 => {
            Ok(None::<Bytes>)
        }
        Adt17::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt17::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f661(v0: Bytes) -> R<Option<Bytes>> {
    let m3498 = f684(v0)?;
    match m3498 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f685(v1)?))
        }
    }
}

pub fn f662(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3499 = v1;
    match m3499.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3500 = f665(v0, v2)?;
            match m3500 {
                None => {
                    f662(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f662(v0, v3)?))
                }
            }
        }
    }
}

pub fn f663(p3504: List<Adt17>, v1: Bytes) -> bool {
    let m3503 = p3504;
    {
        let mut m3503 = m3503;
        loop {
            let step = {
                let v0 = m3503;
                let m3501 = v0;
                match m3501.uncons() {
                    None => {
                        Err::<List<Adt17>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3502 = f673(v2, v1.clone());
                        if match m3502 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt17>, bool>(v3)
                        } else {
                            Err::<List<Adt17>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3503 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f664(v0: Adt17) -> R<Option<Bytes>> {
    let m3505 = f336(f239(v0.clone(), f8(Str::lit("$ref"))));
    match m3505 {
        None => {
            Ok(f336(f239(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f585(v1)
        }
    }
}

pub fn f665(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f683(v1)
    } else {
        f661(v1)
    }
}

pub fn f666(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f686(f554(v0), v1)
}

pub fn f667(v0: bool, v1: Bytes, v2: List<Adt17>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3506 = v2;
    match m3506.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3507 = f673(v5.clone(), v3.clone());
            match m3507 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3508 = f665(v0, v7.clone())?;
                    match m3508 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f41(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3511 = f592({
                                    let a3509 = v1.clone();
                                    let a3510 = f330(v7)?;
                                    append_bytes(a3509, a3510)
                                }, f512(f469(f239(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f243(f244(f239(v5, f8(Str::lit("required"))))), true)?;
                                match m3511 {
                                    Ok(v10) => {
                                        let m3512 = f667(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3512 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f346(f594(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f594(v12.clone())), f473(f595(v10), f595(v12)))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt29, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt29, Bytes>(v9))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f668(v0: List<Adt17>, v1: Bytes) -> List<Bytes> {
    let m3513 = v0;
    match m3513.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3514 = f673(v2, v1.clone());
            match m3514 {
                None => {
                    f668(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f668(v3, v1))
                }
            }
        }
    }
}

pub fn f669(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3515 = f665(v0, v1)?;
    match m3515 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f28(v2.clone(), f8(Str::lit("mk"))) {
                f552(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f670(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3516 = v1;
    match m3516.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3517 = f669(v0, v2)?;
            match m3517 {
                None => {
                    f670(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f670(v0, v3)?))
                }
            }
        }
    }
}

pub fn f671(v0: Adt17) -> Option<Adt17> {
    let m3518 = f469(f239(v0, f8(Str::lit("patternProperties"))));
    match m3518.uncons() {
        None => {
            None::<Adt17>
        }
        Some((v1, v2)) => {
            let m3519 = v2;
            match m3519.uncons() {
                None => {
                    Some({
                        let (_, h3520) = v1;
                        h3520
                    })
                }
                Some((_, _)) => {
                    None::<Adt17>
                }
            }
        }
    }
}

pub fn f672(v0: Adt17) -> bool {
    f28(f242(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f673(v0: Adt17, v1: Bytes) -> Option<Bytes> {
    if f41(v1.clone(), f243(f244(f239(v0.clone(), f8(Str::lit("required")))))) {
        let m3521 = f239(v0, f8(Str::lit("properties")));
        match m3521 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3522 = f239(v2, v1);
                match m3522 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f336(f239(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f674(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3525 = {
        let a3523 = v0;
        let a3524 = v1;
        index_bytes(a3523, a3524)
    };
    match m3525 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f228({
                let a3526 = 128u64;
                let a3527 = v3;
                nat_le(a3526, a3527)
            }, {
                let a3528 = v3;
                let a3529 = 191u64;
                nat_le(a3528, a3529)
            }) {
                Ok(Some({
                    let a3530 = v3;
                    let a3531 = 128u64;
                    nat_sub(a3530, a3531)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f675(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f228({
        let a3532 = v1;
        let a3533 = v0;
        nat_le(a3532, a3533)
    }, f228({
        let a3534 = v0;
        let a3535 = v2;
        nat_le(a3534, a3535)
    }, f433({
        let a3536 = v0;
        let a3537 = 55296u64;
        nat_lt(a3536, a3537)
    }, {
        let a3538 = 57343u64;
        let a3539 = v0;
        nat_lt(a3538, a3539)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f676(p3576: u64, v1: Bytes, p3577: u64, p3578: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3575 = (p3576, (p3577, p3578));
    {
        let mut m3575 = m3575;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3575;
                let m3540 = v0;
                if m3540 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3540 - 1;
                    let v5: u64 = f350(v1.clone(), v2)?;
                    let a3542 = {
                        let a3541 = v1.clone();
                        length_bytes(a3541)
                    };
                    let a3543 = v2;
                    if nat_le(a3542, a3543) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3544 = v5;
                        let a3545 = 93u64;
                        if nat_eq(a3544, a3545) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3546 = v2;
                                let a3547 = 1u64;
                                nat_add(a3546, a3547)?
                            }))))
                        } else {
                            let a3548 = v5;
                            let a3549 = 92u64;
                            if if nat_eq(a3548, a3549) {
                                true
                            } else {
                                let a3550 = 127u64;
                                let a3551 = v5;
                                nat_lt(a3550, a3551)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3554 = f350(v1.clone(), {
                                    let a3552 = v2;
                                    let a3553 = 1u64;
                                    nat_add(a3552, a3553)?
                                })?;
                                let a3555 = 45u64;
                                if if nat_eq(a3554, a3555) {
                                    f228({
                                        let a3559 = {
                                            let a3556 = v2;
                                            let a3557 = 2u64;
                                            nat_add(a3556, a3557)?
                                        };
                                        let a3560 = {
                                            let a3558 = v1.clone();
                                            length_bytes(a3558)
                                        };
                                        nat_lt(a3559, a3560)
                                    }, f228(f651({
                                        let a3563 = f350(v1.clone(), {
                                            let a3561 = v2;
                                            let a3562 = 2u64;
                                            nat_add(a3561, a3562)?
                                        })?;
                                        let a3564 = 93u64;
                                        nat_eq(a3563, a3564)
                                    }), f651({
                                        let a3567 = f350(v1.clone(), {
                                            let a3565 = v2;
                                            let a3566 = 2u64;
                                            nat_add(a3565, a3566)?
                                        })?;
                                        let a3568 = 92u64;
                                        nat_eq(a3567, a3568)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3569 = v2;
                                        let a3570 = 3u64;
                                        nat_add(a3569, a3570)?
                                    }, List::cons((v5, f350(v1.clone(), {
                                        let a3571 = v2;
                                        let a3572 = 2u64;
                                        nat_add(a3571, a3572)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3573 = v2;
                                        let a3574 = 1u64;
                                        nat_add(a3573, a3574)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677(v0: u64) -> u64 {
    let a3579 = v0;
    let a3580 = 42u64;
    if nat_eq(a3579, a3580) {
        1u64
    } else {
        let a3581 = v0;
        let a3582 = 43u64;
        if nat_eq(a3581, a3582) {
            2u64
        } else {
            let a3583 = v0;
            let a3584 = 63u64;
            if nat_eq(a3583, a3584) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f678(p3593: List<(u64, u64)>, v1: u64) -> bool {
    let m3592 = p3593;
    {
        let mut m3592 = m3592;
        loop {
            let step = {
                let v0 = m3592;
                let m3585 = v0;
                match m3585.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3587 = {
                            let (h3586, _) = v2;
                            h3586
                        };
                        let a3588 = v1;
                        if if nat_le(a3587, a3588) {
                            let a3590 = v1;
                            let a3591 = {
                                let (_, h3589) = v2;
                                h3589
                            };
                            nat_le(a3590, a3591)
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
                Ok(next) => { m3592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f679(v0: Fn24, p3598: u64, p3599: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3597 = (p3598, p3599);
    {
        let mut m3597 = m3597;
        loop {
            let step = {
                let (v1, v2) = m3597;
                let m3594 = v1;
                if m3594 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3594 - 1;
                    let m3596 = {
                        let c3595: Fn24 = v0.clone();
                        c3595.apply(v2.clone())?
                    };
                    match m3596 {
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
                Ok(next) => { m3597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f680(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f687(v0, v1)
}

pub fn f681(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3604 = {
            let a3602 = {
                let a3600 = v0.clone();
                length_list(a3600)
            };
            let a3603 = {
                let a3601 = v1.clone();
                length_list(a3601)
            };
            nat_add(a3602, a3603)?
        };
        let a3605 = 1u64;
        nat_add(a3604, a3605)?
    };
    Ok(f688({
        let (_, h3606) = {
            let (_, h3607) = {
                let (h3608, _) = f689(Fn25::F690, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3608
            };
            h3607
        };
        h3606
    }))
}

pub fn f682(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3609, _) = v0.clone();
        h3609
    };
    let v2: List<(Bytes, ())> = {
        let (h3610, _) = {
            let (_, h3611) = v0.clone();
            h3611
        };
        h3610
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3612) = {
            let (_, h3613) = v0;
            h3613
        };
        h3612
    };
    let m3614 = v1.clone();
    match m3614.uncons() {
        None => {
            let m3615 = v2;
            match m3615.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3616 = v2.clone();
            match m3616.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f691({
                        let (h3617, _) = v8.clone();
                        h3617
                    }, {
                        let (h3618, _) = v6.clone();
                        h3618
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

pub fn f683(v0: Bytes) -> R<Option<Bytes>> {
    let m3620 = f692({
        let a3619 = v0.clone();
        length_bytes(a3619)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3620 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3622 = {
                let a3621 = v1.clone();
                length_bytes(a3621)
            };
            let a3623 = 0u64;
            if nat_eq(a3622, a3623) {
                let a3625 = {
                    let a3624 = v0;
                    length_bytes(a3624)
                };
                let a3626 = 0u64;
                if nat_eq(a3625, a3626) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f685(v1)?))
            }
        }
    }
}

pub fn f684(v0: Bytes) -> R<Option<Bytes>> {
    if f693({
        let a3627 = v0.clone();
        length_bytes(a3627)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f694(v0)?;
        if f695(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3630 = {
                let a3628 = v1.clone();
                let a3629 = 0u64;
                index_bytes(a3628, a3629)
            };
            match m3630 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f467(f62(v2)?) {
                        Ok(Some({
                            let a3631 = f8(Str::lit("n"));
                            let a3632 = v1;
                            append_bytes(a3631, a3632)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f696({
            let a3633 = v0.clone();
            length_bytes(a3633)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f685(v0: Bytes) -> R<Bytes> {
    if f41(v0.clone(), f697()) {
        f552(v0)
    } else {
        Ok(v0)
    }
}

pub fn f686(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3634 = v0.clone();
    match m3634 {
        Adt17::C0 => {
            Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C1(_, _) => {
            Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C2(v4) => {
            if v4 {
                Ok(f642())
            } else {
                Ok(f588(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt17::C3(_, _) => {
            Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C4(_, _, _) => {
            Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C5(_) => {
            Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C6(_) => {
            Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C7(_) => {
            let m3635 = f698(f510(v0.clone()));
            match m3635 {
                None => {
                    let m3636 = f336(f239(v0.clone(), f8(Str::lit("$ref"))));
                    match m3636 {
                        None => {
                            let m3638 = f239(v0.clone(), f8(Str::lit("allOf")));
                            match m3638 {
                                None => {
                                    let m3643 = f239(v0.clone(), f8(Str::lit("enum")));
                                    match m3643 {
                                        None => {
                                            let m3644 = f239(v0.clone(), f8(Str::lit("const")));
                                            match m3644 {
                                                None => {
                                                    let m3646 = f239(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3646 {
                                                        None => {
                                                            let v32: Bytes = f242(v0.clone(), f8(Str::lit("type")));
                                                            if if f28(v32.clone(), Bytes::lit(&[])) {
                                                                !f699(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f642())
                                                            } else {
                                                                if f28(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f589(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f28(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f589(f700(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f28(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f589(f701(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f28(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f589(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f28(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3647 = f469(f239(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3647.uncons() {
                                                                                        None => {
                                                                                            if f596(v0.clone()) {
                                                                                                Ok(f589(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3648 = f637(v0);
                                                                                                match m3648 {
                                                                                                    None => {
                                                                                                        Ok(f640(f8(Str::lit("List")), f702(f642())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f640(f8(Str::lit("List")), f702(f703(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f589(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f588(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f589(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3645 = f336(Some(v29));
                                                    match m3645 {
                                                        None => {
                                                            Ok(f588(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f589(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f589(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3639 = f337(v21);
                                    match m3639.uncons() {
                                        None => {
                                            Ok(f588(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3640 = v23;
                                            match m3640.uncons() {
                                                None => {
                                                    let m3641 = f336(f239(v22, f8(Str::lit("$ref"))));
                                                    match m3641 {
                                                        None => {
                                                            Ok(f588(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3642 = f585(v26)?;
                                                            match m3642 {
                                                                None => {
                                                                    Ok(f588(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f589(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f588(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3637 = f585(v19)?;
                            match m3637 {
                                None => {
                                    Ok(f588(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f589(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f588(if f28(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt17::C8 => {
            Ok(f642())
        }
        Adt17::C9(_, _) => {
            Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C10(_, _, _) => {
            Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f687(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3649, _) = v1;
        h3649
    };
    let m3651 = {
        let (_, h3650) = v1;
        h3650
    };
    match m3651 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3654 = {
                let a3652 = v0.clone();
                let a3653 = v2;
                index_bytes(a3652, a3653)
            };
            match m3654 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3659 = {
                        let a3657 = v0;
                        let a3658 = {
                            let a3655 = v2;
                            let a3656 = 1u64;
                            nat_add(a3655, a3656)?
                        };
                        index_bytes(a3657, a3658)
                    };
                    match m3659 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f704(v4)? {
                                f704(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3660 = v2;
                                    let a3661 = 2u64;
                                    nat_add(a3660, a3661)?
                                }, Some({
                                    let a3666 = v3;
                                    let a3667 = f496(f705(), {
                                        let a3664 = {
                                            let a3662 = f706(v4)?;
                                            let a3663 = 16u64;
                                            nat_mul(a3662, a3663)?
                                        };
                                        let a3665 = f706(v5)?;
                                        nat_add(a3664, a3665)?
                                    });
                                    append_bytes(a3666, a3667)
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

pub fn f688(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f358(Fn4::F707, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f689(v0: Fn25, p3672: u64, p3673: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3671 = (p3672, p3673);
    {
        let mut m3671 = m3671;
        loop {
            let step = {
                let (v1, v2) = m3671;
                let m3668 = v1;
                if m3668 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3668 - 1;
                    let m3670 = {
                        let c3669: Fn25 = v0.clone();
                        c3669.apply(v2.clone())
                    };
                    match m3670 {
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
                Ok(next) => { m3671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f690(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f708(v0)
}

pub fn f691(v0: Bytes, v1: Bytes) -> bool {
    let a3678 = {
        let a3674 = v0;
        let a3675 = v1;
        compare_bytes(a3674, a3675)
    };
    let a3679 = {
        let a3676 = Bytes::lit(&[]);
        let a3677 = Bytes::lit(&[0x01]);
        compare_bytes(a3676, a3677)
    };
    equal(a3678, a3679)
}

pub fn f692(p3692: u64, v1: Bytes, p3693: u64, p3694: Bytes) -> R<Option<Bytes>> {
    let m3691 = (p3692, (p3693, p3694));
    {
        let mut m3691 = m3691;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3691;
                let m3680 = v0;
                if m3680 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3680 - 1;
                    let a3682 = {
                        let a3681 = v1.clone();
                        length_bytes(a3681)
                    };
                    let a3683 = v2;
                    if nat_le(a3682, a3683) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3684 = f650(v1.clone(), v2)?;
                        match m3684 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3686 = v2;
                                    let a3687 = {
                                        let (_, h3685) = v5;
                                        h3685
                                    };
                                    nat_add(a3686, a3687)?
                                }, {
                                    let a3689 = v3;
                                    let a3690 = f709(v1.clone(), v2, {
                                        let (h3688, _) = v5;
                                        h3688
                                    });
                                    append_bytes(a3689, a3690)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f693(p3704: u64, v1: Bytes, p3705: u64) -> R<bool> {
    let m3703 = (p3704, p3705);
    {
        let mut m3703 = m3703;
        loop {
            let step = {
                let (v0, v2) = m3703;
                let m3695 = v0;
                if m3695 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3695 - 1;
                    let m3698 = {
                        let a3696 = v1.clone();
                        let a3697 = v2;
                        index_bytes(a3696, a3697)
                    };
                    match m3698 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3699 = f62(v4)?;
                            let a3700 = 128u64;
                            if nat_lt(a3699, a3700) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3701 = v2;
                                    let a3702 = 1u64;
                                    nat_add(a3701, a3702)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
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

pub fn f694(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f411(v0.clone(), 0u64, {
        let a3706 = v0;
        length_bytes(a3706)
    }, false, Bytes::lit(&[]))?;
    let m3709 = {
        let a3707 = v1.clone();
        let a3708 = 0u64;
        index_bytes(a3707, a3708)
    };
    match m3709 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f466(v3) {
                let a3716 = f35({
                    let a3710 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3710)
                }, {
                    let a3711 = v3;
                    let a3712 = 65u64;
                    nat_sub(a3711, a3712)
                }, 1u64);
                let a3717 = f35(v1.clone(), 1u64, {
                    let a3714 = {
                        let a3713 = v1;
                        length_bytes(a3713)
                    };
                    let a3715 = 1u64;
                    nat_sub(a3714, a3715)
                });
                Ok(append_bytes(a3716, a3717))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f695(v0: Bytes) -> R<bool> {
    let m3720 = {
        let a3718 = v0;
        let a3719 = 0u64;
        index_bytes(a3718, a3719)
    };
    match m3720 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f433(f465(f62(v1)?), f466(f62(v1)?)))
        }
    }
}

pub fn f696(p3733: u64, v1: Bytes, p3734: u64, p3735: Bytes) -> R<Option<Bytes>> {
    let m3732 = (p3733, (p3734, p3735));
    {
        let mut m3732 = m3732;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3732;
                let m3721 = v0;
                if m3721 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3721 - 1;
                    let a3723 = {
                        let a3722 = v1.clone();
                        length_bytes(a3722)
                    };
                    let a3724 = v2;
                    if nat_le(a3723, a3724) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3725 = f650(v1.clone(), v2)?;
                        match m3725 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3727 = v2;
                                    let a3728 = {
                                        let (_, h3726) = v5;
                                        h3726
                                    };
                                    nat_add(a3727, a3728)?
                                }, {
                                    let a3730 = v3;
                                    let a3731 = f710({
                                        let (h3729, _) = v5;
                                        h3729
                                    });
                                    append_bytes(a3730, a3731)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f697() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f698(p3740: List<(Bytes, Adt17)>) -> Option<Bytes> {
    let m3739 = p3740;
    {
        let mut m3739 = m3739;
        loop {
            let step = {
                let v0 = m3739;
                let m3736 = v0;
                match m3736.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h3737, _) = v1.clone();
                            h3737
                        }, f711()) {
                            Err::<List<(Bytes, Adt17)>, Option<Bytes>>(Some({
                                let (h3738, _) = v1;
                                h3738
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt17)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f699(v0: Adt17) -> bool {
    if f601(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f601(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f601(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f601(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f601(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f601(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f700(v0: Adt17) -> Bytes {
    let m3741 = f568(f239(v0, f8(Str::lit("minimum"))));
    match m3741 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f701() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f702(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3742 = v0;
    match m3742 {
        Ok(v2) => {
            f589(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f518(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f517(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f703(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f712(16u64, v0, v1)
}

pub fn f704(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3787 = {
            let a3763 = {
                let a3751 = {
                    let a3745 = {
                        let a3743 = v1;
                        let a3744 = 128u8;
                        fixed_u8::bit_and(a3743, a3744)
                    };
                    let a3746 = 128u8;
                    if equal(a3745, a3746) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3752 = {
                    let a3749 = {
                        let a3747 = v1;
                        let a3748 = 64u8;
                        fixed_u8::bit_and(a3747, a3748)
                    };
                    let a3750 = 64u8;
                    if equal(a3749, a3750) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3751, a3752)?
            };
            let a3764 = {
                let a3761 = {
                    let a3755 = {
                        let a3753 = v1;
                        let a3754 = 32u8;
                        fixed_u8::bit_and(a3753, a3754)
                    };
                    let a3756 = 32u8;
                    if equal(a3755, a3756) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3762 = {
                    let a3759 = {
                        let a3757 = v1;
                        let a3758 = 16u8;
                        fixed_u8::bit_and(a3757, a3758)
                    };
                    let a3760 = 16u8;
                    if equal(a3759, a3760) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3761, a3762)?
            };
            nat_add(a3763, a3764)?
        };
        let a3788 = {
            let a3785 = {
                let a3773 = {
                    let a3767 = {
                        let a3765 = v1;
                        let a3766 = 8u8;
                        fixed_u8::bit_and(a3765, a3766)
                    };
                    let a3768 = 8u8;
                    if equal(a3767, a3768) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3774 = {
                    let a3771 = {
                        let a3769 = v1;
                        let a3770 = 4u8;
                        fixed_u8::bit_and(a3769, a3770)
                    };
                    let a3772 = 4u8;
                    if equal(a3771, a3772) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3773, a3774)?
            };
            let a3786 = {
                let a3783 = {
                    let a3777 = {
                        let a3775 = v1;
                        let a3776 = 2u8;
                        fixed_u8::bit_and(a3775, a3776)
                    };
                    let a3778 = 2u8;
                    if equal(a3777, a3778) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3784 = {
                    let a3781 = {
                        let a3779 = v1;
                        let a3780 = 1u8;
                        fixed_u8::bit_and(a3779, a3780)
                    };
                    let a3782 = 1u8;
                    if equal(a3781, a3782) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3783, a3784)?
            };
            nat_add(a3785, a3786)?
        };
        nat_add(a3787, a3788)?
    };
    let a3789 = 48u64;
    let a3790 = v2;
    if if nat_le(a3789, a3790) {
        let a3791 = v2;
        let a3792 = 57u64;
        nat_le(a3791, a3792)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3793 = 97u64;
        let a3794 = v2;
        if nat_le(a3793, a3794) {
            let a3795 = v2;
            let a3796 = 102u64;
            Ok(nat_le(a3795, a3796))
        } else {
            Ok(false)
        }
    }
}

pub fn f705() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f706(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3841 = {
            let a3817 = {
                let a3805 = {
                    let a3799 = {
                        let a3797 = v1;
                        let a3798 = 128u8;
                        fixed_u8::bit_and(a3797, a3798)
                    };
                    let a3800 = 128u8;
                    if equal(a3799, a3800) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3806 = {
                    let a3803 = {
                        let a3801 = v1;
                        let a3802 = 64u8;
                        fixed_u8::bit_and(a3801, a3802)
                    };
                    let a3804 = 64u8;
                    if equal(a3803, a3804) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3805, a3806)?
            };
            let a3818 = {
                let a3815 = {
                    let a3809 = {
                        let a3807 = v1;
                        let a3808 = 32u8;
                        fixed_u8::bit_and(a3807, a3808)
                    };
                    let a3810 = 32u8;
                    if equal(a3809, a3810) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3816 = {
                    let a3813 = {
                        let a3811 = v1;
                        let a3812 = 16u8;
                        fixed_u8::bit_and(a3811, a3812)
                    };
                    let a3814 = 16u8;
                    if equal(a3813, a3814) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3815, a3816)?
            };
            nat_add(a3817, a3818)?
        };
        let a3842 = {
            let a3839 = {
                let a3827 = {
                    let a3821 = {
                        let a3819 = v1;
                        let a3820 = 8u8;
                        fixed_u8::bit_and(a3819, a3820)
                    };
                    let a3822 = 8u8;
                    if equal(a3821, a3822) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3828 = {
                    let a3825 = {
                        let a3823 = v1;
                        let a3824 = 4u8;
                        fixed_u8::bit_and(a3823, a3824)
                    };
                    let a3826 = 4u8;
                    if equal(a3825, a3826) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3827, a3828)?
            };
            let a3840 = {
                let a3837 = {
                    let a3831 = {
                        let a3829 = v1;
                        let a3830 = 2u8;
                        fixed_u8::bit_and(a3829, a3830)
                    };
                    let a3832 = 2u8;
                    if equal(a3831, a3832) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3838 = {
                    let a3835 = {
                        let a3833 = v1;
                        let a3834 = 1u8;
                        fixed_u8::bit_and(a3833, a3834)
                    };
                    let a3836 = 1u8;
                    if equal(a3835, a3836) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3837, a3838)?
            };
            nat_add(a3839, a3840)?
        };
        nat_add(a3841, a3842)?
    };
    let a3843 = v2;
    let a3844 = 57u64;
    if nat_le(a3843, a3844) {
        let a3845 = v2;
        let a3846 = 48u64;
        Ok(nat_sub(a3845, a3846))
    } else {
        let a3847 = v2;
        let a3848 = 87u64;
        Ok(nat_sub(a3847, a3848))
    }
}

pub fn f707(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f708(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3849, _) = v0.clone();
        h3849
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3850, _) = {
            let (_, h3851) = v0.clone();
            h3851
        };
        h3850
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3852) = {
            let (_, h3853) = v0;
            h3853
        };
        h3852
    };
    let m3854 = v1.clone();
    match m3854.uncons() {
        None => {
            let m3855 = v2;
            match m3855.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3856 = v2.clone();
            match m3856.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f691({
                        let (h3857, _) = v8.clone();
                        h3857
                    }, {
                        let (h3858, _) = v6.clone();
                        h3858
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

pub fn f709(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f433(f228(f433(f465(v2), f466(v2)), f228({
        let a3859 = v2;
        let a3860 = 117u64;
        !nat_eq(a3859, a3860)
    }, {
        let a3861 = v2;
        let a3862 = 85u64;
        !nat_eq(a3861, a3862)
    })), f228(f467(v2), {
        let a3863 = 0u64;
        let a3864 = v1;
        nat_lt(a3863, a3864)
    })) {
        f35(v0, v1, 1u64)
    } else {
        let a3865 = v2;
        let a3866 = 65536u64;
        if nat_lt(a3865, a3866) {
            let a3867 = f8(Str::lit("u"));
            let a3868 = f586(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3867, a3868)
        } else {
            let a3869 = f8(Str::lit("U"));
            let a3870 = f586(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3869, a3870)
        }
    }
}

pub fn f710(v0: u64) -> Bytes {
    let a3871 = f8(Str::lit("u"));
    let a3872 = f586(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3871, a3872)
}

pub fn f711() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f712(v0: u64, v1: Adt17, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3873 = v0;
    if m3873 == 0 {
        Ok(f588(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3873 - 1;
        let m3874 = v1.clone();
        match m3874 {
            Adt17::C0 => {
                Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C1(_, _) => {
                Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C2(v6) => {
                if v6 {
                    Ok(f642())
                } else {
                    Ok(f588(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt17::C3(_, _) => {
                Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C4(_, _, _) => {
                Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C5(_) => {
                Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C6(_) => {
                Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C7(_) => {
                let m3875 = f336(f239(v1.clone(), f8(Str::lit("$ref"))));
                match m3875 {
                    None => {
                        if if f601(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f601(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f601(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f589(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f242(v1.clone(), f8(Str::lit("type")));
                            if if f28(v22.clone(), Bytes::lit(&[])) {
                                !f699(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f642())
                            } else {
                                if f28(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f589(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f28(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f589(f700(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f28(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f589(f701(), List::<Adt25>::nil()))
                                        } else {
                                            if f28(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f589(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f28(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3877 = f239(v1, f8(Str::lit("items")));
                                                    match m3877 {
                                                        None => {
                                                            Ok(f640(f8(Str::lit("List")), f642()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f640(f8(Str::lit("List")), f712(v3, v23, {
                                                                let a3878 = v2;
                                                                let a3879 = f8(Str::lit("Item"));
                                                                append_bytes(a3878, a3879)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f28(v22.clone(), f8(Str::lit("object"))) {
                                                        if f596(v1.clone()) {
                                                            Ok(f589(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3880 = f637(v1);
                                                            match m3880 {
                                                                None => {
                                                                    Ok(f640(f8(Str::lit("List")), f702(f642())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f640(f8(Str::lit("List")), f702(f712(v3, v24, {
                                                                        let a3881 = v2;
                                                                        let a3882 = f8(Str::lit("Value"));
                                                                        append_bytes(a3881, a3882)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f588(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(v20) => {
                        let m3876 = f585(v20)?;
                        match m3876 {
                            None => {
                                Ok(f588(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f589(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt17::C8 => {
                Ok(f642())
            }
            Adt17::C9(_, _) => {
                Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C10(_, _, _) => {
                Ok(f588(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}
